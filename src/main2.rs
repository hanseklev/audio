use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    BufferSize, SampleRate, Sample,
};

struct OscSin {
    sample_rate: f32,
    buffer: Vec<f32>,
    buffer_pos: usize,
    x: f32,
}

impl OscSin {
    pub fn get_next_sample(&mut self, freq: f32) -> f32 {
        let fs = freq * self.sample_rate_inv();
        let value = self.x.cos();
        self.x += fs;
        if self.x >= 1.0f32 {
            self.x -= 1.0f32;
        }
        return OscSin::get_next_blep(self) - value;
    }

    fn get_next_blep(&mut self) -> f32 {
        self.buffer[self.buffer_pos] = 0.0f32;
        self.buffer_pos += 1;

        if self.buffer_pos >= 128 {
            self.buffer_pos -= 128;
        }
        return self.buffer[self.buffer_pos];
    }

    fn sample_rate_inv(&self) -> f32 {
        1f32 / self.sample_rate
    }
}

fn main() {
    println!("Hello, world!");

    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("No output device available");

    /*    let mut configs = device.supported_output_configs().unwrap();
    let config = configs.next().unwrap(); */

    let config = cpal::StreamConfig {
        channels: 2,
        buffer_size: BufferSize::Fixed(128),
        sample_rate: SampleRate(41_000),
    };

    let mut osci = OscSin {
        sample_rate: 41_000f32,
        buffer: vec![0.0; 128],
        buffer_pos: 0,
        x: 0.0f32,
    };

    let stream = device
        .build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                for sample in data.iter_mut() {
                    *sample = osci.get_next_sample(440f32);
                    //sample = Sample::from(&0.4);
                    println!("{}", sample);
                }
            },
            move |_err| {
                println!("{}", _err);
            },
        )
        .unwrap();

    //let stream2 =  device.build_output_stream(&config, write_things::<f32>, err_fn);

    stream.play().unwrap();

/*     fn write_things<T: Sample>(data: &mut [T], _: &cpal::OutputCallbackInfo) {
        for sample in data.iter_mut() {
            *sample = Sample::from(&0.4);
        }
    } */
    /*
       event_loop.run(move |_stream_id, mut stream_data| match stream_data {
           StreamData::Output {
               buffer: UnknownTypeOutputBuffer::F32(mut buffer),
           } => {
               println!("{}", buffer.len());
               for elem in buffer.iter_mut() {
                   /* *elem = osc_sin(&tick);
                   tick += 1;
                   println!("{}", elem); */
               }
           }
           _ => (),
       });
    fn osc_sin(tick: &i32) -> f32 {
        f32::sin(220.0 * (2.0 * PI) * *tick as f32 / 41000.0)
    }    */

}
