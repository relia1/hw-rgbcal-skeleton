use crate::*;

pub type Adc = saadc::Saadc<'static, 2>;

pub struct A2d(Adc);
impl A2d {
    pub async fn new(adc: Adc) -> Self {
        adc.calibrate().await;
        Self(adc)
    }

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

    pub async fn measure_ldr(&mut self) -> u32 {
        let mut buf = [0, 0];
        self.0.sample(&mut buf).await;
        buf[1] as u32
    }
}
/*
pub struct Knob<'a> {
    saadc: &'a mut Adc2,
}

impl<'a> Knob<'a> {
    pub async fn new(adc: &'a mut Adc2) -> Self {
        adc.calibrate().await;
        Self { saadc: adc }
    }

    pub async fn measure(&mut self) -> u32 {
        let mut buf = [0, 0];
        self.saadc.sample(&mut buf).await;
        let raw = buf[0].clamp(0, 0x7fff) as u16;
        let scaled = raw as f32 / 10_000.0;
        let result = ((LEVELS + 2) as f32 * scaled - 2.0)
            .clamp(0.0, (LEVELS - 1) as f32)
            .floor();
        result as u32
    }
}

/// Light dependent resistor
pub struct Ldr<'a> {
    saadc: &'a mut Adc2,
}

impl<'a> Ldr<'a> {
    pub async fn new(adc: &'a mut Adc2) -> Self {
        adc.calibrate().await;
        Self { saadc: adc }
    }

    pub async fn measure(&mut self) -> u32 {
        let mut buf = [0, 0];
        self.0.sample(&mut buf).await;
        let raw = buf[0].clamp(0, 0x7fff) as u16;
        let scaled = raw as f32 / 10_000.0;
        let result = ((LEVELS + 2) as f32 * scaled - 2.0)
            .clamp(0.0, (LEVELS - 1) as f32)
            .floor();
        result as u32
    }
}
*/
