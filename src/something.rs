use std::io::{stdin, stdout, Read};
use termion::event::Key;
use termion::input::TermRead;

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    println!(stdout, "Press 'Esc' to quit").unwrap();
    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('w') => {
                println!("Gedrukt op w, versnellen.");
                accelerate();
            }
            Key::Char('a') => {
                println!("Gedrukt op a, naar links.");
                turn_left();
            }
            Key::Char('s') => {
                println!("Gedrukt op w, afremmen.");
                break();
            }
            Key::Char('d') => {
                println!("Gedrukt op w, naar rechts.");
                turn_right();
            }
            Key::Esc => {
                writeln!(stdout, "Goodbye!").unwrap();
                break;
            }
            _ => {}
        }
    }
}
