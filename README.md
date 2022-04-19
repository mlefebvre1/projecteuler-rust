# ProjectEuler-Rust
[![Rust](https://github.com/mlefebvre1/projecteuler-rust/actions/workflows/rust.yml/badge.svg)](https://github.com/mlefebvre1/projecteuler-rust/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/mlefebvre1/projecteuler-rust/branch/main/graph/badge.svg?token=2K3BD4KVTE)](https://codecov.io/gh/mlefebvre1/projecteuler-rust)
[![EulerBadge](https://projecteuler.net/profile/mlefebvre.png)](https://projecteuler.net/profile/mlefebvre.png)

Original solutions from ProjectEuler-Python re-written in Rust for improved speed.


### Run all problems
```shell
$ cargo run --release
```
### Run a single problem (ex: run problem #15)
```shell
cargo run --release -- -n 15
```

### Run using docker (for vscode use devcontainer)
```shell
docker build -t projecteuler-rust .
docker run projecteuler-rust
```

### Testing
```shell
$ cargo test --release
```
