//! Trusted synthetic fixture catalog for the preview routes (F-13, F-14; TS-53).
//!
//! The catalog is compiled in: a preview never reads an upload, a path or the file system.
//! `lookup` is an exact match on a closed set, so ids that look like paths or package names
//! are simply unknown.

/// Preflight outcome a trusted fixture is canned to produce.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreviewResult {
    Compatible,
    Incompatible,
    Corrupt,
}

impl PreviewResult {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Compatible => "compatible",
            Self::Incompatible => "incompatible",
            Self::Corrupt => "corrupt",
        }
    }
}

/// One trusted synthetic fixture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixtureEntry {
    pub id: &'static str,
    pub label: &'static str,
    pub result: PreviewResult,
    pub data_scope: &'static [&'static str],
}

/// Data in scope for every U1 fixture: database rows only, all synthetic.
const SYNTHETIC_DATABASE_SCOPE: &[&str] = &["database (synthetic rows only)"];

static CATALOG: [FixtureEntry; 3] = [
    FixtureEntry {
        id: "fixture-valid",
        label: "Synthetic valid fixture",
        result: PreviewResult::Compatible,
        data_scope: SYNTHETIC_DATABASE_SCOPE,
    },
    FixtureEntry {
        id: "fixture-incompatible",
        label: "Synthetic incompatible fixture",
        result: PreviewResult::Incompatible,
        data_scope: SYNTHETIC_DATABASE_SCOPE,
    },
    FixtureEntry {
        id: "fixture-corrupt",
        label: "Synthetic corrupt fixture",
        result: PreviewResult::Corrupt,
        data_scope: SYNTHETIC_DATABASE_SCOPE,
    },
];

/// Trusted fixture ids, in catalog order.
pub const FIXTURE_IDS: &[&str] = &["fixture-valid", "fixture-incompatible", "fixture-corrupt"];

/// Exact-match lookup. Anything not in the catalog is `None`.
pub fn lookup(id: &str) -> Option<&'static FixtureEntry> {
    CATALOG.iter().find(|entry| entry.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TS-53 (F-13): the catalog is exactly the three trusted synthetic fixtures and the
    /// lookup is an exact match: ids that look like paths, uploads or variants are unknown.
    #[test]
    fn ts53_catalog_trusts_exactly_three_synthetic_fixtures_by_exact_id() {
        assert_eq!(
            FIXTURE_IDS,
            ["fixture-valid", "fixture-incompatible", "fixture-corrupt"]
        );
        let expected = [
            (
                "fixture-valid",
                "Synthetic valid fixture",
                PreviewResult::Compatible,
            ),
            (
                "fixture-incompatible",
                "Synthetic incompatible fixture",
                PreviewResult::Incompatible,
            ),
            (
                "fixture-corrupt",
                "Synthetic corrupt fixture",
                PreviewResult::Corrupt,
            ),
        ];
        for (id, label, result) in expected {
            let entry = lookup(id).unwrap_or_else(|| panic!("{id} is trusted"));
            assert_eq!(entry.id, id);
            assert_eq!(entry.label, label);
            assert_eq!(entry.result, result);
            assert_eq!(entry.data_scope, ["database (synthetic rows only)"]);
        }
        assert_eq!(PreviewResult::Compatible.as_str(), "compatible");
        assert_eq!(PreviewResult::Incompatible.as_str(), "incompatible");
        assert_eq!(PreviewResult::Corrupt.as_str(), "corrupt");
        for id in [
            "",
            "unknown",
            "FIXTURE-VALID",
            "fixture-valid ",
            " fixture-valid",
            "fixture-valid/",
            "../fixture-valid",
            "/tmp/backup.tar.gz",
            "fixture-valid\u{0}",
            "fixture-",
        ] {
            assert!(lookup(id).is_none(), "{id:?} must not be trusted");
        }
    }
}
