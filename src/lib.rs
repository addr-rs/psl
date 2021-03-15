//! A native Rust library for Mozilla's Public Suffix List

#![no_std]
#![forbid(unsafe_code)]

pub use psl_types::{Domain, Info, List as Psl, Suffix, Type};

mod list;

/// A static public suffix list
pub struct List;

impl Psl for List {
    #[inline]
    fn find<'a, T>(&self, labels: T) -> Info
    where
        T: Iterator<Item = &'a [u8]>,
    {
        list::lookup(labels)
    }
}
