# maze  
A Cellular Automaton (CA) that generates a family of maze-like structures.  
# Building  
Build with `cargo build --release` if you want optimizations, `cargo build` otherwise.  
# Running  
If built with `cargo build --release`, the binary will be located under
`/target/release/maze`.  
If built with `cargo build` the binary will be located under
`/target/debug/maze`.  
In both cases, the preferred way to run the simulation is with `cargo run`.

# Customizing the behaviour  
I intentionally left some parts of the code commented out. these parts act as a 
"randomizer" for the simulation. that is, the simulation will start with a random
initial state for each cell (either dead or alive) and it will evolve from there.

