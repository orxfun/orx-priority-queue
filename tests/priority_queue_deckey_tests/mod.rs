mod change_key;
mod change_key_or_push;
mod contains;
mod key_of;
mod mixed;
mod remove;

pub use change_key::{test_change_key, ChangeKeyMethod};
pub use change_key_or_push::test_change_key_or_push;
pub use contains::test_contains;
pub use key_of::test_key_of;
pub use mixed::test_mixed;
pub use remove::test_remove;
