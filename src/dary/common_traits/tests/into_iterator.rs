use crate::{DaryHeap, DaryHeapOfIndices, DaryHeapWithMap, PriorityQueue};
use orx_iterable::Collection;

const LEN: usize = 100;

fn validate(queue: impl Collection<Item = (usize, i64)> + IntoIterator<Item = (usize, i64)>) {
    let mut values: Vec<_> = queue.iter().cloned().collect();
    values.sort();
    for (i, (a, b)) in values.into_iter().enumerate() {
        assert_eq!(i, a);
        assert_eq!(i, b as usize);
    }

    let mut values: Vec<_> = queue.into_iter().collect();
    values.sort();
    for (i, (a, b)) in values.into_iter().enumerate() {
        assert_eq!(i, a);
        assert_eq!(i, b as usize);
    }
}

#[test]
fn dary_heap_into_iterator() {
    let mut heap = DaryHeap::<_, _, 4>::new();

    for i in 0..LEN {
        heap.push(LEN - 1 - i, (LEN - 1 - i) as i64);
    }

    validate(heap);
}

#[test]
fn dary_heap_index_into_iterator() {
    let mut heap = DaryHeapOfIndices::<_, _, 4>::with_index_bound(LEN);

    for i in 0..LEN {
        heap.push(LEN - 1 - i, (LEN - 1 - i) as i64);
    }

    validate(heap);
}

#[test]
fn dary_heap_map_into_iterator() {
    let mut heap = DaryHeapWithMap::<_, _, 4>::with_capacity(LEN);

    for i in 0..LEN {
        heap.push(LEN - 1 - i, (LEN - 1 - i) as i64);
    }

    validate(heap);
}
