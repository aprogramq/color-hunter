use std::{
    borrow::Cow,
    error::Error,
    io::{self, Write},
    process::{Command, Stdio},
};

use arboard::{Clipboard, ImageData};
use image::{ImageEncoder, codecs::png::PngEncoder};

#[derive(Debug, PartialEq)]
pub enum ClipboardContent {
    Text(String),
    Image(image::RgbaImage),
}

pub fn set(clipboard: &mut Clipboard, content: ClipboardContent) -> Result<(), Box<dyn Error>> {
    match content {
        ClipboardContent::Text(text) => {
            if is_wsl() {
                return set_wsl_text(&text);
            }

            clipboard.set_text(text)?;
            Ok(())
        }
        ClipboardContent::Image(image) => {
            if is_wsl() {
                return set_wsl_image(&image);
            }

            let (width, height) = image.dimensions();
            let data = ImageData {
                width: width as usize,
                height: height as usize,
                bytes: Cow::Owned(image.into_raw()),
            };

            clipboard.set_image(data)?;
            Ok(())
        }
    }
}

fn set_wsl_text(text: &str) -> Result<(), Box<dyn Error>> {
    let mut child = Command::new("clip.exe").stdin(Stdio::piped()).spawn()?;
    child
        .stdin
        .take()
        .ok_or_else(|| io::Error::other("Failed to open clip.exe stdin"))?
        .write_all(text.as_bytes())?;

    let status = child.wait()?;
    if !status.success() {
        return Err(io::Error::other(format!("clip.exe exited with status {status}")).into());
    }

    Ok(())
}

fn set_wsl_image(image: &image::RgbaImage) -> Result<(), Box<dyn Error>> {
    let mut png = Vec::new();
    PngEncoder::new(&mut png).write_image(
        image.as_raw(),
        image.width(),
        image.height(),
        image::ExtendedColorType::Rgba8,
    )?;

    let script = r#"
$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing

$stdin = [Console]::OpenStandardInput()
$memory = [System.IO.MemoryStream]::new()

try {
    $stdin.CopyTo($memory)
    $memory.Position = 0
    $image = [System.Drawing.Image]::FromStream($memory)

    try {
        [System.Windows.Forms.Clipboard]::SetImage($image)
    }
    finally {
        $image.Dispose()
    }
}
finally {
    $memory.Dispose()
}
"#;

    let mut child = Command::new("powershell.exe")
        .args(["-NoProfile", "-NonInteractive", "-STA", "-Command", script])
        .stdin(Stdio::piped())
        .spawn()?;

    child
        .stdin
        .take()
        .ok_or_else(|| io::Error::other("Failed to open PowerShell stdin"))?
        .write_all(&png)?;

    let status = child.wait()?;
    if !status.success() {
        return Err(io::Error::other(format!("PowerShell exited with status {status}")).into());
    }

    Ok(())
}

fn is_wsl() -> bool {
    std::env::var_os("WSL_DISTRO_NAME").is_some()
        || std::fs::read_to_string("/proc/sys/kernel/osrelease")
            .is_ok_and(|release| release.to_ascii_lowercase().contains("microsoft"))
}
