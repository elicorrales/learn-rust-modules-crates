# Brief Intro to Rust Libraries, Modules, Crates  
  
## Prep For Learning  
We created this very project by typing ```cargo init --bin --vcs none``` at the command-line, while within the ```learn-rust-modules-crates-pallets``` directory.  

## Bin Vs Lib  
  
Looking online, there are many ways to explain this.  To keep it simple (oversimplified?), we'll say that:
- a bin is an executable; something you can run on the command-line.
    - example: ```cargo run``` will work if there is a ```main.rs``` with "Hello World" output.
- a lib is typically not executable by itself.
    - example: ```cargo run``` will fail if all you have is a ```lib.rs```.
- a lib typically contains helper functionality that can be used by a bin.  

### Real-life Examples  
  
#### Attempt to execute (run) each project.  
  
```
cd a-bin-project/
IamDeveloper@SoftwareDevelopUbuntu2004 
~/MySoftwareProjects/blockchain/rust/rust-substrate-blockchain-projects/my-first-substrate-projects/my-first-project-prep-lesson/learn-rust-modules-crates-pallets/a-bin-project
$ cargo run
   Compiling a-bin-project v0.1.0 (/home/IamDeveloper/MySoftwareProjects/blockchain/rust/rust-substrate-blockchain-projects/my-first-substrate-projects/my-first-project-prep-lesson/learn-rust-modules-crates-pallets/a-bin-project)
    Finished dev [unoptimized + debuginfo] target(s) in 0.57s
     Running `target/debug/a-bin-project`
Hello, world!
```

```
$ cd a-lib-project/
IamDeveloper@SoftwareDevelopUbuntu2004 
~/MySoftwareProjects/blockchain/rust/rust-substrate-blockchain-projects/my-first-substrate-projects/my-first-project-prep-lesson/learn-rust-modules-crates-pallets/a-lib-project
$ cargo run
error: a bin target must be available for `cargo run`
```
  
However, we can build each project.

  
#### Attempt to only build each project.  
  
##### Let's start with fresh bin and lib projects.  Here is what exists at the moment.  
```
$ tree
.
├── README.md
├── a-bin-project
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── a-lib-project
    ├── Cargo.toml
    └── src
        └── lib.rs
```
  
##### Contents of ```a-bin-project/src/main.rs```:  
```
$ cat a-bin-project/src/main.rs
fn say_hello() {
    println!("Hello, world!");
}
fn main() {
  say_hello();
}
```
  
##### Contents of ```a-lib-project/src/lib.rs```:  
```
$ cat a-lib-project/src/lib.rs
fn say_hello() {
    println!("Hello, world!");
}
```
  
##### We build:  
```
$ cd a-bin-project/
$ cargo build --release
    Finished release [optimized] target(s) in 0.00s
```
```
$ cd a-lib-project/
$ cargo build --release
warning: function is never used: `say_hello`
 --> src/lib.rs:1:4
  |
1 | fn say_hello() {
  |    ^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: `a-lib-project` (lib) generated 1 warning
    Finished release [optimized] target(s) in 0.01s
```
  
##### Contents of each project now:  

```
$ tree
.
├── README.md
├── a-bin-project
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── src
│   │   └── main.rs
│   └── target
│       ├── CACHEDIR.TAG
│       └── release
│           ├── a-bin-project
│           ├── a-bin-project.d
│           ├── build
│           ├── deps
│           │   ├── a_bin_project-83c0f8ea33b2ef4d
│           │   └── a_bin_project-83c0f8ea33b2ef4d.d
│           ├── examples
│           └── incremental
└── a-lib-project
    ├── Cargo.lock
    ├── Cargo.toml
    ├── src
    │   └── lib.rs
    └── target
        ├── CACHEDIR.TAG
        └── release
            ├── build
            ├── deps
            │   ├── a_lib_project-d5d45a4da9430f60.d
            │   ├── liba_lib_project-d5d45a4da9430f60.rlib
            │   └── liba_lib_project-d5d45a4da9430f60.rmeta
            ├── examples
            ├── incremental
            ├── liba_lib_project.d
            └── liba_lib_project.rlib
```
  
##### Let's look at the important files:  
  
```
$ file a-bin-project/target/release/a-bin-project
a-bin-project/target/release/a-bin-project: ELF 64-bit LSB shared object, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, BuildID[sha1]=a12aac9241d01b70f36192686bdd50dcf2cc4b20, for GNU/Linux 3.2.0, with debug_info, not stripped
```
  
```
$ file a-lib-project/target/release/liba_lib_project.rlib
a-lib-project/target/release/liba_lib_project.rlib: current ar archive
```
  
Ok, we know that the bin project has an ELF executable.  But what about the lib?  Is an ELF? The ```file``` command didn't really say.  

Let's use ```readelf -h``` to see what's in the lib project.  
  
Here's just part of the output:  
```
$ readelf -h a-lib-project/target/release/liba_lib_project.rlib

File: a-lib-project/target/release/liba_lib_project.rlib(lib.rmeta)
ELF Header:
  Magic:   7f 45 4c 46 02 01 01 00 00 00 00 00 00 00 00 00
  Class:                             ELF64
  Data:                              2's complement, little endian
  Version:                           1 (current)
  OS/ABI:                            UNIX - System V
  ABI Version:                       0
  Type:                              REL (Relocatable file)
  Machine:                           Advanced Micro Devices X86-64
  Version:                           0x1
```
  
