use crate::core::image::{
    codec::pnm::{decode::PnmSample, Encoding, Endian, Header, Subtype},
    error::{EncodingError, ImageError},
    ImageFormat,
};
use std::io::{Error, Write};

fn map_io_error_encoding(err: Error) -> ImageError {
    ImageError::Encoding(EncodingError::new(ImageFormat::Pnm, err))
}

impl Header {
    pub fn encode<W: Write>(&self, w: &mut W) -> Result<(), ImageError> {
        let header_string = match self.subtype {
            Subtype::BitMap(_) => {
                format!(
                    "{}\n {} {}\n",
                    self.subtype.magic_number(),
                    self.width,
                    self.height
                )
            }
            Subtype::GrayMap(_)
            | Subtype::FloatGrayMap
            | Subtype::FloatPixMap
            | Subtype::PixMap(_) => {
                format!(
                    "{}\n{} {}\n{}\n",
                    self.subtype.magic_number(),
                    self.width,
                    self.height,
                    self.max_val
                )
            }
            Subtype::ArbitraryMap => {
                format!(
                    "{}\n\
                WIDTH {}\n\
                HEIGHT {}\n\
                DEPTH {}\n\
                MAXVAL {}\n\
                TUPLTYPE {}\n\
                ENDHDR\n",
                    self.subtype.magic_number(),
                    self.width,
                    self.height,
                    self.n_channels,
                    self.max_val,
                    self.tuple_type.as_str()
                )
            }
        };
        w.write_all(header_string.as_bytes())
            .map_err(map_io_error_encoding)
    }
}

pub(crate) fn write_pnm_to_stream<S: PnmSample, W: Write>(
    stream: &mut W,
    header: Header,
    samples: &[S],
) -> Result<(), ImageError> {
    header.encode(stream)?;

    match header.subtype.encoding() {
        Encoding::Ascii => {
            for (i, sample) in samples.iter().enumerate() {
                write!(stream, "{}", sample).map_err(map_io_error_encoding)?;
                if i % header.width as usize == header.width as usize - 1 {
                    writeln!(stream).map_err(map_io_error_encoding)?;
                } else {
                    write!(stream, " ").map_err(map_io_error_encoding)?;
                }
            }
        }
        Encoding::Binary => {
            let n_samples = (header.width * header.height * header.n_channels) as usize;
            let n_bytes = if header.subtype.is_bit_map() {
                n_samples / 8 + if n_samples % 8 != 0 { 1 } else { 0 }
            } else {
                n_samples * S::N_BYTES
            };
            let mut bytes = vec![0u8; n_bytes];
            match header.endian() {
                Endian::Big => {
                    S::encode_be_bytes(samples, &mut bytes)?;
                }
                Endian::Little => {
                    S::encode_le_bytes(samples, &mut bytes)?;
                }
            }
            stream.write_all(&bytes).map_err(map_io_error_encoding)?;
        }
    }
    Ok(())
}
