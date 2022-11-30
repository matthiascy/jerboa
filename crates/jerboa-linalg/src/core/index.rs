// use crate::core::{ArrayCore, CShape, Data, ShapeStorage};
// use core::ops::Index;

// Array indexes are always written row-first.
// Indexing returns ArrayView.

// impl<D, S> Index<<S as CShape>::UnderlyingType> for ArrayCore<D, S>
// where
//     D: Data,
//     S: CShape,
// {
//     type Output = D::Elem;
//
//     fn index(&self, index: <S as CShape>::UnderlyingType) -> &Self::Output {
//         // todo
//         let indices = index.as_slice();
//         let shape = self.shape().as_slice();
//         let strides = self.strides().as_slice();
//         assert!(indices.len() < shape.len(), "Index out of bounds");
//         let idx: usize = indices.iter().zip(strides.iter()).map(|(i, s)| i *
// s).sum();         // assert!(idx < self.n_elems(), "index out of bounds");
//         // let idx = indices.iter().zip(shape.iter()).fold(0, |acc, (i, s)|
// acc * s +         // i);
//         &self.data.as_slice()[idx]
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use crate::core::{s, ArrayCore, DynSized, FixedSized};
//
//     #[test]
//     fn indexing() {
//         let array0: ArrayCore<FixedSized<u32, 4>, s!(2, 2)> = ArrayCore {
//             data: FixedSized([0, 1, 2, 3]),
//             shape: [2, 2],
//             strides: [2, 1],
//         };
//         let array1: ArrayCore<DynSized<u32>, s!(2, 3)> = ArrayCore {
//             data: DynSized(vec![0, 1, 2, 3, 4, 5]),
//             shape: [2, 3],
//             strides: [3, 1],
//         };
//
//         assert_eq!(array0[[1, 1]], 3);
//         assert_eq!(array1[[0, 0]], 0);
//     }
// }
