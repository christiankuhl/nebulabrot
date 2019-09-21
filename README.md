# nebulabrot

A simple tool for plotting the Nebulabrot visualisation of the Mandelbrot set in Rust.

![alt text](https://github.com/christiankuhl/mandelbrot-rs/raw/master/nebulabrot.png "nebulabrot")

## Installation and usage

```
git clone https://github.com/christiankuhl/nebulabrot.git
cd nebulabrot
cargo build --release
cd target/release
```

### Invocation and command line options

```
nebulabrot [OPTIONS] <--output <OUT_FILE>|--dump <DUMP_FILE>|--width <WIDTH>|--height <HEIGHT>>

FLAGS:
    --help       Prints help information
-V, --version    Prints version information

OPTIONS:
-d, --dump <DUMP_FILE>     Dump iteration data to [DUMP_FILE]
-h, --height <HEIGHT>      Height of the output image
-i, --input <IN_FILE>      Get input from iteration dump [IN_FILE]
-o, --output <OUT_FILE>    Output png to [OUT_FILE]
-w, --width <WIDTH>        Width of the output image
```
