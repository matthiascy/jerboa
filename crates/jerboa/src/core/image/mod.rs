use crate::core::{
    image::{
        codec::{
            pnm,
            pnm::{Encoding, Header, Subtype, TupleType},
        },
        error::ImageError,
    },
    Vec1, Vec2, Vec3, Vec4,
};
use std::{
    fmt::{Debug, Display},
    fs::File,
    io,
    io::{BufRead, BufReader, BufWriter},
    ops::{Deref, DerefMut},
    path::Path,
};

pub mod codec;
pub mod error;

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

macro_rules! impl_pixel_trait {
    ($($name:ident<S> { $channels:expr };)*) => {
        $(
            impl<S: Sample> Pixel for $name<S> {
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

impl_pixel_trait! {
    Vec1<S> { 1 }; Vec2<S> { 2 }; Vec3<S> { 3 }; Vec4<S> { 4 };
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct PixelBuffer<P: Pixel> {
    width: u32,
    height: u32,
    pixels: Vec<P::Subpixel>,
}

pub type PixelBufferRgb8 = PixelBuffer<Vec3<u8>>;
pub type PixelBufferRgb16 = PixelBuffer<Vec3<u16>>;
pub type PixelBufferRgb32f = PixelBuffer<Vec3<f32>>;

impl<P: Pixel> PixelBuffer<P> {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn new(width: u32, height: u32) -> Self {
        PixelBuffer {
            width,
            height,
            pixels: vec![P::Subpixel::default(); (width * height) as usize * P::N_CHANNELS],
        }
    }

    pub fn n_channels(&self) -> usize {
        P::N_CHANNELS
    }

    pub fn flat_pixels(&self) -> &[P::Subpixel] {
        self.pixels.as_slice()
    }

    pub fn pixel_at(&self, x: u32, y: u32) -> Option<&P> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let index = (y * self.width + x) as usize * P::N_CHANNELS;
        Option::from(P::from_slice(&self.pixels[index..index + P::N_CHANNELS]))
    }

    pub fn pixel_at_mut(&mut self, x: u32, y: u32) -> Option<&mut P> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let index = (y * self.width + x) as usize * P::N_CHANNELS;
        Option::from(P::from_slice_mut(
            &mut self.pixels[index..index + P::N_CHANNELS],
        ))
    }
}

impl PixelBuffer<Vec1<Bit>> {
    pub fn write_as_pbm<P: AsRef<Path>>(
        &self,
        path: P,
        encoding: Encoding,
    ) -> Result<(), ImageError> {
        self._write_as_pbm(path.as_ref(), encoding)
    }

    fn _write_as_pbm(&self, path: &Path, encoding: Encoding) -> Result<(), ImageError> {
        let path = path.with_extension("pbm");
        let mut writer = BufWriter::new(File::create(path)?);
        let header = Header {
            subtype: Subtype::BitMap(encoding),
            width: self.width,
            height: self.height,
            max_val: 1.0,
            n_channels: 1,
            tuple_type: TupleType::BlackAndWhiteBit,
        };
        pnm::write_pnm_to_stream::<Bit, _>(&mut writer, header, self.flat_pixels())
    }

    pub fn write_as_pam<P: AsRef<Path>>(&self, path: P) -> Result<(), ImageError> {
        self._write_as_pam(path.as_ref())
    }

    fn _write_as_pam(&self, path: &Path) -> Result<(), ImageError> {
        let path = path.with_extension("pam");
        let mut writer = BufWriter::new(File::create(path)?);
        let header = Header {
            subtype: Subtype::ArbitraryMap,
            width: self.width,
            height: self.height,
            max_val: 1.0,
            n_channels: 1,
            tuple_type: TupleType::BlackAndWhite,
        };
        let samples = self.flat_pixels().iter().map(|x| x.0).collect::<Vec<_>>();
        pnm::write_pnm_to_stream::<u8, _>(&mut writer, header, &samples)
    }
}

impl PixelBuffer<Vec1<u8>> {
    pub fn write_as_pgm<P: AsRef<Path>>(
        &self,
        path: P,
        encoding: Encoding,
    ) -> Result<(), ImageError> {
        self._write_as_pgm(path.as_ref(), encoding)
    }

    fn _write_as_pgm(&self, path: &Path, encoding: Encoding) -> Result<(), ImageError> {
        let path = path.with_extension("pgm");
        let mut writer = BufWriter::new(File::create(path)?);
        let header = Header {
            subtype: Subtype::GrayMap(encoding),
            width: self.width,
            height: self.height,
            max_val: 255.0,
            n_channels: 1,
            tuple_type: TupleType::GrayScale,
        };
        pnm::write_pnm_to_stream::<u8, _>(&mut writer, header, self.flat_pixels())
    }
}

impl PixelBuffer<Vec1<f32>> {
    pub fn write_as_pfm<P: AsRef<Path>>(&self, path: P) -> Result<(), ImageError> {
        self._write_as_pfm(path.as_ref())
    }

    fn _write_as_pfm(&self, path: &Path) -> Result<(), ImageError> {
        let path = path.with_extension("pfm");
        let mut writer = BufWriter::new(File::create(path)?);
        let header = Header {
            subtype: Subtype::FloatGrayMap,
            width: self.width,
            height: self.height,
            max_val: 1.0,
            n_channels: 1,
            tuple_type: TupleType::FloatGrayScale,
        };
        pnm::write_pnm_to_stream::<f32, _>(&mut writer, header, self.flat_pixels())
    }
}

impl PixelBuffer<Vec3<u8>> {
    pub fn write_as_ppm<P: AsRef<Path>>(
        &self,
        path: P,
        encoding: Encoding,
    ) -> Result<(), ImageError> {
        self._write_as_ppm(path.as_ref(), encoding)
    }

    fn _write_as_ppm(&self, path: &Path, encoding: Encoding) -> Result<(), ImageError> {
        let path = path.with_extension("ppm");
        let mut writer = BufWriter::new(File::create(path)?);
        let header = Header {
            subtype: Subtype::PixMap(encoding),
            width: self.width,
            height: self.height,
            max_val: 255.0,
            n_channels: 3,
            tuple_type: TupleType::Rgb,
        };
        pnm::write_pnm_to_stream::<u8, _>(&mut writer, header, self.flat_pixels())
    }
}

impl PixelBuffer<Vec3<u16>> {
    pub fn write_as_ppm<P: AsRef<Path>>(
        &self,
        path: P,
        encoding: Encoding,
    ) -> Result<(), ImageError> {
        self._write_as_ppm(path.as_ref(), encoding)
    }

    fn _write_as_ppm(&self, path: &Path, encoding: Encoding) -> Result<(), ImageError> {
        let path = path.with_extension("ppm");
        let mut writer = BufWriter::new(File::create(path)?);
        let header = Header {
            subtype: Subtype::PixMap(encoding),
            width: self.width,
            height: self.height,
            max_val: 65535.0,
            n_channels: 3,
            tuple_type: TupleType::Rgb,
        };
        pnm::write_pnm_to_stream::<u16, _>(&mut writer, header, self.flat_pixels())
    }
}

macro_rules! impl_write_as_pam {
    (
        $(PixelBuffer<$p:ident<$s:ty>, $n:expr, $tupltype:path>;)*
    ) => {
        $(
            impl PixelBuffer<$p<$s>> {
                pub fn write_as_pam<P: AsRef<Path>>(&self, path: P) -> Result<(), ImageError> {
                    self._write_as_pam(path.as_ref())
                }

                fn _write_as_pam(&self, path: &Path) -> Result<(), ImageError> {
                    let path = path.with_extension("pam");
                    let mut writer = BufWriter::new(File::create(path)?);
                    let header = Header {
                        subtype: Subtype::ArbitraryMap,
                        width: self.width,
                        height: self.height,
                        max_val: <$s>::MAX as f32,
                        n_channels: $n,
                        tuple_type: $tupltype,
                    };
                    pnm::write_pnm_to_stream::<$s, _>(&mut writer, header, self.flat_pixels())
                }
            }
        )*
    };
}

impl PixelBuffer<Vec3<f32>> {
    pub fn write_as_pfm<P: AsRef<Path>>(&self, path: P) -> Result<(), ImageError> {
        self._write_as_pfm(path.as_ref())
    }

    pub fn _write_as_pfm(&self, path: &Path) -> Result<(), ImageError> {
        let path = path.with_extension("pfm");
        let mut writer = BufWriter::new(File::create(path)?);
        let header = Header {
            subtype: Subtype::FloatPixMap,
            width: self.width,
            height: self.height,
            max_val: 1.0,
            n_channels: 3,
            tuple_type: TupleType::FloatRgb,
        };
        pnm::write_pnm_to_stream::<f32, _>(&mut writer, header, self.flat_pixels())
    }
}

impl_write_as_pam! {
    PixelBuffer<Vec1<u8>, 1, TupleType::GrayScale>;
    PixelBuffer<Vec1<u16>, 1, TupleType::GrayScale>;
    PixelBuffer<Vec2<u16>, 2, TupleType::GrayScaleAlpha>;
    PixelBuffer<Vec2<u8>, 2, TupleType::GrayScaleAlpha>;
    PixelBuffer<Vec3<u8>, 3, TupleType::Rgb>;
    PixelBuffer<Vec3<u16>, 3, TupleType::Rgb>;
    PixelBuffer<Vec4<u8>, 4, TupleType::RgbAlpha>;
    PixelBuffer<Vec4<u16>, 4, TupleType::RgbAlpha>;
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum ImageBuffer {
    Bitmap(PixelBuffer<Vec1<Bit>>),
    Luma8(PixelBuffer<Vec1<u8>>),
    LumaA8(PixelBuffer<Vec2<u8>>),
    Luma16(PixelBuffer<Vec1<u16>>),
    LumaA16(PixelBuffer<Vec2<u16>>),
    Luma32F(PixelBuffer<Vec1<f32>>),
    Rgb8(PixelBuffer<Vec3<u8>>),
    RgbA8(PixelBuffer<Vec4<u8>>),
    Rgb16(PixelBuffer<Vec3<u16>>),
    RgbA16(PixelBuffer<Vec4<u16>>),
    Rgb32F(PixelBuffer<Vec3<f32>>),
}

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
        pnm::read_pnm_from_stream(&mut self.reader)
    }
}
