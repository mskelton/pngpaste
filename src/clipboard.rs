use arboard::{Clipboard, Error};
use image::{ImageBuffer, RgbaImage};

pub fn read_image() -> Result<RgbaImage, Error> {
    let mut clipboard = Clipboard::new().unwrap();
    let image = clipboard.get_image()?;

    let image = ImageBuffer::from_raw(
        image.width.try_into().unwrap(),
        image.height.try_into().unwrap(),
        image.bytes.into_owned(),
    );

    return match image {
        Some(image) => Ok(image),
        None => Err(Error::ContentNotAvailable),
    };
}
