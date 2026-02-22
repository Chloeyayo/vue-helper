use crate::framework_data;

/// Provide hover documentation for a word
pub fn provide_hover_info(word: &str, frameworks: &[String], tab_size: &str) -> Option<String> {
    let documents = framework_data::get_documents(frameworks, tab_size);
    documents.get(word).cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hover_existing_tag() {
        let frameworks = vec!["element-ui".to_string()];
        let result = provide_hover_info("el-button", &frameworks, "  ");
        assert!(result.is_some());
        assert!(result.unwrap().contains("element"));
    }

    #[test]
    fn test_hover_unknown_tag() {
        let frameworks = vec!["element-ui".to_string()];
        let result = provide_hover_info("unknown-tag", &frameworks, "  ");
        assert!(result.is_none());
    }
}
