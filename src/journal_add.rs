use crate::journal_fs::{self, JournalEntry};
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use std::io::{self, Write};
use termcolor::StandardStream;

pub fn add(stdout: &mut StandardStream, custom_dir: Option<String>) {
    let mut journal_entry = JournalEntry::new("", "", false);
    write!(stdout, "Title: ");
    stdout.flush().expect("failed flushing stdout");
    io::stdin()
        .read_line(&mut journal_entry.title)
        .expect("error getting user input from stdin");
    journal_entry.title = journal_entry.title.trim().to_owned();
    writeln!(
        stdout,
        "Write some text. (Make a new line with the text: \"qq!\" to quit)"
    )
    .expect("error when writing to stdout");
    let mut lines: u32 = 0;
    loop {
        let mut buf = String::new();
        lines += 1;
        write!(stdout, "{}    ", lines);
        stdout.flush().expect("unable to flush stdout");
        io::stdin()
            .read_line(&mut buf)
            .expect("unable to read line from stdin");
        // let buf = buf.trim();
        if buf.trim() == "qq!" {
            break;
        }
        journal_entry.content.push_str(buf.as_str());
    }
    let mut encrypt_buf = String::new();
    write!(stdout, "Encrypt File? (y/N): ");
    stdout.flush().expect("unable to flush stdout");
    io::stdin()
        .read_line(&mut encrypt_buf)
        .expect("unable to get input from stdin");
    // writeln!(stdout, "{}", encrypt_buf);
    if encrypt_buf.trim().to_lowercase().as_str() == "y" {
        write!(stdout, "Enter a key: ");
        stdout.flush().expect("unable to flush stdout");
        let mut encryption_key = String::new();
        io::stdin()
            .read_line(&mut encryption_key)
            .expect("unable to get input from stdin");
        let magic = new_magic_crypt!(encryption_key.trim(), 256);

        let encrypted_content = magic.encrypt_str_to_base64(journal_entry.content.as_str());
        // let encrypted_title = magic.encrypt_str_to_base64(journal_entry.title.as_str());

        journal_entry.content = encrypted_content;
        // journal_entry.title = encrypted_title;
        journal_entry.encrypted = true;
    }
    journal_fs::save_journal(stdout, journal_entry, &custom_dir);
}
