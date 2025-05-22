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

#### Usage of "take" in Rust
src = https://doc.rust-lang.org/std/mem/fn.take.html

```
use std::mem;

struct Buffer<T> { buf: Vec<T> }

impl<T> Buffer<T> {
    fn get_and_reset(&mut self) -> Vec<T> {
        // the following line uses std::mem::take to do the following;
        // a. return initial value in self.buf
        // and 
        // b. assigns default value ( Vec::new() ) to self.buf
        
        mem::take(&mut self.buf)  
    }
}

fn main( ) {
  
  // create instance of Buffer<int32>
  let mut buffer = Buffer { buf: vec![0, 1] };
  
  // len==2
  assert_eq!(buffer.buf.len(), 2);
  
  // call `mem::take(&mut self.buf)` 
  // returns buffer.buf (=vec![0,1])
  // and resets buffer.buf  (ie buffer.buf := vec![] )
  let mut initial_buf =buffer.get_and_reset();
  
  assert_eq!(initial_buf, vec![0, 1]);
  
  // assess current value of buffer.buf
  assert_eq!(buffer.buf, vec![]);
  assert_eq!(buffer.buf.len(), 0);

}



```

### References
1. [Install Rust using rustup](https://www.rust-lang.org/tools/install)
2. [Learn Rust from rust-lang.org/learn](https://www.rust-lang.org/learn) 
  - [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
3. [Jet Brains RustRover](https://www.jetbrains.com/help/rust/quick-start-guide-rustrover.html#build-run)


