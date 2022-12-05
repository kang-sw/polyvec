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
