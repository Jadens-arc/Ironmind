# The Iron Mind Interpreter
A BrainF*ck interpreter written in Rust


## Example
Sample program:
```brainfuck
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
Output:
```
Hello, World!
```

## Build Instructions
### Build Dependencies
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
The executable will be found at ``` target/release/ironmind```

Run using ```./ironmind```

Feel free to move it to a ```Bin/``` directory if you want

If you do move it, you can use   ```ironmind myfile.bf``` anywhere on your system


Also, I found [this really cool](http://www.99-bottles-of-beer.net/language-brainfuck-101.html) BrainF*ck program that outputs 99 Bottles of Beer by Andrew Paczkowski

Definitely worth checking out

