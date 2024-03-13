use crate::*;

struct UiState {
    levels: [u32; 3],
    frame_rate: u64,
}

impl UiState {
    fn show(&self) {
        let names = ["red", "green", "blue"];
        rprintln!();
        for (name, level) in names.iter().zip(self.levels.iter()) {
            rprintln!("{}: {}", name, level);
        }
        rprintln!("frame rate: {}", self.frame_rate);
    }
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            levels: [LEVELS - 1, LEVELS - 1, LEVELS - 1],
            frame_rate: 100,
        }
    }
}

pub struct Ui /*<'a>*/ {
    //knob: Knob<'a>,
    a2d: A2d,
    button_a: Button,
    button_b: Button,
    state: UiState,
}

impl Ui /*<'_>*/ {
    pub fn new(/*knob: Knob*/ a2d: A2d, button_a: Button, button_b: Button) -> Self {
        Self {
            // knob,
            a2d,
            button_a,
            button_b,
            state: UiState::default(),
        }
    }

    pub async fn run(&mut self) -> ! {
        /*
        self.update_fps().await;
        self.update_level(0).await;
        self.update_level(1).await;
        self.update_level(2).await;
        */
        loop {
            let led_index = match (self.button_a.is_low(), self.button_b.is_low()) {
                (true, true) => 0,  // red
                (true, false) => 2, // blue
                (false, true) => 1, // green
                _ => 99,
            };

            if led_index > 2 {
                // update framerate
                #[cfg(debug_assertions)]
                rprintln!("Checking if fps are being updated\n");
                self.update_fps().await;
            } else {
                #[cfg(debug_assertions)]
                rprintln!("Checking if rgb levels are being updated\n");
                self.update_level(led_index).await;
            }
            // Timer::after_millis(1000 / get_fps().await).await;
            // Timer::after_millis(400).await;
        }
    }

    pub async fn init(&mut self) {}
    pub async fn update_level(&mut self, led_index: usize) {
        // let level = self.knob.measure().await;
        let level = self.a2d.measure_knob().await;
        if level != self.state.levels[led_index] {
            self.state.levels[led_index] = level;
            self.state.show();
            set_rgb_levels(|rgb| {
                *rgb = self.state.levels;
            })
            .await;
        }
    }

    pub async fn update_fps(&mut self) {
        let fps: u64 = (self.a2d.measure_knob().await as u64 * 10) + 10;
        // rprintln!("fps: {}\n", fps);
        //let ldr = self.a2d.measure_ldr().await;
        //rprintln!("ldr measurement: {}\n", ldr);
        if fps != self.state.frame_rate {
            self.state.frame_rate = fps;
            // self.state.show();
            rprintln!("setting fps: {}\n", self.state.frame_rate);
            set_fps(|fps| {
                *fps = self.state.frame_rate;
            })
            .await;
        }
    }
}
