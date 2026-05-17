use adk_realtime::openai::OpenAIRealtimeModel;
use adk_realtime::{RealtimeConfig, RealtimeModel, ServerEvent};
use base64::Engine;

const MODEL_NAME: &str = "gpt-4o-mini-realtime-preview-2024-12-17";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    rustls::crypto::ring::default_provider()
        .install_default()
        .ok();

    let api_key = std::env::var("OPENAI_API_KEY")?;
    let model = OpenAIRealtimeModel::new(&api_key, MODEL_NAME);
    let config = RealtimeConfig::default()
        .with_instruction(
            "You are a warm storyteller. The user gives you a topic. Reply with a vivid micro-story, short enough to speak in about 30 seconds.",
        )
        .with_voice("shimmer")
        .with_text_and_audio();

    println!("<!--AUDIO_STREAM_START:24000-->");
    println!("=== Realtime Voice: OpenAI Realtime API ===\n");
    println!("Connecting to OpenAI Realtime API...");

    let session = model.connect(config).await?;
    println!("Connected.\n");

    let prompt = "A lighthouse keeper who discovers a message in a bottle from the future.";
    println!("Prompt: {prompt}\n");
    session.send_text(prompt).await?;
    session.create_response().await?;

    let mut audio_bytes = Vec::new();
    let base64 = base64::engine::general_purpose::STANDARD;

    print!("Transcript: ");
    while let Some(event) = session.next_event().await {
        match event? {
            ServerEvent::AudioDelta { delta, .. } => {
                let encoded = base64.encode(&delta);
                println!("<!--AUDIO_CHUNK:{encoded}-->");
                audio_bytes.extend_from_slice(&delta);
            }
            ServerEvent::TranscriptDelta { delta, .. } => {
                print!("{delta}");
            }
            ServerEvent::ResponseDone { .. } => {
                println!("\n");
                break;
            }
            ServerEvent::Error { error, .. } => {
                println!("\nError: {}", error.message);
                break;
            }
            _ => {}
        }
    }

    println!("<!--AUDIO_STREAM_END-->");

    if audio_bytes.is_empty() {
        println!("No audio received.");
        return Ok(());
    }

    let duration_s = audio_bytes.len() as f64 / (24_000.0 * 2.0);
    let size_kb = audio_bytes.len() / 1024;
    println!("Audio: {duration_s:.1}s at 24kHz, {size_kb}KB");

    let wav = pcm16_to_wav(&audio_bytes, 24_000, 1);
    let output_dir = std::path::PathBuf::from("audio-output");
    std::fs::create_dir_all(&output_dir)?;
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();
    let filename = format!("realtime-{timestamp}.wav");
    std::fs::write(output_dir.join(&filename), &wav)?;

    println!("<!--AUDIO_URL:/api/audio/{filename}-->");
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
