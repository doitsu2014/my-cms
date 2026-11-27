//! Closed, low-cardinality metric labels (FN-36, TS-57).

use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricJobKind {
    BackupDbSlice,
}

impl MetricJobKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BackupDbSlice => "backup_db_slice",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricOutcome {
    Accepted,
    Incomplete,
    Failed,
    Denied,
    Refused,
}

impl MetricOutcome {
    pub const ALL: [Self; 5] = [
        Self::Accepted,
        Self::Incomplete,
        Self::Failed,
        Self::Denied,
        Self::Refused,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Incomplete => "incomplete",
            Self::Failed => "failed",
            Self::Denied => "denied",
            Self::Refused => "refused",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricErrorCode {
    None,
    BackupDisabled,
    BackupNotReady,
    StoreUnavailable,
    AuditUnavailable,
    ValidationError,
    Forbidden,
    NotFound,
    Conflict,
    PayloadTooLarge,
    UnsupportedMediaType,
}

impl MetricErrorCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "NONE",
            Self::BackupDisabled => "BACKUP_DISABLED",
            Self::BackupNotReady => "BACKUP_NOT_READY",
            Self::StoreUnavailable => "STORE_UNAVAILABLE",
            Self::AuditUnavailable => "AUDIT_UNAVAILABLE",
            Self::ValidationError => "MALFORMED_REQUEST",
            Self::Forbidden => "FORBIDDEN",
            Self::NotFound => "JOB_NOT_FOUND",
            Self::Conflict => "CONCURRENCY_LIMIT",
            Self::PayloadTooLarge => "PAYLOAD_TOO_LARGE",
            Self::UnsupportedMediaType => "UNSUPPORTED_MEDIA_TYPE",
        }
    }
}

/// Closed fields only; identifiers, paths, actors and arbitrary strings cannot be labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MetricLabels {
    pub job_kind: MetricJobKind,
    pub outcome: MetricOutcome,
    pub error_code: MetricErrorCode,
}

impl MetricLabels {
    /// Fixed exporter attribute names and enum-backed low-cardinality values.
    pub fn attributes(self) -> [(&'static str, &'static str); 3] {
        [
            ("job_kind", self.job_kind.as_str()),
            ("outcome", self.outcome.as_str()),
            ("error_code", self.error_code.as_str()),
        ]
    }
}

/// In-memory observable counter dimensions remain a closed product of enums.
#[derive(Debug)]
pub struct MetricRecorder {
    counts: [AtomicU64; 5 * 11],
}

impl Default for MetricRecorder {
    fn default() -> Self {
        Self {
            counts: std::array::from_fn(|_| AtomicU64::new(0)),
        }
    }
}

impl MetricRecorder {
    pub fn record(&self, labels: MetricLabels) {
        self.counts[metric_index(labels)].fetch_add(1, Ordering::Relaxed);
    }

    pub fn count(&self, labels: MetricLabels) -> u64 {
        self.counts[metric_index(labels)].load(Ordering::Relaxed)
    }

    pub fn snapshot(&self) -> Vec<(MetricLabels, u64)> {
        let mut values = Vec::new();
        for outcome in MetricOutcome::ALL {
            for error_code in [
                MetricErrorCode::None,
                MetricErrorCode::BackupDisabled,
                MetricErrorCode::BackupNotReady,
                MetricErrorCode::StoreUnavailable,
                MetricErrorCode::AuditUnavailable,
                MetricErrorCode::ValidationError,
                MetricErrorCode::Forbidden,
                MetricErrorCode::NotFound,
                MetricErrorCode::Conflict,
                MetricErrorCode::PayloadTooLarge,
                MetricErrorCode::UnsupportedMediaType,
            ] {
                let labels = MetricLabels {
                    job_kind: MetricJobKind::BackupDbSlice,
                    outcome,
                    error_code,
                };
                let count = self.count(labels);
                if count != 0 {
                    values.push((labels, count));
                }
            }
        }
        values
    }
}

fn metric_index(labels: MetricLabels) -> usize {
    let outcome = match labels.outcome {
        MetricOutcome::Accepted => 0,
        MetricOutcome::Incomplete => 1,
        MetricOutcome::Failed => 2,
        MetricOutcome::Denied => 3,
        MetricOutcome::Refused => 4,
    };
    let error = match labels.error_code {
        MetricErrorCode::None => 0,
        MetricErrorCode::BackupDisabled => 1,
        MetricErrorCode::BackupNotReady => 2,
        MetricErrorCode::StoreUnavailable => 3,
        MetricErrorCode::AuditUnavailable => 4,
        MetricErrorCode::ValidationError => 5,
        MetricErrorCode::Forbidden => 6,
        MetricErrorCode::NotFound => 7,
        MetricErrorCode::Conflict => 8,
        MetricErrorCode::PayloadTooLarge => 9,
        MetricErrorCode::UnsupportedMediaType => 10,
    };
    outcome * 11 + error
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ts57_metric_label_vocabulary_is_closed_and_identifier_free() {
        let labels = MetricLabels {
            job_kind: MetricJobKind::BackupDbSlice,
            outcome: MetricOutcome::Accepted,
            error_code: MetricErrorCode::ValidationError,
        };
        assert_eq!(
            labels.attributes(),
            [
                ("job_kind", "backup_db_slice"),
                ("outcome", "accepted"),
                ("error_code", "MALFORMED_REQUEST")
            ]
        );
        assert_eq!(MetricOutcome::ALL.len(), 5);
        assert_eq!(MetricRecorder::default().snapshot(), Vec::new());
    }

    #[test]
    fn ts57_metric_recorder_emits_only_closed_label_dimensions() {
        let recorder = MetricRecorder::default();
        let labels = MetricLabels {
            job_kind: MetricJobKind::BackupDbSlice,
            outcome: MetricOutcome::Accepted,
            error_code: MetricErrorCode::ValidationError,
        };
        recorder.record(labels);
        assert_eq!(recorder.count(labels), 1);
        let snapshot = recorder.snapshot();
        assert_eq!(snapshot, vec![(labels, 1)]);
        let attributes = snapshot[0].0.attributes();
        assert_eq!(attributes.len(), 3);
        assert!(attributes
            .iter()
            .all(|(key, _)| ["job_kind", "outcome", "error_code"].contains(key)));
    }
}
