use num::Complex;
use clap::{App, Arg, ArgGroup};

pub mod file_io;
use file_io::{save_png, buffer_from_file, buffer_to_file};

pub mod colour;
use colour::colour_function;

pub mod plot;
use plot::PlotRange;

const MAX_ITERATIONS: [usize; 3] = [5000, 500, 50];
const WIDTH: u32 = 2048;
const HEIGHT: u32 = 1536;
const TOP_LEFT: Complex<f64> = Complex {re: -19.0/9.0, im: 1.25};
const BOTTOM_RIGHT: Complex<f64> = Complex {re: 11.0/9.0, im: -1.25};

fn main() {
    let matches = App::new("nebulabrot")
                          .version("1.0")
                          .author("Christian Kuhl <christian.kuhl84@gmail.com>")
                          .about("Renders the nebulabrot fractal as png")
                          .args_from_usage(
                              "-o, --output=[OUT_FILE] 'Output png to OUT_FILE'
                               -d, --dump=[DUMP_FILE]  'Dump iteration data to DUMP_FILE'")
                          .group(ArgGroup::with_name("out")
                                .args(&["output", "dump"])
                                .multiple(true)
                                .required(true))
                          .arg(Arg::from_usage("-i, --input=[IN_FILE] 'Get input from iteration dump IN_FILE'")
                                .conflicts_with("dump"))
                          .arg(Arg::from_usage("-h, --height=[HEIGHT] 'Height of the output image'")
                                .requires("width"))
                          .arg_from_usage("-w, --width=[WIDTH] 'Width of the output image'")
                          .arg_from_usage("-c, --colour_function=[COLOUR_FUNCTION] 'Colouring function to use. One of atan_scaled, linear_capped.'")
                          .get_matches();

    let height = matches.value_of("height").unwrap_or_default().parse::<u32>().unwrap_or(HEIGHT);
    let width = matches.value_of("width").unwrap_or_default().parse::<u32>().unwrap_or(WIDTH);
    let pixels = (height * width) as usize;
    let buffer: Box<Vec<u32>>;
    let mut calculate: bool = true;
    if let Some(input_file) = matches.value_of("input") {
        println!("Reading input from {}...", input_file);
        buffer = buffer_from_file(input_file, pixels);
        calculate = false;
    } else {
        buffer = Box::new(Vec::with_capacity(3*pixels));
    }
    let mut plot_range = PlotRange {top_left: TOP_LEFT,
                                    bottom_right: BOTTOM_RIGHT,
                                    buffer: buffer,
                                    output_width: width,
                                    output_height: height};
    if calculate {
        plot_range.iterate(MAX_ITERATIONS);
    }
    if let Some(dump_file) = matches.value_of("dump") {
        println!("Dumping iteration data to {}...", dump_file);
        buffer_to_file(dump_file, &plot_range.buffer);
    }
    if let Some(output_file) = matches.value_of("output") {
        println!("Calculating png data...");
        let col_func = colour_function(matches.value_of("colour_function").unwrap_or_default());
        let pixel_data = plot_range.renormalize(col_func);
        save_png(output_file, &pixel_data, width, height);
    }
    println!("Done.");
}
