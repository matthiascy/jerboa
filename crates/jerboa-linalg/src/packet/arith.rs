use core::ops::{Add, Mul};
use std::ops::Div;

// todo: const fn
// todo: use references `&self` instead of `self`?

// impl<T, const N: usize> Packet<T, N> {
//     pub fn normalized(&mut self) where
//         T: Zero + Sqrt<Output = T>,
//         for <'e> &'e T: Mul<&'e T, Output = T>,
//         for<'e> &'e T: Div<T, Output = T>
//     {
//         *self = self.normalize();
//     }
// }

// // Macro to implement unary arithmetic trait for `Packet`.
// macro impl_unary_op_body($op:ident, $self:ident) {
//     {
//         let mut data: [T; N] = unsafe { core::mem::zeroed() };
//         let mut i = 0;
//         while i < N {
//             data[i] = $self.data[i].$op();
//             i += 1;
//         }
//         Packet::new(data)
//     }
// }

// macro_rules! impl_unary_arith_trait_body {
//     ($arith_op:ident, $self:ident) => {
//         {
//             let mut data: [T; N] = unsafe { core::mem::zeroed() };
//             let mut i = 0;
//             while i < N {
//                 data[i] = $self.data[i].$arith_op();
//                 i += 1;
//             }
//             Array::new(data)
//         }
//     };
// }

// macro_rules! impl_arith_trait_unary {
//     ($arith_trait:ident, $arith_op:ident) => {
//         impl<T, const N: usize> const $arith_trait for Array<T, N>
//         where
//             T: $arith_trait<Output = T>,
//         {
//             type Output = Array<T, N>;
//
//             fn $arith_op(self) -> Self::Output {
//                 impl_arith_trait_unary_body!($arith_op, self)
//             }
//         }
//
//         impl<'a, T, const N: usize> const $arith_trait for &'a Array<T, N>
//         where
//             &'a T: $arith_trait<Output = T>,
//         {
//             type Output = Array<T, N>;
//
//             fn $arith_op(self) -> Self::Output {
//                 impl_arith_trait_unary_body!($arith_op, self)
//             }
//         }
//     };
// }
//
// impl_arith_trait_unary!(Sqrt, sqrt);
// impl_arith_trait_unary!(Abs, abs);
// impl_arith_trait_unary!(Signum, signum);
// impl_arith_trait_unary!(Recip, recip);
//
// impl<T, const N: usize> const Sum for Packet<T, N>
// where T: Zero + Add<Output = T>
// {
//     type Output = T;
//
//     fn sum(self) -> T {
//         let mut sum = T::zero();
//         for val in self.data {
//             sum = sum + val;
//         }
//         sum
//     }
// }
//
// impl<'a, T, const N: usize> const Sum for &'a Packet<T, N>
//     where T: Zero + Add<&'a T, Output = T>
// {
//     type Output = T;
//
//     fn sum(self) -> T {
//         let mut sum = T::zero();
//         for val in &self.data {
//             sum = sum + val;
//         }
//         sum
//     }
// }
//
// impl<T, const N: usize> const DotProduct<Packet<T, N>> for Packet<T, N>
//     where T: Zero + Mul<T, Output = T>
// {
//     type Output = T;
//
//     fn dot(self, rhs: Packet<T, N>) -> Self::Output {
//         let mut result = T::zero();
//         for (lhs, rhs) in self.data.into_iter().zip(rhs.data.into_iter()) {
//             result = lhs * rhs;
//         }
//         result
//     }
// }
//
// impl<'a, T, const N: usize> const DotProduct<&'a Packet<T, N>> for Packet<T,
// N>     // where T: Zero + Mul<&'a T, Output = T>
//     where T: Zero + Mul<T, Output = T> + Copy
// {
//     type Output = T;
//
//     fn dot(self, rhs: &'a Packet<T, N>) -> Self::Output {
//         let mut result = T::zero();
//         for (lhs, rhs) in self.data.into_iter().zip(rhs.data.iter()) {
//             result = lhs * *rhs;
//         }
//         result
//     }
// }
//
// impl<'a, T, const N: usize> const DotProduct<Packet<T, N>> for &'a Packet<T,
// N>     where for<'e> &'e T: Mul<T, Output = T>,
//           T: Zero
// {
//     type Output = T;
//
//     fn dot(self, rhs: Packet<T, N>) -> Self::Output {
//         let mut result = T::zero();
//         for (lhs, rhs) in self.data.iter().zip(rhs.data.into_iter()) {
//             result = lhs * rhs;
//         }
//         result
//     }
// }
//
// impl<'a, 'b, T, const N: usize> const DotProduct<&'a Packet<T, N>> for &'b
// Packet<T, N>     where for<'e> &'e T: Mul<&'e T, Output = T>,
//           T: Zero
// {
//     type Output = T;
//
//     fn dot(self, rhs: &'a Packet<T, N>) -> Self::Output {
//         let mut result = T::zero();
//         for (lhs, rhs) in self.data.iter().zip(rhs.data.iter()) {
//             result = lhs * rhs;
//         }
//         result
//     }
// }
//
// impl<T, const N: usize> const Norm for Packet<T, N>
//     where T: Zero + Add<Output = T> + Mul<T, Output = T> + Sqrt<Output = T>
// {
//     type Output = T;
//
//     fn sqr_norm(self) -> Self::Output {
//         self.dot(self)
//     }
//
//     fn norm(self) -> Self::Output {
//         self.dot(self).sqrt()
//     }
// }
//
// impl<'a, T, const N: usize> const Norm for &'a Packet<T, N>
//     where T: Zero + Sqrt<Output = T>,
//           for<'e> &'e T: Mul<&'e T, Output = T>
// {
//     type Output = T;
//
//     fn sqr_norm(self) -> Self::Output {
//         self.dot(self)
//     }
//
//     fn norm(self) -> Self::Output {
//         self.dot(self).sqrt()
//     }
// }
//
// impl<T, const N: usize> const Normalization for Packet<T, N>
//     where T: Zero + Add<Output = T> + Mul<T, Output = T> + Sqrt<Output = T> +
// Div<T, Output = T> {
//     type Output = Packet<T, N>;
//
//     fn normalize(self) -> Self::Output {
//         let norm = self.norm();
//         self / norm
//     }
// }
//
// impl<'a, T, const N: usize> const Normalization for &'a Packet<T, N>
//     where T: Zero + Sqrt<Output = T>,
//           for<'e> &'e T: Mul<&'e T, Output = T>,
//           for<'e> &'e T: Div<T, Output = T>
// {
//     type Output = Packet<T, N>;
//
//     fn normalize(self) -> Self::Output {
//         let norm = self.norm();
//         self / norm
//     }
// }
