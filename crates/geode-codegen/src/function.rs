use crate::android_symbol::generate_android_symbol;
use crate::windows_symbol::generate_windows_symbol;
use broma_rs::{
    Function, FunctionBindField, FunctionType, Platform as BromaPlatform, PlatformNumber,
};

use crate::platform::Platform;
use crate::types::{RustType, cpp_to_rust_type};

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
    let linked = func.prototype.attributes.links;

    if addr == INLINE || (addr == UNSPECIFIED && !is_platform_linked(linked, platform)) {
        return format!("// {} - inline or unspecified\n", name);
    }

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

    output.push_str(&format!(
        "    let addr = {resolver};\n    assert!(addr != 0, \"failed to resolve {}()\");\n    unsafe {{\n        let func: {} = std::mem::transmute(addr);\n        func({})\n    }}\n",
        name,
        fn_type,
        call_args.join(", "),
        resolver = generate_free_function_address_resolver(func, platform)
    ));
    output.push_str("}\n\n");

    output
}

pub fn generate_member_function(
    func: &FunctionBindField,
    full_class_name: &str,
    class_name: &str,
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

    if !should_generate_member_function(full_class_name, func) {
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
        full_class_name,
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
        "    let addr = {prefix}{addr}();\n    assert!(addr != 0, \"failed to resolve {class_name}::{func_name}\");\n    unsafe {{\n        let func: {fn_type} = std::mem::transmute(addr);\n        func({args})\n    }}\n",
        fn_type = fn_type,
        prefix = if is_impl { "Self::" } else { "" },
        addr = addr_const_name,
        class_name = class_name,
        func_name = func_name,
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
    full_class_name: &str,
    class_name: &str,
    func: &FunctionBindField,
) -> String {
    let mut output = String::new();
    let const_name = format!("{}_ADDR", func_name.to_uppercase());

    output.push_str(&format!("pub fn {}() -> usize {{\n", const_name));

    output.push_str(&generate_platform_branch(
        Platform::Windows,
        get_platform_address(binds, Platform::Windows),
        &func.prototype.attributes.links,
        full_class_name,
        class_name,
        func,
    ));
    output.push_str(&generate_platform_branch(
        Platform::MacIntel,
        get_platform_address(binds, Platform::MacIntel),
        &func.prototype.attributes.links,
        full_class_name,
        class_name,
        func,
    ));
    output.push_str(&generate_platform_branch(
        Platform::MacArm,
        get_platform_address(binds, Platform::MacArm),
        &func.prototype.attributes.links,
        full_class_name,
        class_name,
        func,
    ));
    output.push_str(&generate_platform_branch(
        Platform::IOS,
        get_platform_address(binds, Platform::IOS),
        &func.prototype.attributes.links,
        full_class_name,
        class_name,
        func,
    ));
    output.push_str(&generate_platform_branch(
        Platform::Android32,
        get_platform_address(binds, Platform::Android32),
        &func.prototype.attributes.links,
        full_class_name,
        class_name,
        func,
    ));
    output.push_str(&generate_platform_branch(
        Platform::Android64,
        get_platform_address(binds, Platform::Android64),
        &func.prototype.attributes.links,
        full_class_name,
        class_name,
        func,
    ));

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

fn generate_free_function_address_resolver(func: &Function, platform: Platform) -> String {
    let addr = get_platform_address(&func.binds, platform);
    if addr > 0 {
        format!("crate::base::get() + 0x{addr:x}")
    } else {
        "0".to_string()
    }
}

fn generate_platform_branch(
    platform: Platform,
    addr: isize,
    linked: &BromaPlatform,
    full_class_name: &str,
    class_name: &str,
    func: &FunctionBindField,
) -> String {
    let mut output = String::new();
    output.push_str(&format!("    #[cfg({})]", platform.cfg_condition()));

    if addr == INLINE {
        output.push_str(" { return 0; }\n");
        return output;
    }

    if addr > 0 {
        output.push_str(&format!(
            " {{ return {}; }}\n",
            absolute_address_expr(platform, full_class_name, addr as usize)
        ));
        return output;
    }

    if !can_resolve_symbol(platform, *linked, full_class_name, func) {
        output.push_str(" { return 0; }\n");
        return output;
    }

    match platform {
        Platform::Windows => {
            if let Some(symbol) = generate_windows_symbol(full_class_name, func) {
                let module = windows_module_expr(full_class_name);
                output.push_str(&format!(
                    " {{ static A: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0); return crate::base::resolve_windows_symbol_abs({module}, b\"{symbol}\\0\", &A); }}\n"
                ));
            } else {
                output.push_str(&format!(
                    " {{ let _ = \"{}::{}\"; return 0; }}\n",
                    class_name, func.prototype.name
                ));
            }
        }
        Platform::Android32 | Platform::Android64 => {
            let symbol = generate_android_symbol(full_class_name, func);
            output.push_str(&format!(
                " {{ static A: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0); return crate::base::android_resolve_symbol_abs(b\"{symbol}\\0\", &A); }}\n"
            ));
        }
        Platform::MacIntel | Platform::MacArm | Platform::IOS => {
            let symbol = generate_android_symbol(full_class_name, func);
            output.push_str(&format!(
                " {{ static A: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0); return crate::base::resolve_dylib_symbol_abs(b\"{symbol}\\0\", &A); }}\n"
            ));
        }
    }

    output
}

fn should_generate_member_function(full_class_name: &str, func: &FunctionBindField) -> bool {
    Platform::all().iter().copied().any(|platform| {
        let addr = get_platform_address(&func.binds, platform);
        addr > 0
            || (addr == UNSPECIFIED
                && can_resolve_symbol(
                    platform,
                    func.prototype.attributes.links,
                    full_class_name,
                    func,
                ))
    })
}

fn absolute_address_expr(platform: Platform, class_name: &str, addr: usize) -> String {
    match platform {
        Platform::Windows if is_in_extensions_dll(class_name) => {
            format!("crate::base::get_extensions() + 0x{addr:x}")
        }
        Platform::Windows if is_in_cocos_dll(class_name) => {
            format!("crate::base::get_cocos() + 0x{addr:x}")
        }
        _ => format!("crate::base::get() + 0x{addr:x}"),
    }
}

fn windows_module_expr(class_name: &str) -> &'static str {
    if is_in_extensions_dll(class_name) {
        "crate::base::get_extensions()"
    } else if is_in_cocos_dll(class_name) {
        "crate::base::get_cocos()"
    } else {
        "crate::base::get()"
    }
}

fn is_in_extensions_dll(class_name: &str) -> bool {
    class_name.contains("cocos2d::extension")
}

fn is_cocos_class(class_name: &str) -> bool {
    class_name.contains("cocos2d")
        || class_name.contains("pugi::")
        || matches!(
            class_name,
            "DS_Dictionary" | "ObjectDecoder" | "ObjectDecoderDelegate" | "CCContentManager"
        )
}

fn is_in_cocos_dll(class_name: &str) -> bool {
    is_cocos_class(class_name)
        && !class_name.contains("CCLightning")
        && !is_in_extensions_dll(class_name)
}

fn is_platform_linked(linked: BromaPlatform, platform: Platform) -> bool {
    match platform {
        Platform::Windows => linked.contains(BromaPlatform::Windows),
        Platform::MacIntel => {
            linked.contains(BromaPlatform::MacIntel) || linked.contains(BromaPlatform::Mac)
        }
        Platform::MacArm => {
            linked.contains(BromaPlatform::MacArm) || linked.contains(BromaPlatform::Mac)
        }
        Platform::IOS => linked.contains(BromaPlatform::IOS),
        Platform::Android32 => {
            linked.contains(BromaPlatform::Android32) || linked.contains(BromaPlatform::Android)
        }
        Platform::Android64 => {
            linked.contains(BromaPlatform::Android64) || linked.contains(BromaPlatform::Android)
        }
    }
}

fn has_symbol_source(linked: BromaPlatform, platform: Platform, class_name: &str) -> bool {
    if is_platform_linked(linked, platform) {
        return true;
    }

    matches!(
        platform,
        Platform::Windows | Platform::Android32 | Platform::Android64
    ) && is_cocos_class(class_name)
}

fn can_resolve_symbol(
    platform: Platform,
    linked: BromaPlatform,
    class_name: &str,
    func: &FunctionBindField,
) -> bool {
    if func.prototype.fn_type != FunctionType::Normal {
        return false;
    }

    if !supports_symbol_signature(func) {
        return false;
    }

    if !has_symbol_source(linked, platform, class_name) {
        return false;
    }

    match platform {
        Platform::Windows => generate_windows_symbol(class_name, func).is_some(),
        Platform::MacIntel | Platform::MacArm | Platform::IOS => true,
        Platform::Android32 | Platform::Android64 => true,
    }
}

fn supports_symbol_signature(func: &FunctionBindField) -> bool {
    supports_symbol_return_type(&cpp_to_rust_type(&func.prototype.ret.name))
        && func
            .prototype
            .args
            .iter()
            .all(|arg| supports_symbol_arg_type(&cpp_to_rust_type(&arg.ty.name)))
}

fn supports_symbol_return_type(ty: &RustType) -> bool {
    match ty {
        RustType::Primitive(_) => true,
        RustType::Pointer(inner, _) => supports_symbol_pointee_type(inner),
        _ => false,
    }
}

fn supports_symbol_arg_type(ty: &RustType) -> bool {
    match ty {
        RustType::Primitive(_) => true,
        RustType::Pointer(inner, _) => supports_symbol_pointee_type(inner),
        RustType::CocosType(name) if name == "enumKeyCodes" => true,
        _ => false,
    }
}

fn supports_symbol_pointee_type(ty: &RustType) -> bool {
    matches!(ty, RustType::Primitive(_) | RustType::KnownClass(_))
        || matches!(ty, RustType::CocosType(name) if name == "CCEvent")
}

fn sanitize_function_name(name: &str) -> String {
    if name == "new" {
        return "create".to_string();
    }
    if let Some(stripped) = name.strip_prefix('~') {
        return format!("destructor_{}", stripped);
    }
    sanitize_ident(&to_snake_case(name))
}

fn sanitize_arg_name(name: &str) -> String {
    if name.is_empty() {
        return "_arg".to_string();
    }

    sanitize_ident(&to_snake_case(name))
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

fn sanitize_ident(name: &str) -> String {
    let rust_keywords = [
        "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn",
        "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
        "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe",
        "use", "where", "while", "async", "await", "dyn", "abstract", "become", "box", "do",
        "final", "macro", "override", "priv", "typeof", "unsized", "virtual", "yield",
    ];

    if rust_keywords.contains(&name) {
        format!("{name}_")
    } else {
        name.to_string()
    }
}
