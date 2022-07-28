use crate::core::{
    image::{
        codec::pnm::{Encoding, Endian, Header, Subtype, TupleType},
        error::{DecodingError, EncodingError, ImageError, ParseError},
        Bit, ImageBuffer, ImageFormat, PixelBuffer, Sample,
    },
    Vec1, Vec2, Vec3, Vec4,
};
use std::io::{BufRead, Read};

use super::error::Error as PnmError;

impl From<std::num::ParseIntError> for ImageError {
    fn from(err: std::num::ParseIntError) -> Self {
        ImageError::Decoding(DecodingError {
            fmt: ImageFormat::Pnm,
            err: Box::new(ParseError::Int(err)),
        })
    }
}

impl From<std::num::ParseFloatError> for ImageError {
    fn from(err: std::num::ParseFloatError) -> Self {
        ImageError::Decoding(DecodingError {
            fmt: ImageFormat::Pnm,
            err: Box::new(ParseError::Float(err)),
        })
    }
}

fn map_io_error_decoding(err: std::io::Error) -> ImageError {
    ImageError::Decoding(DecodingError {
        fmt: ImageFormat::Pnm,
        err: Box::new(err),
    })
}

/// Represents a single value of a pixel in a PNM image.
pub trait PnmSample: Sample {
    /// Parse a single sample from an ASCII string then write it into the given buffer.
    fn decode_ascii(src: &str, dst: &mut [Self]) -> Result<(), ImageError>;

    /// Parse samples from a binary buffer (little endian).
    fn decode_le_bytes(src: &[u8], dst: &mut [Self]) -> Result<(), ImageError>;

    /// Parse samples from a binary buffer (big endian).
    fn decode_be_bytes(src: &[u8], dst: &mut [Self]) -> Result<(), ImageError>;

    /// Encode samples into a binary buffer (little endian).
    fn encode_le_bytes(src: &[Self], dst: &mut [u8]) -> Result<(), ImageError>;

    /// Encode samples into a binary buffer (big endian).
    fn encode_be_bytes(src: &[Self], dst: &mut [u8]) -> Result<(), ImageError>;
}

macro_rules! define_sample_types {
    ($(
        {$t:ty, $error:path};
    )*) => {
        $(
            impl PnmSample for $t {
                fn decode_ascii(src: &str, dst: &mut [Self]) -> Result<(), ImageError> {
                    dst[0] = src.parse::<Self>()?;
                    Ok(())
                }

                fn decode_le_bytes(src: &[u8], dst: &mut [Self]) -> Result<(), ImageError> {
                    if src.len() < dst.len() * Self::N_BYTES {
                        return Err(ImageError::Decoding(DecodingError::new(
                            ImageFormat::Pnm,
                            PnmError::NotEnoughSamples {
                                required: dst.len() * Self::N_BYTES,
                                provided: src.len(),
                            },
                        )));
                    }
                    src.chunks_exact(Self::N_BYTES).zip(dst.iter_mut()).for_each(|(chunk, dst)| {
                        *dst = Self::from_le_bytes(chunk[0..Self::N_BYTES].try_into().unwrap());
                    });
                    Ok(())
                }

                fn decode_be_bytes(src: &[u8], dst: &mut [Self]) -> Result<(), ImageError> {
                    if src.len() < dst.len() * Self::N_BYTES {
                        return Err(ImageError::Decoding(DecodingError::new(
                            ImageFormat::Pnm,
                            PnmError::NotEnoughSamples {
                                required: dst.len() * Self::N_BYTES,
                                provided: src.len(),
                            },
                        )));
                    }
                    src.chunks_exact(Self::N_BYTES).zip(dst.iter_mut()).for_each(|(chunk, dst)| {
                        *dst = Self::from_be_bytes(chunk[0..Self::N_BYTES].try_into().unwrap());
                    });
                    Ok(())
                }

                fn encode_le_bytes(src: &[Self], dst: &mut [u8]) -> Result<(), ImageError> {
                    if src.len() * Self::N_BYTES > dst.len(){
                        return Err(ImageError::Encoding(EncodingError::new(
                            ImageFormat::Pnm,
                            PnmError::NotEnoughBuffer {
                                required: src.len() * Self::N_BYTES,
                                provided: dst.len(),
                            },
                        )));
                    }
                    dst.chunks_exact_mut(Self::N_BYTES).zip(src.iter()).for_each(|(chunk, src)| {
                        chunk[0..Self::N_BYTES].copy_from_slice(&Self::to_le_bytes(*src));
                    });
                    Ok(())
                }

                fn encode_be_bytes(src: &[Self], dst: &mut [u8]) -> Result<(), ImageError> {
                    if src.len() * Self::N_BYTES > dst.len(){
                        return Err(ImageError::Encoding(EncodingError::new(
                            ImageFormat::Pnm,
                            PnmError::NotEnoughBuffer {
                                required: src.len() * Self::N_BYTES,
                                provided: dst.len(),
                            },
                        )));
                    }
                    dst.chunks_exact_mut(Self::N_BYTES).zip(src.iter()).for_each(|(chunk, src)| {
                        chunk[0..Self::N_BYTES].copy_from_slice(&Self::to_be_bytes(*src));
                    });
                    Ok(())
                }
            }
        )*
    };
}

