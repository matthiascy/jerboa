use crate::{num::Num};
use super::Packet;
use core::ops::*;

impl_packet_unary_op!(Neg);

macro _impl_packet_binary_op_common_body($self:ident, $rhs:ident) {
    {
        let mut packet = $self;
        packet.data.iter_mut().zip($rhs.data.iter())
            .for_each(|(a, b)| *a = *a + *b);
        packet
    }
}

macro impl_packet_binary_op($($op_trait:ident)*) {
    $(
    )*
}

impl<T, const N: usize> Add for Packet<T, N>
    where T: Add<Output = T> + Copy
{
    type Output = Packet<T, N>;

    fn add(self, rhs: Packet<T, N>) -> Self::Output {
        let mut packet = self;
        packet.data.iter_mut().zip(rhs.data.iter())
            .for_each(|(a, b)| *a = *a + *b);
        packet
    }
}

impl<'a, T, const N: usize> Add<&'a Packet<T, N>> for Packet<T, N>
    where T: Add<Output = T> + Copy
{
    type Output = Packet<T, N>;

    fn add(self, rhs: &'a Packet<T, N>) -> Self::Output {
        let mut packet = self;
        packet.data.iter_mut().zip(rhs.data.iter())
            .for_each(|(a, b)| *a = *a + *b);
        packet
    }
}

impl<'a, 'b, T, const N: usize> Add<&'b Packet<T, N>> for &'b Packet<T, N>
    where T: Add<Output = T> + Copy
{
    type Output = Packet<T, N>;

    fn add(self, rhs: &'b Packet<T, N>) -> Self::Output {
        let mut data = unsafe { core::mem::zeroed::<[T; N]>() };
        data.iter_mut().zip(self.data.iter().zip(rhs.data.iter()))
            .for_each(|(a, (b, c))| *a = *b + *c);
        Packet::new(data)
    }
}

impl<'a, T, const N: usize> Add<Packet<T, N>> for &'a Packet<T, N>
    where T: Add<Output = T> + Copy
{
    type Output = Packet<T, N>;

    fn add(self, rhs: Packet<T, N>) -> Self::Output {
        let mut packet = rhs;
        packet.data.iter_mut().zip(self.data.iter())
            .for_each(|(a, b)| *a = *a + *b);
        packet
    }
}

