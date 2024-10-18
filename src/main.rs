#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embedded_graphics::prelude::Point;
use esp_backtrace as _;
use esp_hal::{delay::Delay, prelude::*};
use watchy::{
    watchy::Watchy,
    widget::{
        default_widgets::{TimeWidget, WeatherWidget},
        Widget,
    },
};
extern crate alloc;
use core::{borrow::Borrow, mem::MaybeUninit, time};
mod watchy;

fn init_heap() {
    const HEAP_SIZE: usize = 72 * 1024;
    static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();

    unsafe {
        esp_alloc::HEAP.add_region(esp_alloc::HeapRegion::new(
            HEAP.as_mut_ptr() as *mut u8,
            HEAP_SIZE,
            esp_alloc::MemoryCapability::Internal.into(),
        ));
    }
}

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) -> ! {
    #[allow(unused)]
    esp_println::logger::init_logger_from_env();

    let delay = Delay::new();
    init_heap();
    //Really don't know how I feel just passing over all of the peripherals
    //The idea currently is this just lets the end user with no schmatic knowlege easily get to writing watch faces
    //Maybe create one that just takes all the different peripherals and passes them over for those who want more control
    //or to implement features not done here
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut watchy = Watchy::new(peripherals);
    let time_widget = TimeWidget {};
    let weather_widget = WeatherWidget { weather: "Sunny" };
    weather_widget.draw_widget(&mut watchy, Point { x: 5, y: 15 });

    time_widget.draw_widget(&mut watchy, Point { x: 50, y: 50 });

    // let _ = watchy.write_text("Hello world", Point { x: 5, y: 15 });

    log::info!("Hello world!");
    loop {
        delay.delay(5_000.millis());
    }
}
