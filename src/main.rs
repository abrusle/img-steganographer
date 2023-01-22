

fn main() {
    let all_commands = ["help", "encode", "decode"];
    let args: Vec<String> = std::env::args().collect();

    let options = args.get(1);

    let command = match options {
        None => {
            print_help();
            return;
        },
        Some(cmd) => {
            if cmd == "help" {
                print_help();
                return;
            }

            if !all_commands.contains(&cmd.as_str()) {
                println!("Unknown command {}", cmd);
                return;
            }

            cmd
        }
    };

    print!("running command {}", command);
}

fn print_help()
{
    println!("\
Available commands:\n\
\t help                                          Shows the help message.\n\
\t encode [-r | --repeat] <message> <file-path>  Encodes the given message into the image.\n\
\t decode <file-path>                            Decodes a message from the image.\n\
\n\
Supported image formats: .png");
}
