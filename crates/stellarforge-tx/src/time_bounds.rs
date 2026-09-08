//! Transaction time bounds.
//!
//! Time bounds restrict the window (as Unix timestamps) during which a transaction is valid for
//! submission to the network, which is what makes signed-but-unsubmitted transactions safely
//! expire.

use crate::error::TxError;

/// A valid submission window for a transaction, as Unix timestamps (seconds since epoch).
///
/// `max_time == 0` means "no upper bound" in Stellar's XDR representation, but StellarForge
/// requires an explicit upper bound via [`TimeBounds::new`] or [`TimeBounds::valid_for`] to
/// avoid transactions that never expire.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeBounds {
    pub min_time: u64,
    pub max_time: u64,
}

impl TimeBounds {
    /// Builds explicit time bounds, validating that `max_time` is strictly after `min_time`.
    pub fn new(min_time: u64, max_time: u64) -> Result<Self, TxError> {
        if max_time <= min_time {
            return Err(TxError::InvalidMemo(format!(
                "max_time ({max_time}) must be after min_time ({min_time})"
            )));
        }
        Ok(Self { min_time, max_time })
    }

    /// Time bounds valid from `now` for `valid_seconds` seconds.
    pub fn valid_for(now: u64, valid_seconds: u64) -> Result<Self, TxError> {
        Self::new(now, now + valid_seconds)
    }

    /// Returns `true` if `timestamp` falls within `[min_time, max_time]`.
    pub fn contains(&self, timestamp: u64) -> bool {
        timestamp >= self.min_time && timestamp <= self.max_time
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_rejects_max_before_min() {
        let err = TimeBounds::new(100, 50).unwrap_err();
        assert!(matches!(err, TxError::InvalidMemo(_)));
    }

    #[test]
    fn new_rejects_equal_bounds() {
        assert!(TimeBounds::new(100, 100).is_err());
    }

    #[test]
    fn valid_for_computes_max_time() {
        let bounds = TimeBounds::valid_for(1_000, 300).unwrap();
        assert_eq!(bounds.min_time, 1_000);
        assert_eq!(bounds.max_time, 1_300);
    }

    #[test]
    fn contains_checks_inclusive_range() {
        let bounds = TimeBounds::new(100, 200).unwrap();
        assert!(bounds.contains(100));
        assert!(bounds.contains(200));
        assert!(bounds.contains(150));
        assert!(!bounds.contains(99));
        assert!(!bounds.contains(201));
    }
}
