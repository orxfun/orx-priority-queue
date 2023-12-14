use crate::priority_queue::PriorityQueue;

impl<N, K> PriorityQueue<N, K> for std::collections::BinaryHeap<(N, K)>
where
    K: PartialOrd + Ord,
    N: Ord,
{
    type NodeKey<'a> = &'a (N, K) where Self: 'a, N: 'a, K: 'a;
    type Iter<'a> = std::collections::binary_heap::Iter<'a, (N, K)> where Self: 'a, N: 'a, K: 'a;

    #[inline(always)]
    fn len(&self) -> usize {
        std::collections::BinaryHeap::len(self)
    }
    #[inline(always)]
    fn capacity(&self) -> usize {
        std::collections::BinaryHeap::capacity(self)
    }
    #[inline(always)]
    fn peek(&self) -> Option<&(N, K)> {
        std::collections::BinaryHeap::peek(self)
    }
    #[inline(always)]
    fn clear(&mut self) {
        std::collections::BinaryHeap::clear(self)
    }
    #[inline(always)]
    fn pop(&mut self) -> Option<(N, K)> {
        std::collections::BinaryHeap::pop(self)
    }
    #[inline(always)]
    fn pop_node(&mut self) -> Option<N> {
        std::collections::BinaryHeap::pop(self).map(|x| x.0)
    }
    #[inline(always)]
    fn pop_key(&mut self) -> Option<K> {
        std::collections::BinaryHeap::pop(self).map(|x| x.1)
    }
    #[inline(always)]
    fn push(&mut self, node: N, key: K) {
        std::collections::BinaryHeap::push(self, (node, key))
    }
    #[inline(always)]
    fn push_then_pop(&mut self, node: N, key: K) -> (N, K) {
        std::collections::BinaryHeap::push(self, (node, key));
        std::collections::BinaryHeap::pop(self).expect("queue cannot be empty")
    }
    #[inline(always)]
    fn iter(&self) -> Self::Iter<'_> {
        self.iter()
    }
}
