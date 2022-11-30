use core::ops::{Index, IndexMut};

/// 3-dimensional axis.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Axis3 {
    X = 0,
    Y = 1,
    Z = 2,
}

impl Axis3 {
    /// Returns the next axis following the x, y, z order.
    pub fn next_axis(&self) -> Axis3 {
        Axis3::from((*self as u32 + 1) % 3)
    }
}

macro_rules! impl_axis_from {
    ($($t:ty;)*) => {
        $(
            impl From<$t> for Axis3 {
                fn from(v: $t) -> Self {
                    assert!((0..3).contains(&v));
                    match v {
                        0 => Axis3::X,
                        1 => Axis3::Y,
                        2 => Axis3::Z,
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

impl<T> Index<Axis3> for [T] {
    type Output = T;

    fn index(&self, index: Axis3) -> &Self::Output {
        assert!(self.len() > index as usize);
        &self[index as usize]
    }
}

impl<T> IndexMut<Axis3> for [T] {
    fn index_mut(&mut self, index: Axis3) -> &mut Self::Output {
        assert!(self.len() > index as usize);
        &mut self[index as usize]
    }
}

// impl<T, const N: usize> Index<Axis3> for Vector<T, N> {
//     type Output = T;
//
//     fn index(&self, index: Axis3) -> &Self::Output {
//         &self[index as usize]
//     }
// }
//
// impl<T, const N: usize> IndexMut<Axis3> for Vector<T, N> {
//     fn index_mut(&mut self, index: Axis3) -> &mut Self::Output {
//         &mut self[index as usize]
//     }
// }

#[cfg(test)]
mod test {
    use super::Axis3;
    // use crate::vector::vec3;
    use quickcheck::quickcheck;

    quickcheck! {
        fn axis_indexing_check_array(a: u32, b: u32, c: u32) -> bool {
            let max = u32::MAX / 2;
            let arr = [a % max, b % max, c % max, 1, 2];

            arr[0] == arr[Axis3::X] && arr[1] == arr[Axis3::Y] && arr[2] == arr[Axis3::Z]
        }
    }

    quickcheck! {
        fn axis_indexing_check_array_mut(a: u32, b: u32, c: u32, d: u32) -> bool {
            let max = u32::MAX / 2;
            let mut arr = [a % max, b % max, c % max, d % max, 2];

            arr[Axis3::X] = a / max;
            arr[Axis3::Y] = b / max;
            arr[Axis3::Z] = c / max;

            arr[0] == arr[Axis3::X] && arr[1] == arr[Axis3::Y] && arr[2] == arr[Axis3::Z]
        }
    }

    // quickcheck! {
    //     fn axis_indexing_check_vec(a: u32, b: u32, c: u32) -> bool {
    //         let v = vec3(a, b, c);
    //
    //         a == v[Axis3::X] && b == v[Axis3::Y] && c == v[Axis3::Z]
    //     }
    // }

    // quickcheck! {
    //     fn axis_indexing_check_vec_mut(a: i32, b: i32, c: i32) -> bool {
    //         let max = i32::MAX / 2;
    //         let mut v = vec3(a % max, b % max, c % max);
    //
    //         v[Axis3::X] += 1;
    //         v[Axis3::Y] += 2;
    //         v[Axis3::Z] += 3;
    //
    //         v[0] == a % max + 1 && v[1] == b % max + 2 && v[2] == c % max + 3
    //     }
    // }
}
