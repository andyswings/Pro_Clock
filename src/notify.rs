use std::process::Command;

/// Sends a native Ubuntu desktop notification via `notify-send` (libnotify).
/// Fails silently if `notify-send` is not installed or no notification daemon is running.
pub fn send_notification(enabled: bool, title: &str, body: &str) {
    if !enabled {
        return;
    }
    let _ = Command::new("notify-send")
        .args([
            "-a",
            "Pro Clock",
            "-i",
            "pro-clock",
            "-u",
            "normal",
            title,
            body,
        ])
        .spawn();
}
