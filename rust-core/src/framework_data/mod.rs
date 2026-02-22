pub mod attribute;
pub mod tag;
pub mod js_tag;
pub mod document;
pub mod global_attribute;
pub mod snippets_html;
pub mod snippets_js;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;

/// Attribute info for a component attribute
#[derive(Clone, Debug)]
pub struct AttrInfo {
    pub attr_type: String, // "attribute" or "method"
    pub description: String,
    pub values: Vec<String>,
}

/// Cached framework data -- all 7 HashMaps in one struct
pub struct FrameworkCache {
    pub key: String,
    pub attributes: HashMap<String, HashMap<String, AttrInfo>>,
    pub tags: HashMap<String, String>,
    pub js_tags: HashMap<String, String>,
    pub documents: HashMap<String, String>,
    pub global_attributes: HashMap<String, AttrInfo>,
    pub snippets_html: HashMap<String, String>,
    pub snippets_js: HashMap<String, String>,
}

static CACHE: Lazy<Mutex<Option<Arc<FrameworkCache>>>> = Lazy::new(|| Mutex::new(None));

/// Build cache key from frameworks + tab_size
fn make_cache_key(frameworks: &[String], tab_size: &str) -> String {
    let mut sorted = frameworks.to_vec();
    sorted.sort();
    format!("{}|{}", sorted.join(","), tab_size)
}

/// Get or build the cached framework data. Returns Arc for zero-copy sharing.
pub fn get_cached_data(frameworks: &[String], tab_size: &str) -> Arc<FrameworkCache> {
    let key = make_cache_key(frameworks, tab_size);
    let mut guard = CACHE.lock().unwrap();
    if let Some(ref cached) = *guard {
        if cached.key == key {
            return Arc::clone(cached);
        }
    }

    // Cache miss -- rebuild all data
    let cache = Arc::new(FrameworkCache {
        key: key.clone(),
        attributes: build_attributes(frameworks, tab_size),
        tags: build_tags(frameworks, tab_size),
        js_tags: build_js_tags(frameworks, tab_size),
        documents: build_documents(frameworks, tab_size),
        global_attributes: build_global_attributes(frameworks, tab_size),
        snippets_html: snippets_html::get_snippets(tab_size),
        snippets_js: snippets_js::get_snippets(tab_size),
    });
    *guard = Some(Arc::clone(&cache));
    cache
}

/// Invalidate the framework cache (call on workspace switch)
pub fn invalidate_cache() {
    let mut guard = CACHE.lock().unwrap();
    *guard = None;
}

// ---- Internal builders (used only on cache miss) ----

fn build_attributes(frameworks: &[String], _tab_size: &str) -> HashMap<String, HashMap<String, AttrInfo>> {
    let mut result = HashMap::new();
    for framework in frameworks {
        if framework == "element-ui" || framework == "element-plus" {
            let attrs = attribute::get_element_ui_attributes();
            for (tag, tag_attrs) in attrs {
                result.entry(tag).or_insert_with(HashMap::new).extend(tag_attrs);
            }
        }
    }
    result
}

fn build_tags(frameworks: &[String], tab_size: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();
    for framework in frameworks {
        if framework == "element-ui" || framework == "element-plus" {
            result.extend(tag::get_element_ui_tags(tab_size));
        }
    }
    result
}

fn build_js_tags(frameworks: &[String], tab_size: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();
    for framework in frameworks {
        if framework == "element-ui" || framework == "element-plus" {
            result.extend(js_tag::get_element_ui_js_tags(tab_size));
        }
    }
    result
}

fn build_documents(frameworks: &[String], _tab_size: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();
    for framework in frameworks {
        if framework == "element-ui" || framework == "element-plus" {
            result.extend(document::get_element_ui_documents());
        }
    }
    result
}

fn build_global_attributes(frameworks: &[String], _tab_size: &str) -> HashMap<String, AttrInfo> {
    let mut result = HashMap::new();
    for framework in frameworks {
        if framework == "element-ui" || framework == "element-plus" {
            result.extend(global_attribute::get_element_ui_global_attributes());
        }
    }
    result
}

// ---- Public API (kept for backward compatibility, now delegates to cache) ----

/// Get merged attributes for given frameworks
pub fn get_attributes(frameworks: &[String], tab_size: &str) -> HashMap<String, HashMap<String, AttrInfo>> {
    get_cached_data(frameworks, tab_size).attributes.clone()
}

/// Get merged tags for given frameworks
pub fn get_tags(frameworks: &[String], tab_size: &str) -> HashMap<String, String> {
    get_cached_data(frameworks, tab_size).tags.clone()
}

/// Get merged JS tags for given frameworks
pub fn get_js_tags(frameworks: &[String], tab_size: &str) -> HashMap<String, String> {
    get_cached_data(frameworks, tab_size).js_tags.clone()
}

/// Get merged documents for given frameworks
pub fn get_documents(frameworks: &[String], tab_size: &str) -> HashMap<String, String> {
    get_cached_data(frameworks, tab_size).documents.clone()
}

/// Get merged global attributes for given frameworks
pub fn get_global_attributes(frameworks: &[String], tab_size: &str) -> HashMap<String, AttrInfo> {
    get_cached_data(frameworks, tab_size).global_attributes.clone()
}

/// Get vue HTML snippets
pub fn get_vue_snippets_html(tab_size: &str) -> HashMap<String, String> {
    // Snippets don't depend on frameworks, but we still cache them via the framework cache
    // For standalone calls, just build directly
    snippets_html::get_snippets(tab_size)
}

/// Get vue JS snippets
pub fn get_vue_snippets_js(tab_size: &str) -> HashMap<String, String> {
    snippets_js::get_snippets(tab_size)
}

#[cfg(test)]
mod tests {
    use super::*;

    // All cache tests must run sequentially since they share global state.
    // Combining into one test avoids parallel interference.
    #[test]
    fn test_framework_cache() {
        // --- Cache hit ---
        invalidate_cache(); // start clean
        let frameworks = vec!["element-ui".to_string()];
        let data1 = get_cached_data(&frameworks, "  ");
        let data2 = get_cached_data(&frameworks, "  ");
        assert!(Arc::ptr_eq(&data1, &data2), "second call should return same Arc (cache hit)");

        // --- Cache invalidation ---
        invalidate_cache();
        let data3 = get_cached_data(&frameworks, "  ");
        assert!(!Arc::ptr_eq(&data1, &data3), "after invalidation, should be a new Arc");
        assert_eq!(data1.key, data3.key, "keys should still match");

        // --- Cache key change (tab_size) ---
        let data4 = get_cached_data(&frameworks, "    ");
        assert!(!Arc::ptr_eq(&data3, &data4), "different tab_size => cache miss");
        assert_ne!(data3.key, data4.key);

        // --- Cache key change (frameworks) ---
        let frameworks2 = vec!["element-plus".to_string()];
        let data5 = get_cached_data(&frameworks2, "  ");
        assert_ne!(data4.key, data5.key, "different frameworks => different key");
    }
}
