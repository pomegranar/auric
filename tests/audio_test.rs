use auric::audio::capture;
use std::sync::mpsc;

#[test]
fn test_audio_capture_initialization() {
    let (tx, _rx) = mpsc::channel();

    // Test if the audio capture initializes without error
    let result = capture::start_audio_capture(tx);
    assert!(result.is_ok(), "Audio capture failed to initialize");
}
