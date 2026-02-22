use napi::bindgen_prelude::*;
use crate::framework_data;
use crate::traverse::VueFile;
use regex::Regex;
use once_cell::sync::Lazy;

// ---- Lazy-compiled regex statics ----

static RE_ATTR_VALUE_GT: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#".*=("[^"]*>|'[^']*>)$"#).unwrap()
});

static RE_TAG_FULL: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"<([\w-]+)(\s*|(\s+[\w_:@.-]+(=("[^"]*"|'[^']*'))?)+)\s*>"#).unwrap()
});

static RE_TAG_FULL_END: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"<([\w-]+)(\s*|(\s+[\w_:@.-]+(=("[^"]*"|'[^']*'))?)+)\s*>$"#).unwrap()
});

static RE_PRE_TAG: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"<([\w-]+)\s+").unwrap()
});

static RE_PRE_TAG_BREAK: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"</?[-\w]+[^<>]*>[\s\w]*<?\s*[\w-]*$").unwrap()
});

static RE_ATTR_CLEAN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#""[^'"]*\s*[^'"]*$"#).unwrap()
});

static RE_ATTR_MATCH: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?:\(|\s*)((\w(-)?)*)=['"][^'"]*"#).unwrap()
});

static RE_FULL_QUOTE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#""[^"]*""#).unwrap()
});

static RE_SCRIPT_START: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^\s*<script.*>\s*$").unwrap()
});

static RE_PROP_NAME: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\s([\w-]*):").unwrap()
});

static RE_EMIT: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"\$emit\(\s?['"](\w*)"#).unwrap()
});

/// A completion suggestion returned to TypeScript
#[napi(object)]
#[derive(Clone, Debug)]
pub struct CompletionSuggestion {
    pub label: String,
    pub sort_text: String,
    pub insert_text: String,
    /// "snippet", "property", "method", "value", "reference", "folder"
    pub kind: String,
    pub detail: String,
    pub documentation: String,
}

/// Tag match result
#[napi(object)]
#[derive(Clone, Debug)]
pub struct TagMatch {
    pub text: String,
    pub offset: i32,
}

/// Check if text before cursor ends with a close tag
pub fn is_close_tag_check(text: &str) -> bool {
    let txt = text.trim();
    if !txt.ends_with('>') {
        return false;
    }
    // Check if it ends with attribute value containing >
    if RE_ATTR_VALUE_GT.is_match(txt) || txt.ends_with("/>") {
        return false;
    }
    if let Some(caps) = RE_TAG_FULL.captures_iter(txt).last() {
        let tag_str = caps.get(0).unwrap().as_str();
        return RE_TAG_FULL_END.is_match(tag_str);
    }
    false
}

/// Get the tag name for auto-closing
pub fn get_close_tag(line_text: &str) -> String {
    if let Some(caps) = RE_TAG_FULL.captures_iter(line_text).last() {
        if let Some(m) = caps.get(1) {
            let tag = m.as_str();
            let exclude = ["br", "img"];
            if !exclude.contains(&tag) {
                return tag.to_string();
            }
        }
    }
    "div".to_string()
}

/// Match pre-tag from the text (find the containing tag before cursor)
pub fn match_pre_tag_from_text(text: &str) -> Option<TagMatch> {
    // Check if we should break (already past a complete tag)
    if RE_PRE_TAG_BREAK.is_match(text) {
        return None;
    }

    let mut last: Option<TagMatch> = None;
    for cap in RE_PRE_TAG.captures_iter(text) {
        if let Some(m) = cap.get(1) {
            last = Some(TagMatch {
                text: m.as_str().to_string(),
                offset: m.start() as i32,
            });
        }
    }
    last
}

/// Match attribute before cursor
pub fn match_pre_attr_from_text(text: &str) -> Option<String> {
    let cleaned = RE_ATTR_CLEAN
        .replace(text, "")
        .to_string();
    if let Some(caps) = RE_ATTR_MATCH.captures(&cleaned) {
        if let Some(m) = caps.get(1) {
            let attr = m.as_str();
            // Check it's not inside a complete attribute value
            if !RE_FULL_QUOTE.is_match(&cleaned) && !attr.is_empty() {
                return Some(attr.to_string());
            }
        }
    }
    None
}

/// Check if a line is an import statement
pub fn is_import_check(text: &str) -> bool {
    let trimmed = text.trim();
    trimmed.starts_with("import")
}

/// Check if position is not in template (i.e., in script section)
pub fn not_in_template(lines: &[String], current_line: usize) -> bool {
    let mut line = current_line;
    while line > 0 {
        if RE_SCRIPT_START.is_match(&lines[line]) {
            return true;
        }
        if line == 0 { break; }
        line -= 1;
    }
    false
}

