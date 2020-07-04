use crate::probe::{Pin, PinConfig, PinMode, Probe};
use std::thread::sleep;
use std::time::Duration;

mod probe;

fn main() {
    let mut pins = PinConfig::new();
    pins.pin_mode(Pin::D12, PinMode::Digital);
    pins.pin_mode(Pin::D4, PinMode::Digital);
    pins.pin_mode(Pin::D6, PinMode::Digital);
    pins.pin_mode(Pin::D8, PinMode::Digital);
    pins.pin_mode(Pin::D10, PinMode::Digital);
    pins.pin_mode(Pin::A0, PinMode::Digital);
    pins.pin_mode(Pin::A1, PinMode::Analog);
    pins.pin_mode(Pin::A2, PinMode::Analog);
    pins.pin_mode(Pin::A3, PinMode::Analog);
    pins.pin_mode(Pin::A4, PinMode::Analog);

    let probe = Probe::new("/dev/ttyACM0");

    probe.reset();
    probe.configure_pins(pins);
    println!("{}", probe.dump_pins());

    probe.start_capture();
    sleep(Duration::from_secs(5));
    let result = probe.stop_capture();
    println!("{}", result.len())
}