pub trait AudioProcessor: Send + Sync + 'static {
    //fn process_stereo(&mut self, input: &[f32], output: &mut [f32]);
    fn process_stereo_output(&mut self, buffer:  usize, output: &mut [f32]);

}

pub trait AudioModule {
    type Processor: AudioProcessor;

    fn create_processor(sample_rate: f64) -> Self::Processor;
}
