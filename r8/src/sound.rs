use anyhow::Result;
use cpal::{
    default_host,
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, Sample, SampleFormat, Stream, StreamConfig,
};

use crate::error::InternalError;

pub struct Beep {
    stream: Stream,
}

impl Beep {
    pub fn new() -> Result<Self> {
        let device = default_host()
            .default_output_device()
            .ok_or(InternalError::InvalidAudioOutput)?;

        let config = device.default_output_config()?;

        let stream = match config.sample_format() {
            SampleFormat::I16 => Self::build_stream::<i16>(&device, &config.into()),
            SampleFormat::U16 => Self::build_stream::<u16>(&device, &config.into()),
            SampleFormat::F32 => Self::build_stream::<f32>(&device, &config.into()),
        }?;

        Ok(Self { stream })
    }

    pub fn play(&self) {
        self.stream.play().ok();
    }

    pub fn pause(&self) {
        self.stream.pause().ok();
    }

    fn build_stream<T: Sample>(device: &Device, config: &StreamConfig) -> Result<Stream> {
        let sample_rate = config.sample_rate.0 as f32;
        let channels = config.channels as usize;

        let mut sample_clock = 0f32;
        let mut next_value = move || {
            sample_clock = (sample_clock + 1.0) % sample_rate;
            (sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin()
        };

        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let stream = device.build_output_stream(
            config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| Self::write_data(data, channels, &mut next_value),
            err_fn,
        )?;

        Ok(stream)
    }

    fn write_data<T: Sample>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32) {
        for frame in output.chunks_mut(channels) {
            let value: T = cpal::Sample::from::<f32>(&next_sample());
            for sample in frame.iter_mut() {
                *sample = value;
            }
        }
    }
}
