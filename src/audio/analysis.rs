use rustfft::{FftPlanner, num_complex::Complex};
use std::sync::mpsc::Sender;

pub fn analyze_audio(samples: &[f32], tx: &Sender<Vec<f32>>) {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(samples.len());
    let mut buffer: Vec<Complex<f32>> = samples.iter().map(|&x| Complex::new(x, 0.0)).collect();
    fft.process(&mut buffer);

    // Extract amplitudes and send them to the rendering thread
    let amplitudes: Vec<f32> = buffer.iter().map(|c| c.norm()).collect();
    tx.send(amplitudes).ok();
}
