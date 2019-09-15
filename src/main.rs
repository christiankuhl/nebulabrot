use num::Complex;
use image::{png::PNGEncoder, ColorType};
use std::fs::File;
use std::f64::consts::PI;
use byteorder::{ReadBytesExt, WriteBytesExt, NativeEndian};
use clap::App;

const MAX_ITERATIONS: [usize; 3] = [100000, 10000, 1000];
const WIDTH: u32 = 2048;
const HEIGHT: u32 = 1192;
const PIXELS: usize = (HEIGHT * WIDTH) as usize;
const ATAN_SCALE: f64 = 10.0;

fn main() {
    let matches = App::new("nebulabrot")
                          .version("1.0")
                          .author("Christian Kuhl <christian.kuhl84@gmail.com>")
                          .about("Renders the nebulabrot fractal as png")
                          .args_from_usage(
                              "-o, --output=[OUT_FILE] 'Output png to [OUT_FILE]'
                               -i, --input=[IN_FILE]   'Get input from iteration dump [IN_FILE]'
                               -d, --dump=[DUMP_FILE]  'Dump iteration data to [DUMP_FILE]'")
                          .get_matches();

    let mut buffer: Box<Vec<u32>>;
    let mut calculate: bool = true;
    if let Some(input_file) = matches.value_of("input") {
        println!("Reading input from {}...", input_file);
        buffer = buffer_from_file(input_file);
        calculate = false;
    } else {
        buffer = Box::new(Vec::with_capacity(3*PIXELS));
    }
    let mut plot_range = PlotRange { top_left: Complex {re: -20.0/9.0-0.5, im: 1.25},
                                    bottom_right: Complex {re: 11.0/9.0, im: -1.25},
                                    buffer: buffer};
    if calculate {
        println!("Calculating iterations...");
        plot_range.iterate(MAX_ITERATIONS);
    }
    if let Some(dump_file) = matches.value_of("dump") {
        println!("Dumping iteration data to {}...", dump_file);
        buffer_to_file(dump_file, &plot_range.buffer);
    }
    if let Some(output_file) = matches.value_of("output") {
        println!("Calculating png data...");
        let pixel_data = plot_range.renormalize();
        save_png(output_file, &pixel_data);
    }
    println!("Done.");
}

fn save_png(file_name: &str, pixel_data: &Box<Vec<u8>>){
    println!("Saving png to {}...", file_name);
    let file_handle = File::create(file_name).expect("Error opening file.");
    let png = PNGEncoder::new(file_handle);
    png.encode(&pixel_data[..], WIDTH, HEIGHT, ColorType::RGB(8)).expect("Error encoding png.");
}


fn buffer_to_file(file_name: &str, buffer: &Box<Vec<u32>>) {
    let mut file_handle = File::create(file_name).expect("Error opening file.");
    for count in buffer.iter() {
        file_handle.write_u32::<NativeEndian>(*count).expect("Error writing to file.");
    }
}

fn buffer_from_file(file_name: &str) -> Box<Vec<u32>> {
    let mut buffer: Box<Vec<u32>> = Box::new(Vec::with_capacity(3*PIXELS));
    for _ in 0..3*PIXELS {
        buffer.push(0);
    }
    let mut file_handle = File::open(file_name).expect("Error opening file.");
    file_handle.read_u32_into::<NativeEndian>(&mut buffer).unwrap();
    buffer
}



fn in_mandelbrot_set(c: &Complex<f64>) -> bool {
   (c.re >  -1.2 && c.re <=  -1.1 && c.im >  -0.1 && c.im < 0.1)
|| (c.re >  -1.1 && c.re <=  -0.9 && c.im >  -0.2 && c.im < 0.2)
|| (c.re >  -0.9 && c.re <=  -0.8 && c.im >  -0.1 && c.im < 0.1)
|| (c.re > -0.69 && c.re <= -0.61 && c.im >  -0.2 && c.im < 0.2)
|| (c.re > -0.61 && c.re <=  -0.5 && c.im > -0.37 && c.im < 0.37)
|| (c.re >  -0.5 && c.re <= -0.39 && c.im > -0.48 && c.im < 0.48)
|| (c.re > -0.39 && c.re <=  0.14 && c.im > -0.55 && c.im < 0.55)
|| (c.re >  0.14 && c.re <   0.29 && c.im > -0.42 && c.im < -0.07)
|| (c.re >  0.14 && c.re <   0.29 && c.im >  0.07 && c.im < 0.42)
}

struct PlotRange {
    top_left: Complex<f64>,
    bottom_right: Complex<f64>,
    buffer: Box<Vec<u32>>
}

impl PlotRange {
    pub fn renormalize(&mut self) -> Box<Vec<u8>> {
        let mut result = Box::new(Vec::with_capacity(3*PIXELS));
        for _ in 0..3*PIXELS {
            result.push(0);
        }
        let mut channel_maxima: [u32; 3] = [0, 0, 0];
        for channel in 0..3 {
            let max = self.buffer.iter().enumerate().max_by_key(|&(i, v)| if i % 3 == channel {*v} else {0}).unwrap();
            channel_maxima[channel] = *max.1;
        }
        println!("{:?}", channel_maxima);
        for (index, val) in self.buffer.iter().enumerate() {
            // if *val > 255 {
            //     result[index] = 255; }
            // else {
            //     result[index] = *val as u8;
            // }
            result[index] = (255.0 * ((*val as f64) / ATAN_SCALE).atan() * 2.0 / PI) as u8;
        }
        result
    }
    fn index_to_point(&self, index: &usize) -> Complex<f64> {
        Complex {re: ((*index % (WIDTH as usize)) as f64) / ((WIDTH - 1) as f64)
                        * self.width() + self.top_left.re,
                 im: self.top_left.im -(((*index / (WIDTH as usize)) as f64).ceil()) / ((HEIGHT - 1) as f64)
                                                                                    * self.height()}
    }
    fn point_to_index(&self, point: &Complex<f64>) -> Option<usize> {
        let index_f: f64 = ((self.top_left.im - point.im) / self.height() * ((HEIGHT - 1) as f64)).floor() * (WIDTH as f64)
                                    + (point.re - self.top_left.re) / self.width() * ((WIDTH - 1) as f64);
        if index_f < 0.0 || index_f >= PIXELS as f64 {
            return None
        } else {
            return Some(index_f as usize)
        }
    }
    fn height(&self) -> f64 {
        self.top_left.im - self.bottom_right.im
    }
    fn width(&self) -> f64 {
        self.bottom_right.re - self.top_left.re
    }
    pub fn iterate(&mut self, max_iterations: [usize; 3]) {
        for _ in 0..3*PIXELS {
            self.buffer.push(0);
        }
        let iteration_limit = max_iterations.iter().max().unwrap();
        for index in 0..PIXELS {
            if index % 50000 == 0 {
                println!("{:.2}% complete...", 100.0 * (index as f32) / (PIXELS as f32));
            }
            let c = self.index_to_point(&(index as usize));
            if in_mandelbrot_set(&c) { continue }
            let mut z = Complex {re: 0.0, im: 0.0};
            let mut tr: Vec<usize> = Vec::with_capacity(*iteration_limit);
            for iter_count in 0..*iteration_limit {
                z = z*z + c;
                if let Some(idx) = self.point_to_index(&z) {tr.push(idx)};
                if z.norm_sqr() > 4.0 {
                    for idx in tr.iter() {
                        for (channel, iterations) in max_iterations.iter().enumerate() {
                            if *iterations >= iter_count {
                                self.buffer[3 * idx + channel] += 1;
                            }
                        }
                    }
                    break;
                }
            }
        }
    }
}
