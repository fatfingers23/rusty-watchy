#![no_std]
#![no_main]

use embedded_graphics::{
    mono_font::{
        ascii::{FONT_6X10, FONT_9X18_BOLD},
        iso_8859_5::FONT_10X20,
        MonoTextStyle,
    },
    pixelcolor::Rgb565,
    prelude::{Point, RgbColor, *},
    text::Text,
};
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Input, Io, Level, Output, Pull},
    prelude::*,
    spi::{master::Spi, SpiMode},
};
use wepd::{DelayWaiter, Display, DisplayConfiguration, Framebuffer};

extern crate alloc;
use core::mem::MaybeUninit;

fn init_heap() {
    const HEAP_SIZE: usize = 32 * 1024;
    static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();

    unsafe {
        esp_alloc::HEAP.add_region(esp_alloc::HeapRegion::new(
            HEAP.as_mut_ptr() as *mut u8,
            HEAP_SIZE,
            esp_alloc::MemoryCapability::Internal.into(),
        ));
    }
}

#[entry]
fn main() -> ! {
    #[allow(unused)]
    esp_println::logger::init_logger_from_env();

    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();
    init_heap();

    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let miso = io.pins.gpio46;

    // let cs = io.pins.gpio33;

    let miso = miso.peripheral_input();
    // let mosi = mosi.into_peripheral_output();

    let mut bus = Spi::new(peripherals.SPI2, 100.kHz(), SpiMode::Mode0)
        .with_mosi(io.pins.gpio48)
        .with_sck(io.pins.gpio47);

    // .with_pins(sclk, mosi, miso, cs);

    let mut display = Display::new(DisplayConfiguration {
        spi: ExclusiveDevice::new(bus, Output::new(io.pins.gpio33, Level::High), delay).unwrap(),
        dc: Output::new(io.pins.gpio34, Level::High),
        rst: Output::new(io.pins.gpio35, Level::High),
        busy: Input::new(io.pins.gpio36, Pull::None),
        delay,
        busy_wait: DelayWaiter::new(delay)
            .with_timeout_ms(100_000)
            .with_delay_ms(1),
    })
    .unwrap();

    display.reset().unwrap();

    display.clear_screen(0xFF).unwrap();
    // display.draw_image(bitmap, x_lo, y_lo, x_hi, y_hi)
    // Create a new character style
    // let style = MonoTextStyle::new(&FONT_6X10, Rgb565::BLACK);

    // Create a text at position (20, 30) and draw it using the previously defined style

    // Text::new("Hello Rust!", Point::new(20, 30), style).draw(&mut display);

    let mut fb = Framebuffer::new();

    let style = MonoTextStyle::new(&FONT_9X18_BOLD, wepd::Color::Black);
    Text::new("Hello world", Point { x: 5, y: 15 }, style)
        .draw(&mut fb)
        .unwrap();
    fb.flush(&mut display).unwrap();

    // let result = display.draw_image(include_bytes!("../image.bin"), 0, 0, 200, 200);
    // if let Err(e) = result {
    //     log::error!("Error writing image: {:?}", e);
    // }
    log::info!("Hello world!");
    loop {
        delay.delay(500.millis());
    }
}
