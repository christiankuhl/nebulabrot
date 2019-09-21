# nebulabrot

A simple tool for generating the [Nebulabrot](https://en.wikipedia.org/wiki/Buddhabrot) visualisation of the [Mandelbrot set](https://en.wikipedia.org/wiki/Mandelbrot_set) in Rust.

![Nebulabrot visualisation](https://github.com/christiankuhl/nebulabrot/raw/master/nebulabrot.png "nebulabrot")

## Installation and usage

```
git clone https://github.com/christiankuhl/nebulabrot.git
cd nebulabrot
cargo build --release
cd target/release
```

### Invocation and command line options

```
USAGE:
    nebulabrot [OPTIONS] <--output <OUT_FILE>|--dump <DUMP_FILE>>

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --colour_function <COLOUR_FUNCTION>    Colouring function to use. One of atan_scaled, linear_capped.
    -d, --dump <DUMP_FILE>                     Dump iteration data to DUMP_FILE
    -h, --height <HEIGHT>                      Height of the output image
    -i, --input <IN_FILE>                      Get input from iteration dump IN_FILE
    -o, --output <OUT_FILE>                    Output png to OUT_FILE
    -w, --width <WIDTH>                        Width of the output image
```
