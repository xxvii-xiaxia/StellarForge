//! The transaction builder.

use crate::error::TxError;
use crate::memo::Memo;
use crate::operation::Operation;
use crate::time_bounds::TimeBounds;
use crate::{MAX_OPERATIONS, MIN_BASE_FEE_STROOPS};

/// An unsigned, constructed Stellar transaction, ready to be encoded to XDR and signed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    pub source_account: String,
    pub sequence_number: i64,
    pub base_fee_stroops: u32,
    pub memo: Memo,
    pub time_bounds: Option<TimeBounds>,
    pub operations: Vec<Operation>,
}

impl Transaction {
    /// The total fee for this transaction: `base_fee_stroops * operations.len()`.
    pub fn total_fee_stroops(&self) -> u64 {
        self.base_fee_stroops as u64 * self.operations.len() as u64
    }
}

/// Builds a [`Transaction`], validating Stellar's structural constraints (operation count, fee
/// floor, memo/time-bounds validity) before producing the final value.
#[derive(Debug, Default)]
pub struct TransactionBuilder {
    source_account: Option<String>,
    sequence_number: Option<i64>,
    base_fee_stroops: Option<u32>,
    memo: Memo,
    time_bounds: Option<TimeBounds>,
    operations: Vec<Operation>,
}

impl TransactionBuilder {
    /// Starts building a transaction for `source_account` at `sequence_number` (the account's
    /// current sequence number; the built transaction consumes `sequence_number + 1`... callers
    /// are expected to pass the sequence number this transaction should use directly).
    pub fn new(source_account: impl Into<String>, sequence_number: i64) -> Result<Self, TxError> {
        let source_account = source_account.into();
        stellarforge_core::validate_address(&source_account)?;
        Ok(Self {
            source_account: Some(source_account),
            sequence_number: Some(sequence_number),
            ..Default::default()
        })
    }

    /// Sets the base fee per operation, in stroops. Defaults to [`MIN_BASE_FEE_STROOPS`] if not
    /// called.
    pub fn base_fee_stroops(mut self, fee: u32) -> Self {
        self.base_fee_stroops = Some(fee);
        self
    }

    /// Sets the transaction memo. Defaults to [`Memo::None`] if not called.
    pub fn memo(mut self, memo: Memo) -> Self {
        self.memo = memo;
        self
    }

    /// Sets the transaction's valid time window.
    pub fn time_bounds(mut self, bounds: TimeBounds) -> Self {
        self.time_bounds = Some(bounds);
        self
    }

    /// Appends an operation to the transaction.
    pub fn add_operation(mut self, operation: Operation) -> Self {
        self.operations.push(operation);
        self
    }

    /// Validates the accumulated fields and produces the final [`Transaction`].
    pub fn build(self) -> Result<Transaction, TxError> {
        let source_account = self
            .source_account
            .ok_or(TxError::MissingField("source_account"))?;
        let sequence_number = self
            .sequence_number
            .ok_or(TxError::MissingField("sequence_number"))?;
        let base_fee_stroops = self.base_fee_stroops.unwrap_or(MIN_BASE_FEE_STROOPS);

        if base_fee_stroops < MIN_BASE_FEE_STROOPS {
            return Err(TxError::FeeTooLow {
                provided: base_fee_stroops,
                minimum: MIN_BASE_FEE_STROOPS,
            });
        }
        if self.operations.is_empty() {
            return Err(TxError::NoOperations);
        }
        if self.operations.len() > MAX_OPERATIONS {
            return Err(TxError::TooManyOperations(self.operations.len()));
        }

        Ok(Transaction {
            source_account,
            sequence_number,
            base_fee_stroops,
            memo: self.memo,
            time_bounds: self.time_bounds,
            operations: self.operations,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stellarforge_core::Asset;

    const SOURCE: &str = "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF";
    const DEST: &str = "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF";

    fn payment() -> Operation {
        Operation::payment(DEST, Asset::Native, "25").unwrap()
    }

    #[test]
    fn builds_minimal_valid_transaction() {
        let tx = TransactionBuilder::new(SOURCE, 100)
            .unwrap()
            .add_operation(payment())
            .build()
            .unwrap();

        assert_eq!(tx.source_account, SOURCE);
        assert_eq!(tx.sequence_number, 100);
        assert_eq!(tx.base_fee_stroops, MIN_BASE_FEE_STROOPS);
        assert_eq!(tx.memo, Memo::None);
        assert_eq!(tx.operations.len(), 1);
    }

    #[test]
    fn new_rejects_invalid_source_account() {
        assert!(TransactionBuilder::new("not-an-address", 1).is_err());
    }

    #[test]
    fn build_rejects_zero_operations() {
        let err = TransactionBuilder::new(SOURCE, 1)
            .unwrap()
            .build()
            .unwrap_err();
        assert!(matches!(err, TxError::NoOperations));
    }

    #[test]
    fn build_rejects_too_many_operations() {
        let mut builder = TransactionBuilder::new(SOURCE, 1).unwrap();
        for _ in 0..=MAX_OPERATIONS {
            builder = builder.add_operation(payment());
        }
        let err = builder.build().unwrap_err();
        assert!(matches!(err, TxError::TooManyOperations(101)));
    }

    #[test]
    fn build_rejects_fee_below_minimum() {
        let err = TransactionBuilder::new(SOURCE, 1)
            .unwrap()
            .base_fee_stroops(50)
            .add_operation(payment())
            .build()
            .unwrap_err();
        assert!(matches!(
            err,
            TxError::FeeTooLow {
                provided: 50,
                minimum: MIN_BASE_FEE_STROOPS
            }
        ));
    }

    #[test]
    fn total_fee_multiplies_by_operation_count() {
        let tx = TransactionBuilder::new(SOURCE, 1)
            .unwrap()
            .base_fee_stroops(200)
            .add_operation(payment())
            .add_operation(payment())
            .build()
            .unwrap();
        assert_eq!(tx.total_fee_stroops(), 400);
    }

    #[test]
    fn memo_and_time_bounds_are_carried_through() {
        let bounds = TimeBounds::new(0, 100).unwrap();
        let tx = TransactionBuilder::new(SOURCE, 1)
            .unwrap()
            .memo(Memo::text("hi").unwrap())
            .time_bounds(bounds)
            .add_operation(payment())
            .build()
            .unwrap();
        assert_eq!(tx.memo, Memo::Text("hi".to_string()));
        assert_eq!(tx.time_bounds, Some(bounds));
    }
}
