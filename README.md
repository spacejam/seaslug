# seaslug (née puke)

small, knowable, locked-down pure code embedded in a high performance runtime, DOESN'T EXIST YET LOL

##### workflow

1. play with some ideas in the repl
1. write a program that uses minimal type or constraint specification
1. write a simple property test
1. check that property with the built-in model checking, fuzzing,
  network simulation, or concurrent interleaving functionality enabled
1. the test system discovers and shrinks a counterexample input that causes the
  property to be violated
1. add additional compile-time constraints, verified behind the scenes via z3
1. repeat as appropriate
1. deploy to production, where IO is facilitated through a high quality
  io_uring/IOCP-backed priority and resource saturation-aware runtime.

##### seaslug should be an ideal language for building

* databases
* distributed systems
* servers
* overlay networks
* scalable concurrent systems
* multitenant infrastructure

##### seaslug does not aim to excel at

* building your own data structures (write this in Rust and use + test it from seaslug)
* intensive numerical processing (write this in Rust and use + test it from seaslug)

##### motivation

I hope to create systems that can be forgotten in the way that an
if-conditional can be forgotten, or the way that SQLite can be forgotten. When
things match our expectations, we do not need to burden our active minds with
their complexity. I hope to be part of communities that are not fixated on the
problems of a programming language itself, but by the fascinating complexity of
reality which the programmer is enabled by a language to be at play with.

Engineers (myself included) are constantly running into the same traps over and
over again. A huge number of problems boil down to the simple fact that we
write a lot of code that is never actually executed while we are developing it.

I've talked about this [again](https://sled.rs/simulation) and
[again](https://sled.rs/errors), but there are two particularly interesting
excerpts from the paper [Simple Testing Can Prevent Most Critical Failures: An
Analysis of Production Failures in Distributed Data-intensive
Systems](http://www.eecg.toronto.edu/~yuan/papers/failure_analysis_osdi14.pdf)
that I feel a language should assist a programmer in addressing:

```
almost all (92%) of the catastrophic system failures
are the result of incorrect handling of non-fatal errors
explicitly signaled in software.
```

That is, we tend to write error handling code that is broken.

```
in 58% of the catastrophic failures, the underlying
faults could easily have been detected through simple
testing of error handling code.
```

That is, if the error handling code had been tested in even a rudimentary way,
the problems that caused the system failure would likely have been detected.

Most of our problems are the kinds of problems that require some sort of fault
injection if we want to find them before reality fault injects them without our
consent while they are running in production. While it is possible in many
cases to avoid a growing body of defects through interesting type systems, the
effort required to thoroughly specify correctness properties of many systems is
made more difficult by the challenging process required to determine what the
correct specification of the program actually is.

I would like to have a language that is lightweight to initially write, but
that allows rich specifications to be incorporated over time as the programmer
learns through the built-in model checking functionality to bolt things down
more and more through an interactive process.

##### influences

* [Erlang](https://erlang.org/doc/) is among the most elegant programming
  languages for building reliable distributed systems in popular use today. [It
  has a variety of features](https://ferd.ca/the-zen-of-erlang.html) that
  strongly contribute to a feeling of lightness while writing programs. Over
  time, the Erlang community has engineered some incredibly powerful testing
  tools such as [PULSE](http://quviq.com/documentation/pulse/index.html),
  [Conquerror](https://github.com/parapluu/Concuerror),
  [CutEr](https://github.com/cuter-testing/cuter), and several high-quality
  property testing frameworks that have seen tremendous success in creating
  robust distributed systems.
* [Ada SPARK](https://www.adacore.com/about-spark) combines functionality for
  restricting language features that are expensive to verify with a built-in
  verification toolchain. Ada and SPARK have seen great success in the
  safety-critical domain due to several features that ultimately make the
  software understandable by humans and the tools we have available for
  assisting us in the understanding process.
* [The P language](https://github.com/p-org/P) is a language with first-class
  state-machine support, which in many ways feels like what would happen if
  TLA+ were to become an actual programming language.
* [The Dafny language](https://github.com/dafny-lang/dafny) is a language
  that leans into [the amazing capabilities of
  z3](https://www.microsoft.com/en-us/research/video/the-varied-forms-of-verification-with-z3/)
  to prove [a powerful set of constraints at
  compile-time](https://dafny-lang.github.io/dafny/QuickReference).
* [The Bologna Optimal Higher-order Machine (BOHM)](https://github.com/asperti/BOHM1.1)
  The BOHM is an implementation of a lazy term reduction strategy sketched out
  in the book "The Optimal Implementation of Functional Programming Languages"
  by Andrea Asperti and Stefano Guerrini that discusses interesting
  applications of linear logic to computation.
* [Celf](https://www.cs.cmu.edu/~cmartens/lpnmr13.pdf)'s nondeterminism and
  interesting applications of linear logic.
* [Constraint Handling Rules](https://en.wikipedia.org/wiki/Constraint_Handling_Rules)'s verification opportunities
* Prolog's declarative debugging, tables, proof capabilities
* Total Functional Programming's restrictions on recursion and looping

##### prospective features

This project is still very much in the prospective research phase where I'm trying
to understand which of these features may not compose well with the others. That
said, here are some of the bits of functionality that I am interested in:

* erlang's terseness, behaviors, focus on immutable message passing, explicit
  supervisor hierarchies, pattern matching, binary parsing capabilities
* built-in fault injection, fuzzing, model checking (with built-in linearizability-aware history validator),
  network simulation, concurrent interleaving exploration, ALICE-like filesystem action verification
* typed actors running pure state transformation functions using inputs provided by a high performance IO engine
* immutable messaging for unidirectional communication
* optimistic STM for modifying shared state in a safe way
* escape hatch: easy calling of Rust functions similar to Erlang NIFs and ports
* the ability to reason about termination (in some contexts), value ranges or other invariants at compile-time inspired by Dafny
* a runtime built from the beginning to take advantage of io_uring/IOCP, and generally paying attention to modern scheduling research
  * the runtime tracks system resource utilization and saturation
  * the default server behavior aims to maximize utilization while minimizing saturation,
    prioritizing writes before reads before accepts for sockets
    to keep in-flight work at an appropriate level for avoiding adverse saturation.
  * infers contention causality encountered in the optimistic state layer and learns to step in as
    a traffic director when a pessimistic approach becomes necessary to avoid
    contention and wasted effort that leads to system saturation, letting users
    avoid thinking about contention.
* leans into linear logic to enforce error handling in well-specified hierarchies, similar to Erlang supervisors
* interpreter-first, but the interpreter itself aims to compile quickly so you can produce static optimized native binaries (something something Futamura something...)
* built-in first-class fault injection, fuzzing, network simulation, model-based testing, and concurrent interleaving exploration
* rich built-in data structures, first-class json support, first-class binary parsing support

##### seaslug is written in Rust and takes advantage of things like

* io_uring
* simdjson
* sled
* software transactional memory
