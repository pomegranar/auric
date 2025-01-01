/// Normalize a vector of floats to a specified range.
///
/// # Arguments
/// * `data` - The input vector to normalize.
/// * `min` - The minimum value of the desired range.
/// * `max` - The maximum value of the desired range.
///
/// # Returns
/// A vector of normalized values.
pub fn normalize(data: &[f32], min: f32, max: f32) -> Vec<f32> {
    let data_min = data.iter().cloned().fold(f32::INFINITY, f32::min);
    let data_max = data.iter().cloned().fold(f32::NEG_INFINITY, f32::max);

    data.iter()
        .map(|&val| min + (val - data_min) / (data_max - data_min) * (max - min))
        .collect()
}

/// Map a frequency index to a screen position.
///
/// # Arguments
/// * `index` - The frequency bin index.
/// * `total_bins` - The total number of bins.
/// * `screen_width` - The width of the screen.
///
/// # Returns
/// The x-coordinate for the given frequency bin.
pub fn frequency_to_screen_position(index: usize, total_bins: usize, screen_width: f32) -> f32 {
    let bin_width = screen_width / total_bins as f32;
    index as f32 * bin_width - screen_width / 2.0
}

/// Smooth a vector of floats using a moving average filter.
///
/// # Arguments
/// * `data` - The input vector to smooth.
/// * `window_size` - The size of the moving average window.
///
/// # Returns
/// A smoothed vector of floats.
pub fn smooth(data: &[f32], window_size: usize) -> Vec<f32> {
    let mut smoothed = vec![];
    for i in 0..data.len() {
        let start = if i >= window_size { i - window_size + 1 } else { 0 };
        let slice = &data[start..=i];
        let average = slice.iter().sum::<f32>() / slice.len() as f32;
        smoothed.push(average);
    }
    smoothed
}

/// Convert linear amplitude values to a logarithmic scale (dB).
///
/// # Arguments
/// * `amplitudes` - A vector of amplitude values.
///
/// # Returns
/// A vector of decibel values.
pub fn to_decibels(amplitudes: &[f32]) -> Vec<f32> {
    amplitudes
        .iter()
        .map(|&amp| 20.0 * (amp.max(1e-10).log10())) // Avoid log(0) by using a minimum value
        .collect()
}
