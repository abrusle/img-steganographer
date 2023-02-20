// Command-Line Interface

pub enum Command
{
    Help,
    Encode(bool, String, String, String),
    Decode(String)
}

pub fn start() -> Result<Command, String>
{
    let args: Vec<String> = std::env::args().collect();

    let cmd_string = args.get(1);

    if cmd_string.is_none() {
        return Result::Ok(Command::Help)
    }

    let cmd_string = cmd_string.unwrap().as_str();

    return match cmd_string {
        "help" => Result::Ok(Command::Help),

        "encode" => {
            let arg1 = args.get(2);
            if arg1.is_none() {
                Result::Err(String::from("Missing required arguments for message and file path."))
            } else {
                let repeat_or_msg = arg1.unwrap();
                let repeat = repeat_or_msg == "-r" || repeat_or_msg == "--repeat";
                let message = if repeat {args.get(3)} else {Some(repeat_or_msg)};
                if message.is_none() {
                    Result::Err(String::from("Missing required argument for message."))
                } else {
                    let mut index = if repeat {4} else {3};
                    let file_path = args.get(index);
                    if file_path.is_none() {
                        Result::Err(String::from("Missing required argument for input file path."))
                    } else {
                        index += 1;
                        if let Some(output_path) = args.get(index) {
                            Result::Ok(Command::Encode(repeat, message.unwrap().to_owned(), file_path.unwrap().to_owned(), output_path.to_owned()))
                        } else {
                            Result::Err(String::from("Missing required argument for output file."))
                        }
                    }
                }
            }
        },

        "decode" => {
            match args.get(2) {
                None => Result::Err(String::from("Missing required argument for file path.")),
                Some(file_path) => Result::Ok(Command::Decode(file_path.to_owned()))
            }
        },

        c => {
            let mut err_msg = String::from("Unkown command ");
            err_msg.push_str(&c);
            Result::Err(err_msg)
        }
    };
}

pub fn print_help()
{
    println!("\
Available commands:\n\
\t help                                                                   Shows the help message.\n\
\t encode [-r | --repeat] <message> <input-file-path> <output-file-path>  Encodes the given message into the image.\n\
\t decode <file-path>                                                     Decodes a message from the image.\n\
\n\
Supported image formats: .png");
}