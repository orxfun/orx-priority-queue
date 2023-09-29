use super::heap_positions::{HeapPositions, HeapPositionsDecKey};
use std::{collections::HashMap, hash::Hash};

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
    fn update_position_of(&mut self, node: &N, position: usize) {
        *self.map.get_mut(node).expect("node must exist") = position;
    }

    fn is_valid<K>(&self, offset: usize, tree: &[(N, K)]) -> bool {
        if self.map.len() != tree.len() - offset {
            false
        } else {
            for (node, &position) in &self.map {
                if let Some(tree_node) = tree.get(position) {
                    if node != &tree_node.0 {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            true
        }
    }
}

impl<N> HeapPositionsDecKey<N> for HeapPositionsMap<N> where N: Eq + Hash + Clone {}
