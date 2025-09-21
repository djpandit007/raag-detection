use anyhow::{Result, bail};
use hound::WavReader;
use std::path::Path;
use std::fs::File;
use symphonia::core::audio::{AudioBufferRef, Signal};
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

pub struct AudioReader {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
    pub channels: u16,
}

impl AudioReader {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        // Detect format by file extension
        let extension = path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_lowercase());

        match extension.as_deref() {
            Some("wav") => Self::from_wav_file(path),
            Some("mp3") | Some("flac") | Some("ogg") => Self::from_symphonia_file(path),
            Some(ext) => bail!("Unsupported audio format: .{}\nSupported formats: .wav, .mp3, .flac, .ogg", ext),
            None => bail!("Could not determine audio format from file extension"),
        }
    }

    fn from_wav_file<P: AsRef<Path>>(path: P) -> Result<Self> {
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

    fn from_symphonia_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_ref = path.as_ref();

        // Open the media source
        let file = File::open(path_ref)?;
        let mss = MediaSourceStream::new(Box::new(file), Default::default());

        // Create a probe hint using the file extension
        let mut hint = Hint::new();
        if let Some(extension) = path_ref.extension() {
            if let Some(extension_str) = extension.to_str() {
                hint.with_extension(extension_str);
            }
        }

        // Use the default options for metadata and format readers
        let meta_opts: MetadataOptions = Default::default();
        let fmt_opts: FormatOptions = Default::default();

        // Probe the media source
        let probed = symphonia::default::get_probe().format(&hint, mss, &fmt_opts, &meta_opts)?;

        // Get the instantiated format reader
        let mut format = probed.format;

        // Find the first audio track with a known (decodeable) codec
        let track = format.tracks()
            .iter()
            .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
            .ok_or_else(|| anyhow::anyhow!("No supported audio tracks found"))?;

        // Use the default options for the decoder
        let dec_opts: DecoderOptions = Default::default();

        // Create a decoder for the track
        let mut decoder = symphonia::default::get_codecs().make(&track.codec_params, &dec_opts)?;

        // Store the track identifier, we'll use it to filter packets
        let track_id = track.id;

        let mut samples = Vec::new();
        let mut sample_rate = 0u32;
        let mut channels = 0u16;
        let max_samples = 10 * 44100; // Limit to first 10 seconds for testing

        // The decode loop
        loop {
            // Get the next packet from the media format
            let packet = match format.next_packet() {
                Ok(packet) => packet,
                Err(SymphoniaError::ResetRequired) => {
                    // The track list has been changed. Re-examine it and create a new set of decoders,
                    // then restart the decode loop. This is an advanced feature and we ignore it.
                    unimplemented!();
                }
                Err(SymphoniaError::IoError(_)) => {
                    // The packet reader has reached the end of the media, exit the loop
                    break;
                }
                Err(err) => {
                    // A unrecoverable error occurred, halt decoding
                    return Err(err.into());
                }
            };

            // Consume any new metadata that has been read since the last packet
            while !format.metadata().is_latest() {
                // Pop the latest metadata
                format.metadata().pop();
            }

            // If the packet does not belong to the selected track, skip over it
            if packet.track_id() != track_id {
                continue;
            }

            // Decode the packet into an audio buffer
            let decoded_buffer = match decoder.decode(&packet) {
                Ok(buf) => buf,
                Err(SymphoniaError::DecodeError(_)) => {
                    // Skip malformed frames and continue
                    continue;
                }
                Err(err) => return Err(err.into()),
            };

            match decoded_buffer {
                AudioBufferRef::F32(buf) => {
                    sample_rate = buf.spec().rate;
                    channels = buf.spec().channels.count() as u16;

                    // Convert planar to interleaved
                    for ch in 0..buf.spec().channels.count() {
                        let channel = buf.chan(ch);
                        for &sample in channel.iter() {
                            samples.push(sample);
                            if samples.len() >= max_samples {
                                break;
                            }
                        }
                        if samples.len() >= max_samples {
                            break;
                        }
                    }
                }
                AudioBufferRef::U8(buf) => {
                    sample_rate = buf.spec().rate;
                    channels = buf.spec().channels.count() as u16;

                    for ch in 0..buf.spec().channels.count() {
                        let channel = buf.chan(ch);
                        for &sample in channel.iter() {
                            samples.push((sample as f32 - 128.0) / 128.0);
                        }
                    }
                }
                AudioBufferRef::U16(buf) => {
                    sample_rate = buf.spec().rate;
                    channels = buf.spec().channels.count() as u16;

                    for ch in 0..buf.spec().channels.count() {
                        let channel = buf.chan(ch);
                        for &sample in channel.iter() {
                            samples.push((sample as f32 - 32768.0) / 32768.0);
                        }
                    }
                }
                AudioBufferRef::U24(buf) => {
                    sample_rate = buf.spec().rate;
                    channels = buf.spec().channels.count() as u16;

                    for ch in 0..buf.spec().channels.count() {
                        let channel = buf.chan(ch);
                        for &sample in channel.iter() {
                            let sample_f32 = sample.inner() as f32;
                            samples.push((sample_f32 - 8388608.0) / 8388608.0);
                        }
                    }
                }
                AudioBufferRef::U32(buf) => {
                    sample_rate = buf.spec().rate;
                    channels = buf.spec().channels.count() as u16;

                    for ch in 0..buf.spec().channels.count() {
                        let channel = buf.chan(ch);
                        for &sample in channel.iter() {
                            samples.push((sample as f32 - 2147483648.0) / 2147483648.0);
                        }
                    }
                }
                AudioBufferRef::S8(buf) => {
                    sample_rate = buf.spec().rate;
                    channels = buf.spec().channels.count() as u16;

                    for ch in 0..buf.spec().channels.count() {
                        let channel = buf.chan(ch);
                        for &sample in channel.iter() {
                            samples.push(sample as f32 / 128.0);
                        }
                    }
                }
                AudioBufferRef::S16(buf) => {
                    sample_rate = buf.spec().rate;
                    channels = buf.spec().channels.count() as u16;

                    for ch in 0..buf.spec().channels.count() {
                        let channel = buf.chan(ch);
                        for &sample in channel.iter() {
                            samples.push(sample as f32 / 32768.0);
                        }
                    }
                }
                AudioBufferRef::S24(buf) => {
                    sample_rate = buf.spec().rate;
                    channels = buf.spec().channels.count() as u16;

                    for ch in 0..buf.spec().channels.count() {
                        let channel = buf.chan(ch);
                        for &sample in channel.iter() {
                            let sample_f32 = sample.inner() as f32;
                            samples.push(sample_f32 / 8388608.0);
                        }
                    }
                }
                AudioBufferRef::S32(buf) => {
                    sample_rate = buf.spec().rate;
                    channels = buf.spec().channels.count() as u16;

                    for ch in 0..buf.spec().channels.count() {
                        let channel = buf.chan(ch);
                        for &sample in channel.iter() {
                            samples.push(sample as f32 / 2147483648.0);
                        }
                    }
                }
                AudioBufferRef::F64(buf) => {
                    sample_rate = buf.spec().rate;
                    channels = buf.spec().channels.count() as u16;

                    for ch in 0..buf.spec().channels.count() {
                        let channel = buf.chan(ch);
                        for &sample in channel.iter() {
                            samples.push(sample as f32);
                        }
                    }
                }
            }

            // Break if we've collected enough samples
            if samples.len() >= max_samples {
                break;
            }
        }

        if samples.is_empty() {
            bail!("No audio data found in file");
        }

        Ok(AudioReader {
            samples,
            sample_rate,
            channels,
        })
    }

    pub fn duration_seconds(&self) -> f32 {
        self.samples.len() as f32 / (self.sample_rate as f32 * self.channels as f32)
    }
}