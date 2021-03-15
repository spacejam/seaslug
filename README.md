# seaslug

small, beautiful, knowable, DOESN'T EXIST YET LOL

* non-turing complete, verified terminating code placed into well-defined interfaces similar to Erlang behaviors
* strongly typed, powerful inference to minimize type noise
* a runtime built from the beginning to take advantage of io_uring, immutable messaging, optimistic STM, and generally paying attention to modern scheduling research
  * the runtime tracks overall utilization and saturation
  * aims to maximize utilization while minimizing saturation (writes before reads before accepts for sockets)
  * infers contention causality encountered in the optimistic state layer and learns to step in as
    a traffic director, letting users avoid thinking about contention
* leans into linear logic to enforce error handling in well-specified hierarchies, similar to Erlang supervisors
* interpreter-first, but the interpreter itself aims to compile quickly so you can produce static optimized native binaries (something something Futamura something...)
* built-in first-class fault injection, fuzzing, network simulation, model-based testing, and concurrent interleaving exploration
* rich built-in data structures, first-class json support, first-class binary parsing support

##### seaslug should be an ideal language for building

* databases
* distributed systems
* servers
* scalable concurrent systems
* multitenant infrastructure

##### seaslug does not aim to excel at

* building your own data structures
* intensive numerical processing
* short-lived CLI applications
* embedded programming

##### ideas that may or may not lead to seaslug

* erlang's terseness, behaviors, focus on immutable message passing, explicit supervisor hierarchies, pattern matching, binary parsing capabilities
* celf's nondeterminism and linear logic
* constraint handling rules's verification opportunities
* prolog's declarative debugging, tables, provability of pure code
* total functional programming's restrictions on recursion and looping

##### seaslug is written in rust and takes advantage of things like

* io_uring
* simdjson
* sled
* software transactional memory
