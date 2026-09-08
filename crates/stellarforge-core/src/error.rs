//! Common error types shared across StellarForge crates.

use std::fmt;

/// A structured error returned by StellarForge core operations.
///
/// Downstream crates (transactions, payments, Soroban, etc.) define their own error types but
/// generally convert into or wrap this type so that applications can match on a consistent set
/// of failure categories.
#[derive(Debug)]
pub enum Error {
    /// A Stellar address (account, contract, or muxed address) failed strkey validation.
    InvalidAddress { address: String, reason: String },
    /// A network configuration value was invalid (e.g. an empty URL).
    InvalidNetworkConfig(String),
    /// A lower-level I/O or transport failure occurred.
    Transport(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidAddress { address, reason } => {
                write!(f, "invalid Stellar address `{address}`: {reason}")
            }
            Error::InvalidNetworkConfig(reason) => {
                write!(f, "invalid network configuration: {reason}")
            }
            Error::Transport(reason) => write!(f, "transport error: {reason}"),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_address_display_includes_address_and_reason() {
        let err = Error::InvalidAddress {
            address: "not-an-address".to_string(),
            reason: "bad checksum".to_string(),
        };
        let message = err.to_string();
        assert!(message.contains("not-an-address"));
        assert!(message.contains("bad checksum"));
    }

    #[test]
    fn invalid_network_config_display() {
        let err = Error::InvalidNetworkConfig("horizon_url must not be empty".to_string());
        assert_eq!(
            err.to_string(),
            "invalid network configuration: horizon_url must not be empty"
        );
    }
}
