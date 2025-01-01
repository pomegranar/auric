use auric::audio::{capture, analysis};
use auric::render::pipeline;
use std::sync::mpsc::{self, Sender};
use std::thread;

fn main() {
    println!("🎨 Starting auric: Advanced Visualizer 🎨");

    let (raw_tx, raw_rx) = mpsc::channel();
    let (fft_tx, fft_rx) = mpsc::channel();

    // Audio capture thread
    thread::spawn(move || {
        if let Err(err) = capture::start_audio_capture(raw_tx) {
            eprintln!("Error during audio capture: {}", err);
        }
    });

    // FFT analysis thread
    thread::spawn(move || {
        for samples in raw_rx {
            analysis::analyze_audio(&samples, &fft_tx);
        }
    });

    // Start the rendering loop with processed FFT data
    pollster::block_on(pipeline::run(fft_rx)).expect("Rendering pipeline failed");
}
