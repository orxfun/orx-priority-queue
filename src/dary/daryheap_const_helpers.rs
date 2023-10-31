use alloc::vec;
use alloc::vec::Vec;
use core::mem::MaybeUninit;

pub(crate) fn init_tree<N, K>(capacity: Option<usize>) -> Vec<(N, K)> {
    match capacity {
        Some(c) => Vec::with_capacity(c),
        None => vec![],
    }
}

/// # SAFETY
/// In order to avoid certain arithmetic operations, the first offset number of entries of the tree are skipped.
/// `tree` is a private field and none of the methods access the element at 0; hence, the offset will never be read.
/// The tree is kept private and the offset entries are never accessed.
pub(crate) unsafe fn add_offset_to_tree<N, K, const D: usize>(tree: &mut Vec<(N, K)>) {
    let offset = offset::<D>();
    for _ in 0..offset {
        let uninit_offset = MaybeUninit::<(N, K)>::uninit();
        let offset_value = uninit_offset.assume_init();
        tree.push(offset_value);
    }
}
/// when `D` = 2^k
/// * offset = 2^k-1
///
/// otherwise
/// * offset = 0
pub(crate) const fn offset<const D: usize>() -> usize {
    match D {
        2 => 1,
        4 => 3,
        8 => 7,
        16 => 15,
        32 => 31,
        64 => 63,
        _ => 0,
    }
}
/// Let c = `child`.
///
/// when `D` = 2^k
/// * parent_offset = 2^k - 2
/// * parent = c / 2^k + parent_offset
///
/// otherwise
/// * parent = (c - 1) / `D`
pub(crate) const fn parent_of<const D: usize>(child: usize) -> usize {
    match D {
        2 => child >> 1,
        4 => (child >> 2) + 2,
        8 => (child >> 3) + 6,
        16 => (child >> 4) + 14,
        32 => (child >> 5) + 30,
        64 => (child >> 6) + 62,
        _ => (child - 1) / D,
    }
}
/// Let p = `parent`.
///
/// when `D` = 2^k
/// * parent_offset = 2^k - 2
/// * left_child = p * 2^k - parent_offset * 2^k
///
/// otherwise
/// * left_child = `D` * parent + 1
pub(crate) const fn left_child_of<const D: usize>(parent: usize) -> usize {
    match D {
        2 => parent << 1,
        4 => (parent << 2) - 8,
        8 => (parent << 3) - 48,
        16 => (parent << 4) - 224,
        32 => (parent << 5) - 960,
        64 => (parent << 6) - 3968,
        _ => D * parent + 1,
    }
}
