use image::{codecs::png::PngEncoder, ColorType, ImageEncoder};
use std::fs::File;
use byteorder::{ReadBytesExt, WriteBytesExt, NativeEndian};

pub fn save_png(file_name: &str, pixel_data: &Box<Vec<u8>>, width: u32, height: u32){
    println!("Saving png to {}...", file_name);
    let file_handle = File::create(file_name).expect("Error opening file.");
    let png = PngEncoder::new(file_handle);
    png.write_image(&pixel_data[..], width, height, ColorType::Rgb8).expect("Error encoding png.");
}


pub fn buffer_to_file(file_name: &str, buffer: &Box<Vec<u32>>) {
    let mut file_handle = File::create(file_name).expect("Error opening file.");
    for count in buffer.iter() {
        file_handle.write_u32::<NativeEndian>(*count).expect("Error writing to file.");
    }
}

pub fn buffer_from_file(file_name: &str, pixels: usize) -> Box<Vec<u32>> {
    let mut buffer: Box<Vec<u32>> = Box::new(Vec::with_capacity(3*pixels));
    for _ in 0..3*pixels {
        buffer.push(0);
    }
    let mut file_handle = File::open(file_name).expect("Error opening file.");
    file_handle.read_u32_into::<NativeEndian>(&mut buffer).unwrap();
    buffer
}
