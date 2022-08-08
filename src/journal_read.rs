use crate::journal_fs;
use serde_json;
use std::fs::OpenOptions;
use std::io::{Read, Write};
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
    let journal_entries = serde_json::from_str::<journal_fs::JournalEntry>(&file_string.trim())
        .expect("unable to deserialise file");
    writeln!(stdout, "{:?}", journal_entries);
}
