// Main Rust file voor de battlebot.
// Version: 0.1


//------------------------------
// Main Rust crate
//------------------------------
use std::error::Error;
use std::thread;
use std::time::Duration;
use std::io::{stdin, stdout, Write};

//------------------------------
// Rppal crate
//------------------------------
use rppal::gpio::{Gpio, OutputPin};

//------------------------------
// Termion crate
//------------------------------
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

//------------------------------
// Constants
//------------------------------
const GPIO_PWM0: u8 = 12; // Fysieke pin: 32
const GPIO_PWM1: u8 = 13; // Fysieke pin: 33

// Servo configuratie: 
// !important: niet aanpassen.
const PERIOD_MS: u64 = 20; // Periode: 100 Hz.
const PULSE_MIN_US: u64 = 1200; // Pulse width min. 1000 µs (1000 microseconden)
const PULSE_NEUTRAL_US: u64 = 1500; // Pulse width neutraal. 1500 µs (1500 microseconden)
const PULSE_MAX_US: u64 = 1800; // Pulse width max. 2000 µs (2000 microseconden)

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, r#"{}{} Druk op Esc om af te sluiten."#, termion::cursor::Goto(1, 1), termion::clear::All).unwrap();
    stdout.flush().unwrap();

    for character in stdin.keys() {
        //clearing the screen and going to top left corner
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
                let speed: u64 = 1200;
                accelerate(speed).unwrap();
            },
            Key::Char('s') => {
                println!("S: Afremmen!");
                let speed: u64 = 1800;
                deaccelerate(speed).unwrap();
            },
            Key::Char('a') => {
                println!("A: Naar links!");
                let left: u64 = 1200;
                let right : u64 = 1800;
                turn_left(left, right).unwrap();
            },
            Key::Char('d') => {
                println!("D: Naar rechts!");
                let left: u64 = 1800;
                let right : u64 = 1200;
                turn_right(left, right).unwrap();
            },
            Key::Alt('w') => {
                println!("W: Überhard naar voor!");
                let speed: u64 = 1000;
                accelerate(speed).unwrap();
            },
            Key::Alt('s') => {
                println!("S: Überhard naar achter!");
                let speed: u64 = 2000;
                deaccelerate(speed).unwrap();
            },
            Key::Alt('a') => {
                let left: u64 = 2000;
                let right: u64 = 1000;
                println!("A: Überhard naar links!");
                turn_left(left, right).unwrap();
            },
            Key::Alt('d') => {
                let left: u64 = 1000;
                let right: u64 = 2000;
                println!("A: Überhard naar links!");
                turn_right(left, right).unwrap();
                println!("D: Überhard naar rechts!");

            },
            Key::Esc => {
                write!(
                    stdout,
                    "{}{}",
                    termion::cursor::Goto(1, 1),
                    termion::clear::All
                ).unwrap();
                println!("Escaped the Matrix!");
                turn_neutral().expect("Kon niet naar standaard!");
                break;
            },
            _ => {
                println!("Druk op Esc om af te sluiten.");
            },
        }
        stdout.flush().unwrap();
    }
}

fn accelerate(speed: u64) -> std::result::Result<(), Box<dyn Error>> { // Retrieve the GPIO pin and configure it as an output.
    let mut pin: OutputPin = Gpio::new()?.get(GPIO_PWM0)?.into_output();
    let mut pin2: OutputPin = Gpio::new()?.get(GPIO_PWM1)?.into_output();

    pin.set_pwm(
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(speed),
    )?;

    pin2.set_pwm(
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(speed),
    )?;

    thread::sleep(Duration::from_millis(25));

    Ok(())
}

fn deaccelerate(speed: u64) -> std::result::Result<(), Box<dyn Error>>  {
    let mut pin: OutputPin = Gpio::new()?.get(GPIO_PWM0)?.into_output();
    let mut pin2: OutputPin = Gpio::new()?.get(GPIO_PWM1)?.into_output();

    pin.set_pwm(
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(speed),
    )?;

    pin2.set_pwm(
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(speed),
    )?;

    thread::sleep(Duration::from_millis(25));

    Ok(())
}

fn turn_left(left: u64, right: u64) -> std::result::Result<(), Box<dyn Error>> {
    let mut pin: OutputPin = Gpio::new()?.get(GPIO_PWM0)?.into_output();
    let mut pin2: OutputPin = Gpio::new()?.get(GPIO_PWM1)?.into_output();

    pin.set_pwm(
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(left),
    )?;

    pin2.set_pwm(
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(right),
    )?;

    thread::sleep(Duration::from_millis(25));

    Ok(())
}

fn turn_right(left: u64, right: u64) -> std::result::Result<(), Box<dyn Error>> {
    let mut pin: OutputPin = Gpio::new()?.get(GPIO_PWM0)?.into_output();
    let mut pin2: OutputPin = Gpio::new()?.get(GPIO_PWM1)?.into_output();

    pin.set_pwm(
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(left),
    )?;

    pin2.set_pwm(
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(right),
    )?;

    thread::sleep(Duration::from_millis(25));

    Ok(())
}

fn turn_neutral() -> std::result::Result<(), Box<(dyn std::error::Error + 'static)>> { // Roteert de Servo's naar zijn originele staat.
    let mut pin22: OutputPin = Gpio::new()?.get(GPIO_PWM0)?.into_output();
    let mut pin27: OutputPin = Gpio::new()?.get(GPIO_PWM1)?.into_output();

    for pulse in (PULSE_MIN_US..=PULSE_NEUTRAL_US).step_by(10) {
        pin22.set_pwm(
            Duration::from_millis(PERIOD_MS),
            Duration::from_micros(pulse),
        )?;
        pin27.set_pwm(
            Duration::from_millis(PERIOD_MS),
            Duration::from_micros(pulse),
        )?;
        thread::sleep(Duration::from_millis(20));
    }
    Ok(())
}
