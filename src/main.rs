// Main Rust file voor de battlebot.
// Version: 0.2

//------------------------------
// Main Rust crate
//------------------------------
use std::error::Error;
use std::io::{BufRead, BufReader};
use std::process::{Child, ChildStdout, Command, Stdio};
use std::thread;

//------------------------------
// Core
//------------------------------
use core::time::Duration;

//------------------------------
// Rppal crate
//------------------------------
use rppal::pwm::{Channel, Polarity, Pwm};

//------------------------------
// Termion crate
//------------------------------
use termion::clear::All;
use termion::cursor::Goto;

//------------------------------
// Servo configuratie:
//------------------------------
// !important: NIET AANPASSEN.
const PERIOD_MS: u64 = 20;
// Periode: 100 Hz.
const PULSE_MIN_US: u64 = 1000;
// Pulse width min. 1000 µs (1000 microseconden)
const PULSE_NEUTRAL_US: u64 = 1500;
// Pulse width neutraal. 1500 µs (1500 microseconden)
const PULSE_MAX_US: u64 = 2000;
// Pulse width max. 2000 µs (2000 microseconden)

fn main() -> Result<(), Box<dyn Error>> {
    let pwm: Pwm = Pwm::with_period(
        Channel::Pwm0,
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(PULSE_NEUTRAL_US),
        Polarity::Normal,
        true,
    )?;
    let pwm1: Pwm = Pwm::with_period(
        Channel::Pwm1,
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(PULSE_NEUTRAL_US),
        Polarity::Normal,
        true,
    )?;
    let mut cmd: Command = Command::new("jstest");
    cmd.arg("--event");
    cmd.arg("/dev/input/js0");
    cmd.stdout(Stdio::piped());

    let mut child: Child = cmd.spawn()?;
    let stdout: ChildStdout = child.stdout.take().ok_or("Kon niet Stdout zetten.")?;
    let reader: BufReader<ChildStdout> = BufReader::new(stdout);

    for line in reader.lines() {
        let line: String = line?;
        let parts: Vec<&str> = line.split(", ").collect();

        let event_type: &str = match parts[0].split(": ").nth(1) {
            Some(s) => match s.split(" ").nth(1) {
                Some(t) => t,
                None => continue,
            },
            None => continue,
        };
        let number: &str = match parts[2].split(" ").nth(1) {
            Some(n) => n,
            None => continue,
        };
        let value: &str = match parts[3].split(" ").nth(1) {
            Some(v) => v,
            None => continue,
        };

        let event_type: i32 = event_type.parse::<i32>().unwrap();
        // Event type 1 of 2
        // 1 = Buttons
        // 2 = Joysticks en L2/R2
        let number: i32 = number.parse::<i32>().unwrap();
        // Nummer voor de knoppen. Nummers verschillen op basis van Event type.
        let value: i32 = value.parse::<i32>().unwrap();
        // Waardes van de knoppen. Waardes verschillen op basis van event type.
        // Voor meer info kijk naar de Google Drive `Joystick`

        if event_type == 1 {
            match number {
                9 => {
                    print!("{}{}", All, Goto(1, 1));
                    println!("Gestopt!");
                    turn_neutral(&pwm, &pwm1).unwrap();
                    let mut script_active: bool = false;
                    'active: loop {
                        if number == 9 && value == 1 {
                            script_active = true;
                        }

                        if script_active && number == 9 && value == 1 {
                            break 'active;
                        }
                    }
                }
                _ => {
                    print!("{}{}", All, Goto(1, 1));
                    println!("Druk op Options om te stoppen.");
                }
            }
        } else if event_type == 2 {
            match number {
                1 => {
                    print!("{}{}", All, Goto(1, 1));
                    let speed: u64 = speed_calc(value);
                    left_movement(&pwm, speed).unwrap();
                    println!("Nummer: {} en snelheid: {}", number, speed);
                }
                4 => {
                    print!("{}{}", All, Goto(1, 1));
                    let speed: u64 = speed_calc(value);
                    right_movement(&pwm1, speed).unwrap();
                    println!("Nummer: {} en snelheid: {}", number, speed);
                }
                _ => {
                    print!("{}{}", All, Goto(1, 1));
                    println!("Druk op Options om te stoppen.");
                }
            }
        }
    }
    Ok(())
}

fn right_movement(pwm: &Pwm, pwm_pulse: u64) -> Result<(), Box<dyn Error>> {
    // Zet de PWM voor de rechter servo motor.
    pwm.set_pulse_width(Duration::from_micros(pwm_pulse))?;
    Ok(())
}

fn left_movement(pwm1: &Pwm, pwm1_pulse: u64) -> Result<(), Box<dyn Error>> {
    // Zet de PWM voor de linker servo motor.
    pwm1.set_pulse_width(Duration::from_micros(pwm1_pulse))?;
    Ok(())
}

fn turn_neutral(
    pwm: &Pwm,
    pwm1: &Pwm,
) -> std::result::Result<(), Box<(dyn std::error::Error + 'static)>> {
    // Roteert de Servo's naar hun originele staat.
    for pulse in (PULSE_MIN_US..=PULSE_NEUTRAL_US).step_by(10) {
        // Rekent de benodigde pulse uit en zet vervolgens de wielen naar hun originele staat.
        pwm.set_pulse_width(Duration::from_micros(pulse)).unwrap();
        pwm1.set_pulse_width(Duration::from_micros(pulse)).unwrap();
        thread::sleep(Duration::from_millis(20));
    }
    Ok(())
}

fn speed_calc(value: i32) -> u64 {
    // Rekent de pulse width in microseconden uit met de value.
    // Value = -32767 / 32767
    let result: f32 = ((value as f32 / -32767.0) * 500.0) + 1500.0;
    let end_result: f32 = result.round();
    end_result as u64
}
