//! Transaction memos.
//!
//! Mirrors the Stellar `Memo` XDR union: `MEMO_NONE`, `MEMO_TEXT`, `MEMO_ID`, `MEMO_HASH`, and
//! `MEMO_RETURN`.

use crate::error::TxError;

/// The maximum length, in bytes, of a `MEMO_TEXT` value.
pub const MAX_TEXT_MEMO_BYTES: usize = 28;

/// A Stellar transaction memo.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Memo {
    /// No memo.
    #[default]
    None,
    /// A free-form UTF-8 text memo, at most 28 bytes.
    Text(String),
    /// A 64-bit identifier, commonly used to route deposits to a specific customer/sub-account.
    Id(u64),
    /// An opaque 32-byte hash (e.g. a hash of an off-chain document).
    Hash([u8; 32]),
    /// A 32-byte hash of the transaction that this transaction is returning funds for.
    Return([u8; 32]),
}

impl Memo {
    /// Builds a text memo, validating that it does not exceed [`MAX_TEXT_MEMO_BYTES`] bytes.
    pub fn text(value: impl Into<String>) -> Result<Self, TxError> {
        let value = value.into();
        if value.len() > MAX_TEXT_MEMO_BYTES {
            return Err(TxError::InvalidMemo(format!(
                "text memo is {} bytes, maximum is {MAX_TEXT_MEMO_BYTES}",
                value.len()
            )));
        }
        Ok(Memo::Text(value))
    }

    /// Returns `true` if this memo is [`Memo::None`].
    pub fn is_none(&self) -> bool {
        matches!(self, Memo::None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_memo_within_limit_succeeds() {
        let memo = Memo::text("invoice-1038").unwrap();
        assert_eq!(memo, Memo::Text("invoice-1038".to_string()));
    }

    #[test]
    fn text_memo_at_exact_limit_succeeds() {
        let value = "a".repeat(MAX_TEXT_MEMO_BYTES);
        assert!(Memo::text(value).is_ok());
    }

    #[test]
    fn text_memo_over_limit_is_rejected() {
        let value = "a".repeat(MAX_TEXT_MEMO_BYTES + 1);
        let err = Memo::text(value).unwrap_err();
        assert!(matches!(err, TxError::InvalidMemo(_)));
    }

    #[test]
    fn default_memo_is_none() {
        assert_eq!(Memo::default(), Memo::None);
        assert!(Memo::None.is_none());
        assert!(!Memo::Id(5).is_none());
    }
}
