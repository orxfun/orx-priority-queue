use crate::PriorityQueue;

/// A [PriorityQueueDecKey] is a more advanced [PriorityQueue] with additional features
/// mainly related to accessing or modifying already pushed nodes such as:
/// * checking if a node is present in the queue,
/// * getting the key of an alrady pushed node,
/// * decreasing or updating the key of an already pushed node.
///
/// This is often achieved by additional memory requirement; hence, it is separated from the [PriorityQueue].
///
/// Another fundamental difference of [PriorityQueueDecKey] from [PriorityQueue] is that
/// it behaves as a set enabled by the `contains` method.
/// In other words,
/// * the same node can be pushed to a [PriorityQueue] an arbitrary number of times with same or different keys;
/// * on the other hand, a node can only be pushed to the [PriorityQueueDecKey] only once; however, the node in the queue can be mutated.
///
/// The [PriorityQueue] requires more space to handle a problem with lots of decrease key operations;
/// [PriorityQueueDecKey] aims to be memory efficient in such situations.
/// On the other hand, [PriorityQueue] could be preferable where number of such upadtes is limited
/// due to its lack of additional checks and tracking.
pub trait PriorityQueueDecKey<N, K>: PriorityQueue<N, K>
where
    N: Clone,
    K: PartialOrd + Clone,
{
    /// Returns whether the given `node` is in the queue or not.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_priority_queue::*;
    ///
    /// let mut queue = BinaryHeapWithMap::default();
    /// queue.push('a', 42);
    ///
    /// assert!(queue.contains(&'a'));
    /// assert!(!queue.contains(&'x'));
    /// ```
    fn contains(&self, node: &N) -> bool;

    /// Returns the key of the given `node` if it is in the queue;
    /// returns None otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_priority_queue::*;
    ///
    /// let mut queue = BinaryHeapOfIndices::with_index_bound(12);
    /// queue.push(7usize, 42.0);
    ///
    /// assert_eq!(Some(42.0), queue.key_of(&7));
    /// assert_eq!(None, queue.key_of(&3));
    /// ```
    fn key_of(&self, node: &N) -> Option<K>;

    /// Decreases key of the `node` which is already in the queue to the given `decreased_key`.
    ///
    /// This method is commonly used to increase priority of a node putting it closer to the peek of the queue;
    /// alternative to inserting the same node multiple times with different keys.
    /// This allows for memory efficient implementations of certain algorithms such as the
    /// Dijkstra's shortest path algorithm.
    ///
    /// # Panics
    /// This method panics:
    /// * if the `node` is not in the queue; or
    ///     * see [`PriorityQueueDecKey::try_decrease_key_or_push`] for a variant which pushes the
    /// `(node, new_key)` pair if the `node` is absent, rather than panicking;
    /// * if `decreased_key` is strictly larger than key of the `node` in the queue,
    ///     * see [`PriorityQueueDecKey::try_decrease_key`] for a variant which does nothing
    /// if the new key is strictly larger than key of the `node` in the queue, rather than panicking.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_priority_queue::*;
    ///
    /// let mut queue = BinaryHeapOfIndices::with_index_bound(12);
    ///
    /// queue.push(7usize, 42.0);
    /// assert_eq!(Some(42.0), queue.key_of(&7));
    ///
    /// queue.decrease_key(&7, 21.0);
    /// assert_eq!(Some(21.0), queue.key_of(&7));
    ///
    /// // the following lines would've panicked:
    /// // queue.decrease_key(&10, 21.0); // due to absent node
    /// // queue.decrease_key(&7, 100.0); // due to greater new key
    /// ```
    fn decrease_key(&mut self, node: &N, decreased_key: K);
    /// Updates key of the `node` which is already in the queue as the given `new_key`;
    /// and returns the result of the operation:
    ///
    /// * `ResUpdateKey::Decreased` if the prior key was strictly greater than the `new_key`;
    /// * `ResUpdateKey::Increased` if the prior key was less than or equal to the `new_key`.
    ///
    /// # Panics
    /// This method panics if:
    /// * the `node` is not in the queue,
    ///     * see [`PriorityQueueDecKey::update_key_or_push`] for a variant which pushes the
    /// `(node, new_key)` pair if the `node` is absent, rather than panicking.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_priority_queue::*;
    ///
    /// let mut queue = BinaryHeapWithMap::default();
    ///
    /// queue.push(7usize, 42.0);
    /// assert_eq!(Some(42.0), queue.key_of(&7));
    ///
    /// let result = queue.update_key(&7, 21.0);
    /// assert_eq!(Some(21.0), queue.key_of(&7));
    /// assert!(matches!(result, ResUpdateKey::Decreased));
    ///
    /// let result = queue.update_key(&7, 200.0);
    /// assert_eq!(Some(200.0), queue.key_of(&7));
    /// assert!(matches!(result, ResUpdateKey::Increased));
    ///
    /// // the following line would've panicked:
    /// // queue.update_key(&10, 21.0); // due to absent node
    /// ```
    fn update_key(&mut self, node: &N, new_key: K) -> ResUpdateKey;
    /// Tries to decrease the key of the `node` which is already in the queue if its prior key is strictly larger than the `new_key`;
    /// otherwise, it does nothing leaving the queue unchanged.
    ///
    /// Returns the result of the operation:
    ///
    /// * `ResTryDecreaseKey::Decreased` if the prior key was strictly greater than the `new_key`;
    /// * `ResTryDecreaseKey::Unchanged` if the prior key was less than or equal to the `new_key`.
    ///
    /// # Panics
    /// This method panics if:
    /// * the `node` is not in the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_priority_queue::*;
    ///
    /// let mut queue = BinaryHeapOfIndices::with_index_bound(12);
    ///
    /// queue.push(7usize, 42.0);
    /// assert_eq!(Some(42.0), queue.key_of(&7));
    ///
    /// let result = queue.try_decrease_key(&7, 21.0);
    /// assert_eq!(Some(21.0), queue.key_of(&7));
    /// assert!(matches!(result, ResTryDecreaseKey::Decreased));
    ///
    /// let result = queue.try_decrease_key(&7, 200.0);
    /// assert_eq!(Some(21.0), queue.key_of(&7));
    /// assert!(matches!(result, ResTryDecreaseKey::Unchanged));
    ///
    /// // the following line would've panicked:
    /// // queue.decrease_key(&10, 21.0); // due to absent node
    /// ```
    #[inline(always)]
    fn try_decrease_key(&mut self, node: &N, new_key: K) -> ResTryDecreaseKey {
        let old_key = self.key_of(node).expect("node must exist on the heap.");
        if new_key < old_key {
            self.decrease_key(node, new_key);
            ResTryDecreaseKey::Decreased
        } else {
            ResTryDecreaseKey::Unchanged
        }
    }

    /// If the `node` is present in the queue:
    /// * decreases key of the `node` to the given `decreased_key`; `decreased_key` is expected to be less than or equal
    /// to the prior key;
    ///
    /// otherwise:
    /// * pushes the new (node, key) pair to the queue.
    ///
    /// Returns the result of the operation:
    ///
    /// * `ResDecreaseKeyOrPush::Decreased` if the `node` was present in the queue and its key is decreased to `decreased_key`;
    /// * `ResDecreaseKeyOrPush::Pushed` if the `node` was absent and it is pushed with the given `decreased_key`.
    ///
    /// # Panics
    /// This method panics
    /// * if the `node` is in the queue; however, its current key is strictly less than the provided `key`;
    ///     * see [`PriorityQueueDecKey::update_key_or_push`] for a variant which increases the key
    /// if the new key is strictly larger than key of the `node` in the queue, rather than panicking; or
    ///     * see [`PriorityQueueDecKey::try_decrease_key_or_push`] for a variant which does nothing
    /// if the new key is strictly larger than key of the `node` in the queue, rather than panicking.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_priority_queue::*;
    ///
    /// let mut queue = BinaryHeapOfIndices::with_index_bound(12);
    ///
    /// queue.push(7usize, 42.0);
    /// assert_eq!(Some(42.0), queue.key_of(&7));
    ///
    /// let result = queue.decrease_key_or_push(&7, 21.0);
    /// assert_eq!(Some(21.0), queue.key_of(&7));
    /// assert!(matches!(result, ResDecreaseKeyOrPush::Decreased));
    ///
    /// let result = queue.decrease_key_or_push(&0, 10.0);
    /// assert_eq!(Some(10.0), queue.key_of(&0));
    /// assert!(matches!(result, ResDecreaseKeyOrPush::Pushed));
    ///
    /// // the following line would've panicked:
    /// // queue.decrease_key_or_push(&7, 100.0); // due to greater new key
    /// ```
    #[inline(always)]
    fn decrease_key_or_push(&mut self, node: &N, key: K) -> ResDecreaseKeyOrPush {
        if self.contains(node) {
            self.decrease_key(node, key);
            ResDecreaseKeyOrPush::Decreased
        } else {
            self.push(node.clone(), key.clone());
            ResDecreaseKeyOrPush::Pushed
        }
    }
    /// If the `node` is present in the queue:
    /// * updates key of the `node` to the given `new_key`;
    ///
    /// otherwise:
    /// * pushes the new (node, key) pair to the queue.
    ///
    /// Returns the result of the operation:
    ///
    /// * `ResUpdateKeyOrPush::Decreased` if the `node` was present in the queue with a key strictly larger than the `new_key`;
    /// * `ResUpdateKeyOrPush::Increased` if the `node` was present in the queue with a key less than or equal to the `new_key`;
    /// * `ResUpdateKeyOrPush::Pushed` if the `node` was absent and it is pushed with the given `new_key`.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_priority_queue::*;
    ///
    /// let mut queue = BinaryHeapWithMap::default();
    ///
    /// queue.push(7usize, 42.0);
    /// assert_eq!(Some(42.0), queue.key_of(&7));
    ///
    /// let result = queue.update_key_or_push(&7, 21.0);
    /// assert_eq!(Some(21.0), queue.key_of(&7));
    /// assert!(matches!(result, ResUpdateKeyOrPush::Decreased));
    ///
    /// let result = queue.update_key_or_push(&7, 200.0);
    /// assert_eq!(Some(200.0), queue.key_of(&7));
    /// assert!(matches!(result, ResUpdateKeyOrPush::Increased));
    ///
    /// let result = queue.update_key_or_push(&0, 10.0);
    /// assert_eq!(Some(10.0), queue.key_of(&0));
    /// assert!(matches!(result, ResUpdateKeyOrPush::Pushed));
    /// ```
    fn update_key_or_push(&mut self, node: &N, key: K) -> ResUpdateKeyOrPush {
        if self.contains(node) {
            self.update_key(node, key).into()
        } else {
            self.push(node.clone(), key.clone());
            ResUpdateKeyOrPush::Pushed
        }
    }
    /// If the `node` is present in the queue, tries to decrease its key to the given `key`:
    /// * its key is set to the new `key` if the prior key was strictly larger than the given key;
    /// * the queue remains unchanged if the prior key was less than or equal to the given key;
    ///
    /// otherwise, if the `node` is absent:
    /// * the new (node, key) pair is pushed to the queue.
    ///
    /// Returns the result of the operation:
    ///
    /// * `ResTryDecreaseKeyOrPush::Decreased` if the `node` was present in the queue with a key strictly larger than the `key`;
    /// * `ResTryDecreaseKeyOrPush::Unchanged` if the `node` was present in the queue with a key less than or equal to the `key`;
    /// * `ResTryDecreaseKeyOrPush::Pushed` if the `node` was absent and it is pushed with the given `new_key`.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_priority_queue::*;
    ///
    /// let mut queue = BinaryHeapOfIndices::with_index_bound(12);
    ///
    /// queue.push(7usize, 42.0);
    /// assert_eq!(Some(42.0), queue.key_of(&7));
    ///
    /// let result = queue.try_decrease_key_or_push(&7, 21.0);
    /// assert_eq!(Some(21.0), queue.key_of(&7));
    /// assert!(matches!(result, ResTryDecreaseKeyOrPush::Decreased));
    ///
    /// let result = queue.try_decrease_key_or_push(&7, 200.0);
    /// assert_eq!(Some(21.0), queue.key_of(&7));
    /// assert!(matches!(result, ResTryDecreaseKeyOrPush::Unchanged));
    ///
    /// let result = queue.try_decrease_key_or_push(&0, 10.0);
    /// assert_eq!(Some(10.0), queue.key_of(&0));
    /// assert!(matches!(result, ResTryDecreaseKeyOrPush::Pushed));
    /// ```
    #[inline(always)]
    fn try_decrease_key_or_push(&mut self, node: &N, key: K) -> ResTryDecreaseKeyOrPush {
        match self.key_of(node) {
            Some(old_key) => {
                if key < old_key {
                    self.decrease_key(node, key);
                    ResTryDecreaseKeyOrPush::Decreased
                } else {
                    ResTryDecreaseKeyOrPush::Unchanged
                }
            }
            None => {
                self.push(node.clone(), key.clone());
                ResTryDecreaseKeyOrPush::Pushed
            }
        }
    }

    /// Removes the `node` from the queue; and returns its current key.
    ///
    /// # Panics
    /// This method panics if:
    /// * the `node` is not in the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_priority_queue::*;
    ///
    /// let mut queue = BinaryHeapWithMap::default();
    ///
    /// queue.push(7usize, 42.0);
    /// assert_eq!(Some(42.0), queue.key_of(&7));
    ///
    /// let key = queue.remove(&7);
    /// assert_eq!(42.0, key);
    /// assert!(queue.is_empty());
    ///
    /// // the following line would've panicked due to absent node
    /// // let key = queue.remove(&7);
    /// ```
    fn remove(&mut self, node: &N) -> K;
}

