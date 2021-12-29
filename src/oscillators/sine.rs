use std::f64::consts::PI;

use super::Oscillator;

pub struct SineWave {
    time: f64,
    freq: f64,
    volume: f64,
    sample_rate: f64,
}

/* impl SineWave {
    pub fn new(freq: f64, volume: f64, sample_rate: f64) -> Self {}
} */
impl Oscillator for SineWave {
    fn tick(&mut self) -> (f32, f32) {
        self.time += 1. / self.sample_rate;
        let output = ((self.freq * self.time * PI * 2.).sin() * self.volume) as f32;
        (output, output)
    }

  fn new(freq: f64, volume: f64, sample_rate: f64) -> Self {
        SineWave {
            time: 0. /sample_rate,
            freq,
            volume,
            sample_rate,
        }

  }
}
