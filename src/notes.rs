use crate::color;
use std::io::Write;
use termcolor::{Color, StandardStream};

pub fn error(stdout: &mut StandardStream, msg: &str) {
    color::set_color(stdout, Color::Red);
    write!(stdout, "error: ");
    color::set_color(stdout, Color::White);
    writeln!(stdout, "{}", msg);
}
pub fn note(stdout: &mut StandardStream, msg: &str) {
    color::set_color(stdout, Color::Green);

    write!(stdout, "note: ");
    color::set_color(stdout, Color::White);
    write!(stdout, "{}", msg);
}
