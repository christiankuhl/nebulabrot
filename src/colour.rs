use std::f64::consts::PI;

const BOOST: f64 = 4.0;
const ATAN_SCALE: f64 = 25.0;

pub fn colour_function(name: &str) -> fn(usize, &u32, &[u32; 3]) -> u8 {
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

fn linear_capped(channel: usize, val: &u32, channel_maxima: &[u32; 3]) -> u8 {
    if *val as f64 > BOOST * 255.0 * (channel_maxima[channel] as f64) {
        return 255;
    } else {
        return (4.0 * *val as f64 / (channel_maxima[channel] as f64) * 255.0) as u8;
    }
}

fn atan_scaled(_channel: usize, val: &u32, _channel_maxima: &[u32; 3]) -> u8 {
    return (255.0 * ((*val as f64) / ATAN_SCALE).atan() * 2.0 / PI) as u8;
}
