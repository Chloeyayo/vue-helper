use napi::bindgen_prelude::*;
use std::fs;
use std::path::Path;

/// Represents a found Vue file
#[napi(object)]
#[derive(Clone, Debug)]
pub struct VueFile {
    pub name: String,
    pub path: String,
}

/// Search for files in the project directory
pub fn search(
    root_path: &str,
    poster: &str,
    search_name: &str,
    use_prefix: bool,
    prefix_alias: &str,
    prefix_path: &str,
) -> Vec<VueFile> {
    let mut files = Vec::new();

    if root_path.is_empty() {
        return files;
    }

    let ignore = ["node_modules", "dist", "build"];

    let root = Path::new(root_path);
    if !root.exists() || !root.is_dir() {
        return files;
    }

    let entries = match fs::read_dir(root) {
        Ok(e) => e,
        Err(_) => return files,
    };

    let (alias, path_prefix) = if use_prefix {
        (prefix_alias, prefix_path)
    } else {
        ("", "")
    };

    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with('.') || ignore.contains(&name.as_str()) {
            continue;
        }

        if let Ok(ft) = entry.file_type() {
            if ft.is_dir() {
                traverse_handle(
                    root_path,
                    &name,
                    &mut files,
                    alias,
                    path_prefix,
                    poster,
                    search_name,
                );
            } else {
                traverse_add(
                    &name,
                    &name,
                    &mut files,
                    alias,
                    path_prefix,
                    poster,
                    search_name,
                );
            }
        }
    }

    files
}

fn traverse_add(
    root_path: &str,
    dir: &str,
    files: &mut Vec<VueFile>,
    prefix_alias: &str,
    prefix_path: &str,
    poster: &str,
    search: &str,
) {
    if !poster.is_empty() && !root_path.ends_with(poster) {
        return;
    }

    let poster_pattern = if poster.is_empty() {
        r"-?(.*)\.\w*$"
    } else {
        "" // handled below
    };

    let name = if !poster.is_empty() {
        let suffix = poster;
        if let Some(idx) = root_path.rfind(suffix) {
            let base = &root_path[..idx];
            // Remove leading dash if present
            if base.starts_with('-') {
                base[1..].to_string()
            } else {
                base.to_string()
            }
        } else {
            root_path.to_string()
        }
    } else {
        // Remove extension
        if let Some(dot_idx) = root_path.rfind('.') {
            let base = &root_path[..dot_idx];
            if base.starts_with('-') {
                base[1..].to_string()
            } else {
                base.to_string()
            }
        } else {
            root_path.to_string()
        }
    };

    if search.is_empty() || dir.contains(search) {
        let mut file_path = dir.replace('\\', "/");
        if !prefix_alias.is_empty() && !prefix_path.is_empty() {
            if let Some(stripped) = file_path.strip_prefix(prefix_path) {
                file_path = format!("{}{}", prefix_alias, stripped);
            }
        }

        files.push(VueFile {
            name: name.clone(),
            path: file_path.clone(),
        });

        // Also add index-based name
        if name == "index" {
            let normalized = dir.replace('\\', "/");
            // Extract parent directory name
            let parts: Vec<&str> = normalized.split('/').collect();
            if parts.len() >= 2 {
                let parent_name = parts[parts.len() - 2].to_string();
                files.push(VueFile {
                    name: parent_name,
                    path: file_path,
                });
            }
        }
    }
}

fn traverse_handle(
    project_root: &str,
    post_path: &str,
    files: &mut Vec<VueFile>,
    prefix_alias: &str,
    prefix_path: &str,
    poster: &str,
    search: &str,
) {
    let full_path = Path::new(project_root).join(post_path);
    let entries = match fs::read_dir(&full_path) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with('.') {
            continue;
        }

        let dir = format!("{}/{}", post_path.replace('\\', "/"), name);

        if let Ok(ft) = entry.file_type() {
            if ft.is_dir() {
                traverse_handle(
                    project_root,
                    &dir,
                    files,
                    prefix_alias,
                    prefix_path,
                    poster,
                    search,
                );
            } else {
                traverse_add(
                    &name,
                    &dir,
                    files,
                    prefix_alias,
                    prefix_path,
                    poster,
                    search,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traverse_add_vue() {
        let mut files = Vec::new();
        traverse_add("MyComponent.vue", "src/components/MyComponent.vue", &mut files, "", "", ".vue", "");
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].name, "MyComponent");
        assert_eq!(files[0].path, "src/components/MyComponent.vue");
    }

    #[test]
    fn test_traverse_add_index() {
        let mut files = Vec::new();
        traverse_add("index.vue", "src/components/Header/index.vue", &mut files, "", "", ".vue", "");
        assert_eq!(files.len(), 2);
        assert_eq!(files[0].name, "index");
        assert_eq!(files[1].name, "Header");
    }

    #[test]
    fn test_traverse_add_with_search() {
        let mut files = Vec::new();
        traverse_add("MyComponent.vue", "src/components/MyComponent.vue", &mut files, "", "", ".vue", "Button");
        assert_eq!(files.len(), 0); // "Button" not in path

        traverse_add("MyComponent.vue", "src/components/MyComponent.vue", &mut files, "", "", ".vue", "MyComponent");
        assert_eq!(files.len(), 1);
    }
}
