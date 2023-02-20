use std::io::BufWriter;
use std::path::Path;
use std::fs::File;

use png::Encoder;

pub fn run(repeat: bool, message: &String, input_path: &Path, output_path: &Path) {
    println!("TODO : encoding image at {:?} with message (repeat={}) '{}'", input_path, repeat, message);

    let (mut pixels, reader) = super::decoding::extract_data(input_path);

    let message_bytes = message.as_bytes();
    let message_length = message_bytes.len();

    for (i, p) in pixels.iter_mut().enumerate() {
        *p |= (message_bytes[i % message_length] << 7) >> 7;
    }


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