use anyhow::Result;
use discord_webhook_rs::Webhook;
use rodio::{Decoder, OutputStream, Sink};
use std::{fs::File, io::BufReader};

/// Plays a notification sound from the specified file path with the given volume.
///
/// # Arguments
/// * `sound_path` - Path to the sound file.
/// * `volume` - Volume as a percentage (0-100).
///
/// # Returns
/// Ok(()) if successful, or an error if playback fails.
pub fn play_notification_sound(
    sound_path: &String,
    volume: u8,
) -> Result<(), Box<dyn std::error::Error>> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    sink.set_volume(volume as f32 / 100.0);

    let file = File::open(sound_path)?;
    let source = Decoder::new(BufReader::new(file))?;
    sink.append(source);
    sink.sleep_until_end();
    Ok(())
}

/// Sends a Discord webhook notification with the specified content, username, and avatar URL.
///
/// # Arguments
/// * `webhook_url` - Discord webhook URL.
/// * `content` - Message content to send.
/// * `username` - Optional username to display.
/// * `avatar_url` - Optional avatar URL to use.
///
/// # Returns
/// Ok(()) if successful, or an error if sending fails.
pub fn send_discord_webhook(
    webhook_url: &str,
    content: &str,
    username: Option<String>,
    avatar_url: Option<String>,
) -> Result<()> {
    let mut webhook = Webhook::new(webhook_url).content(content);

    let name = match username {
        Some(ref s) if !s.trim().is_empty() => s.clone(),
        _ => "Alert Sentinel".to_string(),
    };
    webhook = webhook.username(name);

    if let Some(url) = avatar_url {
        webhook = webhook.avatar_url(url);
    }

    let resp = webhook
        .send()
        .map_err(|e| anyhow::Error::msg(format!("{:?}", e)))?;
    if !resp.status().is_success() {
        println!("Discord webhook error body: {:?}", resp.text());
    }
    Ok(())
}
