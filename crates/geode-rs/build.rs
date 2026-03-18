use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let out_dir = std::env::var("OUT_DIR")?;
    let out_path = PathBuf::from(&out_dir).join("geode_generated");

    let broma_dir = PathBuf::from("bindings/2.2081");
    let broma_files: Vec<PathBuf> = glob::glob(&format!("{}/*.bro", broma_dir.display()))?
        .filter_map(|e| e.ok())
        .collect();

    if broma_files.is_empty() {
        panic!("No .bro files found in bindings/2.2081/");
    }

    let use_cocos_bindgen = cfg!(feature = "bindgen");

    geode_codegen::generate(geode_codegen::Config {
        broma_paths: broma_files,
        output_dir: out_path.clone(),
        platform: None,
        generate_docs: true,
        separate_files: false,
        use_cocos_bindgen,
    })?;

    #[cfg(feature = "bindgen")]
    generate_cocos_bindings(&out_path)?;

    #[cfg(feature = "bindgen")]
    generate_fmod_bindings(&out_path)?;

    println!("cargo:rerun-if-changed=bindings/");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=cocos/");

    Ok(())
}

#[cfg(feature = "bindgen")]
fn generate_fmod_bindings(out_path: &std::path::Path) -> anyhow::Result<()> {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();

    let clang_target = match (target_os.as_str(), target_arch.as_str()) {
        ("macos", "aarch64") => "aarch64-apple-macosx11.0.0",
        ("macos", "x86_64") => "x86_64-apple-macosx10.15.0",
        ("ios", _) => "aarch64-apple-ios14.0",
        ("android", "aarch64") => "aarch64-linux-android21",
        ("android", _) => "armv7-linux-android21",
        _ => "x86_64-pc-win32",
    };

    let bindings = bindgen::Builder::default()
        .header("fmod/fmod.h")
        .header("fmod/fmod_codec.h")
        .header("fmod/fmod_common.h")
        .header("fmod/fmod_dsp.h")
        .header("fmod/fmod_dsp_effects.h")
        .header("fmod/fmod_errors.h")
        .header("fmod/fmod_output.h")
        .clang_arg(format!("--target={clang_target}"))
        .prepend_enum_name(false)
        .derive_debug(false);
    bindings
        .generate()
        .expect("unable to generate fmod bindings")
        .write_to_file(out_path.join("fmod.rs"))?;
    Ok(())
}

