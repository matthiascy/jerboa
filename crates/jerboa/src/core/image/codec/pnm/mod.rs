use std::io::Read;
use std::str::FromStr;
use byteorder::ByteOrder;
use crate::core::image::codec::pnm::error::Error;

mod error;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TupleType {
    BlackAndWhite,
    GrayScale,
    Rgb,
    BlackAndWhiteAlpha,
    GrayScaleAlpha,
    RgbAlpha,
    FloatGrayScale,
    FloatRgb,
    BlackAndWhiteBit,
}

/// Represents a single value of a pixel in a PNM image.
pub trait Sample: Sized {
    /// Size of a sample in bytes.
    const N_BYTES: usize;

    /// Underlying type of the sample.
    type Underlying;

    /// Result obtained through parsing.
    type ParsingRecord = Self::Underlying;

    /// Parse a single sample from an ASCII string then write it into the given buffer.
    fn parse_from_ascii(src: &str) -> Result<Self::Underlying, Error>;

    /// Parse a single sample from a binary buffer (little endian).
    fn parse_from_bytes_le(src: &[u8; Self::N_BYTES]) -> Result<Self::ParsingRecord, Error>;

    /// Parse a single sample from a binary buffer (big endian).
    fn parse_from_bytes_be(src: &[u8; Self::N_BYTES]) -> Result<Self::ParsingRecord, Error>;
}

/// Represent a bit sample in PBM file.
/// Black is encoded as 1 and white as 0.
#[derive(Debug)]
pub struct Sbit;

impl Sample for Sbit {
    const N_BYTES: usize = 1;
    type Underlying = u8;
    type ParsingRecord = [u8; 8];

    fn parse_from_ascii(src: &str) -> Result<u8, Error> {
        src.parse::<u8>().map_err(Into::into)
    }

    fn parse_from_bytes_le(src: &[u8; 1]) -> Result<[u8; 8], Error> {
        // Each bit is encoded as a single byte.
        let mut output = [0; 8];
        output.iter_mut().enumerate().for_each(|(i, val)| {
            *val = (((src[0] << i) & 128) >> 7) ^ 1;
        });
        Ok(output)
    }

    fn parse_from_bytes_be(src: &[u8; 1]) -> Result<[u8; 8], Error> {
        Self::parse_from_bytes_le(src)
    }
}

#[derive(Debug)]
pub struct Su8;

impl Sample for Su8 {
    const N_BYTES: usize = 1;
    type Underlying = u8;

    fn parse_from_ascii(src: &str) -> Result<Self::Underlying, Error> {
        src.parse::<u8>().map_err(Into::into)
    }

    fn parse_from_bytes_le(src: &[u8; Self::N_BYTES]) -> Result<Self::ParsingRecord, Error> {
        Ok(src[0])
    }

    fn parse_from_bytes_be(src: &[u8; Self::N_BYTES]) -> Result<Self::ParsingRecord, Error> {
        Self::parse_from_bytes_le(src)
    }
}

#[derive(Debug)]
pub struct Su16;

impl Sample for Su16 {
    const N_BYTES: usize = 2;
    type Underlying = u16;

    fn parse_from_ascii(src: &str) -> Result<Self::Underlying, Error> {
        src.parse::<u16>().map_err(Into::into)
    }

    fn parse_from_bytes_le(src: &[u8; Self::N_BYTES]) -> Result<Self::ParsingRecord, Error> {
        Ok(byteorder::LittleEndian::read_u16(src))
    }

    fn parse_from_bytes_be(src: &[u8; Self::N_BYTES]) -> Result<Self::ParsingRecord, Error> {
        Ok(byteorder::BigEndian::read_u16(src))
    }
}

#[derive(Debug)]
pub struct Sf32;

impl Sample for Sf32 {
    const N_BYTES: usize = 4;
    type Underlying = f32;

    fn parse_from_ascii(src: &str) -> Result<Self::Underlying, Error> {
        src.parse::<f32>().map_err(Into::into)
    }

