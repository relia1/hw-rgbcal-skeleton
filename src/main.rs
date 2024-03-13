#![no_std]
#![no_main]

mod knob;
mod rgb;
mod ui;
pub use knob::*;
pub use rgb::*;
pub use ui::*;

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use embassy_executor::Spawner;
use embassy_futures::join;
// use embassy_nrf::saadc::{AnyInput, ChannelConfig, Config, Saadc};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, mutex::Mutex};
use embassy_time::Timer;
use microbit_bsp::{
    embassy_nrf::{
        bind_interrupts,
        gpio::{AnyPin, Input, Level, Output, OutputDrive, Pull},
        // peripherals::{P0_14, P0_23},
        saadc,
    },
    Button, // Microbit,
};
use num_traits::float::FloatCore;

pub static RGB_LEVELS: Mutex<ThreadModeRawMutex, [u32; 3]> = Mutex::new([0; 3]);
pub static FPS: Mutex<ThreadModeRawMutex, u64> = Mutex::new(1);
pub const LEVELS: u32 = 16;

async fn get_rgb_levels() -> [u32; 3] {
    let rgb_levels = RGB_LEVELS.lock().await;
    *rgb_levels
}

async fn set_rgb_levels<F>(setter: F)
where
    F: FnOnce(&mut [u32; 3]),
{
    let mut rgb_levels = RGB_LEVELS.lock().await;
    setter(&mut rgb_levels);
}

async fn get_fps() -> u64 {
    let fps = FPS.lock().await;
    *fps
}

async fn set_fps<F>(setter: F)
where
    F: FnOnce(&mut u64),
{
    let mut fps = FPS.lock().await;
    setter(&mut fps);
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    rtt_init_print!();
    let mut p = embassy_nrf::init(Default::default());
    //let mut board = Microbit::default();
    // let knob: Knob;
    // let _ldr: Ldr;
    // let shared_saadc = &mut board.saadc;

    bind_interrupts!(struct Irqs {
        SAADC => saadc::InterruptHandler;
    });

    let led_pin = |p| Output::new(p, Level::Low, OutputDrive::Standard);
    let red = led_pin(AnyPin::from(p.P0_09));
    let green = led_pin(AnyPin::from(p.P0_10));
    let blue = led_pin(AnyPin::from(p.P1_02));
    let rgb: Rgb = Rgb::new([red, green, blue], 100);

    let mut saadc_config = saadc::Config::default();
    saadc_config.resolution = saadc::Resolution::_14BIT;
    let saadc = saadc::Saadc::new(
        p.SAADC,
        Irqs,
        saadc_config,
        [
            saadc::ChannelConfig::single_ended(&mut p.P0_04), // P2
            saadc::ChannelConfig::single_ended(&mut p.P0_28), // P4
        ],
    );
    /*
        {
            let temp = &mut saadc;
            knob = Knob::new(temp).await;
        }
        {
            _ldr = Ldr::new(&mut saadc).await;
        }
    */
    let a2d = A2d::new(saadc).await;

    let mut ui = Ui::new(
        a2d,
        Input::new(AnyPin::from(p.P0_14), Pull::Up), // button_a
        Input::new(AnyPin::from(p.P0_23), Pull::Up), // button_b
    );

    join::join(rgb.run(), ui.run()).await;

    panic!("fell off end of main loop");
}
