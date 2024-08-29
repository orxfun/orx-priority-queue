use super::heap::Heap;
use crate::{positions::none::HeapPositionsNone, PriorityQueue};

/// Type alias for `DaryHeap<N, K, 2>`; see [`DaryHeap`] for details.
pub type BinaryHeap<N, K> = DaryHeap<N, K, 2>;
/// Type alias for `DaryHeap<N, K, 4>`; see [`DaryHeap`] for details.
pub type QuaternaryHeap<N, K> = DaryHeap<N, K, 4>;

/// A d-ary heap which implements `PriorityQueue`, but not `PriorityQueueDecKey`.
///
/// *Its interface is similar to `std::collections:BinaryHeap; however, provides a generalization by allowing different d values.
/// `DaryHeapMap` and DaryHeapOfIndices` on the other hand, provides the additional functionality of `PriorityQueueDecKey`
/// which are crucial for providing better space complexity in algorithms such as the Dijkstra's shortest path algorithm.*
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
///     assert_eq!(Some(&0), pq.peek().map(|x| x.node()));
///     assert_eq!(Some(&42.0), pq.peek().map(|x| x.key()));
///
///     pq.push(1, 7.0);
///     assert_eq!(Some(&1), pq.peek().map(|x| x.node()));
///     assert_eq!(Some(&7.0), pq.peek().map(|x| x.key()));
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
/// // basic d-heap without any means to located existing nodes
/// test_priority_queue(DaryHeap::<_, _, 4>::default());
/// test_priority_queue(DaryHeap::<_, _, 3>::with_capacity(16));
/// // using type aliases to simplify signatures
/// test_priority_queue(BinaryHeap::default());
/// test_priority_queue(BinaryHeap::with_capacity(16));
/// test_priority_queue(QuaternaryHeap::default());
/// test_priority_queue(QuaternaryHeap::with_capacity(16));
/// test_priority_queue(QuaternaryHeap::default());
/// test_priority_queue(QuaternaryHeap::with_capacity(16));
/// ```
#[derive(Clone, Debug)]
pub struct DaryHeap<N, K, const D: usize = 2>
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
    /// Creates a new empty d-ary heap.
    ///
    ///  # Examples
    ///
    /// ```
    /// use orx_priority_queue::*;
    ///
    /// let mut heap = BinaryHeap::new();
    ///
    /// heap.push('a', 4);
    /// heap.push('b', 42);
    ///
    /// assert_eq!(Some('a'), heap.pop_node());
    /// assert_eq!(Some('b'), heap.pop_node());
    /// assert!(heap.is_empty());
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new d-ary heap with the given initial `capacity` on the number of nodes to simultaneously exist on the heap.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_priority_queue::*;
    ///
    /// // create a queue with an expected space complexity of 4
    /// let mut queue = DaryHeap::<_, _, 4>::with_capacity(4);
    /// queue.push('a', 4);
    /// assert_eq!(Some('a'), queue.pop_node());
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            heap: Heap::new(Some(capacity), HeapPositionsNone),
        }
    }

    /// Returns the 'd' of the d-ary heap.
    /// In other words, it represents the maximum number of children that each node on the heap can have.
    pub const fn d() -> usize {
        D
    }

    // additional functionalities
    /// Returns the nodes and keys currently in the queue as a slice;
    /// not necessarily sorted.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_priority_queue::*;
    ///
    /// let mut queue = QuaternaryHeapWithMap::default();
    /// queue.push("x", 42);
    /// queue.push("y", 7);
    /// queue.push("z", 99);
    ///
    /// let slice = queue.as_slice();
    ///
    /// assert_eq!(3, slice.len());
    /// assert!(slice.contains(&("x", 42)));
    /// assert!(slice.contains(&("y", 7)));
    /// assert!(slice.contains(&("z", 99)));
    /// ```
    pub fn as_slice(&self) -> &[(N, K)] {
        self.heap.as_slice()
    }
}

impl<N, K, const D: usize> PriorityQueue<N, K> for DaryHeap<N, K, D>
where
    N: Clone,
    K: PartialOrd + Clone,
{
    type NodeKey<'a> = &'a (N, K) where Self: 'a, N: 'a, K: 'a;
    type Iter<'a> = core::slice::Iter<'a, (N, K)> where Self: 'a, N: 'a, K: 'a;

    #[inline(always)]
    fn len(&self) -> usize {
        self.heap.len()
    }

    #[inline(always)]
    fn capacity(&self) -> usize {
        self.heap.capacity()
    }

    fn peek(&self) -> Option<&(N, K)> {
        self.heap.peek()
    }

    fn clear(&mut self) {
        self.heap.clear()
    }

    #[inline(always)]
    fn pop(&mut self) -> Option<(N, K)> {
        self.heap.pop()
    }

    #[inline(always)]
    fn pop_node(&mut self) -> Option<N> {
        self.heap.pop_node()
    }

    #[inline(always)]
    fn pop_key(&mut self) -> Option<K> {
        self.heap.pop_key()
    }

    #[inline(always)]
    fn push(&mut self, node: N, key: K) {
        self.heap.push(node, key)
    }

    #[inline(always)]
    fn push_then_pop(&mut self, node: N, key: K) -> (N, K) {
        self.heap.push_then_pop(node, key)
    }

    fn iter(&self) -> Self::Iter<'_> {
        self.as_slice().iter()
    }
}
