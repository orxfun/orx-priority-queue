use crate::priority_queue::PriorityQueue;

impl<N, K> PriorityQueue<N, K> for alloc::collections::BinaryHeap<(N, K)>
where
    K: PartialOrd + Ord,
    N: Ord,
{
    type NodeKey<'a> = &'a (N, K) where Self: 'a, N: 'a, K: 'a;
    type Iter<'a> = alloc::collections::binary_heap::Iter<'a, (N, K)> where Self: 'a, N: 'a, K: 'a;

    #[inline(always)]
    fn len(&self) -> usize {
        alloc::collections::BinaryHeap::len(self)
    }

    #[inline(always)]
    fn capacity(&self) -> usize {
        alloc::collections::BinaryHeap::capacity(self)
    }

    #[inline(always)]
    fn peek(&self) -> Option<&(N, K)> {
        alloc::collections::BinaryHeap::peek(self)
    }

    #[inline(always)]
    fn clear(&mut self) {
        alloc::collections::BinaryHeap::clear(self)
    }

    #[inline(always)]
    fn pop(&mut self) -> Option<(N, K)> {
        alloc::collections::BinaryHeap::pop(self)
    }

    #[inline(always)]
    fn pop_node(&mut self) -> Option<N> {
        alloc::collections::BinaryHeap::pop(self).map(|x| x.0)
    }

    #[inline(always)]
    fn pop_key(&mut self) -> Option<K> {
        alloc::collections::BinaryHeap::pop(self).map(|x| x.1)
    }

    #[inline(always)]
    fn push(&mut self, node: N, key: K) {
        alloc::collections::BinaryHeap::push(self, (node, key))
    }

    #[inline(always)]
    fn push_then_pop(&mut self, node: N, key: K) -> (N, K) {
        alloc::collections::BinaryHeap::push(self, (node, key));
        alloc::collections::BinaryHeap::pop(self).expect("queue cannot be empty")
    }

    #[inline(always)]
    fn iter(&self) -> Self::Iter<'_> {
        self.iter()
    }
}