/// Macro for implementing component wise binary operations for `Packet`.
macro impl_packet_binary_op_component_wise {
    ($op_trait:ident, $op:ident) => {
        impl<T: Num, const N: usize> $op_trait for Packet<T, N>
        where
            T: $op_trait<T, Output = T>,
        {
            type Output = Packet<T, N>;
            fn $op(self, rhs: Self) -> Self::Output {
                let mut data: [T; N] = unsafe { ::core::mem::zeroed::<[T; N]>() };
                for (i, (a, b)) in self.data.into_iter().zip(rhs.data.into_iter()).enumerate() {
                    data[i] = a.$op(b);
                }
                Packet::new(data)
            }
        }

        impl<'a, T: Num, const N: usize> $op_trait<&'a Packet<T, N>> for Packet<T, N>
        where
            T: $op_trait<T, Output = T>,
        {
            type Output = Packet<T, N>;

            fn $op(self, rhs: &'a Packet<T, N>) -> Self::Output {
                let mut data: [T; N] = unsafe { ::core::mem::zeroed::<[T; N]>() };
                for (i, (a, b)) in self.data.into_iter().zip(rhs.data.iter()).enumerate() {
                    data[i] = a.$op(*b);
                }
                Packet::new(data)
            }
        }

        impl<'a, T: Num, const N: usize> $op_trait<Packet<T, N>> for &'a Packet<T, N>
        where
            T: $op_trait<T, Output = T>,
        {
            type Output = Packet<T, N>;

            fn $op(self, rhs: Packet<T, N>) -> Self::Output {
                let mut data: [T; N] = unsafe { ::core::mem::zeroed::<[T; N]>() };
                for (i, (a, b)) in self.data.iter().zip(rhs.data.into_iter()).enumerate() {
                    data[i] = a.$op(b);
                }
                Packet::new(data)
            }
        }

        impl<'a, 'b, T: Num, const N: usize> $op_trait<&'a Packet<T, N>> for &'b Packet<T, N>
        where
            T: $op_trait<T, Output = T>,
        {
            type Output = Packet<T, N>;

            fn $op(self, rhs: &'a Packet<T, N>) -> Self::Output {
                let mut data: [T; N] = unsafe { ::core::mem::zeroed::<[T; N]>() };
                for (i, (a, b)) in self.data.iter().zip(rhs.data.iter()).enumerate() {
                    data[i] = a.$op(*b);
                }
                Packet::new(data)
            }
        }
    },
    (shift $op_trait:ident, $op:ident) => {
        impl<T: Num, U: Num, const N: usize> $op_trait<Packet<U, N>> for Packet<T, N>
        where
            T: $op_trait<U, Output = T>
        {
            type Output = Packet<T, N>;

            fn $op(self, rhs: Packet<U, N>) -> Self::Output {
                let mut data: [T; N] = unsafe { ::core::mem::zeroed() };
                for (i, (a, b)) in self.data.into_iter().zip(rhs.into_iter()).enumerate() {
                    data[i] = a.$op(b)
                }
                Packet::new(data)
            }
        }

        impl<'a, T: Num, U: Num, const N: usize> $op_trait<&'a Packet<U, N>> for Packet<T, N>
        where
            T: $op_trait<U, Output = T>
        {
            type Output = Packet<T, N>;

            fn $op(self, rhs: &'a Packet<U, N>) -> Self::Output {
                let mut data: [T; N] = unsafe { ::core::mem::zeroed() };
                for (i, (a, b)) in self.data.into_iter().zip(rhs.iter()).enumerate() {
                    data[i] = a.$op(*b)
                }
                Packet::new(data)
            }
        }

        impl<'a, T: Num, U: Num, const N: usize> $op_trait<Packet<U, N>> for &'a Packet<T, N>
        where
            T: $op_trait<U, Output = T>
        {
            type Output = Packet<T, N>;

            fn $op(self, rhs: Packet<U, N>) -> Self::Output {
                let mut data: [T; N] = unsafe { ::core::mem::zeroed() };
                for (i, (a, b)) in self.data.iter().zip(rhs.into_iter()).enumerate() {
                    data[i] = a.$op(b)
                }
                Packet::new(data)
            }
        }

        impl<'a, 'b, T: Num, U: Num, const N: usize> $op_trait<&'a Packet<U, N>> for &'b Packet<T, N>
        where
            T: $op_trait<U, Output = T>
        {
            type Output = Packet<T, N>;

            fn $op(self, rhs: &'a Packet<U, N>) -> Self::Output {
                let mut data: [T; N] = unsafe { ::core::mem::zeroed() };
                for (i, (a, b)) in self.data.iter().zip(rhs.iter()).enumerate() {
                    data[i] = a.$op(*b)
                }
                Packet::new(data)
            }
        }
    },
    (asgmt $op_trait:ident, $op:ident) => {
        impl<T: Num, const N: usize> $op_trait for Packet<T, N>
        where
            T: $op_trait,
        {
            fn $op(&mut self, rhs: Self) {
                for (i, x) in rhs.into_iter().enumerate() {
                    self.data[i].$op(x);
                }
            }
        }

        impl<'a, T: Num, const N: usize> $op_trait<&'a Packet<T, N>> for Packet<T, N>
        where
            T: $op_trait,
        {
            fn $op(&mut self, rhs: &'a Packet<T, N>) {
                for i in 0..N {
                    self.data[i].$op(rhs[i]);
                }
            }
        }
    }
}

