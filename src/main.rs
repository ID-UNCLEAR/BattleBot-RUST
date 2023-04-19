// Main Rust file voor de battlebot.
// Version: 0.1

//------------------------------
// Main Rust crate
//------------------------------
use std::error::Error;
use std::io::{stdin, stdout, Write};
use std::thread;
use std::time::Duration;

//------------------------------
// Rppal crate
//------------------------------
use rppal::pwm::{Channel, Polarity, Pwm};

//------------------------------
// Termion crate
//------------------------------
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

// Servo configuratie:
// !important: niet aanpassen.
const PERIOD_MS: u64 = 20; // Periode: 100 Hz.

fn main() -> Result<(), Box<dyn Error>> {
    let pulse_min_us: u64 = 1200;
    // Pulse width min. 1000 µs (1000 microseconden)
    let pulse_neutral_us: u64 = 1500;
    // Pulse width neutraal. 1500 µs (1500 microseconden)
    let pulse_max_us: u64 = 1800;
    // Pulse width max. 2000 µs (2000 microseconden)

    let pwm: Pwm = Pwm::with_period(
        Channel::Pwm0,
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(pulse_neutral_us),
        Polarity::Normal,
        true,
    )?;
    let pwm1: Pwm = Pwm::with_period(
        Channel::Pwm1,
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(pulse_neutral_us),
        Polarity::Normal,
        true,
    )?;

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(
        stdout,
        r#"{}{} Druk op Esc om af te sluiten."#,
        termion::cursor::Goto(1, 1),
        termion::clear::All
    )
    .unwrap();
    stdout.flush().unwrap();
    for character in stdin.keys() {
        // Klaart het scherm en zet de cursor op startpositie
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All
        )
        .unwrap();
        match character.unwrap() {
            Key::Char('w') => {
                println!("W: Versnellen!");
                movement(&pwm, &pwm1, pulse_max_us, pulse_max_us).unwrap();
            }
            Key::Char('s') => {
                println!("S: Afremmen!");
                movement(&pwm, &pwm1, pulse_min_us, pulse_min_us).unwrap();
            }
            Key::Char('a') => {
                println!("A: Links!");
                movement(&pwm, &pwm1, pulse_min_us, pulse_max_us).unwrap();
            }
            Key::Char('d') => {
                println!("D: Rechts!");
                movement(&pwm, &pwm1, pulse_max_us, pulse_min_us).unwrap();
            }
            Key::Alt('w') => {
                let speed: u64 = pulse_max_us + 200;
                println!("W: Extrahard versnellen!");
                movement(&pwm, &pwm1, speed, speed).unwrap();
            }
            Key::Alt('s') => {
                let speed: u64 = pulse_min_us - 200;
                println!("S: Extrahard afremmen!");
                movement(&pwm, &pwm1, speed, speed).unwrap();
            }
            Key::Alt('a') => {
                let speed_left: u64 = pulse_min_us - 200;
                let speed_right: u64 = pulse_max_us + 200;
                println!("A: Extrahard links!");
                movement(&pwm, &pwm1, speed_left, speed_right).unwrap();
            }
            Key::Alt('d') => {
                let speed_left: u64 = pulse_max_us + 200;
                let speed_right: u64 = pulse_min_us - 200;
                println!("D: Extrahard rechts!");
                movement(&pwm, &pwm1, speed_left, speed_right).unwrap();
            }
            Key::BackTab => {
                println!("Shift + Tab: Stil staan!");
                movement(&pwm, &pwm1, 1, 1).unwrap();
            }
            Key::Esc => {
                // Sluit het programma definitief af.
                write!(
                    stdout,
                    "{}{}",
                    termion::cursor::Goto(1, 1),
                    termion::clear::All
                )
                .unwrap();
                println!("Escaped the Matrix!");
                turn_neutral(&pwm, &pwm1, pulse_min_us, pulse_neutral_us)
                    .expect("Kon niet naar standaard!");
                break;
            }
            _ => {
                println!("Druk op Esc om af te sluiten.");
            }
        }
        stdout.flush().unwrap();
    }
    Ok(())
}

fn movement(pwm: &Pwm, pwm1: &Pwm, pwm_pulse: u64, pwm1_pulse: u64) -> Result<(), Box<dyn Error>> {
    pwm.set_pulse_width(Duration::from_micros(pwm_pulse))?;
    pwm1.set_pulse_width(Duration::from_micros(pwm1_pulse))?;
    Ok(())
}

fn turn_neutral(
    pwm: &Pwm,
    pwm1: &Pwm,
    pwm_min_pulse: u64,
    pwm_neutral_pulse: u64,
) -> std::result::Result<(), Box<(dyn std::error::Error + 'static)>> {
    // Roteert de Servo's naar hun originele staat.
    for pulse in (pwm_min_pulse..=pwm_neutral_pulse).step_by(10) {
        // Rekent de benodigde pulse uit en zet vervolgens de wielen naar hun originele staat.
        pwm.set_pulse_width(Duration::from_micros(pulse)).unwrap();
        pwm1.set_pulse_width(Duration::from_micros(pulse)).unwrap();
        thread::sleep(Duration::from_millis(20));
    }
    pwm.disable().expect("Kon pwm0 niet afsluiten.");
    pwm1.disable().expect("Kon pwm1 niet afsluiten.");
    Ok(())
}