impl PnmSample for Bit {
    fn decode_ascii(src: &str, dst: &mut [Self]) -> Result<(), ImageError> {
        let val = src.parse::<u8>()?;
        if val == 0 || val == 1 {
            dst[0] = Bit(1 - val); // PBM is inverted.
            Ok(())
        } else {
            Err(ImageError::Decoding(DecodingError::new(
                ImageFormat::Pnm,
                PnmError::InvalidSample(val as usize),
            )))
        }
    }

    fn decode_le_bytes(src: &[u8], dst: &mut [Self]) -> Result<(), ImageError> {
        // Each bit of the src byte will be encoded as a separate byte.
        if src.len() * 8 < dst.len() {
            return Err(ImageError::Decoding(DecodingError::new(
                ImageFormat::Pnm,
                PnmError::NotEnoughSamples {
                    required: dst.len(),
                    provided: src.len() * 8,
                },
            )));
        }
        // Required number of bits in total.
        let n_bits = dst.len();
        // Required number of bytes to decode required bits.
        let n_bytes = n_bits / 8 + 1;
        // Number of bits remaining in the last byte.
        let n_bits_in_last_byte = n_bits % 8;
        for i in 0..n_bytes - 1 {
            for j in 0..8 {
                // Start from the most significant bit.
                dst[i * 8 + j] = Bit((((src[i] << j) & 128) >> 7) ^ 1); // PBM is inverted.
            }
        }
        for i in 0..n_bits_in_last_byte {
            dst[(n_bytes - 1) * 8 + i] = Bit((((src[n_bytes - 1] << i) & 128) >> 7) ^ 1);
            // PBM is inverted.
        }
        Ok(())
    }

    fn decode_be_bytes(src: &[u8], dst: &mut [Self]) -> Result<(), ImageError> {
        Self::decode_le_bytes(src, dst)
    }

    fn encode_le_bytes(src: &[Self], dst: &mut [u8]) -> Result<(), ImageError> {
        if src.len() / 8 + 1 > dst.len() {
            return Err(ImageError::Encoding(EncodingError::new(
                ImageFormat::Pnm,
                PnmError::NotEnoughBuffer {
                    required: src.len() / 8 + 1,
                    provided: dst.len(),
                },
            )));
        }
        let n_bits = src.len();
        let n_bytes = n_bits / 8 + if n_bits % 8 == 0 { 0 } else { 1 };
        for i in 0..n_bytes {
            let mut byte = 0;
            for j in 0..8 {
                if i * 8 + j >= n_bits {
                    break;
                }
                let inverted = 1 - src[i * 8 + j].0;
                byte |= inverted << (7 - j);
            }
            dst[i] = byte;
        }
        Ok(())
    }

    fn encode_be_bytes(src: &[Self], dst: &mut [u8]) -> Result<(), ImageError> {
        Self::encode_le_bytes(src, dst)
    }
}

impl PnmSample for u8 {
    fn decode_ascii(src: &str, dst: &mut [Self]) -> Result<(), ImageError> {
        dst[0] = src.parse::<u8>()?;
        Ok(())
    }

