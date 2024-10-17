use embedded_graphics::{
    image::Image,
    mono_font::{
        ascii::{self, FONT_10X20},
        MonoTextStyle,
    },
    pixelcolor::BinaryColor,
    prelude::{Point, *},
    text::{Alignment, LineHeight, Text, TextStyleBuilder},
};
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_hal::{
    delay::Delay,
    gpio::{Input, Io, Level, Output, Pull},
    peripheral::Peripheral,
    peripherals::Peripherals,
    prelude::*,
    spi::{master::Spi, FullDuplexMode, SpiMode},
};
use wepd::{DelayWaiter, Display, DisplayConfiguration, Framebuffer, IsDisplayConfiguration};

pub struct Wathcy<'a> {
    display: wepd::Display<
        DisplayConfiguration<
            ExclusiveDevice<
                esp_hal::spi::master::Spi<'a, esp_hal::peripherals::SPI2, FullDuplexMode>,
                esp_hal::gpio::Output<'a>,
                Delay,
            >,
            esp_hal::gpio::Output<'a>,
            esp_hal::gpio::Output<'a>,
            esp_hal::gpio::Input<'a>,
            Delay,
            DelayWaiter<Delay>,
        >,
    >,
}

impl<'a> Wathcy<'a> {
    //TODO: Look at doing feature flags for previous versions of watchy
    pub fn new(peripherals: Peripherals) -> Self {
        let delay = Delay::new();

        let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

        let bus = Spi::new(peripherals.SPI2, 100.kHz(), SpiMode::Mode0)
            .with_mosi(io.pins.gpio48)
            .with_sck(io.pins.gpio47);

        let mut display = Display::new(DisplayConfiguration {
            spi: ExclusiveDevice::new(bus, Output::new(io.pins.gpio33, Level::High), delay)
                .unwrap(),
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
        Wathcy { display }
    }

    pub fn write_some_text(&mut self) {
        let mut fb = Framebuffer::new();
        let style = MonoTextStyle::new(&ascii::FONT_10X20, BinaryColor::Off);
        Text::new("Hello world", Point { x: 5, y: 15 }, style)
            .draw(&mut fb)
            .unwrap();
        fb.flush(&mut self.display).unwrap();
    }
}
