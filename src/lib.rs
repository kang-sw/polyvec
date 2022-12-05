//!
//! This crate helps reusing allocated `Vec` memory.
//!
//! ## Example 1
//!
//! ```
//! use polyvec::*;
//!
//! let mut v: PolyVec = Vec::<(usize, usize)>::with_capacity(128).into();
//!
//! for _ in 0..10 {
//!     let g: Vec<usize> = v.try_into().unwrap();
//!
//!     assert!(g.capacity() >= 256);
//!     v = g.into();
//! }
//! ```
//!
//! ## Example 2
//!
//! ```
//! use polyvec::*;
//!
//! let src_vec: Vec<_> = (0..256).collect();
//! let mut poly = PolyVec::new::<&usize>();
//!
//! for i in 1..=3 {
//!     println!("ITERATION {i} --- ");
//!
//!     let mut refs: Vec<&usize> = poly.try_into().unwrap();
//!     let mut prev_capacity = refs.capacity();
//!     let mut num_realloc = 0;
//!
//!     for i in 0..src_vec.len() {
//!         refs.push(&src_vec[i]);
//!         if prev_capacity != refs.capacity() {
//!             println!("  Cap {prev_capacity} -> {}", refs.capacity());
//!             prev_capacity = refs.capacity();
//!             num_realloc += 1;
//!         }
//!     }
//!
//!     println!(" => reallocation: {num_realloc}");
//!     poly = refs.into();
//! }
//! ```
//!

use std::mem::size_of;

use raw_parts::RawParts;

pub struct PolyVec {
    pointer: *mut u8,
    capacity: usize,
    elem_size: usize,
}

impl PolyVec {
    pub fn new<T>() -> Self {
        Vec::<T>::new().into()
    }
}

impl<T> From<Vec<T>> for PolyVec {
    fn from(mut vec: Vec<T>) -> Self {
        vec.clear();

        let RawParts { capacity, ptr, .. } = RawParts::from_vec(vec);

        Self {
            pointer: ptr as *mut u8,
            elem_size: std::mem::size_of::<T>(),
            capacity: capacity,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PolyVecError {
    #[error("Invalid memory alignment")]
    InvalidMemAlign { expected: usize, detected: usize },
}

impl<T> TryFrom<PolyVec> for Vec<T> {
    type Error = PolyVecError;

    fn try_from(value: PolyVec) -> Result<Self, Self::Error> {
        if value.elem_size == size_of::<T>()
            || value.elem_size % size_of::<usize>() == 0 && size_of::<T>() % size_of::<usize>() == 0
        {
            unsafe {
                Ok(Self::from_raw_parts(
                    value.pointer as *mut T,
                    0,
                    value.capacity * value.elem_size / size_of::<T>(),
                ))
            }
        } else {
            Err(PolyVecError::InvalidMemAlign {
                expected: value.elem_size,
                detected: size_of::<T>(),
            })
        }
    }
}
