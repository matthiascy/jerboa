use crate::core::image::ImageFormat;
use std::{
    error::Error,
    fmt::{Display, Formatter},
    io,
};

#[derive(Debug)]
pub enum ImageError {
    /// An error occurred while reading an image.
    Decoding(DecodingError),

    /// An error occurred while writing an image.
    Encoding(EncodingError),

    UnsupportedFormat(String),

    Io(io::Error),
}

impl Display for ImageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageError::Decoding(err) => write!(f, "decoding error: {}", err),
            ImageError::Encoding(err) => write!(f, "encoding error: {}", err),
            ImageError::UnsupportedFormat(err) => write!(f, "unsupported format: {}", err),
            ImageError::Io(err) => write!(f, "io error: {}", err),
        }
    }
}

impl Error for ImageError {}

impl From<io::Error> for ImageError {
    fn from(err: io::Error) -> Self {
        ImageError::Io(err)
    }
}

#[derive(Debug)]
pub struct DecodingError {
    /// The format of the image that was being decoded.
    pub fmt: ImageFormat,
    /// The underlying error.
    pub err: Box<dyn Error + Send + Sync>,
}

impl Display for DecodingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "decoding error [{}]: {}", self.fmt, self.err)
    }
}

impl DecodingError {
    pub fn new<E: Error + Sync + Send + 'static>(fmt: ImageFormat, err: E) -> Self {
        DecodingError {
            fmt,
            err: Box::new(err),
        }
    }
}

#[derive(Debug)]
pub struct EncodingError {
    /// The format of the image that was being encoded.
    pub fmt: ImageFormat,
    /// The underlying error.
    pub err: Box<dyn Error + Send + Sync>,
}

impl Display for EncodingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "encoding error [{}]: {}", self.fmt, self.err)
    }
}

impl EncodingError {
    pub fn new<E: Error + Sync + Send + 'static>(fmt: ImageFormat, err: E) -> Self {
        EncodingError {
            fmt,
            err: Box::new(err),
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    /// An error occurred while parsing an integer.
    Int(std::num::ParseIntError),
    /// An error occurred while parsing a floating point number.
    Float(std::num::ParseFloatError),
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Int(err) => write!(f, "int parse error: {}", err),
            ParseError::Float(err) => write!(f, "float parse error: {}", err),
        }
    }
}

impl From<std::num::ParseIntError> for ParseError {
    fn from(err: std::num::ParseIntError) -> Self {
        ParseError::Int(err)
    }
}

impl From<std::num::ParseFloatError> for ParseError {
    fn from(err: std::num::ParseFloatError) -> Self {
        ParseError::Float(err)
    }
}
