//! Priority queue traits; d-ary heap implementations having binary heap as a special case.

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

mod dary;
mod has_index;
mod positions;
/// Common traits, structs and enums.
pub mod prelude;
mod priority_queue;
mod priority_queue_deckey;

pub use dary::daryheap::DaryHeap;
pub use dary::daryheap_index::DaryHeapOfIndices;
pub use dary::daryheap_map::DaryHeapWithMap;
pub use has_index::HasIndex;
pub use priority_queue::PriorityQueue;
pub use priority_queue_deckey::PriorityQueueDecKey;
