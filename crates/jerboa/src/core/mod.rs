use std::ops::{Deref, DerefMut};

pub mod axis;
pub mod bounds;
pub mod error;
pub mod image;
pub mod num;
pub mod rounding;

#[derive(Debug, Copy, Clone)]
pub struct Vec1<T>([T; 1]);

#[derive(Debug, Copy, Clone)]
pub struct Vec2<T>([T; 2]);

#[derive(Debug, Copy, Clone)]
pub struct Vec3<T>([T; 3]);

#[derive(Debug, Copy, Clone)]
pub struct Vec4<T>([T; 4]);

macro_rules! impl_deref {
    ($($name:ident<T> $n:expr;)*) => {
        $(
            impl<T> Deref for $name<T> {
                type Target = [T; $n];
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl<T> DerefMut for $name<T> {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }

            impl<T: Copy + Default> Default for $name<T> {
                fn default() -> Self {
                    $name([T::default(); $n])
                }
            }
        )*
    };
}

impl_deref! {
    Vec1<T> 1; Vec2<T> 2; Vec3<T> 3; Vec4<T> 4;
}

#[derive(Debug, Copy, Clone)]
pub enum Angle<T> {
    Deg(T),
    Rad(T),
}
