# ProjectEuler-Rust
[![Rust](https://github.com/mlefebvre1/projecteuler-rust/actions/workflows/rust.yml/badge.svg)](https://github.com/mlefebvre1/projecteuler-rust/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/mlefebvre1/projecteuler-rust/branch/main/graph/badge.svg?token=2K3BD4KVTE)](https://codecov.io/gh/mlefebvre1/projecteuler-rust)
[![rust_version](https://img.shields.io/badge/rustc-1.60%2B-blue.svg)](https://img.shields.io/badge/rustc-1.60%2B-blue.svg)
<br>
[![EulerBadge](https://projecteuler.net/profile/mlefebvre.png)](https://projecteuler.net/profile/mlefebvre.png)




### Run all problems
```shell
$ cargo test --release
```
or using nextest
```shell
$ cargo nextest --release run
```
### Run a single problem (ex: run problem #15)
```shell
cargo test --release -- problem15 --nocapture
```
or using nextest
```shell
$ cargo nextest run --release problem15 --nocapture
```
