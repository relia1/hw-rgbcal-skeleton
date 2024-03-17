use crate::*;

/// Return the number of ticks per frame based on the frame rate and levels
pub fn frame_tick_time(frame_rate: u64) -> u64 {
    1_000_000 / (3 * frame_rate * LEVELS as u64)
}

pub fn convert_to_fps(knob_level: u32) -> u64 {
    let fps = (knob_level as u64 * 10) + 10;
    fps.clamp(10, 160)
}
