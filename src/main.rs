#![no_std]
#![no_main]

mod a2d;
mod macros;
mod rgb;
mod ui;
/// Reexports
pub use a2d::*;

pub use rgb::*;
pub use ui::*;

use panic_rtt_target as _;
use rtt_target::{debug_rprintln, rprintln, rtt_init_print};

use embassy_executor::Spawner;
use embassy_futures::join;
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, mutex::Mutex};
use embassy_time::Timer;
use microbit_bsp::{
    embassy_nrf::{
        bind_interrupts,
        gpio::{AnyPin, Input, Level, Output, OutputDrive, Pull},
        saadc,
    },
    Button,
};
use num_traits::float::FloatCore;

/// Global mutex tracking the scaled value from the potentiometer assigned to the LED
pub static RGB_LEVELS: Mutex<ThreadModeRawMutex, [u32; 3]> = Mutex::new([0; 3]);
/// Global mutex tracking the scaled value from the potentiometer for the FPS
pub static FPS: Mutex<ThreadModeRawMutex, u64> = Mutex::new(10);
/// Global mutex tracking the scaled ldr values
pub static LDR_LEVELS: Mutex<ThreadModeRawMutex, [u32; 3]> = Mutex::new([0; 3]);
// Global constant for the number of levels in our scaling
pub const LEVELS: u32 = 16;

async_getter!(
    /// Async function that returns the RGB levels for the LEDs
    get_rgb_levels,
    rgb_levels,
    RGB_LEVELS,
    [u32; 3]
);
/*async fn get_rgb_levels() -> [u32; 3] {
    let rgb_levels = RGB_LEVELS.lock().await;
    *rgb_levels
}*/

async_setter!(
    /// Async function that sets the RGB levels for the LEDs
    /// F must implement the FnOnce trait
    set_rgb_levels,
    rgb_levels,
    [u32; 3],
    RGB_LEVELS,
    {
        let mut fps = FPS.lock().await;
        setter(&mut fps);
    }
);

/*
async fn set_rgb_levels<F>(setter: F)
where
    F: FnOnce(&mut [u32; 3]),
{
    let mut rgb_levels = RGB_LEVELS.lock().await;
    setter(&mut rgb_levels);
}
*/

/*
/// Async function that returns the ldr levels for the LEDs
async_getter!(get_ldr_levels, ldr_levels, LDR_LEVELS, [u32; 3]);
async fn get_ldr_levels() -> [u32; 3] {
    let ldr_levels = LDR_LEVELS.lock().await;
    *ldr_levels
}
*/

async_getter!(
    /// Async function that returns the FPS
    get_fps,
    fps,
    FPS,
    u64
);
/*
async fn get_fps() -> u64 {
    let fps = FPS.lock().await;
    *fps
}
*/

async_setter!(
    /// Async function that sets the FPS
    /// F must implement the FnOnce trait
    set_fps,
    fps,
    u64,
    FPS,
    {
        let mut fps = FPS.lock().await;
        setter(&mut fps);
    }
);

/*
async fn set_fps<F>(setter: F)
where
    F: FnOnce(&mut u64),
{
    let mut fps = FPS.lock().await;
    setter(&mut fps);
}
*/

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    rtt_init_print!();
    // Grabbing peripherals
    let mut board = embassy_nrf::init(Default::default());

    // Sets up the interrupt for SAADC
    bind_interrupts!(struct Irqs {
        SAADC => saadc::InterruptHandler;
    });

    // Closure to help set up pins
    let led_pin = |p| Output::new(p, Level::Low, OutputDrive::Standard);
    // Setting up RGB pins
    let red = led_pin(AnyPin::from(board.P0_09));
    let green = led_pin(AnyPin::from(board.P0_10));
    let blue = led_pin(AnyPin::from(board.P1_02));
    let rgb: Rgb = Rgb::new([red, green, blue], 100);

    // Setting up SAADC
    let mut saadc_config = saadc::Config::default();
    saadc_config.resolution = saadc::Resolution::_14BIT;
    let saadc = saadc::Saadc::new(
        board.SAADC,
        Irqs,
        saadc_config,
        [
            // Configure first channel to use the pin the potentiometer is wired to
            saadc::ChannelConfig::single_ended(&mut board.P0_04), // P2
            // Configure second channel to use the pin the photoresistor is wired to
            saadc::ChannelConfig::single_ended(&mut board.P0_28), // P4
        ],
    );

    // Create a new instance of our a2d with our saadc config
    let a2d = A2d::new(saadc).await;

    // Create a new Ui using our a2d and button_a/button_b
    let mut ui = Ui::new(
        a2d,
        Input::new(AnyPin::from(board.P0_14), Pull::Up), // button_a
        Input::new(AnyPin::from(board.P0_23), Pull::Up), // button_b
    );

    // Join the result of the two futures
    // Neither of the two futures should return though
    // ui.led_brightness().await;
    join::join(rgb.run(), ui.run()).await;

    panic!("fell off end of main loop");
}
