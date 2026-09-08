//! Transaction construction primitives for StellarForge.
//!
//! This crate provides the building blocks used to construct a Stellar transaction: fee
//! configuration, memos, time bounds, and the [`TransactionBuilder`] itself. It does not
//! perform network I/O — submission and confirmation tracking build on top of these types.

pub mod error;
pub mod memo;
pub mod time_bounds;

pub use error::TxError;
pub use memo::Memo;
pub use time_bounds::TimeBounds;

/// The network minimum base fee, in stroops, per operation.
pub const MIN_BASE_FEE_STROOPS: u32 = 100;

/// The maximum number of operations a single Stellar transaction may contain.
pub const MAX_OPERATIONS: usize = 100;
