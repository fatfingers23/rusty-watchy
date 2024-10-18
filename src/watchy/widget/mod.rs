pub mod default_widgets;

use embedded_graphics::prelude::Point;

use crate::watchy::watchy::Watchy;

pub trait Widget {
    fn draw_widget(&self, watchy: &mut Watchy, location: Point) -> Result<(), WidgetError>;
}

pub enum WidgetError {
    CouldNotDrawText,
}
