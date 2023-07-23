use crate::{
    positions::heap_positions::{HeapPositions, HeapPositionsDecKey},
    PriorityQueue, PriorityQueueDecKey,
};

#[derive(Clone)]
pub(crate) struct Heap<N, K, P, const D: usize>
where
    N: Clone,
    K: PartialOrd + Clone,
    P: HeapPositions<N>,
{
    pub tree: Vec<(N, K)>,
    pub positions: P,
}

impl<N, K, P, const D: usize> Heap<N, K, P, D>
where
    N: Clone,
    K: PartialOrd + Clone,
    P: HeapPositions<N>,
{
    fn heapify_up(&mut self, starting_position: usize) {
        if starting_position == 0 {
            return;
        }

        let mut child = starting_position;
        let mut parent = parent_of::<D>(child);

        if self.tree[child].1 >= self.tree[parent].1 {
            return;
        }

        // take out the child node to carry upwards in the tree
        let node = self.tree[child].clone();
        let key = &node.1;

        while key < &self.tree[parent].1 {
            self.positions
                .update_position_of(&self.tree[parent].0, child);
            self.tree[child] = self.tree[parent].clone();
            child = parent;
            if child == 0 {
                break;
            }
            parent = parent_of::<D>(child);
        }

        self.positions.update_position_of(&node.0, child);
        self.tree[child] = node;
    }
    fn heapify_down(&mut self, starting_position: usize) {
        let num_nodes = self.tree.len();

        let mut parent = starting_position;
        let first_child = child_of::<D>(starting_position);
        if first_child >= num_nodes {
            return;
        }

        let mut best_child = first_child;
        let mut best_child_key = self.tree[best_child].1.clone();
        for i in 1..D {
            let next_child = first_child + i;
            if next_child >= num_nodes {
                break;
            } else if self.tree[next_child].1 < best_child_key {
                best_child = first_child + i;
                best_child_key = self.tree[next_child].1.clone();
            }
        }

        if self.tree[parent].1 <= best_child_key {
            return;
        }

        // take out the parent node to carry downwards in the tree
        let node = self.tree[parent].clone();
        let key = &node.1;

        while key > &best_child_key {
            self.positions
                .update_position_of(&self.tree[best_child].0, parent);
            self.tree[parent] = self.tree[best_child].clone();

            parent = best_child;
            let first_child = child_of::<D>(parent);
            if first_child >= num_nodes {
                break;
            }
            best_child = first_child;
            best_child_key = self.tree[best_child].1.clone();
            for i in 1..D {
                let next_child = first_child + i;
                if next_child >= num_nodes {
                    break;
                } else if self.tree[next_child].1 < best_child_key {
                    best_child = first_child + i;
                    best_child_key = self.tree[next_child].1.clone();
                }
            }
        }

        self.positions.update_position_of(&node.0, parent);
        self.tree[parent] = node;
    }
    fn remove_and_heapify(&mut self, starting_position: usize) {
        let num_nodes = self.tree.len();
        let last = num_nodes - 1;
        if num_nodes == 1 {
            debug_assert_eq!(0, starting_position);
            self.positions.remove(&self.tree[0].0);
            self.tree.clear();
        } else if starting_position == last {
            self.positions.remove(&self.tree[starting_position].0);
            self.tree.truncate(last);
        } else {
            // put last element to starting_position, and
            // drop the new-last element (originally at the starting position)
            self.positions.remove(&self.tree[starting_position].0);
            self.positions
                .update_position_of(&self.tree[last].0, starting_position);
            self.tree[starting_position] = self.tree[last].clone();
            self.tree.truncate(last);

            let key_of_disturbed = &self.tree[starting_position].1;
            if starting_position > 0
                && key_of_disturbed < &self.tree[parent_of::<D>(starting_position)].1
            {
                self.heapify_up(starting_position);
            } else {
                self.heapify_down(starting_position);
            }
        }
    }
}

