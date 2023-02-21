use std::io::BufWriter;
use std::path::Path;
use std::fs::File;

use png::Encoder;

pub fn run(repeat: bool, message: &String, input_path: &Path, output_path: &Path) {
    println!("Encoding image at {:?} with message (repeat={}) '{}'", input_path, repeat, message);

    let (mut pixels, reader) = super::decoding::extract_data(input_path);

    encode_message(&mut pixels, message);

    let output_file = File::create(output_path).unwrap();
    let w = &mut BufWriter::new(output_file);

    let info = reader.info();
    let (width, height) = info.size();
    let mut encoder = Encoder::new(w, width, height);
    encoder.set_color(info.color_type);
    encoder.set_depth(info.bit_depth);

    if let Some(source_gamma) = info.source_gamma {
        encoder.set_source_gamma(source_gamma);
    }

    if let Some(source_chromaticities) = info.source_chromaticities {
        encoder.set_source_chromaticities(source_chromaticities);
    }

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&pixels).unwrap();

    println!("Done.");
}

pub fn encode_message(bytes: &mut [u8], message: &String) {

    // TODO there is definitely a better way of doing this.

    let mut message_bytes = Vec::new();
    for c in message.as_bytes() {
        message_bytes.push(apply_mask(*c, 0b01111111, 7));
        message_bytes.push(apply_mask(*c, 0b10111111, 6));
        message_bytes.push(apply_mask(*c, 0b11011111, 5));
        message_bytes.push(apply_mask(*c, 0b11101111, 4));
        message_bytes.push(apply_mask(*c, 0b11110111, 3));
        message_bytes.push(apply_mask(*c, 0b11111011, 2));
        message_bytes.push(apply_mask(*c, 0b11111101, 1));
        message_bytes.push(apply_mask(*c, 0b11111110, 0));

        fn apply_mask(cc: u8, mask: u8, index: usize) -> u8 {
            ((cc | mask) & !mask) >> index
        }
    }


    let message_length = message_bytes.len();
    let bitmask_lastbit: u8 = 0b0000001;

    for (i, px) in bytes.iter_mut().enumerate() {
        let character_data = message_bytes[i % message_length];
        *px = ((*px | bitmask_lastbit) & !bitmask_lastbit) | character_data;
    }
}