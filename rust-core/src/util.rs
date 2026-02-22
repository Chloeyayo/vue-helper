use std::path::Path;

/// Result of get_word function
#[napi(object)]
#[derive(Clone, Debug)]
pub struct WordResult {
    pub select_text: String,
    pub start_text: String,
}

/// Handle Windows root path (remove leading slash on Windows)
pub fn win_root_path_handle(page_path: &str) -> String {
    if page_path.is_empty() {
        return String::new();
    }
    if cfg!(windows) && (page_path.starts_with('\\') || page_path.starts_with('/')) {
        page_path[1..].to_string()
    } else {
        page_path.to_string()
    }
}

/// Get relative path between two paths
pub fn get_relative_path(src: &str, dist: &str) -> String {
    let src_handled = win_root_path_handle(src);
    let src_path = Path::new(&src_handled);
    let dist_path = Path::new(dist);

    if let Some(rel) = pathdiff_relative(dist_path, src_path) {
        let mut result = rel.replace('\\', "/");
        if result.starts_with("../") {
            result = result[1..].to_string();
        }
        result
    } else {
        dist.replace('\\', "/")
    }
}

fn pathdiff_relative(to: &Path, from: &Path) -> Option<String> {
    // Simple relative path computation
    let to_str = to.to_string_lossy().replace('\\', "/");
    let from_str = from.to_string_lossy().replace('\\', "/");

    let to_parts: Vec<&str> = to_str.split('/').filter(|s| !s.is_empty()).collect();
    let from_parts: Vec<&str> = from_str.split('/').filter(|s| !s.is_empty()).collect();

    let mut common = 0;
    for i in 0..std::cmp::min(to_parts.len(), from_parts.len()) {
        if to_parts[i].eq_ignore_ascii_case(from_parts[i]) {
            common = i + 1;
        } else {
            break;
        }
    }

    let mut result = String::new();
    for _ in common..from_parts.len() {
        result.push_str("../");
    }
    for i in common..to_parts.len() {
        if i > common {
            result.push('/');
        }
        result.push_str(to_parts[i]);
    }

    if result.is_empty() {
        result = ".".to_string();
    }

    Some(result)
}

/// Get current word at cursor position
pub fn get_current_word(text: &str, character: usize) -> String {
    let chars: Vec<char> = text.chars().collect();
    let stop_chars = " \t\n\r\x0B\":{[,";
    let mut i = if character > 0 { character - 1 } else { 0 };

    while i < chars.len() {
        if stop_chars.contains(chars[i]) {
            break;
        }
        if i == 0 {
            break;
        }
        i -= 1;
    }

    if i < chars.len() && stop_chars.contains(chars[i]) {
        i += 1;
    }

    chars[i..character.min(chars.len())]
        .iter()
        .collect()
}

/// Get word at position with custom delimiters
pub fn get_word(text: &str, character: usize, delimiters: &[String]) -> WordResult {
    let chars: Vec<char> = text.chars().collect();
    let delim_chars: Vec<char> = delimiters.iter().filter_map(|s| s.chars().next()).collect();

    let mut select_text = String::new();
    let mut start_text = String::new();

    // Forward: get characters after cursor
    let mut pos = character;
    while pos < chars.len() {
        let ch = chars[pos];
        if delim_chars.contains(&ch) {
            break;
        }
        select_text.push(ch);
        pos += 1;
    }

    // Backward: get characters before cursor
    if character > 0 {
        pos = character - 1;
        loop {
            let ch = chars[pos];
            if delim_chars.contains(&ch) {
                start_text = ch.to_string();
                break;
            }
            select_text.insert(0, ch);
            if pos == 0 {
                break;
            }
            pos -= 1;
        }
    }

    WordResult {
        select_text,
        start_text,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_current_word() {
        assert_eq!(get_current_word("el-button type", 14), "type");
        assert_eq!(get_current_word("import Vue", 10), "Vue");
        assert_eq!(get_current_word("  hello", 7), "hello");
    }

    #[test]
    fn test_win_root_path_handle() {
        if cfg!(windows) {
            assert_eq!(win_root_path_handle("/c:/test"), "c:/test");
            assert_eq!(win_root_path_handle("c:/test"), "c:/test");
        }
        assert_eq!(win_root_path_handle(""), "");
    }

    #[test]
    fn test_get_word() {
        let delimiters: Vec<String> = vec![" ", "<", ">", "\""].iter().map(|s| s.to_string()).collect();
        let result = get_word("  <el-button type=\"primary\">", 5, &delimiters);
        assert_eq!(result.select_text, "el-button");
        assert_eq!(result.start_text, "<");
    }
}
