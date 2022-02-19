# adb-rust
a simple adb implemantation for rust

example usage: 

```rust
mod adb;

fn main() {
    adb::adb("shell bu help", "bin/adb");
}
```

file structure
```
├── adb.rs //adb-rust code
├── bin
│   └── adb
├── main //compiled binary
├── main.rs //example code
```
