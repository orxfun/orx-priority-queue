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
    fn contains(&self, node: &N) -> bool;

    /// Returns the key of the given `node` if it is in the queue;
    /// returns None otherwise.
    fn key_of(&self, node: &N) -> Option<K>;

    /// Decreases key of the `node` which is already in the queue to the given `decreased_key`.
    /// This method is commonly use to increase priority of a node;
    /// rather than to re-insert it to keep the size of the queue smaller.
    ///
    /// # Panics
    /// This method panics if:
    /// * the `node` is not in the queue; or
    /// * `decreased_key` is strictly larget than key of the `node` in the queue.
    ///
    /// # See also
    /// Note that the following methods have minor but important differences
    /// making them suitable for different cases/algorithms:
    /// `decrease_key`, `update_key`, `try_decrease_key`, `decrease_key_or_push`, `update_key_or_push` and `try_decrease_key_or_push`.
    fn decrease_key(&mut self, node: &N, decreased_key: &K);
    /// Updates key of the `node` which is already in the queue as the given `new_key`;
    /// and returns whether the node's key is strictly decreased or not.
    ///
    /// # Panics
    /// This method panics if:
    /// * the `node` is not in the queue.
    ///
    /// # See also
    /// Note that the following methods have minor but important differences
    /// making them suitable for different cases/algorithms:
    /// `decrease_key`, `update_key`, `try_decrease_key`, `decrease_key_or_push`, `update_key_or_push` and `try_decrease_key_or_push`.
    fn update_key(&mut self, node: &N, new_key: &K) -> bool;
    /// This method:
    /// * when `new_key` is strictly less than the `node`'s current key:
    ///     * decreases the key of the node to the given `new_key`, and
    ///     * returns true;
    /// * otherwise:
    ///     * does not change the queue, and
    ///     * returns false.
    ///
    /// In brief, the method returns whether the key of the `node` is decreased or not.
    ///
    /// # Panics
    /// This method panics if:
    /// * the `node` is not in the queue.
    ///
    /// # See also
    /// Note that the following methods have minor but important differences
    /// making them suitable for different cases/algorithms:
    /// `decrease_key`, `update_key`, `try_decrease_key`, `decrease_key_or_push`, `update_key_or_push` and `try_decrease_key_or_push`.
    fn try_decrease_key(&mut self, node: &N, new_key: &K) -> bool {
        let old_key = self.key_of(node).expect("node must exist on the heap.");
        if new_key < &old_key {
            self.decrease_key(node, new_key);
            true
        } else {
            false
        }
    }

    /// This method
    /// * when the `node` is present in the queue:
    ///     * decreases its key to the given new `key` which is expected to be less than or equal to the current key, and
    ///     * returns true;
    /// * otherwise:
    ///     * pushes the `node` with the given `key` to the queue, and
    ///     * returns false.
    ///
    /// In brief, the method returns whether the key of the `node` is decreased or not.
    ///
    /// # Panics
    /// This method panics if:
    /// * the `node` is in the queue; however, its current key is strictly less than the provided `key`.
    ///
    /// # See also
    /// Note that the following methods have minor but important differences
    /// making them suitable for different cases/algorithms:
    /// `decrease_key`, `update_key`, `try_decrease_key`, `decrease_key_or_push`, `update_key_or_push` and `try_decrease_key_or_push`.
    fn decrease_key_or_push(&mut self, node: &N, key: &K) -> bool {
        if self.contains(node) {
            self.decrease_key(node, key);
            true
        } else {
            self.push(node.clone(), key.clone());
            false
        }
    }
    /// This method
    /// * when the `node` is present in the queue:
    ///     * updates its key to the given new `key`, and
    ///     * returns whether the update operation strictly decreased the node's key or not;
    /// * otherwise:
    ///     * pushes the `node` with the given `key` to the queue, and
    ///     * returns false.
    ///
    /// In brief, the method returns whether the key of the `node` is decreased or not.
    ///
    /// # See also
    /// Note that the following methods have minor but important differences
    /// making them suitable for different cases/algorithms:
    /// `decrease_key`, `update_key`, `try_decrease_key`, `decrease_key_or_push`, `update_key_or_push` and `try_decrease_key_or_push`.
    fn update_key_or_push(&mut self, node: &N, key: &K) -> bool {
        if self.contains(node) {
            self.update_key(node, key)
        } else {
            self.push(node.clone(), key.clone());
            false
        }
    }
    /// This method
    /// * when the `node` is present in the queue:
    ///     * when the new `key` is strictly less than the `node`'s current key:
    ///         * decreases the key of the node to the given `key`, and
    ///         * returns true;
    ///     * otherwise:
    ///         * does not change the queue, and
    ///         * returns false;
    /// * otherwise:
    ///     * pushes the `node` with the given `key` to the queue, and
    ///     * returns false.
    ///
    /// In brief, the method returns whether the key of the `node` is decreased or not.
    ///
    /// # See also
    /// Note that the following methods have minor but important differences
    /// making them suitable for different cases/algorithms:
    /// `decrease_key`, `update_key`, `try_decrease_key`, `decrease_key_or_push`, `update_key_or_push` and `try_decrease_key_or_push`.
    fn try_decrease_key_or_push(&mut self, node: &N, key: &K) -> bool {
        if self.contains(node) {
            self.try_decrease_key(node, key)
        } else {
            self.push(node.clone(), key.clone());
            false
        }
    }

    /// Removes the `node` from the queue; and returns its current key.
    ///
    /// # Panics
    /// This method panics if:
    /// * the `node` is not in the queue.
    fn remove(&mut self, node: &N) -> K;
}
