//! Shared primitives for StellarForge.
//!
//! This crate provides the foundational types used across every other StellarForge crate:
//! network configuration, common error types, and Stellar address validation.

pub mod network;

pub use network::{Network, NetworkConfig};
