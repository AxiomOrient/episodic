use crate::OmPendingMessage;
use crate::xml::{escape_xml_attribute, escape_xml_text};

use super::OmObserverThreadMessages;

fn normalize_role(role: &str) -> String {
    let mut chars = role.trim().chars();
    let Some(first) = chars.next() else {
        return "Unknown".to_string();
    };
    let mut normalized = first.to_uppercase().collect::<String>();
    normalized.push_str(chars.as_str());
    normalized
}

fn format_timestamp_for_observer(raw: &str) -> Option<String> {
    chrono::DateTime::parse_from_rfc3339(raw)
        .ok()
        .map(|value| value.with_timezone(&chrono::Utc))
        .map(|value| value.format("%b %-d, %Y, %-I:%M %p").to_string())
}

fn normalize_message_id_for_prompt(raw: &str) -> Option<String> {
    let mut out = String::new();
    let mut last_was_separator = false;

    for ch in raw.trim().chars() {
        let allowed = ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.' | '/');
        if allowed {
            out.push(ch);
            last_was_separator = false;
        } else if !last_was_separator {
            out.push('_');
            last_was_separator = true;
        }
    }

    let normalized = out.trim_matches('_');
    if normalized.is_empty() {
        None
    } else {
        Some(normalized.to_string())
    }
}

pub fn format_observer_messages_for_prompt(messages: &[OmPendingMessage]) -> String {
    messages
        .iter()
        .map(|message| {
            let role = normalize_role(&message.role);
            let id = normalize_message_id_for_prompt(&message.id);
            let text = message.text.trim();
            let timestamp = message
                .created_at_rfc3339
                .as_deref()
                .and_then(format_timestamp_for_observer);
            let timestamp_suffix = timestamp
                .map(|value| format!(" ({value})"))
                .unwrap_or_default();
            let id_suffix = id.map(|value| format!(" [id:{value}]")).unwrap_or_default();
            format!("**{role}{timestamp_suffix}{id_suffix}:**\n{text}")
        })
        .collect::<Vec<_>>()
        .join("\n\n---\n\n")
}

pub fn format_multi_thread_observer_messages_for_prompt(
    threads: &[OmObserverThreadMessages],
) -> String {
    threads
        .iter()
        .filter_map(|thread| {
            let thread_id = thread.thread_id.trim();
            let message_history = thread.message_history.trim();
            if thread_id.is_empty() || message_history.is_empty() {
                return None;
            }
            let thread_id = escape_xml_attribute(thread_id);
            let message_history = escape_xml_text(message_history);
            Some(format!(
                "<thread id=\"{thread_id}\">\n{message_history}\n</thread>"
            ))
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}
