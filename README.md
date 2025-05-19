# Learn Rust 2025

### Install Rust
```bash
# follow the instructions on https://www.rust-lang.org/tools/install
# 1. install cargo and rust kit via rustup  
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. add ${HOME}/.cargo to ${PATH}
export PATH=${PATH}:${HOME}/.cargo

# 3. here's the directory structure that rustup created.
  tree ${HOME}/.cargo -L 2
  
  
    .cargo
     ├── bin
     │  ├── cargo -> rustup
     │  ├── cargo-clippy -> rustup
     │  ├── cargo-fmt -> rustup
     │  ├── cargo-miri -> rustup
     │  ├── clippy-driver -> rustup
     │  ├── rls -> rustup
     │  ├── rust-analyzer -> rustup
     │  ├── rustc -> rustup
     │  ├── rustdoc -> rustup
     │  ├── rustfmt -> rustup
     │  ├── rust-gdb -> rustup
     │  ├── rust-gdbgui -> rustup
     │  ├── rust-lldb -> rustup
     │  └── rustup
     ├── env
     └── registry
         ├── cache
         ├── CACHEDIR.TAG
         ├── index
         └── src

```

### Build and Run A Rust Project

```bash

# 1. create a project
cargo new hello_rust_proj
cd ./hello_rust_proj

  # here's the directory structure
  # hello_rust_proj/
  # ├── Cargo.toml
  # ├── src
  #      └── main.rs# here's the directory structure
  # hello_rust_proj/
  # ├── Cargo.toml
  # ├── src
  #      └── main.rs

# 2. build the project
cargo build

# 3. run the project
# run the debug exe at ./target/debug/hello_rust_proj
cargo run
  # output:
  # hello world

```

### References
1. [Install Rust using rustup](https://www.rust-lang.org/tools/install)
2. [Learn Rust from rust-lang.org/learn](https://www.rust-lang.org/learn) 
  - [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
3. [Jet Brains RustRover](https://www.jetbrains.com/help/rust/quick-start-guide-rustrover.html#build-run)


