use std::{mem::size_of, sync::Arc};

use polyvec::PolyVec;

#[test]
fn basic_usage() {
    let v: PolyVec = Vec::<(usize, usize)>::with_capacity(128).into();
    let g: Vec<usize> = v.try_into().unwrap();

    assert!(g.capacity() >= 256);
}

#[test]
fn non_trivial() {
    let mut v = PolyVec::new::<usize>();
    let mut g: Vec<Arc<usize>> = v.try_into().unwrap();

    g.reserve(256);
    let cap = g.capacity();
    let arc = Arc::new(0usize);
    (0..cap).for_each(|_| g.push(arc.clone()));

    assert!(cap == g.capacity());
    assert!(Arc::strong_count(&arc) == 1 + cap);
    v = g.into();
    assert!(Arc::strong_count(&arc) == 1);

    let g: Vec<usize> = v.try_into().unwrap();
    assert!(g.capacity() * size_of::<usize>() == cap * size_of::<Arc<usize>>());
}

#[test]
fn reuse_src() {
    let src_vec: Vec<_> = (0..256).collect();
    let mut poly = PolyVec::new::<&usize>();

    for i in 1..=3 {
        println!("ITERATION {i} --- ");

        let mut refs: Vec<&usize> = poly.try_into().unwrap();
        let mut prev_capacity = refs.capacity();
        let mut num_realloc = 0;

        for i in 0..src_vec.len() {
            refs.push(&src_vec[i]);
            if prev_capacity != refs.capacity() {
                println!("  Cap {prev_capacity} -> {}", refs.capacity());
                prev_capacity = refs.capacity();
                num_realloc += 1;
            }
        }

        println!(" => reallocation: {num_realloc}");
        poly = refs.into();
    }
}
