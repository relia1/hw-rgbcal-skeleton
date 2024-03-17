use crate::*;

/// Type alias for our LED output pins
type RgbPins = [Output<'static, AnyPin>; 3];

/// This datatype provides our functionality for our LEDs
/// They will be able to have their levels changed
/// `levels` and `RgbPins` are also shared with the Ui datatype
pub struct Rgb {
    #[doc(alias = "RgbPins")]
    rgb: RgbPins,
    // Shadow variables to minimize lock contention.
    levels: [u32; 3],
    // Number of ticks per frame
    tick_time: u64,
}

/// Implementation of RGB functionality
impl Rgb {
    /// Create a new instance of RGB
    pub fn new(rgb: RgbPins, frame_rate: u64) -> Self {
        // Calculate initial frame tick time
        let tick_time = frame_tick_time(frame_rate);
        Self {
            rgb,
            levels: [0; 3],
            tick_time,
        }
    }

    /// Have the LED be on/off for certain duration based on the level out of
    /// 16 (0..16, not inclusive of 16)
    async fn step(&mut self, led: usize) {
        let level = self.levels[led];
        // When the level is greater than 0, determine the number of ticks that
        // the LED will be set to high
        if level > 0 {
            self.rgb[led].set_high();
            let on_time = level as u64 * self.tick_time;
            Timer::after_micros(on_time).await;
            self.rgb[led].set_low();
        }

        // Determine the remaining ticks for the LED to be off
        let level = LEVELS - level;
        if level > 0 {
            let off_time = level as u64 * self.tick_time;
            Timer::after_micros(off_time).await;
        }
    }

    /// Async function that runs the RGB related events
    pub async fn run(mut self) -> ! {
        loop {
            // Grab rgb levels
            self.levels = get_rgb_levels().await;
            // Grab the frame tick time
            self.tick_time = frame_tick_time(get_fps().await);
            debug_rprintln!("frame tick time {}", self.tick_time);

            // For each of the 3 LEDs perform the events related to RGB
            for led in 0..3 {
                self.step(led).await;
            }
        }
    }
}
