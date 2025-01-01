use wgpu::util::DeviceExt;

pub struct VisualData {
    amplitudes: Vec<f32>,
}

impl VisualData {
    pub fn new() -> Self {
        Self {
            amplitudes: vec![],
        }
    }

    pub fn update(&mut self, amplitudes: Vec<f32>) {
        self.amplitudes = amplitudes;
    }

    pub fn create_vertex_data(&self) -> Vec<f32> {
        self.amplitudes
            .iter()
            .enumerate()
            .flat_map(|(i, amp)| {
                let x = i as f32 / self.amplitudes.len() as f32 * 2.0 - 1.0; // Normalize to screen space
                let y = *amp; // Use amplitude for vertical position
                vec![x, y, 0.0] // Vertex format: [x, y, z]
            })
            .collect()
    }
}
