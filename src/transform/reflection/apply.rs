use std::collections::BTreeSet;

use crate::model::{
    OmObservationEntryV2, OmObservationOriginKind, OmObservationPriority, OmReflectionResponseV2,
};

fn normalize_text(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn normalize_ids(ids: &[String]) -> Vec<String> {
    let mut normalized = ids
        .iter()
        .filter_map(|id| normalize_text(id))
        .collect::<Vec<_>>();
    normalized.sort();
    normalized.dedup();
    normalized
}

#[must_use]
pub fn apply_reflection_response_v2(
    entries: &[OmObservationEntryV2],
    response: &OmReflectionResponseV2,
    reflection_entry_id: &str,
    scope_key: &str,
    thread_id: &str,
    created_at_rfc3339: &str,
) -> Vec<OmObservationEntryV2> {
    let Some(reflection_id) = normalize_text(reflection_entry_id) else {
        return entries.to_vec();
    };
    let Some(reflection_text) = normalize_text(&response.reflection_text) else {
        return entries.to_vec();
    };

    let covered_ids = normalize_ids(&response.covers_entry_ids)
        .into_iter()
        .collect::<BTreeSet<_>>();
    if covered_ids.is_empty() {
        return entries.to_vec();
    }

    let mut next = entries.to_vec();
    let mut reflection_source_ids = BTreeSet::<String>::new();
    let mut matched_covered_entry = false;

    for entry in &mut next {
        if !covered_ids.contains(&entry.entry_id) {
            continue;
        }
        matched_covered_entry = true;
        entry.superseded_by = Some(reflection_id.clone());
        for source_id in &entry.source_message_ids {
            if let Some(normalized) = normalize_text(source_id) {
                reflection_source_ids.insert(normalized);
            }
        }
    }
    if !matched_covered_entry {
        return entries.to_vec();
    }

    if let Some(existing) = next
        .iter_mut()
        .find(|entry| entry.entry_id == reflection_id)
    {
        existing.scope_key =
            normalize_text(scope_key).unwrap_or_else(|| existing.scope_key.clone());
        existing.thread_id =
            normalize_text(thread_id).unwrap_or_else(|| existing.thread_id.clone());
        existing.priority = OmObservationPriority::Medium;
        existing.text = reflection_text;
        existing.origin_kind = OmObservationOriginKind::Reflection;
        existing.source_message_ids = reflection_source_ids.into_iter().collect::<Vec<_>>();
        existing.created_at_rfc3339 = normalize_text(created_at_rfc3339)
            .unwrap_or_else(|| existing.created_at_rfc3339.clone());
        existing.superseded_by = None;
        return next;
    }

    next.push(OmObservationEntryV2 {
        entry_id: reflection_id,
        scope_key: normalize_text(scope_key).unwrap_or_default(),
        thread_id: normalize_text(thread_id).unwrap_or_default(),
        priority: OmObservationPriority::Medium,
        text: reflection_text,
        source_message_ids: reflection_source_ids.into_iter().collect::<Vec<_>>(),
        origin_kind: OmObservationOriginKind::Reflection,
        created_at_rfc3339: normalize_text(created_at_rfc3339).unwrap_or_default(),
        superseded_by: None,
    });
    next
}
