use std::path::Path;
use std::fs::File;
use png::{Decoder, Reader};

pub fn run(file_path: &Path) {
    
    let buffer = extract_data(file_path).0;

    let text_bytes: Vec<u8> = buffer.chunks_exact(8)
        .map(|chunk| {
            let mut byte = 0;
            for c in chunk {
                let mut c = c << 7;
                c >>= 7;
                byte = byte << 1 | c;
            }
            byte
        }).collect();

    println!("{}", String::from_utf8_lossy(&text_bytes));
}

pub fn extract_data(file_path: &Path) -> (Vec<u8>, Reader<File>) {
    let file = File::open(file_path).unwrap();
    let decoder = Decoder::new(file);

    let mut reader = decoder.read_info().unwrap();

    // Allocate a buffer to hold the decoded pixel dataa
    let mut buffer = vec![0; reader.output_buffer_size()];
    
    // Decode to buffer
    reader.next_frame(&mut buffer).unwrap();

    (buffer, reader)
}