    fn decode_le_bytes(src: &[u8], dst: &mut [Self]) -> Result<(), ImageError> {
        if src.len() < dst.len() * Self::N_BYTES {
            return Err(ImageError::Decoding(DecodingError::new(
                ImageFormat::Pnm,
                PnmError::NotEnoughSamples {
                    required: dst.len() * Self::N_BYTES,
                    provided: src.len(),
                },
            )));
        }
        dst.copy_from_slice(src);
        Ok(())
    }

    fn decode_be_bytes(src: &[u8], dst: &mut [Self]) -> Result<(), ImageError> {
        Self::decode_le_bytes(src, dst)
    }

    fn encode_le_bytes(src: &[Self], dst: &mut [u8]) -> Result<(), ImageError> {
        if src.len() > dst.len() {
            return Err(ImageError::Encoding(EncodingError::new(
                ImageFormat::Pnm,
                PnmError::NotEnoughBuffer {
                    required: dst.len(),
                    provided: src.len(),
                },
            )));
        }
        dst.copy_from_slice(src);
        Ok(())
    }

    fn encode_be_bytes(src: &[Self], dst: &mut [u8]) -> Result<(), ImageError> {
        Self::encode_le_bytes(src, dst)
    }
}

define_sample_types! {
    {u16, ParseError::Int};
    {f32, ParseError::Float};
}

impl Header {
    /// Returns the number of bytes per sample (channel).
    pub fn bytes_per_channel(&self) -> usize {
        if self.subtype.is_float_map() {
            4
        } else if self.subtype.is_bit_map() {
            1
        } else {
            // Calculate required byte size according to its max value.
            (self.max_val.abs()).log(2.0).floor() as usize / 8 + 1
        }
    }

    pub fn endian(&self) -> Endian {
        if self.max_val.is_sign_positive() {
            Endian::Big
        } else {
            Endian::Little
        }
    }

