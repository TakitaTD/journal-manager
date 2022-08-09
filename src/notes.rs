use crate::color;
use std::io::Write;
use termcolor::{Color, StandardStream};

pub fn error(stdout: &mut StandardStream, msg: &str) {
    color::set_color(stdout, Color::Red);
    write!(stdout, "error: ").expect("error when changing term color");
    color::set_color(stdout, Color::White);
    writeln!(stdout, "{}", msg).expect("error when changing term color");
}
pub fn note(stdout: &mut StandardStream, msg: &str) {
    color::set_color(stdout, Color::Green);
    write!(stdout, "note: ").expect("error when changing term color");
    color::set_color(stdout, Color::White);
    write!(stdout, "{}", msg).expect("error when changing term color");
}
