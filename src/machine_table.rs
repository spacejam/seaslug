//! A simple wait-free, grow-only pagetable, assumes a dense keyspace.
#![allow(unsafe_code)]

use std::{
    alloc::{alloc_zeroed, Layout},
    convert::TryFrom,
    mem::{align_of, size_of},
    sync::atomic::{
        AtomicU64,
        Ordering::{Acquire, Relaxed, Release},
    },
};

use crate::{
    debug_delay,
    ebr::{pin, Atomic, Guard, Owned, Shared},
    Machine, Stack,
};

#[cfg(feature = "metrics")]
use crate::{Measure, M};

#[allow(unused)]
#[doc(hidden)]
pub const PAGETABLE_NODE_SZ: usize = size_of::<Node1>();

const MAX_MID_BITS: usize = 37;
const MAX_MID: u64 = 1 << MAX_MID_BITS;
const NODE2_FAN_FACTOR: usize = 18;
const NODE1_FAN_OUT: usize = 1 << (MAX_MID_BITS - NODE2_FAN_FACTOR);
const NODE2_FAN_OUT: usize = 1 << NODE2_FAN_FACTOR;
const FAN_MASK: u64 = (NODE2_FAN_OUT - 1) as u64;

pub type MachineId = u64;

struct Node1 {
    children: [Atomic<Node2>; NODE1_FAN_OUT],
}

struct Node2 {
    children: [Atomic<Machine>; NODE2_FAN_OUT],
}

impl Node1 {
    fn new() -> Owned<Self> {
        let size = size_of::<Self>();
        let align = align_of::<Self>();

        unsafe {
            let layout = Layout::from_size_align_unchecked(size, align);

            #[allow(clippy::cast_ptr_alignment)]
            let ptr = alloc_zeroed(layout) as *mut Self;

            Owned::from_raw(ptr)
        }
    }
}

impl Node2 {
    fn new() -> Owned<Node2> {
        let size = size_of::<Self>();
        let align = align_of::<Self>();

        unsafe {
            let layout = Layout::from_size_align_unchecked(size, align);

            #[allow(clippy::cast_ptr_alignment)]
            let ptr = alloc_zeroed(layout) as *mut Self;

            Owned::from_raw(ptr)
        }
    }
}

impl Drop for Node1 {
    fn drop(&mut self) {
        drop_iter(self.children.iter());
    }
}

impl Drop for Node2 {
    fn drop(&mut self) {
        drop_iter(self.children.iter());
    }
}

fn drop_iter<T>(iter: core::slice::Iter<'_, Atomic<T>>) {
    let guard = pin();
    for child in iter {
        let shared_child = child.load(Relaxed, &guard);
        if shared_child.is_null() {
            // this does not leak because the MachineTable is
            // assumed to be dense.
            break;
        }
        unsafe {
            drop(shared_child.into_owned());
        }
    }
}

/// A simple lock-free radix tree.
pub struct MachineTable {
    head: Atomic<Node1>,
    free: Stack<MachineId>,
    highest_mid: AtomicU64,
    max_mid: u64,
}

impl Default for MachineTable {
    fn default() -> Self {
        let head = Node1::new();
        Self {
            head: Atomic::from(head),
            free: Stack::default(),
            highest_mid: 0.into(),
            max_mid: MAX_MID,
        }
    }
}

impl MachineTable {
    /// # Panics
    ///
    /// will panic if the item is not null already,
    /// which represents a serious failure to
    /// properly handle lifecycles of pages in the
    /// using system.
    pub(crate) fn insert<'g>(&self, item: Machine, guard: &'g Guard) -> Option<MachineId> {
        debug_delay();
        let mid = if let Some(mid) = self.free.pop(guard) {
            mid
        } else {
            let mid = self.highest_mid.fetch_add(1, Acquire);
            if mid <= self.max_mid {
                mid
            } else {
                return None;
            }
        };

        let tip = self.traverse(mid, guard);

        let shared = Owned::new(item).into_shared(guard);
        let old = tip.swap(shared, Release, guard);
        assert!(old.is_null());

        Some(mid)
    }

    /// Try to get a value from the tree.
    ///
    /// # Panics
    ///
    /// Panics if the page has never been allocated.
    pub(crate) fn get<'g>(&self, pid: MachineId, guard: &'g Guard) -> Shared<'g, Machine> {
        #[cfg(feature = "metrics")]
        let _measure = Measure::new(&M.get_pagetable);
        debug_delay();
        let tip = self.traverse(pid, guard);

        debug_delay();
        let res = tip.load(Acquire, guard);

        assert!(!res.is_null(), "tried to get pid {}", pid);

        res
    }

    pub(crate) fn contains_pid(&self, pid: MachineId, guard: &Guard) -> bool {
        #[cfg(feature = "metrics")]
        let _measure = Measure::new(&M.get_pagetable);
        debug_delay();
        let tip = self.traverse(pid, guard);

        debug_delay();
        let res = tip.load(Acquire, guard);

        !res.is_null()
    }

    fn traverse<'g>(&self, k: MachineId, guard: &'g Guard) -> &'g Atomic<Machine> {
        let (l1k, l2k) = split_fanout(k);

        debug_delay();
        let head = self.head.load(Acquire, guard);

        debug_delay();
        let l1 = unsafe { &head.deref().children };

        debug_delay();
        let mut l2_ptr = l1[l1k].load(Acquire, guard);

        if l2_ptr.is_null() {
            let next_child = Node2::new();

            debug_delay();
            let ret = l1[l1k].compare_and_set(Shared::null(), next_child, Release, guard);

            l2_ptr = match ret {
                Ok(next_child) => next_child,
                Err(returned) => {
                    drop(returned.new);
                    returned.current
                }
            };
        }

        debug_delay();
        let l2 = unsafe { &l2_ptr.deref().children };

        &l2[l2k]
    }
}

#[inline]
fn split_fanout(id: MachineId) -> (usize, usize) {
    // right shift 32 on 32-bit pointer systems panics
    #[cfg(target_pointer_width = "64")]
    assert!(
        id <= 1 << MAX_MID_BITS,
        "trying to access key of {}, which is \
         higher than 2 ^ {}",
        id,
        MAX_MID_BITS,
    );

    let left = id >> NODE2_FAN_FACTOR;
    let right = id & FAN_MASK;

    (safe_usize(left), safe_usize(right))
}

#[inline]
fn safe_usize(value: MachineId) -> usize {
    usize::try_from(value).unwrap()
}

impl Drop for MachineTable {
    fn drop(&mut self) {
        let guard = pin();
        let head = self.head.load(Relaxed, &guard);
        unsafe {
            drop(head.into_owned());
        }
    }
}

#[test]
fn fanout_functionality() {
    assert_eq!(
        split_fanout(0b11_1111_1111_1111_1111),
        (0, 0b11_1111_1111_1111_1111)
    );
    assert_eq!(
        split_fanout(0b111_1111_1111_1111_1111),
        (0b1, 0b11_1111_1111_1111_1111)
    );
}
