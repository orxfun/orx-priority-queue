# Why Decrease Key?

Decrease key operations are critical in certain cases.

Assume a scenario where the same candidate node might be evaluated multiple times with different priorities throughout the life of the queue. Just as in Dijkstra's shortest path algorithm.

We can handle this with a basic `PriorityQueue` as follows:

* every time a node is observed as a candidate with a given priority, we push it again to the priority queue;
* this means that we will pop the same node multiple times with priorities in worsening order;
* we additionally keep track of each node that is popped from the queue;
* when we pop the same node the second or third time, we simply ignore it.

This approach might work well in many cases. However, it is not the most memory efficient solution. If we assume that the algorithm using the queue is the Dijkstra's shortest path, and if our graph has N nodes, each node can enter the queue N times in the worst case. This would lead to a space complexity of O(N^2).

On the other hand, with a `PriorityQueueDecKey`, each time we observe a candidate with a priority:

* we can push the (node, priority) if it doesn't exist in the queue,
* we can decrease its priority on the queue if it exists in the queue with a worse priority,
* or lastly, we can ignore the candidate if the node already exists in the queue with a better priority.

This approach would guarantee that each node enters the queue at most once. It would reduce the space complexity of the Dijkstra's shortest path to O(N). Another benefit of this approach is to push some part of the book keeping to the queue allowing the algorithm to be more concise. For instance, `try_decrease_key_or_push` method removes almost all book keeping from Dijkstra's shortest path algorithm.

However, there is not a clear winner. Most performant queue often depends on the input data and can be decided empirically. As mentioned, allowing these experiments and benchmarks is one of the key motivations behind the priority queue traits. You may see such an exercise in the repository [https://github.com/orxfun/orx-bench-shortest-path](https://github.com/orxfun/orx-bench-shortest-path).
