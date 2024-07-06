# Benchmarks

In scenarios in tested "src/benches", `DaryHeap` performs comparable to (slightly faster than) `std::collections::BinaryHeap` and significantly faster than decrease key variants.

For the cases where we need decrease key operations, `DaryHeapOfIndices` performs significantly faster than other tested priority queues.

## Basic Queue Operations

This benchmark compares basic push & pop operations on different `PriorityQueue` implementations. See "benches/basic_queue.rs" for details. The computation times in µs for different test data sizes are presented in the following table.

| PriorityQueue / Data Size             |  1,000 |   10,000 |   100,000 |
|---------------------------------------|-------:|---------:|----------:|
| orx_priority_queue::DaryHeap<_, _, 2> |  66.76 |   972.52 | 19,102.00 |
| orx_priority_queue::DaryHeap<_, _, 4> |  **55.16** |   **719.96** | **11,625.00** |
| orx_priority_queue::DaryHeap<_, _, 8> |  65.13 |   957.14 | 14,532.00 |
| std::collections::BinaryHeap          |  75.94 | 1,117.30 | 14,388.00 |
| priority_queue::PriorityQueue         | 252.21 | 3,612.80 | 62,637.00 |

* `DaryHeap` implementations with different d-values and `std::collections::BinaryHeap` perform at a similar scale.
  * Among these, `DaryHeap<_, _, 4>` performs consistently faster.
* `priority_queue::PriorityQueue` is around five times slower.

## Decrease Key Operations

In the next benchmark, different `PriorityQueueDecKey` implementations are tested on a benchmark where decrease key calls are made in addition to push & pop operations. See "benches/deckey_queue.rs" for details. The computation times in µs for different test data sizes are presented in the following table. Since `std::collections::BinaryHeap` does not provide the required methods it is excluded from the analysis.

| PriorityQueueDecKey / Data Size                |  1,000 |   10,000 |   100,000 |
|------------------------------------------------|-------:|---------:|----------:|
| orx_priority_queue::DaryHeapOfIndices<_, _, 2> |  45.56 |   787.09 | 15,060.00 |
| orx_priority_queue::DaryHeapOfIndices<_, _, 4> |  **36.19** |   **592.22** |  **9,865.20** |
| orx_priority_queue::DaryHeapOfIndices<_, _, 8> |  39.02 |   698.46 | 10,919.00 |
| orx_priority_queue::DaryHeapWithMap<_, _, 2>   | 327.13 | 4,188.90 | 66,162.00 |
| orx_priority_queue::DaryHeapWithMap<_, _, 4>   | 249.46 | 3,043.60 | 44,246.00 |
| orx_priority_queue::DaryHeapWithMap<_, _, 8>   | 217.34 | 2,647.20 | 39,115.00 |
| priority_queue::PriorityQueue                  | 162.85 | 2,321.00 | 37,249.00 |

* There are three clusters with respect to performance:
  * `DaryHeapOfIndices` is significantly faster than others (~5 times faster than `DaryHeapWithMap` and ~3.5 times faster than `priority_queue::PriorityQueue`),
  * `priority_queue::PriorityQueue` is around 1.5 times faster than `DaryHeapWithMap`, and
  * `DaryHeapWithMap` is the slowest.
* Among all variants, `DaryHeapOfIndices<_, _, 4>` consistently outperforms the others.
