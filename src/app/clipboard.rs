use std::borrow::Cow;

use arboard::{Clipboard, ImageData};

#[derive(Debug, PartialEq)]
pub enum ClipboardContent {
    Text(String),
    Image(image::RgbaImage),
}

pub fn set(clipboard: &mut Clipboard, content: ClipboardContent) -> Result<(), arboard::Error> {
    match content {
        ClipboardContent::Text(text) => clipboard.set_text(text),
        ClipboardContent::Image(image) => {
            let (width, height) = image.dimensions();
            let data = ImageData {
                width: width as usize,
                height: height as usize,
                bytes: Cow::Owned(image.into_raw()),
            };

            clipboard.set_image(data)
        }
    }
}
