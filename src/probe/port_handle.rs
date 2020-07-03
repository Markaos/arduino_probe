use serial::{SerialPort, BaudRate};
use std::sync::mpsc::{Sender, Receiver, channel};
use crate::probe::ProbeMessage;
use std::thread::{spawn, sleep};
use std::time::Duration;
use serial::core::Parity::ParityNone;
use serial::core::FlowControl::FlowHardware;
use serial::core::StopBits::Stop1;
use serial::core::CharSize::Bits8;
use crate::probe::ProbeMessage::*;

pub struct Handle<P: SerialPort> {
    port: P,
    buffer: String,
}

impl<P: SerialPort> Handle<P> {
    pub fn new(port: P) -> Handle<P> {
        Handle {
            port,
            buffer: String::new()
        }
    }

    pub fn spawn(path: &'static str, baud: BaudRate) -> (Sender<ProbeMessage>, Receiver<String>) {
        let (command_tx, command_rx) = channel();
        let (data_tx, data_rx) = channel();
        spawn(move || {
            let mut port = serial::open(path).expect(format!("Couldn't open {}", path).as_str());
            port.set_timeout(Duration::from_millis(2000)).expect("Couldn't set timeout");
            port.reconfigure(&|settings| -> Result<(), serial_core::Error> {
                settings.set_parity(ParityNone);
                settings.set_flow_control(FlowHardware);
                settings.set_stop_bits(Stop1);
                settings.set_char_size(Bits8);
                settings.set_baud_rate(baud)
            }).expect("Unexpected configuration");

            let mut handle = Handle::new(port);
            handle.port_monitor(command_rx, data_tx);
        });
        (command_tx, data_rx)
    }

    fn wait_for(&mut self, what: &str) -> String {
        let mut read_raw = [0u8; 4096];
        while !self.buffer.contains(what) {
            match self.port.read(&mut read_raw) {
                Ok(length) => {
                    self.buffer.push_str(String::from_utf8_lossy(&read_raw[0..length]).to_string().as_str())
                }
                _ => {
                    break;
                }
            }
        }

        match self.buffer.find(what) {
            Some(index) => {
                self.buffer.split_off(index);
                self.buffer.clone()
            }
            _ => {
                self.buffer.clone()
            }
        }
    }

    fn port_monitor(&mut self, command_rx: Receiver<ProbeMessage>, data_tx: Sender<String>) {
        loop {
            match command_rx.recv() {
                Ok(message) => {
                    match message {
                        Reset => {
                            self.port.set_dtr(false).expect("Issue with the serial port");
                            self.port.set_rts(false).expect("Issue with the serial port");
                            sleep(Duration::from_millis(50));
                            self.port.set_dtr(true).expect("Issue with the serial port");
                            self.port.set_rts(true).expect("Issue with the serial port");

                            self.wait_for("Arduino Probe ready\n");
                            data_tx.send(String::from("Done")).expect("Broken link");
                        }
                        ConfigurePins(config) => {
                            self.port.write(format!("pins {}", config.as_string()).as_bytes()).expect("Issue with the serial port");
                            self.wait_for("\nReady\n");
                            data_tx.send(String::from("Done")).expect("Broken link");
                        }
                        DumpPins => {
                            self.port.write("dump".as_bytes()).expect("Issue with the serial port");
                            let result = self.wait_for(" \n");
                            self.wait_for("\nReady\n");
                            data_tx.send(result).expect("Broken link");
                        }
                        StartCapture => {
                            self.port.write("start".as_bytes()).expect("Issue with the serial port");
                        }
                        StopCapture => {
                            self.port.write("s".as_bytes()).expect("Issue with the serial port");
                            // TODO: this is broken
                            let result = self.wait_for("\nReady\n");
                            data_tx.send(result).expect("Broken link");
                        }
                    }
                }
                _ => break
            }
        }
    }
}