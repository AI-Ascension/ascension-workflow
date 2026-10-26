//! Receipt validation only; owner values remain opaque JSON.
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

pub const ROLES: [&str; 3] = ["protocol", "studio", "observability"];
const PROJECTION: [&str; 11] = [
    "digest",
    "identity",
    "identities",
    "counts",
    "evidence",
    "accounting",
    "omissions",
    "observations",
    "events",
    "provenance",
    "completeness",
];

pub fn expectations(document: &Value) -> Result<BTreeMap<String, bool>, String> {
    let cases = document["cases"]
        .as_array()
        .ok_or("missing conformance cases")?;
    let mut expected = BTreeMap::new();
    for case in cases {
        let path = case["path"].as_str().ok_or("invalid conformance path")?;
        let valid = case["valid"]
            .as_bool()
            .ok_or("invalid conformance verdict")?;
        if path.is_empty() || expected.insert(path.to_owned(), valid).is_some() {
            return Err("duplicate or empty conformance path".into());
        }
    }
    if expected.is_empty() {
        return Err("empty conformance cases".into());
    }
    Ok(expected)
}

pub fn validate(reports: &[Value], expected: &BTreeMap<String, bool>) -> Vec<String> {
    let mut findings = Vec::new();
    let mut owners = BTreeMap::new();
    for (index, report) in reports.iter().enumerate() {
        let Some(role) = report["role"].as_str() else {
            findings.push("missing owner role".into());
            continue;
        };
        if ROLES.get(index) != Some(&role) || owners.contains_key(role) {
            findings.push("unexpected or duplicate owner role".into());
            continue;
        }
        if report.get("setup_error").is_some() {
            findings.push(format!("{role} setup failed"));
            continue;
        }
        let Some(cases) = report["results"].as_array() else {
            findings.push(format!("{role} missing results"));
            continue;
        };
        let mut indexed = BTreeMap::new();
        let mut seen = BTreeSet::new();
        for case in cases {
            let Some(path) = case["path"].as_str() else {
                findings.push(format!("{role} invalid case path"));
                continue;
            };
            if !seen.insert(path) {
                findings.push("duplicate case path".into());
            }
            let Some(valid) = expected.get(path) else {
                findings.push("unexpected case path".into());
                continue;
            };
            if case["expected"].as_bool() != Some(*valid)
                || case["accepted"].as_bool() != Some(*valid)
            {
                findings.push(format!("{role} {path} acceptance mismatch"));
            }
            if *valid {
                let summary = &case["summary"];
                if !summary.is_object()
                    || PROJECTION
                        .iter()
                        .any(|key| summary.get(key).is_none_or(Value::is_null))
                {
                    findings.push(format!("{role} {path} missing projection"));
                }
                if ["digest", "identity"]
                    .iter()
                    .any(|k| !summary[k].is_string())
                    || [
                        "counts",
                        "evidence",
                        "omissions",
                        "provenance",
                        "completeness",
                    ]
                    .iter()
                    .any(|k| !summary[k].is_object())
                    || ["events", "accounting", "observations"]
                        .iter()
                        .any(|k| !summary[k].is_array())
                {
                    findings.push(format!("{role} {path} invalid projection type"));
                }
                if role == "protocol" || role == "observability" {
                    for key in ["original_events", "original_accounting"] {
                        if !summary[key].is_array() {
                            findings.push(format!("{role} {path} missing {key}"));
                        }
                    }
                }
                if role == "observability" {
                    let p = &summary["persistence"];
                    if p["first"] != "imported"
                        || p["retry"] != "duplicate"
                        || p["revisions"] != 1
                        || p["delivery"] != "local_only"
                    {
                        findings.push(format!("{role} {path} invalid persistence receipt"));
                    }
                }
                indexed.insert(path, summary);
            }
        }
        for path in expected.keys() {
            if !seen.contains(path.as_str()) {
                findings.push(format!("{role} missing {path}"));
            }
        }
        owners.insert(role, indexed);
    }
    for role in ROLES {
        if !owners.contains_key(role) {
            findings.push(format!("missing owner {role}"));
        }
    }
    if let Some(protocol) = owners.get("protocol") {
        for role in ["studio", "observability"] {
            if let Some(consumer) = owners.get(role) {
                for (path, baseline) in protocol {
                    if let Some(summary) = consumer.get(path) {
                        for key in PROJECTION.into_iter().chain(
                            ["original_events", "original_accounting"]
                                .into_iter()
                                .filter(|_| role == "observability"),
                        ) {
                            if summary[key] != baseline[key] {
                                findings.push(format!("{role} {path} {key} disagreement"));
                            }
                        }
                    }
                }
            }
        }
    }
    findings
}
