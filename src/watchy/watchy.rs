use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::{Point, *},
    text::Text,
};
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_hal::{
    delay::Delay,
    gpio::{Input, Io, Level, Output, Pull},
    peripherals::{Peripherals, SPI2},
    prelude::*,
    spi::{master::Spi, FullDuplexMode, SpiMode},
};
use log::{error, info};
use wepd::{DelayWaiter, Display, DisplayConfiguration, Framebuffer};
pub struct Configy<'a> {
    text_style: MonoTextStyle<'a, BinaryColor>,
}

impl<'a> Configy<'a> {
    pub fn default() -> Self {
        Configy {
            text_style: MonoTextStyle::new(&FONT_10X20, BinaryColor::Off),
        }
    }
}

pub struct Watchy<'a> {
    config: Configy<'a>,
    pub display: Display<
        DisplayConfiguration<
            ExclusiveDevice<Spi<'a, SPI2, FullDuplexMode>, Output<'a>, Delay>,
            Output<'a>,
            Output<'a>,
            Input<'a>,
            Delay,
            DelayWaiter<Delay>,
        >,
    >,
    pub frame_buffer: Framebuffer,
}

impl<'a> Watchy<'a> {
    //TODO: Look at doing feature flags for previous versions of watchy
    //TODO: Make a override to pass in config
    pub fn new(peripherals: Peripherals) -> Self {
        let delay = Delay::new();

        //Display setup
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

        Watchy {
            config: Configy::default(),
            display: display,
            frame_buffer: Framebuffer::new(),
        }
    }

    pub fn write_text(&mut self, text: &str, point: Point) -> Result<(), DisplayErrors> {
        self.write_different_style_text(text, point, self.config.text_style)
    }

    pub fn write_different_style_text(
        &mut self,
        text: &str,
        point: Point,
        style: MonoTextStyle<'a, BinaryColor>,
    ) -> Result<(), DisplayErrors> {
        let result = Text::new(text, point, style).draw(&mut self.frame_buffer);
        match result {
            Ok(_) => {
                let draw_to_buffer = self.frame_buffer.flush(&mut self.display);
                info!("Drawing to buffer");
                if let Err(e) = draw_to_buffer {
                    error!("{:?}", e);
                    return Err(DisplayErrors::ErrorLoadingEmbeddedText);
                }
            }
            Err(_) => {
                return Err(DisplayErrors::CouldNotDrawText);
            }
        }

        Ok(())
    }
}

pub enum DisplayErrors {
    ErrorLoadingEmbeddedText,
    CouldNotDrawText,
}
