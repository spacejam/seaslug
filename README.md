# puke

a language that is happy and eager to suck at many things in pursuit of 
low-boilerplate, low-noise, highly reliable, high performance state machines 
with enforced strict error handling and built-in fault injection.

puke may become a language that encourages users to write
code that is usable by others and respectful to themselves
by providing superior scheduling and concurrency facilities
to what is generally available in current industrial languages.

##### ideas that may or may not lead to puke

* erlang's terseness, explicit supervisor hierarchies, pattern matching
* prolog's declarative debugging, tables, provability of pure code
* total functional programming
* unison's effect systems
* rust's restricted linear typing

##### puke should excel at

* testability
  * built-in network simulation, fail-points, concurrent interleaving exploration, model-based testing support
* optimistic concurrency
  * shared mutable state regulated by optimistic software transactional memory
* scheduling
  * the runtime tracks overall utilization and saturation
  * aims to maximize utilization while minimizing saturation (writes before reads before accepts for sockets)

##### puke does not aim to excel at

* data structures
* CLI applications
* embedded programming

##### puke is written in rust and takes advantage of things like

* io_uring
* simdjson