    pub fn decode<R: BufRead>(stream: &mut R) -> Result<Self, ImageError> {
        let magic_number = {
            let mut buf = [0u8; 2];
            stream.read_exact(&mut buf).map_err(map_io_error_decoding)?;
            buf
        };

        let subtype = match magic_number {
            [b'P', b'1'] => Subtype::BitMap(Encoding::Ascii),
            [b'P', b'2'] => Subtype::GrayMap(Encoding::Ascii),
            [b'P', b'3'] => Subtype::PixMap(Encoding::Ascii),
            [b'P', b'4'] => Subtype::BitMap(Encoding::Binary),
            [b'P', b'5'] => Subtype::GrayMap(Encoding::Binary),
            [b'P', b'6'] => Subtype::PixMap(Encoding::Binary),
            [b'P', b'7'] => Subtype::ArbitraryMap,
            [b'P', b'F'] => Subtype::FloatPixMap,
            [b'P', b'f'] => Subtype::FloatGrayMap,
            _ => {
                return Err(ImageError::Decoding(DecodingError::new(
                    ImageFormat::Pnm,
                    PnmError::UnknownMagicNumber(magic_number),
                )))
            }
        };

        match subtype {
            Subtype::ArbitraryMap => {
                let mut line = String::with_capacity(1024);
                let mut width = 0;
                let mut height = 0;
                let mut depth = 0;
                let mut max_val = 0.0;
                let mut tuple_type = None;

                'outer: loop {
                    line.clear();
                    stream.read_line(&mut line).map_err(map_io_error_decoding)?;

                    let trimmed = line.trim();

                    if trimmed.starts_with('#') || trimmed.is_empty() {
                        continue 'outer;
                    }

                    let attrib_and_values = trimmed.split_ascii_whitespace().collect::<Vec<_>>();
                    if attrib_and_values.is_empty() {
                        continue 'outer;
                    } else if attrib_and_values[0] == "ENDHDR" {
                        break 'outer;
                    } else if attrib_and_values.len() % 2 != 0 {
                        return Err(ImageError::Decoding(DecodingError::new(
                            ImageFormat::Pnm,
                            PnmError::InvalidAttributeFormat,
                        )));
                    }

                    for i in 0..attrib_and_values.len() / 2 {
                        match attrib_and_values[i * 2] {
                            "ENDHDR" => break 'outer,
                            "WIDTH" => {
                                width = attrib_and_values[i * 2 + 1].parse::<u32>()?;
                            }
                            "HEIGHT" => {
                                height = attrib_and_values[i * 2 + 1].parse::<u32>()?;
                            }
                            "DEPTH" => {
                                depth = attrib_and_values[i * 2 + 1].parse::<u32>()?;
                            }
                            "MAXVAL" => {
                                max_val = attrib_and_values[i * 2 + 1].parse::<f32>()?;
                            }
                            "TUPLTYPE" => {
                                tuple_type = match attrib_and_values[i * 2 + 1] {
                                    "BLACKANDWHITE" => Some(TupleType::BlackAndWhite),
                                    "GRAYSCALE" => Some(TupleType::GrayScale),
                                    "RGB" => Some(TupleType::Rgb),
                                    "BLACKANDWHITE_ALPHA" => Some(TupleType::BlackAndWhiteAlpha),
                                    "GRAYSCALE_ALPHA" => Some(TupleType::GrayScaleAlpha),
                                    "RGB_ALPHA" => Some(TupleType::RgbAlpha),
                                    _ => {
                                        return Err(ImageError::Decoding(DecodingError::new(
                                            ImageFormat::Pnm,
                                            PnmError::UnknownTupleType,
                                        )))
                                    }
                                };
                            }
                            attrib => {
                                return Err(ImageError::Decoding(DecodingError::new(
                                    ImageFormat::Pnm,
                                    PnmError::UnknownAttribute(attrib.to_string()),
                                )))
                            }
                        }
                    }
                }

                if tuple_type.is_none() {
                    return Err(ImageError::Decoding(DecodingError::new(
                        ImageFormat::Pnm,
                        PnmError::MissingTupleType,
                    )));
                }

                Ok(Header {
                    subtype: Subtype::ArbitraryMap,
                    width,
                    height,
                    max_val,
                    n_channels: depth,
                    tuple_type: tuple_type.unwrap(),
                })
            }
            _ => {
                let mut line = String::with_capacity(1024);
                let num_values_to_read = match subtype {
                    Subtype::BitMap(_) => 2,
                    Subtype::GrayMap(_)
                    | Subtype::PixMap(_)
                    | Subtype::FloatGrayMap
                    | Subtype::FloatPixMap => 3,
                    _ => unreachable!(),
                };
                let mut values_to_read = [0.0, 0.0, 1.0];
                let mut num_values_read = 0;

                while num_values_read < num_values_to_read {
                    line.clear();
                    stream.read_line(&mut line).map_err(|err| {
                        ImageError::Decoding(DecodingError::new(ImageFormat::Pnm, err))
                    })?;

                    let trimmed = line.trim();
                    // Skip if the line is a comment or empty
                    if trimmed.starts_with('#') || trimmed.is_empty() {
                        continue;
                    }

                    for value in trimmed
                        .split_ascii_whitespace()
                        .filter_map(|s| s.parse::<f32>().ok())
                    {
                        values_to_read[num_values_read] = value;
                        num_values_read += 1;
                    }
                }

                let (n_channels, tuple_type, max_val) = match subtype {
                    Subtype::BitMap(_) => (1, TupleType::BlackAndWhiteBit, 1.0),
                    Subtype::GrayMap(_) => (1, TupleType::GrayScale, values_to_read[2]),
                    Subtype::PixMap(_) => (3, TupleType::Rgb, values_to_read[2]),
                    Subtype::FloatGrayMap => (1, TupleType::FloatGrayScale, values_to_read[2]),
                    Subtype::FloatPixMap => (3, TupleType::FloatRgb, values_to_read[2]),
                    Subtype::ArbitraryMap => unreachable!(),
                };

                Ok(Header {
                    subtype,
                    width: values_to_read[0] as u32,
                    height: values_to_read[1] as u32,
                    max_val,
                    n_channels,
                    tuple_type,
                })
            }
        }
    }
}

