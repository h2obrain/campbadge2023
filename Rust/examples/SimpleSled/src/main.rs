#![feature(str_split_remainder)]
#![feature(strict_provenance)]
#![feature(lazy_cell)]

mod led_matrix;
mod sled_hijack;

use crate::sled_hijack::sled_modules;

use esp_idf_hal::peripherals::Peripherals;
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use log::*;

use smart_leds::RGB8;

use esp_idf_hal::gpio::PinDriver;
use std::thread::sleep;
use std::time::Duration;

use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::nvs::EspDefaultNvsPartition;


use sled_hijack::SLED_STATE;


fn main() -> ! {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    let _nvs = EspDefaultNvsPartition::take().unwrap();

    // let led_pin = 1;
    // let led_channel = 0;

    let peripherals = Peripherals::take().unwrap();
    let mut vcc_pin = PinDriver::output(peripherals.pins.gpio0).unwrap();
    vcc_pin.set_high().unwrap();

    // let mut leds = LedMatrix::new(led_pin, led_channel, 5, 5);
    // let mut led_state = LedState::new();
    
    info!("Hello, world!!");

    {
        info!("locking leds");
        let mut leds = SLED_STATE.leds.lock().unwrap();
        info!("set all pixels");
        leds.set_all_pixel(RGB8::new(25, 0, 0));
        leds.write_pixels();
    }
    
    let _sysloop = EspSystemEventLoop::take().unwrap();
    
    {
        let mut leds = SLED_STATE.leds.lock().unwrap();
        leds.set_all_pixel(RGB8::new(0, 25, 25));
        leds.write_pixels();
    }

    loop {
        // led_state = led_state.tick(&mut leds);
        SLED_STATE.sled_tick();
        sleep(Duration::from_millis(1));
    }
}
