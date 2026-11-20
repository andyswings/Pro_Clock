use std::path::Path;
use std::process::Command;

/// Sends a native desktop notification (Ubuntu/Linux `notify-send`, Windows balloon/toast via PowerShell, macOS `osascript`).
/// Fails silently if the platform notification tool is unavailable.
pub fn send_notification(enabled: bool, title: &str, body: &str) {
    if !enabled {
        return;
    }

    #[cfg(target_os = "linux")]
    {
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

    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;

        let safe_title = title.replace('\'', "''");
        let safe_body = body.replace('\'', "''");
        let script = format!(
            "Add-Type -AssemblyName System.Windows.Forms; \
             $n = New-Object System.Windows.Forms.NotifyIcon; \
             $n.Icon = [System.Drawing.SystemIcons]::Information; \
             $n.Visible = $true; \
             $n.ShowBalloonTip(5000, '{safe_title}', '{safe_body}', [System.Windows.Forms.ToolTipIcon]::Info); \
             Start-Sleep -Seconds 6; \
             $n.Dispose()"
        );
        let _ = Command::new("powershell")
            .args([
                "-NoProfile",
                "-NonInteractive",
                "-WindowStyle",
                "Hidden",
                "-Command",
                &script,
            ])
            .creation_flags(CREATE_NO_WINDOW)
            .spawn();
    }

    #[cfg(target_os = "macos")]
    {
        let safe_title = title.replace('"', "\\\"");
        let safe_body = body.replace('"', "\\\"");
        let script = format!(
            "display notification \"{}\" with title \"{}\"",
            safe_body, safe_title
        );
        let _ = Command::new("osascript").args(["-e", &script]).spawn();
    }
}

/// Opens a folder in the native OS file manager (`explorer` on Windows, `open` on macOS, `xdg-open` on Linux).
pub fn open_folder(path: &Path) {
    #[cfg(target_os = "windows")]
    {
        let _ = Command::new("explorer").arg(path).spawn();
    }

    #[cfg(target_os = "macos")]
    {
        let _ = Command::new("open").arg(path).spawn();
    }

    #[cfg(target_os = "linux")]
    {
        let _ = Command::new("xdg-open").arg(path).spawn();
    }
}
