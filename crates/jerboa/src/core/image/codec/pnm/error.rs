use crate::core::image::codec::pnm::TupleType;
use std::fmt::Display;

#[derive(Debug)]
pub(crate) enum Error {
    UnknownMagicNumber([u8; 2]),
    UnknownTupleType,
    UnknownAttribute(String),
    UnmatchedTupleTypeAndPixelSize(TupleType, usize),
    MissingTupleType,
    InvalidAttributeFormat,
    InvalidSample(usize),
    NotEnoughSamples { required: usize, provided: usize },
    NotEnoughBuffer { required: usize, provided: usize },
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::UnknownMagicNumber(buf) => write!(f, "unknown magic number {:?}", buf),
            Error::UnknownTupleType => write!(f, "unknown tuple type"),
            Error::UnknownAttribute(attr) => write!(f, "unknown attribute: {}", attr),
            Error::UnmatchedTupleTypeAndPixelSize(tuple_type, pixel_size) => {
                write!(
                    f,
                    "unmatched tuple type and pixel size: {:?} and {}",
                    tuple_type, *pixel_size
                )
            }
            Error::MissingTupleType => write!(f, "missing tuple type"),
            Error::InvalidAttributeFormat => write!(f, "invalid attribute format"),
            Error::InvalidSample(sample_value) => {
                write!(f, "invalid sample value: {}", sample_value)
            }
            Error::NotEnoughSamples { required, provided } => {
                write!(
                    f,
                    "not enough samples: {} required, {} provided",
                    *required, *provided
                )
            }
            Error::NotEnoughBuffer { required, provided } => {
                write!(
                    f,
                    "not enough space: {} bytes required, {} bytes provided",
                    *required, *provided
                )
            }
        }
    }
}

impl std::error::Error for Error {}
