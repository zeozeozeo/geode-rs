use broma_rs::{Field, FieldInner, MemberField, PadField};

use crate::types::cpp_to_rust_type;

pub fn generate_member_field(field: &MemberField) -> String {
    let rust_type = cpp_to_rust_type(&field.ty.name);
    let type_str = rust_type.to_rust_str();
    let name = sanitize_member_name(&field.name);
    format!("    pub {}: {},", name, type_str)
}

pub fn generate_padding_field(pad: &PadField, index: usize) -> String {
    let mut output = String::new();

    if pad.amount.win > 0 {
        output.push_str(&format!(
            "    #[cfg(target_os = \"windows\")]\n    pub _pad_{}: [u8; 0x{:x}],\n",
            index, pad.amount.win as usize
        ));
    }
    if pad.amount.imac > 0 {
        output.push_str(&format!(
            "    #[cfg(all(target_os = \"macos\", target_arch = \"x86_64\"))]\n    pub _pad_{}: [u8; 0x{:x}],\n",
            index, pad.amount.imac as usize
        ));
    }
    if pad.amount.m1 > 0 {
        output.push_str(&format!(
            "    #[cfg(all(target_os = \"macos\", target_arch = \"aarch64\"))]\n    pub _pad_{}: [u8; 0x{:x}],\n",
            index, pad.amount.m1 as usize
        ));
    }
    if pad.amount.ios > 0 {
        output.push_str(&format!(
            "    #[cfg(target_os = \"ios\")]\n    pub _pad_{}: [u8; 0x{:x}],\n",
            index, pad.amount.ios as usize
        ));
    }
    if pad.amount.android32 > 0 {
        output.push_str(&format!(
            "    #[cfg(all(target_os = \"android\", target_arch = \"arm\"))]\n    pub _pad_{}: [u8; 0x{:x}],\n",
            index, pad.amount.android32 as usize
        ));
    }
    if pad.amount.android64 > 0 {
        output.push_str(&format!(
            "    #[cfg(all(target_os = \"android\", target_arch = \"aarch64\"))]\n    pub _pad_{}: [u8; 0x{:x}],\n",
            index, pad.amount.android64 as usize
        ));
    }

    output.trim_end_matches('\n').to_string()
}

fn has_any_padding(pad: &PadField) -> bool {
    pad.amount.win > 0
        || pad.amount.imac > 0
        || pad.amount.m1 > 0
        || pad.amount.ios > 0
        || pad.amount.android32 > 0
        || pad.amount.android64 > 0
}

pub fn sanitize_member_name(name: &str) -> String {
    let rust_keywords = [
        "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn",
        "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
        "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe",
        "use", "where", "while", "async", "await", "dyn", "abstract", "become", "box", "do",
        "final", "macro", "override", "priv", "typeof", "unsized", "virtual", "yield",
    ];

    let mut result = if let Some(stripped) = name.strip_prefix("m_") {
        stripped.to_string()
    } else if let Some(stripped) = name.strip_prefix('_') {
        format!("underscore_{}", stripped)
    } else {
        name.to_string()
    };

    if result
        .chars()
        .next()
        .map(|c| c.is_numeric())
        .unwrap_or(false)
    {
        result = format!("field_{}", result);
    }

    if rust_keywords.contains(&result.as_str()) {
        result = format!("{}_", result);
    }

    result
}

pub fn generate_field(field: &Field, pad_index: usize) -> Option<String> {
    match &field.inner {
        FieldInner::Member(member) => Some(generate_member_field(member)),
        FieldInner::Pad(pad) => {
            if has_any_padding(pad) {
                Some(generate_padding_field(pad, pad_index))
            } else {
                None
            }
        }
        FieldInner::FunctionBind(_) => None,
        FieldInner::Inline(_) => None,
    }
}
