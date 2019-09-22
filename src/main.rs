use num::Complex;
use std::num::{ParseIntError, ParseFloatError};
use clap::{App, Arg, ArgGroup, crate_version, crate_authors};

pub mod file_io;
use file_io::{save_png, buffer_from_file, buffer_to_file};

pub mod colour;
use colour::{colour_function, ColourFunctionArgs};

pub mod plot;
use plot::PlotRange;

const RED: usize = 1000000;
const GREEN: usize = 100000;
const BLUE: usize = 10000;
const WIDTH: u32 = 2048;
const HEIGHT: u32 = 1536;
const X1: f64 = -19.0/9.0;
const Y1: f64 = 1.25;
const X2: f64 = 11.0/9.0;
const Y2: f64 = -1.25;
const BOOST: f64 = 4.0;
const ATAN_SCALE: f64 = 25.0;

fn main() {
    let matches = App::new("nebulabrot")
                          .version(crate_version!())
                          .author(crate_authors!())
                          .about("Renders the nebulabrot fractal as png")
                          .args_from_usage(
                              "-o, --output=[OUT_FILE] 'Output png to OUT_FILE'
                               -d, --dump=[DUMP_FILE]  'Dump iteration data to DUMP_FILE'")
                          .group(ArgGroup::with_name("out")
                                .args(&["output", "dump"])
                                .multiple(true)
                                .required(true))
                          .arg(Arg::from_usage("--x1=[X1] 'Real part of top left corner'")
                                .requires_all(&["y1", "x2", "y2"])
                                .allow_hyphen_values(true))
                          .arg(Arg::from_usage("--y1=[Y1] 'Imaginary part of top left corner'")
                                .allow_hyphen_values(true))
                          .arg(Arg::from_usage("--x2=[X2] 'Real part of bottom right corner'")
                                .allow_hyphen_values(true))
                          .arg(Arg::from_usage("--y2=[Y2] 'Imaginary part of bottom right corner'")
                                .allow_hyphen_values(true))
                          .arg(Arg::from_usage("-R, --red=[RED] 'Number of iterations for the red channel'")
                                .requires_all(&["green", "blue"]))
                          .args_from_usage("-G, --green=[GREEN] 'Number of iterations for the green channel'
                                            -B, --blue=[BLUE] 'Number of iterations for the blue channel'")
                          .arg(Arg::from_usage("-i, --input=[IN_FILE] 'Get input from iteration dump IN_FILE'")
                                .conflicts_with_all(&["dump", "red", "green", "blue"]))
                          .arg(Arg::from_usage("-h, --height=[HEIGHT] 'Height of the output image'")
                                .requires("width"))
                          .arg(Arg::from_usage("-c, --colour_function=[COLOUR_FUNCTION] 'Colouring function to use.'")
                                .possible_values(&["atan_scaled", "linear_capped"]))
                          .args_from_usage("-w, --width=[WIDTH] 'Width of the output image'
                                            -b, --boost=[BOOST] 'Colour boost factor for linear_capped'
                                            -s, --scale=[SCALE] 'Scaling factor for atan_scaled'
                                            --debug 'Display debug data'")
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
    let mut check_range: Vec<Result<f64, ParseFloatError>> = vec![Ok(X1), Ok(Y1), Ok(X2), Ok(Y2)];
    if matches.is_present("x1") {
        check_range = (vec!["x1", "y1", "x2", "y2"]).iter().map(
                        |n| matches.value_of(n).unwrap_or_default().parse::<f64>()).collect()
    }
    if check_range.iter().any(|v| v.is_err()) {
        println!("Please specify valid plot boundaries. Reverting to defaults.");
        check_range = vec![Ok(X1), Ok(Y1), Ok(X2), Ok(Y2)];
    }
    let result_range: Result<Vec<f64>, ParseFloatError> = check_range.into_iter().collect();
    let range = result_range.unwrap();
    let top_left = Complex {re: range[0], im: range[1]};
    let bottom_right = Complex {re: range[2], im: range[3]};
    let mut plot_range = PlotRange {top_left: top_left,
                                    bottom_right: bottom_right,
                                    buffer: buffer,
                                    output_width: width,
                                    output_height: height};
    if calculate {
        let mut check_iterations: Vec<Result<usize, ParseIntError>> = vec![Ok(RED), Ok(GREEN), Ok(BLUE)];
        if matches.is_present("red") {
            check_iterations = (vec!["red", "green", "blue"]).iter().map(
                                |n| matches.value_of(n).unwrap_or_default().parse::<usize>())
                                .collect();
        }
        if check_iterations.iter().any(|v| v.is_err()) {
            println!("Please specify valid iteration counts for red, green and blue. Reverting to defaults.");
            check_iterations = vec![Ok(RED), Ok(GREEN), Ok(BLUE)];
        }
        let max_iterations: Result<Vec<usize>, ParseIntError> = check_iterations.into_iter().collect();
        plot_range.iterate(max_iterations.unwrap());
    }
    if let Some(dump_file) = matches.value_of("dump") {
        println!("Dumping iteration data to {}...", dump_file);
        buffer_to_file(dump_file, &plot_range.buffer);
    }
    if let Some(output_file) = matches.value_of("output") {
        println!("Calculating png data...");
        let col_func = colour_function(matches.value_of("colour_function").unwrap_or_default());
        let mut args = ColourFunctionArgs {
            channel_maxima: [0; 3],
            boost: matches.value_of("boost").unwrap_or_default().parse::<f64>().unwrap_or(BOOST),
            atan_scale: matches.value_of("scale").unwrap_or_default().parse::<f64>().unwrap_or(ATAN_SCALE),
            debug: matches.is_present("debug")
        };
        let pixel_data = plot_range.renormalize(col_func, &mut args);
        save_png(output_file, &pixel_data, width, height);
    }
    println!("Done.");
}
