// Main Rust file voor de battlebot.
// Version: 1.0.0 5-6-2023

//------------------------------
// Main Rust crate
//------------------------------
use std::error::Error;
use std::io::{BufRead, BufReader};
use std::process::{Child, ChildStdout, Command, Output, Stdio};
use std::thread;

//------------------------------
// Core
//------------------------------
use core::time::Duration;

//------------------------------
// Rppal crate
//------------------------------
use rppal::gpio::{OutputPin, Gpio};
use rppal::pwm::{Channel, Polarity, Pwm};

//------------------------------
// Termion crate
//------------------------------
use termion::clear::All;
use termion::cursor::Goto;

//------------------------------
// Servo configuration:
//------------------------------
// !important: DON'T CHANGE.
const PERIOD_MS: u64 = 20;
// Periode: 100 Hz.
const PULSE_MIN_US: u64 = 1000;
// Pulse width min. 1000 µs (1000 microseconden)
const PULSE_NEUTRAL_US: u64 = 1500;
// Pulse width neutraal. 1500 µs (1500 microseconden)
const _PULSE_MAX_US: u64 = 2000;
// Pulse width max. 2000 µs (2000 microseconden)

// pin connected to the relay
const RELAY_PIN: u8 = 17;

fn main() -> Result<(), Box<dyn Error>> {
    let mut relay_output_pin: OutputPin = match Gpio::new() {
        Ok(gpio) => gpio.get(RELAY_PIN).unwrap().into_output(),
        Err(e) => panic!("Error: {}", e),
    };
    // Sets the relay pin to output and panics if it fails.

    let mut exit_status: i32 = 1;
    let mac: &str = "98:B6:E9:B6:D4:F9";
    println!("Connecting with {}", mac);
    while exit_status != 0 {
        let output: Output = Command::new("bluetoothctl")
            .args(["connect", "98:B6:E9:B6:D4:F9"])
            .output()
            .expect("failed to execute process");
        exit_status = output.status.code().unwrap_or(1);
    }
    println!("Connected succesfully with {}", mac);
    // Connects to the dualshock 4 controller with the given mac address.

    let pwm: Pwm = Pwm::with_period(
        Channel::Pwm0,
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(PULSE_NEUTRAL_US),
        Polarity::Normal,
        true,
    )?;
    // Initialises the first PWM channel (PWM0).
    let pwm1: Pwm = Pwm::with_period(
        Channel::Pwm1,
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(PULSE_NEUTRAL_US),
        Polarity::Normal,
        true,
    )?;
    // Initialises the second PWM channel (PWM1).

    let mut cmd: Command = Command::new("jstest");
    cmd.arg("--event");
    cmd.arg("/dev/input/js0");
    cmd.stdout(Stdio::piped());

    let mut child: Child = cmd.spawn()?;
    let stdout: ChildStdout = child.stdout.take().ok_or("Kon niet Stdout zetten.")?;
    let reader: BufReader<ChildStdout> = BufReader::new(stdout);
    // Starts the jstest command and reads the output.

    let mut state: i32 = 1;
    // State for the toggle of the PWM channels.

    for line in reader.lines() {
        // Reads the output of the jstest command and collects the necessary data.

        let line: String = line?;
        let parts: Vec<&str> = line.split(", ").collect();

        let event_type: &str = match parts[0].split(": ").nth(1) {
            Some(s) => match s.split(" ").nth(1) {
                Some(t) => t,
                None => continue,
            },
            None => continue,
        };
        // Event type 1 or 2 based on the input.

        let number: &str = match parts[2].split(" ").nth(1) {
            Some(n) => n,
            None => continue,
        };
        // Number of the button based on the event type.

        let value: &str = match parts[3].split(" ").nth(1) {
            Some(v) => v,
            None => continue,
        };
        // Value of the button based on the event type and number.

        let event_type: i32 = event_type.parse::<i32>().unwrap();
        /*
            Event type 1 or 2
            1 = Buttons
            2 = Joysticks and L2/R2
        */
        let number: i32 = number.parse::<i32>().unwrap();
        // Numbers for the buttons. Numbers can be different based on the event type.
        let value: i32 = value.parse::<i32>().unwrap();
        // Values of the buttons. Values can be different based on the event type.
        // For more information please consult the jstest manual and the google drive.

        if event_type == 1 {
            match number {
                0 => {
                    // X button toggles the relay.
                    if value == 1 {
                        toggle_relay(&mut relay_output_pin);
                    }
                }
                8 => {
                    // Share button shuts down the Raspberry Pi.
                    print!("{}{}", All, Goto(1, 1));
                    shutdown();
                }
                9 => {
                    // Options button pauses the script and turns the wheels to their original state.
                    if value == 1 {
                        print!("{}{}", All, Goto(1, 1));
                        println!("Gestopt!");
                        turn_neutral(&pwm, &pwm1).unwrap();
                        state += 1;
                        if state % 2 == 0 {
                            // Disabled both PWM channels.
                            pwm.disable().expect("Kon PWM0 niet uitzetten.");
                            pwm1.disable().expect("Kon PWM1 niet uitzetten.");
                        } else {
                            // Enables both PWM channels.
                            pwm.enable().expect("Kon PWM0 niet aanzetten.");
                            pwm1.enable().expect("Kon PWM1 niet aanzetten.");
                        }
                    }
                }
                _ => {
                    // Fallback if none match.
                    print!("{}{}", All, Goto(1, 1));
                    println!("Druk op Options om te stoppen.");
                }
            }
        } else if event_type == 2 {
            match number {
                1 => {
                    // Left joystick to control the left servo motor.
                    print!("{}{}", All, Goto(1, 1));
                    let speed: u64 = speed_calc(value);
                    left_movement(&pwm, speed).unwrap();
                    println!("Nummer: {} en snelheid: {}", number, speed);
                }
                4 => {
                    // Right joystick to control the right servo motor.
                    print!("{}{}", All, Goto(1, 1));
                    let speed: u64 = speed_calc(value);
                    right_movement(&pwm1, speed).unwrap();
                    println!("Nummer: {} en snelheid: {}", number, speed);
                }
                _ => {
                    // Fallback if none match.
                    print!("{}{}", All, Goto(1, 1));
                    println!("Druk op Options om te stoppen.");
                }
            }
        }
    }
    Ok(())
}

