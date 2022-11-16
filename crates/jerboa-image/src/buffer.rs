use std::{fs::File, io::BufWriter, path::Path};
use crate::iters::{Pixels, PixelsMut};
use crate::{Bit, Pixel};
use crate::codec::pnm;
use crate::codec::pnm::Encoding;
use crate::error::ImageError;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct PixelBuffer<P: Pixel> {
    width: u32,
    height: u32,
    samples: Vec<P::Subpixel>,
}

pub type PixelBufferRgb8 = PixelBuffer<[u8; 3]>;
pub type PixelBufferRgb16 = PixelBuffer<[u16; 3]>;
pub type PixelBufferRgb32f = PixelBuffer<[f32; 3]>;

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
            samples: vec![P::Subpixel::default(); (width * height) as usize * P::N_CHANNELS],
        }
    }

    pub fn from_samples(width: u32, height: u32, samples: Vec<P::Subpixel>) -> Self {
        PixelBuffer {
            width,
            height,
            samples,
        }
    }

    pub fn n_channels(&self) -> usize {
        P::N_CHANNELS
    }

    pub fn samples(&self) -> &[P::Subpixel] {
        &self.samples
    }

    pub fn pixel_at(&self, x: u32, y: u32) -> Option<&P> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let index = (y * self.width + x) as usize * P::N_CHANNELS;
        Option::from(P::from_slice(&self.samples[index..index + P::N_CHANNELS]))
    }

    pub fn pixel_at_mut(&mut self, x: u32, y: u32) -> Option<&mut P> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let index = (y * self.width + x) as usize * P::N_CHANNELS;
        Option::from(P::from_slice_mut(
            &mut self.samples[index..index + P::N_CHANNELS],
        ))
    }

    pub fn pixels(&self) -> Pixels<P> {
        Pixels::new(&self.samples, self.width)
    }

    pub fn pixels_mut(&mut self) -> PixelsMut<P> {
        PixelsMut::new(&mut self.samples, self.width)
    }
}

impl PixelBuffer<[Bit; 1]> {
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
        let header = pnm::Header {
            subtype: pnm::Subtype::BitMap(encoding),
            width: self.width,
            height: self.height,
            max_val: 1.0,
            n_channels: 1,
            tuple_type: pnm::TupleType::BlackAndWhiteBit,
        };
        pnm::write_pnm_to_stream::<Bit, _>(&mut writer, header, self.samples())
    }

    pub fn write_as_pam<P: AsRef<Path>>(&self, path: P) -> Result<(), ImageError> {
        self._write_as_pam(path.as_ref())
    }

    fn _write_as_pam(&self, path: &Path) -> Result<(), ImageError> {
        let path = path.with_extension("pam");
        let mut writer = BufWriter::new(File::create(path)?);
        let header = pnm::Header {
            subtype: pnm::Subtype::ArbitraryMap,
            width: self.width,
            height: self.height,
            max_val: 1.0,
            n_channels: 1,
            tuple_type: pnm::TupleType::BlackAndWhite,
        };
        let samples = self.samples().iter().map(|x| x.0).collect::<Vec<_>>();
        pnm::write_pnm_to_stream::<u8, _>(&mut writer, header, &samples)
    }
}

impl PixelBuffer<[u8; 1]> {
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
        let header = pnm::Header {
            subtype: pnm::Subtype::GrayMap(encoding),
            width: self.width,
            height: self.height,
            max_val: 255.0,
            n_channels: 1,
            tuple_type: pnm::TupleType::GrayScale,
        };
        pnm::write_pnm_to_stream::<u8, _>(&mut writer, header, self.samples())
    }
}

impl PixelBuffer<[f32; 1]> {
    pub fn write_as_pfm<P: AsRef<Path>>(&self, path: P) -> Result<(), ImageError> {
        self._write_as_pfm(path.as_ref())
    }

