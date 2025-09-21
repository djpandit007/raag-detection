mod audio;
mod features;
mod classification;

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use audio::AudioReader;
use features::{PitchExtractor, ChromagramExtractor, SpectralAnalyzer};
use classification::{RaagClassifier, AudioFeatures};

#[derive(Parser)]
#[command(name = "raag-detection")]
#[command(about = "A Hindustani Raag detection system")]
struct Args {
    #[arg(help = "Path to the audio file")]
    audio_file: PathBuf,

    #[arg(short, long, help = "Output detailed analysis")]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("Analyzing audio file: {}", args.audio_file.display());

    // Read audio file
    let audio = AudioReader::from_file(&args.audio_file)?;
    println!("Audio loaded: {:.2}s, {} Hz, {} channels",
             audio.duration_seconds(),
             audio.sample_rate,
             audio.channels);

    // Extract features
    let pitch_extractor = PitchExtractor::new(audio.sample_rate);
    let chroma_extractor = ChromagramExtractor::new(audio.sample_rate, 2048);
    let spectral_analyzer = SpectralAnalyzer::new(audio.sample_rate, 2048);

    let pitch_contour = pitch_extractor.extract_f0(&audio.samples);
    let chromagram = chroma_extractor.extract_chromagram(&audio.samples);

    if args.verbose {
        println!("Extracted {} pitch frames", pitch_contour.len());
        println!("Extracted {} chroma frames", chromagram.len());
    }

    // Classify raag
    let features = AudioFeatures {
        pitch_contour,
        chromagram,
        spectral_centroid: vec![], // Placeholder
    };

    let classifier = RaagClassifier::new();
    match classifier.classify(&features)? {
        Some(raag) => println!("Detected Raag: {}", raag),
        None => println!("Could not identify raag"),
    }

    Ok(())
}
