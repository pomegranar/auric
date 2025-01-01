mod audio;  // Link to the audio module
mod render; // Link to the rendering module
mod utils;  // Link to utility functions

use std::sync::mpsc;
use std::thread;

fn main() {
    // Print a welcome message to the console
    println!("🌟 Welcome to auric – let the visuals sync with the sound! 🌟");

    // Set up a communication channel between audio processing and rendering
    let (tx, rx) = mpsc::channel();

    // Spawn a thread for audio capture and analysis
    thread::spawn(move || {
        if let Err(err) = audio::capture::start_audio_capture(tx) {
            eprintln!("Error capturing audio: {}", err);
        }
    });

    // Run the visualizer rendering loop
    pollster::block_on(render::pipeline::run(rx));
}
