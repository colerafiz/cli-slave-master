#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! tts = "0.26"
//! rand = "0.8"
//! ```

use rand::seq::SliceRandom;
use std::env;
use tts::Tts;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize TTS engine
    let mut tts = Tts::default()?;
    
    // Configure settings
    tts.set_rate(1.2)?;    // Slightly faster than normal
    tts.set_volume(0.8)?;  // 80% volume
    
    println!("🎙️  Rust TTS");
    println!("===============");
    
    // Get text from command line arguments or use default
    let args: Vec<String> = env::args().collect();
    let text = if args.len() > 1 {
        args[1..].join(" ")
    } else {
        // Default completion messages
        let messages = vec![
            "Work complete!",
            "All done!",
            "Task finished!",
            "Job complete!",
            "Ready for next task!",
        ];
        messages.choose(&mut rand::thread_rng())
            .unwrap_or(&"Done!")
            .to_string()
    };
    
    println!("🎯 Text: {}", text);
    println!("🔊 Speaking...");
    
    // Speak the text
    tts.speak(&text, true)?;
    
    println!("✅ Playback complete!");
    
    Ok(())
}