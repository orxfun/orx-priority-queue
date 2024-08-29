use crate::NodeKeyRef;

/// A priority queue which allows pushing (N, K)=(node, key) pairs to the collection,
/// and popping the foremost element having the lowest key.
pub trait PriorityQueue<N, K>
where
    K: PartialOrd,
{
    /// Item providing references to node & key pairs which are yielded by the iterator.
    type NodeKey<'a>: NodeKeyRef<'a, N, K>
    where
        Self: 'a,
        N: 'a,
        K: 'a;

    /// An iterator over the (node, key) pairs on the priority queue in an arbitrary order.
    type Iter<'a>: Iterator<Item = Self::NodeKey<'a>>
    where
        Self: 'a,
        N: 'a,
        K: 'a;

    /// Number of elements in the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_priority_queue::*;
    ///
    /// let mut queue = DaryHeap::<_, _, 16>::default();
    ///
    /// queue.push('a', 42);
    /// queue.push('b', 7);
    /// assert_eq!(2, queue.len());
    ///
    /// _ = queue.pop();
    /// assert_eq!(1, queue.len());
    /// ```
    fn len(&self) -> usize;

    /// Capacity of the heap.
    fn capacity(&self) -> usize;

    /// Returns whether he queue is empty or not.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_priority_queue::*;
    ///
    /// let mut queue = QuaternaryHeap::default();
    /// assert!(queue.is_empty());
    ///
    /// queue.push("wisdom", 42);
    /// assert!(!queue.is_empty());
    /// ```
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns, without popping, a reference to the foremost element of the queue;
    /// returns None if the queue is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_priority_queue::*;
    ///
    /// let mut queue = BinaryHeap::default();
    /// assert_eq!(None, queue.peek());
    ///
    /// queue.push(0, 12.0);
    /// queue.push(42, 1.0);
    /// queue.push(21, 5.0);
    /// assert_eq!(Some(&(42, 1.0)), queue.peek());
    /// ```
    fn peek(&self) -> Option<Self::NodeKey<'_>>;

    /// Clears the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_priority_queue::*;
    ///
    /// let mut queue = QuaternaryHeap::default();
    /// assert!(queue.is_empty());
    ///
    /// queue.push(0, 12.0);
    /// queue.push(42, 1.0);
    /// queue.push(21, 5.0);
    /// assert!(!queue.is_empty());
    ///
    /// queue.clear();
    /// assert!(queue.is_empty());
    /// ```
    fn clear(&mut self);

    /// Removes and returns the (node, key) pair with the lowest key in the queue;
    /// returns None if the queue is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_priority_queue::*;
    ///
    /// let mut queue = BinaryHeap::default();
    /// assert_eq!(None, queue.pop());
    ///
    /// queue.push(0, 12.0);
    /// queue.push(42, 1.0);
    /// queue.push(21, 5.0);
    /// assert_eq!(3, queue.len());
    ///
    /// assert_eq!(Some((42, 1.0)), queue.pop());
    /// assert_eq!(2, queue.len());
    ///
    /// assert_eq!(Some((21, 5.0)), queue.pop());
    /// assert_eq!(1, queue.len());
    ///
    /// assert_eq!(Some((0, 12.0)), queue.pop());
    /// assert!(queue.is_empty());
    ///
    /// assert_eq!(None, queue.pop());
    /// ```
    fn pop(&mut self) -> Option<(N, K)>;

    /// Removes and returns the node with the lowest key in the queue;
    /// returns None if the queue is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_priority_queue::*;
    ///
    /// let mut queue = BinaryHeap::default();
    /// assert_eq!(None, queue.pop_node());
    ///
    /// queue.push(0, 12.0);
    /// queue.push(42, 1.0);
    /// queue.push(21, 5.0);
    /// assert_eq!(3, queue.len());
    ///
    /// assert_eq!(Some(42), queue.pop_node());
    /// assert_eq!(2, queue.len());
    ///
    /// assert_eq!(Some(21), queue.pop_node());
    /// assert_eq!(1, queue.len());
    ///
    /// assert_eq!(Some(0), queue.pop_node());
    /// assert!(queue.is_empty());
    ///
    /// assert_eq!(None, queue.pop_node());
    /// ```
    fn pop_node(&mut self) -> Option<N>;

    /// Removes and returns the key of the node with the lowest key in the queue;
    /// returns None if the queue is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_priority_queue::*;
    ///
    /// let mut queue = BinaryHeap::default();
    /// assert_eq!(None, queue.pop_key());
    ///
    /// queue.push(0, 12.0);
    /// queue.push(42, 1.0);
    /// queue.push(21, 5.0);
    /// assert_eq!(3, queue.len());
    ///
    /// assert_eq!(Some(1.0), queue.pop_key());
    /// assert_eq!(2, queue.len());
    ///
    /// assert_eq!(Some(5.0), queue.pop_key());
    /// assert_eq!(1, queue.len());
    ///
    /// assert_eq!(Some(12.0), queue.pop_key());
    /// assert!(queue.is_empty());
    ///
    /// assert_eq!(None, queue.pop_key());
    /// ```
    fn pop_key(&mut self) -> Option<K>;

    /// Pushes the given (`node`, `key`) pair to the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_priority_queue::*;
    ///
    /// let mut queue = BinaryHeap::default();
    /// assert!(queue.is_empty());
    ///
    /// queue.push(0, 12.0);
    /// queue.push(42, 1.0);
    /// queue.push(21, 5.0);
    /// assert_eq!(3, queue.len());
    /// ```
    fn push(&mut self, node: N, key: K);

    /// Performs the push with given (`node`, `key`) followed by the pop operation.
    ///
    /// Since the queue cannot be empty after the push, the return type is not optional.
    ///
    /// The reason of merging the calls is that handling two instructions at once is
    /// significantly more efficient for implementations taking benefit of this composition.
    /// `DaryHeap` and its variants such the `BinaryHeap` are examples to this (see "benches/push_then_pop.rs").
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_priority_queue::*;
    ///
    /// let mut queue = BinaryHeap::default();
    /// assert!(queue.is_empty());
    ///
    /// // returns the (node, key) back when the queue is empty
    /// let popped = queue.push_then_pop(3, 33.3);
    /// assert_eq!((3, 33.3), popped);
    /// assert!(queue.is_empty());
    ///
    /// queue.push(0, 12.0);
    /// queue.push(42, 1.0);
    /// queue.push(21, 5.0);
    /// assert_eq!(3, queue.len()); // sorted-nodes: 42 (1.0) << 21 (5.0) << 0 (12.0)
    ///
    /// let popped = queue.push_then_pop(100, 100.0);
    /// assert_eq!((42, 1.0), popped);
    /// assert_eq!(3, queue.len()); // sorted-nodes: 21 (5.0) << 0 (12.0) << 100 (100.0)
    ///
    /// let popped = queue.push_then_pop(6, 6.0);
    /// assert_eq!((21, 5.0), popped);
    /// assert_eq!(3, queue.len()); // sorted-nodes: 6 (6.0) << 0 (12.0) << 100 (100.0)
    ///
    /// let popped = queue.push_then_pop(13, 13.0);
    /// assert_eq!((6, 6.0), popped);
    /// assert_eq!(3, queue.len()); // sorted-nodes: 0 (12.0) << 13 (13.0) << 100 (100.0)
    ///
    /// assert_eq!(Some((0, 12.0)), queue.pop());
    /// assert_eq!(Some((13, 13.0)), queue.pop());
    /// assert_eq!(Some((100, 100.0)), queue.pop());
    /// assert!(queue.is_empty());
    /// ```
    fn push_then_pop(&mut self, node: N, key: K) -> (N, K);

    /// Returns an iterator visiting all values on the heap in arbitrary order.
    fn iter(&self) -> Self::Iter<'_>;
}
