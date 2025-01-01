use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};
use std::sync::mpsc::Sender;

pub fn start_audio_capture(tx: Sender<Vec<f32>>) -> Result<(), Box<dyn std::error::Error>> {
    let host = cpal::default_host();
    let device = host.default_input_device().ok_or("No input device available")?;
    let format = device.default_input_format()?;
    let event_loop = host.event_loop();
    let stream_id = event_loop.build_input_stream(&device, &format)?;

    event_loop.play_stream(stream_id)?;

    event_loop.run(move |stream_id, stream_result| {
        if let Ok(cpal::StreamData::Input { buffer }) = stream_result {
            match buffer {
                cpal::UnknownTypeInputBuffer::F32(buffer) => {
                    tx.send(buffer.to_vec()).ok(); // Send audio data to the analysis thread
                }
                _ => eprintln!("Unsupported buffer type"),
            }
        }
    });

    Ok(())
}
