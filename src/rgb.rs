use crate::*;

type RgbPins = [Output<'static, AnyPin>; 3];

pub struct Rgb {
    #[doc(alias = "RgbPins")]
    rgb: RgbPins,
    // Shadow variables to minimize lock contention.
    levels: [u32; 3],
    tick_time: u64,
}

impl Rgb {
    pub fn frame_tick_time(frame_rate: u64) -> u64 {
        1_000_000 / (3 * frame_rate * LEVELS as u64)
    }

    pub fn new(rgb: RgbPins, frame_rate: u64) -> Self {
        let tick_time = Self::frame_tick_time(frame_rate);
        Self {
            rgb,
            levels: [0; 3],
            tick_time,
        }
    }

    /// Have the LED be on/off for certain duration based on the level out of
    /// 16 (0..16)
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

        // Determine the remaining the remaining ticks for the LED to be off
        let level = LEVELS - level;
        if level > 0 {
            let off_time = level as u64 * self.tick_time;
            Timer::after_micros(off_time).await;
        }
    }

    pub async fn run(mut self) -> ! {
        loop {
            self.levels = get_rgb_levels().await;
            self.tick_time = rgb::Rgb::frame_tick_time(get_fps().await);
            #[cfg(debug_assertions)]
            rprintln!("frame tick time {}", self.tick_time);

            for led in 0..3 {
                self.step(led).await;
            }
        }
    }
}
