use super::heap::Heap;
use crate::{
    positions::has_index::HeapPositionsHasIndex, HasIndex, PriorityQueue, PriorityQueueDecKey,
    ResUpdateKey,
};

/// Type alias for `DaryHeapOfIndices<N, K, 2>`; see [`DaryHeapOfIndices`] for details.
pub type BinaryHeapOfIndices<N, K> = DaryHeapOfIndices<N, K, 2>;
/// Type alias for `DaryHeapOfIndices<N, K, 4>`; see [`DaryHeapOfIndices`] for details.
pub type QuaternaryHeapOfIndices<N, K> = DaryHeapOfIndices<N, K, 4>;

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
/// # Flexibility (`DaryHeapWithMap`) vs Performance (`DaryHeapOfIndices`)
///
/// `DaryHeapWithMap` (hence its variants such as `BinaryHeapWithMap`) does not require to know
/// the absolute size of the closed set.
/// Furthermore, the node type needs to implement `Hash + Eq` rather than `HasIndex` trait defined in this crate.
/// Due to these, `DaryHeapWithMap` might be considered as the more flexible [`PriorityQueueDecKey`] variant.
///
/// On the other hand, [`DaryHeapOfIndices`] (hence its variants such as [`BinaryHeapOfIndices`]),
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
/// // d-hap heap using id's to locate existing nodes (although decrease-key is not used here)
/// test_priority_queue(DaryHeapOfIndices::<_, _, 4>::with_index_bound(32));
/// // using type aliases to simplify signatures
/// test_priority_queue(BinaryHeapOfIndices::with_index_bound(16));
/// test_priority_queue(QuaternaryHeapOfIndices::with_index_bound(16));
/// test_priority_queue(QuaternaryHeapOfIndices::with_index_bound(16));
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
/// // d-ary heap using id's to locate existing nodes
/// test_priority_queue_deckey(DaryHeapOfIndices::<_, _, 3>::with_index_bound(32));
/// // using type aliases to simplify signatures
/// test_priority_queue_deckey(BinaryHeapOfIndices::with_index_bound(16));
/// test_priority_queue_deckey(QuaternaryHeapOfIndices::with_index_bound(16));
/// test_priority_queue_deckey(QuaternaryHeapOfIndices::with_index_bound(16));
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
    /// Therefore, the heap has a strict exclusive upper bound on the index of a node which can enter the heap,
    /// defined by the argument `with_index_bound`.
    ///
    /// The closed set of indices which can enter the heap is [0, 1, ..., `index_bound`).
    ///
    /// The upper bound on the indices of a `DaryHeapOfIndices` can be obtained by the `index_bound` method.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_priority_queue::*;
    ///
    /// // set of possible nodes which can enter the heap is closed and has 16 elements
    /// let mut pq = BinaryHeapOfIndices::with_index_bound(16);
    ///
    /// assert_eq!(16, pq.index_bound());
    ///
    /// // 8-th node enters the queue with key of 100.0
    /// pq.push(7usize, 100.0);
    ///
    /// // third node enters
    /// pq.push(2, 42.0);
    ///
    /// // the following line would've panicked since there exist no node with index 16 in the closed set [0, 1, ..., 15]
    /// // pq.push(16, 7.0);
    /// ```
    pub fn with_index_bound(index_bound: usize) -> Self {
        Self {
            heap: Heap::new(None, HeapPositionsHasIndex::with_index_bound(index_bound)),
        }
    }

    /// Cardinality of the closed set which the nodes are sampled from.
    ///
    /// # Panics
    ///
    /// Panics if a node with an index greater than or equal to the `index_bound` is pushed to the queue.
    pub fn index_bound(&self) -> usize {
        self.heap.positions().index_bound()
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

impl<N, K, const D: usize> PriorityQueue<N, K> for DaryHeapOfIndices<N, K, D>
where
    N: HasIndex,
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
impl<N, K, const D: usize> PriorityQueueDecKey<N, K> for DaryHeapOfIndices<N, K, D>
where
    N: HasIndex,
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
