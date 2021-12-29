use std::f64::consts::PI;

use crate::oscillators::Oscillator;
pub struct SquareWave {
    time: f64,
    freq: f64,
    volume: f64,
    sample_rate: f64,
}
/* 
impl SquareWave {
    pub fn new(freq: f64, volume: f64, sample_rate: f64) -> Self {}
} */

impl Oscillator for SquareWave {
    fn tick(&mut self) -> (f32, f32) {
        self.time += 1. / self.sample_rate;
        let amp = self.volume.clone() as f32;
        let output = ((self.freq * self.time * PI * 2.).sin() * self.volume) as f32;
        if output > 0. {
            (amp, amp)
        } else {
            (-amp, -amp)
        }
    }

    fn new(freq: f64, volume: f64, sample_rate: f64) -> Self {
        SquareWave {
            time: 0. / sample_rate,
            freq,
            volume,
            sample_rate,
        }
    }
}
