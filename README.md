# JPEB Emulator

This is an emulator for programs written or compiled to JPEB assembly. We built off [this emulator](https://github.com/b-Rocks2718/c-compiler/tree/master/src) by adding graphics and keyboard input.

## Building
1. [Install Rust and Cargo](https://www.rust-lang.org/tools/install)  
2. `cargo build --release`  
   The release build will be used for full-stack tests.  

Run the program with a binary file of JPEB machine code and a path to data directory (omit to use the default).  
`cargo run --release program.bin data/`  