/// Get tag completion suggestions
pub fn get_tag_suggestions(
    frameworks: &[String],
    tab_size: &str,
    use_vue_snippets: bool,
) -> Vec<CompletionSuggestion> {
    let mut suggestions = Vec::new();
    let mut id = 1;

    if use_vue_snippets {
        let snippets = framework_data::get_vue_snippets_html(tab_size);
        for (key, snippet) in &snippets {
            suggestions.push(CompletionSuggestion {
                label: key.clone(),
                sort_text: format!("0{}{}", id, key),
                insert_text: snippet.clone(),
                kind: "snippet".to_string(),
                detail: "vue-helper".to_string(),
                documentation: String::new(),
            });
            id += 1;
        }
    }

    let tags = framework_data::get_tags(frameworks, tab_size);
    for (tag, snippet) in &tags {
        suggestions.push(CompletionSuggestion {
            label: tag.clone(),
            sort_text: format!("00{}{}", id, tag),
            insert_text: snippet.clone(),
            kind: "snippet".to_string(),
            detail: "vue-helper".to_string(),
            documentation: String::new(),
        });
        id += 1;
    }

    suggestions
}

/// Get JS tag completion suggestions
pub fn get_js_tag_suggestions(
    frameworks: &[String],
    tab_size: &str,
    use_vue_snippets: bool,
) -> Vec<CompletionSuggestion> {
    let mut suggestions = Vec::new();
    let mut id = 1;

    if use_vue_snippets {
        let snippets = framework_data::get_vue_snippets_js(tab_size);
        for (key, snippet) in &snippets {
            suggestions.push(CompletionSuggestion {
                label: key.clone(),
                sort_text: format!("0{}{}", id, key),
                insert_text: snippet.clone(),
                kind: "snippet".to_string(),
                detail: "vue-helper".to_string(),
                documentation: String::new(),
            });
            id += 1;
        }
    }

    let js_tags = framework_data::get_js_tags(frameworks, tab_size);
    for (tag, snippet) in &js_tags {
        suggestions.push(CompletionSuggestion {
            label: tag.clone(),
            sort_text: format!("00{}{}", id, tag),
            insert_text: snippet.clone(),
            kind: "snippet".to_string(),
            detail: "vue-helper".to_string(),
            documentation: snippet.clone(),
        });
        id += 1;
    }

    suggestions
}

/// Get attribute suggestions for a given tag
pub fn get_attr_suggestions(
    tag: &str,
    frameworks: &[String],
    tab_size: &str,
    prefix_type: &str,
) -> Vec<CompletionSuggestion> {
    let mut suggestions = Vec::new();
    let attr_type = if prefix_type.starts_with('@') { "method" } else { "attribute" };

    let attributes = framework_data::get_attributes(frameworks, tab_size);
    if let Some(tag_attrs) = attributes.get(tag) {
        for (name, info) in tag_attrs {
            if name == "_self" { continue; }
            if info.attr_type == attr_type {
                suggestions.push(CompletionSuggestion {
                    label: name.clone(),
                    sort_text: format!("000{}", name),
                    insert_text: name.clone(),
                    kind: if attr_type == "method" { "method".to_string() } else { "property".to_string() },
                    detail: "vue-helper".to_string(),
                    documentation: info.description.clone(),
                });
            }
        }
    }

    // Global attributes
    let global_attrs = framework_data::get_global_attributes(frameworks, tab_size);
    for (name, info) in &global_attrs {
        if info.attr_type == attr_type {
            suggestions.push(CompletionSuggestion {
                label: name.clone(),
                sort_text: format!("000{}", name),
                insert_text: name.clone(),
                kind: if attr_type == "method" { "method".to_string() } else { "property".to_string() },
                detail: "vue-helper".to_string(),
                documentation: info.description.clone(),
            });
        }
    }

    suggestions
}

/// Get attribute value suggestions
pub fn get_attr_value_suggestions(
    tag: &str,
    attr: &str,
    frameworks: &[String],
    tab_size: &str,
) -> Vec<CompletionSuggestion> {
    let mut suggestions = Vec::new();
    let mut values = Vec::new();

    // Check global attributes first
    let global_attrs = framework_data::get_global_attributes(frameworks, tab_size);
    if let Some(info) = global_attrs.get(attr) {
        values = info.values.clone();
    }

    // Then check tag-specific
    let attributes = framework_data::get_attributes(frameworks, tab_size);
    if let Some(tag_attrs) = attributes.get(tag) {
        if let Some(info) = tag_attrs.get(attr) {
            values = info.values.clone();
        }
    }

    for value in &values {
        suggestions.push(CompletionSuggestion {
            label: value.clone(),
            sort_text: format!("000{}", value),
            insert_text: value.clone(),
            kind: "value".to_string(),
            detail: "vue-helper".to_string(),
            documentation: String::new(),
        });
    }

    suggestions
}