/// Result of `queue.update_key(node, new_key)` operation : [`PriorityQueueDecKey::update_key`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResUpdateKey {
    /// Existing key of the `node` was higher; and hence, decreased to the `new_key`.
    Decreased,
    /// Existing key of the `node` was lower; and hence, increased to the `new_key`.
    Increased,
}
/// Result of `queue.try_decrease_key(node, new_key)` operation : [`PriorityQueueDecKey::try_decrease_key`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResTryDecreaseKey {
    /// Existing key of the `node` was higher; and hence, decreased to the `new_key`.
    Decreased,
    /// Existing key of the `node` was lower; and hence, the queue is not changed.
    Unchanged,
}
/// Result of `queue.decrease_key_or_push(node, key)` operation : [`PriorityQueueDecKey::decrease_key_or_push`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResDecreaseKeyOrPush {
    /// The `node` did not exist in the queue; and hence, pushed to the queue with the given `key`.
    Pushed,
    /// The `node` existed in the queue, its key was higher; and hence, decreased to the given `key`.
    Decreased,
}
/// Result of `queue.update_key_or_push(node, key)` operation : [`PriorityQueueDecKey::update_key_or_push`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResUpdateKeyOrPush {
    /// The `node` did not exist in the queue; and hence, pushed to the queue with the given `key`.
    Pushed,
    /// The `node` existed in the queue, its key was higher; and hence, decreased to the given `key`.
    Decreased,
    /// The `node` existed in the queue, its key was lower; and hence, increased to the given `key`.
    Increased,
}
/// Result of `queue.try_decrease_key_or_push(node, key)` operation : [`PriorityQueueDecKey::try_decrease_key_or_push`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResTryDecreaseKeyOrPush {
    /// The `node` did not exist in the queue; and hence, pushed to the queue with the given `key`.
    Pushed,
    /// The `node` existed in the queue, its key was higher; and hence, decreased to the given `key`.
    Decreased,
    /// The `node` existed in the queue, its key was lower; and hence, the queue is not changed.
    Unchanged,
}

impl From<ResUpdateKey> for ResUpdateKeyOrPush {
    fn from(value: ResUpdateKey) -> Self {
        match value {
            ResUpdateKey::Decreased => Self::Decreased,
            ResUpdateKey::Increased => Self::Increased,
        }
    }
}
impl From<ResTryDecreaseKey> for ResTryDecreaseKeyOrPush {
    fn from(value: ResTryDecreaseKey) -> Self {
        match value {
            ResTryDecreaseKey::Decreased => Self::Decreased,
            ResTryDecreaseKey::Unchanged => Self::Unchanged,
        }
    }
}
