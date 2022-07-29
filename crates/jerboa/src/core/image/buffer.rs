use crate::core::{
    image::{
        codec::{pnm, pnm::Encoding},
        error::ImageError,
        iters::{Pixels, PixelsMut},
        Bit, Pixel,
    },
    Vec1, Vec2, Vec3, Vec4,
};
use std::{fs::File, io::BufWriter, path::Path};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct PixelBuffer<P: Pixel> {
    width: u32,
    height: u32,
    samples: Vec<P::Subpixel>,
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

impl PixelBuffer<Vec1<f32>> {
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

impl PixelBuffer<Vec3<f32>> {
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
    PixelBuffer<Vec1<u8>, 1, pnm::TupleType::GrayScale>;
    PixelBuffer<Vec1<u16>, 1, pnm::TupleType::GrayScale>;
    PixelBuffer<Vec2<u16>, 2, pnm::TupleType::GrayScaleAlpha>;
    PixelBuffer<Vec2<u8>, 2, pnm::TupleType::GrayScaleAlpha>;
    PixelBuffer<Vec3<u8>, 3, pnm::TupleType::Rgb>;
    PixelBuffer<Vec3<u16>, 3, pnm::TupleType::Rgb>;
    PixelBuffer<Vec4<u8>, 4, pnm::TupleType::RgbAlpha>;
    PixelBuffer<Vec4<u16>, 4, pnm::TupleType::RgbAlpha>;
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
