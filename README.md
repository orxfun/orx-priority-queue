# orx-priority-queue

[![orx-priority-queue crate](https://img.shields.io/crates/v/orx-priority-queue.svg)](https://crates.io/crates/orx-priority-queue)
[![orx-priority-queue documentation](https://docs.rs/orx-priority-queue/badge.svg)](https://docs.rs/orx-priority-queue)


Priority queue traits and high performance d-ary heap implementations.

## A. Priority Queue Traits

This crate aims to provide algorithms with the abstraction over priority queues. In order to achieve this, two traits are defined: **`PriorityQueue<N, K>`** and **`PriorityQueueDecKey<N, K>`**. The prior is a simple queue while the latter extends it by providing additional methods to change priorities of the items that already exist in the queue.

The separation is important since additional operations often requires the implementors to allocate internal memory for bookkeeping. Therefore, we would prefer `PriorityQueueDecKey<N, K>` only when we need to change the priorities.

See [DecreaseKey](https://github.com/orxfun/orx-priority-queue/blob/main/docs/DecreaseKey.md) section for a discussion on when decrease-key operations are required and why they are important.

## B. d-ary Heap Implementations

Three categories of d-ary heap implementations are provided.

All the heap types have a constant generic parameter `D` which defines the maximum number of children of a node in the tree. Note that d-ary heap is a generalization of the binary heap for which d=2:
* With a large d: number of per level comparisons increases while the tree depth becomes smaller.
* With a small d: each level required fewer comparisons while the tree gets deeper.

There is no dominating variant for all use cases. Binary heap is often the preferred choice due to its simplicity of implementation. However, the d-ary implementations in this crate, taking benefit of the **const generics**, provide a generalization, making it easy to switch between the variants. The motivation is to allow for tuning the heap to the algorithms and relevant input sets for performance critical methods.

### `DaryHeap<N, K, const D: usize>`

This is the basic d-ary heap implementing `PriorityQueue<N, K>`. It is to be the default choice unless priority updates or decrease-key operations are required.

### `DaryHeapOfIndices<N, K, const D>`

This is a d-ary heap paired up with a positions array and implements `PriorityQueueDecKey<N, K>`.

* It requires the nodes to implement `HasIndex` trait which is nothing but `fn index(&self) -> usize`. Note that `usize`, `u64`, etc., already implements `HasIndex`.
* Further, it requires the maximum index that is expected to enter the queue (candidates coming from a closed set).

Once these conditions are satisfied, it performs **significantly faster** than the alternative decrease key queues. Although the closed set requirement might sound strong, it is often naturally satisfied in mathematical algorithms. For instance, for most network traversal algorithms, the candidates set is the nodes of the graph, or indices in `0..numNodes`.

This is the default decrease-key queue provided that the requirements are satisfied.

### `DaryHeapWithMap<N, K, const D>`

This is a d-ary heap paired up with a positions map (`HashMap` or `BTreeMap` when no-std) and implements `PriorityQueueDecKey<N, K>`.

This is the most general decrease-key queue that provides the open-set flexibility and fits to almost all cases.

The following two types additionally implement `PriorityQueueDecKey<N, K>` which serve different purposes:

* **`DaryHeapOfIndices<N, K, const D>`** is a d-ary heap paired up with a positions array. It requires the nodes to implement `HasIndex` trait which is nothing but `fn index(&self) -> usize`. Further, it requires the maximum index that is expected to enter the queue (candidates coming from a closed set). Once these conditions are satisfied, it performs **significantly faster** than the alternative decrease key queues.
  * Although the closed set requirement might sound strong, it is often satisfied in mathematical algorithms. For instance, for most network traversal algorithms, the candidates set is the nodes of the graph.
* **`DaryHeapWithMap<N, K, const D: usize>`** is a d-ary heap paired up with a positions `HashMap` (`BTreeMap` with no-std). This provides the open-set flexibility and fits better to more general cases, rather than mathematical algorithms. 

All three variants of the d-ary heap implementations take complete benefit of const generics to speed up traversal on the heap when d is a power of two.

### Other Queues

In addition, queue implementations are provided in this crate for the following external data structures:

* `std::collections::BinaryHeap<(N, K)>` implements only `PriorityQueue<N, K>`,
* `priority_queue:PriorityQueue<N, K>` implements both `PriorityQueue<N, K>` and `PriorityQueueDecKey<N, K>`
  * requires `--features impl_priority_queue`

This allows to use all the queue implementations interchangeably and measure performance.

### Performance & Benchmarks

In scenarios in tested "src/benches", `DaryHeap` performs:
* comparable to, slightly faster than, `std::collections::BinaryHeap` for simple queue operations; and
* significantly faster than queues implementing PriorityQueueDecKey for decrease key operations.

See [Benchmarks](https://github.com/orxfun/orx-priority-queue/blob/main/docs/Benchmarks.md) section to see the experiments and observations.

## C. Examples

### C.1. Basic Usage

```rust
use orx_priority_queue::*;

// generic over simple priority queues
fn test_priority_queue<P>(mut pq: P)
where
    P: PriorityQueue<usize, f64>,
{
    pq.clear();

    pq.push(0, 42.0);
    assert_eq!(Some(&0), pq.peek().map(|x| x.node()));
    assert_eq!(Some(&42.0), pq.peek().map(|x| x.key()));

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

// generic over decrease-key priority queues
fn test_priority_queue_deckey<P>(mut pq: P)
where
    P: PriorityQueueDecKey<usize, f64>,
{
    pq.clear();

    pq.push(0, 42.0);
    assert_eq!(Some(&0), pq.peek().map(|x| x.node()));
    assert_eq!(Some(&42.0), pq.peek().map(|x| x.key()));

    let popped = pq.pop();
    assert_eq!(Some((0, 42.0)), popped);
    assert!(pq.is_empty());

    pq.push(0, 42.0);
    assert!(pq.contains(&0));

    pq.decrease_key(&0, 7.0);
    assert_eq!(Some(&0), pq.peek().map(|x| x.node()));
    assert_eq!(Some(&7.0), pq.peek().map(|x| x.key()));

    let deckey_result = pq.try_decrease_key(&0, 10.0);
    assert!(matches!(ResTryDecreaseKey::Unchanged, deckey_result));
    assert_eq!(Some(&0), pq.peek().map(|x| x.node()));
    assert_eq!(Some(&7.0), pq.peek().map(|x| x.key()));

    while let Some(popped) = pq.pop() {
        println!("pop {:?}", popped);
    }
}

// d-ary heap generic over const d
const D: usize = 4;

test_priority_queue(DaryHeap::<usize, f64, D>::default());
test_priority_queue(DaryHeapWithMap::<usize, f64, D>::default());
test_priority_queue(DaryHeapOfIndices::<usize, f64, D>::with_index_bound(100));

test_priority_queue_deckey(DaryHeapWithMap::<usize, f64, D>::default());
test_priority_queue_deckey(DaryHeapOfIndices::<usize, f64, D>::with_index_bound(100));

// or type aliases for common heaps to simplify signature
// Binary or Quarternary to fix d of d-ary
test_priority_queue(BinaryHeap::default());
test_priority_queue(BinaryHeapWithMap::default());
test_priority_queue(BinaryHeapOfIndices::with_index_bound(100));
test_priority_queue_deckey(QuarternaryHeapOfIndices::with_index_bound(100));
```

### C.2. Usage in Dijkstra's Shortest Path

You may see below two implementations one using a `PriorityQueue` and the other with a `PriorityQueueDecKey`. Please note the following:

* `PriorityQueue` and `PriorityQueueDecKey` traits enable algorithm implementations for generic queue types. Therefore we are able to implement the shortest path algorithm once that works for any specific queue implementation. This allows to benchmark and tune specific queues for specific algorithms or input families.
* The second implementation with a decrease key queue pushes a great portion of complexity, or bookkeeping, to the queue and leads to a cleaner algorithm implementation.

```rust
use orx_priority_queue::*;

// Some additional types to set up the example

type Weight = u32;

pub struct Edge {
    head: usize,
    weight: Weight,
}

pub struct Graph(Vec<Vec<Edge>>);

impl Graph {
    fn num_nodes(&self) -> usize {
        self.0.len()
    }

    fn out_edges(&self, node: usize) -> impl Iterator<Item = &Edge> {
        self.0[node].iter()
    }
}

// Implementation using a PriorityQueue

fn dijkstras_with_basic_pq<Q: PriorityQueue<usize, Weight>>(
    graph: &Graph,
    queue: &mut Q,
    source: usize,
    sink: usize,
) -> Option<Weight> {
    // reset
    queue.clear();
    let mut dist = vec![Weight::MAX; graph.num_nodes()];

    // init
    dist[source] = 0;
    queue.push(source, 0);

    // iterate
    while let Some((node, cost)) = queue.pop() {
        if node == sink {
            return Some(cost);
        } else if cost > dist[node] {
            continue;
        }

        let mut out_edges = graph.out_edges(node);
        while let Some(Edge { head, weight }) = out_edges.next() {
            let next_cost = cost + weight;
            if next_cost < dist[*head] {
                queue.push(*head, next_cost);
                dist[*head] = next_cost;
            }
        }
    }

    None
}

// Implementation using a PriorityQueueDecKey

fn dijkstras_with_deckey_pq<Q: PriorityQueueDecKey<usize, Weight>>(
    graph: &Graph,
    queue: &mut Q,
    source: usize,
    sink: usize,
) -> Option<Weight> {
    // reset
    queue.clear();
    let mut visited = vec![false; graph.num_nodes()];

    // init
    visited[source] = true;
    queue.push(source, 0);

    // iterate
    while let Some((node, cost)) = queue.pop() {
        if node == sink {
            return Some(cost);
        }

        let mut out_edges = graph.out_edges(node);
        while let Some(Edge { head, weight }) = out_edges.next() {
            if !visited[*head] {
                queue.try_decrease_key_or_push(&head, cost + weight);
            }
        }
        visited[node] = true;
    }

    None
}


// TESTS: basic priority queues

let e = |head: usize, weight: Weight| Edge { head, weight };
let graph = Graph(vec![
    vec![e(1, 4), e(2, 5)],
    vec![e(0, 3), e(2, 6), e(3, 1)],
    vec![e(1, 3), e(3, 9)],
    vec![],
]);

let mut pq = BinaryHeap::new();
assert_eq!(Some(5), dijkstras_with_basic_pq(&graph, &mut pq, 0, 3));
assert_eq!(None, dijkstras_with_basic_pq(&graph, &mut pq, 3, 1));

let mut pq = QuarternaryHeap::new();
assert_eq!(Some(5), dijkstras_with_basic_pq(&graph, &mut pq, 0, 3));
assert_eq!(None, dijkstras_with_basic_pq(&graph, &mut pq, 3, 1));

let mut pq = DaryHeap::<_, _, 8>::new();
assert_eq!(Some(5), dijkstras_with_basic_pq(&graph, &mut pq, 0, 3));
assert_eq!(None, dijkstras_with_basic_pq(&graph, &mut pq, 3, 1));

// TESTS: decrease key priority queues

let mut pq = BinaryHeapOfIndices::with_index_bound(graph.num_nodes());
assert_eq!(Some(5), dijkstras_with_deckey_pq(&graph, &mut pq, 0, 3));
assert_eq!(None, dijkstras_with_deckey_pq(&graph, &mut pq, 3, 1));

let mut pq = DaryHeapOfIndices::<_, _, 8>::with_index_bound(graph.num_nodes());
assert_eq!(Some(5), dijkstras_with_deckey_pq(&graph, &mut pq, 0, 3));
assert_eq!(None, dijkstras_with_deckey_pq(&graph, &mut pq, 3, 1));

let mut pq = BinaryHeapWithMap::new();
assert_eq!(Some(5), dijkstras_with_deckey_pq(&graph, &mut pq, 0, 3));
assert_eq!(None, dijkstras_with_deckey_pq(&graph, &mut pq, 3, 1));
```

## Contributing

Contributions are welcome! If you notice an error, have a question or think something could be improved, please open an [issue](https://github.com/orxfun/orx-priority-queue/issues/new) or create a PR.

## License

This library is licensed under MIT license. See LICENSE for details.
