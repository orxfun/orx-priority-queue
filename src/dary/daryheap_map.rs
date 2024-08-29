use super::heap::Heap;
use crate::{
    positions::map::{HeapPositionsMap, Index},
    PriorityQueue, PriorityQueueDecKey, ResUpdateKey,
};

/// Type alias for `DaryHeapWithMap<N, K, 2>`; see [`DaryHeapWithMap`] for details.
pub type BinaryHeapWithMap<N, K> = DaryHeapWithMap<N, K, 2>;
/// Type alias for `DaryHeapWithMap<N, K, 4>`; see [`DaryHeapWithMap`] for details.
pub type QuaternaryHeapWithMap<N, K> = DaryHeapWithMap<N, K, 4>;

/// A d-ary heap which implements both `PriorityQueue` and `PriorityQueueDecKey`.
///
/// See [`PriorityQueueDecKey`] for additional functionalities.
///
/// `DaryHeapWithMap` achieves the additional features by making use of a map of nodes to positions on the heap.
///
/// # Flexibility (`DaryHeapWithMap`) vs Performance (`DaryHeapOfIndices`)
///
/// [`DaryHeapWithMap`] (hence its variants such as [`BinaryHeapWithMap`]) does not require to know
/// the absolute size of the closed set.
/// Furthermore, the node type needs to implement `Hash + Eq` rather than `HasIndex` trait defined in this crate.
/// Due to these, `DaryHeapWithMap` might be considered as the more flexible [`PriorityQueueDecKey`] variant.
///
/// On the other hand, `DaryHeapOfIndices` (hence its variants such as `BinaryHeapOfIndices`),
/// provides significantly faster accesses to positions of nodes on the heap.
/// This is important for [`PriorityQueueDecKey`] operations such as `decrease_key` or `contains`.
/// Furthermore, in many algorithms such as certain network algorithms where nodes enter and exit the queue,
/// `index_bound` can often trivially be set to number of nodes.
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
/// // d-ary heap using a hash map to locate existing nodes (although decrease-key is not used here)
/// test_priority_queue(DaryHeapWithMap::<_, _, 3>::default());
/// test_priority_queue(DaryHeapWithMap::<_, _, 4>::with_capacity(16));
/// // using type aliases to simplify signatures
/// test_priority_queue(BinaryHeapWithMap::default());
/// test_priority_queue(BinaryHeapWithMap::with_capacity(16));
/// test_priority_queue(QuaternaryHeapWithMap::default());
/// test_priority_queue(QuaternaryHeapWithMap::with_capacity(16));
/// test_priority_queue(QuaternaryHeapWithMap::default());
/// test_priority_queue(QuaternaryHeapWithMap::with_capacity(16));
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
///     assert_eq!(Some(&0), pq.peek().map(|x| x.node()));
///     assert_eq!(Some(&42.0), pq.peek().map(|x| x.key()));
///
///     pq.push(1, 17.0);
///     assert_eq!(Some(&1), pq.peek().map(|x| x.node()));
///     assert_eq!(Some(&17.0), pq.peek().map(|x| x.key()));
///
///     pq.decrease_key(&0, 7.0);
///     assert_eq!(Some(&0), pq.peek().map(|x| x.node()));
///     assert_eq!(Some(&7.0), pq.peek().map(|x| x.key()));
///
///     let res_try_deckey = pq.try_decrease_key(&1, 20.0);
///     assert_eq!(res_try_deckey, ResTryDecreaseKey::Unchanged);
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
/// test_priority_queue_deckey(QuaternaryHeapWithMap::default());
/// test_priority_queue_deckey(QuaternaryHeapWithMap::with_capacity(16));
/// test_priority_queue_deckey(QuaternaryHeapWithMap::default());
/// test_priority_queue_deckey(QuaternaryHeapWithMap::with_capacity(16));
/// ```
#[derive(Debug, Clone)]
pub struct DaryHeapWithMap<N, K, const D: usize = 2>
where
    N: Index,
    K: PartialOrd + Clone,
{
    heap: Heap<N, K, HeapPositionsMap<N>, D>,
}

impl<N, K, const D: usize> Default for DaryHeapWithMap<N, K, D>
where
    N: Index,
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
    N: Index,
    K: PartialOrd + Clone,
{
    /// Creates a new empty d-ary heap.
    ///
    ///  # Examples
    ///
    /// ```
    /// use orx_priority_queue::*;
    ///
    /// let mut heap = BinaryHeapWithMap::new();
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
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            heap: Heap::new(Some(capacity), HeapPositionsMap::with_capacity(capacity)),
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

impl<N, K, const D: usize> PriorityQueue<N, K> for DaryHeapWithMap<N, K, D>
where
    N: Index,
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
impl<N, K, const D: usize> PriorityQueueDecKey<N, K> for DaryHeapWithMap<N, K, D>
where
    N: Index,
    K: PartialOrd + Clone,
{
    #[inline(always)]
    fn contains(&self, node: &N) -> bool {
        self.heap.contains(node)
    }

    #[inline(always)]
    fn key_of(&self, node: &N) -> Option<K> {
        self.heap.key_of(node)
    }

    #[inline(always)]
    fn decrease_key(&mut self, node: &N, decreased_key: K) {
        self.heap.decrease_key(node, decreased_key)
    }

    #[inline(always)]
    fn update_key(&mut self, node: &N, new_key: K) -> ResUpdateKey {
        self.heap.update_key(node, new_key)
    }

    #[inline(always)]
    fn remove(&mut self, node: &N) -> K {
        self.heap.remove(node)
    }
}
