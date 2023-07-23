# orx-priority-queue
Priority queue traits; binary and generalized d-ary heap implementations.

## Traits

This crate defines two priority queue traits for (node, key) pairs with the following features:

* `PriorityQueue<N, K>`
    * `fn len(&self) -> usize;`
    * `fn is_empty(&self) -> bool;`
    * `fn is_empty(&self) -> bool;`
    * `fn peek(&self) -> Option<&(N, K)>;`
    * `fn clear(&mut self);`
    * `fn pop(&mut self) -> Option<(N, K)>;`
    * `fn push(&mut self, node: N, key: K);`
    * `fn push_then_pop(&mut self, node: N, key: K) -> (N, K);`

* `PriorityQueueDecKey<N, K>: PriorityQueue<N, K>`
    * `fn contains(&self, node: &N) -> bool;`
    * `fn key_of(&self, node: &N) -> Option<K>;`
    * `fn decrease_key(&mut self, node: &N, decreased_key: &K);`
    * `fn update_key(&mut self, node: &N, new_key: &K) -> bool;`
    * `fn try_decrease_key(&mut self, node: &N, new_key: &K) -> bool;`
    * `fn decrease_key_or_push(&mut self, node: &N, key: &K) -> bool;`
    * `fn update_key_or_push(&mut self, node: &N, key: &K) -> bool;`
    * `fn try_decrease_key_or_push(&mut self, node: &N, key: &K) -> bool`;
    * `fn remove(&mut self, node: &N) -> K;`

separating more advanced `PriorityQueueDecKey` from the basic queue since additional functionalities
are often made available through usage of additional memory.

## Implementations

### d-ary Heap

The core [d-ary heap](https://en.wikipedia.org/wiki/D-ary_heap) is implemented thanks to const generics.
Three structs are created from this core struct:

* `DaryHeap<N, K, const D: usize>` which implements `PriorityQueue<N, K>` to be preferred when the additional 
features are not required.
* `DaryHeapWithMap<N, K, const D: usize>` where `N: Hash + Equal` which is combination of the d-ary heap and a hash-map to track positions of nodes.
This might be considered as the default way to extend the heap to enable additional funcitonalities without requiring a linear search.
* `DaryHeapOfIndices<N, K, const D: usize>` where `N: HasIndex` which implements `PriorityQueueDecKey<N, K>: PriorityQueue<N, K>`.
This variant is and alternative to the hash-map extention and is particularly useful in algorithms where nodes to be enqueued are sampled from a closed set with known elements
and the size of the queue is likely to get close to total number of candidates.

## Example

```rust
fn operate_on_priority_queue<P>(mut pq: P)
where
    P: PriorityQueueDecKey<usize, f64>,
{
    pq.clear();

    pq.push(0, 42.0);
    assert_eq!(Some(&(0, 42.0)), pq.peek());

    let popped = pq.pop();
    assert_eq!(Some((0, 42.0)), pq.pop());
    assert!(pq.is_empty());

    pq.push(0, 42.0);
    assert!(pq.contains(0));

    pq.decrease_key(0, 7.0);
    assert_eq!(Some(&(0, 7.0)), pq.peek());

    let is_key_decreased = pq.try_decrease_key(0, 10.0);
    assert!(is_key_decreased);
    assert_eq!(Some(&(0, 7.0)), pq.peek());

    while let Some(popped) = pq.pop() {
        println!("pop {}", popped);
    }
}
```

## License

This library is licensed under MIT license. See LICENSE for details.