macro impl_packet_unary_op($($op_trait:ident),*) {
    $(
        paste::paste! {
            impl<T, const N: usize> $op_trait for Packet<T, N>
            where T: $op_trait<Output = T> + Copy
            {
                type Output = Packet<T, N>;

                fn [<$op_trait:lower>](self) -> Self::Output {
                    let mut packet = self;
                    packet.data.iter_mut().for_each(|x| *x = -(*x));
                    packet
                }
            }

            impl<T, const N: usize> $op_trait for &Packet<T, N>
            where T: $op_trait<Output = T> + Copy
            {
                type Output = Packet<T, N>;

                fn [<$op_trait:lower>](self) -> Self::Output {
                    let mut data = unsafe { core::mem::zeroed::<[T; N]>() };
                    data.iter_mut().zip(self.data.iter()).for_each(|(a, b)| *a = -*b);
                    Packet::new(data)
                }
            }
        }
    )*
}

macro impl_packet_op_scalar_body($self:ident) {
    type Output = Packet<T, N>;

    fn $op($self, rhs: T) -> Self::Output {
        Packet::new(self.data.map(|x| x.$op(rhs)))
    }
}

macro impl_packet_op_scalar {
    ($op_trait:ident, $op:ident) => {
        impl<T: Num, const N: usize> $op_trait<T> for Packet<T, N>
        where
            T: $op_trait<Output = T>
        {
            impl_packet_op_scalar_body!(self);
        }

        impl<'a, T: Num, const N: usize> $op_trait<&'a T> for Packet<T, N>
        where
            T: $op_trait<Output = T>
        {
            impl_packet_op_scalar_body!(self);
        }

        impl<'a, T: Num, const N: usize> $op_trait<T> for &'a Packet<T, N>
        where
            T: $op_trait<Output = T>
        {
            impl_packet_op_scalar_body!(self);
        }

        impl<'a, 'b, T: Num, const N: usize> $op_trait<&'a T> for &'b Packet<T, N>
        where
            T: $op_trait<Output = T>
        {
            impl_packet_op_scalar_body!(self);
        }
    },
    (asgmt $op_trait:ident, $op:ident) => {
        impl<T: Num, const N: usize> $op_trait<T> for Packet<T, N>
        where
            T: $op_trait<T>,
        {
            fn $op(&mut self, rhs: T) {
                for x in &mut self.data {
                    x.$op(rhs);
                }
            }
        }

        impl<'a, T: Num, const N: usize> $op_trait<&'a T> for Packet<T, N>
        where
            T: $op_trait<T>,
        {
            fn $op(&mut self, rhs: &'a T) {
                for x in &mut self.data {
                    x.$op(rhs);
                }
            }
        }
    }
}

// impl_packet_binary_op_component_wise!(Add, add);
impl_packet_binary_op_component_wise!(Sub, sub);
impl_packet_binary_op_component_wise!(Mul, mul);
impl_packet_binary_op_component_wise!(Div, div);
impl_packet_binary_op_component_wise!(Rem, rem);
impl_packet_binary_op_component_wise!(BitOr, bitor);
impl_packet_binary_op_component_wise!(BitAnd, bitand);
impl_packet_binary_op_component_wise!(BitXor, bitxor);
impl_packet_binary_op_component_wise!(shift Shl, shl);
impl_packet_binary_op_component_wise!(shift Shr, shr);
impl_packet_binary_op_component_wise!(asgmt AddAssign, add_assign);
impl_packet_binary_op_component_wise!(asgmt SubAssign, sub_assign);
impl_packet_binary_op_component_wise!(asgmt MulAssign, mul_assign);
impl_packet_binary_op_component_wise!(asgmt DivAssign, div_assign);
impl_packet_binary_op_component_wise!(asgmt RemAssign, rem_assign);
impl_packet_binary_op_component_wise!(asgmt BitOrAssign, bitor_assign);
impl_packet_binary_op_component_wise!(asgmt BitAndAssign, bitand_assign);
impl_packet_binary_op_component_wise!(asgmt BitXorAssign, bitxor_assign);

