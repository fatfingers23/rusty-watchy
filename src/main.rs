#![no_std]
#![no_main]

use embedded_graphics::{
    image::Image,
    mono_font::{iso_8859_5::FONT_10X20, MonoTextStyle},
    pixelcolor::{BinaryColor, Rgb555, Rgb888},
    prelude::{Point, *},
    text::{Alignment, LineHeight, Text, TextStyleBuilder},
};
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Input, Io, Level, Output, Pull},
    prelude::*,
    spi::{master::Spi, SpiMode},
};
use tinybmp::Bmp;
use wepd::{Color, DelayWaiter, Display, DisplayConfiguration, Framebuffer};

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

    let bus = Spi::new(peripherals.SPI2, 100.kHz(), SpiMode::Mode0)
        .with_mosi(io.pins.gpio48)
        .with_sck(io.pins.gpio47);

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

    let mut fb = Framebuffer::new();
    let character_style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
    let text_style = TextStyleBuilder::new()
        .alignment(Alignment::Left)
        .line_height(LineHeight::Pixels(50))
        .build();

    Text::with_text_style(
        "Hello World",
        Point::new(5, 15),
        character_style,
        text_style,
    )
    .draw(&mut fb);

    // let style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
    // Text::new("Hello world", Point { x: 5, y: 40 }, style)
    //     .draw(&mut fb)
    //     .unwrap();

    let bmp_data = include_bytes!("../ferris.bmp");
    // let idk = Bmp::<Rgb888>::from_slice(bmp_data).unwrap();

    let bmp: Bmp<BinaryColor> = Bmp::from_slice(bmp_data).unwrap();
    Image::new(&bmp, Point::new(10, 20)).draw(&mut fb);
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
