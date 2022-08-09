use crate::journal_fs::{self, JournalData, JournalEntry};
use crate::notes;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use serde_json;
use std::fs::OpenOptions;
use std::io::{self, Read, Write};
use std::path::Path;
use termcolor::StandardStream;

pub fn read(stdout: &mut StandardStream, custom_dir: &Option<String>) {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_fs::get_journal_file(stdout, custom_dir))
        .expect("unable to read file");
    let mut file_string = String::new();
    file.read_to_string(&mut file_string)
        .expect("cannot read file to string");

    let journal_entries =
        serde_json::from_str::<Vec<JournalData>>(&file_string).expect("unable to deserialise file");
    let mut count: u16 = 0;
    for journal in &journal_entries {
        count += 1;
        write!(stdout, "   {count}. {} ", journal.title);
        write!(stdout, ", {}", journal.created);

        if journal.encrypted {
            write!(stdout, ", (encrypted)\n");
        } else {
            write!(stdout, "\n");
        }
    }
    writeln!(stdout, "\n   Select entry to read:");
    let mut selected_entry: u16 = 0;

    loop {
        write!(stdout, "   > ");
        stdout.flush().expect("unable to flush stdout");
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("error getting user input from stdin");
        match buffer.trim().parse::<u16>() {
            Ok(choice) => {
                if choice > count {
                    notes::error(stdout, "number out of range");
                    continue;
                }
                selected_entry = choice;
                break;
            }
            Err(err) => {
                notes::error(stdout, format!("{}", err).as_str());
                continue;
                // writeln!(stdout, "{}", err);
            }
        }
    }
    let mut journal_file = &journal_entries[(selected_entry - 1) as usize];
    let path = format!(
        "{}/{}.json",
        journal_fs::get_journal_dir(stdout, custom_dir),
        journal_file.id
    );
    let mut journal_file = OpenOptions::new()
        .read(true)
        .open(format!(
            "{}/{}.json",
            journal_fs::get_journal_dir(stdout, custom_dir),
            journal_file.id
        ))
        .expect("unable to read file");
    let mut journal_data = String::new();
    journal_file
        .read_to_string(&mut journal_data)
        .expect("unable to read file to string");
    let journal_data =
        serde_json::from_str::<JournalEntry>(&journal_data).expect("unable to deserialise string");
    writeln!(
        stdout,
        "Title: {}, Created At: {}",
        journal_data.title, journal_data.created
    );
    if journal_data.encrypted {
        write!(stdout, "Uh oh! Looks like this file is encrypted!\n");
        let mut decrypted_text = String::new();
        loop {
            print!("Enter key: ");
            stdout.flush().expect("unable to flush stdout");
            let mut encryption_key = String::new();
            io::stdin()
                .read_line(&mut encryption_key)
                .expect("unable to read input from stdin");
            let magic = new_magic_crypt!(encryption_key.trim(), 256);

            match magic.decrypt_base64_to_string(&journal_data.content) {
                Ok(text) => {
                    writeln!(stdout, "Content:");
                    let mut current_line: u32 = 0;
                    for line in text.lines() {
                        current_line += 1;
                        writeln!(stdout, "{}    {}", current_line, line);
                    }
                    break;
                }
                Err(err) => {
                    notes::error(stdout, "YOU SHALL NOT PASS");
                    continue;
                }
            };
        }
        return;
    }
    writeln!(stdout, "Content:");
    let mut current_line: u32 = 0;
    for line in journal_data.content.split("\n") {
        current_line += 1;
        writeln!(stdout, "{}    {}", current_line, line);
    }
    // writeln!(stdout, "{:?}", journal_entries[1]);
}
