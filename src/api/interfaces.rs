use futures::lock::Mutex;
use std::collections::hash_map::DefaultHasher;
use std::sync::Arc;

/// ========= TYPE ABSTRACTIONS ========= ///

pub type CFilterConnection = Arc<Mutex<cuckoofilter::CuckooFilter<DefaultHasher>>>;
