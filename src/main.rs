
mod engine;
mod cli;

use cli::Command;
use std::path::Path;

fn main() {

    match cli::start() {
        Ok(cmd) => run_command(&cmd),
        Err(msg) => {
            println!("{}", msg);
            return;
        }
    }
}

fn run_command(cmd: &cli::Command) {
    match cmd {
        Command::Help => cli::print_help(),
        Command::Decode(file_path) => {
            let path = Path::new(file_path);
            match check_image_path(path) {
                Ok(_) => engine::decoding::run(path),
                Err(msg) => println!("{}", msg)
            }
        },

        Command::Encode(repeat, message, file_path) => {
            
            if message.len() == 0 {
                println!("Message cannot be empty.");
                return;
            }
            
            let path = Path::new(file_path);
            match check_image_path(path) {
                Ok(_) => engine::encoding::run(*repeat, message, path),
                Err(msg) => println!("{}", msg),
            }
        }
    };
}

fn check_image_path(path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Result::Err(String::from("File does not exist."))
    }

    match path.extension() {
        None => Result::Err(String::from("Unknown file extension")),
        Some(extension) => {
            if engine::is_img_supported(extension.to_str().unwrap_or_default()) {
                Result::Ok(())
            } else {
                let mut err_msg = String::from("Unsupported file extension '");
                err_msg.push_str(extension.to_str().unwrap_or_default());
                err_msg.push_str("'.");
                Result::Err(err_msg)
            }
        }
    }

}