// impl_packet_op_scalar!(Add, add);
// impl_packet_op_scalar!(Sub, sub);
// impl_packet_op_scalar!(Mul, mul);
// impl_packet_op_scalar!(Div, div);
// impl_packet_op_scalar!(Rem, rem);

impl_packet_op_scalar!(asgmt AddAssign, add_assign);
impl_packet_op_scalar!(asgmt SubAssign, sub_assign);
impl_packet_op_scalar!(asgmt MulAssign, mul_assign);
impl_packet_op_scalar!(asgmt DivAssign, div_assign);
impl_packet_op_scalar!(asgmt RemAssign, rem_assign);
//
// #[cfg(test)]
// mod array_ops_tests {
//     use crate::{packet::Packet, traits::Sqrt};
//
//     #[test]
//     fn addition_scalar() {
//         let a = Packet::new([1, 2, 3, 4]);
//         let b = Packet::new([5, 6, 7, 8]);
//         let c = a + b;
//         assert_eq!(c.data, [6, 8, 10, 12]);
//     }
//
//     #[test]
//     fn addition_elem() {
//         let a = Packet::new([1, 2, 3, 4]);
//         let b = 10;
//         let c = &b;
//         let d = a + c;
//
//         let e = Packet::new([Packet::new([1, 2, 3]); 3]);
//         let f = Packet::new([5, 6, 7]);
//         let g = e + f;
//         assert_eq!(d.data, [11, 12, 13, 14]);
//         assert_eq!(g.data, [Packet::new([6, 8, 10]); 3]);
//     }
//
//     #[test]
//     fn addition_nested() {
//         let a = Packet::new([1, 3]);
//         let b = Packet::new([a; 3]);
//         let c = Packet::new([a; 3]);
//         let c = b + c;
//         assert_eq!(c.data, [a + a, a + a, a + a]);
//     }
//
//     #[test]
//     fn addition_assign_scalar() {
//         let mut a = Packet::new([1, 2, 3, 4]);
//         let b = Packet::new([5, 6, 7, 8]);
//         a += b;
//         assert_eq!(a.data, [6, 8, 10, 12]);
//     }
//
//     #[test]
//     fn addition_assign_nested() {
//         let mut a = Packet::new([Packet::new([1, 2, 3, 4]); 4]);
//         let b = Packet::new([
//             Packet::new([5, 6, 7, 8]),
//             Packet::new([9, 10, 11, 12]),
//             Packet::new([13, 14, 15, 16]),
//             Packet::new([17, 18, 19, 20]),
//         ]);
//         a += b;
//         assert_eq!(
//             a.data,
//             [
//                 Packet::new([6, 8, 10, 12]),
//                 Packet::new([10, 12, 14, 16]),
//                 Packet::new([14, 16, 18, 20]),
//                 Packet::new([18, 20, 22, 24]),
//             ]
//         );
//     }
//
//     #[test]
//     fn addition_assign_elem() {
//         let mut a = Packet::new([1, 2, 3, 4]);
//         let b = 10;
//         a += &b;
//         assert_eq!(a.data, [11, 12, 13, 14]);
//     }
//
//     #[test]
//     fn subtraction_scalar() {
//         let a = Packet::new([0.3, 0.5, 3.1, 4.2]);
//         let b = Packet::new([1.8, 2.5, 7.2, 8.0]);
//         let c = a - b;
//         assert_eq!(c.data, [-1.5, -2.0, -4.1, -3.8]);
//     }
//
//     #[test]
//     fn subtraction_assign() {
//         let mut a = Packet::new([0.3, 0.5, 3.1, 4.2]);
//         let b = Packet::new([1.8, 2.5, 7.2, 8.0]);
//         a -= b;
//         assert_eq!(a.data, [-1.5, -2.0, -4.1, -3.8]);
//     }
//
//     #[test]
//     fn subtraction_nested() {
//         let a = Packet::new([1.6, 3.2]);
//         let b = Packet::new([1.2, 0.2]);
//         let c = Packet::new([a; 3]);
//         let d = Packet::new([b; 3]);
//         let e = c - d;
//         assert_eq!(e.data, [a - b, a - b, a - b]);
//     }
//
//     #[test]
//     fn shift_left() {
//         let a = Packet::new([1, 2, 3u32]);
//         let b = Packet::new([2, 2, 4]);
//         let c = a << b;
//         assert_eq!(c.data, [4, 8, 48]);
//         let d = Packet::new([Packet::new([2, 2, 2]); 3]);
//         let e = Packet::new([Packet::new([2, 2, 2]); 3]);
//         let f = d << e;
//         assert_eq!(f.data, [Packet::new([8, 8, 8]); 3]);
//     }
//
//     #[test]
//     fn component_wise_mul() {
//         let a = Packet::new([1, 2, 3, 4]);
//         let b = Packet::new([5, 6, 7, 8]);
//         let c = a * b;
//         assert_eq!(c.data, [5, 12, 21, 32]);
//     }
//
//     #[test]
//     fn component_wise_mul_nested() {
//         let a = Packet::new([Packet::new([1, 2, 3]); 3]);
//         let b = Packet::new([Packet::new([5, 6, 7]); 3]);
//         let c = a * b;
//         assert_eq!(c.data, [Packet::new([5, 12, 21]); 3]);
//
//         let a = Packet::new([Packet::new([Packet::new([1, 2, 3]); 3]); 3]);
//         let b = Packet::new([Packet::new([Packet::new([1, 2, 3]); 3]); 3]);
//         let c = a * b;
//         assert_eq!(c.data, [Packet::new([Packet::new([1, 4, 9]); 3]); 3]);
//     }
//
//     #[test]
//     fn element_div() {
//         let a = Packet::new([Packet::new([1, 2]); 3]);
//         let b = Packet::new([1, 2]);
//         let c = a / b;
//         assert_eq!(c.data, [Packet::new([1, 1]); 3]);
//     }
//
//     #[test]
//     fn scalar_div() {
//         let a = Packet::new([1, 2, 3, 4]);
//         let b = 10;
//         let c = &a / b;
//         assert_eq!(c.data, [10, 20, 30, 40]);
//     }
//
//     #[test]
//     fn scalar_mul() {
//         let a = Packet::new([1, 2, 3, 4, 5]);
//         let b = 10;
//         let c = a * b;
//
//         let d = Packet::new([Packet::new([1, 2, 3]); 3]);
//         assert_eq!(c.data, [10, 20, 30, 40, 50]);
//     }
//
//     #[test]
//     fn element_mul() {
//         let a = Packet::new([Packet::new([1, 2, 3]); 3]);
//         let b = Packet::new([10; 3]);
//         let c = a * b;
//         assert_eq!(c.data, [Packet::new([10, 20, 30]); 3]);
//     }
//
//     #[test]
//     fn negation() {
//         let a = Packet::new([Packet::new([1, 2, 3, 4]); 2]);
//         let b = -a;
//         assert_eq!(b.data, [Packet::new([-1, -2, -3, -4]); 2]);
//     }
//
//     #[test]
//     fn sqrt() {
//         let a = Packet::new([1.0, 4.0, 9.0]);
//         let b = a.sqrt();
//         assert_eq!(b.data, [1.0, 2.0, 3.0]);
//
//         let c = Packet::new([Packet::new([1.0, 4.0, 9.0]); 3]);
//         let d = c.sqrt();
//         assert_eq!(d.data, [Packet::new([1.0, 2.0, 3.0]); 3]);
//     }
// }
