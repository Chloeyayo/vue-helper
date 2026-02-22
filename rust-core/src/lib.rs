#[macro_use]
extern crate napi_derive;

pub mod framework_data;
pub mod traverse;
pub mod util;
pub mod completion;
pub mod hover;
pub mod definition;
pub mod assist;

use napi::bindgen_prelude::*;
use std::collections::HashMap;

/// Initialize the framework provider with detected frameworks
#[napi]
pub fn init_frameworks(package_json_content: String) -> Vec<String> {
    let mut frameworks = Vec::new();
    if package_json_content.contains("element-plus") {
        frameworks.push("element-plus".to_string());
    }
    if package_json_content.contains("element-ui") {
        frameworks.push("element-ui".to_string());
    }
    if package_json_content.contains("ant-design-vue") {
        frameworks.push("ant-design-vue".to_string());
    }
    frameworks
}

/// Search for vue files in the project
#[napi]
pub fn search_files(
    root_path: String,
    poster: String,
    search_name: String,
    use_prefix: bool,
    prefix_alias: String,
    prefix_path: String,
) -> Vec<traverse::VueFile> {
    traverse::search(
        &root_path,
        &poster,
        &search_name,
        use_prefix,
        &prefix_alias,
        &prefix_path,
    )
}

/// Get completion suggestions for tags
#[napi]
pub fn get_tag_completions(
    frameworks: Vec<String>,
    tab_size: String,
    use_vue_snippets: bool,
) -> Vec<completion::CompletionSuggestion> {
    completion::get_tag_suggestions(&frameworks, &tab_size, use_vue_snippets)
}

/// Get completion suggestions for JS tags
#[napi]
pub fn get_js_tag_completions(
    frameworks: Vec<String>,
    tab_size: String,
    use_vue_snippets: bool,
) -> Vec<completion::CompletionSuggestion> {
    completion::get_js_tag_suggestions(&frameworks, &tab_size, use_vue_snippets)
}

/// Get attribute completions for a given tag
#[napi]
pub fn get_attr_completions(
    tag: String,
    frameworks: Vec<String>,
    tab_size: String,
    attr_type: String,
) -> Vec<completion::CompletionSuggestion> {
    completion::get_attr_suggestions(&tag, &frameworks, &tab_size, &attr_type)
}

/// Get attribute value completions
#[napi]
pub fn get_attr_value_completions(
    tag: String,
    attr: String,
    frameworks: Vec<String>,
    tab_size: String,
) -> Vec<completion::CompletionSuggestion> {
    completion::get_attr_value_suggestions(&tag, &attr, &frameworks, &tab_size)
}

/// Get element tag label suggestions (tag names from attribute data)
#[napi]
pub fn get_element_tag_labels(
    frameworks: Vec<String>,
    tab_size: String,
    extension_name: String,
) -> Vec<completion::CompletionSuggestion> {
    completion::get_element_tag_label_suggestions(&frameworks, &tab_size, &extension_name)
}

/// Provide hover information for a word
#[napi]
pub fn provide_hover(word: String, frameworks: Vec<String>, tab_size: String) -> Option<String> {
    hover::provide_hover_info(&word, &frameworks, &tab_size)
}

/// Check if a line is a close tag
#[napi]
pub fn is_close_tag(text_before_cursor: String) -> bool {
    completion::is_close_tag_check(&text_before_cursor)
}

/// Get the close tag name
#[napi]
pub fn get_close_tag_name(line_text: String) -> String {
    completion::get_close_tag(&line_text)
}

/// Match a tag from the text before position
#[napi]
pub fn match_pre_tag(text: String) -> Option<completion::TagMatch> {
    completion::match_pre_tag_from_text(&text)
}

/// Match the attribute before cursor
#[napi]
pub fn match_pre_attr(text: String) -> Option<String> {
    completion::match_pre_attr_from_text(&text)
}

/// Get the current word at position
#[napi]
pub fn get_current_word_at(text: String, character: u32) -> String {
    util::get_current_word(&text, character as usize)
}

/// Check if text is an import statement
#[napi]
pub fn is_import_line(text: String) -> bool {
    completion::is_import_check(&text)
}

/// Check if the position is not in template (i.e., in script section)
#[napi]
pub fn check_not_in_template(lines: Vec<String>, current_line: u32) -> bool {
    completion::not_in_template(&lines, current_line as usize)
}

/// Get definition position from a line (file path extraction from import/require)
#[napi]
pub fn get_definition_path(line_text: String) -> Option<String> {
    definition::get_definition_position(&line_text)
}

/// Resolve file path with extensions
#[napi]
pub fn resolve_file_path(
    base_path: String,
    file_path: String,
    project_root: String,
    is_absolute: bool,
) -> Option<String> {
    definition::resolve_file(&base_path, &file_path, &project_root, is_absolute)
}

/// Compute block selection range
#[napi]
pub fn compute_block_select(
    lines: Vec<String>,
    start_line: u32,
    start_char: u32,
) -> Option<assist::SelectionRange> {
    assist::compute_block_selection(&lines, start_line as usize, start_char as usize)
}

/// Compute smart backspace edit
#[napi]
pub fn compute_backspace(
    lines: Vec<String>,
    cursor_line: u32,
    cursor_char: u32,
) -> Option<assist::EditOperation> {
    assist::compute_backspace_edit(&lines, cursor_line as usize, cursor_char as usize)
}

/// Compute function enhance text
#[napi]
pub fn compute_func_enhance(
    lines: Vec<String>,
    cursor_line: u32,
    cursor_char: u32,
    tab_size: String,
) -> Option<assist::EnhanceResult> {
    assist::compute_func_enhance_text(&lines, cursor_line as usize, cursor_char as usize, &tab_size)
}

/// Get props from a vue file content
#[napi]
pub fn extract_vue_props(file_content: String) -> Vec<completion::CompletionSuggestion> {
    completion::extract_props_from_vue(&file_content)
}

/// Get import suggestions from vue files
#[napi]
pub fn get_import_suggestions(
    search_text: String,
    vue_files: Vec<traverse::VueFile>,
    document_path: String,
    project_root: String,
) -> Vec<completion::CompletionSuggestion> {
    completion::get_import_suggestion_items(&search_text, &vue_files, &document_path, &project_root)
}

/// Search for definition in file (Vue2 in-file jump)
#[napi]
pub fn find_definition_in_file(
    lines: Vec<String>,
    select_text: String,
    start_text: String,
) -> Option<definition::DefinitionLocation> {
    definition::find_in_file(&lines, &select_text, &start_text)
}

/// Get word at position with custom delimiters
#[napi]
pub fn get_word_at_position(
    line_text: String,
    character: u32,
    delimiters: Vec<String>,
) -> util::WordResult {
    util::get_word(&line_text, character as usize, &delimiters)
}

/// Invalidate the framework data cache (call on workspace switch)
#[napi]
pub fn invalidate_framework_cache() {
    framework_data::invalidate_cache();
}
