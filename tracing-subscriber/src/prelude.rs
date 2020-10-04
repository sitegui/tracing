//! The `tracing-subscriber` prelude.
//!
//! This brings into scope a number of extension traits that define methods on
//! types defined here and in other crates.

pub use crate::field::{
    MakeExt as __tracing_subscriber_field_MakeExt,
    RecordFields as __tracing_subscriber_field_RecordFields,
};
pub use crate::layer::{
    Subscriber as __tracing_subscriber_Layer, SubscriberExt as __tracing_subscriber_SubscriberExt,
};

pub use crate::util::SubscriberInitExt as _;