/// Get element tag label suggestions (tag names from attribute data)
pub fn get_element_tag_label_suggestions(
    frameworks: &[String],
    tab_size: &str,
    extension_name: &str,
) -> Vec<CompletionSuggestion> {
    let mut suggestions = Vec::new();
    let attributes = framework_data::get_attributes(frameworks, tab_size);
    let mut labels = Vec::new();
    let mut id = 1;

    for tag in attributes.keys() {
        let label = tag.split(':').next().unwrap_or(tag);
        if !labels.contains(&label.to_string()) {
            labels.push(label.to_string());
            suggestions.push(CompletionSuggestion {
                label: label.to_string(),
                sort_text: format!("00{}{}", id, label),
                insert_text: format!("{}$0></{}>", label, label),
                kind: "snippet".to_string(),
                detail: extension_name.to_string(),
                documentation: String::new(),
            });
            id += 1;
        }
    }

    suggestions
}

/// Extract props from a vue file content
pub fn extract_props_from_vue(content: &str) -> Vec<CompletionSuggestion> {
    let mut props = Vec::new();

    // Find script section and extract props
    if let Some(script_idx) = content.find("<script") {
        let doc_text = &content[script_idx..];
        if let Some(prop_idx) = doc_text.find("props") {
            let after_props = &doc_text[prop_idx..];
            if let Some(brace_start) = after_props.find('{') {
                // Parse props block by tracking braces
                let chars: Vec<char> = after_props[brace_start..].chars().collect();
                let mut depth = 0;
                let mut prop_text = String::new();
                let mut in_top_level = false;

                for ch in &chars {
                    match ch {
                        '{' => {
                            depth += 1;
                            if depth == 1 { in_top_level = true; continue; }
                            if depth >= 2 { continue; }
                        }
                        '}' => {
                            depth -= 1;
                            if depth == 0 { break; }
                            if depth >= 1 { continue; }
                        }
                        _ => {
                            if depth == 1 {
                                prop_text.push(*ch);
                            }
                        }
                    }
                }

                // Extract prop names
                let mut idx = 0;
                for cap in RE_PROP_NAME.captures_iter(&prop_text) {
                    if let Some(m) = cap.get(1) {
                        let mut prop_name = m.as_str().to_string();
                        // Convert camelCase to kebab-case
                        let kebab = to_kebab_case(&prop_name);
                        props.push(CompletionSuggestion {
                            label: kebab.clone(),
                            sort_text: format!("0{}", idx),
                            insert_text: format!(":{}=\"$0\"", kebab),
                            kind: "property".to_string(),
                            detail: String::new(),
                            documentation: String::new(),
                        });
                        idx += 1;
                    }
                }
            }
        }
    }

    // Extract $emit events
    for cap in RE_EMIT.captures_iter(content) {
        if let Some(m) = cap.get(1) {
            let emit_name = m.as_str();
            props.push(CompletionSuggestion {
                label: emit_name.to_string(),
                sort_text: format!("0{}", props.len() + 1),
                insert_text: format!("@{}=\"$0\"", emit_name),
                kind: "method".to_string(),
                detail: String::new(),
                documentation: String::new(),
            });
        }
    }

    props
}

fn to_kebab_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                result.push('-');
            }
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }
    result
}

/// Get import suggestion items
pub fn get_import_suggestion_items(
    search_text: &str,
    vue_files: &[VueFile],
    document_path: &str,
    project_root: &str,
) -> Vec<CompletionSuggestion> {
    let mut suggestions = Vec::new();
    let search = search_text.trim().strip_prefix("import").unwrap_or(search_text).trim();

    if search.is_empty() {
        return suggestions;
    }

    for vf in vue_files {
        if !vf.path.contains(search) && !vf.name.contains(search) {
            continue;
        }

        let mut insert_path = vf.path.clone();
        if insert_path.ends_with(".ts") {
            insert_path = insert_path[..insert_path.len() - 3].to_string();
        }

        suggestions.push(CompletionSuggestion {
            label: vf.name.clone(),
            sort_text: format!("0{}", vf.name),
            insert_text: format!("${{1:{}}} from '{}'", vf.name, insert_path),
            kind: "reference".to_string(),
            detail: vf.name.clone(),
            documentation: format!("import {} from {}", vf.name, vf.path),
        });
    }

    suggestions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_close_tag() {
        assert!(is_close_tag_check("<div>"));
        assert!(!is_close_tag_check("<div/>"));
        assert!(!is_close_tag_check("<div"));
    }

    #[test]
    fn test_get_close_tag() {
        assert_eq!(get_close_tag("<div>"), "div");
        assert_eq!(get_close_tag("<el-button type=\"primary\">"), "el-button");
    }

    #[test]
    fn test_is_import() {
        assert!(is_import_check("import Vue from 'vue'"));
        assert!(is_import_check("  import { ref } from 'vue'"));
        assert!(!is_import_check("const x = 1"));
    }

    #[test]
    fn test_to_kebab_case() {
        assert_eq!(to_kebab_case("myProp"), "my-prop");
        assert_eq!(to_kebab_case("userName"), "user-name");
        assert_eq!(to_kebab_case("name"), "name");
    }

    #[test]
    fn test_get_tag_suggestions() {
        let frameworks = vec!["element-ui".to_string()];
        let suggestions = get_tag_suggestions(&frameworks, "  ", false);
        assert!(!suggestions.is_empty());
    }
}
