use clap::Parser;
use std::io::{self, Write};
use termcolor::{Color, ColorChoice, StandardStream};
mod color;
mod journal_add;
mod journal_fs;
mod journal_read;
mod notes;

#[derive(Parser)]
pub struct CLIArgs {
    journal_path: Option<String>,
}

fn main() {
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    let cli_args = CLIArgs::parse();
    color::set_color(&mut stdout, Color::White);
    journal_fs::init(&mut stdout, &cli_args.journal_path);
    writeln!(stdout, "Welcome to Tim's Journal Manager!");
    writeln!(stdout, "1. Add journal entry");
    writeln!(stdout, "2. Read your journal entries");
    writeln!(stdout, "3. Edit a journal entry");
    writeln!(stdout, "4. Delete a journal entry");
    let option: u8;
    loop {
        write!(stdout, "> ");
        stdout.flush().expect("unable to flush stdout");
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("error getting user input from stdin");
        match buffer.trim().parse::<u8>() {
            Ok(choice) => {
                if choice > 3 {
                    notes::error(&mut stdout, "number out of range");
                    continue;
                }
                option = choice;
                break;
            }
            Err(err) => {
                notes::error(&mut stdout, format!("{}", err).as_str());
                continue;
                // writeln!(stdout, "{}", err);
            }
        }
    }
    match option {
        1 => journal_add::add(&mut stdout, cli_args.journal_path),
        2 => journal_read::read(&mut stdout, &cli_args.journal_path),
        3 => {
            writeln!(stdout, "Edit one");
        }
        4 => {
            writeln!(stdout, "Delete one");
        }
        _ => {
            notes::error(&mut stdout, "This code is not meant to be reachable. If you are seeing this, please submit a bug report at: https://gitlab.com/TakitaTD/JournalManager");
        }
    }
}
