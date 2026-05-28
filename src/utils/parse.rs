//! XML and markdown parsing utilities.
//! Extracts structured data from clients prompts.

use regex::Regex;

/// Extract XML-like tags and their attributes/content as JSON.
pub fn extract_tags(raw: &str, tags: &[&str]) -> serde_json::Value {
    let mut result = serde_json::Map::new();
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
            let mut attrs = serde_json::Map::new();

            for cap in re_attr.captures_iter(open_tag_content) {
                attrs.insert(
                    cap[1].to_string(),
                    serde_json::Value::String(cap[2].to_string()),
                );
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

            attrs.insert("content".to_string(), serde_json::Value::String(content));
            items.push(serde_json::Value::Object(attrs));
        }

        if !items.is_empty() {
            if *tag == "file_content" {
                result.insert(tag.to_string(), serde_json::Value::Array(items));
            } else if let Some(first) = items.first() {
                if let Some(content) = first.get("content") {
                    result.insert(tag.to_string(), content.clone());
                }
            }
        }
    }

    serde_json::Value::Object(result)
}

/// Extract markdown sections from environment_details text.
pub fn extract_sections(raw: &str, sections: &[&str]) -> serde_json::Value {
    let mut result = serde_json::Map::new();

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
                result.insert(
                    section.to_string(),
                    serde_json::Value::String(section_lines.remove(0)),
                );
            } else {
                let json_array = section_lines
                    .into_iter()
                    .map(serde_json::Value::String)
                    .collect();
                result.insert(section.to_string(), serde_json::Value::Array(json_array));
            }
        }
    }

    serde_json::Value::Object(result)
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
