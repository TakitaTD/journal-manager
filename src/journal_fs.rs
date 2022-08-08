use crate::notes;
use dirs;
use serde::{Deserialize, Serialize};
use std::{
    fs::{create_dir_all, File, OpenOptions},
    io::Write,
    path::Path,
};
use termcolor::StandardStream;
use users;

#[derive(Serialize, Deserialize, Debug)]
pub struct JournalEntry {
    pub title: String,
    pub content: String,
    pub encrypted: bool,
    pub created: String,
    pub updated: String,
}
impl JournalEntry {
    pub fn new(title: &str, content: &str, encrypted: bool) -> JournalEntry {
        return JournalEntry {
            title: String::from(title),
            content: String::from(content),
            encrypted,
            created: chrono::offset::Local::now().to_string(),
            updated: chrono::offset::Local::now().to_string(),
        };
    }
}

pub fn get_journal_dir(stdout: &mut StandardStream, custom_dir: &Option<String>) -> String {
    match custom_dir {
        Some(dir) => {
            return String::from(dir);
        }
        None => match dirs::home_dir() {
            Some(dir) => {
                return format!(
                    "{}/.journals",
                    dir.as_os_str().to_str().unwrap().to_string()
                )
            }
            None => {
                notes::note(stdout, "home directory not found, assuming unix");
                format!("/home/{}/.journals", users::get_current_username().expect("user must have been deleted while running application, run with a good user next time dude.").to_string_lossy())
            }
        },
    }
}
pub fn get_journal_file(stdout: &mut StandardStream, custom_dir: &Option<String>) -> String {
    return format!("{}/journals.json", get_journal_dir(stdout, custom_dir));
}
pub fn save_journal(
    stdout: &mut StandardStream,
    journals: JournalEntry,
    custom_dir: &Option<String>,
) {
    let journal_file = get_journal_file(stdout, custom_dir);
    let journal_file = Path::new(&journal_file);
    let mut journal_file = OpenOptions::new()
        .write(true)
        .read(true)
        .open(journal_file)
        .unwrap();
    journal_file
        .set_len(0)
        .expect("error setting length of file");
    journal_file
        .write_all(
            serde_json::to_string(&journals)
                .expect("error when serializing object")
                .as_bytes(),
        )
        .expect("error when writing to file");
}
pub fn init(stdout: &mut StandardStream, custom_dir: &Option<String>) {
    let journal_dir = get_journal_dir(stdout, custom_dir);
    let journal_dir = Path::new(&journal_dir);
    let journal_file = get_journal_file(stdout, custom_dir);
    let journal_file = Path::new(&journal_file);
    if journal_dir.is_file() {
        notes::error(stdout, "it seems that a file exists where your journal directory is supposed to be. This means I cannot operate correctly.");
        panic!("it seems that a file exists where your journal directory is supposed to be. This means I cannot operate correctly.");
    }
    if !journal_dir.exists() {
        notes::note(
            stdout,
            format!("creating {}/... ", journal_dir.display()).as_str(),
        );
        create_dir_all(journal_dir).expect("journal directory creation failed. Aborting...");
        writeln!(stdout, "done.");
    }
    if !journal_file.exists() {
        notes::note(
            stdout,
            format!("creating {}... ", journal_file.display()).as_str(),
        );
        File::create(journal_file).expect("error when creating journal file");
        writeln!(stdout, "done.");
    }
}
