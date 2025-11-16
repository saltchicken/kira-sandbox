use kira::sound::streaming::StreamingSoundData;
use kira::{AudioManager, AudioManagerSettings, Easing, StartTime, Tween};
use std::{thread, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut audio_manager: AudioManager = AudioManager::new(AudioManagerSettings::default())?;

    let sound_data = StreamingSoundData::from_file("hello.ogg")?;

    let mut handle = audio_manager.play(sound_data)?;

    // This is the new, correct way to build a linear tween
    let pitch_tween = Tween {
        start_time: StartTime::Immediate,
        duration: Duration::from_secs(3),
        easing: Easing::Linear,
    };

    handle.set_playback_rate(0.5, pitch_tween);

    println!("Playing 'hello.ogg' with pitch-down effect...");
    println!("(Program will exit after 5 seconds)");
    thread::sleep(Duration::from_secs(5));

    Ok(())
}
