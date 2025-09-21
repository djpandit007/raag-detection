use rustfft::{FftPlanner, num_complex::Complex};

pub struct SpectralAnalyzer {
    sample_rate: u32,
    fft_size: usize,
}

impl SpectralAnalyzer {
    pub fn new(sample_rate: u32, fft_size: usize) -> Self {
        Self {
            sample_rate,
            fft_size,
        }
    }

    pub fn spectral_centroid(&self, spectrum: &[f32]) -> f32 {
        let mut weighted_sum = 0.0;
        let mut magnitude_sum = 0.0;

        for (bin, &magnitude) in spectrum.iter().enumerate() {
            let frequency = bin as f32 * self.sample_rate as f32 / self.fft_size as f32;
            weighted_sum += frequency * magnitude;
            magnitude_sum += magnitude;
        }

        if magnitude_sum > 0.0 {
            weighted_sum / magnitude_sum
        } else {
            0.0
        }
    }

    pub fn spectral_rolloff(&self, spectrum: &[f32], percentile: f32) -> f32 {
        let total_energy: f32 = spectrum.iter().map(|x| x * x).sum();
        let threshold = total_energy * percentile;

        let mut cumulative_energy = 0.0;
        for (bin, &magnitude) in spectrum.iter().enumerate() {
            cumulative_energy += magnitude * magnitude;
            if cumulative_energy >= threshold {
                return bin as f32 * self.sample_rate as f32 / self.fft_size as f32;
            }
        }

        self.sample_rate as f32 / 2.0 // Nyquist frequency
    }

    pub fn spectral_flux(&self, prev_spectrum: &[f32], curr_spectrum: &[f32]) -> f32 {
        prev_spectrum
            .iter()
            .zip(curr_spectrum.iter())
            .map(|(&prev, &curr)| (curr - prev).max(0.0).powi(2))
            .sum::<f32>()
            .sqrt()
    }
}