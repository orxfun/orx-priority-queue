use criterion::{
    black_box, criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, BenchmarkId,
    Criterion,
};
use orx_priority_queue::*;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

struct TestData {
    push: Vec<(usize, u64)>,
    first_deckey: Vec<(usize, u64)>,
    second_deckey: Vec<(usize, u64)>,
}
impl TestData {
    fn new(seed: u64, n_push: usize, n_deckey1: usize, n_deckey2: usize) -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);

        let mut push = Vec::new();
        for node in 0..n_push {
            push.push((node, rng.random()));
        }

        let mut first_deckey = Vec::new();
        for _ in 0..n_deckey1 {
            let node = rng.random_range(0..n_push);
            first_deckey.push((node, rng.random()));
        }

        let mut second_deckey = Vec::new();
        for _ in 0..n_deckey2 {
            let node = rng.random_range(0..n_push);
            second_deckey.push((node, rng.random()));
        }

        Self {
            push,
            first_deckey,
            second_deckey,
        }
    }

    fn n_first_pop(&self) -> usize {
        self.push.len() / 2
    }
}

// data
fn run_on_deckey_queue<P>(mut pq: P, data: &TestData) -> (usize, u64)
where
    P: PriorityQueueDecKey<usize, u64>,
{
    let mut sum_keys = 0;
    let mut sum_nodes = 0;

    for (node, key) in &data.push {
        pq.push(*node, *key);
    }

    for (node, key) in &data.first_deckey {
        _ = pq.try_decrease_key_or_push(node, *key);
    }

    for _ in 0..data.n_first_pop() {
        if let Some((node, key)) = pq.pop() {
            sum_nodes += node;
            sum_keys += key;
        }
    }

    for (node, key) in &data.second_deckey {
        _ = pq.try_decrease_key_or_push(node, *key);
    }

    while let Some((node, key)) = pq.pop() {
        sum_nodes += node;
        sum_keys += key;
    }

    (sum_nodes, sum_keys)
}

fn run_on_dary_heap_of_indices<const D: usize>(
    group: &mut BenchmarkGroup<WallTime>,
    n: usize,
    data: &TestData,
) {
    group.bench_with_input(
        BenchmarkId::new(format!("DaryHeapOfIndices<_, _, {}>", D), n),
        &n,
        |b, _| {
            b.iter(|| {
                let pq = DaryHeapOfIndices::<_, _, D>::with_index_bound(n);
                run_on_deckey_queue(black_box(pq), black_box(data))
            })
        },
    );
}
fn run_on_dary_heap_with_map<const D: usize>(
    group: &mut BenchmarkGroup<WallTime>,
    n: usize,
    data: &TestData,
) {
    group.bench_with_input(
        BenchmarkId::new(format!("DaryHeapWithMap<_, _, {}>", D), n),
        &n,
        |b, _| {
            b.iter(|| {
                let pq = DaryHeapWithMap::<_, _, D>::with_capacity(n);
                run_on_deckey_queue(black_box(pq), black_box(data))
            })
        },
    );
}

fn bench_deckey_queue(c: &mut Criterion) {
    let treatments = vec![1_000, 10_000, 100_000];

    let mut group = c.benchmark_group("deckey_queue");

    for n in &treatments {
        let data = TestData::new(8498723, *n, n / 2, n / 2);

        run_on_dary_heap_of_indices::<2>(&mut group, *n, &data);
        run_on_dary_heap_of_indices::<4>(&mut group, *n, &data);
        run_on_dary_heap_of_indices::<8>(&mut group, *n, &data);

        run_on_dary_heap_with_map::<2>(&mut group, *n, &data);
        run_on_dary_heap_with_map::<4>(&mut group, *n, &data);
        run_on_dary_heap_with_map::<8>(&mut group, *n, &data);

        #[cfg(feature = "impl_priority_queue")]
        {
            group.bench_with_input(
                BenchmarkId::new("priority_queue::PriorityQueue", n),
                n,
                |b, _| {
                    b.iter(|| {
                        let pq = priority_queue::PriorityQueue::with_capacity(*n);
                        run_on_deckey_queue(black_box(pq), black_box(&data))
                    })
                },
            );
        }
    }

    group.finish();
}

criterion_group!(benches, bench_deckey_queue);
criterion_main!(benches);
