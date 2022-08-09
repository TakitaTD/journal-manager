use crate::notes;
use dirs;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::{
    fs::{create_dir_all, File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    path::Path,
};

use chrono::Datelike;
use termcolor::StandardStream;
use users;

#[derive(Serialize, Deserialize, Debug)]
pub struct JournalEntry {
    pub title: String,
    pub content: String,
    pub encrypted: bool,
    pub created: String,
    pub updated: String,
    pub id: u32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct JournalData {
    pub title: String,
    pub id: String,
    pub created: String,
}
impl JournalData {
    pub fn new(journal: JournalEntry) -> JournalData {
        return JournalData {
            title: journal.title,
            id: format!("{}-{}", journal.created, journal.id),
            created: journal.created,
        };
    }
}
impl JournalEntry {
    pub fn lines(&self) -> u32 {
        return self.content.split_whitespace().count() as u32;
    }
    pub fn new(title: &str, content: &str, encrypted: bool) -> JournalEntry {
        let current_date = chrono::offset::Local::now();

        let mut rng = rand::thread_rng();

        return JournalEntry {
            title: String::from(title),
            content: String::from(content),
            encrypted,
            created: format!(
                "{}.{}.{}",
                current_date.day(),
                current_date.month(),
                current_date.year()
            ),
            updated: format!(
                "{}.{}.{}",
                current_date.day(),
                current_date.month(),
                current_date.year()
            ),
            id: rng.gen_range(0..10000),
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
    journal: JournalEntry,
    custom_dir: &Option<String>,
) {
    let journal_file = format!(
        "{}/{}-{}.json",
        get_journal_dir(stdout, custom_dir),
        journal.created,
        journal.id
    );
    let journals_file = get_journal_file(stdout, custom_dir);
    let journal_file = Path::new(&journal_file);
    let mut journal_file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(journal_file)
        .unwrap();
    journal_file
        .set_len(0)
        .expect("error setting length of file");
    journal_file
        .write_all(
            serde_json::to_string(&journal)
                .expect("error when serializing object")
                .as_bytes(),
        )
        .expect("error when writing to file");
    let mut journals_file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(journals_file)
        .expect("unable to open journals file.");
    let mut journals_data = String::new();
    journals_file
        .read_to_string(&mut journals_data)
        .expect("cannot read file to string");
    journals_file.seek(SeekFrom::Start(0));
    let mut journals_data =
        serde_json::from_str::<Vec<JournalData>>(journals_data.as_str()).unwrap_or(vec![]);
    journals_data.push(JournalData::new(journal));
    journals_file.write_all(serde_json::to_string(&journals_data).unwrap().as_bytes());
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
        File::create(journal_file).expect("cannot create file");
        writeln!(stdout, "done.");
    }
}
