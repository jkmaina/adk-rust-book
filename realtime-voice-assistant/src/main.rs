use adk_realtime::openai::OpenAIRealtimeModel;
use adk_realtime::{RealtimeConfig, RealtimeModel, ServerEvent};
use anyhow::bail;

const MODEL_NAME: &str = "gpt-4o-mini-realtime-preview-2024-12-17";

fn live_smoke_requested() -> bool {
    matches!(std::env::var("BOOK_RUN_LIVE_SMOKE").as_deref(), Ok("1"))
}

// Realtime systems are event-driven. The application receives partial updates
// and must decide what to do with each event instead of waiting for one final
// blocking response.
async fn run_realtime_story() -> anyhow::Result<()> {
    rustls::crypto::ring::default_provider()
        .install_default()
        .ok();

    let api_key = std::env::var("OPENAI_API_KEY")?;
    let model = OpenAIRealtimeModel::new(&api_key, MODEL_NAME);
    let config = RealtimeConfig::default()
        .with_instruction(
            "You are a concise voice assistant for an operations team. Reply with a short spoken update, under 30 seconds, and keep the language crisp.",
        )
        .with_voice("shimmer")
        .with_text_and_audio();

    println!("Connecting to the realtime model...");
    let session = model.connect(config).await?;
    println!("Connected.\n");

    let prompt = "Give me a short spoken update for the incident channel: checkout latency is up, rollback is in progress, and the next status update will be in 10 minutes.";
    println!("Prompt: {prompt}\n");

    session.send_text(prompt).await?;
    session.create_response().await?;

    let mut transcript = String::new();
    let mut audio_bytes = Vec::new();

    print!("Transcript: ");
    while let Some(event) = session.next_event().await {
        match event? {
            ServerEvent::TranscriptDelta { delta, .. } => {
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
                bail!("realtime API error: {}", error.message);
            }
            _ => {}
        }
    }

    if transcript.trim().is_empty() {
        bail!("realtime session ended without transcript text");
    }

    if audio_bytes.is_empty() {
        println!("No audio buffer received.");
        return Ok(());
    }

    let wav = pcm16_to_wav(&audio_bytes, 24_000, 1);
    let output_dir = std::path::PathBuf::from("realtime-voice-assistant/audio-output");
    std::fs::create_dir_all(&output_dir)?;
    let output_path = output_dir.join("incident-update.wav");
    std::fs::write(&output_path, wav)?;

    let duration_s = audio_bytes.len() as f64 / (24_000.0 * 2.0);
    println!("Audio captured: {duration_s:.1}s");
    println!("Saved WAV: {}", output_path.display());

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

    println!("=== Realtime Voice Assistant ===\n");

    if !live_smoke_requested() {
        println!("This example demonstrates event-driven realtime output instead of one blocking final reply.");
        println!("Run with:");
        println!("  export OPENAI_API_KEY=...");
        println!("  BOOK_RUN_LIVE_SMOKE=1 cargo run -p realtime-voice-assistant");
        println!();
        println!("Expected live behavior:");
        println!("  - transcript text arrives incrementally");
        println!("  - audio chunks arrive separately");
        println!("  - the crate assembles the PCM stream into a WAV file");
        return Ok(());
    }

    match run_realtime_story().await {
        Ok(()) => Ok(()),
        Err(error) => {
            let message = error.to_string();
            if message.contains("Realtime Beta API is no longer supported") {
                println!("Live realtime path reached the provider, but published adk-realtime 0.8.2 still targets the retired OpenAI beta session contract.");
                println!("The example remains valid for architecture and event-loop design, but the live OpenAI path needs a newer upstream transport update.");
                return Ok(());
            }
            Err(error)
        }
    }
}
