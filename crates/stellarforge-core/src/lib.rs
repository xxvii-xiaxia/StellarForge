//! Shared primitives for StellarForge.
//!
//! This crate provides the foundational types used across every other StellarForge crate:
//! network configuration, common error types, and Stellar address validation.

pub mod address;
pub mod error;
pub mod network;

pub use address::{is_valid_address, validate_address, AddressKind};
pub use error::Error;
pub use network::{Network, NetworkConfig};
