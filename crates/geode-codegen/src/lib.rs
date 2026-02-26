pub mod android_symbol;
pub mod class;
pub mod function;
pub mod member;
pub mod platform;
pub mod types;

use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::Result;
use broma_rs::Root;

#[derive(Debug, Clone)]
pub struct Config {
    pub broma_paths: Vec<PathBuf>,
    pub output_dir: PathBuf,
    pub platform: Option<platform::Platform>,
    pub generate_docs: bool,
    pub separate_files: bool,
    pub use_cocos_bindgen: bool,
}

pub fn generate(config: Config) -> Result<()> {
    let mut roots = Vec::new();
    for path in &config.broma_paths {
        let root = broma_rs::parse_file(path)?;
        roots.push(root);
    }

    let merged = merge_roots(&roots);

    let class_names: Vec<String> = merged
        .classes
        .iter()
        .map(|c| extract_class_name(&c.name).to_string())
        .collect();
    types::register_classes(&class_names);

    std::fs::create_dir_all(&config.output_dir)?;

    let platform = config.platform.unwrap_or_else(detect_platform);

    let classes_dir = config.output_dir.join("classes");
    let functions_dir = config.output_dir.join("functions");
    std::fs::create_dir_all(&classes_dir)?;
    std::fs::create_dir_all(&functions_dir)?;

    if config.separate_files {
        let mut module_counts: HashMap<String, usize> = HashMap::new();
        let mut class_modules: Vec<(String, String)> = Vec::new();

        for class in &merged.classes {
            let base_module_name = to_snake_case(&class.name);
            let count = module_counts.entry(base_module_name.clone()).or_insert(0);
            *count += 1;

            let module_name = if *count > 1 {
                format!("{}_{}", base_module_name, count)
            } else {
                base_module_name.clone()
            };

            let output =
                class::generate_class(class, &merged, platform, config.generate_docs, true, true);
            let file_path = classes_dir.join(format!("{module_name}.rs"));
            std::fs::write(&file_path, output)?;
            class_modules.push((
                module_name,
                class::serialize_name(&class.name).to_string(),
            ));
        }

        let classes_mod = generate_classes_mod(&class_modules);
        std::fs::write(classes_dir.join("mod.rs"), classes_mod)?;
    } else {
        let mut classes_output = String::new();
        let mut first = true;

        for class in &merged.classes {
            let output =
                class::generate_class(class, &merged, platform, config.generate_docs, first, false);
            classes_output.push_str(&output);
            classes_output.push('\n');
            first = false;
        }

        std::fs::write(classes_dir.join("mod.rs"), classes_output)?;
    }

    let functions_output =
        function::generate_free_functions(&merged.functions, platform, config.generate_docs);
    std::fs::write(functions_dir.join("global.rs"), functions_output)?;
    std::fs::write(
        functions_dir.join("mod.rs"),
        "#![allow(unused_imports)]\nmod global;\npub use global::*;\n",
    )?;

    let types_output = types::generate_types_mod(config.use_cocos_bindgen);
    std::fs::write(config.output_dir.join("types.rs"), types_output)?;

    let mod_output = generate_root_mod(config.use_cocos_bindgen);
    std::fs::write(config.output_dir.join("mod.rs"), mod_output)?;

    Ok(())
}

fn extract_class_name(full_name: &str) -> &str {
    if let Some(pos) = full_name.rfind("::") {
        &full_name[pos + 2..]
    } else {
        full_name
    }
}

fn merge_roots(roots: &[Root]) -> Root {
    let mut merged = Root::default();
    for root in roots {
        merged.headers.extend(root.headers.iter().cloned());
        merged.classes.extend(root.classes.iter().cloned());
        merged.functions.extend(root.functions.iter().cloned());
    }
    merged
}

fn detect_platform() -> platform::Platform {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "windows")] {
            platform::Platform::Windows
        } else if #[cfg(all(target_os = "macos", target_arch = "x86_64"))] {
            platform::Platform::MacIntel
        } else if #[cfg(all(target_os = "macos", target_arch = "aarch64"))] {
            platform::Platform::MacArm
        } else if #[cfg(target_os = "ios")] {
            platform::Platform::IOS
        } else if #[cfg(all(target_os = "android", target_arch = "arm"))] {
            platform::Platform::Android32
        } else if #[cfg(all(target_os = "android", target_arch = "aarch64"))] {
            platform::Platform::Android64
        } else {
            platform::Platform::Windows
        }
    }
}

pub fn to_snake_case(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    let mut prev_is_lower_or_digit = false;
    while let Some(c) = chars.next() {
        if c == ':' && chars.peek() == Some(&':') {
            chars.next();
            if !result.is_empty() && !result.ends_with('_') {
                result.push('_');
            }
            prev_is_lower_or_digit = false;
            continue;
        }
        if c.is_uppercase() {
            let next_is_lower = chars.peek().is_some_and(|n| n.is_lowercase());
            if !result.is_empty()
                && (prev_is_lower_or_digit || next_is_lower)
                && !result.ends_with('_')
            {
                result.push('_');
            }
            result.extend(c.to_lowercase());
            prev_is_lower_or_digit = false;
        } else {
            result.push(c);
            prev_is_lower_or_digit = c.is_lowercase() || c.is_ascii_digit();
        }
    }
    result
}

fn generate_classes_mod(modules: &[(String, String)]) -> String {
    let mut output = String::new();
    output.push_str("#![allow(unused_imports)]\n\n");
    for (module_name, _) in modules {
        output.push_str(&format!("pub mod {module_name};\n"));
    }
    output.push('\n');
    for (module_name, class_name) in modules {
        if module_name == class_name {
            output.push_str(&format!("pub use self::{module_name}::*;\n"));
        } else {
            output.push_str(&format!("pub use {module_name}::*;\n"));
        }
    }
    output
}

fn generate_root_mod(use_cocos_bindgen: bool) -> String {
    if use_cocos_bindgen {
        "pub mod cocos;\npub mod types;\npub mod classes;\npub mod functions;\n".to_string()
    } else {
        "pub mod types;\npub mod classes;\npub mod functions;\n".to_string()
    }
}
