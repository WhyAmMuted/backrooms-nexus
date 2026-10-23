use std::time::Duration;

use cpal::{
    StreamConfig,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};
use rtrb::Producer;
const BUFFER_SIZE: usize = (1 << 16);

pub struct AudioPlayback {
    prod: Producer<f32>,
    stream: cpal::Stream,
    pub sample_rate: u32,
}

impl AudioPlayback {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or("no output device available")?;
        let default_config = device.default_output_config()?;
        let sample_rate = default_config.sample_rate();
        let mut stream_config: StreamConfig = default_config.into();
        stream_config.channels = 2;

        let (prod, mut cons) = rtrb::RingBuffer::<f32>::new(BUFFER_SIZE);

        let err_fn = |err| eprintln!("Error while audio playback: {}", err);
        let stream = device.build_output_stream(
            stream_config,
            move |data: &mut [f32], _| {
                let (_popped, remainder) = cons.pop_partial_slice(data);
                remainder.fill(0.0);
            },
            err_fn,
            None,
        )?;

        stream.play()?;
        Ok(Self {
            stream: stream,
            sample_rate: sample_rate,
            prod: prod,
        })
    }

    pub fn add_samples(&mut self, samples: &[f32]) {
        let mut chunk = samples;

        while !chunk.is_empty() {
            let (_pushed, remainder) = self.prod.push_partial_slice(chunk);
            chunk = remainder;

            if !chunk.is_empty() {
                std::thread::sleep(Duration::from_millis(1));
            }
        }
    }
}
