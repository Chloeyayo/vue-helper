use napi::bindgen_prelude::*;
use regex::Regex;
use once_cell::sync::Lazy;

// ---- Lazy-compiled regex statics ----

static RE_HTML_OPEN_TAG: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"<([\w-]+)").unwrap()
});

static RE_DATA_FUNC: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^\s*data\s*\(\s*\)\s*\{\s*$").unwrap()
});

static RE_METHOD_NAME: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^\s*(async\s*)?(\w+)\s*$").unwrap()
});

/// Selection range for block select
#[napi(object)]
#[derive(Clone, Debug)]
pub struct SelectionRange {
    pub start_line: u32,
    pub start_char: u32,
    pub end_line: u32,
    pub end_char: u32,
}

/// Edit operation for smart backspace
#[napi(object)]
#[derive(Clone, Debug)]
pub struct EditOperation {
    pub start_line: u32,
    pub start_char: u32,
    pub end_line: u32,
    pub end_char: u32,
    pub text: String,
}

/// Enhance result from function enhancement
#[napi(object)]
#[derive(Clone, Debug)]
pub struct EnhanceResult {
    pub insert_text: String,
    pub cursor_line: u32,
    pub cursor_char: u32,
    /// "insert" or "snippet"
    pub action_type: String,
}

/// Compute block selection based on cursor position
pub fn compute_block_selection(
    lines: &[String],
    start_line: usize,
    start_char: usize,
) -> Option<SelectionRange> {
    if start_line >= lines.len() {
        return None;
    }

    // Try JS block select (braces)
    if let Some(range) = try_js_block(lines, start_line, start_char, '{', '}') {
        return Some(range);
    }

    // Try array block select (brackets)
    if let Some(range) = try_js_block(lines, start_line, start_char, '[', ']') {
        return Some(range);
    }

    // Try parentheses
    if let Some(range) = try_js_block(lines, start_line, start_char, '(', ')') {
        return Some(range);
    }

    // Try HTML block select
    if let Some(range) = try_html_block(lines, start_line, start_char) {
        return Some(range);
    }

    // Try line block (method chain)
    if let Some(range) = try_line_block(lines, start_line, start_char) {
        return Some(range);
    }

    None
}

fn try_js_block(
    lines: &[String],
    cursor_line: usize,
    cursor_char: usize,
    open: char,
    close: char,
) -> Option<SelectionRange> {
    let current_line = &lines[cursor_line];
    let chars: Vec<char> = current_line.chars().collect();

    // Search backward for open bracket
    let mut depth_back = 0i32;
    let mut found_start: Option<(usize, usize)> = None;

    // Search in current line first
    let mut pos = if cursor_char > 0 { cursor_char - 1 } else { 0 };
    loop {
        if pos < chars.len() {
            if chars[pos] == close {
                depth_back += 1;
            } else if chars[pos] == open {
                if depth_back == 0 {
                    found_start = Some((cursor_line, pos));
                    break;
                }
                depth_back -= 1;
            }
        }
        if pos == 0 { break; }
        pos -= 1;
    }

    // Search in previous lines
    if found_start.is_none() {
        let mut line = cursor_line;
        while line > 0 {
            line -= 1;
            let line_chars: Vec<char> = lines[line].chars().collect();
            let mut p = line_chars.len();
            while p > 0 {
                p -= 1;
                if line_chars[p] == close {
                    depth_back += 1;
                } else if line_chars[p] == open {
                    if depth_back == 0 {
                        found_start = Some((line, p));
                        break;
                    }
                    depth_back -= 1;
                }
            }
            if found_start.is_some() { break; }
        }
    }

    let (start_l, start_c) = found_start?;

    // Search forward for close bracket
    let mut depth_fwd = 0i32;
    let mut found_end: Option<(usize, usize)> = None;

    // Start from the opening bracket position
    let start_search_line = start_l;
    let start_search_char = start_c + 1;

    for line_idx in start_search_line..lines.len() {
        let ln_chars: Vec<char> = lines[line_idx].chars().collect();
        let start = if line_idx == start_search_line { start_search_char } else { 0 };
        for char_idx in start..ln_chars.len() {
            if ln_chars[char_idx] == open {
                depth_fwd += 1;
            } else if ln_chars[char_idx] == close {
                if depth_fwd == 0 {
                    found_end = Some((line_idx, char_idx));
                    break;
                }
                depth_fwd -= 1;
            }
        }
        if found_end.is_some() { break; }
    }

    let (end_l, end_c) = found_end?;

    // Return inner selection (inside the brackets)
    if start_l == end_l {
        Some(SelectionRange {
            start_line: start_l as u32,
            start_char: (start_c + 1) as u32,
            end_line: end_l as u32,
            end_char: end_c as u32,
        })
    } else {
        Some(SelectionRange {
            start_line: (start_l + 1) as u32,
            start_char: 0,
            end_line: (end_l) as u32,
            end_char: 0,
        })
    }
}

