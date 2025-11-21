#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::gpio::{Level, Output, OutputConfig, Pin};
use esp_hal::timer::timg::TimerGroup;

use esp_backtrace as _;

esp_bootloader_esp_idf::esp_app_desc!();

#[embassy_executor::task]
async fn run() {
    loop {
        esp_println::println!("Hello world from embassy using esp-hal-async!");
        Timer::after(Duration::from_millis(1_000)).await;
    }
}

// Blink a single LED pin with given period
#[embassy_executor::task(pool_size = 4)]
async fn blink(mut pin: Output<'static>, period_ms: u64) {
    loop {
        pin.set_high();
        Timer::after(Duration::from_millis(period_ms / 2)).await;
        pin.set_low();
        Timer::after(Duration::from_millis(period_ms / 2)).await;
    }
}

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    esp_println::logger::init_logger_from_env();

    // Init peripherals
    let peripherals = esp_hal::init(esp_hal::Config::default());

    esp_println::println!("Init!");

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);

    // Spawn Hello World task
    spawner.spawn(run()).unwrap();

    // Configure RGB pins as outputs
    let blue = Output::new(peripherals.GPIO21, Level::High, OutputConfig::default());
    let red = Output::new(peripherals.GPIO22, Level::High, OutputConfig::default());
    let green = Output::new(peripherals.GPIO23, Level::High, OutputConfig::default());

    // Spawn blinking tasks with different periods
    spawner.spawn(blink(blue, 1000)).unwrap();   // Blue blinks every 500ms
    spawner.spawn(blink(red, 2000)).unwrap();   // Red blinks every 1s
    spawner.spawn(blink(green, 4000)).unwrap(); // Green blinks every 1.5s

    // Optional main loop
    loop {
        esp_println::println!("Bing!");
        Timer::after(Duration::from_millis(5_000)).await;
    }
}