pub(crate) fn read_pnm_from_stream<R: BufRead>(stream: &mut R) -> Result<ImageBuffer, ImageError> {
    let header = Header::decode(stream)?;
    let n_samples = (header.width * header.height * header.n_channels) as usize;
    let encoding = header.subtype.encoding();
    let endian = header.endian();
    match (header.tuple_type, header.bytes_per_channel()) {
        (TupleType::BlackAndWhiteBit, _) => {
            let samples = read_samples::<Bit, R>(stream, n_samples, encoding, None)?;
            Ok(ImageBuffer::Bitmap(PixelBuffer {
                width: header.width,
                height: header.height,
                pixels: samples
                    .chunks(1)
                    .map(|s| {
                        let val = if s[0] == Bit(0) { Bit(1) } else { Bit(0) };
                        Vec1([val])
                    })
                    .collect(),
            }))
        }
        (TupleType::BlackAndWhite, 1) | (TupleType::GrayScale, 1) => {
            let samples = read_samples::<u8, R>(stream, n_samples, encoding, Some(endian))?;
            Ok(ImageBuffer::Luma8(PixelBuffer {
                width: header.width,
                height: header.height,
                pixels: samples.chunks(1).map(|s| Vec1([s[0]])).collect(),
            }))
        }
        (TupleType::BlackAndWhiteAlpha, 1) => {
            let samples = read_samples::<u8, R>(stream, n_samples, encoding, Some(endian))?;
            Ok(ImageBuffer::LumaA8(PixelBuffer {
                width: header.width,
                height: header.height,
                pixels: samples.chunks(2).map(|s| Vec2([s[0], s[1]])).collect(),
            }))
        }
        (TupleType::GrayScale, 2) => {
            let samples = read_samples::<u16, R>(stream, n_samples, encoding, Some(endian))?;
            Ok(ImageBuffer::Luma16(PixelBuffer {
                width: header.width,
                height: header.height,
                pixels: samples.chunks(1).map(|s| Vec1([s[0]])).collect(),
            }))
        }
        (TupleType::GrayScaleAlpha, 1) => {
            let samples = read_samples::<u8, R>(stream, n_samples, encoding, Some(endian))?;
            Ok(ImageBuffer::LumaA8(PixelBuffer {
                width: header.width,
                height: header.height,
                pixels: samples.chunks(2).map(|s| Vec2([s[0], s[1]])).collect(),
            }))
        }
        (TupleType::GrayScaleAlpha, 2) => {
            let samples = read_samples::<u16, R>(stream, n_samples, encoding, Some(endian))?;
            Ok(ImageBuffer::LumaA16(PixelBuffer {
                width: header.width,
                height: header.height,
                pixels: samples.chunks(2).map(|s| Vec2([s[0], s[1]])).collect(),
            }))
        }
        (TupleType::Rgb, 1) => {
            let samples = read_samples::<u8, R>(stream, n_samples, encoding, Some(endian))?;
            Ok(ImageBuffer::Rgb8(PixelBuffer {
                width: header.width,
                height: header.height,
                pixels: samples
                    .chunks(3)
                    .map(|s| Vec3([s[0], s[1], s[2]]))
                    .collect(),
            }))
        }
        (TupleType::Rgb, 2) => {
            let samples = read_samples::<u16, R>(stream, n_samples, encoding, Some(endian))?;
            Ok(ImageBuffer::Rgb16(PixelBuffer {
                width: header.width,
                height: header.height,
                pixels: samples
                    .chunks(3)
                    .map(|s| Vec3([s[0], s[1], s[2]]))
                    .collect(),
            }))
        }
        (TupleType::RgbAlpha, 1) => {
            let samples = read_samples::<u8, R>(stream, n_samples, encoding, Some(endian))?;
            Ok(ImageBuffer::RgbA8(PixelBuffer {
                width: header.width,
                height: header.height,
                pixels: samples
                    .chunks(4)
                    .map(|s| Vec4([s[0], s[1], s[2], s[3]]))
                    .collect(),
            }))
        }
        (TupleType::RgbAlpha, 2) => {
            let samples = read_samples::<u16, R>(stream, n_samples, encoding, Some(endian))?;
            Ok(ImageBuffer::RgbA16(PixelBuffer {
                width: header.width,
                height: header.height,
                pixels: samples
                    .chunks(4)
                    .map(|s| Vec4([s[0], s[1], s[2], s[3]]))
                    .collect(),
            }))
        }
        (TupleType::FloatGrayScale, 4) => {
            let samples = read_samples::<f32, R>(stream, n_samples, encoding, Some(endian))?;
            Ok(ImageBuffer::Luma32F(PixelBuffer {
                width: header.width,
                height: header.height,
                pixels: samples.chunks(1).map(|s| Vec1([s[0]])).collect(),
            }))
        }
        (TupleType::FloatRgb, 4) => {
            let samples = read_samples::<f32, R>(stream, n_samples, encoding, Some(endian))?;
            Ok(ImageBuffer::Rgb32F(PixelBuffer {
                width: header.width,
                height: header.height,
                pixels: samples
                    .chunks(3)
                    .map(|s| Vec3([s[0], s[1], s[2]]))
                    .collect(),
            }))
        }
        _ => Err(ImageError::Decoding(DecodingError::new(
            ImageFormat::Pnm,
            PnmError::UnmatchedTupleTypeAndPixelSize(header.tuple_type, header.bytes_per_channel()),
        ))),
    }
}

