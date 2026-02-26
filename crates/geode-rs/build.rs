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
    let bindings = bindgen::Builder::default()
        .header("fmod/fmod.h")
        .header("fmod/fmod_codec.h")
        .header("fmod/fmod_common.h")
        .header("fmod/fmod_dsp.h")
        .header("fmod/fmod_dsp_effects.h")
        .header("fmod/fmod_errors.h")
        .header("fmod/fmod_output.h")
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

    let builder = bindgen::Builder::default()
        .header("cocos/include/cocos2d.h")
        .clang_arg("-xc++")
        .clang_arg("-std=c++20")
        .clang_arg(format!("-I{}", cocos_include.display()))
        .clang_arg(format!("-I{}", cocos_dir.display()))
        .clang_arg("-Icocos/kazmath/include")
        .clang_arg("-Icocos/platform/win32")
        .clang_arg("-DCC_TARGET_PLATFORM=CC_PLATFORM_WIN32")
        .clang_arg("-DGEODE_IS_WINDOWS")
        .clang_arg("-DGEODE_IS_DESKTOP")
        .clang_arg("-DGEODE_FRIEND_MODIFY=")
        .clang_arg("-DGEODE_CUSTOM_CONSTRUCTOR_BEGIN(Class_)=")
        .clang_arg("-DGEODE_ZERO_CONSTRUCTOR_BEGIN(Class_)=")
        .clang_arg("-DGEODE_CUTOFF_CONSTRUCTOR_BEGIN(Class_)=")
        .clang_arg("-DGEODE_DLL=")
        .clang_arg("-DGEODE_NONINHERITED_MEMBERS=")
        .clang_arg("-DGEODE_IS_MEMBER_TEST")
        .clang_arg("-DDWORD=unsigned long")
        .clang_arg("-DWORD=unsigned short")
        .clang_arg("-D_AMD64_")
        .clang_arg("-D_AMD64")
        .clang_arg("-D_M_AMD64")
        .clang_arg("-DWIN64")
        .clang_arg("-D_WIN64")
        .clang_arg("--target=x86_64-pc-win32")
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
        .blocklist_type("std_allocator")
        .blocklist_type("std_basic_string")
        .blocklist_type("std_char_traits")
        .blocklist_type("std_filesystem.*")
        .blocklist_type(".*__bindgen_vtable.*")
        .blocklist_function("std_.*")
        .blocklist_function("__.*")
        .blocklist_function("pugi.*")
        .blocklist_var("std_.*")
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
    output.push_str("use super::types::GdString;\n\n");

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
                        result.push_str(&current_method);
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
                    result.push_str(&current_method);
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
    result
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
