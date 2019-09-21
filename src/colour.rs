use std::f64::consts::PI;

pub struct ColourFunctionArgs {
    pub channel_maxima: [u32; 3],
    pub boost: f64,
    pub atan_scale: f64,
    pub debug: bool
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
