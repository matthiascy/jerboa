use std::io;
use std::borrow::Cow;
use std::fmt::Display;
use crate::core::image::codec::pnm::TupleType;

#[derive(Debug)]
pub enum Error {
    UnknownMagicNumber,
    EmptyFile,
    Io(io::Error),
    UnknownTupleType,
    UnknownAttribute(Cow<'static, str>),
    UnmatchedTupleTypeAndPixelSize(TupleType, usize),
    MissingTupleType,
    InvalidAttributeFormat,
    InvalidSampleValue(usize),
    NotEnoughSamples{
        required: usize,
        provided: usize,
    },
    ParseError(Box<dyn ParseError>),
}

pub trait ParseError: std::error::Error + Send + Sync + 'static {}

impl ParseError for std::num::ParseIntError {}
impl ParseError for std::num::ParseFloatError {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::UnknownMagicNumber => write!(f, "unknown magic number"),
            Error::EmptyFile => write!(f, "empty file"),
            Error::Io(e) => write!(f, "io error: {}", e),
            Error::UnknownTupleType => write!(f, "unknown tuple type"),
            Error::UnknownAttribute(attr) => write!(f, "unknown attribute: {}", attr),
            Error::UnmatchedTupleTypeAndPixelSize(tuple_type, pixel_size) => {
                write!(f, "unmatched tuple type and pixel size: {:?} and {}", tuple_type, *pixel_size)
            }
            Error::MissingTupleType => write!(f, "missing tuple type"),
            Error::InvalidAttributeFormat => write!(f, "invalid attribute format"),
            Error::InvalidSampleValue(sample_value) => write!(f, "invalid sample value: {}", sample_value),
            Error::NotEnoughSamples { required, provided } => {
                write!(f, "not enough samples: {} required, {} provided", *required, *provided)
            }
            Error::ParseError(e) => write!(f, "parse error: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(err: std::num::ParseFloatError) -> Self {
        Error::ParseError(Box::new(err))
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::ParseError(Box::new(err))
    }
}