//! Structured errors for transaction construction and submission.

use std::fmt;

/// Errors that can occur while building, submitting, or tracking a Stellar transaction.
#[derive(Debug)]
pub enum TxError {
    /// A field required to build a valid transaction was missing (e.g. no source account).
    MissingField(&'static str),
    /// A transaction had no operations; Stellar requires at least one.
    NoOperations,
    /// Too many operations were added; Stellar caps a transaction at 100 operations.
    TooManyOperations(usize),
    /// The base fee per operation was below the network minimum (100 stroops).
    FeeTooLow { provided: u32, minimum: u32 },
    /// The memo did not meet the constraints for its type (e.g. text memo over 28 bytes).
    InvalidMemo(String),
    /// An address (source, destination, etc.) failed validation.
    InvalidAddress(stellarforge_core::Error),
    /// The transaction was rejected during simulation or submission.
    Rejected(String),
    /// Confirmation tracking failed (e.g. timed out or lost connection).
    ConfirmationFailed(String),
}

impl fmt::Display for TxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TxError::MissingField(field) => write!(f, "missing required field: {field}"),
            TxError::NoOperations => write!(f, "transaction must have at least one operation"),
            TxError::TooManyOperations(count) => {
                write!(f, "transaction has {count} operations, maximum is 100")
            }
            TxError::FeeTooLow { provided, minimum } => write!(
                f,
                "base fee {provided} stroops is below the network minimum of {minimum} stroops"
            ),
            TxError::InvalidMemo(reason) => write!(f, "invalid memo: {reason}"),
            TxError::InvalidAddress(err) => write!(f, "{err}"),
            TxError::Rejected(reason) => write!(f, "transaction rejected: {reason}"),
            TxError::ConfirmationFailed(reason) => write!(f, "confirmation failed: {reason}"),
        }
    }
}

impl std::error::Error for TxError {}

impl From<stellarforge_core::Error> for TxError {
    fn from(err: stellarforge_core::Error) -> Self {
        TxError::InvalidAddress(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fee_too_low_display() {
        let err = TxError::FeeTooLow {
            provided: 50,
            minimum: 100,
        };
        assert_eq!(
            err.to_string(),
            "base fee 50 stroops is below the network minimum of 100 stroops"
        );
    }

    #[test]
    fn too_many_operations_display() {
        let err = TxError::TooManyOperations(101);
        assert_eq!(
            err.to_string(),
            "transaction has 101 operations, maximum is 100"
        );
    }

    #[test]
    fn from_core_error_wraps_as_invalid_address() {
        let core_err = stellarforge_core::Error::InvalidAddress {
            address: "bad".to_string(),
            reason: "too short".to_string(),
        };
        let tx_err: TxError = core_err.into();
        assert!(matches!(tx_err, TxError::InvalidAddress(_)));
    }
}
