# The Iron Mind Interpreter
A BrainF*ck interpreter written in Rust

[![Crates.io](https://img.shields.io/crates/l/ironmind)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/d/ironmind)](https://crates.io/crates/ironmind)
[![Crates.io](https://img.shields.io/github/actions/workflow/status/jadens-arc/Iron-Mind/rust.yml)](https://github.com/Jadens-Arc/Iron-Mind)



[What is BrainF*ck](https://www.youtube.com/watch?v=hdHjjBS4cs8)

## Installation Instructions
### Cargo
Run `cargo install ironmind` to install the application from [crates.io](https://crates.io/).

## Usage
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

### Run Program

```shell
ironmind myfile.bf
```
Output:
```
Hello, World!
```

### Visualize Execution

```shell
ironmind -v myfile.bf
```
Output:
![visualizer](doc/visualizer.png)

## Build Instructions
### System Dependencies
- Rust
- Cargo

### Rust Dependencies (These install automatically)
- Clap for processing command line arguments and flags

### Run for Debugging / Testing
```shell
git clone https://github.com/Jadens-arc/Iron-Mind
cd Iron-Mind
cargo run myfile.bf
```
To run automated tests run
```shell
cargo test
```
Tests are found in ```src/tests```

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

