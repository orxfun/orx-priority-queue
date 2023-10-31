# orx-priority-queue

This crate aims to address the following:

1. To provide priority queue traits for two goals:
    * to separate simple and decrease key queues, and
    * to enable developing functions or algorithms which are generic over the actual queue type.
2. To provide d-ary heap implementations, which is generic over `d`:
    * the implementation makes use of the const-generics which conveniently allows to take advantage of bit operations for the cases where `d` is a power of two, while having a general implementation for others.


## Traits

This crate defines two priority queue traits for (node, key) pairs with the following features:

* `PriorityQueue<N, K>`: providing basic priority queue functionalities.
* `PriorityQueueDecKey<N, K>`: extends the `PriorityQueue<N, K>` by allowing methods to mutate keys of existing items.

The differences can be summarized as follows:

* A decrease key queue is also a simple priority queue; with additonal operations such as `decrease_key` and `update_key`, etc.
* A simple queue has a lower initial memory requirement; while the decrease key queues need an additional internal data structure to keep track of positions.
* On the other hand, a decrease key queue provides a better space complexity than a simple queue in many algorithms.
    * For instance, a Dijkstra's shortest path algorithm implementation with a simple queue has a space complexity of O(n^2) due to the fact that every node can enter the node n times, where n is the number of nodes.
    * However, the same algorithm using a decrease key queue requires a space complexity of O(n) since each node will enter the queue at most once, and its key will be updated if the same node is observed more than once.

## Choice of the Queue

This crate provides three kinds of queue implementations. You may find below their differences and the situations each could be a better fit.

* `OrxDaryHeap` is a simple `PriorityQueue` implemented as a heap.
    * Benefits from its simplicity.
    * Could be considered as the default queue implementation where the memory efficiency is not very critical.
* `OrxDaryHeapOfIndices` is a `PriorityQueueDecKey` which aims to be performant while requiring the items that can enter the queue to implement `HasIndex` trait, which simply has the single method `fn index(&self) -> usize`. The implementation is a heap parallel to a mapping of items to positions on the heap.
    * Particularly useful in performance critical algorithms which additionally requires memory efficiency.
    * Requires that the size of the closed set of candidates that can enter the queue is known. This is not a strong limitation in many algorithms. For instance, for most network traversal algorithms, this equals to the number of nodes in the graph.
* `OrxDaryHeapWithMap` is also a `PriorityQueueDecKey` which aims to be flexible requiring the items to implement `Hash + Eq`. It has a similar implementation with the prior except that it utilizes a map to keep track of item positions on the heap.
    * This can be considered as the alternative to `OrxDaryHeapOfIndices` in memory critical situations, where it is not possible to satisfy the requirement on knowing the size of the closed set of candidates.

Note that each of the above kinds is generic over `d`, leading to a large number of possible queue choices. The traits come handy here allowing to write algorithms generic over the queue, which then can be benchmarked with different concrete types to find the most performant implementation for the problem. In the next section, we summarize such an experiment.

### Experiments on Shortest Path Algorithm

Using the traits, we can write algorithms which are generic over the queues. This allows to conveniently benchmark over the relevant data to find the best fitting queue implementation. You may see such an example exercise in the repository [https://github.com/orxfun/orx-bench-shortest-path](https://github.com/orxfun/orx-bench-shortest-path) which allows to compare different shortest path algorithm implementations. From our current experiments using random and real life networks, the following table can be consulted.

|                    |     |     | Queue Trait             | Queue Type               |
|--------------------|-----|-----|-------------------------|--------------------------|
| Memory Critical?   | yes |     |  `PriorityQueueDecKey`  |                          |
| - Is Graph Sparse? |     | yes |                         |  `OrxDaryHeapOfIndices`  |
|                    |     | no  |                         |  `OrxDaryHeapOfIndices`  |
|.                   |     |     |                         |                          |
| Memory Critical?   | no  |     |  `PriorityQueue`        |                          |
| - Is Graph Sparse? |     | yes |                         |  `OrxDaryHeap`           |
|                    |     | no  |                         |  `OrxDaryHeap` or `std::collections::BinaryHeap` |

## Example

```rust
use orx_priority_queue::*;

fn test_priority_queue<P>(mut pq: P)
where
    P: PriorityQueue<usize, f64>,
{
    println!("\ntest_priority_queue");
    pq.clear();

    pq.push(0, 42.0);
    assert_eq!(Some(&(0, 42.0)), pq.peek());

    let popped = pq.pop();
    assert_eq!(Some((0, 42.0)), popped);
    assert!(pq.is_empty());

    pq.push(0, 42.0);
    pq.push(1, 7.0);
    pq.push(2, 24.0);
    pq.push(10, 3.0);

    while let Some(popped) = pq.pop() {
        println!("pop {:?}", popped);
    }
}
fn test_priority_queue_deckey<P>(mut pq: P)
where
    P: PriorityQueueDecKey<usize, f64>,
{
    println!("\ntest_priority_queue_deckey");
    pq.clear();

    pq.push(0, 42.0);
    assert_eq!(Some(&(0, 42.0)), pq.peek());

    let popped = pq.pop();
    assert_eq!(Some((0, 42.0)), popped);
    assert!(pq.is_empty());

    pq.push(0, 42.0);
    assert!(pq.contains(&0));

    pq.decrease_key(&0, &7.0);
    assert_eq!(Some(&(0, 7.0)), pq.peek());

    let is_key_decreased = pq.try_decrease_key(&0, &10.0);
    assert!(!is_key_decreased);
    assert_eq!(Some(&(0, 7.0)), pq.peek());

    while let Some(popped) = pq.pop() {
        println!("pop {:?}", popped);
    }
}

// d-ary heap generic over const d
const D: usize = 4;

test_priority_queue(DaryHeap::<usize, f64, D>::default());
test_priority_queue(DaryHeapWithMap::<usize, f64, D>::default());
test_priority_queue(DaryHeapOfIndices::<usize, f64, D>::with_upper_limit(100));

test_priority_queue_deckey(DaryHeapWithMap::<usize, f64, D>::default());
test_priority_queue_deckey(DaryHeapOfIndices::<usize, f64, D>::with_upper_limit(100));

// or type aliases for common heaps to simplify signature
// Binary, Ternary or Quarternary to fix D of Dary
test_priority_queue(BinaryHeap::default());
test_priority_queue(BinaryHeapWithMap::default());
test_priority_queue(BinaryHeapOfIndices::with_upper_limit(100));
test_priority_queue_deckey(BinaryHeapWithMap::default());
test_priority_queue_deckey(BinaryHeapOfIndices::with_upper_limit(100));

test_priority_queue(TernaryHeap::default());
test_priority_queue(TernaryHeapWithMap::default());
test_priority_queue(TernaryHeapOfIndices::with_upper_limit(100));
test_priority_queue_deckey(TernaryHeapWithMap::default());
test_priority_queue_deckey(TernaryHeapOfIndices::with_upper_limit(100));

test_priority_queue(QuarternaryHeap::default());
test_priority_queue(QuarternaryHeapWithMap::default());
test_priority_queue(QuarternaryHeapOfIndices::with_upper_limit(100));
test_priority_queue_deckey(QuarternaryHeapWithMap::default());
test_priority_queue_deckey(QuarternaryHeapOfIndices::with_upper_limit(100));
```

## License

This library is licensed under MIT license. See LICENSE for details.