fn right_movement(
    pwm: &Pwm, 
    pwm_pulse: u64
) -> Result<(), Box<dyn Error>> {
    // Sets the PWM for the right servo motor.
    pwm.set_pulse_width(Duration::from_micros(pwm_pulse))?;
    Ok(())
}

fn left_movement(
    pwm1: &Pwm, 
    pwm1_pulse: u64
) -> Result<(), Box<dyn Error>> {
    // Sets the PWM for the left servo motor.
    pwm1.set_pulse_width(Duration::from_micros(pwm1_pulse))?;
    Ok(())
}

fn turn_neutral(
    pwm: &Pwm,
    pwm1: &Pwm,
) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    // Rotates the wheels to their original state.
    for pulse in (PULSE_MIN_US..=PULSE_NEUTRAL_US).step_by(10) {
        // Calculates the pulse to turn the wheels to their original state.
        pwm.set_pulse_width(Duration::from_micros(pulse)).unwrap();
        pwm1.set_pulse_width(Duration::from_micros(pulse)).unwrap();
        thread::sleep(Duration::from_millis(20));
    }
    Ok(())
}

fn speed_calc(
    value: i32
) -> u64 {
    // Calculates the speed of the wheels with the given value.
    // Value varies between -32767 and 32767 and the result will always be between 1000 and 2000.
    let result: f32 = ((value as f32 / -32767.0) * 500.0) + 1500.0;
    let end_result: f32 = result.round();
    end_result as u64
}

fn shutdown() {
    // Shutdown the Raspberry Pi.
    let mut cmd: Command = Command::new("sudo");
    cmd.args(["shutdown", "-h", "now"]);
    cmd.stdout(Stdio::piped());
    cmd.spawn().expect("Kon niet afsluiten.");
    println!("Shutting down...");
}

fn toggle_relay(
    output_pin: &mut OutputPin
) {
    // Toggle the relay.
    let current_state: bool = output_pin.is_set_high();

    if current_state == true {
        // Turns the relay off.
        output_pin.set_low();
        println!("Relais uitgeschakeld.");
    } else {
        // Turns the relay on.
        output_pin.set_high();
        println!("Relais ingeschakeld.");
    }
}
