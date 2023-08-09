use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

use rppal::spi::{Spi, Bus, SlaveSelect, Mode};
use rppal::gpio::{Gpio, Pin};

use embedded_nrf24l01::{NRF24L01, ChangeModes, Tx};
use embedded_nrf24l01::config::{NRF24L01Config, InterruptMask, RetransmitConfig, CrcMode, DataRate, PALevel};

fn main() -> Result<(), Box<dyn Error>> {
    let gpio = Gpio::new()?;

    println!("Initializing Radio");
    let radio_spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 8_000_000, Mode::Mode0)?;

    let radio_ce = gpio.get(1u8)?;
    let radio_ce = Pin::into_output(radio_ce);

    let radio_cs = gpio.get(2u8)?;
    let radio_cs = Pin::into_output(radio_cs);

    let radio_config = NRF24L01Config {
        data_rate: DataRate::R1Mbps,
        crc_mode: CrcMode::Disabled,
        rf_channel: 0u8,
        pa_level: PALevel::PA0dBm,
        interrupt_mask: InterruptMask { data_ready_rx: true, data_sent_tx: false, max_retramsits_tx: false },
        read_enabled_pipes: [true; 6],
        rx_addrs: [b"aag", b"aah", b"aai", b"aaj", b"aak", b"aal"],
        tx_addr: b"aaa",
        retransmit_config: RetransmitConfig { delay: 0u8, count: 0u8 },
        auto_ack_pipes: [false; 6],
        address_width: 3u8,
        pipe_payload_lengths: [None; 6],
    };

    let mut nrf_radio = match NRF24L01::new_with_config(radio_ce, radio_cs, radio_spi, radio_config) {
        Err(err) => panic!("Error Initializing NRF24L01: {:?}", err),
        Ok(radio) => radio,
    };

    for _ in 0..=100 {
        nrf_radio.to_tx().unwrap();
        match nrf_radio.can_send() {
            Ok(can_send) => {
                if can_send {
                    nrf_radio.send(b"Hello").unwrap();
                    nrf_radio.poll_send().unwrap();
                    nrf_radio.to_rx().unwrap();
                } else {
                    println!("Cannot Send Data");
                }
            },
            Err(err) => println!("Error in attempt to send: {:?}", err),
        }

        sleep(Duration::from_millis(1000));
    }

    Ok(())
}