fn try_html_block(
    lines: &[String],
    cursor_line: usize,
    _cursor_char: usize,
) -> Option<SelectionRange> {
    let current = &lines[cursor_line];

    let cap = RE_HTML_OPEN_TAG.captures(current)?;
    let tag_name = cap.get(1)?.as_str();

    let mut depth = 0i32;
    let mut start_line = cursor_line;
    let mut end_line = cursor_line;
    // Dynamic regexes using regex::escape -- cannot be cached statically
    let open_tag_re = Regex::new(&format!(r"<{}\b", regex::escape(tag_name))).ok()?;
    let close_tag_re = Regex::new(&format!(r"</{}\s*>", regex::escape(tag_name))).ok()?;

    // Search backward to find the opening tag
    let mut found_start = false;
    let mut line = cursor_line;
    loop {
        let lt = &lines[line];
        let opens = open_tag_re.find_iter(lt).count() as i32;
        let closes = close_tag_re.find_iter(lt).count() as i32;
        depth += closes - opens;
        if depth <= 0 && opens > 0 {
            start_line = line;
            found_start = true;
            break;
        }
        if line == 0 { break; }
        line -= 1;
    }

    if !found_start { return None; }

    // Search forward for closing tag
    depth = 0;
    for line in start_line..lines.len() {
        let lt = &lines[line];
        let opens = open_tag_re.find_iter(lt).count() as i32;
        let closes = close_tag_re.find_iter(lt).count() as i32;
        depth += opens - closes;
        if depth <= 0 && closes > 0 {
            end_line = line;
            break;
        }
    }

    if start_line < end_line {
        Some(SelectionRange {
            start_line: (start_line + 1) as u32,
            start_char: 0,
            end_line: end_line as u32,
            end_char: 0,
        })
    } else {
        None
    }
}

fn try_line_block(
    lines: &[String],
    cursor_line: usize,
    _cursor_char: usize,
) -> Option<SelectionRange> {
    // Select chain of lines (e.g., method chains that start with .)
    let current = &lines[cursor_line];
    let trimmed = current.trim();
    if trimmed.starts_with('.') || trimmed.starts_with('|') || trimmed.starts_with('+') || trimmed.starts_with(',') {
        let mut start = cursor_line;
        while start > 0 {
            let prev = lines[start - 1].trim();
            if prev.starts_with('.') || prev.starts_with('|') || prev.starts_with('+') || prev.starts_with(',') {
                start -= 1;
            } else if !prev.is_empty() {
                start -= 1;
                break;
            } else {
                break;
            }
        }
        let mut end = cursor_line;
        while end + 1 < lines.len() {
            let next = lines[end + 1].trim();
            if next.starts_with('.') || next.starts_with('|') || next.starts_with('+') || next.starts_with(',') {
                end += 1;
            } else {
                break;
            }
        }
        if start < end {
            return Some(SelectionRange {
                start_line: start as u32,
                start_char: 0,
                end_line: (end + 1) as u32,
                end_char: 0,
            });
        }
    }
    None
}

