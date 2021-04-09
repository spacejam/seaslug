//! SMT solver based on Dutertre & de Moura's paper
//! [A Fast Linear-Arithmetic Solver for DPLL(T)](cav06.pdf).
//!
//! For cases where z3 is too heavy of a dependency.

pub struct Input;
pub struct Output;

pub fn solve(_: Input) -> Output {
    todo!()
}

struct Atom;

struct Solver {
    atoms: Vec<Atom>,
}

impl Solver {
    /// Asserts atom gamma in the current state.
    /// If consistent, gamma is inserted into
    /// alpha. If alpha union gamma is inconsistent,
    /// big gamma is returned which is the shrunken
    /// minimal failing subset of gamma that results
    /// in inconsistency.
    fn assert(&mut self, proposed: Atom) -> Result<(), Vec<Atom>> {
        todo!()
    }

    /// Checks whether our atoms are consistent.
    /// A new checkpoint is created when we return Ok.
    fn check(&self) -> Result<(), Vec<Atom>> {
        todo!()
    }
}

struct Dpll;
enum Clause {
    Literal(bool),
    Empty,
    Unit,
}
struct Literal;

impl Dpll {
    fn dpll(f: &[Clause]) -> bool {
        if todo!("f is a consistent set of literals") {
            return true;
        }
        if todo!("f contains an empty clause") {
            return false;
        }

        for l in todo!("each unit clause in f") {
            f.unit_propagate(l);
        }

        for l in todo!("each pure literal in f") {
            f.pure_literal_assign(l);
        }
        let l = f.choose_literal();
        Dpll::dpll(f && l) || Dpll::dpll(f && !l)
    }

    fn consistent(f: &[Literal]) -> bool {
        todo!()
    }

    /// apply unit propagation rule
    fn unit_propagate() {}

    /// apply pure literal rule
    fn pure_literal_assign() {}
}
