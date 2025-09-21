use anyhow::Result;
use hound::WavReader;
use std::path::Path;

pub struct AudioReader {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
    pub channels: u16,
}

impl AudioReader {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut reader = WavReader::open(path)?;
        let spec = reader.spec();

        let samples: Result<Vec<f32>, _> = reader
            .samples::<i16>()
            .map(|s| s.map(|sample| sample as f32 / 32768.0))
            .collect();
        let samples = samples?;

        Ok(AudioReader {
            samples,
            sample_rate: spec.sample_rate,
            channels: spec.channels,
        })
    }

    pub fn duration_seconds(&self) -> f32 {
        self.samples.len() as f32 / (self.sample_rate as f32 * self.channels as f32)
    }
}