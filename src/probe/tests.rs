use super::*;
use std::time::Duration;
use std::thread::sleep;

#[test]
fn pinconfig_output() {
    let mut pc = PinConfig::new();
    pc.pin_mode(Pin::D2, PinMode::Digital);
    pc.pin_mode(Pin::A0, PinMode::Analog);
    pc.pin_mode(Pin::A1, PinMode::Analog);
    pc.pin_mode(Pin::A2, PinMode::Digital);
    pc.pin_mode(Pin::A0, PinMode::Off);

    assert_eq!(pc.as_string(), "0=o,1=o,2=d,3=o,4=o,5=o,6=o,7=o,8=o,9=o,10=o,11=o,12=o,13=o,14=o,15=a,16=d,17=o,18=o,19=o");
}

#[test]
fn pinmode_names() {
    assert_eq!(PinMode::Off.as_char(), 'o');
    assert_eq!(PinMode::Digital.as_char(), 'd');
    assert_eq!(PinMode::Analog.as_char(), 'a');
}

#[test]
#[ignore]
fn probe_handling() {
    let mut pc = PinConfig::new();
    pc.pin_mode(Pin::D2, PinMode::Digital);
    pc.pin_mode(Pin::A0, PinMode::Analog);
    pc.pin_mode(Pin::A1, PinMode::Analog);
    pc.pin_mode(Pin::A2, PinMode::Digital);
    pc.pin_mode(Pin::A0, PinMode::Off);

    let probe = Probe::new("/dev/ttyACM0");

    probe.reset();
    probe.configure_pins(pc);
    assert_eq!(probe.dump_pins(), "0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 2 1 0 0 0");

    probe.start_capture();
    sleep(Duration::from_secs(5));
    let _result = probe.stop_capture();
}