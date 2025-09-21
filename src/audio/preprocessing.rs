pub struct AudioPreprocessor;

impl AudioPreprocessor {
    pub fn normalize(samples: &[f32]) -> Vec<f32> {
        let max_val = samples.iter().map(|x| x.abs()).fold(0.0, f32::max);
        if max_val > 0.0 {
            samples.iter().map(|x| x / max_val).collect()
        } else {
            samples.to_vec()
        }
    }

    pub fn apply_window(samples: &[f32], window_size: usize) -> Vec<Vec<f32>> {
        samples
            .chunks(window_size)
            .map(|chunk| chunk.to_vec())
            .collect()
    }

    pub fn remove_dc_offset(samples: &[f32]) -> Vec<f32> {
        let mean = samples.iter().sum::<f32>() / samples.len() as f32;
        samples.iter().map(|x| x - mean).collect()
    }
}