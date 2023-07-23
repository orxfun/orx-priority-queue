use super::heap::Heap;
use crate::{positions::map::HeapPositionsMap, PriorityQueue, PriorityQueueDecKey};
use std::hash::Hash;

#[derive(Clone)]
pub struct DaryHeapWithMap<N, K, const D: usize>
where
    N: Eq + Hash + Clone,
    K: PartialOrd + Clone,
{
    heap: Heap<N, K, HeapPositionsMap<N>, D>,
}

impl<N, K, const D: usize> Default for DaryHeapWithMap<N, K, D>
where
    N: Eq + Hash + Clone,
    K: PartialOrd + Clone,
{
    fn default() -> Self {
        Self {
            heap: Heap {
                tree: Vec::new(),
                positions: HeapPositionsMap::default(),
            },
        }
    }
}
impl<N, K, const D: usize> DaryHeapWithMap<N, K, D>
where
    N: Eq + Hash + Clone,
    K: PartialOrd + Clone,
{
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            heap: Heap {
                tree: Vec::with_capacity(capacity),
                positions: HeapPositionsMap::with_capacity(capacity),
            },
        }
    }
}

impl<N, K, const D: usize> PriorityQueue<N, K> for DaryHeapWithMap<N, K, D>
where
    N: Eq + Hash + Clone,
    K: PartialOrd + Clone,
{
    fn len(&self) -> usize {
        self.heap.len()
    }
    fn peek(&self) -> Option<&(N, K)> {
        self.heap.peek()
    }
    fn clear(&mut self) {
        self.heap.clear()
    }
    fn pop(&mut self) -> Option<(N, K)> {
        self.heap.pop()
    }
    fn pop_node(&mut self) -> Option<N> {
        self.heap.pop_node()
    }
    fn pop_key(&mut self) -> Option<K> {
        self.heap.pop_key()
    }
    fn push(&mut self, node: N, key: K) {
        self.heap.push(node, key)
    }
    fn push_then_pop(&mut self, node: N, key: K) -> (N, K) {
        self.heap.push_then_pop(node, key)
    }
}
impl<N, K, const D: usize> PriorityQueueDecKey<N, K> for DaryHeapWithMap<N, K, D>
where
    N: Eq + Hash + Clone,
    K: PartialOrd + Clone,
{
    fn contains(&self, node: &N) -> bool {
        self.heap.contains(node)
    }
    fn key_of(&self, node: &N) -> Option<K> {
        self.heap.key_of(node)
    }
    fn decrease_key(&mut self, node: &N, decreased_key: &K) {
        self.heap.decrease_key(node, decreased_key)
    }
    fn update_key(&mut self, node: &N, new_key: &K) -> bool {
        self.heap.update_key(node, new_key)
    }
    fn remove(&mut self, node: &N) -> K {
        self.heap.remove(node)
    }
}
