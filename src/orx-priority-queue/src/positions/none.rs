use super::heap_positions::HeapPositions;

#[derive(Clone, Debug)]
pub(crate) struct HeapPositionsNone;

impl<N> HeapPositions<N> for HeapPositionsNone {
    fn clear(&mut self) {}
    fn contains(&self, _node: &N) -> bool {
        false
    }
    fn position_of(&self, _node: &N) -> Option<usize> {
        None
    }
    fn insert(&mut self, _node: &N, _pos: usize) {}
    fn remove(&mut self, _node: &N) {}
    fn update_position_of(&mut self, _node: &N, _pos: usize) {}

    fn is_valid<K>(&self, _offset: usize, _tree: &[(N, K)]) -> bool {
        true
    }
}
