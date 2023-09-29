use super::heap::Heap;
use crate::{positions::map::HeapPositionsMap, PriorityQueue, PriorityQueueDecKey};
use std::hash::Hash;

/// Type alias for `DaryHeapWithMap<N, K, 2>`; see [`DaryHeapWithMap`] for details.
pub type BinaryHeapWithMap<N, K> = DaryHeapWithMap<N, K, 2>;
/// Type alias for `DaryHeapWithMap<N, K, 3>`; see [`DaryHeapWithMap`] for details.
pub type TernaryHeapWithMap<N, K> = DaryHeapWithMap<N, K, 3>;
/// Type alias for `DaryHeapWithMap<N, K, 4>`; see [`DaryHeapWithMap`] for details.
pub type QuarternaryHeapWithMap<N, K> = DaryHeapWithMap<N, K, 4>;

/// A d-ary heap which implements both `PriorityQueue` and `PriorityQueueDecKey`.
///
/// See [`PriorityQueueDecKey`] for additional functionalities.
///
/// `DaryHeapWithMap` achieves the additional features by making use of a map of nodes to positions on the heap.
///
/// # Examples
///
/// ## Heap as a `PriorityQueue`
///
/// Usage of d-ary heap as a basic priority queue.
///
/// ```
/// use orx_priority_queue::*;
///
/// fn test_priority_queue<P>(mut pq: P)
/// where
///     P: PriorityQueue<usize, f64>
/// {
///     pq.clear();
///     
///     pq.push(0, 42.0);
///     assert_eq!(Some(&(0, 42.0)), pq.peek());
///
///     pq.push(1, 7.0);
///     assert_eq!(Some(&(1, 7.0)), pq.peek());
///
///     let popped = pq.pop();
///     assert_eq!(Some((1, 7.0)), popped);
///
///     let popped = pq.pop();
///     assert_eq!(Some((0, 42.0)), popped);
///
///     assert!(pq.is_empty());
/// }
///
/// // d-ary heap using a hash map to locate existing nodes (although decrease-key is not used here)
/// test_priority_queue(DaryHeapWithMap::<_, _, 3>::default());
/// test_priority_queue(DaryHeapWithMap::<_, _, 4>::with_capacity(16));
/// // using type aliases to simplify signatures
/// test_priority_queue(BinaryHeapWithMap::default());
/// test_priority_queue(BinaryHeapWithMap::with_capacity(16));
/// test_priority_queue(TernaryHeapWithMap::default());
/// test_priority_queue(TernaryHeapWithMap::with_capacity(16));
/// test_priority_queue(QuarternaryHeapWithMap::default());
/// test_priority_queue(QuarternaryHeapWithMap::with_capacity(16));
/// ```
///
/// ## Heap as a `PriorityQueueDecKey`
///
/// Usage of a d-ary heap as a priority queue with decrease key operation and its variants.
///
/// ```
/// use orx_priority_queue::*;
///
/// fn test_priority_queue_deckey<P>(mut pq: P)
/// where
///     P: PriorityQueueDecKey<usize, f64>
/// {
///     pq.clear();
///     
///     pq.push(0, 42.0);
///     assert_eq!(Some(&(0, 42.0)), pq.peek());
///
///     pq.push(1, 17.0);
///     assert_eq!(Some(&(1, 17.0)), pq.peek());
///
///     pq.decrease_key(&0, &7.0);
///     assert_eq!(Some(&(0, 7.0)), pq.peek());
///
///     let is_key_decreased = pq.try_decrease_key(&1, &20.0);
///     assert!(!is_key_decreased);
///
///     let popped = pq.pop();
///     assert_eq!(Some((0, 7.0)), popped);
///
///     let popped = pq.pop();
///     assert_eq!(Some((1, 17.0)), popped);
///
///     assert!(pq.is_empty());
/// }
/// // d-ary heap using a hash map to locate existing nodes
/// test_priority_queue_deckey(DaryHeapWithMap::<_, _, 3>::default());
/// test_priority_queue_deckey(DaryHeapWithMap::<_, _, 4>::with_capacity(16));
/// // using type aliases to simplify signatures
/// test_priority_queue_deckey(BinaryHeapWithMap::default());
/// test_priority_queue_deckey(BinaryHeapWithMap::with_capacity(16));
/// test_priority_queue_deckey(TernaryHeapWithMap::default());
/// test_priority_queue_deckey(TernaryHeapWithMap::with_capacity(16));
/// test_priority_queue_deckey(QuarternaryHeapWithMap::default());
/// test_priority_queue_deckey(QuarternaryHeapWithMap::with_capacity(16));
/// ```
#[derive(Debug, Clone)]
pub struct DaryHeapWithMap<N, K, const D: usize = 2>
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
            heap: Heap::new(None, HeapPositionsMap::default()),
        }
    }
}
impl<N, K, const D: usize> DaryHeapWithMap<N, K, D>
where
    N: Eq + Hash + Clone,
    K: PartialOrd + Clone,
{
    /// Creates a new d-ary heap with the given initial `capacity` on the number of nodes to simultaneously exist on the heap.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            heap: Heap::new(Some(capacity), HeapPositionsMap::with_capacity(capacity)),
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
