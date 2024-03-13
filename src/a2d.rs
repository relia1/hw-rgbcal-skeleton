/// SAADC - Successive Approximation Analog-toDigital Converter
/// Using SAADC configured with analog inputs, take measurements
/// and convert those to a digital representation
use crate::*;

/// A type alias for an Saadc object with 2 channels
pub type Adc = saadc::Saadc<'static, 2>;

/// This datatype calibrates the ADC and provides 2 async functions
/// 1. measure_knob(): This gets a sample back from our potentometer
/// 2. measure_ldr():  This gets a sample back from our photoresistor
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

    /// Return a sample from the potentometer and scale it between 0 and 15

    pub async fn measure_knob(&mut self) -> u32 {
        let mut buf = [0, 0];
        self.0.sample(&mut buf).await;
        let raw = buf[0].clamp(0, 0x7fff) as u16;
        let scaled = raw as f32 / 10_000.0;
        let result = ((LEVELS + 2) as f32 * scaled - 2.0)
            .clamp(0.0, (LEVELS - 1) as f32)
            .floor();
        result as u32
    }

    /// Return a sample from the ldr
    /// TODO add scaling
    pub async fn measure_ldr(&mut self) -> u32 {
        let mut buf = [0, 0];
        self.0.sample(&mut buf).await;
        buf[1] as u32
    }
}
