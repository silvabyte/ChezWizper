use anyhow::{Result, Context};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use hound::{WavWriter, WavSpec};
use tracing::{error, info};

pub struct AudioRecorder {
    device: cpal::Device,
    config: cpal::StreamConfig,
    samples: Arc<Mutex<Vec<f32>>>,
}

impl AudioRecorder {
    pub fn new() -> Result<Self> {
        let host = cpal::default_host();
        let device = host.default_input_device()
            .context("No input device available")?;
        
        info!("Using audio device: {}", device.name()?);
        
        let _config = device.default_input_config()?;
        let config = cpal::StreamConfig {
            channels: 1,
            sample_rate: cpal::SampleRate(16000), // Whisper optimal
            buffer_size: cpal::BufferSize::Default,
        };
        
        Ok(Self {
            device,
            config,
            samples: Arc::new(Mutex::new(Vec::new())),
        })
    }
    
    pub async fn start_recording(&self) -> Result<()> {
        let samples = self.samples.clone();
        samples.lock().unwrap().clear();
        
        let err_fn = |err| error!("Audio stream error: {}", err);
        
        let samples_clone = samples.clone();
        let stream = self.device.build_input_stream(
            &self.config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let mut samples = samples_clone.lock().unwrap();
                samples.extend_from_slice(data);
            },
            err_fn,
            None,
        )?;
        
        stream.play()?;
        info!("Started audio recording");
        
        // Keep stream alive
        std::mem::forget(stream);
        
        Ok(())
    }
    
    pub async fn stop_recording(&self, output_path: PathBuf) -> Result<PathBuf> {
        let samples = self.samples.lock().unwrap().clone();
        
        if samples.is_empty() {
            return Err(anyhow::anyhow!("No audio samples recorded"));
        }
        
        info!("Stopping recording, {} samples captured", samples.len());
        
        // Write WAV file
        let spec = WavSpec {
            channels: 1,
            sample_rate: 16000,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        };
        
        let mut writer = WavWriter::create(&output_path, spec)?;
        for sample in samples {
            writer.write_sample(sample)?;
        }
        writer.finalize()?;
        
        info!("Audio saved to: {:?}", output_path);
        Ok(output_path)
    }
}