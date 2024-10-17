#![no_std]
#![no_main]

use embedded_graphics::prelude::Point;
use esp_backtrace as _;
use esp_hal::{delay::Delay, prelude::*};
use watchy::watchy::Watchy;

extern crate alloc;
use core::mem::MaybeUninit;
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

const SSID: &str = env!("SSID");
const PASSWORD: &str = env!("PASSWORD");

#[entry]
fn main() -> ! {
    #[allow(unused)]
    esp_println::logger::init_logger_from_env();

    let delay = Delay::new();
    init_heap();
    //Really don't know how I feel just passing over all of the peripherals
    //Idealy project won't need it, but eh. Need to look to just see what is needed to pass over
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut watchy = Watchy::new(peripherals);
    let _ = watchy.write_text("Hello world", Point { x: 5, y: 15 });

    log::info!("Hello world!");
    loop {
        delay.delay(5_000.millis());
    }
}
