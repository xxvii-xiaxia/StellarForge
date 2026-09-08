//! Stellar address (strkey) validation.
//!
//! Wraps the official [`stellar_strkey`] crate (SEP-23) to validate the address kinds
//! StellarForge cares about and report which kind an address is, without callers needing
//! to depend on `stellar-strkey` directly.

use crate::error::Error;
use stellar_strkey::Strkey;

/// The kind of Stellar address a validated strkey represents.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressKind {
    /// A classic Stellar account (`G...`).
    Account,
    /// A muxed account (`M...`), used to identify sub-accounts sharing one underlying account.
    MuxedAccount,
    /// A Soroban contract (`C...`).
    Contract,
}

/// Validates that `address` is a well-formed Stellar strkey and returns which kind it is.
///
/// This checks the strkey's version byte and CRC16-XMODEM checksum (SEP-23); it does not check
/// that the address exists on any particular network.
pub fn validate_address(address: &str) -> Result<AddressKind, Error> {
    match Strkey::from_string(address) {
        Ok(Strkey::PublicKeyEd25519(_)) => Ok(AddressKind::Account),
        Ok(Strkey::MuxedAccountEd25519(_)) => Ok(AddressKind::MuxedAccount),
        Ok(Strkey::Contract(_)) => Ok(AddressKind::Contract),
        Ok(_) => Err(Error::InvalidAddress {
            address: address.to_string(),
            reason: "strkey is well-formed but not an account, muxed account, or contract address"
                .to_string(),
        }),
        Err(decode_err) => Err(Error::InvalidAddress {
            address: address.to_string(),
            reason: decode_err.to_string(),
        }),
    }
}

/// Returns `true` if `address` is a valid, well-formed Stellar address of any kind StellarForge
/// recognizes (account, muxed account, or contract).
pub fn is_valid_address(address: &str) -> bool {
    validate_address(address).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_ACCOUNT: &str = "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF";
    const VALID_CONTRACT: &str = "CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABSC4";

    #[test]
    fn validates_account_address() {
        assert_eq!(validate_address(VALID_ACCOUNT).unwrap(), AddressKind::Account);
    }

    #[test]
    fn validates_contract_address() {
        assert_eq!(
            validate_address(VALID_CONTRACT).unwrap(),
            AddressKind::Contract
        );
    }

    #[test]
    fn rejects_malformed_address() {
        let err = validate_address("not-a-real-address").unwrap_err();
        assert!(matches!(err, Error::InvalidAddress { .. }));
    }

    #[test]
    fn rejects_address_with_bad_checksum() {
        // Same as VALID_ACCOUNT but with the last character altered.
        let tampered = "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHZ";
        assert!(!is_valid_address(tampered));
    }

    #[test]
    fn is_valid_address_matches_validate_address() {
        assert!(is_valid_address(VALID_ACCOUNT));
        assert!(!is_valid_address("garbage"));
    }
}
