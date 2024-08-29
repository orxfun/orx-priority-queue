use super::heap_positions::{HeapPositions, HeapPositionsDecKey};

#[cfg(not(feature = "std"))]
use alloc::collections::BTreeMap;
#[cfg(feature = "std")]
use std::{collections::HashMap, hash::Hash};

#[cfg(not(feature = "std"))]
pub trait Index: Eq + Clone + Ord {}
#[cfg(not(feature = "std"))]
impl<T> Index for T where T: Eq + Clone + Ord {}
#[cfg(feature = "std")]
pub trait Index: Eq + Clone + Hash {}
#[cfg(feature = "std")]
impl<T> Index for T where T: Eq + Clone + Hash {}

#[cfg(not(feature = "std"))]
type Map<N> = BTreeMap<N, usize>;
#[cfg(feature = "std")]
type Map<N> = HashMap<N, usize>;

#[derive(Clone, Debug)]
pub struct HeapPositionsMap<N>
where
    N: Index,
{
    map: Map<N>,
}
impl<N> Default for HeapPositionsMap<N>
where
    N: Index,
{
    fn default() -> Self {
        Self { map: Map::new() }
    }
}
impl<N> HeapPositionsMap<N>
where
    N: Index,
{
    #[allow(unused)]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            #[cfg(not(feature = "std"))]
            map: Map::new(),
            #[cfg(feature = "std")]
            map: Map::with_capacity(capacity),
        }
    }
}
impl<N> HeapPositions<N> for HeapPositionsMap<N>
where
    N: Index,
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

    #[cfg(test)]
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

impl<N> HeapPositionsDecKey<N> for HeapPositionsMap<N> where N: Index {}
