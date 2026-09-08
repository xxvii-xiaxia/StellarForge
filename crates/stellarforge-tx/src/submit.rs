//! Transaction submission against Horizon.
//!
//! StellarForge does not sign transactions (see [`crate`] docs and `project.md` §18) — callers
//! provide their own signing mechanism and pass the resulting base64-encoded transaction
//! envelope XDR to [`submit_transaction`].

use crate::error::TxError;
use serde::Deserialize;
use stellarforge_core::NetworkConfig;

/// The result of a transaction that Horizon confirmed was applied to the ledger.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubmitResult {
    /// The transaction hash, hex-encoded.
    pub hash: String,
    /// The ledger sequence the transaction was included in.
    pub ledger: i64,
}

/// Submits a signed transaction envelope (base64-encoded XDR) to Horizon and waits for the
/// result. Horizon's transaction endpoint blocks until the transaction is either applied or
/// definitively rejected, so a successful return means the transaction is on the ledger.
pub async fn submit_transaction(
    config: &NetworkConfig,
    signed_envelope_xdr: &str,
) -> Result<SubmitResult, TxError> {
    let url = format!("{}/transactions", config.horizon_url.trim_end_matches('/'));

    let response = reqwest::Client::new()
        .post(&url)
        .form(&[("tx", signed_envelope_xdr)])
        .send()
        .await
        .map_err(|err| TxError::Rejected(format!("submission request failed: {err}")))?;

    let status = response.status();
    let raw = response
        .text()
        .await
        .map_err(|err| TxError::Rejected(format!("failed to read response body: {err}")))?;

    if status.is_success() {
        let body: HorizonSuccess = serde_json::from_str(&raw).map_err(|err| {
            TxError::Rejected(format!("failed to parse successful response: {err}"))
        })?;
        Ok(SubmitResult {
            hash: body.hash,
            ledger: body.ledger,
        })
    } else {
        let body: HorizonError = serde_json::from_str(&raw)
            .unwrap_or_else(|_| HorizonError::fallback(status.as_u16(), &raw));
        Err(TxError::Rejected(body.describe()))
    }
}

#[derive(Debug, Deserialize)]
struct HorizonSuccess {
    hash: String,
    ledger: i64,
}

#[derive(Debug, Deserialize)]
struct HorizonError {
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    extras: Option<HorizonErrorExtras>,
}

#[derive(Debug, Deserialize, Default)]
struct HorizonErrorExtras {
    #[serde(default)]
    result_codes: Option<HorizonResultCodes>,
}

#[derive(Debug, Deserialize, Default)]
struct HorizonResultCodes {
    #[serde(default)]
    transaction: Option<String>,
    #[serde(default)]
    operations: Option<Vec<String>>,
}

impl HorizonError {
    fn fallback(status: u16, raw: &str) -> Self {
        const MAX_BODY_PREVIEW: usize = 200;
        let preview = raw.chars().take(MAX_BODY_PREVIEW).collect::<String>();
        HorizonError {
            title: Some(format!(
                "Horizon returned HTTP {status} with an unparseable body: {preview}"
            )),
            extras: None,
        }
    }

    /// A human-readable summary combining Horizon's title with its structured result codes,
    /// e.g. `"Transaction Failed (tx_failed): op_underfunded"`.
    fn describe(&self) -> String {
        let title = self
            .title
            .clone()
            .unwrap_or_else(|| "unknown error".to_string());

        let Some(codes) = self.extras.as_ref().and_then(|e| e.result_codes.as_ref()) else {
            return title;
        };

        let tx_code = codes.transaction.as_deref().unwrap_or("unknown");
        match &codes.operations {
            Some(ops) if !ops.is_empty() => {
                format!("{title} ({tx_code}): {}", ops.join(", "))
            }
            _ => format!("{title} ({tx_code})"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stellarforge_core::NetworkConfig;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn mock_config(server: &MockServer) -> NetworkConfig {
        NetworkConfig::custom(
            server.uri(),
            "http://localhost:8000/soroban/rpc",
            "Standalone Network ; February 2017",
        )
    }

    #[tokio::test]
    async fn successful_submission_returns_hash_and_ledger() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/transactions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "hash": "abc123",
                "ledger": 42
            })))
            .mount(&server)
            .await;

        let config = mock_config(&server).await;
        let result = submit_transaction(&config, "AAAA...signed...")
            .await
            .unwrap();

        assert_eq!(result.hash, "abc123");
        assert_eq!(result.ledger, 42);
    }

    #[tokio::test]
    async fn rejected_submission_decodes_horizon_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/transactions"))
            .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
                "title": "Transaction Failed",
                "extras": {
                    "result_codes": {
                        "transaction": "tx_failed",
                        "operations": ["op_underfunded"]
                    }
                }
            })))
            .mount(&server)
            .await;

        let config = mock_config(&server).await;
        let err = submit_transaction(&config, "AAAA...signed...")
            .await
            .unwrap_err();

        assert!(matches!(err, TxError::Rejected(_)));
        assert_eq!(
            err.to_string(),
            "transaction rejected: Transaction Failed (tx_failed): op_underfunded"
        );
    }

    #[tokio::test]
    async fn rejected_submission_without_result_codes_falls_back_to_title() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/transactions"))
            .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
                "title": "Bad Request"
            })))
            .mount(&server)
            .await;

        let config = mock_config(&server).await;
        let err = submit_transaction(&config, "AAAA...signed...")
            .await
            .unwrap_err();

        assert_eq!(err.to_string(), "transaction rejected: Bad Request");
    }

    #[tokio::test]
    async fn unparseable_error_body_falls_back_to_status_code() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/transactions"))
            .respond_with(ResponseTemplate::new(503).set_body_string("<html>down</html>"))
            .mount(&server)
            .await;

        let config = mock_config(&server).await;
        let err = submit_transaction(&config, "AAAA...signed...")
            .await
            .unwrap_err();

        assert_eq!(
            err.to_string(),
            "transaction rejected: Horizon returned HTTP 503 with an unparseable body: <html>down</html>"
        );
    }
}
