pub mod sine;
pub mod square;

use std::marker::PhantomData;

use crate::audio_module::{AudioModule, AudioProcessor};

pub trait Oscillator {
    fn new(freq: f64, volume: f64, sample_rate: f64) -> Self;
    fn tick(&mut self) -> (f32, f32);
}

pub struct OscProcessor<T: Oscillator> {
    wave: T,
}

impl<T> OscProcessor<T>
where
    T: Oscillator,
{
    fn new(sample_rate: f64) -> Self {
        Self {
            wave: self::Oscillator::new(440., 0.1, sample_rate),
        }
    }
}

impl<T: 'static> AudioProcessor for OscProcessor<T>
where
    T: Oscillator + Send + Sync,
{
    fn process_stereo_output(&mut self, buffer: usize, output: &mut [f32]) {
        println!("processing sine");
        for i in 0..buffer {
            let (sample_l, sample_r) = self.wave.tick();
            println!("amp: {}", sample_l);
            output[2 * i] = sample_l;
            output[2 * i + 1] = sample_r;
        }
    }
}

pub struct OscModule<T>
where
    T: Oscillator,
{
    phantom: PhantomData<T>,
}

pub struct SquareModule {}

impl<T: 'static> AudioModule for OscModule<T>
where
    T: Oscillator + Send + Sync,
{
    type Processor = OscProcessor<T>;

    fn create_processor(sample_rate: f64) -> Self::Processor {
        OscProcessor::new(sample_rate)
    }
}

/* impl AudioModule for OscModule {
    type Processor = OscProcessor;

    fn create_processor(sample_rate: f64) -> Self::Processor {
        OscProcessor::new(sample_rate)
    }
} */

/* pub enum Oscillators {
    SquareWave(SquareWave),
    SineWave(SineWave),
}

pub struct OscProcessor {
    wave: SineWave,
}

impl OscProcessor {
    fn new(sample_rate: f64) -> Self {
        Self {
            wave: SineWave::new(440., 0.5, sample_rate),
        }
    }
} */
