use super::heap::Heap;
use crate::{positions::none::HeapPositionsNone, PriorityQueue};

#[derive(Clone, Debug)]
pub struct DaryHeap<N, K, const D: usize>
where
    N: Clone,
    K: PartialOrd + Clone,
{
    heap: Heap<N, K, HeapPositionsNone, D>,
}

impl<N, K, const D: usize> Default for DaryHeap<N, K, D>
where
    N: Clone,
    K: PartialOrd + Clone,
{
    fn default() -> Self {
        Self {
            heap: Heap::new(None, HeapPositionsNone),
        }
    }
}
impl<N, K, const D: usize> DaryHeap<N, K, D>
where
    N: Clone,
    K: PartialOrd + Clone,
{
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            heap: Heap::new(Some(capacity), HeapPositionsNone),
        }
    }
}

impl<N, K, const D: usize> PriorityQueue<N, K> for DaryHeap<N, K, D>
where
    N: Clone,
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
    #[cfg(test)]
    fn is_valid(&self) -> bool {
        self.heap.is_valid()
    }
}
