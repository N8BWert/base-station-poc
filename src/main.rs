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

// use rppal::spi::{Spi, Bus, SlaveSelect, Mode};
// use rppal::gpio::{Gpio, Pin};

fn main() -> Result<(), Box<dyn Error>> {
    let (tx, rx) = watch::channel(false);

    ctrlc::set_handler(move || {
        tx.send(true).unwrap();
    }).expect("Could not Set Ctrl-C Handler");

    let (field_status_code, robot_status_code) = thread::scope(|s| {
        let field_computer_handle = s.spawn(|| {
            let rx = rx.clone();
            let mut listener = UdpSocket::bind("0.0.0.0:8000").expect("Could not Bind to Udp Socket");
            listener.set_nonblocking(false).unwrap();
            listener.set_read_timeout(Some(Duration::from_millis(100))).unwrap();

            while !rx.has_changed().unwrap() {
                wait_for_incoming_field_messages(&mut listener);
            }

            return;
        });

        let robot_handle = s.spawn(|| {
            let rx = rx.clone();

            while !rx.has_changed().unwrap() {
                wait_for_incoming_robot_message();
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
    let mut buffer = [0u8; 10];
    if let Ok(message_size) = listener.recv(&mut buffer) {
        let robot_id: u8 = buffer[0] & 0b11111100 >> 2;

        println!("Got Message of Size: {}\nSending to Robot: {}", message_size, robot_id);

        // TODO: Send data to radio
    }
}

fn wait_for_incoming_robot_message() {
    thread::sleep(Duration::from_millis(1_000));
}
