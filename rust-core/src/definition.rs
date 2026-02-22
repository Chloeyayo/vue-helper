use napi::bindgen_prelude::*;
use regex::Regex;
use once_cell::sync::Lazy;
use std::path::Path;

// ---- Lazy-compiled regex statics ----

static RE_IMPORT_FROM: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"import\s+.*\s+from\s+['"](.*)['"']\s*;?\s*$"#).unwrap()
});

static RE_IMPORT_DYNAMIC: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"import\s*[^'"]*\(['"](.*)['"']\)[^'"]*"#).unwrap()
});

static RE_REQUIRE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#".*require\s*\([^'"]*['"](.*)['"]\)"#).unwrap()
});

static RE_IMPORT_BARE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"import\s+['"](.*)['"]\s*;?\s*$"#).unwrap()
});

static RE_IMPORT_COMMENT: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"import\s*\([^'"]*(?:/\*.*\*/)\s*['"](.*)['"]\)*"#).unwrap()
});

static RE_HAS_EXT: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(.*\/.*|[^.]+)\..*$").unwrap()
});

static RE_SCRIPT_END: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^\s*</script>\s*$").unwrap()
});

static RE_SCRIPT_START: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^\s*<script.*>\s*$").unwrap()
});

static RE_KEY_SECTION: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\s*(\w*)\s*(\(\s*\)|:|(:?\s*function\s*\(\s*\)))\s*\{\s*").unwrap()
});

static RE_DATA_PROP: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\s*(\w+):.+").unwrap()
});

static RE_METHOD_PROP: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"\s*(async\s*)?(\w*)\s*(:|\().*"#).unwrap()
});

static RE_DATA_RETURN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\s*return\s*\{\s*").unwrap()
});

/// Definition location result
#[napi(object)]
#[derive(Clone, Debug)]
pub struct DefinitionLocation {
    pub file_path: String,
    pub line: u32,
    pub character: u32,
}

/// Extract file path from import/require statement
pub fn get_definition_position(line_text: &str) -> Option<String> {
    let regexes: [&Lazy<Regex>; 5] = [
        &RE_IMPORT_FROM,
        &RE_IMPORT_DYNAMIC,
        &RE_REQUIRE,
        &RE_IMPORT_BARE,
        &RE_IMPORT_COMMENT,
    ];

    for re in &regexes {
        if let Some(caps) = re.captures(line_text) {
            if let Some(m) = caps.get(1) {
                let path = m.as_str().to_string();
                if !path.is_empty() {
                    return Some(path);
                }
            }
        }
    }
    None
}

/// Resolve a file path, trying various extensions
pub fn resolve_file(
    base_path: &str,
    file_path: &str,
    project_root: &str,
    is_absolute: bool,
) -> Option<String> {
    // Check if file has extension
    let has_ext = RE_HAS_EXT.is_match(file_path);

    if has_ext {
        let temp = if is_absolute {
            Path::new(project_root).join(file_path)
        } else {
            let parent = Path::new(base_path).parent().unwrap_or(Path::new(""));
            parent.join(file_path)
        };
        if temp.exists() {
            return Some(temp.to_string_lossy().to_string());
        }
    } else {
        let extensions = ["vue", "js", "ts", "css", "scss", "less"];
        for ext in &extensions {
            let temp = if is_absolute {
                Path::new(project_root).join(file_path)
            } else {
                let parent = Path::new(base_path).parent().unwrap_or(Path::new(""));
                parent.join(file_path)
            };

            // Try with extension
            let with_ext = temp.with_extension(ext);
            if with_ext.exists() {
                return Some(with_ext.to_string_lossy().to_string());
            }

            // Try as directory with index file
            let index_file = temp.join(format!("index.{}", ext));
            if index_file.exists() {
                return Some(index_file.to_string_lossy().to_string());
            }
        }
    }

    None
}

