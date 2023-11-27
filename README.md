# The Iron Mind Interpreter
A BrainF*ck interpreter written in Rust

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[What is BrainF*ck](https://www.youtube.com/watch?v=hdHjjBS4cs8)

## Installation Instructions
### Cargo
Run `cargo install ironmind` to install the application from [crates.io](https://crates.io/).

## Example
Sample program:
```brainfuck
myfile.bf
++++++++++++[>++++++<-]>.           H
>++++++++++[>++++++++++<-]>+.       e
>+++++++++[>++++++++++++<-]>..      l (printed twice)
>++++++++++[>+++++++++++<-]>+.      o
>++++[>+++++++++++<-]>.             (comma)
>++++[>++++++++<-]>.                (space)
>++++++++[>+++++++++++<-]>-.        W
>++++++++++[>+++++++++++<-]>+.      o
>++++++++++[>+++++++++++<-]>++++.   r
>+++++++++[>++++++++++++<-]>.       l
>++++++++++[>++++++++++<-]>.        d
>++++[>++++++++<-]>+.               (exclamation)
>+++[>+++<-]>+.                     (new line)
```
Run Using:
```shell
ironmind myfile.bf
```
Output:
```
Hello, World!
```

## Build Instructions
### Prerequisite Dependencies
- Rust
- Cargo

### Run for Debugging / Testing
```shell
git clone https://github.com/Jadens-arc/Iron-Mind
cd Iron-Mind
cargo run myfile.bf
```

### Compiling Executable
```shell
git clone https://github.com/Jadens-arc/Iron-Mind
cd Iron-Mind
cargo build --release
```
The executable will be found at ```target/release/ironmind```

Run using ```./ironmind```

Feel free to move it to a ```Bin/``` directory if you want

If you do move it, you can use   ```ironmind myfile.bf``` anywhere on your system


Also, I found [this really cool](http://www.99-bottles-of-beer.net/language-brainfuck-101.html) BrainF*ck program from 2005 that outputs 99 Bottles of Beer by Andrew Paczkowski

Definitely worth checking out

