# untokio: pretend tokio doesn't exist

[![GitHub](https://img.shields.io/github/stars/MaulingMonkey/untokio.svg?label=GitHub&style=social)](https://github.com/MaulingMonkey/untokio)
[![crates.io](https://img.shields.io/crates/v/untokio.svg)](https://crates.io/crates/untokio)
[![docs.rs](https://docs.rs/untokio/badge.svg)](https://docs.rs/untokio)
[![License](https://img.shields.io/crates/l/untokio.svg)](https://github.com/MaulingMonkey/untokio)
[![Build Status](https://travis-ci.com/MaulingMonkey/untokio.svg?branch=master)](https://travis-ci.com/MaulingMonkey/untokio)
<!-- [![dependency status](https://deps.rs/repo/github/MaulingMonkey/untokio/status.svg)](https://deps.rs/repo/github/MaulingMonkey/untokio) -->

Do you struggle with any of the following symptoms?

* Intense ennui triggered by the thought of inflicting tokio::main or tokio runtime management on users of your library
* Indecision about what runtime configuration you should use for **your** runtimes?
* `thread 'main' panicked at 'not currently running on the Tokio runtime.'`

Struggle no more, with `untokio`!  `untokio` will automatically create a runtime so you don't have to.

## [example](examples/example02.rs): tokio = "0.2", reqwest = "0.10"

```toml
[dependencies]
untokio = { version = "0.1", features = ["v02"] }
```
```rust
untokio::v02::spawn(async {
    // code requiring a tokio 0.2 runtime
    reqwest::get("http://example.com/").await?.text().await
}).await.unwrap()
```

## [example](examples/example03.rs): tokio = "0.3"

```toml
[dependencies]
untokio = { version = "0.1", features = ["v02"] }
```
```rust
untokio::v03::spawn(async{
    // code requiring a tokio 0.3 runtime
    tokio::fs::read_to_string("Cargo.toml").await
}).await.unwrap()
```



<h2 name="license">License</h2>

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.



<h2 name="contribution">Contribution</h2>

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