const fn parent_of<const D: usize>(child: usize) -> usize {
    match D {
        2 => (child - 1) >> 1,
        4 => (child - 1) >> 2,
        8 => (child - 1) >> 3,
        16 => (child - 1) >> 4,
        _ => (child - 1) / D,
    }
}
const fn child_of<const D: usize>(parent: usize) -> usize {
    match D {
        2 => (parent << 1) + 1,
        4 => (parent << 2) + 1,
        8 => (parent << 3) + 1,
        16 => (parent << 4) + 1,
        _ => D * parent + 1,
    }
}

impl<N, K, P, const D: usize> PriorityQueue<N, K> for Heap<N, K, P, D>
where
    N: Clone,
    K: PartialOrd + Clone,
    P: HeapPositions<N>,
{
    fn len(&self) -> usize {
        self.tree.len()
    }

    fn peek(&self) -> Option<&(N, K)> {
        self.tree.first()
    }

    fn clear(&mut self) {
        self.tree.clear();
        self.positions.clear();
    }

    fn pop(&mut self) -> Option<(N, K)> {
        if self.tree.is_empty() {
            None
        } else {
            self.positions.remove(&self.tree[0].0);
            let popped = self.tree.swap_remove(0);
            self.heapify_down(0);
            Some(popped)
        }
    }
    fn pop_node(&mut self) -> Option<N> {
        if self.tree.is_empty() {
            None
        } else {
            self.positions.remove(&self.tree[0].0);
            let popped = self.tree.swap_remove(0).0;
            self.heapify_down(0);
            Some(popped)
        }
    }
    fn pop_key(&mut self) -> Option<K> {
        if self.tree.is_empty() {
            None
        } else {
            self.positions.remove(&self.tree[0].0);
            let popped = self.tree.swap_remove(0).1;
            self.heapify_down(0);
            Some(popped)
        }
    }

    fn push(&mut self, node: N, key: K) {
        let position = self.tree.len();
        self.positions.insert(&node, position);
        self.tree.push((node, key));
        self.heapify_up(position);
    }

    fn push_then_pop(&mut self, node: N, key: K) -> (N, K) {
        if self.tree.is_empty() || self.tree[0].1 >= key {
            (node, key)
        } else {
            self.positions.remove(&self.tree[0].0);
            self.positions.insert(&node, 0);
            let popped_node = self.tree[0].clone();
            self.tree[0].0 = node;
            self.tree[0].1 = key;
            self.heapify_down(0);
            popped_node
        }
    }
}

impl<N, K, P, const D: usize> PriorityQueueDecKey<N, K> for Heap<N, K, P, D>
where
    N: Clone,
    K: PartialOrd + Clone,
    P: HeapPositionsDecKey<N>,
{
    fn contains(&self, node: &N) -> bool {
        self.positions.contains(node)
    }
    fn key_of(&self, node: &N) -> Option<K> {
        self.positions
            .position_of(node)
            .map(|i| self.tree[i].1.clone())
    }
    fn decrease_key(&mut self, node: &N, decreased_key: &K) {
        let position = self
            .positions
            .position_of(node)
            .expect("cannot decrease key of a node that is not on the queue");
        assert!(
            decreased_key <= &self.tree[position].1,
            "decrease_key is called with a greater key"
        );
        self.tree[position].1 = decreased_key.clone();
        self.heapify_up(position);
    }
    fn update_key(&mut self, node: &N, new_key: &K) -> bool {
        let position = self
            .positions
            .position_of(node)
            .expect("cannot update key of a node that is not on the queue");
        let up = new_key < &self.tree[position].1;
        self.tree[position].1 = new_key.clone();
        if up {
            self.heapify_up(position);
            true
        } else {
            self.heapify_down(position);
            false
        }
    }

    fn remove(&mut self, node: &N) -> K {
        let position = self
            .positions
            .position_of(node)
            .expect("cannot remove a node that is not on the queue");
        let key_of_removed = self.tree[position].1.clone();
        self.remove_and_heapify(position);
        key_of_removed
    }
}
