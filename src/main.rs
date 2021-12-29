mod audio_module;
use oscillators::{OscModule, sine::SineWave, square::SquareWave};

use crate::audio_module::AudioModule;
mod audio_thread;
mod oscillators;

const SAMPLE_RATE: f64 = 44100.;

fn main() -> Result<(), coreaudio::Error> {
    run_mains::<OscModule<SquareWave>>();
    Ok(())
}

fn run_mains<Module: AudioModule>() {
    let _audio_stream = audio_thread::start_audio::<Module>(SAMPLE_RATE);
}
