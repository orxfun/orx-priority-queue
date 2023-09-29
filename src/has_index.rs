/// A struct which provides an index of type [usize].
/// Index of the struct can be considered its unchanging id
/// defined its position in a collection.
///
/// For instance consider a tour generation problem for cities in the collection {X, Y, Z}.
/// Provided that this collection does not change, the cities can be identified by their indices:
/// X => 0, Y => 1, Z => 2.
///
/// In certain algorithms, as well as, in certain priority queue implementations in this crate,
/// this convention allows to replace a hashmap with an array.
/// This may be useful in simplifying the algorithms and improving performance when
/// the elements entering the queue are sampled from a closed and known set, as the cities above.
pub trait HasIndex: Clone {
    /// Returns the index of the element.
    fn index(&self) -> usize;
}

impl HasIndex for usize {
    fn index(&self) -> usize {
        *self
    }
}
impl HasIndex for u64 {
    fn index(&self) -> usize {
        *self as usize
    }
}
impl HasIndex for u32 {
    fn index(&self) -> usize {
        *self as usize
    }
}
impl HasIndex for u16 {
    fn index(&self) -> usize {
        *self as usize
    }
}
impl HasIndex for u8 {
    fn index(&self) -> usize {
        *self as usize
    }
}
