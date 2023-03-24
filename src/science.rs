// Main Rust file voor de battlebot.
// Version: 0.1


//------------------------------
// Main Rust crate
//------------------------------
use std::error::Error;
use std::thread;
use std::time::Duration;
use std::io::{stdin, stdout, Read};

//------------------------------
// Rppal crate
//------------------------------
use rppal::gpio::Gpio;
use rppal::gpio::{pin, OutputPin};

//------------------------------
// Termion crate
//------------------------------
use termion::event::Key;
use termion::input::TermRead;
use termion::clear::BeforeCursor;

//------------------------------
// Constants
//------------------------------
const GPIO_PWM0: u64 = 22; // Fysieke pin:
const GPIO_PWM1: u64 = 27; // Fysieke pin: 

// Servo configuratie: 
// !important: niet aanpassen.
const PERIOD_MS: u64 = 10; // Periode: 100 Hz.
const PULSE_MIN_US: u64 = 1000; // Pulse width min. 1000 µs (1000 microseconden)
const PULSE_NEUTRAL_US: u64 = 1500; // Pulse width neutraal. 1500 µs (1500 microseconden)
const PULSE_MAX_US: u64 = 2000; // Pulse width max. 2000 µs (2000 microseconden)

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let stdin = stdin();
    let mut script_active: bool = true;

    while script_active == true {
        println!("Druk op de 'Esc' om het script te stoppen.");
        for character in stdin.keys() {
            match character {
                Key::Char('w') => {
                    BeforeCursor();
                    print!("Up, W").expect("W: Kon niet versnellen!");
                    accelerate();
                }
                Key::Char('s') => {
                    BeforeCursor();
                    print!("Down, S").expect("S: Kon niet remmen!");
                    deaccelerate();
                }
                Key::Char('a') => {
                    BeforeCursor();
                    print!("Left, A").expect("W: Kon niet naar links draaien!");
                    turn_left();
                }
                Key::Char('d') => {
                    BeforeCursor();
                    print!("Right, D").expect("D: Kon niet naar rechts draaien!");
                    turn_right();
                }
                Key::Esc => {
                    BeforeCursor();
                    print!("Escaped the Matrix.").expect("Esc: Kon niet sluiten!");
                    turn_neutral();
                    script_active = false;
                    break;
                }
                _ => {}
            }
        }   
    }
}

fn accelerate() { // Roteert de Servo's om naar voren te draaien.
    let mut pin22: OutputPin = Gpio::new()?.get(GPIO_PWM0)?.into_output();
    let mut pin27: OutputPin = Gpio::new()?.get(GPIO_PWM1)?.into_output();

    pin22.set_pwm(
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(PULSE_MAX_US)
    );
    pin27.set_pwm(
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(PULSE_MAX_US)
    );
}

fn deaccelerate() { // Roteert de Servo's om naar achter te draaien.
    let mut pin22: OutputPin = Gpio::new()?.get(GPIO_PWM0)?.into_output();
    let mut pin27: OutputPin = Gpio::new()?.get(GPIO_PWM1)?.into_output();

    pin22.set_pwm(
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(PULSE_MIN_US)
    );
    pin27.set_pwm(
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(PULSE_MIN_US)
    );
}

fn turn_left() { // Roteert de Servo's om naar links te draaien.
    let mut pin22: OutputPin = Gpio::new()?.get(GPIO_PWM0)?.into_output();
    let mut pin27: OutputPin = Gpio::new()?.get(GPIO_PWM1)?.into_output();

    pin22.set_pwm(
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(PULSE_MAX_US)
    );
    pin27.set_pwm(
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(PULSE_MIN_US)
    );
}

fn turn_right() { // Roteert de Servo's om naar rechts te draaien.
    let mut pin22: OutputPin = Gpio::new()?.get(GPIO_PWM0)?.into_output();
    let mut pin27: OutputPin = Gpio::new()?.get(GPIO_PWM1)?.into_output();

    pin22.set_pwm(
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(PULSE_MIN_US)
    );
    pin27.set_pwm(
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(PULSE_MAX_US)
    );
}

fn turn_neutral() -> Result<(), Box<dyn Error>> { // Roteert de Servo's naar zijn originele staat.
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
}