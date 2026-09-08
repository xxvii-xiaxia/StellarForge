//! Network configuration for connecting to Stellar (mainnet, testnet, or a custom network).

use serde::{Deserialize, Serialize};

/// A Stellar network passphrase, used to sign and verify transactions for a specific network.
pub const PUBLIC_NETWORK_PASSPHRASE: &str = "Public Global Stellar Network ; September 2015";
pub const TESTNET_NETWORK_PASSPHRASE: &str = "Test SDF Network ; September 2015";

/// Identifies which Stellar network a [`NetworkConfig`] targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Network {
    Public,
    Testnet,
    Custom,
}

/// Configuration required to talk to a Stellar network: the Horizon and Soroban RPC endpoints,
/// and the network passphrase used for transaction signing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub network: Network,
    pub horizon_url: String,
    pub rpc_url: String,
    pub passphrase: String,
}

impl NetworkConfig {
    /// The official Stellar public (mainnet) network.
    pub fn public() -> Self {
        Self {
            network: Network::Public,
            horizon_url: "https://horizon.stellar.org".to_string(),
            rpc_url: "https://mainnet.sorobanrpc.com".to_string(),
            passphrase: PUBLIC_NETWORK_PASSPHRASE.to_string(),
        }
    }

    /// The official Stellar testnet.
    pub fn testnet() -> Self {
        Self {
            network: Network::Testnet,
            horizon_url: "https://horizon-testnet.stellar.org".to_string(),
            rpc_url: "https://soroban-testnet.stellar.org".to_string(),
            passphrase: TESTNET_NETWORK_PASSPHRASE.to_string(),
        }
    }

    /// A custom network, e.g. a local `stellar-core` / RPC instance or a private network.
    pub fn custom(
        horizon_url: impl Into<String>,
        rpc_url: impl Into<String>,
        passphrase: impl Into<String>,
    ) -> Self {
        Self {
            network: Network::Custom,
            horizon_url: horizon_url.into(),
            rpc_url: rpc_url.into(),
            passphrase: passphrase.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_network_has_expected_passphrase() {
        let config = NetworkConfig::public();
        assert_eq!(config.network, Network::Public);
        assert_eq!(config.passphrase, PUBLIC_NETWORK_PASSPHRASE);
        assert_eq!(config.horizon_url, "https://horizon.stellar.org");
    }

    #[test]
    fn testnet_network_has_expected_passphrase() {
        let config = NetworkConfig::testnet();
        assert_eq!(config.network, Network::Testnet);
        assert_eq!(config.passphrase, TESTNET_NETWORK_PASSPHRASE);
    }

    #[test]
    fn custom_network_uses_provided_values() {
        let config = NetworkConfig::custom(
            "http://localhost:8000",
            "http://localhost:8000/soroban/rpc",
            "Standalone Network ; February 2017",
        );
        assert_eq!(config.network, Network::Custom);
        assert_eq!(config.horizon_url, "http://localhost:8000");
        assert_eq!(config.rpc_url, "http://localhost:8000/soroban/rpc");
        assert_eq!(config.passphrase, "Standalone Network ; February 2017");
    }
}