fn read_samples<S: PnmSample, R: BufRead>(
    stream: &mut R,
    n_samples: usize,
    encoding: Encoding,
    endian: Option<Endian>,
) -> Result<Vec<S>, ImageError> {
    let mut samples = vec![S::default(); n_samples];
    match encoding {
        Encoding::Ascii => {
            read_samples_ascii::<R, S>(stream, &mut samples, n_samples)?;
        }
        Encoding::Binary => {
            let endian = endian.unwrap_or(Endian::Big);
            let n_bytes = n_samples * S::n_bytes();
            match endian {
                Endian::Big => read_samples_be_bytes::<R, S>(stream, &mut samples, n_bytes)?,
                Endian::Little => read_samples_le_bytes::<R, S>(stream, &mut samples, n_bytes)?,
            }
        }
    }
    Ok(samples)
}

fn read_samples_ascii<R: BufRead, S: PnmSample>(
    stream: &mut R,
    samples: &mut [S],
    n_samples: usize,
) -> Result<(), ImageError> {
    let mut i = 0;
    'outer: for line in stream.lines() {
        let line = line.map_err(map_io_error_decoding)?;

        if line.starts_with('#') || line.is_empty() {
            continue;
        }

        for val_str in line.trim().split_ascii_whitespace() {
            if i >= samples.len() {
                break 'outer;
            }
            S::decode_ascii(val_str, samples[i..].as_mut())?;
            i += 1;
        }
    }
    if i < n_samples - 1 {
        return Err(ImageError::Decoding(DecodingError::new(
            ImageFormat::Pnm,
            PnmError::NotEnoughSamples {
                required: n_samples,
                provided: i + 1,
            },
        )));
    }
    Ok(())
}

fn read_samples_le_bytes<R: BufRead, S: PnmSample>(
    stream: &mut R,
    samples: &mut [S],
    n_bytes: usize,
) -> Result<(), ImageError> {
    assert_eq!(samples.len() * S::N_BYTES, n_bytes);
    let mut bytes = vec![];
    stream
        .by_ref()
        .take(n_bytes as u64)
        .read_to_end(&mut bytes)
        .map_err(map_io_error_decoding)?;
    S::decode_le_bytes(&bytes, samples)?;
    Ok(())
}