// FIXME: This is really ugly.... but what else can we do when bindgen is so stupid
#[cfg(feature = "bindgen")]
fn generate_cocos_bindings(out_path: &std::path::Path) -> anyhow::Result<()> {
    let cocos_include = PathBuf::from("cocos/include");
    let cocos_dir = PathBuf::from("cocos");

    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();

    let (platform_include, cc_platform, geode_platform, clang_target, extra_defines) =
        match (target_os.as_str(), target_arch.as_str()) {
            ("macos", "aarch64") => (
                "cocos/platform/mac",
                "CC_PLATFORM_MAC",
                "GEODE_IS_MACOS",
                "aarch64-apple-macosx11.0.0",
                vec!["-DGEODE_IS_DESKTOP"],
            ),
            ("macos", "x86_64") => (
                "cocos/platform/mac",
                "CC_PLATFORM_MAC",
                "GEODE_IS_MACOS",
                "x86_64-apple-macosx10.15.0",
                vec!["-DGEODE_IS_DESKTOP"],
            ),
            ("ios", _) => (
                "cocos/platform/ios",
                "CC_PLATFORM_IOS",
                "GEODE_IS_IOS",
                "aarch64-apple-ios14.0",
                vec![],
            ),
            ("android", "aarch64") => (
                "cocos/platform/android",
                "CC_PLATFORM_ANDROID",
                "GEODE_IS_ANDROID",
                "aarch64-linux-android21",
                vec![],
            ),
            ("android", _) => (
                "cocos/platform/android",
                "CC_PLATFORM_ANDROID",
                "GEODE_IS_ANDROID",
                "armv7-linux-android21",
                vec![],
            ),
            _ => (
                "cocos/platform/win32",
                "CC_PLATFORM_WIN32",
                "GEODE_IS_WINDOWS",
                "x86_64-pc-win32",
                vec![
                    "-DGEODE_IS_DESKTOP",
                    "-DDWORD=unsigned long",
                    "-DWORD=unsigned short",
                    "-D_AMD64_",
                    "-D_AMD64",
                    "-D_M_AMD64",
                    "-DWIN64",
                    "-D_WIN64",
                ],
            ),
        };

    let mut builder = bindgen::Builder::default()
        .header("cocos/include/cocos2d.h")
        .clang_arg("-xc++")
        .clang_arg("-std=c++20")
        .clang_arg(format!("-I{}", cocos_include.display()))
        .clang_arg(format!("-I{}", cocos_dir.display()))
        .clang_arg("-Icocos/kazmath/include")
        .clang_arg(format!("-I{platform_include}"))
        .clang_arg(format!("-DCC_TARGET_PLATFORM={cc_platform}"))
        .clang_arg(format!("-D{geode_platform}"))
        .clang_arg("-DGEODE_FRIEND_MODIFY=")
        .clang_arg("-DGEODE_CUSTOM_CONSTRUCTOR_BEGIN(Class_)=")
        .clang_arg("-DGEODE_ZERO_CONSTRUCTOR_BEGIN(Class_)=")
        .clang_arg("-DGEODE_CUTOFF_CONSTRUCTOR_BEGIN(Class_)=")
        .clang_arg("-DGEODE_DLL=")
        .clang_arg("-DGEODE_NONINHERITED_MEMBERS=")
        .clang_arg("-DGEODE_IS_MEMBER_TEST")
        .clang_arg(format!("--target={clang_target}"));

    for def in &extra_defines {
        builder = builder.clang_arg(*def);
    }

    let builder = builder
        .derive_default(true)
        .derive_copy(true)
        .derive_debug(false)
        .generate_comments(false)
        .allowlist_type("cocos2d::.*")
        .allowlist_type("enumKeyCodes")
        .allowlist_var("ccWHITE|ccYELLOW|ccBLUE|ccGREEN|ccRED|ccMAGENTA|ccBLACK|ccORANGE|ccGRAY")
        .blocklist_type("std.*")
        .blocklist_type("std_.*")
        .blocklist_type("__.*")
        .blocklist_type("_.*")
        .blocklist_type(".*locale.*")
        .blocklist_type(".*ios_base.*")
        .blocklist_type(".*char_traits.*")
        .blocklist_type(".*allocator.*")
        .blocklist_type(".*iterator.*")
        .blocklist_type(".*Mbstatet.*")
        .blocklist_type(".*basic_string.*")
        .blocklist_type(".*string_view.*")
        .blocklist_type("geode.*")
        .blocklist_type("geode_.*")
        .blocklist_type("gd.*")
        .blocklist_type("gd_.*")
        .blocklist_type("pugi.*")
        .blocklist_type("LARGE_INTEGER")
        .blocklist_type("XINPUT.*")
        .blocklist_type("std_vector")
        .blocklist_type("std_map")
        .blocklist_type("std_set")
        .blocklist_type("std_list")
        .blocklist_type("std_deque")
        .blocklist_type("std_queue")
        .blocklist_type("std_stack")
        .blocklist_type("std_multimap")
        .blocklist_type("std_multiset")
        .blocklist_type("std_unordered_map")
        .blocklist_type("std_unordered_set")
        .blocklist_type("std_unordered_multimap")
        .blocklist_type("std_unordered_multiset")
        .blocklist_type("std_pair")
        .blocklist_type("std_tuple")
        .blocklist_type("std_optional")
        .blocklist_type("std_variant")
        .blocklist_type("std_unique_ptr")
        .blocklist_type("std_shared_ptr")
        .blocklist_type("std_weak_ptr")
        .blocklist_type("std_function")
        .blocklist_type("std_allocator")
        .blocklist_type("std_basic_string")
        .blocklist_type("std_char_traits")
        .blocklist_type("std_filesystem.*")
        .blocklist_type(".*__bindgen_vtable.*")
        .blocklist_type("va_list")
        .blocklist_type("__builtin_va_list")
        .blocklist_type("__va_list_tag")
        .blocklist_type(".*libcpp.*")
        .blocklist_type(".*shared_count.*")
        .blocklist_type(".*once_flag.*")
        .blocklist_type(".*error_category.*")
        .blocklist_type(".*error_condition.*")
        .blocklist_type(".*error_code.*")
        .blocklist_type(".*system_error.*")
        .blocklist_type(".*exception.*")
        .blocklist_type(".*runtime_error.*")
        .blocklist_function("std_.*")
        .blocklist_function("__.*")
        .blocklist_function("pugi.*")
        .blocklist_function(".*libcpp.*")
        .blocklist_function(".*shared_count.*")
        .blocklist_function(".*once_flag.*")
        .blocklist_function(".*error_category.*")
        .blocklist_function(".*error_condition.*")
        .blocklist_function(".*error_code.*")
        .blocklist_function(".*system_error.*")
        .blocklist_function(".*exception.*")
        .blocklist_function(".*runtime_error.*")
        .blocklist_var("std_.*")
        .blocklist_var(".*once_flag.*")
        .blocklist_var(".*libcpp.*")
        .parse_callbacks(Box::new(CocosCallbacks));

    let bindings = builder
        .generate()
        .expect("Unable to generate cocos bindings");

    let mut output = String::new();
    output.push_str("#![allow(non_upper_case_globals)]\n");
    output.push_str("#![allow(non_camel_case_types)]\n");
    output.push_str("#![allow(non_snake_case)]\n");
    output.push_str("#![allow(dead_code)]\n");
    output.push_str("#![allow(improper_ctypes)]\n");
    output.push_str("#![allow(clippy::missing_safety_doc)]\n");
    output.push_str("#![allow(clippy::too_many_arguments)]\n");
    output.push_str("#![allow(unsafe_op_in_unsafe_fn)]\n\n");
    output.push_str("use super::types::GdString;\n");
    output.push_str("#[allow(non_camel_case_types)] pub type va_list = *mut std::ffi::c_void;\n");
    output.push_str(
        "#[allow(non_camel_case_types)] #[repr(C)] pub struct __va_list_tag { _priv: [u8; 0] }\n\n",
    );

    let bindings_str = bindings.to_string();
    let processed = process_cocos_bindings(&bindings_str);
    output.push_str(&processed);

    std::fs::write(out_path.join("cocos.rs"), output)?;

    Ok(())
}

