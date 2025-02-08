use crate::{
    dary::daryheap_const_helpers::offset, positions::map::Index, DaryHeap, DaryHeapOfIndices,
    DaryHeapWithMap, HasIndex,
};

// DaryHeap

impl<'a, N, K, const D: usize> IntoIterator for &'a DaryHeap<N, K, D>
where
    N: Clone,
    K: PartialOrd + Clone,
{
    type Item = &'a (N, K);

    type IntoIter = alloc::slice::Iter<'a, (N, K)>;

    fn into_iter(self) -> Self::IntoIter {
        self.heap().as_slice().into_iter()
    }
}

impl<N, K, const D: usize> IntoIterator for DaryHeap<N, K, D>
where
    N: Clone,
    K: PartialOrd + Clone,
{
    type Item = (N, K);

    type IntoIter = core::iter::Skip<alloc::vec::IntoIter<(N, K)>>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_heap().into_tree().into_iter().skip(offset::<D>())
    }
}

// DaryHeapOfIndices

impl<'a, N, K, const D: usize> IntoIterator for &'a DaryHeapOfIndices<N, K, D>
where
    N: HasIndex,
    K: PartialOrd + Clone,
{
    type Item = &'a (N, K);

    type IntoIter = alloc::slice::Iter<'a, (N, K)>;

    fn into_iter(self) -> Self::IntoIter {
        self.heap().as_slice().into_iter()
    }
}

impl<N, K, const D: usize> IntoIterator for DaryHeapOfIndices<N, K, D>
where
    N: HasIndex,
    K: PartialOrd + Clone,
{
    type Item = (N, K);

    type IntoIter = core::iter::Skip<alloc::vec::IntoIter<(N, K)>>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_heap().into_tree().into_iter().skip(offset::<D>())
    }
}

// DaryHeapWithMap

impl<'a, N, K, const D: usize> IntoIterator for &'a DaryHeapWithMap<N, K, D>
where
    N: Index,
    K: PartialOrd + Clone,
{
    type Item = &'a (N, K);

    type IntoIter = alloc::slice::Iter<'a, (N, K)>;

    fn into_iter(self) -> Self::IntoIter {
        self.heap().as_slice().into_iter()
    }
}

impl<N, K, const D: usize> IntoIterator for DaryHeapWithMap<N, K, D>
where
    N: Index,
    K: PartialOrd + Clone,
{
    type Item = (N, K);

    type IntoIter = core::iter::Skip<alloc::vec::IntoIter<(N, K)>>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_heap().into_tree().into_iter().skip(offset::<D>())
    }
}
