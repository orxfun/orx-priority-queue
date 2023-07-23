use super::heap::Heap;
use crate::{
    positions::has_index::HeapPositionsHasIndex, HasIndex, PriorityQueue, PriorityQueueDecKey,
};

#[derive(Clone)]
pub struct DaryHeapOfIndices<N, K, const D: usize>
where
    N: HasIndex,
    K: PartialOrd + Clone,
{
    heap: Heap<N, K, HeapPositionsHasIndex<N>, D>,
}

impl<N, K, const D: usize> DaryHeapOfIndices<N, K, D>
where
    N: HasIndex,
    K: PartialOrd + Clone,
{
    pub fn with_upper_limit(upper_limit: usize) -> Self {
        Self {
            heap: Heap {
                tree: Vec::with_capacity(upper_limit),
                positions: HeapPositionsHasIndex::with_upper_limit(upper_limit),
            },
        }
    }
}

impl<N, K, const D: usize> PriorityQueue<N, K> for DaryHeapOfIndices<N, K, D>
where
    N: HasIndex,
    K: PartialOrd + Clone,
{
    fn len(&self) -> usize {
        self.heap.len()
    }
    fn as_slice(&self) -> &[(N, K)] {
        self.heap.as_slice()
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
impl<N, K, const D: usize> PriorityQueueDecKey<N, K> for DaryHeapOfIndices<N, K, D>
where
    N: HasIndex,
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
