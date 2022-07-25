use std::io;
use std::error::Error;
use crate::core::image::ImageFormat;

pub enum ImageError {
    /// An error occurred while reading an image.
    Decoding(DecodingError),

    /// An error occurred while writing an image.
    Encoding(EncodingError),
}

pub struct DecodingError {
    fmt: ImageFormat,
    err: Option<Box<dyn Error + Send + Sync>>
}

pub struct EncodingError {
    fmt: ImageFormat,
    err: Option<Box<dyn Error + Send + Sync>>
}