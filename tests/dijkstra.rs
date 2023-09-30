#[test]
fn dijkstra() {
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
    fn shortest_path(adj_list: &[Vec<Edge>], start: usize, goal: usize) -> Option<usize> {
        let mut heap = BinaryHeapWithMap::default();

        // We're at `start`, with a zero cost
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
                heap.try_decrease_key_or_push(&edge.node, &(cost + edge.cost));
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
}
