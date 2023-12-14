use crate::{
    priority_queue::PriorityQueue, PriorityQueueDecKey, ResTryDecreaseKeyOrPush, ResUpdateKey,
};
use std::hash::Hash;

impl<N, K> PriorityQueue<N, K> for priority_queue::PriorityQueue<N, K>
where
    K: PartialOrd + Ord,
    N: Ord + Hash,
{
    type NodeKey<'a> = (&'a N, &'a K) where Self: 'a, N: 'a, K: 'a;
    type Iter<'a> = priority_queue::core_iterators::Iter<'a, N, K> where Self: 'a, N: 'a, K: 'a;

    fn len(&self) -> usize {
        priority_queue::PriorityQueue::len(self)
    }
    fn capacity(&self) -> usize {
        priority_queue::PriorityQueue::capacity(self)
    }
    #[inline(always)]
    fn peek(&self) -> Option<Self::NodeKey<'_>> {
        priority_queue::PriorityQueue::peek(self)
    }
    #[inline(always)]
    fn clear(&mut self) {
        priority_queue::PriorityQueue::clear(self)
    }
    #[inline(always)]
    fn pop(&mut self) -> Option<(N, K)> {
        priority_queue::PriorityQueue::pop(self)
    }
    #[inline(always)]
    fn pop_node(&mut self) -> Option<N> {
        priority_queue::PriorityQueue::pop(self).map(|x| x.0)
    }
    #[inline(always)]
    fn pop_key(&mut self) -> Option<K> {
        priority_queue::PriorityQueue::pop(self).map(|x| x.1)
    }
    #[inline(always)]
    fn push(&mut self, node: N, key: K) {
        priority_queue::PriorityQueue::push(self, node, key);
    }
    #[inline(always)]
    fn push_then_pop(&mut self, node: N, key: K) -> (N, K) {
        priority_queue::PriorityQueue::push(self, node, key);
        priority_queue::PriorityQueue::pop(self).expect("queue is not empty")
    }
    fn iter(&self) -> Self::Iter<'_> {
        priority_queue::PriorityQueue::iter(self)
    }
}

impl<N, K> PriorityQueueDecKey<N, K> for priority_queue::PriorityQueue<N, K>
where
    K: PartialOrd + Ord + Clone,
    N: Ord + Hash + Clone,
{
    #[inline(always)]
    fn contains(&self, node: &N) -> bool {
        priority_queue::PriorityQueue::get(self, node).is_some()
    }
    #[inline(always)]
    fn key_of(&self, node: &N) -> Option<K> {
        priority_queue::PriorityQueue::get(self, node).map(|x| x.1.clone())
    }
    fn decrease_key(&mut self, node: &N, decreased_key: K) {
        let old_key =
            priority_queue::PriorityQueue::change_priority(self, node, decreased_key.clone())
                .expect("Failed to update key of the node, it is not present in the queue");
        let _decreased = (if decreased_key <= old_key {
            Some(true)
        } else {
            None
        })
        .expect("Failed to decrease the key of the node, received a greater key");
    }
    fn update_key(&mut self, node: &N, new_key: K) -> ResUpdateKey {
        let old_key = priority_queue::PriorityQueue::change_priority(self, node, new_key.clone())
            .expect("Failed to update key of the node, it is not present in the queue");
        if new_key < old_key {
            ResUpdateKey::Decreased
        } else {
            ResUpdateKey::Increased
        }
    }
    #[inline(always)]
    fn remove(&mut self, node: &N) -> K {
        priority_queue::PriorityQueue::remove(self, node)
            .expect("Failed to remove the node, it is not present in the queue")
            .1
    }

    fn try_decrease_key_or_push(&mut self, node: &N, key: K) -> ResTryDecreaseKeyOrPush {
        let old_key = priority_queue::PriorityQueue::push_decrease(self, node.clone(), key.clone());
        match old_key {
            None => ResTryDecreaseKeyOrPush::Pushed,
            Some(old_key) => {
                if old_key <= key {
                    ResTryDecreaseKeyOrPush::Unchanged
                } else {
                    ResTryDecreaseKeyOrPush::Decreased
                }
            }
        }
    }
}
