use crate::core::image::Pixel;
use std::slice::{ChunksExact, ChunksExactMut};

/// Iterator over the pixels (reference) with coordinates.
/// Pixel coordinates are in the range [0, width - 1] x [0, height - 1],
/// where (0, 0) is the top-left pixel.
#[derive(Debug)]
pub struct Pixels<'a, P: Pixel + 'a>
where
    P::Subpixel: 'a,
{
    width: u32,
    count: usize,
    chunks: ChunksExact<'a, P::Subpixel>,
}

impl<'a, P: Pixel> Pixels<'a, P>
where
    P::Subpixel: 'a,
{
    pub fn new(samples: &'a [P::Subpixel], width: u32) -> Self {
        Pixels {
            width,
            count: 0,
            chunks: samples.chunks_exact(P::N_CHANNELS),
        }
    }
}

impl<'a, P: Pixel + 'a> Iterator for Pixels<'a, P>
where
    P::Subpixel: 'a,
{
    type Item = ((usize, usize), &'a P);

    #[inline(always)]
    fn next(&mut self) -> Option<((usize, usize), &'a P)> {
        let chunk = self.chunks.next()?;
        let i = self.count;
        self.count += 1;
        Some((
            (i % self.width as usize, i / self.width as usize),
            P::from_slice(chunk),
        ))
    }
}

impl<'a, P: Pixel + 'a> ExactSizeIterator for Pixels<'a, P>
where
    P::Subpixel: 'a,
{
    #[inline(always)]
    fn len(&self) -> usize {
        self.chunks.len()
    }
}

impl<'a, P: Pixel + 'a> DoubleEndedIterator for Pixels<'a, P>
where
    P::Subpixel: 'a,
{
    #[inline(always)]
    fn next_back(&mut self) -> Option<((usize, usize), &'a P)> {
        let chunk = self.chunks.next_back()?;
        let i = self.count;
        self.count += 1;
        Some((
            (i % self.width as usize, i / self.width as usize),
            P::from_slice(chunk),
        ))
    }
}

impl<P: Pixel> Clone for Pixels<'_, P> {
    fn clone(&self) -> Self {
        Pixels {
            width: self.width,
            count: self.count,
            chunks: self.chunks.clone(),
        }
    }
}

/// Iterator over the pixels (mutable reference) with pixel coordinates.
#[derive(Debug)]
pub struct PixelsMut<'a, P: Pixel + 'a>
where
    P::Subpixel: 'a,
{
    width: u32,
    count: usize,
    chunks: ChunksExactMut<'a, P::Subpixel>,
}

impl<'a, P: Pixel> PixelsMut<'a, P>
where
    P::Subpixel: 'a,
{
    pub fn new(samples: &'a mut [P::Subpixel], width: u32) -> Self {
        PixelsMut {
            width,
            count: 0,
            chunks: samples.chunks_exact_mut(P::N_CHANNELS),
        }
    }
}

impl<'a, P: Pixel + 'a> Iterator for PixelsMut<'a, P>
where
    P::Subpixel: 'a,
{
    type Item = ((usize, usize), &'a mut P);

    #[inline(always)]
    fn next(&mut self) -> Option<((usize, usize), &'a mut P)> {
        let chunk = self.chunks.next()?;
        let i = self.count;
        self.count += 1;
        Some((
            (i % self.width as usize, i / self.width as usize),
            P::from_slice_mut(chunk),
        ))
    }
}

impl<'a, P: Pixel + 'a> ExactSizeIterator for PixelsMut<'a, P>
where
    P::Subpixel: 'a,
{
    #[inline(always)]
    fn len(&self) -> usize {
        self.chunks.len()
    }
}

impl<'a, P: Pixel + 'a> DoubleEndedIterator for PixelsMut<'a, P>
where
    P::Subpixel: 'a,
{
    #[inline(always)]
    fn next_back(&mut self) -> Option<((usize, usize), &'a mut P)> {
        let chunk = self.chunks.next_back()?;
        let i = self.count;
        self.count += 1;
        Some((
            (i % self.width as usize, i / self.width as usize),
            P::from_slice_mut(chunk),
        ))
    }
}

/// Iterator over a block of pixels (reference) with coordinates.
#[derive(Debug)]
pub struct Blocks<'a, P: Pixel + 'a>
    where
        P::Subpixel: 'a,
{
    index: (u32, u32),
    chunks: ChunksExact<'a, P::Subpixel>,
}