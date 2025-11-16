use crossterm::{
    event::{Event, KeyCode, KeyEvent, poll, read},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use kira::sound::static_sound::StaticSoundData;
use kira::{
    AudioManager, AudioManagerSettings, DefaultBackend, effect::filter::FilterBuilder,
    track::TrackBuilder,
};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    println!("Press 'Space' to play, 'q' (quit)...\r\n");

    let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())?;
    let mut track = manager.add_sub_track({
        let mut builder = TrackBuilder::new();
        builder.add_effect(FilterBuilder::new().cutoff(1000.0));
        builder
    })?;

    let sound_data = StaticSoundData::from_file("hello.ogg")?;

    loop {
        // Poll for keyboard events for 10ms
        if poll(Duration::from_millis(10))? {
            // If there's an event, read it
            if let Event::Key(KeyEvent { code, .. }) = read()? {
                match code {
                    KeyCode::Char(' ') => {
                        track.play(sound_data.clone())?;
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
