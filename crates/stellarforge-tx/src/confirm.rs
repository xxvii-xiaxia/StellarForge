//! Confirmation tracking: polling Horizon for a transaction's final result.
//!
//! Horizon's classic `POST /transactions` (see [`crate::submit`]) already blocks until the
//! transaction is applied or rejected, so callers that submit through it don't need this.
//! [`wait_for_confirmation`] exists for the cases that don't get that for free: a transaction
//! hash obtained from elsewhere (e.g. Soroban RPC's asynchronous `sendTransaction`, which
//! returns immediately with a `PENDING` status), or re-checking the status of a transaction
//! submitted by another process.

use std::time::Duration;

use serde::Deserialize;
use stellarforge_core::NetworkConfig;
use tokio::time::Instant;

use crate::error::TxError;
use crate::submit::SubmitResult;

/// Polling behavior for [`wait_for_confirmation`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfirmationOptions {
    /// How long to wait between polls while the transaction hasn't appeared yet.
    pub poll_interval: Duration,
    /// How long to keep polling before giving up.
    pub timeout: Duration,
}

impl ConfirmationOptions {
    pub fn new(poll_interval: Duration, timeout: Duration) -> Self {
        Self {
            poll_interval,
            timeout,
        }
    }
}

impl Default for ConfirmationOptions {
    /// Polls every 2 seconds for up to 60 seconds — generous relative to Stellar's ~5 second
    /// ledger close time, without waiting indefinitely on a transaction that was never applied.
    fn default() -> Self {
        Self {
            poll_interval: Duration::from_secs(2),
            timeout: Duration::from_secs(60),
        }
    }
}

/// Polls Horizon's `GET /transactions/{hash}` for `tx_hash` until it is included in a ledger,
/// definitively failed, or `options.timeout` elapses.
///
/// A `404` means the transaction hasn't reached a ledger yet and is retried after
/// `options.poll_interval`. Any other non-success status is treated as a hard failure and
/// returned immediately, since it indicates a problem with the request itself rather than the
/// transaction still being pending.
pub async fn wait_for_confirmation(
    config: &NetworkConfig,
    tx_hash: &str,
    options: ConfirmationOptions,
) -> Result<SubmitResult, TxError> {
    let url = format!(
        "{}/transactions/{tx_hash}",
        config.horizon_url.trim_end_matches('/')
    );
    let client = reqwest::Client::new();
    let deadline = Instant::now() + options.timeout;

    loop {
        let response = client.get(&url).send().await.map_err(|err| {
            TxError::ConfirmationFailed(format!("confirmation request failed: {err}"))
        })?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            if Instant::now() >= deadline {
                return Err(TxError::ConfirmationFailed(format!(
                    "timed out after {:?} waiting for transaction {tx_hash} to be confirmed",
                    options.timeout
                )));
            }
            tokio::time::sleep(options.poll_interval).await;
            continue;
        }

        if !response.status().is_success() {
            return Err(TxError::ConfirmationFailed(format!(
                "Horizon returned unexpected status {} while polling for transaction {tx_hash}",
                response.status()
            )));
        }

        let raw = response.text().await.map_err(|err| {
            TxError::ConfirmationFailed(format!("failed to read response body: {err}"))
        })?;
        let record: HorizonTransactionRecord = serde_json::from_str(&raw).map_err(|err| {
            TxError::ConfirmationFailed(format!("failed to parse transaction record: {err}"))
        })?;

        return if record.successful {
            Ok(SubmitResult {
                hash: record.hash,
                ledger: record.ledger,
            })
        } else {
            Err(TxError::ConfirmationFailed(format!(
                "transaction {tx_hash} was included in ledger {} but failed",
                record.ledger
            )))
        };
    }
}

#[derive(Debug, Deserialize)]
struct HorizonTransactionRecord {
    hash: String,
    ledger: i64,
    successful: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn mock_config(server: &MockServer) -> NetworkConfig {
        NetworkConfig::custom(
            server.uri(),
            "http://localhost:8000/soroban/rpc",
            "Standalone Network ; February 2017",
        )
    }

    fn fast_options() -> ConfirmationOptions {
        ConfirmationOptions::new(Duration::from_millis(10), Duration::from_millis(200))
    }

    #[tokio::test]
    async fn confirmation_succeeds_immediately_when_already_included() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/transactions/abc123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "hash": "abc123",
                "ledger": 42,
                "successful": true
            })))
            .mount(&server)
            .await;

        let config = mock_config(&server).await;
        let result = wait_for_confirmation(&config, "abc123", fast_options())
            .await
            .unwrap();

        assert_eq!(result.hash, "abc123");
        assert_eq!(result.ledger, 42);
    }

    #[tokio::test]
    async fn confirmation_polls_through_not_found_until_included() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/transactions/abc123"))
            .respond_with(ResponseTemplate::new(404))
            .up_to_n_times(2)
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/transactions/abc123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "hash": "abc123",
                "ledger": 42,
                "successful": true
            })))
            .mount(&server)
            .await;

        let config = mock_config(&server).await;
        let result = wait_for_confirmation(&config, "abc123", fast_options())
            .await
            .unwrap();

        assert_eq!(result.hash, "abc123");
        assert_eq!(result.ledger, 42);
    }

    #[tokio::test]
    async fn confirmation_reports_a_failed_transaction() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/transactions/abc123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "hash": "abc123",
                "ledger": 42,
                "successful": false
            })))
            .mount(&server)
            .await;

        let config = mock_config(&server).await;
        let err = wait_for_confirmation(&config, "abc123", fast_options())
            .await
            .unwrap_err();

        assert!(matches!(err, TxError::ConfirmationFailed(_)));
        assert_eq!(
            err.to_string(),
            "confirmation failed: transaction abc123 was included in ledger 42 but failed"
        );
    }

    #[tokio::test]
    async fn confirmation_times_out_if_never_included() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/transactions/abc123"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let config = mock_config(&server).await;
        let err = wait_for_confirmation(&config, "abc123", fast_options())
            .await
            .unwrap_err();

        assert!(matches!(err, TxError::ConfirmationFailed(_)));
        assert!(err.to_string().contains("timed out"));
    }

    #[tokio::test]
    async fn confirmation_fails_fast_on_unexpected_status() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/transactions/abc123"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let config = mock_config(&server).await;
        let err = wait_for_confirmation(&config, "abc123", fast_options())
            .await
            .unwrap_err();

        assert!(err.to_string().contains("unexpected status 500"));
    }
}
