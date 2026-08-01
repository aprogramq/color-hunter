use core::fmt;
use std::{borrow::Cow, cell::RefCell, rc::Rc};

use arboard::ImageData;

#[derive(Clone, Default)]
pub struct Clipboard {
    clipboard: Rc<RefCell<Option<arboard::Clipboard>>>,
}

impl Clipboard {
    pub fn set_text(&self, text: String) -> Result<(), arboard::Error> {
        let mut clipboard = self.clipboard.borrow_mut();
        if clipboard.is_none() {
            *clipboard = Some(arboard::Clipboard::new()?);
        }

        clipboard
            .as_mut()
            .expect("clipboard was initialized")
            .set_text(text)
    }

    pub fn set_image(&self, image: image::RgbaImage) -> Result<(), arboard::Error> {
        let (width, height) = image.dimensions();
        let data = ImageData {
            width: width as usize,
            height: height as usize,
            bytes: Cow::Owned(image.into_raw()),
        };

        let mut clipboard = self.clipboard.borrow_mut();
        if clipboard.is_none() {
            *clipboard = Some(arboard::Clipboard::new()?);
        }
        clipboard
            .as_mut()
            .expect("clipboard was initialized")
            .set_image(data)
    }
}

impl fmt::Debug for Clipboard {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ClipboardState")
            .field("initialized", &self.clipboard.borrow().is_some())
            .finish()
    }
}
