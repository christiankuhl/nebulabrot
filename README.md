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
    nebulabrot [FLAGS] [OPTIONS] <--output <OUT_FILE>|--dump <DUMP_FILE>>

FLAGS:
        --debug      Display debug data
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -B, --blue <BLUE>                          Number of iterations for the blue channel
    -b, --boost <BOOST>                        Colour boost factor for linear_capped
    -c, --colour_function <COLOUR_FUNCTION>    Colouring function to use. [possible values: atan_scaled, linear_capped]
    -d, --dump <DUMP_FILE>                     Dump iteration data to DUMP_FILE
    -G, --green <GREEN>                        Number of iterations for the green channel
    -h, --height <HEIGHT>                      Height of the output image
    -i, --input <IN_FILE>                      Get input from iteration dump IN_FILE
    -o, --output <OUT_FILE>                    Output png to OUT_FILE
    -R, --red <RED>                            Number of iterations for the red channel
    -s, --scale <SCALE>                        Scaling factor for atan_scaled
    -w, --width <WIDTH>                        Width of the output image
        --x1 <X1>                              Real part of top left corner
        --x2 <X2>                              Real part of bottom right corner
        --y1 <Y1>                              Imaginary part of top left corner
        --y2 <Y2>                              Imaginary part of bottom right corner
```
