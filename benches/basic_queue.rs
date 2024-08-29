use criterion::{
    black_box, criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, BenchmarkId,
    Criterion,
};
use orx_priority_queue::*;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

struct TestData {
    first_push: Vec<(usize, u64)>,
    second_push: Vec<(usize, u64)>,
}
impl TestData {
    fn new(seed: u64, n_first: usize, n_second: usize) -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);

        let mut first_push = Vec::new();
        for node in 0..n_first {
            first_push.push((node, rng.gen()));
        }

        let mut second_push = Vec::new();
        for node in n_first..(n_first + n_second) {
            second_push.push((node, rng.gen()));
        }

        Self {
            first_push,
            second_push,
        }
    }

    fn n_first_pop(&self) -> usize {
        self.first_push.len() / 5 * 4
    }
}

// data
fn run_on_basic_queue<P>(mut pq: P, data: &TestData) -> (usize, u64)
where
    P: PriorityQueue<usize, u64>,
{
    let mut sum_keys = 0;
    let mut sum_nodes = 0;

    for (node, key) in &data.first_push {
        pq.push(*node, *key);
    }

    for _ in 0..data.n_first_pop() {
        if let Some((node, key)) = pq.pop() {
            sum_nodes += node;
            sum_keys += key;
        }
    }

    for (node, key) in &data.second_push {
        pq.push(*node, *key);
    }

    while let Some((node, key)) = pq.pop() {
        sum_nodes += node;
        sum_keys += key;
    }

    (sum_nodes, sum_keys)
}

fn run_on_dary_heap<const D: usize>(
    group: &mut BenchmarkGroup<WallTime>,
    n: usize,
    data: &TestData,
) {
    group.bench_with_input(
        BenchmarkId::new(format!("DaryHeap<_, _, {}>", D), n),
        &n,
        |b, _| {
            b.iter(|| {
                let pq = DaryHeap::<_, _, D>::default();
                run_on_basic_queue(black_box(pq), black_box(data))
            })
        },
    );
}
fn bench_basic_queue(c: &mut Criterion) {
    let treatments = vec![100_000];

    let mut group = c.benchmark_group("basic_queue");

    for n in &treatments {
        let data = TestData::new(8498723, *n, *n);

        group.bench_with_input(
            BenchmarkId::new("std::collections::BinaryHeap", n),
            n,
            |b, _| {
                b.iter(|| {
                    let pq = std::collections::BinaryHeap::default();
                    run_on_basic_queue(black_box(pq), black_box(&data))
                })
            },
        );

        run_on_dary_heap::<2>(&mut group, *n, &data);
        run_on_dary_heap::<4>(&mut group, *n, &data);
        run_on_dary_heap::<8>(&mut group, *n, &data);

        #[cfg(feature = "impl_priority_queue")]
        {
            group.bench_with_input(
                BenchmarkId::new("priority_queue::PriorityQueue", n),
                n,
                |b, _| {
                    b.iter(|| {
                        let pq = priority_queue::PriorityQueue::default();
                        run_on_basic_queue(black_box(pq), black_box(&data))
                    })
                },
            );
        }
    }

    group.finish();
}

criterion_group!(benches, bench_basic_queue);
criterion_main!(benches);
