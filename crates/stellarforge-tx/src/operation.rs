//! Transaction operations.
//!
//! An initial, focused subset of the Stellar operation set — enough to build a working payment
//! workflow, including Soroban token (SEP-41) transfers via contract invocation.

use crate::arg::ScArg;
use crate::error::TxError;
use stellarforge_core::{AddressKind, Asset};

/// A single operation within a transaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation {
    /// Sends `amount` of `asset` from the transaction's source account to `destination`.
    Payment {
        destination: String,
        asset: Asset,
        amount: String,
    },
    /// Invokes `function` on the Soroban contract at `contract` with `args`.
    ///
    /// This is the operation SEP-41 token transfers (and other contract calls) are built on;
    /// full argument/return XDR encoding is handled by the Soroban module.
    InvokeContract {
        contract: String,
        function: String,
        args: Vec<ScArg>,
    },
}

impl Operation {
    /// A payment operation, validating that the destination is a well-formed Stellar address
    /// and the amount is a positive decimal string.
    pub fn payment(
        destination: impl Into<String>,
        asset: Asset,
        amount: impl Into<String>,
    ) -> Result<Self, TxError> {
        let destination = destination.into();
        let amount = amount.into();

        stellarforge_core::validate_address(&destination)?;
        validate_amount(&amount)?;

        Ok(Operation::Payment {
            destination,
            asset,
            amount,
        })
    }

    /// A contract invocation, validating that `contract` is a well-formed Soroban contract
    /// address.
    pub fn invoke_contract(
        contract: impl Into<String>,
        function: impl Into<String>,
        args: Vec<ScArg>,
    ) -> Result<Self, TxError> {
        let contract = contract.into();

        match stellarforge_core::validate_address(&contract)? {
            AddressKind::Contract => {}
            other => {
                return Err(TxError::Rejected(format!(
                    "invoke_contract target must be a contract address, got {other:?}"
                )))
            }
        }

        Ok(Operation::InvokeContract {
            contract,
            function: function.into(),
            args,
        })
    }
}

/// Validates that `amount` is a positive decimal amount, as Stellar expects (up to 7 decimal
/// places, matching the network's fixed-point representation of asset quantities).
fn validate_amount(amount: &str) -> Result<(), TxError> {
    let invalid = || TxError::Rejected(format!("invalid amount: `{amount}`"));

    if amount.is_empty() {
        return Err(invalid());
    }

    let mut parts = amount.splitn(2, '.');
    let whole = parts.next().unwrap();
    let fraction = parts.next();

    if whole.is_empty() || !whole.chars().all(|c| c.is_ascii_digit()) {
        return Err(invalid());
    }
    if let Some(fraction) = fraction {
        if fraction.is_empty()
            || fraction.len() > 7
            || !fraction.chars().all(|c| c.is_ascii_digit())
        {
            return Err(invalid());
        }
    }
    if whole.chars().all(|c| c == '0') && fraction.is_none_or(|f| f.chars().all(|c| c == '0')) {
        return Err(TxError::Rejected(format!(
            "amount must be greater than zero: `{amount}`"
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_DESTINATION: &str = "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF";

    #[test]
    fn payment_with_valid_fields_succeeds() {
        let op = Operation::payment(VALID_DESTINATION, Asset::Native, "25").unwrap();
        assert!(
            matches!(op, Operation::Payment { asset, amount, .. } if asset == Asset::Native && amount == "25")
        );
    }

    #[test]
    fn payment_with_fractional_amount_succeeds() {
        assert!(Operation::payment(VALID_DESTINATION, Asset::Native, "25.5000000").is_ok());
    }

    #[test]
    fn payment_to_invalid_destination_is_rejected() {
        let err = Operation::payment("not-an-address", Asset::Native, "25").unwrap_err();
        assert!(matches!(err, TxError::InvalidAddress(_)));
    }

    #[test]
    fn payment_with_zero_amount_is_rejected() {
        assert!(Operation::payment(VALID_DESTINATION, Asset::Native, "0").is_err());
        assert!(Operation::payment(VALID_DESTINATION, Asset::Native, "0.0").is_err());
    }

    #[test]
    fn payment_with_negative_amount_is_rejected() {
        assert!(Operation::payment(VALID_DESTINATION, Asset::Native, "-5").is_err());
    }

    #[test]
    fn payment_with_too_many_decimal_places_is_rejected() {
        assert!(Operation::payment(VALID_DESTINATION, Asset::Native, "1.12345678").is_err());
    }

    #[test]
    fn payment_with_non_numeric_amount_is_rejected() {
        assert!(Operation::payment(VALID_DESTINATION, Asset::Native, "abc").is_err());
    }

    const VALID_CONTRACT: &str = "CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABSC4";

    #[test]
    fn invoke_contract_with_valid_address_succeeds() {
        let op = Operation::invoke_contract(
            VALID_CONTRACT,
            "transfer",
            vec![
                ScArg::Address(VALID_DESTINATION.to_string()),
                ScArg::I128(25),
            ],
        )
        .unwrap();
        assert!(matches!(
            op,
            Operation::InvokeContract { function, args, .. }
                if function == "transfer" && args.len() == 2
        ));
    }

    #[test]
    fn invoke_contract_rejects_non_contract_address() {
        let err = Operation::invoke_contract(VALID_DESTINATION, "transfer", vec![]).unwrap_err();
        assert!(matches!(err, TxError::Rejected(_)));
    }

    #[test]
    fn invoke_contract_rejects_malformed_address() {
        assert!(Operation::invoke_contract("not-a-contract", "transfer", vec![]).is_err());
    }
}