/// Find definition in file (Vue2 in-file navigation)
pub fn find_in_file(
    lines: &[String],
    select_text: &str,
    start_text: &str,
) -> Option<DefinitionLocation> {
    let is_component = start_text == "<";
    let mut pos = 0usize;
    let mut begin = false;
    let mut brace_left_count: i32 = 0;
    let mut attr = String::new();

    let vue_attrs = [
        "props", "computed", "methods", "watch",
        "beforeCreate", "created", "beforeMount", "mounted",
        "beforeUpdate", "updated", "activated", "deactivated",
        "beforeDestroy", "destroyed", "directives", "filters",
        "components", "data",
    ];

    while pos < lines.len() {
        let line_text = &lines[pos];

        // Look for </script>
        if begin && RE_SCRIPT_END.is_match(line_text) {
            break;
        }

        // Look for <script>
        if !begin {
            if RE_SCRIPT_START.is_match(line_text) {
                begin = true;
            }
            pos += 1;
            continue;
        }

        // Determine which Vue option section we're in
        if let Some(caps) = RE_KEY_SECTION.captures(line_text) {
            if let Some(m) = caps.get(1) {
                let keyword = m.as_str();
                if vue_attrs.contains(&keyword) && brace_left_count == 0 {
                    attr = keyword.to_string();
                    brace_left_count = 0;
                }
            }
        }

        if is_component {
            let tag = select_text.to_lowercase().replace("-", "");
            if !attr.is_empty() {
                // Past import section, not found
                break;
            }
            // Check import lines
            if line_text.to_lowercase().replace("-", "").contains(&tag)
                && (line_text.trim().starts_with("import") || line_text.trim().starts_with("require"))
            {
                // Found the import - extract path
                if let Some(path) = get_definition_position(line_text) {
                    return Some(DefinitionLocation {
                        file_path: path,
                        line: pos as u32,
                        character: 0,
                    });
                }
            }
        } else {
            // Count braces
            let left_count = line_text.matches('{').count() as i32;
            let right_count = line_text.matches('}').count() as i32;

            if attr == "data" && brace_left_count >= 2 {
                // In data's return block
                if let Some(caps) = RE_DATA_PROP.captures(line_text) {
                    if let Some(m) = caps.get(1) {
                        if m.as_str() == select_text && brace_left_count == 2 {
                            return Some(DefinitionLocation {
                                file_path: String::new(), // same file
                                line: pos as u32,
                                character: (line_text.find(select_text).unwrap_or(0) + select_text.len()) as u32,
                            });
                        }
                    }
                }
                brace_left_count += left_count - right_count;
            } else if !attr.is_empty() {
                // In other sections (methods, computed, watch, etc.)
                if let Some(caps) = RE_METHOD_PROP.captures(line_text) {
                    if let Some(m) = caps.get(2) {
                        if m.as_str() == select_text && brace_left_count == 1 {
                            return Some(DefinitionLocation {
                                file_path: String::new(), // same file
                                line: pos as u32,
                                character: (line_text.find(select_text).unwrap_or(0) + select_text.len()) as u32,
                            });
                        }
                    }
                }
                brace_left_count += left_count - right_count;
            }

            // data return detection
            if attr == "data" {
                if RE_DATA_RETURN.is_match(line_text) {
                    brace_left_count = 2;
                }
            }
        }

        pos += 1;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_definition_position() {
        assert_eq!(
            get_definition_position("import Vue from 'vue'"),
            Some("vue".to_string())
        );
        assert_eq!(
            get_definition_position("import { ref } from 'vue'"),
            Some("vue".to_string())
        );
        assert_eq!(
            get_definition_position("const x = require('lodash')"),
            Some("lodash".to_string())
        );
        assert_eq!(
            get_definition_position("const x = 1"),
            None
        );
    }

    #[test]
    fn test_find_in_file() {
        let lines = vec![
            "<template>".to_string(),
            "  <div>{{ message }}</div>".to_string(),
            "</template>".to_string(),
            "<script>".to_string(),
            "export default {".to_string(),
            "  data() {".to_string(),
            "    return {".to_string(),
            "      message: 'hello'".to_string(),
            "    }".to_string(),
            "  }".to_string(),
            "}".to_string(),
            "</script>".to_string(),
        ];
        let result = find_in_file(&lines, "message", "\"");
        assert!(result.is_some());
        let loc = result.unwrap();
        assert_eq!(loc.line, 7);
    }
}
