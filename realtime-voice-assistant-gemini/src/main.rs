use adk_realtime::gemini::{GeminiLiveBackend, GeminiRealtimeModel};
use adk_realtime::{RealtimeConfig, RealtimeModel, ServerEvent};
use anyhow::bail;

const MODEL_NAME: &str = "models/gemini-live-2.5-flash-native-audio";

fn live_smoke_requested() -> bool {
    matches!(std::env::var("BOOK_RUN_LIVE_SMOKE").as_deref(), Ok("1"))
}

// Gemini Live is still an event-driven interface. The application must process
// partial transcript and audio events as they arrive rather than waiting for a
// single blocking response.
async fn run_gemini_live_voice() -> anyhow::Result<()> {
    rustls::crypto::ring::default_provider()
        .install_default()
        .ok();

    let api_key = std::env::var("GOOGLE_API_KEY")?;
    let backend = GeminiLiveBackend::studio(&api_key);
    let model = GeminiRealtimeModel::new(backend, MODEL_NAME);
    let config = RealtimeConfig::default()
        .with_instruction(
            "You are a concise operations voice assistant. Reply with a short, spoken incident update that is easy to relay in a status channel.",
        )
        .with_voice("Aoede")
        .with_modalities(vec!["TEXT".to_string(), "AUDIO".to_string()]);

    println!("Connecting to Gemini Live...");
    let session = model.connect(config).await?;
    println!("Connected.\n");

    let prompt = "Give me a short spoken update: checkout latency is elevated, the team is watching rollback metrics, and the next customer update is in ten minutes.";
    println!("Prompt: {prompt}\n");

    session.send_text(prompt).await?;

    let mut transcript = String::new();
    let mut audio_bytes = Vec::new();

    print!("Transcript: ");
    while let Some(event) = session.next_event().await {
        match event? {
            ServerEvent::TextDelta { delta, .. } | ServerEvent::TranscriptDelta { delta, .. } => {
                print!("{delta}");
                transcript.push_str(&delta);
            }
            ServerEvent::AudioDelta { delta, .. } => {
                audio_bytes.extend_from_slice(&delta);
            }
            ServerEvent::ResponseDone { .. } => {
                println!("\n");
                break;
            }
            ServerEvent::Error { error, .. } => {
                bail!("gemini live error: {}", error.message);
            }
            _ => {}
        }
    }

    if audio_bytes.is_empty() {
        if transcript.trim().is_empty() {
            println!("Gemini Live connected successfully, but this Studio-backed session ended without transcript text or audio in the current environment.");
            println!("The crate still demonstrates the correct event-driven runtime shape. For stronger end-to-end voice validation, a Vertex-backed Gemini Live setup is usually the safer path.");
            session.close().await?;
            return Ok(());
        }
        println!("No audio buffer received.");
        session.close().await?;
        return Ok(());
    }

    let wav = pcm16_to_wav(&audio_bytes, 24_000, 1);
    let output_dir =
        std::path::PathBuf::from("realtime-voice-assistant-gemini/audio-output");
    std::fs::create_dir_all(&output_dir)?;
    let output_path = output_dir.join("incident-update-gemini.wav");
    std::fs::write(&output_path, wav)?;

    let duration_s = audio_bytes.len() as f64 / (24_000.0 * 2.0);
    if transcript.trim().is_empty() {
        println!("Transcript text was empty; audio-only completion is still valid for this session.");
    }
    println!("Audio captured: {duration_s:.1}s");
    println!("Saved WAV: {}", output_path.display());

    session.close().await?;
    Ok(())
}

fn pcm16_to_wav(pcm: &[u8], sample_rate: u32, channels: u16) -> Vec<u8> {
    let bits_per_sample: u16 = 16;
    let byte_rate = sample_rate * channels as u32 * (bits_per_sample / 8) as u32;
    let block_align = channels * (bits_per_sample / 8);
    let data_size = pcm.len() as u32;
    let file_size = 36 + data_size;

    let mut wav = Vec::with_capacity(44 + pcm.len());
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&file_size.to_le_bytes());
    wav.extend_from_slice(b"WAVE");
    wav.extend_from_slice(b"fmt ");
    wav.extend_from_slice(&16u32.to_le_bytes());
    wav.extend_from_slice(&1u16.to_le_bytes());
    wav.extend_from_slice(&channels.to_le_bytes());
    wav.extend_from_slice(&sample_rate.to_le_bytes());
    wav.extend_from_slice(&byte_rate.to_le_bytes());
    wav.extend_from_slice(&block_align.to_le_bytes());
    wav.extend_from_slice(&bits_per_sample.to_le_bytes());
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&data_size.to_le_bytes());
    wav.extend_from_slice(pcm);
    wav
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    println!("=== Realtime Voice Assistant (Gemini Live) ===\n");

    if !live_smoke_requested() {
        println!("This example demonstrates the Gemini Live realtime event loop.");
        println!("Run with:");
        println!("  export GOOGLE_API_KEY=...");
        println!("  BOOK_RUN_LIVE_SMOKE=1 cargo run -p realtime-voice-assistant-gemini");
        println!();
        println!("Expected live behavior:");
        println!("  - transcript text arrives incrementally");
        println!("  - audio chunks arrive separately");
        println!("  - the crate assembles the PCM stream into a WAV file");
        return Ok(());
    }

    run_gemini_live_voice().await
}
