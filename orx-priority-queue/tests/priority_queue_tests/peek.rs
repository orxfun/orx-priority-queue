use orx_priority_queue::PriorityQueue;

pub fn test_peek<P>(mut pq: P)
where
    P: PriorityQueue<usize, f64>,
{
    pq.clear();
    assert_eq!(None, pq.peek());

    pq.push(1, 2.0);
    assert_eq!(Some(&(1, 2.0)), pq.peek());

    pq.push(2, 3.0);
    assert_eq!(Some(&(1, 2.0)), pq.peek());

    pq.push(3, 1.0);
    assert_eq!(Some(&(3, 1.0)), pq.peek());

    pq.pop();
    assert_eq!(Some(&(1, 2.0)), pq.peek());

    pq.pop();
    assert_eq!(Some(&(2, 3.0)), pq.peek());

    pq.pop();
    assert_eq!(None, pq.peek());
}
