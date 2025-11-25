use crossterm::{
    event::{Event, KeyCode, KeyEvent, poll, read},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use kira::Tween;
use kira::sound::static_sound::{StaticSoundData, StaticSoundHandle};
use kira::{
    AudioManager, AudioManagerSettings, DefaultBackend, effect::filter::FilterBuilder,
    track::TrackBuilder,
};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    println!("Press 'Space' to play, 's' to stop, 'q' (quit)...\r\n");

    let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())?;
    let mut track = manager.add_sub_track({
        let mut builder = TrackBuilder::new();
        builder.add_effect(FilterBuilder::new().cutoff(1000.0));
        builder
    })?;
    let sound_data = StaticSoundData::from_file("hello.ogg")?;

    // ‼️ Change the handle's type from Option<SoundHandle> to Option<StaticSoundHandle>
    let mut handle: Option<StaticSoundHandle> = None;

    loop {
        // Poll for keyboard events for 10ms
        if poll(Duration::from_millis(10))? {
            // If there's an event, read it
            if let Event::Key(KeyEvent { code, .. }) = read()? {
                match code {
                    KeyCode::Char(' ') => {
                        if let Some(mut h) = handle.take() {
                            h.stop(Tween::default());
                        }
                        handle = Some(track.play(sound_data.clone())?);
                    }
                    KeyCode::Char('s') => {
                        if let Some(mut h) = handle.take() {
                            println!("Stopping sound...\r");
                            h.stop(Tween::default());
                        }
                    }
                    KeyCode::Char('q') => {
                        println!("Quitting...\r");
                        break; // Exit the loop
                    }
                    _ => {}
                }
            }
        }
    }
    disable_raw_mode()?;
    Ok(())
}
