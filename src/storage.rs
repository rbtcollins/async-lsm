//! The storage interface for async-lsm
//!
//! This models IO operations not details such as database formats

/// Storage offers an async interface for IO operations that is can be implemented to host an LSM on arbitrary storage
/// providers. Storage is async for all operations, as cloud storage generally is over the network and thus the very
/// best IO latency is still high enough to cause substantial blocking. (And, most cloud storage SDKs are async).
pub trait Storage: std::fmt::Debug {}
