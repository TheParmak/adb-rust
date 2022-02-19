# adb-rust
a simple adb implemantation for rust

example usage: 

```rust
mod adb;

fn main() {
    adb::adb("shell bu help", "bin/adb");
}
```
