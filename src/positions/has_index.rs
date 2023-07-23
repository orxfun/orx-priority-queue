use super::heap_positions::{HeapPositions, HeapPositionsDecKey};
use crate::HasIndex;
use std::marker::PhantomData;

/// using usize::MAX as None
const NONE: usize = usize::MAX;

#[derive(Clone, Debug)]
pub struct HeapPositionsHasIndex<N>
where
    N: HasIndex,
{
    positions: Vec<usize>,
    ph: PhantomData<N>,
}
impl<N> HeapPositionsHasIndex<N>
where
    N: HasIndex,
{
    pub fn with_upper_limit(upper_limit: usize) -> Self {
        Self {
            positions: vec![NONE; upper_limit],
            ph: PhantomData,
        }
    }
}
impl<N> HeapPositions<N> for HeapPositionsHasIndex<N>
where
    N: HasIndex,
{
    fn clear(&mut self) {
        self.positions.iter_mut().for_each(|p| *p = NONE);
    }
    #[inline(always)]
    fn contains(&self, node: &N) -> bool {
        self.positions[node.index()] != NONE
    }
    fn position_of(&self, node: &N) -> Option<usize> {
        let position = self.positions[node.index()];
        if position == NONE {
            None
        } else {
            Some(position)
        }
    }
    fn insert(&mut self, node: &N, positions: usize) {
        debug_assert!(!self.contains(node), "re-inserting already added node");
        self.positions[node.index()] = positions;
    }
    fn remove(&mut self, node: &N) {
        debug_assert!(self.contains(node), "removing an absent node");
        self.positions[node.index()] = NONE;
    }
    fn update_position_of(&mut self, node: &N, position: usize) {
        debug_assert!(self.contains(node), "updating position of an absent node");
        self.positions[node.index()] = position;
    }
}

impl<N> HeapPositionsDecKey<N> for HeapPositionsHasIndex<N> where N: HasIndex {}
