#![doc = include_str!("../README.md")]
#![warn(
    missing_docs,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::float_cmp,
    clippy::float_cmp_const,
    clippy::missing_panics_doc,
    clippy::todo
)]
#![no_std]

#[cfg(any(test, feature = "std"))]
extern crate std;

extern crate alloc;

mod dary;
mod has_index;
mod impl_queues;
mod node_key_ref;
mod positions;
mod priority_queue;
mod priority_queue_deckey;

pub use crate::priority_queue::PriorityQueue;
pub use dary::daryheap::{BinaryHeap, DaryHeap, QuaternaryHeap};
pub use dary::daryheap_index::{BinaryHeapOfIndices, DaryHeapOfIndices, QuaternaryHeapOfIndices};
pub use dary::daryheap_map::{BinaryHeapWithMap, DaryHeapWithMap, QuaternaryHeapWithMap};
pub use has_index::HasIndex;
pub use node_key_ref::NodeKeyRef;
pub use priority_queue_deckey::{
    PriorityQueueDecKey, ResDecreaseKeyOrPush, ResTryDecreaseKey, ResTryDecreaseKeyOrPush,
    ResUpdateKey, ResUpdateKeyOrPush,
};
