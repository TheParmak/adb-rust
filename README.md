# adb-rust
a simple adb implemantation for rust

example usage with dynamic path variable: 
	```rust
	mod adb;
	use adb::adb as adb;

	fn main() {
		adb("shell bu help", "bin/adb");
	}
	```

example usage with static path variable:
	```rust
	mod adb_static_path;
	use adb_static_path::adb_static_path as adb;

	fn main() {
		adb("shell bu help");
	}
	```
	adb_static_path.rs:
	```rust
	...
	let adb_path = "bin/adb"; //set your adb path here
	...
	```


file structure:
	```
	├── adb.rs //adb-rust code
	├── adb_static_path.rs //adb-rust code but it doesn't take an adb_path variable instead you set it statically in the file
	├── bin //bin folder
	│   └── adb //adb stand-alone binary
	├── LICENSE //License file
	├── README.md //Readme file
	└── main.rs //example code
	```
