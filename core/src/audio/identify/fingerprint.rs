use crate::error::Error;
use base64::engine::general_purpose;
use base64::Engine;
use rusty_chromaprint::{Configuration, FingerprintCompressor, Fingerprinter};
use std::fs::File;
use std::io::Cursor;
use std::path::Path;
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::{CodecRegistry, DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia_adapter_libopus::OpusDecoder;

pub fn song_file_fingerprint(path: impl AsRef<Path>) -> crate::Result<(String, f64)> {
    let path = path.as_ref();
    let src = File::open(path)?;

    let mss = MediaSourceStream::new(Box::new(src), Default::default());

    let mut hint = Hint::new();
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        hint.with_extension(ext);
    }

    compute_fingerprint(mss, hint)
}

pub fn song_data_fingerprint(data: &[u8], ext: Option<&str>) -> crate::Result<(String, f64)> {
    let cursor = Cursor::new(data.to_vec());
    let mss = MediaSourceStream::new(Box::new(cursor), Default::default());

    let mut hint = Hint::new();
    if let Some(ext) = ext {
        hint.with_extension(ext);
    }

    compute_fingerprint(mss, hint)
}

fn compute_fingerprint(mss: MediaSourceStream, hint: Hint) -> crate::Result<(String, f64)> {
    let probed = symphonia::default::get_probe().format(
        &hint,
        mss,
        &FormatOptions::default(),
        &MetadataOptions::default(),
    )?;
    let mut format = probed.format;

    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .ok_or(Error::NoTrackFound)?;

    let mut registry = CodecRegistry::new();
    symphonia::default::register_enabled_codecs(&mut registry);
    registry.register_all::<OpusDecoder>();

    let mut decoder = registry.make(&track.codec_params, &DecoderOptions::default())?;

    let track_id = track.id;
    let sample_rate = track.codec_params.sample_rate.unwrap_or(44100);
    let channels = track.codec_params.channels.unwrap_or_default().count() as u8;

    let config = Configuration::preset_test2();
    let mut printer = Fingerprinter::new(&config);
    printer.start(sample_rate, channels as u32)?;

    let mut sample_buf = None;
    let mut total_samples = 0;

    while let Ok(packet) = format.next_packet() {
        if packet.track_id() != track_id {
            continue;
        }

        match decoder.decode(&packet) {
            Ok(audio_buf) => {
                if sample_buf.is_none() {
                    let spec = *audio_buf.spec();
                    let duration = audio_buf.capacity() as u64;
                    sample_buf = Some(SampleBuffer::<i16>::new(duration, spec));
                }

                if let Some(buf) = &mut sample_buf {
                    buf.copy_interleaved_ref(audio_buf);
                    printer.consume(buf.samples());
                    total_samples += buf.samples().len() / channels as usize;
                }
            }
            Err(err) => return Err(err.into()),
        }
    }

    printer.finish();

    let raw_fp = printer.fingerprint();
    let compressed_fp = FingerprintCompressor::from(&config).compress(raw_fp);
    let fingerprint = general_purpose::URL_SAFE_NO_PAD.encode(compressed_fp);
    let duration = total_samples as f64 / sample_rate as f64;

    Ok((fingerprint, duration))
}
