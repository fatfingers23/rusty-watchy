use embedded_graphics::{
    mono_font::{
        ascii::{self, FONT_10X20},
        MonoTextStyle,
    },
    pixelcolor::BinaryColor,
    prelude::{Point, *},
    text::Text,
};
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_hal::{
    delay::Delay,
    gpio::{Input, Io, Level, Output, Pull},
    peripheral::{Peripheral, PeripheralRef},
    peripherals::{Peripherals, SPI2, WIFI},
    prelude::*,
    rng::Rng,
    spi::{master::Spi, FullDuplexMode, SpiMode},
    time,
    timer::timg::TimerGroup,
};
use esp_println::println;
use esp_wifi::{
    init,
    wifi::{
        utils::create_network_interface, AccessPointInfo, ClientConfiguration, Configuration,
        WifiError, WifiStaDevice,
    },
    wifi_interface::WifiStack,
    EspWifiInitFor,
};
use log::error;
use smoltcp::iface::SocketStorage;
use wepd::{DelayWaiter, Display, DisplayConfiguration, Framebuffer};

//TODO must pass over?
const SSID: &str = env!("SSID");
const PASSWORD: &str = env!("PASSWORD");

pub struct Configy<'a> {
    text_style: MonoTextStyle<'a, BinaryColor>,
    socket_set_entries: [SocketStorage<'a>; 3],
}

impl<'a> Configy<'a> {
    pub fn default() -> Self {
        Configy {
            text_style: MonoTextStyle::new(&FONT_10X20, BinaryColor::Off),
            socket_set_entries: Default::default(),
        }
    }
}

pub struct Wathcy<'a> {
    config: Configy<'a>,
    display: Display<
        DisplayConfiguration<
            ExclusiveDevice<Spi<'a, SPI2, FullDuplexMode>, Output<'a>, Delay>,
            Output<'a>,
            Output<'a>,
            Input<'a>,
            Delay,
            DelayWaiter<Delay>,
        >,
    >,
    frame_buffer: Framebuffer,
    wifi_stack: WifiStack<'a, WifiStaDevice>,
    wifi: PeripheralRef<'a, WIFI>,
    // socket_set_entries: [SocketStorage<'a>; 3],
}

impl<'a> Wathcy<'a> {
    //TODO: Look at doing feature flags for previous versions of watchy
    pub fn new(peripherals: Peripherals, mut wifi: impl Peripheral<P = WIFI> + 'a) -> Self {
        let config = Configy::default();
        let delay = Delay::new();
        let mut config = Configy::default();

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

        let timg0 = TimerGroup::new(peripherals.TIMG0);

        let init = init(
            EspWifiInitFor::Wifi,
            timg0.timer0,
            Rng::new(peripherals.RNG),
            peripherals.RADIO_CLK,
        )
        .unwrap();

        // let mut wifi = peripherals.WIFI;
        let (iface, device, mut controller, sockets) = create_network_interface(
            &init,
            &mut wifi,
            WifiStaDevice,
            &mut config.socket_set_entries,
        )
        .unwrap();
        let now = || time::now().duration_since_epoch().to_millis();
        let wifi_stack = WifiStack::new(iface, device, sockets, now);

        let client_config = Configuration::Client(ClientConfiguration {
            ssid: SSID.try_into().unwrap(),
            password: PASSWORD.try_into().unwrap(),
            ..Default::default()
        });
        let res = controller.set_configuration(&client_config);
        println!("wifi_set_configuration returned {:?}", res);

        controller.start().unwrap();
        println!("is wifi started: {:?}", controller.is_started());

        println!("Start Wifi Scan");
        let res: Result<(heapless::Vec<AccessPointInfo, 10>, usize), WifiError> =
            controller.scan_n();
        if let Ok((res, _count)) = res {
            for ap in res {
                println!("{:?}", ap);
            }
        }

        println!("{:?}", controller.get_capabilities());
        println!("wifi_connect {:?}", controller.connect());

        // wait to get connected
        println!("Wait to get connected");
        loop {
            match controller.is_connected() {
                Ok(true) => break,
                Ok(false) => {}
                Err(err) => {
                    println!("{:?}", err);
                    loop {}
                }
            }
        }
        println!("{:?}", controller.is_connected());

        // wait for getting an ip address
        println!("Wait to get an ip address");
        loop {
            wifi_stack.work();

            if wifi_stack.is_iface_up() {
                println!("got ip {:?}", wifi_stack.get_ip_info());
                break;
            }
        }

        println!("Start busy loop on main");

        // let mut rx_buffer = [0u8; 1536];
        // let mut tx_buffer = [0u8; 1536];
        // let mut socket = wifi_stack.get_socket(&mut rx_buffer, &mut tx_buffer);

        Wathcy {
            config,
            display,
            frame_buffer: Framebuffer::new(),
            wifi_stack,
            wifi: wifi.into_ref(),
            // socket_set_entries: config.socket_set_entries,
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
