// Main Rust file voor de battlebot.
// Version: 0.1


//------------------------------
// Main Rust crate
//------------------------------
use std::error::Error;
use std::thread;
use std::time::Duration;
use std::io::{stdin, stdout, Read, Stdin, Write};

//------------------------------
// Rppal crate
//------------------------------
use rppal::gpio::{Gpio, Result, Pin, OutputPin};

//------------------------------
// Termion crate
//------------------------------
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

//------------------------------
// Constants
//------------------------------
const GPIO_PWM0: u8 = 22; // Fysieke pin:
const GPIO_PWM1: u8 = 27; // Fysieke pin: 

// Servo configuratie: 
// !important: niet aanpassen.
const PERIOD_MS: u64 = 10; // Periode: 100 Hz.
const PULSE_MIN_US: u64 = 1000; // Pulse width min. 1000 µs (1000 microseconden)
const PULSE_NEUTRAL_US: u64 = 1500; // Pulse width neutraal. 1500 µs (1500 microseconden)
const PULSE_MAX_US: u64 = 2000; // Pulse width max. 2000 µs (2000 microseconden)

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
                accelerate().unwrap();
            },
            Key::Char('s') => {
                println!("S: Afremmen!");
                deaccelerate().unwrap();
            },
            Key::Char('a') => {
                println!("A: Naar links!");
                turn_left().unwrap();
            },
            Key::Char('d') => {
                println!("D: Naar rechts!");
                turn_right().unwrap();
            },
            Key::Alt('w') => {
                println!("W: Überhard naar voor!");

            },
            Key::Alt('s') => {
                println!("W: Überhard naar achter!");

            },
            Key::Alt('a') => {
                println!("A: Überhard naar links!");

            },
            Key::Alt('d') => {
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

fn accelerate() -> std::result::Result<(), Box<dyn Error>> {    // Retrieve the GPIO pin and configure it as an output.

    let mut pin = Gpio::new()?.get(GPIO_PWM0)?.into_output();
    let mut pin2 = Gpio::new()?.get(GPIO_PWM1)?.into_output();

    pin.set_pwm(
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(PULSE_MIN_US),
    )?;

    pin2.set_pwm(
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(PULSE_MIN_US),
    )?;

    thread::sleep(Duration::from_millis(25));

    Ok(())
}

fn deaccelerate() -> std::result::Result<(), Box<dyn Error>>  {
    let mut pin = Gpio::new()?.get(GPIO_PWM0)?.into_output();
    let mut pin2 = Gpio::new()?.get(GPIO_PWM1)?.into_output();

    pin.set_pwm(
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(PULSE_MAX_US),
    )?;

    pin2.set_pwm(
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(PULSE_MAX_US),
    )?;

    thread::sleep(Duration::from_millis(25));

    Ok(())
}

fn turn_left() -> std::result::Result<(), Box<dyn Error>> {
    let mut pin = Gpio::new()?.get(GPIO_PWM0)?.into_output();
    let mut pin2 = Gpio::new()?.get(GPIO_PWM1)?.into_output();

    pin.set_low();
    pin2.set_low();

    pin.set_pwm(
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(PULSE_MIN_US),
    )?;

    pin2.set_pwm(
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(PULSE_MAX_US),
    )?;

    thread::sleep(Duration::from_millis(25));

    Ok(())
}

fn turn_right() -> std::result::Result<(), Box<dyn Error>> {
    let mut pin = Gpio::new()?.get(GPIO_PWM0)?.into_output();
    let mut pin2 = Gpio::new()?.get(GPIO_PWM1)?.into_output();

    pin.set_low();
    pin2.set_low();

    pin.set_pwm(
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(PULSE_MAX_US),
    )?;

    pin2.set_pwm(
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(PULSE_MIN_US),
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
