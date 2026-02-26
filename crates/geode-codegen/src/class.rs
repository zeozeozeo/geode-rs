use std::collections::HashMap;

use broma_rs::{Class, FieldInner, FunctionType, Root};

use crate::function::generate_member_function;
use crate::member::generate_field;
use crate::platform::Platform;

pub fn generate_class(
    class: &Class,
    root: &Root,
    platform: Platform,
    generate_docs: bool,
    generate_prelude: bool,
    import_classes: bool,
) -> String {
    let mut output = String::new();

    if generate_prelude {
        output.push_str("#![allow(unused_imports, non_snake_case, non_camel_case_types, dead_code, unsafe_op_in_unsafe_fn, clippy::missing_safety_doc, clippy::too_many_arguments, unreachable_code)]\n");
        output.push_str("use std::ffi::c_void;\n");
        output.push_str("use crate::base;\n");
        output.push_str("use crate::types::*;\n");
        if import_classes {
            output.push_str("use crate::classes::*;\n\n");
        }
    }

    if generate_docs && !class.attributes.docs.is_empty() {
        output.push_str(&format!("/// {}\n", class.attributes.docs));
    }

    let class_name = serialize_name(&class.name);
    output.push_str("#[repr(C)]\n");
    output.push_str(&format!("pub struct {} {{\n", class_name));

    if let Some(primary_base) = class.superclasses.first() {
        if let Some(base_class) = root.find_class(primary_base) {
            let base_name = serialize_name(&base_class.name);
            output.push_str(&format!("    pub base: {},\n", base_name));
        } else {
            let base_name = serialize_name(primary_base);
            output.push_str(&format!("    pub base: {},\n", base_name));
        }
    }

    for secondary_base in class.superclasses.iter().skip(1) {
        if let Some(base_class) = root.find_class(secondary_base) {
            if class_has_virtual_functions(base_class) {
                let base_name = serialize_name(secondary_base).to_string();
                output.push_str(&format!("    pub _vt_{}: *mut c_void,\n", base_name));
            } else {
                let base_name = serialize_name(&base_class.name);
                output.push_str(&format!(
                    "    pub base_{}: {},\n",
                    base_name.to_lowercase(),
                    base_name
                ));
            }
        } else {
            let base_name = serialize_name(secondary_base).to_string();
            output.push_str(&format!("    pub _vt_{}: *mut c_void,\n", base_name));
        }
    }

    let mut pad_index = 0;
    for field in &class.fields {
        if let Some(generated) = generate_field(field, pad_index) {
            output.push_str(&generated);
            output.push('\n');
            if matches!(field.inner, FieldInner::Pad(_)) {
                pad_index += 1;
            }
        }
    }

    output.push_str("}\n\n");
    output.push_str(&generate_impl_block(class, platform, generate_docs));
    output
}

fn generate_impl_block(class: &Class, platform: Platform, generate_docs: bool) -> String {
    let mut output = String::new();
    let class_name = serialize_name(&class.name);

    output.push_str(&format!("impl {} {{\n", class_name));

    let mut function_counts: HashMap<String, usize> = HashMap::new();
    let mut function_indices: HashMap<String, usize> = HashMap::new();

    for field in &class.fields {
        if let FieldInner::FunctionBind(func) = &field.inner
            && func.prototype.fn_type != FunctionType::Destructor
        {
            let base_name = get_function_base_name(func);
            *function_counts.entry(base_name).or_insert(0) += 1;
        }
    }

    for field in &class.fields {
        if let FieldInner::FunctionBind(func) = &field.inner
            && func.prototype.fn_type != FunctionType::Destructor
        {
            let base_name = get_function_base_name(func);
            let count = function_counts.get(&base_name).copied().unwrap_or(1);
            let suffix = if count > 1 {
                let idx = function_indices.entry(base_name.clone()).or_insert(0);
                *idx += 1;
                Some(format!("_{}", idx))
            } else {
                None
            };

            let generated = generate_member_function(
                func,
                class_name,
                platform,
                generate_docs,
                suffix.as_deref(),
                true,
            );
            for line in generated.lines() {
                output.push_str("    ");
                output.push_str(line);
                output.push('\n');
            }
        }
    }

    output.push_str("}\n\n");
    output.push_str(&generate_ctors_and_dtor(class, platform, generate_docs));
    output
}

fn generate_ctors_and_dtor(class: &Class, platform: Platform, generate_docs: bool) -> String {
    let mut output = String::new();
    let class_name = serialize_name(&class.name);

    let mut has_ctor = false;
    let mut ctor_count = 0;
    let mut dtor_count = 0;

    for field in &class.fields {
        if let FieldInner::FunctionBind(func) = &field.inner {
            match func.prototype.fn_type {
                FunctionType::Constructor => ctor_count += 1,
                FunctionType::Destructor => dtor_count += 1,
                FunctionType::Normal => {}
            }
        }
    }

    let mut ctor_idx = 0;
    let mut dtor_idx = 0;

    for field in &class.fields {
        if let FieldInner::FunctionBind(func) = &field.inner {
            match func.prototype.fn_type {
                FunctionType::Constructor => {
                    has_ctor = true;
                    let suffix = if ctor_count > 1 {
                        ctor_idx += 1;
                        Some(format!("_{}", ctor_idx))
                    } else {
                        None
                    };
                    let generated = generate_member_function(
                        func,
                        class_name,
                        platform,
                        generate_docs,
                        suffix.as_deref(),
                        false,
                    );
                    output.push_str(&generated);
                }
                FunctionType::Destructor => {
                    let suffix = if dtor_count > 1 {
                        dtor_idx += 1;
                        Some(format!("_{}", dtor_idx))
                    } else {
                        None
                    };
                    let generated = generate_member_function(
                        func,
                        class_name,
                        platform,
                        generate_docs,
                        suffix.as_deref(),
                        false,
                    );
                    output.push_str(&generated);
                }
                FunctionType::Normal => {}
            }
        }
    }

    if !has_ctor {
        output.push_str(&format!("// No constructor binding for {}\n\n", class_name));
    }

    output
}

fn get_function_base_name(func: &broma_rs::FunctionBindField) -> String {
    if func.prototype.fn_type == FunctionType::Constructor {
        "ctor".to_string()
    } else if func.prototype.fn_type == FunctionType::Destructor {
        "dtor".to_string()
    } else {
        to_snake_case(&func.prototype.name)
    }
}

fn to_snake_case(s: &str) -> String {
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

pub fn serialize_name(full_name: &str) -> &str {
    if let Some(pos) = full_name.rfind("::") {
        &full_name[pos + 2..]
    } else {
        full_name
    }
}

fn class_has_virtual_functions(class: &Class) -> bool {
    class.fields.iter().any(|f| {
        if let Some(func) = f.as_function_bind() {
            func.prototype.is_virtual
        } else {
            false
        }
    })
}
