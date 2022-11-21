use std::ops::Index;
use crate::inner::{ArrayInner, Shape, ShapeStorage, Storage};

impl<D, S> Index<<S as Shape>::UnderlyingType> for ArrayInner<D, S>
    where
        D: Storage,
        S: Shape,
{
    type Output = D::Elem;

    fn index(&self, index: <S as Shape>::UnderlyingType) -> &Self::Output {
        // todo
        let indices = index.as_slice();
        let shape = self.shape().as_slice();
        let strides = self.strides().as_slice();
        assert!(indices.len() < shape.len(), "Index out of bounds");
        let idx: usize = indices.iter().zip(strides.iter()).map(|(i, s)| i * s).sum();
        assert!(, "index out of bounds");
        let idx = indices.iter().zip(shape.iter()).fold(0, |acc, (i, s)| acc * s + i);
        &self.data.as_slice()[idx]
    }
}

#[cfg(test)]
mod tests {
    use crate::inner::{ArrayInner, cs, DynSized, FixedSized};

    #[test]
    fn indexing() {
        let array0: ArrayInner<FixedSized<u32, 4>, cs!(2, 2)> = ArrayInner::new(FixedSized([0, 1, 2, 3]), [2, 2]);
        let array1: ArrayInner<DynSized<u32>, cs!(2, 3)> = ArrayInner::new(DynSized(vec![0, 1, 2, 3, 4, 5]), [2, 3]);

        assert_eq!(array0[[1, 1]], 3);
        assert_eq!(array1[[1, 0]], 2);
    }
}
