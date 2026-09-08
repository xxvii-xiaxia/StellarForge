//! Payment request definitions.

use stellarforge_core::Asset;

/// A request to pay `amount` of `asset` to `destination`, with an optional memo.
///
/// This is the input to [`crate::build_payment_transaction`], which turns it into an unsigned
/// [`stellarforge_tx::Transaction`] against a given source account and sequence number.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaymentRequest {
    pub destination: String,
    pub asset: Asset,
    pub amount: String,
    pub memo: Option<String>,
}

impl PaymentRequest {
    /// A native XLM payment with no memo.
    pub fn xlm(destination: impl Into<String>, amount: impl Into<String>) -> Self {
        Self {
            destination: destination.into(),
            asset: Asset::Native,
            amount: amount.into(),
            memo: None,
        }
    }

    /// Attaches a text memo to this request.
    pub fn with_memo(mut self, memo: impl Into<String>) -> Self {
        self.memo = Some(memo.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xlm_builds_native_request_with_no_memo() {
        let req = PaymentRequest::xlm("GDEST", "25");
        assert_eq!(req.asset, Asset::Native);
        assert_eq!(req.amount, "25");
        assert_eq!(req.memo, None);
    }

    #[test]
    fn with_memo_sets_memo() {
        let req = PaymentRequest::xlm("GDEST", "25").with_memo("invoice-1038");
        assert_eq!(req.memo, Some("invoice-1038".to_string()));
    }
}
