pub(crate) trait HeapPositions<N>: Clone {
    fn contains(&self, node: &N) -> bool;

    fn position_of(&self, node: &N) -> Option<usize>;

    fn clear(&mut self);

    fn insert(&mut self, node: &N, position: usize);

    fn remove(&mut self, node: &N);

    fn update_position_of(&mut self, node: &N, position: usize);

    #[cfg(test)]
    fn is_valid<K>(&self, offset: usize, tree: &[(N, K)]) -> bool;
}

pub(crate) trait HeapPositionsDecKey<N>: HeapPositions<N> {}
