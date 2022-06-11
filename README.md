# Music To Musicians

### Install Rust

See this instructions for install [Rust](https://www.rust-lang.org/tools/install).

Rocket Framework need nightly version. Run this command:

```
rustup default nightly
```

### Build and Run

1. Configure .env file with next keys

```
MONGO_URI = <mongo string connection>
RUST_BACKTRACE=1
```

2. Run commands

```
cargo build
cargo run
```