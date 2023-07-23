use orx_priority_queue::PriorityQueue;
use rand::prelude::*;

pub fn test_push_then_pop<P>(mut pq: P)
where
    P: PriorityQueue<usize, f64>,
{
    // init
    pq.clear();
    assert_eq!(0, pq.len());

    // when empty
    let popped = pq.push_then_pop(0, 10.0);
    assert_eq!((0, 10.0), popped);

    // when better than the only item
    pq.push(1, 20.0);
    let popped = pq.push_then_pop(0, 10.0);
    assert_eq!((0, 10.0), popped);

    // when worse than the only item
    pq.clear();
    pq.push(1, 5.0);
    let popped = pq.push_then_pop(0, 10.0);
    assert_eq!((1, 5.0), popped);
    assert_eq!(Some(&(0, 10.0)), pq.peek());

    // when better than all items
    pq.clear();
    for i in 1..10 {
        pq.push(i, 100.0 - i as f64);
    }
    let popped = pq.push_then_pop(0, 10.0);
    assert_eq!((0, 10.0), popped);

    // when worse than the best item
    let popped = pq.push_then_pop(0, 100.0);
    assert_ne!((0, 10.0), popped);
}

pub fn test_push_then_pop_randomized<P>(mut pq: P)
where
    P: PriorityQueue<usize, f64>,
{
    const N: usize = 50;

    // fill it up
    pq.clear();
    assert!(pq.is_empty());

    let mut rng = rand::thread_rng();
    for node in 0..N {
        let priority = rng.gen();
        pq.push(node, priority);
    }
    let mut pq_pll = pq.clone();

    // push & pull randomly
    for node in N..2 * N {
        let key = rng.gen();

        pq_pll.push(node, key);
        let popped_pll = pq_pll.pop().unwrap();

        let popped = pq.push_then_pop(node, key);

        assert_eq!(popped_pll, popped);
    }
    assert_eq!(pq.len(), N);

    // pop remaining in correct order
    while let Some(popped) = pq.pop() {
        let popped_pll = pq_pll.pop().unwrap();
        assert_eq!(popped_pll, popped);
    }
    assert!(pq_pll.is_empty());
}
