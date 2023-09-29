# orx-priority-queue
Priority queue traits, d-ary heap implementations having binary heap as a special case.

## Traits

This crate defines two priority queue traits for (node, key) pairs with the following features:

* [`PriorityQueue<N, K>`]: providing basic priority queue functionalities.
* [`PriorityQueueDecKey<N, K>`]: adds super powers which is achieved by being able to locate positions of nodes that already exists on the heap.

Separating more advanced `PriorityQueueDecKey` from the basic queue is due to the fact that additional functionalities are often made available through usage of additional memory.

### Benefits of `PriorityQueueDecKey`

Decrease-key, and related operations, are critical for certain algorithms where the key of a particular node is evaluated multiple times. Without the ability to update keys of nodes on the heap, space complexity of correspoinding algorithms
increases exponentially.

Consider Dijkstra's shortest-path algorithm for instance. Space complexity of the algorithm would be *O(n^2)* with a `PriorityQueue` where *n* is the number of nodes on the graph. This is due to the fact that label of each node might be evaluated *n-1* times and consequently each node can be pushed to the queue *n-1* times. As also noted in `std::collections::BinaryHeap` documentation, [*this implementation isn't memory-efficient as it may leave duplicate nodes in the queue.*](https://doc.rust-lang.org/stable/std/collections/binary_heap/index.html)

On the other hand, using a `PriorityQueueDecKey`, space complexity of the algorithm will be kept as *O(n)*: each node will enter the queue at most once; consequent evaluations of its label will be handled by decrease key operation.

Furthermore, the additional functionalities simplify the algorithm implementation pushing some of the complexity to the data structure. This becomes clear when the following `shortest_path` implementation is compared to the corresponding `std::collections::BinaryHeap` example. Note that it is almost a drop-dead substitution while providing a better space complexity, generic dary-heap options and a cleaner algorithm implementation.

```rust
use orx_priority_queue::*;

// Each node is represented as a `usize`, for a shorter implementation.
struct Edge {
    node: usize,
    cost: usize,
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeapWithMap::default();

    // We're at `start`, with a zero cost
    dist[start] = 0;
    heap.push(start, 0);

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some((position, cost)) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal {
            return Some(cost);
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[position] {
            let next_cost = cost + edge.cost;
            if heap.try_decrease_key_or_push(&edge.node, &next_cost) {
                dist[edge.node] = next_cost; // we found a shorter path
            }
        }
    }

    // Goal not reachable
    None
}

// This is the directed graph we're going to use.
// The node numbers correspond to the different states,
// and the edge weights symbolize the cost of moving
// from one node to another.
// Note that the edges are one-way.
//
//                  7
//          +-----------------+
//          |                 |
//          v   1        2    |  2
//          0 -----> 1 -----> 3 ---> 4
//          |        ^        ^      ^
//          |        | 1      |      |
//          |        |        | 3    | 1
//          +------> 2 -------+      |
//           10      |               |
//                   +---------------+
//
// The graph is represented as an adjacency list where each index,
// corresponding to a node value, has a list of outgoing edges.
// Chosen for its efficiency.
let graph = vec![
    // Node 0
    vec![Edge { node: 2, cost: 10 }, Edge { node: 1, cost: 1 }],
    // Node 1
    vec![Edge { node: 3, cost: 2 }],
    // Node 2
    vec![
        Edge { node: 1, cost: 1 },
        Edge { node: 3, cost: 3 },
        Edge { node: 4, cost: 1 },
    ],
    // Node 3
    vec![Edge { node: 0, cost: 7 }, Edge { node: 4, cost: 2 }],
    // Node 4
    vec![],
];

assert_eq!(shortest_path(&graph, 0, 1), Some(1));
assert_eq!(shortest_path(&graph, 0, 3), Some(3));
assert_eq!(shortest_path(&graph, 3, 0), Some(7));
assert_eq!(shortest_path(&graph, 0, 4), Some(5));
assert_eq!(shortest_path(&graph, 4, 0), None);
```

## Implementations

### d-ary heap

The core [d-ary heap](https://en.wikipedia.org/wiki/D-ary_heap) is implemented thanks to const generics.
Three structs are created from this core struct:

* [`DaryHeap<N, K, const D: usize>`] which implements `PriorityQueue<N, K>` to be preferred when the additional 
features are not required.
* [`DaryHeapWithMap<N, K, const D: usize>`] where `N: Hash + Equal` implements `PriorityQueueDecKey<N, K>`.
It is a combination of the d-ary heap and a hash-map to track positions of nodes.
This might be considered as the default way to extend the heap to enable additional funcitonalities without requiring a linear search.
* [`DaryHeapOfIndices<N, K, const D: usize>`] where `N: HasIndex` implements `PriorityQueueDecKey<N, K>`.
This variant is and alternative to the hash-map implementation and is particularly useful in algorithms where nodes to be enqueued are sampled from a closed set with known elements and the size of the queue is likely to get close to total number of candidates.

### Special traversal for d=2: binary-heap

const generics further allows to use special arithmetics for the special case where d=2; i.e.,
when d-ary heap is the binary heap.
In particular, one addition/subtraction is avoided during the traversal through the tree.

However, overall performance of the queues depends on the use case,
ratio of push an decrease-key operations, etc.
Benchmarks will follow.


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
}clear

use orx_priority_queue::*;

// d of the d-ary heap
const D: usize = 4;

test_priority_queue(DaryHeap::<usize, f64, D>::default());
test_priority_queue(DaryHeapWithMap::<usize, f64, D>::default());
test_priority_queue(DaryHeapOfIndices::<usize, f64, D>::with_upper_limit(100));

test_priority_queue_deckey(DaryHeapWithMap::<usize, f64, D>::default());
test_priority_queue_deckey(DaryHeapOfIndices::<usize, f64, D>::with_upper_limit(100));
```

## License

This library is licensed under MIT license. See LICENSE for details.
