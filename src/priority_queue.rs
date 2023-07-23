/// A priority queue which allows pushing (N, K)=(node, key) pairs to the collection,
/// and popping the foremost element having the lowest key.
pub trait PriorityQueue<N, K>: Clone
where
    K: PartialOrd,
{
    /// Number of elements in the queue.
    fn len(&self) -> usize;

    /// Returns whether he queue is empty or not.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the nodes and keys currently in the queue as a slice;
    /// not necessarily sorted.
    fn as_slice(&self) -> &[(N, K)];

    /// Returns, without popping, a reference to the foremost element of the queue;
    /// returns None if the queue is empty.
    fn peek(&self) -> Option<&(N, K)>;

    /// Clears the queue.
    fn clear(&mut self);

    /// Removes and returns the (node, key) pair with the lowest key in the queue;
    /// returns None if the queue is empty.
    fn pop(&mut self) -> Option<(N, K)>;
    /// Removes and returns the node with the lowest key in the queue;
    /// returns None if the queue is empty.
    fn pop_node(&mut self) -> Option<N>;
    /// Removes and returns the key of the node with the lowest key in the queue;
    /// returns None if the queue is empty.
    fn pop_key(&mut self) -> Option<K>;

    /// Pushes the given (`node`, `key`) pair to the queue.
    fn push(&mut self, node: N, key: K);

    /// Performs the push with given (`node`, `key`) followed by the pop operation.
    ///
    /// Since the queue cannot be empty after the push, the return type is not optional.
    ///
    /// The reason of merging the calls is that handling two instructions at once
    /// is more efficient for certain implementations, such as the binary heap,
    fn push_then_pop(&mut self, node: N, key: K) -> (N, K);
}
