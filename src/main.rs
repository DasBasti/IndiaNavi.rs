#![allow(unknown_lints)]
#![allow(unexpected_cfgs)]
use epd_waveshare::{
    color::Color,
    epd4in2_gd::Epd4in2_gd,
    prelude::WaveshareDisplay,
};
use esp_idf_svc::hal::{
    delay::Delay,
    gpio::PinDriver,
    prelude::Peripherals,
    spi::{config::Config, SpiDeviceDriver, SpiDriver, SpiDriverConfig, SPI2},
};

#[cfg(esp32)]
fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Start! ->");

    let peripherals = Peripherals::take()?;

    log::info!("Start GPIO Config");
    // get pins for SPI
    let mosi = peripherals.pins.gpio23;
    let miso = peripherals.pins.gpio19;
    let sck = peripherals.pins.gpio18;
    let cs = peripherals.pins.gpio27;
    
    let busy = PinDriver::input(peripherals.pins.gpio13)?;
    let dc = PinDriver::output(peripherals.pins.gpio14)?;
    let rst = PinDriver::output(peripherals.pins.gpio12)?;

    log::info!("Start Delay");
    let mut delay: Delay = Default::default();
    
    log::info!("Start SPI peripheral");
    let spi_driver = SpiDriver::new::<SPI2>(
        peripherals.spi2,
        sck,
        mosi,
        Some(miso),
        &SpiDriverConfig::new(),
    )?;
    
    log::info!("Start SPI Device");
    let config = Config::new().baudrate(1_000_000.into());
    let mut epd_spi = SpiDeviceDriver::new(&spi_driver, Some(cs), &config)?;

    log::info!("Setup epd");

    // Setup the epd
    let mut epd_dsp =
    Epd4in2_gd::new(&mut epd_spi, busy, dc, rst, &mut delay, Some(1)).expect("eink initalize error");
    epd_dsp.set_background_color(Color::White);
    epd_dsp.clear_frame(&mut epd_spi, &mut delay)?;
    epd_dsp.display_frame(&mut epd_spi, &mut delay)?;
    
    // Going to sleep
    epd_dsp.sleep(&mut epd_spi, &mut delay)?;
    loop {
        log::info!("loop");
        delay.delay_ms(1000);

    }

}
