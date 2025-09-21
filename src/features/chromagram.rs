use rustfft::{FftPlanner, num_complex::Complex};
use std::f32::consts::PI;

pub struct ChromagramExtractor {
    sample_rate: u32,
    fft_size: usize,
}

impl ChromagramExtractor {
    pub fn new(sample_rate: u32, fft_size: usize) -> Self {
        Self {
            sample_rate,
            fft_size,
        }
    }

    pub fn extract_chromagram(&self, samples: &[f32]) -> Vec<[f32; 12]> {
        let hop_size = self.fft_size / 4;
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(self.fft_size);

        samples
            .chunks(self.fft_size)
            .step_by(hop_size)
            .map(|window| self.compute_chroma_vector(window, &fft))
            .collect()
    }

    fn compute_chroma_vector(&self, window: &[f32], fft: &std::sync::Arc<dyn rustfft::Fft<f32>>) -> [f32; 12] {
        // Apply window function (Hanning)
        let windowed: Vec<Complex<f32>> = window
            .iter()
            .enumerate()
            .map(|(i, &sample)| {
                let window_val = 0.5 * (1.0 - (2.0 * PI * i as f32 / window.len() as f32).cos());
                Complex::new(sample * window_val, 0.0)
            })
            .collect();

        let mut spectrum = windowed;
        fft.process(&mut spectrum);

        // Convert to magnitude spectrum
        let magnitude_spectrum: Vec<f32> = spectrum
            .iter()
            .take(self.fft_size / 2)
            .map(|c| c.norm())
            .collect();

        // Map frequencies to chroma bins
        let mut chroma = [0.0f32; 12];

        for (bin, &magnitude) in magnitude_spectrum.iter().enumerate() {
            let frequency = bin as f32 * self.sample_rate as f32 / self.fft_size as f32;

            if frequency > 80.0 && frequency < 2000.0 {
                let chroma_index = self.frequency_to_chroma(frequency);
                chroma[chroma_index] += magnitude;
            }
        }

        // Normalize
        let sum: f32 = chroma.iter().sum();
        if sum > 0.0 {
            for value in &mut chroma {
                *value /= sum;
            }
        }

        chroma
    }

    fn frequency_to_chroma(&self, frequency: f32) -> usize {
        // Convert frequency to MIDI note number, then to chroma class
        let midi_note = 69.0 + 12.0 * (frequency / 440.0).log2();
        (midi_note.round() as usize) % 12
    }
}