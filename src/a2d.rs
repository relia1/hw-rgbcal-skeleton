/// SAADC - Successive Approximation Analog-toDigital Converter
/// Using SAADC configured with analog inputs, take measurements
/// and convert those to a digital representation
use crate::*;

/// A type alias for a Saadc object with 2 channels
pub type Adc = saadc::Saadc<'static, 2>;

/// This datatype calibrates the ADC and provides 2 async functions
/// 1. `measure_knob()`: This gets a sample back from our potentiometer
/// 2. `measure_ldr()`:  This gets a sample back from our photoresistor
pub struct A2d(Adc);
impl A2d {
    /// Create a new 'A2d' and calibrates the ACD
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let mut board = embassy_nrf::init(Default::default());
    /// let mut saadc_config = saadc::Config::default();
    /// saadc_config.resolution = saadc::Resolution::_14BIT;
    /// let saadc = saadc::Saadc::new(
    ///    board.SAADC,
    ///    Irqs,
    ///    saadc_config,
    ///    [
    ///        saadc::ChannelConfig::single_ended(&mut board.P0_04), // P2
    ///        saadc::ChannelConfig::single_ended(&mut board.P0_28), // P4
    ///    ],
    /// );
    ///
    /// let a2d = A2d::new(saadc).await;
    ///
    /// a2d.measure_knob();
    /// ```
    pub async fn new(adc: Adc) -> Self {
        adc.calibrate().await;
        Self(adc)
    }

    /// Return a sample from the potentiometer and scale it between 0 and 15
    /// Digital Output Result = (V(P) – V(N)) * (GAIN/REFERENCE) * (2^(RESOLUTION - m))
    /// V(P) is the voltage at input P
    /// V(N) is the voltage at input N
    /// GAIN is the selected gain
    /// REFERENCE is the selected reference voltage
    /// RESOLUTION is output resolution in bits
    /// m is 0 for single ended channels and 1 for differential channels
    pub async fn measure_knob(&mut self) -> u32 {
        let mut buf = [0, 0];
        self.0.sample(&mut buf).await;
        let raw = buf[0].clamp(0, 0x7fff) as u16;
        // Ratio for scaling to 0 to 15 inclusive of 15
        let ratio = (raw as f32) / 15_000_f32;
        (ratio * 16_f32) as u32
    }

    /// Return a sample from the ldr
    /// TODO add scaling
    pub async fn measure_ldr(&mut self) -> u32 {
        let mut buf = [0, 0];
        self.0.sample(&mut buf).await;
        buf[1] as u32
    }
}
