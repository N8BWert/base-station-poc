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
use std::thread::sleep;
use std::time::Duration;
use std::net::UdpSocket;
use std::thread;

use tokio::sync::watch;

use rppal::spi::{Spi, Bus, SlaveSelect, Mode};
use rppal::gpio::{Gpio, Pin};

fn main() -> Result<(), Box<dyn Error>> {

    let (tx, rx) = watch::channel(false);

    thread::scope(|s| {
        let field_computer_handle = s.spawn(|| {
            let rx = rx.clone();

            while !rx.has_changed().unwrap() {

            }

            return;
        });

        let robot_handle = s.spawn(|| {
            let rx = rx.clone();

            while !rx.has_changed().unwrap() {

            }

            return;
        });

        (field_computer_handle.join(), robot_handle.join())
    });

    Ok(())
}
