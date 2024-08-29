# Why Decrease Key?

Decrease key operations are critical in certain cases.

Assume a scenario where the same candidate node might be evaluated multiple times with different priorities throughout the life of the queue. Just as in Dijkstra's shortest path algorithm.

## Handle with a simple queue

We can handle this with a basic `PriorityQueue` as follows:

* every time a node is observed as a candidate with a given priority, we push it again to the priority queue;
* this means that we will pop the same node multiple times with priorities in worsening order;
* we additionally keep track of each node that is popped from the queue;
* when we pop the same node the second or third time, we simply ignore it.

This approach might work well in many cases. However, it is not the most memory efficient solution.

For instance, for the Dijkstra's shortest path algorithm where our graph has N nodes, each node can enter the queue N times in the worst case. This would lead to a space complexity of O(N^2).

## Handle using decrease key operations

On the other hand, with a `PriorityQueueDecKey`, each time we evaluate a candidate with an observed priority:

1. we push the (node, priority) pair if the node does not exist in the queue,
2. we decrease its priority on the queue if it exists in the queue with a worse priority,
3. or lastly, we ignore the candidate if the node already exists in the queue with a better priority.

This approach would guarantee that each node enters the queue at most once. It would reduce the space complexity of the Dijkstra's shortest path to O(N).

Another benefit of this approach is to push some part of the bookkeeping to the queue allowing the algorithm to be more concise. For this, [`try_decrease_key_or_push`](https://docs.rs/orx-priority-queue/latest/orx_priority_queue/trait.PriorityQueueDecKey.html#method.decrease_key_or_push) method becomes very handy in Dijkstra's leading to a very clean and efficient implementation.

```rust ignore
while let Some((node, cost)) = queue.pop() {
    if node == sink {
        return Some(cost);
    }

    for Edge { head, weight } in graph.out_edges(node) {
        if !visited[*head] {
            queue.try_decrease_key_or_push(&head, cost + weight);
        }
    }
    visited[node] = true;
}
```

Performance-wise, there is not a clear winner. When memory is critical, it would clearly be beneficial to use a `PriorityQueueDecKey`.

In other cases, the most performant queue often depends on the input data and can be decided empirically. As mentioned, allowing these experiments and benchmarks is one of the key motivations behind the priority queue traits. You may see such an exercise in the repository [https://github.com/orxfun/orx-bench-shortest-path](https://github.com/orxfun/orx-bench-shortest-path).
