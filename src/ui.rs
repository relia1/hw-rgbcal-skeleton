use crate::*;

/// Datatype for the state of our ui device
/// Shares access to our RGB pins with the rgb datatype
struct UiState {
    levels: [u32; 3],
    frame_rate: u64,
}

/// Implementation of the UiState
impl UiState {
    /// Displays the LEDs with their associated levels and overall frame rate
    fn show(&self) {
        let names = ["red", "green", "blue"];
        rprintln!();
        for (name, level) in names.iter().zip(self.levels.iter()) {
            rprintln!("{}: {}", name, level);
        }
        rprintln!("frame rate: {}", self.frame_rate);
    }
}

/// Implement the Default trait for UiState
impl Default for UiState {
    /// Set the levels for the 3 pins to 15
    /// Set the frame rate to 100
    fn default() -> Self {
        Self {
            levels: [LEVELS - 1, LEVELS - 1, LEVELS - 1],
            frame_rate: 100,
        }
    }
}

/// Datatype for interacting with our Ui
pub struct Ui {
    // Analog to digital datatype for getting a digital representation of the readings
    // of the potentiometer and photoresistor
    a2d: A2d,
    button_a: Button,
    button_b: Button,
    state: UiState,
}

/// Implementation of our Ui datatype
impl Ui {
    /// Creates a mew instance of Ui
    pub fn new(a2d: A2d, button_a: Button, button_b: Button) -> Self {
        Self {
            a2d,
            button_a,
            button_b,
            state: UiState::default(),
        }
    }

    /// Async function for running our Ui events
    pub async fn run(&mut self) -> ! {
        for i in [0, 1, 2] {
            self.state.levels[i] = self.a2d.measure_knob().await;
        }
        set_rgb_levels(|rgb| *rgb = self.state.levels).await;

        loop {
            // If both buttons are pressed allow changes to the red LED
            // If button_a is pressed allow changes to the green LED
            // If button_b is pressed allow changes to the blue LED
            // Otherwise allow changes to the FPS
            let led_index = match (self.button_a.is_low(), self.button_b.is_low()) {
                (true, true) => 0,  // red
                (true, false) => 2, // blue
                (false, true) => 1, // green
                _ => 99,
            };

            if led_index == 99 {
                // update frame rate
                debug_rprintln!("Checking if fps are being updated\n");
                self.update_fps().await;
            } else {
                debug_rprintln!("Checking if rgb levels are being updated\n");
                self.update_level(led_index).await;
            }
        }
    }

    /// Async function for checking if the current stored level is the same as
    /// the newly received level. If so don't change anything
    /// If it is not, then update the global level value for that LED
    pub async fn update_level(&mut self, led_index: usize) {
        let level = self.a2d.measure_knob().await;
        if level != self.state.levels[led_index] {
            self.state.levels[led_index] = level;
            self.state.show();
            // Set the level using passed in closure
            set_rgb_levels(|rgb| {
                *rgb = self.state.levels;
            })
            .await;
        }
    }

    /// Async function for checking if the current stored FPS is the same as
    /// the newly received FPS. If so don't change anything
    /// If it is not, then update the global FPS
    pub async fn update_fps(&mut self) {
        let fps: u64 = convert_to_fps(self.a2d.measure_knob().await);
        if fps != self.state.frame_rate {
            self.state.frame_rate = fps;
            self.state.show();
            // Set the FPS using passed in closure
            set_fps(|fps| {
                *fps = self.state.frame_rate;
            })
            .await;
        }
    }

    /*
    /// Async function that calculates the brightness of each LED
    pub async fn led_brightness(&mut self) {
        for i in [0, 1, 2] {
            self.state.levels[i] = 0;
        }
        set_rgb_levels(|rgb| *rgb = self.state.levels).await;

        for led in 0..3 {
            self.state.levels[led] = 15;
            set_rgb_levels(|rgb| *rgb = self.state.levels).await;
            Timer::after_millis(1000).await;
            let ldr_level = self.a2d.measure_ldr().await;
            rprintln!("LED {} brightness: {}\n", led, ldr_level);
            Timer::after_millis(1000).await;
        }
    }
    */
}
