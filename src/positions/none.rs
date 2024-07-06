use super::heap_positions::HeapPositions;

#[derive(Clone, Debug)]
pub(crate) struct HeapPositionsNone;

impl<N> HeapPositions<N> for HeapPositionsNone {
    #[inline(always)]
    fn clear(&mut self) {}

    #[inline(always)]
    fn contains(&self, _node: &N) -> bool {
        false
    }

    #[inline(always)]
    fn position_of(&self, _node: &N) -> Option<usize> {
        None
    }

    #[inline(always)]
    fn insert(&mut self, _node: &N, _pos: usize) {}

    #[inline(always)]
    fn remove(&mut self, _node: &N) {}

    #[inline(always)]
    fn update_position_of(&mut self, _node: &N, _pos: usize) {}

    #[cfg(test)]
    fn is_valid<K>(&self, _offset: usize, _tree: &[(N, K)]) -> bool {
        true
    }
}
