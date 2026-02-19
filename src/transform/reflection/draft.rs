use super::super::helpers::estimate_text_tokens;
use super::super::types::ReflectionDraft;

pub fn merge_buffered_reflection(
    active_lines: &[String],
    reflected_line_count: usize,
    buffered_reflection: &str,
) -> String {
    let reflection = buffered_reflection.trim();
    if reflection.is_empty() {
        return active_lines.join("\n").trim().to_string();
    }

    let split_at = reflected_line_count.min(active_lines.len());
    let unreflected = active_lines[split_at..].join("\n");
    let unreflected = unreflected.trim();

    if unreflected.is_empty() {
        reflection.to_string()
    } else {
        format!("{reflection}\n\n{unreflected}")
    }
}

pub fn build_reflection_draft(
    active_observations: &str,
    max_chars: usize,
) -> Option<ReflectionDraft> {
    if max_chars == 0 {
        return None;
    }
    let lines = active_observations
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();
    if lines.is_empty() {
        return None;
    }

    let reflection_input = lines.join(" ");
    let reflection = reflection_input.chars().take(max_chars).collect::<String>();
    if reflection.is_empty() {
        return None;
    }

    // Count only fully represented source lines so downstream merge never over-replaces.
    let mut reflected_line_count = 0usize;
    let mut consumed_chars = 0usize;
    for line in &lines {
        let line_chars = line.chars().count();
        let required = if reflected_line_count == 0 {
            line_chars
        } else {
            line_chars.saturating_add(1)
        };
        if consumed_chars.saturating_add(required) <= max_chars {
            consumed_chars = consumed_chars.saturating_add(required);
            reflected_line_count += 1;
        } else {
            break;
        }
    }
    if reflected_line_count == 0 {
        return None;
    }

    Some(ReflectionDraft {
        reflection_token_count: estimate_text_tokens(&reflection),
        reflected_observation_line_count: reflected_line_count.min(u32::MAX as usize) as u32,
        reflection_input_tokens: estimate_text_tokens(&reflection_input),
        reflection,
    })
}
