//! SEP-41 token (Soroban contract) payments.
//!
//! SEP-41 tokens (including wrapped classic assets and native Soroban tokens) expose a standard
//! `transfer(from: Address, to: Address, amount: i128)` contract function rather than the
//! classic `Payment` operation. `amount` is in the token's smallest unit — callers are
//! responsible for scaling by the token's `decimals()` since that varies per token.

use stellarforge_core::validate_address;
use stellarforge_tx::{Memo, Operation, ScArg, Transaction, TransactionBuilder, TxError};

/// A request to transfer `amount` (in the token's smallest unit) of a SEP-41 token at
/// `contract` to `destination`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenPaymentRequest {
    pub contract: String,
    pub destination: String,
    pub amount: i128,
    pub memo: Option<String>,
}

impl TokenPaymentRequest {
    pub fn new(contract: impl Into<String>, destination: impl Into<String>, amount: i128) -> Self {
        Self {
            contract: contract.into(),
            destination: destination.into(),
            amount,
            memo: None,
        }
    }

    /// Attaches a text memo to this request.
    pub fn with_memo(mut self, memo: impl Into<String>) -> Self {
        self.memo = Some(memo.into());
        self
    }
}

/// Builds an unsigned [`Transaction`] that calls `transfer` on a SEP-41 token contract.
pub fn build_token_payment_transaction(
    source_account: impl Into<String>,
    sequence_number: i64,
    base_fee_stroops: u32,
    request: TokenPaymentRequest,
) -> Result<Transaction, TxError> {
    let source_account = source_account.into();
    validate_address(&source_account)?;

    if request.amount <= 0 {
        return Err(TxError::Rejected(format!(
            "token payment amount must be greater than zero: {}",
            request.amount
        )));
    }

    let operation = Operation::invoke_contract(
        request.contract,
        "transfer",
        vec![
            ScArg::Address(source_account.clone()),
            ScArg::Address(request.destination),
            ScArg::I128(request.amount),
        ],
    )?;

    let mut builder = TransactionBuilder::new(source_account, sequence_number)?
        .base_fee_stroops(base_fee_stroops)
        .add_operation(operation);

    if let Some(memo) = request.memo {
        builder = builder.memo(Memo::text(memo)?);
    }

    builder.build()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOURCE: &str = "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF";
    const DEST: &str = "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF";
    const CONTRACT: &str = "CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABSC4";

    #[test]
    fn builds_token_transfer_transaction() {
        let request = TokenPaymentRequest::new(CONTRACT, DEST, 250_0000000);
        let tx = build_token_payment_transaction(SOURCE, 1, 100, request).unwrap();

        assert_eq!(tx.operations.len(), 1);
        match &tx.operations[0] {
            Operation::InvokeContract {
                contract,
                function,
                args,
            } => {
                assert_eq!(contract, CONTRACT);
                assert_eq!(function, "transfer");
                assert_eq!(
                    args,
                    &vec![
                        ScArg::Address(SOURCE.to_string()),
                        ScArg::Address(DEST.to_string()),
                        ScArg::I128(250_0000000),
                    ]
                );
            }
            other => panic!("expected InvokeContract, got {other:?}"),
        }
    }

    #[test]
    fn rejects_zero_amount() {
        let request = TokenPaymentRequest::new(CONTRACT, DEST, 0);
        assert!(build_token_payment_transaction(SOURCE, 1, 100, request).is_err());
    }

    #[test]
    fn rejects_negative_amount() {
        let request = TokenPaymentRequest::new(CONTRACT, DEST, -5);
        assert!(build_token_payment_transaction(SOURCE, 1, 100, request).is_err());
    }

    #[test]
    fn rejects_non_contract_address() {
        let request = TokenPaymentRequest::new(DEST, DEST, 100);
        assert!(build_token_payment_transaction(SOURCE, 1, 100, request).is_err());
    }

    #[test]
    fn carries_memo_through() {
        let request = TokenPaymentRequest::new(CONTRACT, DEST, 100).with_memo("token-payment");
        let tx = build_token_payment_transaction(SOURCE, 1, 100, request).unwrap();
        assert_eq!(tx.memo, Memo::Text("token-payment".to_string()));
    }
}
