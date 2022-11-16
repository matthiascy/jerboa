use std::{
    fmt::{Debug, Display},
    fs::File,
    io,
    io::{BufRead, BufReader},
    ops::{Deref, DerefMut},
    path::Path,
};

pub mod buffer;
pub mod error;
pub mod iters;
pub mod codec;

pub use buffer::*;
use crate::error::ImageError;

// TODO: color space

pub trait Sample: Copy + Clone + Default + Display + Debug {
    /// Size of a sample in bytes.
    const N_BYTES: usize;

    fn n_bytes() -> usize {
        Self::N_BYTES
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Bit(u8);

impl Deref for Bit {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Bit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for Bit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Sample for Bit {
    const N_BYTES: usize = 1;
}

impl Sample for u8 {
    const N_BYTES: usize = 1;
}
impl Sample for u16 {
    const N_BYTES: usize = 2;
}
impl Sample for u32 {
    const N_BYTES: usize = 4;
}
impl Sample for f32 {
    const N_BYTES: usize = 4;
}

pub trait Pixel: Copy + Clone + Default {
    /// Type of each channel.
    type Subpixel: Sample;

    /// Number of channels in the pixel.
    const N_CHANNELS: usize;

    /// Size of a pixel in bytes.
    const N_BYTES: usize = Self::N_CHANNELS * Self::Subpixel::N_BYTES;

    /// Returns a view of the pixel into a slice.
    fn from_slice(slice: &[Self::Subpixel]) -> &Self;

    /// Returns a mutable view of the pixel into a slice.
    fn from_slice_mut(slice: &mut [Self::Subpixel]) -> &mut Self;
}

macro_rules! impl_pixel_trait_array {
    ($($channels:expr),*) => {
        $(
            impl<S: Sample> Pixel for [S; $channels] {
                type Subpixel = S;

                const N_CHANNELS: usize = $channels;

                fn from_slice(slice: &[S]) -> &Self {
                    assert_eq!(slice.len(), Self::N_CHANNELS, "slice length mismatch while creating pixel view from slice");
                    unsafe {
                        &*(slice as *const [S] as *const Self)
                    }
                }

                fn from_slice_mut(slice: &mut [S]) -> &mut Self {
                    assert_eq!(slice.len(), Self::N_CHANNELS, "slice length mismatch while creating pixel view from slice");
                    unsafe {
                        &mut *(slice as *mut [S] as *mut Self)
                    }
                }
            }
        )*
    };
}

impl_pixel_trait_array! {1, 2, 3, 4}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    /// Portable any-map format. See [Netpbm](https://en.wikipedia.org/wiki/Netpbm).
    Pnm,
}

impl Display for ImageFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ImageFormat::Pnm => write!(f, "Pnm"),
        }
    }
}

impl ImageFormat {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, ImageError> {
        Self::_from_path(path.as_ref())
    }

    fn _from_path(path: &Path) -> Result<Self, ImageError> {
        if let Some(ext) = path.extension() {
            let ext = ext.to_str().unwrap();
            Self::_from_extension(ext).ok_or_else(|| ImageError::UnsupportedFormat(ext.to_string()))
        } else {
            Err(ImageError::UnsupportedFormat("no extension".to_string()))
        }
    }

    pub fn from_extension<S: AsRef<str>>(ext: S) -> Option<Self> {
        Self::_from_extension(ext.as_ref())
    }

    fn _from_extension(ext: &str) -> Option<Self> {
        match ext.to_ascii_lowercase().as_ref() {
            "pnm" => Some(ImageFormat::Pnm),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorType {
    /// 8-bit grayscale.
    Luma8,

    /// 8-bit grayscale with alpha.
    LumaA8,

    /// 8-bit rgb.
    Rgb8,

    /// 8-bit rgb with alpha.
    RgbA8,

    /// 16-bit grayscale.
    Luma16,

    /// 16-bit grayscale with alpha.
    LumaA16,

    /// 16-bit rgb.
    Rgb16,

    /// 16-bit rgb with alpha.
    RgbA16,

    /// 32-bit float grayscale.
    Luma32F,

    /// 32-bit float grayscale with alpha.
    LumaA32F,

    /// 32-bit float rgb.
    Rgb32F,

    /// 32-bit float rgb with alpha.
    RgbA32F,
}

impl ColorType {
    pub fn n_channels(&self) -> u32 {
        match self {
            ColorType::Luma8 | ColorType::Luma16 | ColorType::Luma32F => 1,
            ColorType::LumaA8 | ColorType::LumaA16 | ColorType::LumaA32F => 2,
            ColorType::Rgb8 | ColorType::Rgb16 | ColorType::Rgb32F => 3,
            ColorType::RgbA8 | ColorType::RgbA16 | ColorType::RgbA32F => 4,
        }
    }
}

pub struct ImageDecoder<R> {
    reader: R,
    format: Option<ImageFormat>,
}

impl<R: BufRead> ImageDecoder<R> {
    pub fn new(reader: R) -> Self {
        ImageDecoder {
            reader,
            format: None,
        }
    }

    pub fn with_format(reader: R, format: ImageFormat) -> Self {
        ImageDecoder {
            reader,
            format: Some(format),
        }
    }
}

impl ImageDecoder<BufReader<File>> {
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        Self::_open(path.as_ref())
    }

    fn _open(path: &Path) -> io::Result<Self> {
        Ok(ImageDecoder {
            reader: BufReader::new(File::open(path)?),
            format: ImageFormat::_from_path(path).ok(),
        })
    }

    pub fn decode(self) -> Result<ImageBuffer, ImageError> {
        match self.format {
            Some(ImageFormat::Pnm) => self.decode_pnm(),
            _ => Err(ImageError::UnsupportedFormat("empty".to_string())),
        }
    }

    fn decode_pnm(mut self) -> Result<ImageBuffer, ImageError> {
        codec::pnm::read_pnm_from_stream(&mut self.reader)
    }
}