#[cfg(feature = "bindgen")]
#[derive(Debug)]
struct CocosCallbacks;

#[cfg(feature = "bindgen")]
impl bindgen::callbacks::ParseCallbacks for CocosCallbacks {
    fn item_name(&self, item: bindgen::callbacks::ItemInfo<'_>) -> Option<String> {
        item.name
            .strip_prefix("cocos2d_")
            .map(|rest| rest.to_string())
    }
}

#[cfg(feature = "bindgen")]
fn process_cocos_bindings(input: &str) -> String {
    let mut result = String::new();
    let mut skip_until_close_brace = 0usize;
    let mut in_extern_block = false;
    let mut current_extern_content = String::new();
    let mut current_struct_content = String::new();
    let mut in_struct = false;
    let mut in_impl_block = false;
    let mut impl_brace_depth = 0usize;
    let mut current_impl_content = String::new();
    let mut skip_const_assertion = false;

    for line in input.lines() {
        let trimmed = line.trim();

        if skip_until_close_brace > 0 {
            skip_until_close_brace += line.matches('{').count();
            skip_until_close_brace -= line.matches('}').count();
            continue;
        }

        if skip_const_assertion {
            if trimmed.ends_with("};") {
                skip_const_assertion = false;
            }
            continue;
        }

        if trimmed.starts_with("#[allow(")
            && (trimmed.contains("clippy::unnecessary_operation")
                || trimmed.contains("clippy::identity_op"))
        {
            skip_const_assertion = true;
            continue;
        }

        if trimmed.starts_with("const _: () = {") {
            skip_const_assertion = true;
            continue;
        }

        if trimmed.starts_with("unsafe extern \"C\" {") || trimmed.starts_with("extern \"C\" {") {
            in_extern_block = true;
            current_extern_content.clear();
            current_extern_content.push_str(line);
            current_extern_content.push('\n');
            continue;
        }

        if in_extern_block {
            current_extern_content.push_str(line);
            current_extern_content.push('\n');

            if trimmed == "}" {
                in_extern_block = false;
                if should_keep_extern_block(&current_extern_content) {
                    result.push_str(&process_extern_block(&current_extern_content));
                }
            }
            continue;
        }

        if trimmed.starts_with("pub const std_")
            || trimmed.starts_with("pub const std__")
            || trimmed.contains("std_strong_ordering")
            || trimmed.contains("std_locale")
            || trimmed.contains("std_ios_base")
            || trimmed.contains("std__Iterator")
        {
            continue;
        }

        if trimmed.starts_with("pub type std_")
            || trimmed.starts_with("pub struct std_")
            || trimmed.starts_with("pub union std_")
            || trimmed.starts_with("pub type iterator")
            || trimmed.starts_with("pub type value_type")
            || trimmed.starts_with("pub struct pugi_")
            || trimmed.starts_with("pub type pugi_")
            || trimmed.starts_with("pub struct gd_set")
            || trimmed.starts_with("pub type geode_Anchor")
            || trimmed.starts_with("pub struct geode_Anchor")
            || trimmed.starts_with("pub type geode_Layout")
            || trimmed.starts_with("pub struct geode_Layout")
            || trimmed.starts_with("pub type geode_comm_")
            || trimmed.starts_with("pub struct geode_comm_")
            || trimmed.starts_with("pub type _LARGE_INTEGER")
            || trimmed.starts_with("pub struct _LARGE_INTEGER")
            || trimmed.starts_with("pub type _XINPUT")
            || trimmed.starts_with("pub struct _XINPUT")
        {
            skip_until_close_brace = trimmed.matches('{').count();
            if skip_until_close_brace == 0 && !trimmed.ends_with(';') {
                skip_until_close_brace = 1;
            }
            continue;
        }

        if trimmed.starts_with("pub struct ")
            && (trimmed.contains("std_vector<")
                || trimmed.contains("std_map<")
                || trimmed.contains("std_set<")
                || trimmed.contains("std_list<")
                || trimmed.contains("std_deque<")
                || trimmed.contains("std_unordered_map<")
                || trimmed.contains("std_unordered_set<")
                || trimmed.contains("gd_vector<")
                || trimmed.contains("gd_map<")
                || trimmed.contains("gd_set<"))
        {
            skip_until_close_brace = trimmed.matches('{').count();
            if skip_until_close_brace == 0 {
                skip_until_close_brace = 1;
            }
            continue;
        }

        if trimmed.starts_with("#[repr(C)]") || trimmed.starts_with("pub struct ") {
            in_struct = true;
            current_struct_content.clear();
        }

        if in_struct {
            current_struct_content.push_str(line);
            current_struct_content.push('\n');

            if trimmed.starts_with("}")
                || trimmed.starts_with("} ")
                || (trimmed.ends_with("}") && !trimmed.starts_with("#"))
            {
                in_struct = false;
                result.push_str(&replace_struct_template_types(&current_struct_content));
            }
            continue;
        }

        if trimmed.starts_with("impl ") {
            in_impl_block = true;
            impl_brace_depth = 0;
            current_impl_content.clear();
        }

        if in_impl_block {
            current_impl_content.push_str(line);
            current_impl_content.push('\n');
            impl_brace_depth += line.matches('{').count();
            impl_brace_depth -= line.matches('}').count();

            if impl_brace_depth == 0 {
                in_impl_block = false;
                result.push_str(&process_impl_block(&current_impl_content));
            }
            continue;
        }

        let processed_line = process_line(line);
        result.push_str(&processed_line);
        result.push('\n');
    }

    replace_all_template_types(&result)
}

