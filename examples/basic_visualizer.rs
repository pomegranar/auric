use auric::audio::capture;
use auric::render::pipeline;
use std::sync::mpsc;
use std::thread;

fn main() {
    println!("🎵 Starting auric: Basic Visualizer 🎵");

    // Set up the communication channel
    let (tx, rx) = mpsc::channel();

    // Start audio capture in a separate thread
    thread::spawn(move || {
        if let Err(err) = capture::start_audio_capture(tx) {
            eprintln!("Error during audio capture: {}", err);
        }
    });

    // Start the rendering loop
    pollster::block_on(pipeline::run(rx)).expect("Rendering pipeline failed");
}
