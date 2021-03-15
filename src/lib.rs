//! puke
//!
//! a **very** early-stage language with a focus on:
//!
//! * the terseness of erlang/prolog
//! * first-class fault injection, simulation and model-based testing capabilities
//! * built-in data structures
//! * high quality scheduling built from the beginning to play well with io_uring and iocp

mod args;
mod ast;
mod cache_padded;
mod ebr;
mod histogram;
mod io;
mod lazy;
mod machine_table;
mod stack;

use lalrpop_util::lalrpop_mod;

pub use self::args::Args;

use self::{
    cache_padded::CachePadded, ebr::pin, histogram::Histogram, lazy::Lazy,
    machine_table::MachineTable, stack::Stack,
};

#[cfg(any(test, feature = "lock_free_delays"))]
mod debug_delay;

#[cfg(any(test, feature = "lock_free_delays"))]
use debug_delay::debug_delay;

lalrpop_mod!(pub syntax);

/// This function is useful for inducing random jitter into our atomic
/// operations, shaking out more possible interleavings quickly. It gets
/// fully eliminated by the compiler in non-test code.
#[cfg(not(any(test, feature = "lock_free_delays")))]
const fn debug_delay() {}

/// The puke interpreter.
pub struct Interpreter {
    ast: Ast,
    scheduler: Scheduler,
}

impl Interpreter {
    pub fn run(args: Args) {
        println!(
            "
            +-------------------------------+
             w e l c o m e   2   p u k e  :]
            +-------------------------------+"
        );
        println!();
        println!("{:?}", args);
        println!();

        let mut interpreter = Interpreter {
            ast: args.module.as_ref().map(|m| Ast::load(&m)).unwrap_or(Ast),
            scheduler: Scheduler::start(&args),
        };

        interpreter.repl();
    }

    fn repl(&mut self) {}
}

/// The puke AST.
struct Ast;

impl Ast {
    fn load(path: &str) -> Ast {
        todo!()
    }
}

/// The puke scheduler.
struct Scheduler {
    work: MachineTable,
}

impl Scheduler {
    fn start(args: &Args) -> Scheduler {
        Scheduler {
            work: MachineTable::default(),
        }
    }
}

struct MessageBus;

struct Stm;

struct Machine {
    ast: Ast,
}
