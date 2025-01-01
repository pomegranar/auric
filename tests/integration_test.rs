use auric::audio::{capture, analysis};
use std::sync::mpsc;
use std::thread;

#[test]
fn test_audio_pipeline() {
    let (tx, rx) = mpsc::channel();

    // Simulate audio capture in a separate thread
    thread::spawn(move || {
        let fake_data = vec![0.1, 0.2, 0.3, 0.4];
        tx.send(fake_data).expect("Failed to send data");
    });

    // Receive the captured data
    let samples = rx.recv().expect("Failed to receive data");
    assert_eq!(samples.len(), 4);
}

#[test]
fn test_fft_analysis() {
    let (tx, rx) = mpsc::channel();
    let fake_data = vec![0.0, 1.0, 0.0, -1.0];

    analysis::analyze_audio(&fake_data, &tx);
    let result = rx.recv().expect("Failed to receive FFT data");

    // FFT should return the same length as input for this basic example
    assert_eq!(result.len(), 4);
}
