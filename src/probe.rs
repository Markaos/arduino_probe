use std::sync::mpsc::{Receiver, Sender};
use serial::core::BaudRate::Baud115200;
use serial::unix::TTYPort;

mod port_handle;
use crate::probe::port_handle::Handle;

#[cfg(test)]
mod tests;

#[allow(unused)]
pub enum Pin {
    D0 = 0,
    D1 = 1,
    D2 = 2,
    D3 = 3,
    D4 = 4,
    D5 = 5,
    D6 = 6,
    D7 = 7,
    D8 = 8,
    D9 = 9,
    D10 = 10,
    D11 = 11,
    D12 = 12,
    D13 = 13,
    A0 = 14,
    A1 = 15,
    A2 = 16,
    A3 = 17,
    A4 = 18,
    A5 = 19,
}

#[allow(unused)]
#[derive(Clone,Copy)]
pub enum PinMode {
    Off = 0,
    Digital = 1,
    Analog = 2,
}

impl PinMode {
    fn as_char(&self) -> char {
        match self {
            PinMode::Off     => 'o',
            PinMode::Digital => 'd',
            PinMode::Analog  => 'a',
        }
    }
}

pub struct PinConfig {
    pins: [PinMode; 20],
}

#[allow(unused)]
impl PinConfig {
    pub fn new() -> PinConfig {
        PinConfig { pins: [PinMode::Off; 20] }
    }

    pub fn pin_mode(&mut self, pin: Pin, pin_mode: PinMode) {
        self.pins[pin as usize] = pin_mode;
    }

    pub fn get_pin_mode(&self, pin: Pin) -> PinMode {
        self.pins[pin as usize]
    }

    fn as_string(&self) -> String {
        let mut response = String::with_capacity(90);
        let mut first = true;

        for (pin, mode) in self.pins.iter().enumerate() {
            if !first {
                response.push(',');
            }
            first = false;

            response.push_str(format!("{}={}", pin, mode.as_char()).as_str());
        }
        response
    }
}


pub enum ProbeMessage {
    Reset,
    ConfigurePins(PinConfig),
    DumpPins,
    StartCapture,
    StopCapture,
}

pub struct Probe {
    command_tx: Sender<ProbeMessage>,
    data_rx: Receiver<String>,
}

impl Probe {
    pub fn new(path: &'static str) -> Probe {
        let (command_tx, data_rx) = Handle::<TTYPort>::spawn(path, Baud115200);
        Probe { command_tx, data_rx }
    }

    pub fn reset(&self) {
        self.command_tx.send(ProbeMessage::Reset).expect("Link broken");
        self.data_rx.recv().expect("Broken link");
    }

    pub fn configure_pins(&self, pins: PinConfig) {
        self.command_tx.send(ProbeMessage::ConfigurePins(pins)).expect("Link broken");
        self.data_rx.recv().expect("Broken link");
    }

    pub fn dump_pins(&self) -> String {
        self.command_tx.send(ProbeMessage::DumpPins).expect("Link broken");
        match self.data_rx.recv() {
            Ok(res) => res,
            _ => String::new()
        }
    }

    pub fn start_capture(&self) {
        self.command_tx.send(ProbeMessage::StartCapture).expect("Link broken");
    }

    pub fn stop_capture(&self) -> Vec<Vec<u16>> {
        self.command_tx.send(ProbeMessage::StopCapture).expect("Link broken");
        let capture = self.data_rx.recv().expect("Link broken");
        let lines: Vec<&str> = capture.split("\n").collect::<Vec<&str>>();
        lines.iter().map(|line| {
            line.split(",").map(|column| {
                u16::from_str_radix(column, 10).unwrap()
            }).collect::<Vec<u16>>()
        }).collect()
    }
}