    fn _write_as_pfm(&self, path: &Path) -> Result<(), ImageError> {
        let path = path.with_extension("pfm");
        let mut writer = BufWriter::new(File::create(path)?);
        let header = pnm::Header {
            subtype: pnm::Subtype::FloatGrayMap,
            width: self.width,
            height: self.height,
            max_val: 1.0,
            n_channels: 1,
            tuple_type: pnm::TupleType::FloatGrayScale,
        };
        pnm::write_pnm_to_stream::<f32, _>(&mut writer, header, self.samples())
    }
}

impl PixelBuffer<[u8; 3]> {
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
        let header = pnm::Header {
            subtype: pnm::Subtype::PixMap(encoding),
            width: self.width,
            height: self.height,
            max_val: 255.0,
            n_channels: 3,
            tuple_type: pnm::TupleType::Rgb,
        };
        pnm::write_pnm_to_stream::<u8, _>(&mut writer, header, self.samples())
    }
}

impl PixelBuffer<[u16; 3]> {
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
        let header = pnm::Header {
            subtype: pnm::Subtype::PixMap(encoding),
            width: self.width,
            height: self.height,
            max_val: 65535.0,
            n_channels: 3,
            tuple_type: pnm::TupleType::Rgb,
        };
        pnm::write_pnm_to_stream::<u16, _>(&mut writer, header, self.samples())
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
                    let header = pnm::Header {
                        subtype: pnm::Subtype::ArbitraryMap,
                        width: self.width,
                        height: self.height,
                        max_val: <$s>::MAX as f32,
                        n_channels: $n,
                        tuple_type: $tupltype,
                    };
                    pnm::write_pnm_to_stream::<$s, _>(&mut writer, header, self.samples())
                }
            }
        )*
    };
}

impl PixelBuffer<[f32; 3]> {
    pub fn write_as_pfm<P: AsRef<Path>>(&self, path: P) -> Result<(), ImageError> {
        self._write_as_pfm(path.as_ref())
    }

    pub fn _write_as_pfm(&self, path: &Path) -> Result<(), ImageError> {
        let path = path.with_extension("pfm");
        let mut writer = BufWriter::new(File::create(path)?);
        let header = pnm::Header {
            subtype: pnm::Subtype::FloatPixMap,
            width: self.width,
            height: self.height,
            max_val: 1.0,
            n_channels: 3,
            tuple_type: pnm::TupleType::FloatRgb,
        };
        pnm::write_pnm_to_stream::<f32, _>(&mut writer, header, self.samples())
    }
}

impl_write_as_pam! {
    PixelBuffer<[u8; 1], 1, pnm::TupleType::GrayScale>;
    PixelBuffer<[u16; 1], 1, pnm::TupleType::GrayScale>;
    PixelBuffer<[u16; 2], 2, pnm::TupleType::GrayScaleAlpha>;
    PixelBuffer<[u8; 2], 2, pnm::TupleType::GrayScaleAlpha>;
    PixelBuffer<[u8; 3], 3, pnm::TupleType::Rgb>;
    PixelBuffer<[u16; 3], 3, pnm::TupleType::Rgb>;
    PixelBuffer<[u8; 4], 4, pnm::TupleType::RgbAlpha>;
    PixelBuffer<[u16; 4], 4, pnm::TupleType::RgbAlpha>;
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum ImageBuffer {
    Bitmap(PixelBuffer<[Bit; 1]>),
    Luma8(PixelBuffer<[u8; 1]>),
    LumaA8(PixelBuffer<[u8; 2]>),
    Luma16(PixelBuffer<[u16; 1]>),
    LumaA16(PixelBuffer<[u16; 2]>),
    Luma32F(PixelBuffer<[f32; 1]>),
    Rgb8(PixelBuffer<[u8; 3]>),
    RgbA8(PixelBuffer<[u8; 4]>),
    Rgb16(PixelBuffer<[u16; 3]>),
    RgbA16(PixelBuffer<[u16; 4]>),
    Rgb32F(PixelBuffer<[f32; 3]>),
}
