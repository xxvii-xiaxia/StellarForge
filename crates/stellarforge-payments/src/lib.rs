//! Reusable payment infrastructure for StellarForge.
//!
//! Turns a [`PaymentRequest`] into an unsigned [`stellarforge_tx::Transaction`] containing a
//! single payment operation. Submitting and confirming that transaction is handled by the
//! transaction engine once network I/O is wired up (see `project.md` issue 007).

pub mod request;

pub use request::PaymentRequest;

use stellarforge_tx::{Memo, Operation, Transaction, TransactionBuilder, TxError};

/// Builds an unsigned payment [`Transaction`] for `source_account` at `sequence_number`.
pub fn build_payment_transaction(
    source_account: impl Into<String>,
    sequence_number: i64,
    base_fee_stroops: u32,
    request: PaymentRequest,
) -> Result<Transaction, TxError> {
    let operation = Operation::payment(request.destination, request.asset, request.amount)?;

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
    use stellarforge_core::Asset;

    const SOURCE: &str = "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF";
    const DEST: &str = "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF";

    #[test]
    fn builds_xlm_payment_transaction() {
        let request = PaymentRequest::xlm(DEST, "25");
        let tx = build_payment_transaction(SOURCE, 1, 100, request).unwrap();

        assert_eq!(tx.source_account, SOURCE);
        assert_eq!(tx.operations.len(), 1);
        assert!(matches!(
            &tx.operations[0],
            Operation::Payment { asset, amount, .. }
                if *asset == Asset::Native && amount == "25"
        ));
        assert_eq!(tx.memo, Memo::None);
    }

    #[test]
    fn builds_payment_transaction_with_memo() {
        let request = PaymentRequest::xlm(DEST, "25").with_memo("invoice-1038");
        let tx = build_payment_transaction(SOURCE, 1, 100, request).unwrap();
        assert_eq!(tx.memo, Memo::Text("invoice-1038".to_string()));
    }

    #[test]
    fn rejects_payment_with_invalid_destination() {
        let request = PaymentRequest::xlm("not-an-address", "25");
        assert!(build_payment_transaction(SOURCE, 1, 100, request).is_err());
    }

    #[test]
    fn rejects_payment_with_memo_over_limit() {
        let request = PaymentRequest::xlm(DEST, "25").with_memo("a".repeat(29));
        assert!(build_payment_transaction(SOURCE, 1, 100, request).is_err());
    }
}