#[cfg(feature = "bindgen")]
fn replace_struct_template_types(s: &str) -> String {
    let mut result = s.to_string();
    result = result.replace("cocos2d_", "");
    result = result.replace("gd_string", "GdString");
    result = result.replace("::std::os::raw::", "std::ffi::");
    result = result.replace("::std::option::Option", "Option");
    result = result.replace("::std::mem::", "std::mem::");
    result = result.replace("std_string_view", "GdString");
    result = result.replace("std_string", "GdString");
    result = result.replace("std_filesystem_path", "std::ffi::c_void");
    result = result.replace("geode_ZStringView", "GdString");
    result = result.replace("geode_LayoutOptions", "std::ffi::c_void");
    result = result.replace("geode_Layout", "std::ffi::c_void");
    result = result.replace("geode_Anchor", "std::ffi::c_int");
    result = result.replace("geode_comm_ListenerHandle", "std::ffi::c_void");
    result = result.replace("iterator", "std::ffi::c_void");
    result = result.replace("pugi_xml_document", "std::ffi::c_void");
    result = result.replace("value_type", "std::ffi::c_void");
    result = result.replace("_LARGE_INTEGER", "std::ffi::c_void");
    result = result.replace("_XINPUT_GAMEPAD", "std::ffi::c_void");
    result = result.replace("_XINPUT_STATE", "std::ffi::c_void");
    result = result.replace("LARGE_INTEGER", "std::ffi::c_void");
    result = result.replace("XINPUT_STATE", "std::ffi::c_void");
    result = replace_vtable_types(&result);
    result = replace_all_template_types(&result);
    result
}

#[cfg(feature = "bindgen")]
fn replace_vtable_types(s: &str) -> String {
    let mut result = s.to_string();
    let re = regex::Regex::new(r"\w+__bindgen_vtable").unwrap();
    result = re.replace_all(&result, "std::ffi::c_void").to_string();
    result
}

#[cfg(feature = "bindgen")]
fn process_impl_block(content: &str) -> String {
    let mut result = String::new();
    let mut in_method = false;
    let mut brace_depth = 0usize;
    let mut seen_open_brace = false;
    let mut current_method = String::new();

    for line in content.lines() {
        let trimmed = line.trim();

        if !in_method {
            if trimmed.starts_with("#[inline]")
                || trimmed.starts_with("#[doc]")
                || trimmed.starts_with("pub unsafe fn")
                || trimmed.starts_with("pub fn")
                || trimmed.starts_with("unsafe fn")
                || trimmed.starts_with("fn ")
            {
                in_method = true;
                seen_open_brace = line.contains('{');
                current_method.clear();
                current_method.push_str(line);
                current_method.push('\n');
                brace_depth = line.matches('{').count();
                brace_depth = brace_depth.saturating_sub(line.matches('}').count());
                if seen_open_brace && brace_depth == 0 {
                    if !contains_missing_function(&current_method) {
                        result.push_str(&rewrite_generated_impl_method(&current_method));
                    }
                    in_method = false;
                    current_method.clear();
                }
            } else {
                result.push_str(line);
                result.push('\n');
            }
        } else {
            current_method.push_str(line);
            current_method.push('\n');
            if line.contains('{') {
                seen_open_brace = true;
            }
            brace_depth += line.matches('{').count();
            brace_depth = brace_depth.saturating_sub(line.matches('}').count());

            if seen_open_brace && brace_depth == 0 {
                if !contains_missing_function(&current_method) {
                    result.push_str(&rewrite_generated_impl_method(&current_method));
                }
                in_method = false;
                current_method.clear();
            }
        }
    }

    result = result.replace("cocos2d_", "");
    result = result.replace("gd_string", "GdString");
    result = result.replace("::std::os::raw::", "std::ffi::");
    result = result.replace("::std::option::Option", "Option");
    result = result.replace("::std::mem::", "std::mem::");
    result = result.replace("std_string_view", "GdString");
    result = result.replace("std_string", "GdString");
    result = result.replace("std_filesystem_path", "std::ffi::c_void");
    result = result.replace("geode_ZStringView", "GdString");
    result = result.replace("geode_LayoutOptions", "std::ffi::c_void");
    result = result.replace("geode_Layout", "std::ffi::c_void");
    result = result.replace("geode_Anchor", "std::ffi::c_int");
    result = result.replace("geode_comm_ListenerHandle", "std::ffi::c_void");
    result = result.replace("iterator", "std::ffi::c_void");
    result = result.replace("pugi_xml_document", "std::ffi::c_void");
    result = result.replace("value_type", "std::ffi::c_void");
    result = result.replace("_LARGE_INTEGER", "std::ffi::c_void");
    result = result.replace("_XINPUT_GAMEPAD", "std::ffi::c_void");
    result = result.replace("_XINPUT_STATE", "std::ffi::c_void");
    result = result.replace("LARGE_INTEGER", "std::ffi::c_void");
    result = result.replace("XINPUT_STATE", "std::ffi::c_void");
    result = replace_vtable_types(&result);
    result
}