fn read_samples_be_bytes<R: BufRead, S: PnmSample>(
    stream: &mut R,
    samples: &mut [S],
    n_bytes: usize,
) -> Result<(), ImageError> {
    assert_eq!(samples.len() * S::N_BYTES, n_bytes);
    let mut bytes = vec![];
    stream
        .by_ref()
        .take(n_bytes as u64)
        .read_to_end(&mut bytes)
        .map_err(map_io_error_decoding)?;
    S::decode_be_bytes(&bytes, samples)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{Bit, Encoding, Header, Subtype, TupleType};
    use quickcheck::{quickcheck, Arbitrary, Gen};
    use std::ops::{Deref, DerefMut};

    impl Arbitrary for Bit {
        fn arbitrary(g: &mut Gen) -> Self {
            let val = u8::arbitrary(g);
            Bit(val % 2)
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Array<T, const N: usize>(pub [T; N]);

    impl<T, const N: usize> Deref for Array<T, N> {
        type Target = [T; N];

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T, const N: usize> DerefMut for Array<T, N> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl<T: Arbitrary, const N: usize> Arbitrary for Array<T, N> {
        fn arbitrary(g: &mut Gen) -> Self {
            Array([(); N].map(|_| T::arbitrary(g)))
        }
    }

    macro_rules! check {
        (@ascii $name:ident {$t:ty, $w:expr, $h:expr, $c:expr, $max:expr, $subtype:expr, $tupltype:expr}) => {
            quickcheck! {
                fn $name(samples: Array<$t, {$w * $h * $c}>) -> bool {
                    let content = format!("{} {} {} {}\n{}", $subtype.magic_number(), $w, $h, $max, samples.map(|x| x.to_string()).join(" "));
                    let mut reader = std::io::Cursor::new(content.as_bytes());
                    let n_samples = ($w * $h * $c) as usize;
                    let parsed_header = Header::decode(&mut reader).unwrap();
                    let parsed_samples = super::read_samples::<$t, _>(&mut reader, n_samples, Encoding::Ascii, None).unwrap();
                    let mut success = parsed_header == Header {
                        subtype: $subtype,
                        width: $w,
                        height: $h,
                        max_val: $max as f32,
                        n_channels: $c,
                        tuple_type: $tupltype,
                    };
                    for (parsed, expected) in parsed_samples.iter().zip(samples.iter()) {
                        success &= parsed == expected;
                    }
                    success
                }
            }
        };
        (@binary $name:ident {$t:ty, $w:expr, $h:expr, $c:expr, $max:expr, $subtype:expr, $tupltype:expr}) => {
            quickcheck! {
                fn $name(samples: Array<$t, {$w * $h * $c}>) -> bool {
                    let mut content = format!("{} {} {} {}\n", $subtype.magic_number(), $w, $h, $max).into_bytes();
                    for s in samples.iter() {
                        let bytes = <$t>::to_be_bytes(*s);
                        let _ = &mut content.extend_from_slice(&bytes);
                    }
                    let mut reader = std::io::Cursor::new(content);
                    let parsed_header = Header::decode(&mut reader).unwrap();
                    let parsed_samples = super::read_samples::<$t, _>(&mut reader, samples.len(), Encoding::Binary, None).unwrap();
                    let mut success = parsed_header == Header {
                        subtype: $subtype,
                        width: $w,
                        height: $h,
                        max_val: $max as f32,
                        n_channels: $c,
                        tuple_type: $tupltype,
                    };
                    for (parsed, expected) in parsed_samples.iter().zip(samples.iter()) {
                        success &= *parsed == *expected;
                    }
                    success
                }
            }
        };
        (@binary_f32 $name:ident {$t:ty, $w:expr, $h:expr, $c:expr, $max:expr, $subtype:expr, $tupltype:expr}) => {
            quickcheck! {
                fn $name(arr: Array<$t, {$w * $h * $c}>) -> bool {
                    let samples = arr.map(|x| {
                        let x = x.abs();
                        if x.is_nan() {
                            0.0
                        } else {
                            x.clamp(0.0, ($max as f32).abs())
                        }
                    });
                    let mut content = format!("{} {} {} {}\n", $subtype.magic_number(), $w, $h, $max).into_bytes();
                    for s in samples.iter() {
                        let bytes = <$t>::to_be_bytes(*s);
                        let _ = &mut content.extend_from_slice(&bytes);
                    }
                    let mut reader = std::io::Cursor::new(content);
                    let parsed_header = Header::decode(&mut reader).unwrap();
                    let parsed_samples = super::read_samples::<$t, _>(&mut reader, samples.len(), Encoding::Binary, None).unwrap();
                    let mut success = parsed_header == Header {
                        subtype: $subtype,
                        width: $w,
                        height: $h,
                        max_val: $max as f32,
                        n_channels: $c,
                        tuple_type: $tupltype,
                    };
                    for (parsed, expected) in parsed_samples.iter().zip(samples.iter()) {
                        success &= *parsed == *expected;
                    }
                    success
                }
            }
        };
        (@ascii_bit $name:ident {$t:ty, $w:expr, $h:expr, $c:expr, $subtype:expr, $tupltype:expr}) => {
            quickcheck! {
                fn $name(samples: Array<$t, {$w * $h}>) -> bool {
                    let content = format!("{}\n{} {}\n{}", $subtype.magic_number(), $w, $h, samples.map(|x| x.to_string()).join(" "));
                    let mut reader = std::io::Cursor::new(content.as_bytes());
                    let n_samples = ($w * $h * $c) as usize;
                    let parsed_header = Header::decode(&mut reader).unwrap();
                    let parsed_samples = super::read_samples::<$t, _>(&mut reader, n_samples, Encoding::Ascii, None).unwrap();
                    let mut success = parsed_header == Header {
                        subtype: $subtype,
                        width: $w,
                        height: $h,
                        max_val: 1.0,
                        n_channels: $c,
                        tuple_type: $tupltype,
                    };
                    println!("is success: {}", success);
                    for (parsed, expected) in parsed_samples.iter().zip(samples.iter()) {
                        success &= parsed.0 == 1 - expected.0;
                    }
                    success
                }
            }
        };
        (@binary_bit $name:ident {$s:ty, $t:ty, $w:expr, $h:expr, $c:expr, $subtype:expr, $tupltype:expr}) => {
            quickcheck! {
                fn $name(arr: Array<$t, {(($w * $h) / 8) + 1}>) -> bool {
                    let mut content = format!("{} {} {}\n", $subtype.magic_number(), $w, $h).into_bytes();
                    let _ = &mut content.extend_from_slice(&arr.0);
                    let mut reader = std::io::Cursor::new(content);
                    let samples = arr.iter().flat_map(|x| {
                        let mut bits = [0u8; 8];
                        for i in 0..8 {
                            bits[i] = (((*x << i) & 128) >> 7) ^ 1;
                        }
                        bits
                    }).collect::<Vec<_>>();
                    let n_samples = ($w * $h * $c) as usize;
                    let parsed_header = Header::decode(&mut reader).unwrap();
                    let parsed_samples = super::read_samples::<$s, _>(&mut reader, n_samples, Encoding::Binary, None).unwrap();
                    let mut success = parsed_header == Header {
                        subtype: $subtype,
                        width: $w,
                        height: $h,
                        max_val: 1.0,
                        n_channels: $c,
                        tuple_type: $tupltype,
                    };
                    for (parsed, expected) in parsed_samples.iter().zip(samples.iter()) {
                        success &= parsed.0 == *expected;
                    }
                    success
                }
            }
        };
    }

    check!(@ascii_bit pbm_ascii {Bit, 6, 7, 1, Subtype::BitMap(Encoding::Ascii), TupleType::BlackAndWhiteBit});
    check!(@binary_bit pbm_binary {Bit, u8, 6, 9, 1, Subtype::BitMap(Encoding::Binary), TupleType::BlackAndWhiteBit});
    check!(@ascii pgm_ascii_u8 {u8, 7, 11, 1, 129u8, Subtype::GrayMap(Encoding::Ascii), TupleType::GrayScale});
    check!(@ascii pgm_ascii_u16 {u16, 4, 6, 1, u16::MAX, Subtype::GrayMap(Encoding::Ascii), TupleType::GrayScale});
    check!(@binary pgm_binary_u8 {u8, 10, 6, 1, 255u8, Subtype::GrayMap(Encoding::Binary), TupleType::GrayScale});
    check!(@binary pgm_binary_u16 {u16, 11, 4, 1, 255u8, Subtype::GrayMap(Encoding::Binary), TupleType::GrayScale});
    check!(@ascii ppm_ascii_u8 {u8, 8, 9, 3, 255u8, Subtype::PixMap(Encoding::Ascii), TupleType::Rgb});
    check!(@ascii ppm_ascii_u16 {u16, 7, 13, 3, u16::MAX, Subtype::PixMap(Encoding::Ascii), TupleType::Rgb});
    check!(@binary ppm_binary_u8 {u8, 8, 9, 3, u16::MAX, Subtype::PixMap(Encoding::Binary), TupleType::Rgb});
    check!(@binary ppm_binary_u16 {u16, 7, 13, 3, u16::MAX, Subtype::PixMap(Encoding::Binary), TupleType::Rgb});
    check!(@binary_f32 pfm_gray_be {f32, 11, 12, 1, 1.0, Subtype::FloatGrayMap, TupleType::FloatGrayScale});
    check!(@binary_f32 pfm_gray_le {f32, 11, 12, 1, -1.0, Subtype::FloatGrayMap, TupleType::FloatGrayScale});
    check!(@binary_f32 pfm_pix_be {f32, 11, 12, 3, 2.0, Subtype::FloatPixMap, TupleType::FloatRgb});
    check!(@binary_f32 pfm_pix_le {f32, 11, 12, 3, -3.0, Subtype::FloatPixMap, TupleType::FloatRgb});
}
