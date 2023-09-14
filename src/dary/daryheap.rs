use super::heap::Heap;
use crate::{positions::none::HeapPositionsNone, PriorityQueue};

/// A d-ary heap which implements `PriorityQueue` and `PriorityQueueDecKey`.
///
/// # Examples
///
/// ## Heap as `PriorityQueue`
///
/// Usage of d-ary heap as a basic priority queue.
///
/// ```
/// use orx_priority_queue::prelude::*;
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
/// // basic quaternary heap without any means to located existing nodes
/// test_priority_queue(DaryHeap::<_, _, 4>::default());
/// test_priority_queue(DaryHeap::<_, _, 4>::with_capacity(16));
///
/// // octonary heap using map to locate existing nodes (although decrease-key is not used here)
/// test_priority_queue(DaryHeapWithMap::<_, _, 8>::default());
/// test_priority_queue(DaryHeapWithMap::<_, _, 8>::with_capacity(64));
///
/// // binary heap using id's to locate existing nodes (although decrease-key is not used here)
/// test_priority_queue(DaryHeapOfIndices::<_, _, 2>::with_upper_limit(32));
/// ```
///
/// ## Heap as `PriorityQueueDecKey`
///
/// Usage of a d-ary heap as a priority queue with decrease key operation and its variants.
///
/// ```
/// use orx_priority_queue::prelude::*;
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
///
/// // octonary heap using map to locate existing nodes
/// test_priority_queue_deckey(DaryHeapWithMap::<_, _, 8>::default());
/// test_priority_queue_deckey(DaryHeapWithMap::<_, _, 8>::with_capacity(64));
///
/// // binary heap using id's to locate existing nodes
/// test_priority_queue_deckey(DaryHeapOfIndices::<_, _, 2>::with_upper_limit(32));
/// ```
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
    /// Creates a new d-ary heap with the given initial `capacity` on the number of nodes to simultaneously exist on the heap.
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
