# Out-of-bounds vector access in Rust
This repository demonstrates a common error in Rust: accessing an element in a vector using an index that is out of bounds.  This leads to a runtime panic, abruptly terminating the program.
The `bug.rs` file contains the erroneous code.  The `bugSolution.rs` file provides a corrected version that handles potential out-of-bounds errors gracefully.
This example highlights the importance of careful index checking when working with vectors (or other data structures with bounded sizes) in Rust.