//!
//! The basic principle of the base station is to have two threads that do the
//! following:
//! 
//! 1. Receive Commands from the Field Computer and forward said commands to
//! the robots
//! 2. Receive Information from the Robots and forward said information to
//! the base computer
//! 
//! To accomplish this we'll either have 2 radios or one mutex locked radio shared
//! between the threads.
//! 

use std::error::Error;
use std::time::Duration;
use std::net::UdpSocket;
use std::thread;

use tokio::sync::watch;

use rppal::spi::{Spi, Bus, SlaveSelect, Mode};
use rppal::gpio::{Gpio, OutputPin};

use sx127::{self, LoRa};

pub mod robot_status_message;

const FREQUENCY: i64 = 915;
const RADIO_STARTUP_DELAY: u64 = 2000;

type Radio = LoRa<Spi, OutputPin, OutputPin, rppal::hal::Delay>;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize Radio 1
    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 8_000_000, Mode::Mode0)?;
    let gpio = Gpio::new()?;
    let fake_csn = gpio.get(7u8)?.into_output();
    let reset = gpio.get(0u8)?.into_output();

    // let mut radio = match sx127::LoRa::new(spi, fake_csn, reset, FREQUENCY, rppal::hal::Delay::new()) {
    //     Ok(sx127) => sx127,
    //     Err(err) => match err {
    //         sx127::Error::VersionMismatch(version) => panic!("Version Mismatch: {:?}", version),
    //         sx127::Error::CS(_) => panic!("Chip Select Issue"),
    //         sx127::Error::Reset(_) => panic!("Reset Issue"),
    //         sx127::Error::SPI(_) => panic!("SPI Problem"),
    //         sx127::Error::Transmitting => panic!("Error during spi transmission"),
    //         sx127::Error::Uninformative => panic!("Uninformative error RIP"),
    //     }
    // };

    // Initialize Radio 2
    // let spi = Spi::new(Bus::Spi1, SlaveSelect::Ss1, 8_000_000, Mode::Mode0)?;
    // let fake_csn = gpio.get(20u8)?.into_output();
    // let reset = gpio.get(1u8)?.into_output();

    // let mut rx_radio = match sx127::LoRa::new(spi, fake_csn, reset, FREQUENCY, rppal::hal::Delay::new()) {
    //     Ok(sx127) => sx127,
    //     Err(err) => match err {
    //         sx127::Error::VersionMismatch(version) => panic!("Version Mismatch: {:?}", version),
    //         sx127::Error::CS(_) => panic!("Chip Select Issue"),
    //         sx127::Error::Reset(_) => panic!("Reset Issue"),
    //         sx127::Error::SPI(_) => panic!("SPI Problem"),
    //         sx127::Error::Transmitting => panic!("Error during spi transmission"),
    //         sx127::Error::Uninformative => panic!("Uninformative error RIP"),
    //     }
    // };
    
    thread::sleep(Duration::from_millis(RADIO_STARTUP_DELAY));

    // println!("Lora Version: {:#04x}", radio.get_radio_version().unwrap());

    // match radio.set_tx_power(17, 1) {
    //     Ok(_) => println!("Successfully Set TX Power"),
    //     Err(_) => panic!("Error Setting Tx Power"),
    // }

    // println!("Lora Version (2): {:#04x}", rx_radio.get_radio_version().unwrap());

    // Setup Threads
    let (tx, rx) = watch::channel(false);

    ctrlc::set_handler(move || {
        tx.send(true).unwrap();
    }).expect("Could not Set Ctrl-C Handler");

    let (field_status_code, robot_status_code) = thread::scope(|s| {
        let field_computer_handle = s.spawn(|| {
            let rx = rx.clone();
            let mut listener = UdpSocket::bind("0.0.0.0:8000").expect("Could not Bind to Udp Listener");
            listener.set_nonblocking(false).unwrap();
            listener.set_read_timeout(Some(Duration::from_millis(100))).unwrap();

            while !rx.has_changed().unwrap() {
                wait_for_incoming_field_messages(&mut listener);
            }

            return;
        });

        let robot_handle = s.spawn(|| {
            let rx = rx.clone();
            let mut sender = UdpSocket::bind("0.0.0.0:8001").expect("Could not Bind to Udp Sender");

            while !rx.has_changed().unwrap() {
                wait_for_incoming_robot_message(&mut sender);
            }

            return;
        });

        (field_computer_handle.join(), robot_handle.join())
    });

    match field_status_code {
        Ok(_) => println!("Field Computer Thread Exited Successfully"),
        Err(err) => println!("Field Computer Finished with Error: {:?}", err),
    }

    match robot_status_code {
        Ok(_) => println!("Robot Thread Exited Successfully"),
        Err(err) => println!("Robot Thread Finished with Error: {:?}", err),
    }

    Ok(())
}

/// Block Execution of the Thread to Wait for an incoming Field Message.
/// 
/// The first 6 bytes of a message are the robots id;
fn wait_for_incoming_field_messages(listener: &mut UdpSocket) {
    let mut buffer = [0u8; 255];
    if let Ok(message_size) = listener.recv(&mut buffer) {
        match buffer[0] & 0b11111110 {
            0b10000000 => println!("Recevied Message for Robot 0"),
            0b01000000 => println!("Received Message for Robot 1"),
            0b00100000 => println!("Received Message for Robot 2"),
            0b00010000 => println!("Received Message for Robot 3"),
            0b00001000 => println!("Received Message for Robot 4"),
            0b00000100 => println!("Received Message for Robot 5"),
            _ => println!("Received Message for Unknown Robot"),
        }

        // TODO: Send Message to the Corresponding Robot
        // match radio.transmit_payload_busy(buffer, 10) {
        //     Ok(packet_size) => println!("Sent Packet with Size: {}", packet_size),
        //     Err(err) => println!("Error Sending Packet: {:?}", err),
        // }
    }
}

/// Check for the various 
fn wait_for_incoming_robot_message(sender: &mut UdpSocket) {
// fn wait_for_incoming_robot_message(radio: &mut Radio) {
    // TODO: Send data backwards to field computer
    // match radio.read_packet() {
    //     Ok(buffer) => println!("received data: {:?}", buffer),
    //     Err(err) => println!("Error Receiving Data: {:?}", err),
    // }


}
