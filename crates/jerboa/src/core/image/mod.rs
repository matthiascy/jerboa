pub mod error;
pub mod codec;

#[non_exhaustive]
pub enum ImageFormat {
    /// Portable any-map format. See [Netpbm](https://en.wikipedia.org/wiki/Netpbm).
    Pnm,
}

pub enum ColorType {

}

pub struct Image {
    pub width: u32,

    pub height: u32,

    pub color_type: ColorType,
}