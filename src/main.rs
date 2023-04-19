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

//------------------------------
// DS4
//------------------------------
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

// Servo configuratie:
// !important: niet aanpassen.
const PERIOD_MS: u64 = 20; // Periode: 100 Hz.

fn main() -> Result<(), Box<dyn Error>> {
    let pulse_min_us: u64 = 1000;
    // Pulse width min. 1000 µs (1000 microseconden)
    let pulse_neutral_us: u64 = 1500;
    // Pulse width neutraal. 1500 µs (1500 microseconden)
    let pulse_max_us: u64 = 2000;
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
    let mut cmd = Command::new("jstest");
        cmd.arg("--event");
        cmd.arg("/dev/input/js0");
        cmd.stdout(Stdio::piped());

    let mut child = cmd.spawn()?;
    let stdout = child.stdout.take().ok_or("failed to capture stdout")?;
    let reader = BufReader::new(stdout);

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(", ").collect();

        let event_type = match parts[0].split(": ").nth(1) {
            Some(s) => match s.split(" ").nth(1) {
                Some(t) => t,
                None => continue,
            },
            None => continue,
        };
        let number = match parts[2].split(" ").nth(1) {
            Some(n) => n,
            None => continue,
        };
        let value = match parts[3].split(" ").nth(1) {
            Some(v) => v,
            None => continue,
        };

        let event_type = event_type.parse::<i32>().unwrap();
        let number = number.parse::<i32>().unwrap();
        let value = value.parse::<u64>().unwrap();

        if event_type == 2 && number == 1 {
            speed_calc(value);
            left_movement(&pwm, value).unwrap();
            println!("Nummer: {}", number);
            
        } else if event_type == 2 && number == 4 {
            speed_calc(value);
            right_movement(&pwm1, value).unwrap();
            println!("Nummer: {}", number);
        }
        else if event_type == 1 && number == 9 {
            turn_neutral(&pwm, &pwm1, pulse_min_us, pulse_neutral_us).unwrap();
            println!("Stopped");
            break;
        }
    }
    Ok(())
}

fn right_movement(
    pwm: &Pwm, 
    pwm_pulse: u64,
) -> Result<(), Box<dyn Error>> {
    pwm.set_pulse_width(Duration::from_micros(pwm_pulse))?;
    Ok(())
}

fn left_movement(
    pwm1: &Pwm, 
    pwm1_pulse: u64
) -> Result<(), Box<dyn Error>> {
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

fn speed_calc(value: u64) {
    let result = ((value as f32 / -32767.0) * 500.0) + 1500.0;
    println!("{}", result.floor());
}
