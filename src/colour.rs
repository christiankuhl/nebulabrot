use std::f64::consts::PI;
use clap::Values;

pub struct ColourFunctionArgs {
    pub channel_maxima: [u32; 3],
    pub boost: f64,
    pub atan_scale: f64,
    pub debug: bool
}

fn id(i: usize) -> usize { i }
fn rg(i: usize) -> usize { match i { 0 => 1, 1 => 0, _ => i } }
fn rb(i: usize) -> usize { match i { 0 => 2, 2 => 0, _ => i } }
fn gb(i: usize) -> usize { match i { 1 => 2, 2 => 1, _ => i } }

fn str_to_perm(name: &str) -> fn(usize) -> usize {
    match name {
        "RG" => rg,
        "RB" => rb,
        "GB" => gb,
        _ => id
    }
}

pub fn colour_function(name: &str) -> fn(usize, &u32, &ColourFunctionArgs) -> u8 {
    match name.to_lowercase().as_str() {
        "atan_scaled" => atan_scaled,
        "linear_capped" => linear_capped,
        "" => atan_scaled,
        _ => {
            println!("Colour function {} not found, defaulting to atan_scaled.", name.to_lowercase());
            atan_scaled
        }
    }
}

fn linear_capped(channel: usize, val: &u32, args: &ColourFunctionArgs) -> u8 {
    if *val as f64 > args.boost * 255.0 * (args.channel_maxima[channel] as f64) {
        return 255;
    } else {
        return (args.boost * *val as f64 / (args.channel_maxima[channel] as f64) * 255.0) as u8;
    }
}

fn atan_scaled(_channel: usize, val: &u32, args: &ColourFunctionArgs) -> u8 {
    return (255.0 * ((*val as f64) / args.atan_scale).atan() * 2.0 / PI) as u8;
}

pub fn permute_channels(pixel_data: &mut Box<Vec<u8>>, permutation_input: Option<Values>) {
    match permutation_input {
        Some(permutation_values) => {
            let permutations: Vec<fn(usize) -> usize> = permutation_values.into_iter().map(|n| str_to_perm(n)).collect();
            let orig_data = pixel_data.clone();
            let chunks = orig_data.chunks(3);
            for (chunk_index, chunk) in chunks.enumerate() {
                for (index, value) in chunk.into_iter().enumerate() {
                    let mut target_index = index;
                    for perm in &permutations {
                            target_index = perm(target_index)
                        }
                        pixel_data[3*chunk_index + target_index] = *value;
                }
            }
        },
        None => return
    }
}
