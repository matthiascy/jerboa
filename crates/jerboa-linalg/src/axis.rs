use crate::vector::Vector;
use std::ops::{Index, IndexMut};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Axis {
    X = 0,
    Y = 1,
    Z = 2,
}

impl Axis {
    /// Returns the next axis following the x, y, z order.
    pub fn next_axis(&self) -> Axis {
        Axis::from((*self as u32 + 1) % 3)
    }
}

macro_rules! impl_axis_from {
    ($($t:ty;)*) => {
        $(
            impl From<$t> for Axis {
                fn from(v: $t) -> Self {
                    assert!((0..3).contains(&v));
                    match v {
                        0 => Axis::X,
                        1 => Axis::Y,
                        2 => Axis::Z,
                        _ => unreachable!(),
                    }
                }
            }
        )*
    };
}

impl_axis_from! {
    i8; u8; i16; u16; i32; u32; i64; u64; usize; isize;
}

impl<T> Index<Axis> for [T] {
    type Output = T;

    fn index(&self, index: Axis) -> &Self::Output {
        assert!(self.len() > index as usize);
        &self[index as usize]
    }
}

impl<T> IndexMut<Axis> for [T] {
    fn index_mut(&mut self, index: Axis) -> &mut Self::Output {
        assert!(self.len() > index as usize);
        &mut self[index as usize]
    }
}

impl<T, const N: usize> Index<Axis> for Vector<T, N> {
    type Output = T;

    fn index(&self, index: Axis) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T, const N: usize> IndexMut<Axis> for Vector<T, N> {
    fn index_mut(&mut self, index: Axis) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

#[cfg(test)]
mod test {
    use super::Axis;
    use crate::vector::vec3;
    use quickcheck::quickcheck;

    quickcheck! {
        fn axis_indexing_check_array(a: u32, b: u32, c: u32) -> bool {
            let max = u32::MAX / 2;
            let arr = [a % max, b % max, c % max, 1, 2];

            arr[0] == arr[Axis::X] && arr[1] == arr[Axis::Y] && arr[2] == arr[Axis::Z]
        }
    }

    quickcheck! {
        fn axis_indexing_check_array_mut(a: u32, b: u32, c: u32, d: u32) -> bool {
            let max = u32::MAX / 2;
            let mut arr = [a % max, b % max, c % max, d % max, 2];

            arr[Axis::X] = a / max;
            arr[Axis::Y] = b / max;
            arr[Axis::Z] = c / max;

            arr[0] == arr[Axis::X] && arr[1] == arr[Axis::Y] && arr[2] == arr[Axis::Z]
        }
    }

    quickcheck! {
        fn axis_indexing_check_vec(a: u32, b: u32, c: u32) -> bool {
            let v = vec3(a, b, c);

            a == v[Axis::X] && b == v[Axis::Y] && c == v[Axis::Z]
        }
    }

    quickcheck! {
        fn axis_indexing_check_vec_mut(a: i32, b: i32, c: i32) -> bool {
            let max = i32::MAX / 2;
            let mut v = vec3(a % max, b % max, c % max);

            v[Axis::X] += 1;
            v[Axis::Y] += 2;
            v[Axis::Z] += 3;

            v[0] == a % max + 1 && v[1] == b % max + 2 && v[2] == c % max + 3
        }
    }
}
