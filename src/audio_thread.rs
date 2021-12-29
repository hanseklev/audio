use coreaudio::audio_unit::{
    audio_format::LinearPcmFlags,
    macos_helpers::{
        audio_unit_from_device_id, find_matching_physical_format, get_default_device_id,
    },
    macos_helpers::{
        set_device_physical_stream_format, AliveListener, RateListener,
    },
    {SampleFormat, Scope},
    render_callback::{self, data},
    StreamFormat,
};
use coreaudio::sys::kAudioUnitProperty_StreamFormat;
use crate::audio_module::{AudioModule, AudioProcessor};

const INTERLEAVED: bool = true;
const SAMPLE_FORMAT: SampleFormat = SampleFormat::F32;



pub fn start_audio<Module: AudioModule>(sample_rate: f64) -> Result<(), coreaudio::Error> {
    let mut processor = Module::create_processor(sample_rate);  

    let audio_unit_id = get_default_device_id(false).unwrap();
    let mut audio_unit = audio_unit_from_device_id(audio_unit_id, false)?;

    let mut format_flag = match SAMPLE_FORMAT {
        SampleFormat::F32 => LinearPcmFlags::IS_FLOAT | LinearPcmFlags::IS_PACKED,
        _ => unimplemented!("You must use f32 format"),
    };

    if !INTERLEAVED {
        format_flag = format_flag | LinearPcmFlags::IS_NON_INTERLEAVED;
    }

    let stream_format = StreamFormat {
        sample_rate,
        sample_format: SAMPLE_FORMAT,
        flags: format_flag,
        channels: 2,
    };

    //println!("stream format={:#?}", &stream_format);
    //println!("asbd={:#?}", &stream_format.to_asbd());

    let hw_stream_format = StreamFormat {
        sample_rate,
        sample_format: SampleFormat::I24,
        flags: LinearPcmFlags::empty(),
        channels: 2,
    };
    let hw_asbd = find_matching_physical_format(audio_unit_id, hw_stream_format)
        .ok_or(coreaudio::Error::UnsupportedStreamFormat)?;
    set_device_physical_stream_format(audio_unit_id, hw_asbd)?;

    let id = kAudioUnitProperty_StreamFormat;
    let asbd = stream_format.to_asbd();
    audio_unit.set_property(
        id,
        Scope::Input,
        coreaudio::audio_unit::Element::Output,
        Some(&asbd),
    )?;

    assert!(SampleFormat::F32 == stream_format.sample_format);

    let mut rate_listener = RateListener::new(audio_unit_id, None);
    rate_listener.register()?;
    let mut alive_listener = AliveListener::new(audio_unit_id);
    alive_listener.register()?;

    if INTERLEAVED {
        println!("Register interleaved callback");
        type Args = render_callback::Args<data::Interleaved<f32>>;
        audio_unit.set_render_callback(move |args| {
            let Args {
                num_frames, mut data, ..
            } = args;
            //println!("frames: {}", num_frames);
            processor.process_stereo_output(num_frames, &mut data.buffer);

           
            Ok(())
        })?
    };

    audio_unit.start()?;

    for _ in 0..100 {
        std::thread::sleep(std::time::Duration::from_millis(100));
        // print all sample change events
        println!("rate events: {:?}", rate_listener.copy_values());
        println!("alive state: {}", alive_listener.is_alive());
    }
    Ok(())
}
