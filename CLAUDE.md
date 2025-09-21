# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with
code in this repository.

## Project Overview

A Hindustani Raag detection system implemented in Rust. The system analyzes
audio files to identify raags using signal processing and machine learning
techniques.

## Development Commands

- **Build**: `cargo build`
- **Run**: `cargo run -- <audio_file.wav>`
- **Run with verbose output**: `cargo run -- <audio_file.wav> --verbose`
- **Test**: `cargo test` (use "task test" per global CLAUDE.md)
- **Check**: `cargo check`
- **Lint**: `cargo clippy`
- **Format**: `cargo fmt`

## Project Architecture

### Core Modules

- **`src/audio/`** - Audio file I/O and preprocessing
  - `reader.rs` - WAV file reading with hound crate
  - `preprocessing.rs` - Normalization, windowing, DC removal

- **`src/features/`** - Musical feature extraction
  - `pitch.rs` - F0 extraction using autocorrelation
  - `chromagram.rs` - 12-bin chromagram computation
  - `spectral.rs` - Spectral analysis (centroid, rolloff, flux)

- **`src/classification/`** - Raag identification
  - `raag_db.rs` - Raag characteristics database
  - `classifier.rs` - Pattern matching and ML classification

### Key Dependencies

- **Audio**: `hound` for WAV I/O, `rustfft` for FFT operations
- **Math**: `ndarray` for numerical arrays
- **ML**: `candle-core` and `candle-nn` for neural networks
- **CLI**: `clap` for argument parsing
- **Error handling**: `anyhow` and `thiserror`

## Technical Notes

- Currently implements Raag Yaman and Raag Bhairavi as references
- Uses simplified autocorrelation for pitch detection
- Chromagram maps frequencies to 12 semitone bins
- Tonic (Sa) detection uses histogram-based frequency analysis
- Classification currently rule-based (placeholder for ML)