/// Compute smart backspace edit
pub fn compute_backspace_edit(
    lines: &[String],
    cursor_line: usize,
    cursor_char: usize,
) -> Option<EditOperation> {
    if cursor_line >= lines.len() || cursor_char == 0 {
        return None;
    }

    let line = &lines[cursor_line];
    if cursor_char > line.len() {
        return None;
    }

    let chars: Vec<char> = line.chars().collect();

    // Paired brackets/quotes: if prev char opens and next char closes, delete both
    let pairs = [
        ('{', '}'),
        ('[', ']'),
        ('(', ')'),
        ('\'', '\''),
        ('"', '"'),
        ('`', '`'),
    ];

    if cursor_char > 0 && cursor_char < chars.len() {
        let prev = chars[cursor_char - 1];
        let next = chars[cursor_char];
        for (open, close) in &pairs {
            if prev == *open && next == *close {
                return Some(EditOperation {
                    start_line: cursor_line as u32,
                    start_char: (cursor_char - 1) as u32,
                    end_line: cursor_line as u32,
                    end_char: (cursor_char + 1) as u32,
                    text: String::new(),
                });
            }
        }
    }

    // Empty line: delete the line
    if line.trim().is_empty() && cursor_line > 0 {
        return Some(EditOperation {
            start_line: (cursor_line - 1) as u32,
            start_char: lines[cursor_line - 1].len() as u32,
            end_line: cursor_line as u32,
            end_char: line.len() as u32,
            text: String::new(),
        });
    }

    // Delete spaces to previous tab stop (2-space unindent)
    let leading_spaces = line.len() - line.trim_start().len();
    if cursor_char <= leading_spaces && cursor_char > 0 {
        let spaces_to_delete = if cursor_char % 2 == 0 { 2.min(cursor_char) } else { 1 };
        return Some(EditOperation {
            start_line: cursor_line as u32,
            start_char: (cursor_char - spaces_to_delete) as u32,
            end_line: cursor_line as u32,
            end_char: cursor_char as u32,
            text: String::new(),
        });
    }

    None
}

/// Compute function enhancement text
pub fn compute_func_enhance_text(
    lines: &[String],
    cursor_line: usize,
    _cursor_char: usize,
    tab_size: &str,
) -> Option<EnhanceResult> {
    if cursor_line >= lines.len() {
        return None;
    }

    let line = &lines[cursor_line];
    let trimmed = line.trim();

    // Calculate indentation
    let indent = &line[..line.len() - line.trim_start().len()];

    // Check for function-like patterns
    if trimmed.ends_with('{') || trimmed.ends_with("=> {") || trimmed.ends_with("({") {
        let insert = format!("\n{}{}", indent, tab_size);
        return Some(EnhanceResult {
            insert_text: insert,
            cursor_line: (cursor_line + 1) as u32,
            cursor_char: (indent.len() + tab_size.len()) as u32,
            action_type: "insert".to_string(),
        });
    }

    // Check for various patterns:
    // data() {  -> generate return {}
    if RE_DATA_FUNC.is_match(line) {
        let inner = format!("{}{}",indent, tab_size);
        let insert = format!("\n{}return {{\n{}{}\n{}}}", inner, inner, tab_size, inner);
        return Some(EnhanceResult {
            insert_text: insert,
            cursor_line: (cursor_line + 2) as u32,
            cursor_char: (indent.len() + tab_size.len() * 2) as u32,
            action_type: "insert".to_string(),
        });
    }

    // method name -> generate method template
    if let Some(caps) = RE_METHOD_NAME.captures(trimmed) {
        let is_async = caps.get(1).is_some();
        let name = caps.get(2)?.as_str();
        let async_prefix = if is_async { "async " } else { "" };

        let insert = format!("{}{}() {{\n{}{}\n{}}},", async_prefix, name, indent, tab_size, indent);
        return Some(EnhanceResult {
            insert_text: insert,
            cursor_line: (cursor_line + 1) as u32,
            cursor_char: (indent.len() + tab_size.len()) as u32,
            action_type: "snippet".to_string(),
        });
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_select_js() {
        let lines = vec![
            "function test() {".to_string(),
            "  const x = 1".to_string(),
            "  const y = 2".to_string(),
            "}".to_string(),
        ];
        let result = compute_block_selection(&lines, 1, 5);
        assert!(result.is_some());
        let range = result.unwrap();
        assert_eq!(range.start_line, 1);
        assert_eq!(range.end_line, 3);
    }

    #[test]
    fn test_backspace_pair() {
        let lines = vec![
            "const x = {}".to_string(),
        ];
        let result = compute_backspace_edit(&lines, 0, 11);
        assert!(result.is_some());
        let op = result.unwrap();
        assert_eq!(op.start_char, 10);
        assert_eq!(op.end_char, 12);
    }

    #[test]
    fn test_backspace_empty_line() {
        let lines = vec![
            "line1".to_string(),
            "".to_string(),
        ];
        let result = compute_backspace_edit(&lines, 1, 0);
        // cursor_char is 0, should return None (beginning of line)
        assert!(result.is_none());
    }
}
