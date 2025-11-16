use kira::sound::streaming::StreamingSoundData;
use kira::{AudioManager, AudioManagerSettings, DefaultBackend, Easing, StartTime, Tween};
use std::time::Duration;

use crossterm::{
    event::{Event, KeyCode, KeyEvent, poll, read},
    terminal::{disable_raw_mode, enable_raw_mode},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    println!("Press 'u' (up), 'd' (down), or 'q' (quit)...\r\n");

    // 2. Set up Kira
    let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())?;

    let sound_data = StreamingSoundData::from_file("hello.ogg")?;
    let mut sound = manager.play(sound_data)?;

    let mut current_pitch = 1.0;

    loop {
        // Poll for keyboard events for 10ms
        if poll(Duration::from_millis(10))? {
            // If there's an event, read it
            if let Event::Key(KeyEvent { code, .. }) = read()? {
                let mut pitch_changed = true;

                match code {
                    KeyCode::Char('u') => {
                        current_pitch *= 2.0; // Double frequency = 1 octave up
                        println!("Pitch Up! (Rate: {})\r", current_pitch);
                    }
                    KeyCode::Char('d') => {
                        current_pitch /= 2.0; // Halve frequency = 1 octave down
                        println!("Pitch Down! (Rate: {})\r", current_pitch);
                    }
                    KeyCode::Char('q') => {
                        println!("Quitting...\r");
                        break; // Exit the loop
                    }
                    _ => {
                        pitch_changed = false; // Don't apply tween if no relevant key
                    }
                }

                if pitch_changed {
                    // Apply the new pitch with a very short, smooth tween
                    let tween = Tween {
                        start_time: StartTime::Immediate,
                        duration: Duration::from_millis(100), // 100ms fade
                        easing: Easing::Linear,
                    };
                    sound.set_playback_rate(current_pitch, tween);
                }
            }
        }
    }

    disable_raw_mode()?;
    Ok(())
}

