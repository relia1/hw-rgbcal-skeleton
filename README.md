# rgbcal: RGB LED calibration tool
Bart Massey 2024
Updates by Robert Elia

This tool is designed to find out a decent frame rate and
maximum RGB component values to produce a white-looking RGB
of reasonable brightness.

See below for UI.

## Build and Run

Run with `cargo embed --release`. You'll need `cargo embed`, as
`cargo run` / `probe-rs run` does not reliably maintain a
connection for printing. See
https://github.com/probe-rs/probe-rs/issues/1235 for the
details.

Additionally, this can also be run without `--release` to get extra debug information.

## Wiring

Connect the RGB LED to the MB2 as follows:

* Red to P9 (GPIO1)
* Green to P8 (GPIO2)
* Blue to P16 (GPIO3)
* Gnd to Gnd

Connect the potentiometer (knob) to the MB2 as follows:

* Pin 1 to Gnd
* Pin 2 to P2
* Pin 3 to +3.3V

## UI

The knob controls the individual settings: frame rate and
color levels. Which parameter the knob controls should be
determined by which buttons are held.

* No buttons held: Change the frame rate in steps of 10
  frames per second from 10..160.
* A button held: Change the blue level from off to on over
  16 steps.
* B button held: Change the green level from off to on over
  16 steps.
* A+B buttons held: Change the red level from off to on over
  16 steps.

The "frame rate" (also known as the "refresh rate") is the
time to scan out all three colors. At 30 frames per second, 
every 1/30th of a second the LED should scan out all three 
colors. If the frame rate is too low, the LED will appear to
"blink". If it is too high, it will eat CPU for no reason.

I think the frame rate is probably set higher than it needs
to be right now: it can be tuned lower.

## Measurements
| Parameter   | Value |
|-------------|-------|
| Red Level   | 15    |
| Green Level | 15    |
| Blue Level  | 12    |
| Frame Rate  | 50    |

## What I did
- [x] Commented existing shared code
- [x] Wired up the RGB LED, potentiometer, and buttons to the MB2
- [x] Implemented the UI
  - [x] Configured button a for the blue LED (already implemented)
  - [x] Configured button b for the green LED
  - [x] Configured button a+b for the red LED
  - [x] Configured the knob to change the frame rate when no buttons pushed
  - [x] Configured the knob to change the LEDs when the corresponding button combo is pushed
- [x] Updated existing code to share the frame rate between RGB and UI structs

## How it went
Existing code was pretty straightforward to understand and provide the requested  
extensions. Since I was experimenting with adding a photoresistor to the MB2, I  
ended up having to change how the program is getting the peripherals from the board.  
So instead of using the microbit_bsp::Microbit, I needed to directly use embassy_nrf  
directly. This was due to not all the analog pins being available in the microbit_bsp. 
I have some ongoing experiments with a photoresistor for trying to determine the  
brightness of the LEDs. The thinking is if you can determine the brightness of the  
LEDs, you can adjust the frame rate and RGB levels to get specific colors based on
RGB values.

## Any additional observations of interest
- Ongoing experiments with a photoresistor to determine the brightness of the LEDs
- Was not able to get a smooth fps under 50
- Added some macros, they aren't really useful in this case, but I wanted to experiment with them
- Added cargo build and cargo clippy to github actions on push to main
