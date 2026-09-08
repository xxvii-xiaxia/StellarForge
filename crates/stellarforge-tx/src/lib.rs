//! Transaction construction primitives for StellarForge.
//!
//! This crate provides the building blocks used to construct a Stellar transaction: fee
//! configuration, memos, time bounds, and the [`TransactionBuilder`] itself, plus a submission
//! pipeline ([`submit_transaction`]) for sending an already-signed transaction envelope to
//! Horizon. StellarForge never handles private keys or signs transactions itself — see
//! `project.md` §18 — callers provide their own signing mechanism.

pub mod arg;
pub mod builder;
pub mod error;
pub mod memo;
pub mod operation;
pub mod submit;
pub mod time_bounds;

pub use arg::ScArg;
pub use builder::{Transaction, TransactionBuilder};
pub use error::TxError;
pub use memo::Memo;
pub use operation::Operation;
pub use submit::{submit_transaction, SubmitResult};
pub use time_bounds::TimeBounds;

/// The network minimum base fee, in stroops, per operation.
pub const MIN_BASE_FEE_STROOPS: u32 = 100;

/// The maximum number of operations a single Stellar transaction may contain.
pub const MAX_OPERATIONS: usize = 100;
