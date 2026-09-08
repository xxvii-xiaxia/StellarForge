//! Stellar asset representation (native XLM and issued credit assets).

use crate::address::{validate_address, AddressKind};
use crate::error::Error;

/// A Stellar asset: either the native token (XLM) or an issued credit asset identified by an
/// asset code and issuing account.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Asset {
    /// The network's native asset, XLM (lumens).
    Native,
    /// An issued asset with a 1-4 character alphanumeric code.
    CreditAlphanum4 { code: String, issuer: String },
    /// An issued asset with a 5-12 character alphanumeric code.
    CreditAlphanum12 { code: String, issuer: String },
}

impl Asset {
    /// Builds an issued credit asset, choosing the correct XDR class (`ALPHANUM4` vs
    /// `ALPHANUM12`) based on the code length and validating the code and issuer.
    pub fn credit(code: impl Into<String>, issuer: impl Into<String>) -> Result<Self, Error> {
        let code = code.into();
        let issuer = issuer.into();

        if code.is_empty() || code.len() > 12 {
            return Err(Error::InvalidAddress {
                address: code.clone(),
                reason: "asset code must be 1-12 characters".to_string(),
            });
        }
        if !code.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Err(Error::InvalidAddress {
                address: code.clone(),
                reason: "asset code must be ASCII alphanumeric".to_string(),
            });
        }
        match validate_address(&issuer)? {
            AddressKind::Account => {}
            other => {
                return Err(Error::InvalidAddress {
                    address: issuer,
                    reason: format!("asset issuer must be an account address, got {other:?}"),
                })
            }
        }

        if code.len() <= 4 {
            Ok(Asset::CreditAlphanum4 { code, issuer })
        } else {
            Ok(Asset::CreditAlphanum12 { code, issuer })
        }
    }

    /// Returns `true` if this is the native XLM asset.
    pub fn is_native(&self) -> bool {
        matches!(self, Asset::Native)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_ISSUER: &str = "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF";

    #[test]
    fn native_is_native() {
        assert!(Asset::Native.is_native());
    }

    #[test]
    fn short_code_becomes_alphanum4() {
        let asset = Asset::credit("USDC", VALID_ISSUER).unwrap();
        assert_eq!(
            asset,
            Asset::CreditAlphanum4 {
                code: "USDC".to_string(),
                issuer: VALID_ISSUER.to_string(),
            }
        );
    }

    #[test]
    fn long_code_becomes_alphanum12() {
        let asset = Asset::credit("LONGASSET12", VALID_ISSUER).unwrap();
        assert!(matches!(asset, Asset::CreditAlphanum12 { .. }));
    }

    #[test]
    fn empty_code_is_rejected() {
        assert!(Asset::credit("", VALID_ISSUER).is_err());
    }

    #[test]
    fn code_over_12_chars_is_rejected() {
        assert!(Asset::credit("THIRTEENCHARS", VALID_ISSUER).is_err());
    }

    #[test]
    fn non_alphanumeric_code_is_rejected() {
        assert!(Asset::credit("US-D", VALID_ISSUER).is_err());
    }

    #[test]
    fn invalid_issuer_is_rejected() {
        assert!(Asset::credit("USDC", "not-an-address").is_err());
    }

    #[test]
    fn contract_address_as_issuer_is_rejected() {
        let contract = "CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABSC4";
        assert!(Asset::credit("USDC", contract).is_err());
    }
}
