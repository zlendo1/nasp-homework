use std::fmt::Debug;

use nasp_homework::fibheap::FibHeap;
use quickcheck_macros::*;

#[quickcheck]
fn push_maintains_peek_min(xs: Vec<u32>) {
    let mut heap = FibHeap::new();

    for (i, x) in xs.into_iter().enumerate() {
        if i % 4 == 0 {
            heap.pop();
        } else {
            let min = heap.peek().copied();
            heap.push(x);
            match min {
                Some(x_) if x > x_ => assert_eq!(min.as_ref(), heap.peek()),
                None | Some(_) => assert_eq!(Some(&x), heap.peek()),
            }
        }
    }
}

#[quickcheck]
fn counting_nodes(xs: Vec<u32>) {
    let a = xs.len();

    let mut heap = FibHeap::new();
    for x in xs {
        heap.push(x);
    }

    assert_eq!(heap.len(), a);

    heap.pop();
    assert_eq!(heap.len(), a.saturating_sub(1));
}

#[quickcheck]
fn pops_by_min(xs: Vec<u32>) {
    pops_by_min_check(xs);
}

#[quickcheck]
fn union(xs: Vec<u32>) {
    let mut heap_one = FibHeap::new();
    let mut heap_two = FibHeap::new();

    let mid = xs.len() / 2;
    let first = xs[0..mid].to_vec();
    let second = xs[mid..].to_vec();

    for x in &first {
        heap_one.push(*x);
    }

    for y in &second {
        heap_two.push(*y)
    }

    let heap = FibHeap::union(heap_one, heap_two);

    let mut comb = first.clone();
    comb.extend(second);

    comb.sort();
    comb.reverse();

    assert_heap_vec_eq(heap, comb);
}

fn pops_by_min_check(mut xs: Vec<u32>) {
    let mut heap = FibHeap::new();

    for x in &xs {
        heap.push(*x);
    }

    xs.sort();
    xs.reverse();

    assert_heap_vec_eq(heap, xs);
}

fn assert_heap_vec_eq<T: Ord + Debug>(mut heap: FibHeap<T>, mut vec: Vec<T>) {
    while let Some(b) = heap.pop() {
        let a = vec.pop();
        assert_eq!(a, Some(b), "should in pop ascending order");
    }
}
