
mod engine;
mod cli;

use cli::Command;

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
        _ => return // TODO
    };
}


