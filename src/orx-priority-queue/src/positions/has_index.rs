use super::heap_positions::{HeapPositions, HeapPositionsDecKey};
use crate::HasIndex;
use alloc::vec;
use alloc::vec::Vec;
use core::marker::PhantomData;

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
    pub fn with_index_bound(index_bound: usize) -> Self {
        Self {
            positions: vec![NONE; index_bound],
            ph: PhantomData,
        }
    }
    pub(crate) fn index_bound(&self) -> usize {
        self.positions.len()
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

    fn is_valid<K>(&self, offset: usize, tree: &[(N, K)]) -> bool {
        let mut count = 0;
        for (node, &pos) in self.positions.iter().enumerate() {
            if pos != NONE {
                count += 1;
                if tree[pos].0.index() != node {
                    return false;
                }
            }
        }
        count == tree.len() - offset
    }
}

impl<N> HeapPositionsDecKey<N> for HeapPositionsHasIndex<N> where N: HasIndex {}