    fn parse_from_bytes_le(src: &[u8; Self::N_BYTES]) -> Result<Self::ParsingRecord, Error> {
        Ok(byteorder::LittleEndian::read_f32(src))
    }

    fn parse_from_bytes_be(src: &[u8; Self::N_BYTES]) -> Result<Self::ParsingRecord, Error> {
        Ok(byteorder::BigEndian::read_f32(src))
    }
}

pub trait Pixel {
    const N_CHANNELS: usize;
    const N_BYTES: usize;
}

macro_rules! define_pixel_types {
    ($($name:ident, $n:expr);+) => {
        $(
            #[derive(Debug, Copy, Clone, PartialEq, Eq)]
            pub struct $name<S: Sample>([S; $n]);

            impl<S: Sample> Pixel for $name<S> {
                const N_CHANNELS: usize = $n;
                const N_BYTES: usize = S::N_BYTES * Self::N_CHANNELS;
            }
        )+
    };
}

define_pixel_types! {
    Rgb, 3;
    RgbA, 4;
    Bw, 1;
    BwA, 2;
    Luma, 1;
    LumaA, 2
}

/// Sample encoding
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Encoding {
    Ascii,
    Binary,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Subtype {
    BitMap(Encoding),
    GrayMap(Encoding),
    PixMap(Encoding),
    ArbitraryMap,
    FloatGrayMap,
    FloatPixMap,
}

impl Subtype {
    pub fn encoding(&self) -> Encoding {
        match self {
            Subtype::BitMap(e) | Subtype::GrayMap(e) | Subtype::PixMap(e) => *e,
            Subtype::ArbitraryMap | Subtype::FloatGrayMap | Subtype::FloatPixMap => Encoding::Binary,
        }
    }

    pub fn is_bit_map(&self) -> bool {
        matches!(self, Subtype::BitMap(_))
    }

    pub fn is_gray_map(&self) -> bool {
        matches!(self, Subtype::GrayMap(_))
    }

    pub fn is_pix_map(&self) -> bool {
        matches!(self, Subtype::PixMap(_))
    }

    pub fn is_aritrary_map(&self) -> bool {
        matches!(self, Subtype::ArbitraryMap)
    }

    pub fn is_float_map(&self) -> bool {
        matches!(self, Subtype::FloatGrayMap | Subtype::FloatPixMap)
    }
}

/// Generalised PNM header.
pub struct Header {
    /// Subtype of the PNM file.
    pub subtype: Subtype,

    /// Width of the image.
    pub width: u32,

    /// Height of the image.
    pub height: u32,

    /// Maximum sample value of the image. Note that this value encodes the
    /// endianness of the image data, >0 => big endianness,
    /// <0 => little endianness. By default a PNM file is encoded with big
    /// endianness.
    pub max_val: f32,

    /// Number of channels in the image.
    pub n_channels: u32,

    /// Specifies the kind of the image (for PAM files)
    pub tuple_type: TupleType,
}

impl Header {
    /// Returns the number of bytes per sample (channel).
    pub fn bytes_per_sample(&self) -> usize {
        if self.subtype.is_float_map() {
            4
        } else if self.subtype.is_bit_map() {
            1
        } else {
            // Calculate required byte size according to its max value.
            (self.max_val.abs()).log(2.0).floor() as usize / 8 + 1
        }
    }

    /// Returns the number of bytes per pixel.
    pub fn bytes_per_pixel(&self) -> usize {
        self.bytes_per_sample() * self.n_channels as usize
    }
}

pub fn read_pnm_header(reader: &mut dyn Read) {
    todo!();
}
//
// pub fn read_pbm_header() {
//     todo!()
// }
// pub fn read_pgm_header() {}
// pub fn read_ppm_header() {}
// pub fn read_pfm_header() {}
// pub fn read_pam_header() {}
//
// pub fn read_pbm_data() {}
// pub fn read_pgm_data() {}
// pub fn read_ppm_data() {}
// pub fn read_pfm_data() {}
//
// pub fn write_pbm_file() {}
// pub fn write_pgm_file() {}
// pub fn write_ppm_file() {}
// pub fn write_pfm_file() {}