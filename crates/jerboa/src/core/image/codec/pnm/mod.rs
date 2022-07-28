mod decode;
mod encode;
mod error;

pub(crate) use decode::read_pnm_from_stream;
pub(crate) use encode::write_pnm_to_stream;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TupleType {
    /// Black is encoded as 0 and white as 1.
    BlackAndWhite,
    GrayScale,
    Rgb,
    BlackAndWhiteAlpha,
    GrayScaleAlpha,
    RgbAlpha,
    FloatGrayScale,
    FloatRgb,
    /// Black is encoded as 1 and white as 0 (PBM format).
    BlackAndWhiteBit,
}

impl TupleType {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            TupleType::BlackAndWhite => "BLACKANDWHITE",
            TupleType::GrayScale => "GRAYSCALE",
            TupleType::Rgb => "RGB",
            TupleType::BlackAndWhiteAlpha => "BLACKANDWHITE_ALPHA",
            TupleType::GrayScaleAlpha => "GRAYSCALE_ALPHA",
            TupleType::RgbAlpha => "RGB_ALPHA",
            TupleType::FloatGrayScale => "FLOAT_GRAYSCALE",
            TupleType::FloatRgb => "FLOAT_RGB",
            TupleType::BlackAndWhiteBit => "BLACKANDWHITE_BIT",
        }
    }
}

/// Sample encoding
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Encoding {
    Ascii,
    Binary,
}

impl Encoding {
    pub fn is_ascii(&self) -> bool {
        match self {
            Encoding::Ascii => true,
            Encoding::Binary => false,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Endian {
    Big,
    Little,
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
            Subtype::ArbitraryMap | Subtype::FloatGrayMap | Subtype::FloatPixMap => {
                Encoding::Binary
            }
        }
    }

    pub fn magic_number(&self) -> &'static str {
        match self {
            Subtype::BitMap(Encoding::Ascii) => "P1",
            Subtype::GrayMap(Encoding::Ascii) => "P2",
            Subtype::PixMap(Encoding::Ascii) => "P3",
            Subtype::BitMap(Encoding::Binary) => "P4",
            Subtype::GrayMap(Encoding::Binary) => "P5",
            Subtype::PixMap(Encoding::Binary) => "P6",
            Subtype::ArbitraryMap => "P7",
            Subtype::FloatGrayMap => "Pf",
            Subtype::FloatPixMap => "PF",
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
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Header {
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
