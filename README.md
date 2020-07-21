# Brainfuck-interpreter
A fast brainfuck implementation written in Rust

# Build
```
cargo build --release
```
The `--release` argument is required for integer overflow.

# Optimizations
- Contraction
- Clear loops
- Jump optimization
- Operation offsets
- No bound checking