#[cfg(feature = "bindgen")]
fn rewrite_generated_impl_method(method: &str) -> String {
    let method = method
        .replacen("pub unsafe fn", "pub fn", 1)
        .replacen("unsafe fn", "fn", 1);

    let Some(body_start) = method.find('{') else {
        return method;
    };
    let Some(body_end) = method.rfind('}') else {
        return method;
    };

    let body = &method[body_start + 1..body_end];
    let body_indent = body
        .lines()
        .find(|line| !line.trim().is_empty())
        .map(|line| {
            line.chars()
                .take_while(|ch| ch.is_whitespace())
                .collect::<String>()
        })
        .unwrap_or_else(|| "    ".to_string());
    let inner_indent = format!("{body_indent}    ");

    let mut rewritten = String::with_capacity(method.len() + 32);
    rewritten.push_str(&method[..body_start + 1]);
    rewritten.push('\n');
    rewritten.push_str(&body_indent);
    rewritten.push_str("#[allow(unused_unsafe)]\n");
    rewritten.push_str(&body_indent);
    rewritten.push_str("unsafe {\n");

    for line in body.lines() {
        if line.trim().is_empty() {
            rewritten.push('\n');
        } else {
            rewritten.push_str(&inner_indent);
            rewritten.push_str(line.trim_start());
            rewritten.push('\n');
        }
    }

    rewritten.push_str(&body_indent);
    rewritten.push_str("}\n");
    rewritten.push_str(&method[body_end..]);
    rewritten
}

#[cfg(feature = "bindgen")]
fn contains_missing_function(content: &str) -> bool {
    content.contains("CCNode_addChildAtPosition")
        || content.contains("CCNode_updateAnchoredPosition")
        || content.contains("CCNode_removeEventListener")
        || content.contains("CCNode_getEventListener")
        || content.contains("ZipFile_unzipAllTo")
        || content.contains("__bindgen_vtable")
}

#[cfg(feature = "bindgen")]
fn should_keep_extern_block(content: &str) -> bool {
    !content.contains("std_strong_ordering")
        && !content.contains("std_locale")
        && !content.contains("std_ios_base")
        && !content.contains("std__Iterator")
        && !content.contains("?_Id_cnt@id@locale@std")
        && !content.contains("?_Clocptr@_Locimp@locale@std")
        && !content.contains("?classic@locale@std")
        && !content.contains("?global@locale@std")
        && !content.contains("?empty@locale@std")
        && !content.contains("?_Init_cnt@Init@ios_base@std")
        && !content.contains("?_Addstd@ios_base@std")
        && !content.contains("?_Index@ios_base@std")
        && !content.contains("?_Sync@ios_base@std")
        && !content.contains("pugi_xml")
        && !content.contains("gd_set<")
        && !content.contains("geode_Anchor")
        && !content.contains("geode_comm_")
        && !content.contains("std_filesystem_path")
}

#[cfg(feature = "bindgen")]
fn process_extern_block(content: &str) -> String {
    let mut result = content.to_string();
    result = result.replace("cocos2d_", "");
    result = result.replace("gd_string", "GdString");
    result = result.replace("::std::os::raw::", "std::ffi::");
    result = result.replace("::std::option::Option", "Option");
    result = result.replace("::std::mem::", "std::mem::");
    result = result.replace("std_string_view", "GdString");
    result = result.replace("std_string", "GdString");
    result = result.replace("geode_ZStringView", "GdString");
    result = result.replace("geode_LayoutOptions", "std::ffi::c_void");
    result = result.replace("geode_Layout", "std::ffi::c_void");
    result = result.replace("geode_Anchor", "std::ffi::c_int");
    result = result.replace("geode_comm_ListenerHandle", "std::ffi::c_void");
    result = result.replace("iterator", "std::ffi::c_void");
    result = result.replace("LARGE_INTEGER", "std::ffi::c_void");
    result = result.replace("XINPUT_STATE", "std::ffi::c_void");
    result = replace_vtable_types(&result);
    generate_dynamic_function_wrapper(&result).unwrap_or(result)
}