So both the bin and lib projects compile to ELF binary files.
  
## A Bin + Lib  Project  
  
```
$ tree
.
├── Cargo.toml
└── src
    ├── lib.rs
    └── main.rs

1 directory, 3 files
```
  
The contents of Cargo.toml:
```
$ cat Cargo.toml
[package]
name = "a-bin-lib-project"
version = "0.1.0"
edition = "2021"

[lib]
name = "mylib"
path = "src/alib.rs"
```
  
The contents of src/alib.rs:
```
$ cat src/alib.rs
pub fn say_hello_from_lib() {
    println!("Hello world from from func in lib.rs!");
}
```
  
The contents of src/main.rs:
```
use mylib::*;

fn say_hello_from_main() {
    println!("Hello world from func in main.rs!");
}
fn main() {
  say_hello_from_main();
  say_hello_from_lib();
}
```
  

```
$ cargo run --release
   Compiling a-bin-lib-project v0.1.0 (/home/IamDeveloper/MySoftwareProjects/blockchain/rust/rust-substrate-blockchain-projects/my-first-substrate-projects/my-first-project-prep-lesson/learn-rust-modules-crates-pallets/a-bin-lib-project)
    Finished release [optimized] target(s) in 0.61s
     Running `target/release/a-bin-lib-project`
Hello world from func in main.rs!
Hello world from from func in lib.rs!
```
  
```
$ tree
.
├── Cargo.lock
├── Cargo.toml
├── src
│   ├── alib.rs
│   └── main.rs
└── target
    ├── CACHEDIR.TAG
    └── release
        ├── a-bin-lib-project  <----the executable ELF
        ├── a-bin-lib-project.d
        ├── build
        ├── deps
        │   ├── a_bin_lib_project-35a627ccaca2fd09
        │   ├── a_bin_lib_project-35a627ccaca2fd09.d
        │   ├── libmylib-e455fb1361384432.rlib
        │   ├── libmylib-e455fb1361384432.rmeta
        │   └── mylib-e455fb1361384432.d
        ├── examples
        ├── incremental
        ├── libmylib.d
        └── libmylib.rlib <-- the library ELF
```

## Modules  
  
### We create a new project, using the one above as a starting point.  
```
$ tree
.
├── Cargo.toml
└── src
    ├── alib.rs
    ├── main.rs
    └── module1.rs

1 directory, 4 files
```
  
Contents of Cargo.toml:
```
$ cat Cargo.toml
[package]
name = "a-bin-lib-project"
version = "0.1.0"
edition = "2021"

[lib]
name = "mylib"
path  = "src/alib.rs"
```
  
Contents of src/alib.rs:
```
$ cat src/alib.rs
pub mod module1;
```
  
Contents of src/module1.rs:
```
$ cat src/module1.rs
pub fn say_hello_from_module1() {
    println!("Hello world from from func in module1.rs!");
}
```
  
Contents of src/main.rs:
```
$ cat src/main.rs
use mylib::module1::*;

fn say_hello_from_main() {
    println!("Hello world from func in main.rs!");
}
fn main() {
  say_hello_from_main();
  say_hello_from_module1();
}
```
  
You see how we could keep adding several ```src/moduleXXX.rs``` files, and refer to them in the ```src/lib.rs`` file, and add similar ```use mylib::moduleXXX::*;``` in ```main.rs```.
  
### We run the project
```
$ cargo run --release
   Compiling a-bin-lib-project v0.1.0 (/home/IamDeveloper/MySoftwareProjects/blockchain/rust/rust-substrate-blockchain-projects/my-first-substrate-projects/my-first-project-prep-lesson/learn-rust-modules-crates-pallets/a-module-project)
    Finished release [optimized] target(s) in 0.51s
     Running `target/release/a-bin-lib-project`
Hello world from func in main.rs!
Hello world from from func in module1.rs!
```
  
What does the project contain now?
```
$ tree
.
├── Cargo.lock
├── Cargo.toml
├── src
│   ├── alib.rs
│   ├── main.rs
│   └── module1.rs
└── target
    ├── CACHEDIR.TAG
    └── release
        ├── a-bin-lib-project <--the executable ELF
        ├── a-bin-lib-project.d
        ├── build
        ├── deps
        │   ├── a_bin_lib_project-35a627ccaca2fd09
        │   ├── a_bin_lib_project-35a627ccaca2fd09.d
        │   ├── libmylib-e455fb1361384432.rlib
        │   ├── libmylib-e455fb1361384432.rmeta
        │   └── mylib-e455fb1361384432.d
        ├── examples
        ├── incremental
        ├── libmylib.d
        └── libmylib.rlib <-- the lib ELF
```
  
I will let you do ```readelf -a target/release/libmylib.rlib``` if you're interested.  
  

## Crates  
  
A crate is a compilation unit in Rustkcalled, ```some_file.rs``` is treated as the crate file.  
If ```some_file.rs``` has mod declarations in it, then the contents of the module files would be inserted in places where mod declarations in the crate file are found, before running the compiler over it.    
  
In other words, modules do not get compiled individually, only crates get compiled.  
  
A crate can be compiled into a binary or into a library.  
By default, ```rustc``` will produce a binary from a crate.  
This behavior can be overridden by passing the ```--crate-type``` flag to ```lib```.
