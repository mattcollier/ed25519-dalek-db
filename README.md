# ed25519-dalek

### Install nj-cli
```
cargo install nj-cli
```

### Build with AVX/SIMD optimizations
```
RUSTFLAGS="-C target_cpu=native" nj-cli build --release
```
