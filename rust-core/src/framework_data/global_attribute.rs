use super::AttrInfo;
use std::collections::HashMap;

pub fn get_element_ui_global_attributes() -> HashMap<String, AttrInfo> {
    let mut map = HashMap::new();
    map.insert("v-loading".to_string(), AttrInfo {
        attr_type: "attribute".to_string(),
        description: "el-loading".to_string(),
        values: vec!["string".to_string()],
    });
    map
}
