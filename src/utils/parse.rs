//! XML and markdown parsing utilities.
//! Extracts structured data from clients prompts.

use std::collections::BTreeMap;

use regex::Regex;
use serde_json::{Map, Value};

/// Extract XML-like tags and their attributes/content as JSON.
pub fn extract_tags(raw: &str, tags: &[&str]) -> Value {
    let mut result = Map::new();
    let re_attr = regex::Regex::new(r#"(\w+)="([^"]*)""#).unwrap();

    for tag in tags {
        let open_prefix = format!("<{}", tag);
        let close = format!("</{}>", tag);
        let mut items = Vec::new();

        let mut search_from = 0;
        while let Some(start) = raw[search_from..].find(&open_prefix) {
            let abs_start = search_from + start;

            let open_tag_end = match raw[abs_start..].find('>') {
                Some(p) => abs_start + p,
                None => break,
            };

            let open_tag_content = &raw[abs_start..open_tag_end];
            let mut attrs = Map::new();

            for cap in re_attr.captures_iter(open_tag_content) {
                attrs.insert(cap[1].to_string(), Value::String(cap[2].to_string()));
            }

            let content_start = open_tag_end + 1;

            let content = if let Some(end) = raw[content_start..].find(&close) {
                let content = raw[content_start..content_start + end].to_string();
                search_from = content_start + end + close.len();
                content
            } else {
                let content = raw[content_start..].to_string();
                search_from = raw.len();
                content
            };

            attrs.insert("content".to_string(), Value::String(content));
            items.push(Value::Object(attrs));
        }

        if !items.is_empty() {
            if *tag == "file_content" {
                result.insert(tag.to_string(), Value::Array(items));
            } else if let Some(first) = items.first() {
                if let Some(content) = first.get("content") {
                    result.insert(tag.to_string(), content.clone());
                }
            }
        }
    }

    Value::Object(result)
}

/// Extract markdown sections from environment_details text.
pub fn extract_sections(raw: &str, sections: &[&str]) -> Value {
    let mut result = Map::new();

    for section in sections {
        let marker = format!("# {}", section);
        let mut inside_section = false;
        let mut section_lines = Vec::new();

        for line in raw.lines() {
            if line.starts_with("# ") {
                if line == marker {
                    inside_section = true;
                } else if inside_section {
                    break;
                }
            } else if inside_section {
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    section_lines.push(trimmed.to_string());
                }
            }
        }
        if !section_lines.is_empty() {
            if section_lines.len() == 1 {
                result.insert(section.to_string(), Value::String(section_lines.remove(0)));
            } else {
                let json_array = section_lines.into_iter().map(Value::String).collect();
                result.insert(section.to_string(), Value::Array(json_array));
            }
        }
    }

    Value::Object(result)
}

/// Remove all known XML tags and Cline boilerplate from text.
pub fn strip_tags(raw: &str, tags: &[&str]) -> Option<String> {
    let mut result = raw.to_string();

    // Remove XML tags
    for tag in tags {
        let open = format!("<{}", tag);
        let close = format!("</{}>", tag);

        while let (Some(start), Some(end)) = (result.find(&open), result.find(&close)) {
            let close_end = end + close.len();
            result = format!("{}{}", &result[..start], &result[close_end..]);
        }
    }

    // Cut off at # task_progress
    if let Some(pos) = result.find("# task_progress") {
        result = result[..pos].to_string();
    }

    // Collapse multiple blank lines
    let re = Regex::new(r"\n{3,}").unwrap();
    result = re.replace_all(&result, "\n\n").to_string();

    let data = result.trim().to_string();
    (!data.is_empty()).then_some(data)
}

/// Extracts sections between the specified tags in their order of appearance and returns a JSON object.
pub fn extract_sections_by_tags(raw: &str, tags: &[&str]) -> Value {
    let mut result_map = Map::new();
    let mut found_tags = BTreeMap::new();
    for &tag in tags {
        for (idx, matched_tag) in raw.match_indices(tag) {
            found_tags.insert(idx, matched_tag);
        }
    }
    let sorted_tags: Vec<(usize, &str)> = found_tags.into_iter().collect();
    for i in 0..sorted_tags.len() {
        let (current_idx, current_tag) = sorted_tags[i];
        let start_pos = current_idx + current_tag.len();
        let content = if i + 1 < sorted_tags.len() {
            let (next_idx, _) = sorted_tags[i + 1];
            if next_idx > start_pos {
                raw[start_pos..next_idx].trim().to_string()
            } else {
                String::new()
            }
        } else {
            raw[start_pos..].trim().to_string()
        };
        if !content.is_empty() {
            result_map
                .entry(current_tag.to_string())
                .and_modify(|existing| {
                    if let Value::String(existing_str) = existing {
                        *existing_str = format!("{}\n\n{}", existing_str, content);
                    }
                })
                .or_insert(Value::String(content));
        }
    }
    Value::Object(result_map)
}

/// Filters lines by uppercase word presence and completely removes lines matching any excluded words.
pub fn extract_caps_lines(text: &str, enable_caps: bool, exclude_words: &[&str]) -> String {
    text.lines()
        .filter(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                return false;
            }
            if enable_caps {
                let has_caps_word = trimmed.split_whitespace().any(|word| {
                    let clean_word: String = word.chars().filter(|c| c.is_alphabetic()).collect();
                    clean_word.chars().count() >= 2 && clean_word.chars().all(|c| c.is_uppercase())
                });
                if !has_caps_word {
                    return false;
                }
            }
            if exclude_words.iter().any(|&word| trimmed.contains(word)) {
                return false;
            }
            true
        })
        .collect::<Vec<&str>>()
        .join("\n")
}
