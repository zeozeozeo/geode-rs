use crate::android_symbol::generate_android_symbol;
use broma_rs::{Function, FunctionBindField, FunctionType, PlatformNumber};

use crate::platform::Platform;
use crate::types::cpp_to_rust_type;

const INLINE: isize = -2;
const UNSPECIFIED: isize = -1;

pub fn generate_free_functions(
    functions: &[Function],
    platform: Platform,
    generate_docs: bool,
) -> String {
    let mut output = String::new();
    output.push_str(
        "#![allow(unused_imports, non_snake_case, dead_code, unsafe_op_in_unsafe_fn, clippy::missing_safety_doc, clippy::too_many_arguments)]\n\n",
    );
    output.push_str("use std::ffi::c_void;\nuse crate::base;\n\n");

    for func in functions {
        let generated = generate_free_function(func, platform, generate_docs);
        output.push_str(&generated);
        output.push('\n');
    }

    output
}

fn generate_free_function(func: &Function, platform: Platform, generate_docs: bool) -> String {
    let mut output = String::new();

    if generate_docs && !func.prototype.attributes.docs.is_empty() {
        output.push_str(&format!("/// {}\n", func.prototype.attributes.docs));
    }

    let name = sanitize_function_name(&func.prototype.name);
    let ret_type = cpp_to_rust_type(&func.prototype.ret.name);
    let args: Vec<(String, String)> = func
        .prototype
        .args
        .iter()
        .map(|arg| {
            let ty = cpp_to_rust_type(&arg.ty.name);
            (sanitize_arg_name(&arg.name), ty.to_rust_str())
        })
        .collect();

    let addr = get_platform_address(&func.binds, platform);

    if addr == INLINE || addr == UNSPECIFIED {
        return format!("// {} - inline or unspecified\n", name);
    }

    let addr_hex = format!("0x{:x}", addr);

    let fn_type_args: Vec<String> = args.iter().map(|(_, ty)| ty.clone()).collect();
    let fn_type = format!(
        "extern \"C\" fn({}) -> {}",
        fn_type_args.join(", "),
        ret_type.to_rust_str()
    );

    let call_args: Vec<String> = args.iter().map(|(name, _)| name.clone()).collect();

    output.push_str(&format!(
        "pub fn {}({}) -> {} {{\n",
        name,
        args.iter()
            .map(|(n, t)| format!("{}: {}", n, t))
            .collect::<Vec<_>>()
            .join(", "),
        ret_type.to_rust_str()
    ));

    output.push_str(&format!("    static ADDR: usize = {};\n", addr_hex));
    output.push_str(&format!(
        "    unsafe {{\n        let func: {} = std::mem::transmute(base::get() + ADDR);\n        func({})\n    }}\n",
        fn_type,
        call_args.join(", ")
    ));
    output.push_str("}\n\n");

    output
}

pub fn generate_member_function(
    func: &FunctionBindField,
    class_name: &str,
    platform: Platform,
    generate_docs: bool,
    overload_suffix: Option<&str>,
    is_impl: bool,
) -> String {
    let mut output = String::new();

    if generate_docs && !func.prototype.attributes.docs.is_empty() {
        output.push_str(&format!("/// {}\n", func.prototype.attributes.docs));
    }

    let name = sanitize_function_name(&func.prototype.name);
    let ret_type = cpp_to_rust_type(&func.prototype.ret.name);

    let is_static = func.prototype.is_static;

    let mut fn_type_args: Vec<(String, String)> = Vec::new();
    let mut ref_args: Vec<(String, String)> = Vec::new();

    if !is_static {
        fn_type_args.push(("this".to_string(), format!("*mut {}", class_name)));
        if is_impl {
            ref_args.push(("self".to_string(), format!("&mut {}", class_name)));
        } else {
            ref_args.push(("this".to_string(), format!("&mut {}", class_name)));
        }
    }

    for arg in &func.prototype.args {
        let ty = cpp_to_rust_type(&arg.ty.name);
        let arg_name = sanitize_arg_name(&arg.name);
        let (ref_ty, fn_type_ty) = to_ref_types(&ty);
        fn_type_args.push((arg_name.clone(), fn_type_ty));
        ref_args.push((arg_name, ref_ty));
    }

    let addr = get_platform_address(&func.binds, platform);

    if addr == INLINE || addr == UNSPECIFIED {
        return format!("// {}::{} - inline or unspecified\n", class_name, name);
    }

    let func_name = if func.prototype.fn_type == FunctionType::Constructor {
        let suffix = overload_suffix.unwrap_or("");
        format!("{}_ctor{}", to_snake_case(class_name), suffix)
    } else if func.prototype.fn_type == FunctionType::Destructor {
        format!("{}_dtor", to_snake_case(class_name))
    } else if let Some(suffix) = overload_suffix {
        format!("{}{}", name, suffix)
    } else {
        name.clone()
    };

    let addr_const_name = format!("{}_ADDR", func_name.to_uppercase());
    output.push_str(&generate_platform_addresses_const(
        &func_name,
        &func.binds,
        class_name,
        func,
    ));
    output.push('\n');

    let convention = "extern \"C\"";

    let fn_type_args_str: Vec<String> = fn_type_args.iter().map(|(_, ty)| ty.clone()).collect();
    let fn_type = format!(
        "{} fn({}) -> {}",
        convention,
        fn_type_args_str.join(", "),
        ret_type.to_rust_str()
    );

    let mut call_args: Vec<String> = Vec::new();
    for (n, ref_ty) in &ref_args {
        if ref_ty.starts_with("&mut ") {
            call_args.push(format!("{} as *mut _", n));
        } else if ref_ty.starts_with("&") {
            call_args.push(format!("{} as *const _", n));
        } else {
            call_args.push(n.clone());
        }
    }

    output.push_str(&format!(
        "pub fn {}({}) -> {} {{\n",
        func_name,
        ref_args
            .iter()
            .map(|(n, t)| {
                if n == "self" {
                    "&mut self".to_string()
                } else {
                    format!("{}: {}", n, t)
                }
            })
            .collect::<Vec<_>>()
            .join(", "),
        ret_type.to_rust_str()
    ));

    output.push_str(&format!(
        "    unsafe {{\n        let func: {fn_type} = std::mem::transmute(base::get() + {prefix}{addr}());\n        func({args})\n    }}\n",
        fn_type = fn_type,
        prefix = if is_impl { "Self::" } else { "" },
        addr = addr_const_name,
        args = call_args.join(", ")
    ));
    output.push_str("}\n\n");

    output
}

