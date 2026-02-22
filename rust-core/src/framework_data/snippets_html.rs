use std::collections::HashMap;

pub fn get_snippets(ts: &str) -> HashMap<String, String> {
    let mut m = HashMap::new();
    m.insert("vcomponent".into(), "<component :is=\"${1:componentId}\"></component>$0".into());
    m.insert("vka".into(), format!("<keep-alive$1>\n{ts}$2\n</keep-alive>$0"));
    m.insert("vtransition".into(), format!("<transition$1>\n{ts}$2\n</transition>$0"));
    m.insert("vtg".into(), format!("<transition-group$1>\n{ts}$2\n</transition-group>"));
    m.insert("vrl".into(), "<router-link $1>$2</router-link>$0".into());
    m.insert("vrlt".into(), "<router-link to=\"$1\">$2</router-link>$0".into());
    m.insert("vrv".into(), "<router-view>$1</router-view>$0".into());
    m
}
