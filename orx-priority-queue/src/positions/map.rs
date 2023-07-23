use std::{collections::HashMap, hash::Hash};

use super::heap_positions::{HeapPositions, HeapPositionsDecKey};

#[derive(Clone, Debug)]
pub struct HeapPositionsMap<N>
where
    N: Eq + Hash + Clone,
{
    map: HashMap<N, usize>,
}
impl<N> Default for HeapPositionsMap<N>
where
    N: Eq + Hash + Clone,
{
    fn default() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}
impl<N> HeapPositionsMap<N>
where
    N: Eq + Hash + Clone,
{
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            map: HashMap::with_capacity(capacity),
        }
    }
}
impl<N> HeapPositions<N> for HeapPositionsMap<N>
where
    N: Eq + Hash + Clone,
{
    fn clear(&mut self) {
        self.map.clear();
    }
    #[inline(always)]
    fn contains(&self, node: &N) -> bool {
        self.map.contains_key(node)
    }
    fn position_of(&self, node: &N) -> Option<usize> {
        self.map.get(node).copied()
    }
    fn insert(&mut self, node: &N, position: usize) {
        debug_assert!(!self.contains(node), "re-inserting already added node");
        self.map.insert(node.clone(), position);
    }
    fn remove(&mut self, node: &N) {
        debug_assert!(self.contains(node), "removing an absent node");
        self.map.remove(node);
    }
    fn update_position_of(&mut self, node: &N, positions: usize) {
        *self.map.get_mut(node).unwrap() = positions;
    }
}

impl<N> HeapPositionsDecKey<N> for HeapPositionsMap<N> where N: Eq + Hash + Clone {}
