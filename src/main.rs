use std::{f32::consts::PI, time::Duration};

pub mod audio_playback;
pub mod error;
fn main() {
    println!("Hello, world!");
    println!("{}", audionimbus::STEAMAUDIO_VERSION);

    let mut audio_play = audio_playback::AudioPlayback::new().unwrap();

    let sample_rate = audio_play.sample_rate as f32;
    let freq = 440.0;
    let dur = 5.0;
    let total = (sample_rate * dur) as usize;

    for i in 0..total {
        let itme = i as f32 / sample_rate;
        let sample = (2.0 * PI * freq * itme).sin() * 0.3;

        audio_play.add_samples(&[sample, sample]);
    }

    std::thread::sleep(Duration::from_secs(6));
    println!("Good!");
}