fn to_ref_types(ty: &crate::types::RustType) -> (String, String) {
    use crate::types::RustType;
    match ty {
        RustType::Pointer(inner, is_const) => {
            let inner_str = inner.to_rust_str();
            if *is_const {
                (
                    format!("*const {}", inner_str),
                    format!("*const {}", inner_str),
                )
            } else {
                if let RustType::KnownClass(_) = inner.as_ref() {
                    (format!("&mut {}", inner_str), format!("*mut {}", inner_str))
                } else {
                    (format!("*mut {}", inner_str), format!("*mut {}", inner_str))
                }
            }
        }
        _ => (ty.to_rust_str(), ty.to_rust_str()),
    }
}

pub fn generate_platform_addresses_const(
    func_name: &str,
    binds: &PlatformNumber,
    class_name: &str,
    func: &FunctionBindField,
) -> String {
    let mut output = String::new();
    let const_name = format!("{}_ADDR", func_name.to_uppercase());

    output.push_str(&format!("pub fn {}() -> usize {{\n", const_name));

    output.push_str("    #[cfg(target_os = \"windows\")]");
    output.push_str(&format!(" {{ return 0x{:x}; }}\n", binds.win));
    output.push_str("    #[cfg(all(target_os = \"macos\", target_arch = \"x86_64\"))]");
    output.push_str(&format!(" {{ return 0x{:x}; }}\n", binds.imac));
    output.push_str("    #[cfg(all(target_os = \"macos\", target_arch = \"aarch64\"))]");
    output.push_str(&format!(" {{ return 0x{:x}; }}\n", binds.m1));
    output.push_str("    #[cfg(target_os = \"ios\")]");
    output.push_str(&format!(" {{ return 0x{:x}; }}\n", binds.ios));

    output.push_str("    #[cfg(all(target_os = \"android\", target_arch = \"arm\"))]");
    if binds.android32 > 0 {
        output.push_str(&format!(" {{ return 0x{:x}; }}\n", binds.android32));
    } else {
        let android_symbol = generate_android_symbol(class_name, func);
        output.push_str(&format!(
            " {{ static A: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0); return crate::base::android_resolve_sym(b\"{sym}\\0\", &A); }}\n",
            sym = android_symbol
        ));
    }

    output.push_str("    #[cfg(all(target_os = \"android\", target_arch = \"aarch64\"))]");
    if binds.android64 > 0 {
        output.push_str(&format!(" {{ return 0x{:x}; }}\n", binds.android64));
    } else {
        let android_symbol = generate_android_symbol(class_name, func);
        output.push_str(&format!(
            " {{ static A: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0); return crate::base::android_resolve_sym(b\"{sym}\\0\", &A); }}\n",
            sym = android_symbol
        ));
    }

    output.push_str("    0\n}\n");

    output
}

fn get_platform_address(binds: &PlatformNumber, platform: Platform) -> isize {
    match platform {
        Platform::Windows => binds.win,
        Platform::MacIntel => binds.imac,
        Platform::MacArm => binds.m1,
        Platform::IOS => binds.ios,
        Platform::Android32 => binds.android32,
        Platform::Android64 => binds.android64,
    }
}

fn sanitize_function_name(name: &str) -> String {
    if name == "new" {
        return "create".to_string();
    }
    if let Some(stripped) = name.strip_prefix('~') {
        return format!("destructor_{}", stripped);
    }
    to_snake_case(name)
}

fn sanitize_arg_name(name: &str) -> String {
    let rust_keywords = [
        "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn",
        "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
        "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe",
        "use", "where", "while", "async", "await", "dyn", "abstract", "become", "box", "do",
        "final", "macro", "override", "priv", "typeof", "unsized", "virtual", "yield",
    ];

    if name.is_empty() {
        return "_arg".to_string();
    }

    let mut result = to_snake_case(name);

    if rust_keywords.contains(&result.as_str()) {
        result = format!("{}_", result);
    }

    result
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
