// https://github.com/esp-rs/esp-idf-template
// https://github.com/Matrixchung/SFM-V1.7
// https://github.com/esp-rs/esp-idf-hal/blob/master/examples/blinky.rs
// https://esp-rs.github.io/esp-idf-hal/esp_idf_hal/uart/index.html
// https://x2robotics.ca/uart-capacitive-fingerprint-sensor-kit-sfm-v-1-7
// https://microcontrollerslab.com/wp-content/uploads/2019/02/ESP32-pinout.png
// https://github.com/esp-rs/esp-idf-hal/blob/master/examples/uart_loopback.rs
// https://github.com/milewski/sensors-esp/blob/7c415bb29701eb651625077be37969a54b82f647/esp32/mp3-player/src/main.rs
// https://github.com/Matrixchung/SFM-V1.7/blob/main/examples/Touch_and_unlock/Touch_and_unlock.ino

use std::time::Duration;

use crate::sfm::driver::SfmUart;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::{AnyInputPin, AnyOutputPin, PinDriver};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::uart::config::Config;
use esp_idf_hal::uart::config::DataBits;
use esp_idf_hal::uart::UartDriver;
use esp_idf_hal::units::Hertz;
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use log::info;

mod sfm;
use sfm::error::SfmError;

use crate::sfm::command::CommandBuffer;
use crate::sfm::driver::SfmUartDriver;

use crate::sfm::command::send_command;

// static SFM_SERIAL_TIMEOUT: u32    = 8000; // serial timeout (ms)
// static SFM_DEFAULT_USERROLE: u8  = 0x03;  // Default user role for register,
// static SFM_ACK_SUCCESS: u8       = 0x00;  // Command successful
// static SFM_ACK_FAIL: u8          = 0x01;  // Command failed
// static SFM_ACK_FULL: u8          = 0x04;  // Database full
// static SFM_ACK_NOUSER: u8        = 0x05;  // User does not exist
// static SFM_ACK_USER_EXIST: u8    = 0x07;  // User exists
// static SFM_ACK_TIMEOUT: u8       = 0x08;  // Image collection timeout
// static SFM_ACK_HWERROR: u8       = 0x0A;  // Hardware error
// static SFM_ACK_IMGERROR: u8      = 0x10;  // Image error
// static SFM_ACK_BREAK: u8         = 0x18;  // Stop current cmd
// static SFM_ACK_ALGORITHMFAIL: u8 = 0x11;  // Film/Mask attack detected
// static SFM_ACK_HOMOLOGYFAIL: u8  = 0x12;  // Homology check fail
// static SFM_ACK_SERIALTIMEOUT: u8 = 0x13;  // Serial receive time exceeds SFM_SERIAL_TIMEOUT
// static SFM_ACK_IDLE: u8          = 0x14;  // Module idle
// static SFM_RING_OFF: u8          = 0x07;  // Ring LED Off
// static SFM_RING_RED: u8          = 0x03;  // Ring Color Red
// static SFM_RING_GREEN: u8        = 0x05;  // Ring Color Green
// static SFM_RING_BLUE: u8         = 0x06;  // Ring Color Blue
// static SFM_RING_YELLOW: u8       = 0x01;  // Ring Color Yellow
// static SFM_RING_PURPLE: u8       = 0x02;  // Ring Color Purple
// static SFM_RING_CYAN: u8         = 0x04;  // Ring Color Cyan

static ACK_START: u8 = 0xF5;
static DELAY_DURATION_MS: u32 = 100;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();

    let tx = peripherals.pins.gpio17;
    let rx = peripherals.pins.gpio16;
    let touch = peripherals.pins.gpio18;

    let config = Config::default()
        .baudrate(Hertz(115_200))
        .data_bits(DataBits::DataBits8);

    let driver = SfmUartDriver(
        UartDriver::new(
            peripherals.uart2,
            rx,
            tx,
            Option::<AnyInputPin>::None,
            Option::<AnyOutputPin>::None,
            &config,
        )
        .unwrap(),
    );

    let led = PinDriver::input(touch).unwrap();

    let mut buffer = CommandBuffer::default();

    loop {
        let is_touched = led.is_high();
        info!("is touched? {}", is_touched);

        let read = driver.read(&mut buffer, DELAY_DURATION_MS).unwrap();
        info!("read {}: {:?}", read, buffer);

        if is_touched {
            send_command(&driver, 0xC3, [0x03, 0x00, 0x00], Duration::from_secs(5)).unwrap();
            let read = driver.read(&mut buffer, 100).unwrap();
            info!("read {}: {:?}", read, buffer);
        }

        FreeRtos::delay_ms(DELAY_DURATION_MS);
    }
}
