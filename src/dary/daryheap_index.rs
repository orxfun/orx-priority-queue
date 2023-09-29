use super::heap::Heap;
use crate::{
    positions::has_index::HeapPositionsHasIndex, HasIndex, PriorityQueue, PriorityQueueDecKey,
};

/// Type alias for `DaryHeapOfIndices<N, K, 2>`; see [`DaryHeapOfIndices`] for details.
pub type BinaryHeapOfIndices<N, K> = DaryHeapOfIndices<N, K, 2>;
/// Type alias for `DaryHeapOfIndices<N, K, 3>`; see [`DaryHeapOfIndices`] for details.
pub type TernaryHeapOfIndices<N, K> = DaryHeapOfIndices<N, K, 3>;
/// Type alias for `DaryHeapOfIndices<N, K, 4>`; see [`DaryHeapOfIndices`] for details.
pub type QuarternaryHeapOfIndices<N, K> = DaryHeapOfIndices<N, K, 4>;

/// A d-ary heap which implements both `PriorityQueue` and `PriorityQueueDecKey`.
///
/// See [`PriorityQueueDecKey`] for additional functionalities.
///
/// `DaryHeapOfIndices` achieves the additional features by making use of a fixed size position
/// array which allows to track the position of nodes on the heap.
///
/// It has the limitation that the nodes must implement [`HasIndex`].
/// This trait has a single simple method `fn index(&self) -> usize` which acts as a unique identifier
/// of the actual underlying node which is coming from a closed set.
///
/// Consider for instance the usage of the heap as the priority queue of Dijkstra's shortest path algorithm.
/// The nodes are actual nodes of the graph which is a closed set and can be identified by node indices from
/// zero to `N-1`, where `N` is the number of nodes. This heap fits very well such mathematical algorithms
/// due to the following:
/// * using a fixed size array could be considered as a fast `HashMap`.
/// * we often reuse such heaps many times to solve many problems on the same network,
/// compensating for the allocation of the positions array once.
/// * further, compared to a basic priority queue (or to `std::collections::BinaryHeap`),
/// it reduces the space complexity of the Dijkstra's
/// algorithm from *O(N^2)* to *O(N)* by enabling the `decrease_key` operation.
///
/// However, for situations where
/// * the number of nodes entering the queue is very sparse compared to the size of the set of nodes, or
/// * it is not convenient to index the sets,
///
/// `DaryHeapWithMap` provides a more flexible approach.
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
/// // d-hap heap using id's to locate existing nodes (although decrease-key is not used here)
/// test_priority_queue(DaryHeapOfIndices::<_, _, 4>::with_upper_limit(32));
/// // using type aliases to simplify signatures
/// test_priority_queue(BinaryHeapOfIndices::with_upper_limit(16));
/// test_priority_queue(TernaryHeapOfIndices::with_upper_limit(16));
/// test_priority_queue(QuarternaryHeapOfIndices::with_upper_limit(16));
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
/// // d-ary heap using id's to locate existing nodes
/// test_priority_queue_deckey(DaryHeapOfIndices::<_, _, 3>::with_upper_limit(32));
/// // using type aliases to simplify signatures
/// test_priority_queue_deckey(BinaryHeapOfIndices::with_upper_limit(16));
/// test_priority_queue_deckey(TernaryHeapOfIndices::with_upper_limit(16));
/// test_priority_queue_deckey(QuarternaryHeapOfIndices::with_upper_limit(16));
/// ```
#[derive(Clone, Debug)]
pub struct DaryHeapOfIndices<N, K, const D: usize = 2>
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
    /// As explained in [`DaryHeapOfIndices`],
    /// this heap is useful when the nodes come from a closed set with a known size.
    /// Therefore, the heap has a strict `upper_limit` on the index of a node which can enter the heap.
    ///
    /// The upper limit of the queue can be obtained by the `upper_limit` method.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_priority_queue::*;
    ///
    /// // set of possible nodes which can enter the heap is closed and has 16 elements
    /// let mut pq = DaryHeapOfIndices::<usize, _, 3>::with_upper_limit(16);
    ///
    /// assert_eq!(16, pq.upper_limit());
    ///
    /// // 8-th node enters the queue with key of 100.0
    /// pq.push(7, 100.0);
    ///
    /// // third node enters
    /// pq.push(2, 42.0);
    ///
    /// // the following line would've panicked since there exist no 17-th node in the closed set
    /// // pq.push(16, 7.0);
    /// ```
    pub fn with_upper_limit(upper_limit: usize) -> Self {
        Self {
            heap: Heap::new(
                Some(upper_limit),
                HeapPositionsHasIndex::with_upper_limit(upper_limit),
            ),
        }
    }

    /// Cardinality of the closed set which the nodes are sampled from.
    ///
    /// # Panics
    ///
    /// Panics if a node with an index greater than or equal to the `upper_limit` is pushed to the queue.
    pub fn upper_limit(&self) -> usize {
        self.heap.positions().upper_limit()
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
    /// Pushes the given (`node`, `key`) pair to the queue.
    ///
    /// # Panics
    ///
    /// Panics if `node.index()` is greater than the upper limit of the heap.
    fn push(&mut self, node: N, key: K) {
        self.heap.push(node, key)
    }
    /// Performs the push with given (`node`, `key`) followed by the pop operation.
    ///
    /// Since the queue cannot be empty after the push, the return type is not optional.
    ///
    /// The reason of merging the calls is that handling two instructions at once
    /// is more efficient for certain implementations, such as for the binary heap.
    ///
    /// # Panics
    ///
    /// Panics if `node.index()` is greater than the upper limit of the heap.
    fn push_then_pop(&mut self, node: N, key: K) -> (N, K) {
        self.heap.push_then_pop(node, key)
    }

    #[cfg(test)]
    fn is_valid(&self) -> bool {
        self.heap.is_valid()
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