#[cfg(feature = "bindgen")]
fn generate_dynamic_function_wrapper(content: &str) -> Option<String> {
    if !content.contains("pub fn ") {
        return None;
    }

    let link_name_re = regex::Regex::new(r#"#\[link_name = "([^"]+)""#).unwrap();
    let symbol = link_name_re.captures(content)?.get(1)?.as_str();
    let symbol = symbol.strip_prefix(r"\u{1}").unwrap_or(symbol);

    let fn_start = content.find("pub fn ")?;
    let fn_decl = content[fn_start..]
        .split_once(';')
        .map(|(decl, _)| decl.trim())?;

    let name_start = "pub fn ".len();
    let open_paren = fn_decl[name_start..].find('(')? + name_start;
    let fn_name = fn_decl[name_start..open_paren].trim();

    let close_paren = find_matching_delimiter(fn_decl, open_paren, '(', ')')?;
    let params_str = &fn_decl[open_paren + 1..close_paren];
    let return_str = fn_decl[close_paren + 1..].trim();

    let params = split_top_level(params_str, ',');
    let mut call_args = Vec::new();
    let mut fn_types = Vec::new();
    for param in params.iter().map(|p| p.trim()).filter(|p| !p.is_empty()) {
        let colon = find_top_level_colon(param)?;
        let name = param[..colon].trim();
        let ty = param[colon + 1..].trim();
        call_args.push(name.to_string());
        fn_types.push(ty.to_string());
    }

    let return_clause = if return_str.is_empty() {
        String::new()
    } else {
        format!(" {return_str}")
    };
    let windows_value_return = windows_symbol_needs_sret(symbol);
    let android_aarch64_value_return = android_aarch64_symbol_needs_sret(return_str);
    let is_method = params
        .first()
        .and_then(|param| param.split(':').next())
        .map(str::trim)
        == Some("this");

    let fn_type = if fn_types.is_empty() {
        format!("unsafe extern \"C\" fn(){return_clause}")
    } else {
        format!(
            "unsafe extern \"C\" fn({}){return_clause}",
            fn_types.join(", ")
        )
    };

    let call_expr = if call_args.is_empty() {
        "func()".to_string()
    } else {
        format!("func({})", call_args.join(", "))
    };
    let windows_call = if windows_value_return {
        let ret_ty = return_str.strip_prefix("-> ").unwrap_or(return_str).trim();
        if is_method {
            let this_ty = fn_types.first()?.clone();
            let rest_types = if fn_types.len() > 1 {
                format!(", {}", fn_types[1..].join(", "))
            } else {
                String::new()
            };
            let rest_args = if call_args.len() > 1 {
                format!(", {}", call_args[1..].join(", "))
            } else {
                String::new()
            };
            let this_arg = call_args.first()?;
            format!(
                "let mut out = std::mem::MaybeUninit::<{ret_ty}>::uninit();\n            let func: unsafe extern \"system\" fn({this_ty}, *mut {ret_ty}{rest_types}) -> () = std::mem::transmute(addr);\n            func({this_arg}, out.as_mut_ptr(){rest_args});\n            out.assume_init()"
            )
        } else {
            let arg_types = if fn_types.is_empty() {
                String::new()
            } else {
                format!(", {}", fn_types.join(", "))
            };
            let arg_names = if call_args.is_empty() {
                String::new()
            } else {
                format!(", {}", call_args.join(", "))
            };
            format!(
                "let mut out = std::mem::MaybeUninit::<{ret_ty}>::uninit();\n            let func: unsafe extern \"system\" fn(*mut {ret_ty}{arg_types}) -> () = std::mem::transmute(addr);\n            func(out.as_mut_ptr(){arg_names});\n            out.assume_init()"
            )
        }
    } else {
        format!("let func: {fn_type} = std::mem::transmute(addr);\n            {call_expr}")
    };
    let android_aarch64_call = if android_aarch64_value_return {
        let ret_ty = return_str.strip_prefix("-> ").unwrap_or(return_str).trim();
        emit_android_aarch64_sret_call(&call_args, ret_ty)
    } else {
        format!("let func: {fn_type} = std::mem::transmute(addr);\n        {call_expr}")
    };

    let symbol_bytes = rust_byte_string(symbol);

    Some(format!(
        "#[inline]\npub unsafe fn {fn_name}({params_str}){return_clause} {{\n    fn __addr() -> usize {{\n        #[cfg(target_os = \"windows\")]\n        {{\n            static A: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);\n            crate::base::resolve_windows_symbol_in_modules_abs(\n                &[\n                    crate::base::get_cocos(),\n                    crate::base::get_extensions(),\n                    crate::base::get(),\n                    crate::base::get_geode(),\n                ],\n                {symbol_bytes},\n                &A,\n            )\n        }}\n        #[cfg(all(target_os = \"android\", target_arch = \"arm\"))]\n        {{\n            static A: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);\n            crate::base::android_resolve_symbol_abs({symbol_bytes}, &A)\n        }}\n        #[cfg(all(target_os = \"android\", target_arch = \"aarch64\"))]\n        {{\n            static A: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);\n            crate::base::android_resolve_symbol_abs({symbol_bytes}, &A)\n        }}\n        #[cfg(any(target_os = \"macos\", target_os = \"ios\"))]\n        {{\n            static A: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);\n            crate::base::resolve_dylib_symbol_abs({symbol_bytes}, &A)\n        }}\n        #[cfg(not(any(\n            target_os = \"windows\",\n            all(target_os = \"android\", target_arch = \"arm\"),\n            all(target_os = \"android\", target_arch = \"aarch64\"),\n            target_os = \"macos\",\n            target_os = \"ios\"\n        )))]\n        {{\n            0\n        }}\n    }}\n\n    let addr = __addr();\n    assert!(addr != 0, \"failed to resolve {fn_name}\");\n    #[cfg(target_os = \"windows\")]\n    {{\n        {windows_call}\n    }}\n    #[cfg(all(target_os = \"android\", target_arch = \"aarch64\"))]\n    {{\n        {android_aarch64_call}\n    }}\n    #[cfg(not(any(target_os = \"windows\", all(target_os = \"android\", target_arch = \"aarch64\"))))]\n    {{\n        let func: {fn_type} = std::mem::transmute(addr);\n        {call_expr}\n    }}\n}}\n"
    ))
}

#[cfg(feature = "bindgen")]
fn find_matching_delimiter(s: &str, start: usize, open: char, close: char) -> Option<usize> {
    let mut depth = 0usize;
    for (offset, ch) in s[start..].char_indices() {
        if ch == open {
            depth += 1;
        } else if ch == close {
            depth = depth.saturating_sub(1);
            if depth == 0 {
                return Some(start + offset);
            }
        }
    }
    None
}

#[cfg(feature = "bindgen")]
fn split_top_level(s: &str, separator: char) -> Vec<String> {
    let mut parts = Vec::new();
    let mut start = 0usize;
    let mut paren = 0usize;
    let mut angle = 0usize;
    let mut bracket = 0usize;
    let mut brace = 0usize;

    for (idx, ch) in s.char_indices() {
        match ch {
            '(' => paren += 1,
            ')' => paren = paren.saturating_sub(1),
            '<' => angle += 1,
            '>' => angle = angle.saturating_sub(1),
            '[' => bracket += 1,
            ']' => bracket = bracket.saturating_sub(1),
            '{' => brace += 1,
            '}' => brace = brace.saturating_sub(1),
            _ => {}
        }

        if ch == separator && paren == 0 && angle == 0 && bracket == 0 && brace == 0 {
            parts.push(s[start..idx].to_string());
            start = idx + ch.len_utf8();
        }
    }

    if start < s.len() {
        parts.push(s[start..].to_string());
    }

    parts
}

#[cfg(feature = "bindgen")]
fn find_top_level_colon(s: &str) -> Option<usize> {
    let mut paren = 0usize;
    let mut angle = 0usize;
    let mut bracket = 0usize;
    let mut brace = 0usize;

    for (idx, ch) in s.char_indices() {
        match ch {
            '(' => paren += 1,
            ')' => paren = paren.saturating_sub(1),
            '<' => angle += 1,
            '>' => angle = angle.saturating_sub(1),
            '[' => bracket += 1,
            ']' => bracket = bracket.saturating_sub(1),
            '{' => brace += 1,
            '}' => brace = brace.saturating_sub(1),
            ':' if paren == 0 && angle == 0 && bracket == 0 && brace == 0 => return Some(idx),
            _ => {}
        }
    }

    None
}

#[cfg(feature = "bindgen")]
fn rust_byte_string(s: &str) -> String {
    let mut out = String::from("b\"");
    for &byte in s.as_bytes() {
        match byte {
            b'\\' => out.push_str("\\\\"),
            b'"' => out.push_str("\\\""),
            b'\n' => out.push_str("\\n"),
            b'\r' => out.push_str("\\r"),
            b'\t' => out.push_str("\\t"),
            0x20..=0x7e => out.push(byte as char),
            _ => out.push_str(&format!("\\x{byte:02x}")),
        }
    }
    out.push_str("\\0\"");
    out
}

#[cfg(feature = "bindgen")]
fn windows_symbol_needs_sret(symbol: &str) -> bool {
    if !symbol.starts_with('?') {
        return false;
    }

    regex::Regex::new(r"@@[^@]*\?A[UV]")
        .unwrap()
        .is_match(symbol)
}

#[cfg(feature = "bindgen")]
fn android_aarch64_symbol_needs_sret(return_str: &str) -> bool {
    let ty = return_str
        .strip_prefix("->")
        .map(str::trim)
        .unwrap_or("")
        .trim();

    if ty.is_empty()
        || ty == "()"
        || ty.starts_with('*')
        || ty.starts_with('&')
        || ty.starts_with("Option<")
        || ty.starts_with("unsafe extern")
        || ty.starts_with("extern ")
        || ty.contains("fn(")
        || ty.contains("c_void")
    {
        return false;
    }

    let root = ty
        .split(['<', ' ', '[', ',', ';'])
        .next()
        .unwrap_or(ty)
        .rsplit("::")
        .next()
        .unwrap_or(ty)
        .trim();

    if root.is_empty() {
        return false;
    }

    !matches!(
        root,
        "bool"
            | "char"
            | "f32"
            | "f64"
            | "i8"
            | "i16"
            | "i32"
            | "i64"
            | "i128"
            | "isize"
            | "u8"
            | "u16"
            | "u32"
            | "u64"
            | "u128"
            | "usize"
    ) && root
        .chars()
        .next()
        .map(|ch| ch.is_ascii_uppercase())
        .unwrap_or(false)
}

#[cfg(feature = "bindgen")]
fn emit_android_aarch64_sret_call(call_args: &[String], ret_ty: &str) -> String {
    if call_args.len() > 7 {
        return format!(
            "let func: unsafe extern \"C\" fn() -> {ret_ty} = std::mem::transmute(addr);\n        func()"
        );
    }

    let regs = ["x0", "x1", "x2", "x3", "x4", "x5", "x6"];
    let mut reg_inputs = String::new();
    for (index, arg) in call_args.iter().enumerate() {
        let reg = regs[index];
        reg_inputs.push_str(&format!(
            "in(\"{reg}\") {{\n                let mut __tmp = 0usize;\n                let __size = std::mem::size_of_val(&{arg});\n                let __copy_len = if __size > 8 {{ 8 }} else {{ __size }};\n                std::ptr::copy_nonoverlapping(\n                    &{arg} as *const _ as *const u8,\n                    &mut __tmp as *mut _ as *mut u8,\n                    __copy_len,\n                );\n                __tmp\n            }},\n            "
        ));
    }

    format!(
        "let mut out = std::mem::MaybeUninit::<{ret_ty}>::uninit();\n        unsafe {{\n            std::arch::asm!(\n                \"blr {{fn_ptr}}\",\n                fn_ptr = in(reg) addr,\n                in(\"x8\") out.as_mut_ptr() as usize,\n                {reg_inputs}clobber_abi(\"C\"),\n            );\n        }}\n        out.assume_init()"
    )
}

#[cfg(feature = "bindgen")]
fn process_line(line: &str) -> String {
    let mut result = line.to_string();
    result = result.replace("cocos2d_", "");
    result = result.replace("gd_string", "GdString");
    result = result.replace("::std::os::raw::", "std::ffi::");
    result = result.replace("::std::option::Option", "Option");
    result = result.replace("::std::mem::", "std::mem::");
    result = result.replace("std_string_view", "GdString");
    result = result.replace("std_string", "GdString");
    result = result.replace("std_filesystem_path", "std::ffi::c_void");
    result = result.replace("geode_ZStringView", "GdString");
    result = result.replace("geode_LayoutOptions", "std::ffi::c_void");
    result = result.replace("geode_Layout", "std::ffi::c_void");
    result = result.replace("geode_Anchor", "std::ffi::c_int");
    result = result.replace("geode_comm_ListenerHandle", "std::ffi::c_void");
    result = result.replace("iterator", "std::ffi::c_void");
    result = result.replace("pugi_xml_document", "std::ffi::c_void");
    result = result.replace("value_type", "std::ffi::c_void");
    result = result.replace("_LARGE_INTEGER", "std::ffi::c_void");
    result = result.replace("_XINPUT_GAMEPAD", "std::ffi::c_void");
    result = result.replace("_XINPUT_STATE", "std::ffi::c_void");
    result = result.replace("LARGE_INTEGER", "std::ffi::c_void");
    result = result.replace("XINPUT_STATE", "std::ffi::c_void");
    result = replace_vtable_types(&result);
    result
}

#[cfg(feature = "bindgen")]
fn replace_all_template_types(s: &str) -> String {
    let mut result = s.to_string();
    let templates = [
        "gd_set<",
        "gd_vector<",
        "gd_map<",
        "std_vector<",
        "std_map<",
        "std_set<",
        "std_list<",
        "std_deque<",
        "std_queue<",
        "std_stack<",
        "std_multimap<",
        "std_multiset<",
        "std_unordered_map<",
        "std_unordered_set<",
        "std_unordered_multimap<",
        "std_unordered_multiset<",
        "std_pair<",
        "std_tuple<",
        "std_optional<",
        "std_unique_ptr<",
        "std_shared_ptr<",
        "std_weak_ptr<",
        "std_function<",
        "std_allocator<",
        "std_basic_string<",
        "std_char_traits<",
    ];

    for tmpl in templates {
        while let Some(start) = result.find(tmpl) {
            let search_start = start + tmpl.len();
            let mut depth = 1i32;
            let mut end = None;
            for (i, c) in result[search_start..].chars().enumerate() {
                match c {
                    '<' => depth += 1,
                    '>' => {
                        depth -= 1;
                        if depth == 0 {
                            end = Some(search_start + i);
                            break;
                        }
                    }
                    _ => {}
                }
            }
            if let Some(end_idx) = end {
                result = format!(
                    "{}std::ffi::c_void{}",
                    &result[..start],
                    &result[end_idx + 1..]
                );
            } else {
                result = format!("{}std::ffi::c_void", &result[..start]);
                break;
            }
        }
    }
    result
}
