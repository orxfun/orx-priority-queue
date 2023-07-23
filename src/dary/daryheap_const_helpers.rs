use std::mem::MaybeUninit;

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
pub(crate) const fn offset<const D: usize>() -> usize {
    match D {
        2 => 1,
        _ => 0,
    }
}
pub(crate) const fn parent_of<const D: usize>(child: usize) -> usize {
    match D {
        2 => child >> 1,
        4 => (child - 1) >> 2,
        8 => (child - 1) >> 3,
        16 => (child - 1) >> 4,
        _ => (child - 1) / D,
    }
}
pub(crate) const fn child_of<const D: usize>(parent: usize) -> usize {
    match D {
        2 => parent << 1,
        4 => (parent << 2) + 1,
        8 => (parent << 3) + 1,
        16 => (parent << 4) + 1,
        _ => D * parent + 1,
    }
}
