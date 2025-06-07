use anyhow::{Context, Result};
use image::{ImageBuffer, Rgba};
use scrap::{Capturer, Display};
use std::{cell::RefCell, thread, time::Duration};

/// Manages screen capturing using the scrap crate.
pub struct CaptureManager {
    capturer: Option<Capturer>,
}

impl CaptureManager {
    /// Creates a new CaptureManager instance.
    fn new() -> Self {
        Self { capturer: None }
    }

    /// Captures the current screen and returns it as an ImageBuffer.
    ///
    /// # Returns
    /// Ok(ImageBuffer) if successful, or an error if capture fails.
    fn get_image(&mut self) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
        if self.capturer.is_none() {
            let display = Display::primary().context("Primary display not found")?;
            self.capturer = Some(Capturer::new(display).context("Failed to start capture")?);
        }

        let capturer = self.capturer.as_mut().unwrap();
        let (w, h) = (capturer.width(), capturer.height());

        let frame = loop {
            match capturer.frame() {
                Ok(buf) => break buf,
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(100));
                }
                Err(e) => return Err(e).context("Failed to acquire frame")?,
            }
        };

        let mut rgba_buf = Vec::with_capacity(w * h * 4);
        for chunk in frame.chunks(4) {
            rgba_buf.push(chunk[2]);
            rgba_buf.push(chunk[1]);
            rgba_buf.push(chunk[0]);
            rgba_buf.push(255);
        }

        let img: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_raw(w as u32, h as u32, rgba_buf)
            .context("Failed to create ImageBuffer")?;

        Ok(img)
    }
}

/// Captures the current screen buffer and returns it as an ImageBuffer.
///
/// # Returns
/// Ok(ImageBuffer) if successful, or an error if capture fails.
pub fn capture_screen_buffer() -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    CAPTURE_MANAGER.with(|manager| {
        let mut mgr = manager.borrow_mut();
        if mgr.is_none() {
            *mgr = Some(CaptureManager::new());
        }
        mgr.as_mut().unwrap().get_image()
    })
}

// Thread-local storage for the CaptureManager instance.
thread_local! {
    static CAPTURE_MANAGER: RefCell<Option<CaptureManager>> = const { RefCell::new(None) };
}
