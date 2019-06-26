Background:
- I want to compile `a` and `b` running `cargo build` from the root 
- Crate `a` has a feature `test`, which is needed (here simulated as `compile_error!` if compiled without `test`)
- `cargo build --all-features` works

What does **not** work:
- `cargo build --features a/test` does not work
- `cargo build --features test` also does not work

I need a way to configure various features from the top-level command. In the real world that could be 
`cargo build --features a/training, b/avx, c/fastmath` 

How can I do that?  
