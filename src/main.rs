#![no_std]
#![no_main]

use embedded_graphics::prelude::Point;
use esp_backtrace as _;
use esp_hal::{delay::Delay, prelude::*};
use watchy::watchy::Wathcy;

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
    let mut watchy = Wathcy::new(peripherals);
    let _ = watchy.write_text("Hello world", Point { x: 5, y: 15 });

    // let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    // let bus = Spi::new(peripherals.SPI2, 100.kHz(), SpiMode::Mode0)
    //     .with_mosi(io.pins.gpio48)
    //     .with_sck(io.pins.gpio47);

    // let mut display = Display::new(DisplayConfiguration {
    //     spi: ExclusiveDevice::new(bus, Output::new(io.pins.gpio33, Level::High), delay).unwrap(),
    //     dc: Output::new(io.pins.gpio34, Level::High),
    //     rst: Output::new(io.pins.gpio35, Level::High),
    //     busy: Input::new(io.pins.gpio36, Pull::None),
    //     delay,
    //     busy_wait: DelayWaiter::new(delay)
    //         .with_timeout_ms(100_000)
    //         .with_delay_ms(1),
    // })
    // .unwrap();

    // display.reset().unwrap();

    // display.clear_screen(0xFF).unwrap();

    // let mut fb = wepd::Framebuffer::new();

    // let style = MonoTextStyle::new(&ascii::FONT_10X20, BinaryColor::Off);
    // Text::new("Hello world", Point { x: 5, y: 15 }, style)
    //     .draw(&mut fb)
    //     .unwrap();

    // let bmp_data = include_bytes!("../ferris.bmp");
    // let bmp: Bmp<BinaryColor> = Bmp::from_slice(bmp_data).unwrap();
    // let _ = Image::new(&bmp, Point::new(10, 20)).draw(&mut fb);

    // fb.flush(&mut display).unwrap();

    // let timg0 = TimerGroup::new(peripherals.TIMG0);

    // let init = init(
    //     EspWifiInitFor::Wifi,
    //     timg0.timer0,
    //     Rng::new(peripherals.RNG),
    //     peripherals.RADIO_CLK,
    // )
    // .unwrap();

    // let mut wifi = peripherals.WIFI;
    // let mut socket_set_entries: [SocketStorage; 3] = Default::default();
    // let (iface, device, mut controller, sockets) =
    //     create_network_interface(&init, &mut wifi, WifiStaDevice, &mut socket_set_entries).unwrap();
    // let now = || time::now().duration_since_epoch().to_millis();
    // let wifi_stack = WifiStack::new(iface, device, sockets, now);

    // let client_config = Configuration::Client(ClientConfiguration {
    //     ssid: SSID.try_into().unwrap(),
    //     password: PASSWORD.try_into().unwrap(),
    //     ..Default::default()
    // });
    // let res = controller.set_configuration(&client_config);
    // println!("wifi_set_configuration returned {:?}", res);

    // controller.start().unwrap();
    // println!("is wifi started: {:?}", controller.is_started());

    // println!("Start Wifi Scan");
    // let res: Result<(heapless::Vec<AccessPointInfo, 10>, usize), WifiError> = controller.scan_n();
    // if let Ok((res, _count)) = res {
    //     for ap in res {
    //         println!("{:?}", ap);
    //     }
    // }

    // println!("{:?}", controller.get_capabilities());
    // println!("wifi_connect {:?}", controller.connect());

    // // wait to get connected
    // println!("Wait to get connected");
    // loop {
    //     match controller.is_connected() {
    //         Ok(true) => break,
    //         Ok(false) => {}
    //         Err(err) => {
    //             println!("{:?}", err);
    //             loop {}
    //         }
    //     }
    // }
    // println!("{:?}", controller.is_connected());

    // // wait for getting an ip address
    // println!("Wait to get an ip address");
    // loop {
    //     wifi_stack.work();

    //     if wifi_stack.is_iface_up() {
    //         println!("got ip {:?}", wifi_stack.get_ip_info());
    //         break;
    //     }
    // }

    // println!("Start busy loop on main");

    // let mut rx_buffer = [0u8; 1536];
    // let mut tx_buffer = [0u8; 1536];
    // let mut socket = wifi_stack.get_socket(&mut rx_buffer, &mut tx_buffer);

    log::info!("Hello world!");
    loop {
        delay.delay(500.millis());
    }
}
