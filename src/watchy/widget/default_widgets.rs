use super::{Widget, WidgetError};
use crate::watchy::watchy::Watchy;
use embedded_graphics::{
    image::Image,
    pixelcolor::BinaryColor,
    prelude::{Point, Size, *},
    primitives::Rectangle,
};
use tinybmp::Bmp;

pub struct TimeWidget {}

enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
    P,
    M,
    Colon,
}

impl Digit {
    fn get_dimensions(&self) -> (i32, i32, u32, u32) {
        match self {
            Digit::Zero => (0, 0, 31, 49),
            Digit::One => (32, 0, 9, 49),
            Digit::Two => (42, 0, 31, 49),
            Digit::Three => (74, 0, 31, 49),
            Digit::Four => (106, 0, 31, 49),
            Digit::Five => (138, 0, 31, 49),
            Digit::Six => (170, 0, 31, 49),
            Digit::Seven => (202, 0, 31, 49),
            Digit::Eight => (234, 0, 31, 49),
            Digit::Nine => (266, 0, 31, 49),
            Digit::A => (298, 0, 31, 49),
            Digit::P => (330, 0, 31, 49),
            Digit::M => (362, 0, 31, 49),
            Digit::Colon => (394, 0, 31, 49),
        }
    }
}

impl Widget for TimeWidget {
    fn draw_widget(&self, watchy: &mut Watchy, location: Point) -> Result<(), WidgetError> {
        let raw_data = include_bytes!("../../../assets/digital-7.bmp");
        let bmp: Bmp<BinaryColor> = Bmp::from_slice(raw_data).unwrap();

        let digits = [
            Digit::One,
            Digit::One,
            Digit::Colon,
            Digit::Zero,
            Digit::Zero,
            Digit::P,
            Digit::M,
        ];

        let mut x_offset = 10;
        for digit in &digits {
            let (x, y, width, height) = digit.get_dimensions();
            let sub_image =
                bmp.sub_image(&Rectangle::new(Point::new(x, y), Size::new(width, height)));
            Image::new(&sub_image, Point::new(x_offset, 50)).draw(&mut watchy.frame_buffer);

            //31 is the fontwidth of the digits
            x_offset += 31 as i32; // Adjust spacing as needed
        }

        watchy.frame_buffer.flush(&mut watchy.display).unwrap();

        Ok(())
    }
}

pub struct WeatherWidget<'a> {
    pub weather: &'a str,
}

impl<'a> Widget for WeatherWidget<'a> {
    fn draw_widget(&self, watchy: &mut Watchy, location: Point) -> Result<(), WidgetError> {
        watchy.write_text(&self.weather, location);
        Ok(())
    }
}
