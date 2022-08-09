use crate::journal_fs::{self, JournalEntry};
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
    writeln!(stdout, "{:?}", journal_entry);
    journal_fs::save_journal(stdout, journal_entry, &custom_dir);
}
