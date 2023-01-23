use std::path::Path;
use std::fs::File;

pub fn run(repeat: bool, message: &String, file_path: &Path) {
    println!("TODO : encoding image at {:?} with message (repeat={}) '{}'", file_path, repeat, message);

    let file = File::open(file_path).expect("Failed openning file.");
    let decoder = png::Decoder::new(file);
    let mut reader = decoder.read_info().expect("Failed getting decoder reader.");

    let row = match reader.next_row() {
        Ok(r) => r,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    match row {
        None => {},
        Some(r) => {
            const MAX_INDEX: i32 = 16;
            let mut index = 0;
            for b in r.data() {
                println!("{:#04x}", b);
                index += 1;
                if index >= MAX_INDEX {
                    break;
                }
            }
        }
    }

}