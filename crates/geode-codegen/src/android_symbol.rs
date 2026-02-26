/// Port of bindings/codegen/src/AndroidSymbol.cpp
use broma_rs::{FunctionBindField, FunctionType};

fn mangle_ident(s: &str, ne: bool) -> String {
    if s.contains("::") {
        let mut result = if ne { "N".to_string() } else { String::new() };
        let mut remaining = s;
        loop {
            let (part, rest) = match remaining.find("::") {
                Some(i) => (&remaining[..i], &remaining[i + 2..]),
                None => (remaining, ""),
            };
            result.push_str(&format!("{}{}", part.len(), part));
            if rest.is_empty() {
                break;
            }
            remaining = rest;
        }
        if ne {
            result.push('E');
        }
        result
    } else {
        format!("{}{}", s.len(), s)
    }
}

fn int_to_base36(mut value: u32) -> String {
    const BASE36: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut result = Vec::new();
    loop {
        result.push(BASE36[(value % 36) as usize] as char);
        value /= 36;
        if value == 0 {
            break;
        }
    }
    result.iter().rev().collect()
}

fn look_for_seen(seen: &[String], expanded: &str) -> Option<String> {
    for (i, s) in seen.iter().enumerate() {
        if s == expanded {
            return Some(if i == 0 {
                "S_".to_string()
            } else {
                format!("S{}_", int_to_base36((i - 1) as u32))
            });
        }
    }
    None
}

fn subs_seen(seen: &mut Vec<String>, mangled: String, subs: bool, expanded: &str) -> String {
    if !subs || mangled.is_empty() {
        return mangled;
    }
    if let Some(x) = look_for_seen(seen, expanded) {
        return x;
    }
    seen.push(expanded.to_string());
    mangled
}

fn split_template_recursive(s: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut depth = 0i32;
    let mut current = String::new();
    for c in s.chars() {
        match c {
            '<' => {
                depth += 1;
                if depth > 0 {
                    current.push(c);
                }
            }
            '>' => {
                if depth > 0 {
                    current.push(c);
                }
                depth -= 1;
            }
            ',' if depth == 0 => {
                result.push(current.trim().to_string());
                current = String::new();
            }
            _ => {
                if !current.is_empty() || c != ' ' {
                    current.push(c);
                }
            }
        }
    }
    if !current.is_empty() {
        result.push(current.trim().to_string());
    }
    result
}

fn mangle_type(seen: &mut Vec<String>, name: &str, subs: bool, is_template: bool) -> String {
    match name {
        "void" => return "v".to_string(),
        "bool" => return "b".to_string(),
        "char" => return "c".to_string(),
        "short" => return "s".to_string(),
        "int" => return "i".to_string(),
        "long" => return "l".to_string(),
        "long long" => return "x".to_string(),
        "unsigned" => return "j".to_string(),
        "unsigned char" => return "h".to_string(),
        "unsigned short" => return "t".to_string(),
        "unsigned int" => return "j".to_string(),
        "unsigned long" => return "m".to_string(),
        "unsigned long long" => return "y".to_string(),
        "float" => return "f".to_string(),
        "double" => return "d".to_string(),
        "gd::string" => return "Ss".to_string(),
        "std::allocator" => return "Sa".to_string(),
        "cocos2d::ccColor3B" => return mangle_type(seen, "cocos2d::_ccColor3B", subs, is_template),
        _ => {}
    }

    // pointer
    if let Some(stripped) = name.strip_suffix('*') {
        let inner = stripped.trim_end();
        let unsub = mangle_type(&mut seen.clone(), inner, false, false);
        let key = format!("P{}", unsub);
        if !subs {
            return key;
        }
        if let Some(x) = look_for_seen(seen, &key) {
            return x;
        }
        let result = mangle_type(seen, inner, subs, false);
        return subs_seen(seen, format!("P{}", result), subs, &key);
    }

    // reference
    if let Some(stripped) = name.strip_suffix('&') {
        let inner = stripped.trim_end();
        let unsub = mangle_type(&mut seen.clone(), inner, false, false);
        let key = format!("R{}", unsub);
        if !subs {
            return key;
        }
        if let Some(x) = look_for_seen(seen, &key) {
            return x;
        }
        let result = mangle_type(seen, inner, subs, false);
        return subs_seen(seen, format!("R{}", result), subs, &key);
    }

    // trailing const
    if name.ends_with("const") && name.len() > 5 {
        let inner = name[..name.len() - 5].trim_end();
        let unsub = mangle_type(&mut seen.clone(), inner, false, false);
        let key = format!("K{}", unsub);
        if !subs {
            return key;
        }
        if let Some(x) = look_for_seen(seen, &key) {
            return x;
        }
        let result = mangle_type(seen, inner, subs, false);
        return subs_seen(seen, format!("K{}", result), subs, &key);
    }

    // leading const
    if let Some(inner) = name.strip_prefix("const ") {
        let unsub = mangle_type(&mut seen.clone(), inner, false, false);
        let key = format!("K{}", unsub);
        if !subs {
            return key;
        }
        if let Some(x) = look_for_seen(seen, &key) {
            return x;
        }
        let result = mangle_type(seen, inner, subs, false);
        return subs_seen(seen, format!("K{}", result), subs, &key);
    }

    // template
    if let Some(lt) = name.find('<') {
        let gt = name.rfind('>').unwrap_or(name.len() - 1);
        let base = &name[..lt];
        let inner = &name[lt + 1..gt];
        let parts = split_template_recursive(inner);

        let unsub = handle_template(&mut seen.clone(), base, &parts, false);
        if !subs {
            return unsub;
        }
        if let Some(x) = look_for_seen(seen, &unsub) {
            return x;
        }
        let result = handle_template(seen, base, &parts, subs);
        return subs_seen(seen, result, subs, &unsub);
    }

    // qualified name
    if name.contains("::") {
        let mut result = String::new();
        let mut substituted = String::new();
        let mut remaining = name;
        loop {
            let (part_str, rest) = match remaining.find("::") {
                Some(i) => (&remaining[..i], &remaining[i + 2..]),
                None => (remaining, ""),
            };
            let part = format!("{}{}", part_str.len(), part_str);
            if part_str == "gd" || part_str == "std" {
                substituted = "St".to_string();
            } else if !subs {
                substituted.push_str(&part);
            } else {
                let candidate = format!("{}{}", result, part);
                if let Some(x) = look_for_seen(seen, &candidate) {
                    substituted = x;
                } else {
                    let prev_sub = substituted.clone();
                    substituted =
                        subs_seen(seen, format!("{}{}", prev_sub, part), subs, &candidate);
                }
            }
            result.push_str(&part);
            if rest.is_empty() {
                break;
            }
            remaining = rest;
        }
        if substituted.len() == 3 && substituted.starts_with('S') {
            return substituted;
        }
        if is_template {
            return substituted;
        }
        return format!("N{}E", substituted);
    }

    // name
    let m = mangle_ident(name, true);
    subs_seen(seen, m.clone(), subs, &m)
}

fn handle_template(seen: &mut Vec<String>, base: &str, parts: &[String], subs: bool) -> String {
    let outer = mangle_type(seen, base, subs, true);
    let mut result = String::new();
    for part in parts {
        result.push_str(&mangle_type(seen, part, subs, true));
    }

    match base {
        "gd::map" | "std::map" => {
            result.push_str(&mangle_type(
                seen,
                &format!("std::less<{}>", parts[0]),
                subs,
                true,
            ));
            result.push_str(&mangle_type(
                seen,
                &format!(
                    "std::allocator<std::pair<const {}, {}>>",
                    parts[0], parts[1]
                ),
                subs,
                true,
            ));
        }
        "gd::vector" | "std::vector" => {
            result.push_str(&mangle_type(
                seen,
                &format!("std::allocator<{}>", parts[0]),
                subs,
                true,
            ));
        }
        "gd::set" | "std::set" => {
            result.push_str(&mangle_type(
                seen,
                &format!("std::less<{}>", parts[0]),
                subs,
                true,
            ));
            result.push_str(&mangle_type(
                seen,
                &format!("std::allocator<{}>", parts[0]),
                subs,
                true,
            ));
        }
        "gd::unordered_map" | "std::unordered_map" => {
            result.push_str(&mangle_type(
                seen,
                &format!("std::hash<{}>", parts[0]),
                subs,
                true,
            ));
            result.push_str(&mangle_type(
                seen,
                &format!("std::equal_to<{}>", parts[0]),
                subs,
                true,
            ));
            result.push_str(&mangle_type(
                seen,
                &format!(
                    "std::allocator<std::pair<const {}, {}>>",
                    parts[0], parts[1]
                ),
                subs,
                true,
            ));
        }
        "gd::unordered_set" | "std::unordered_set" => {
            result.push_str(&mangle_type(
                seen,
                &format!("std::hash<{}>", parts[0]),
                subs,
                true,
            ));
            result.push_str(&mangle_type(
                seen,
                &format!("std::equal_to<{}>", parts[0]),
                subs,
                true,
            ));
            result.push_str(&mangle_type(
                seen,
                &format!("std::allocator<{}>", parts[0]),
                subs,
                true,
            ));
        }
        _ => {}
    }

    format!("{}I{}E", outer, result)
}

pub fn generate_android_symbol(class_name: &str, func: &FunctionBindField) -> String {
    let decl = &func.prototype;

    let mut mangled = match decl.fn_type {
        FunctionType::Constructor => {
            format!("_ZN{}C2E", mangle_ident(class_name, false))
        }
        FunctionType::Destructor => {
            format!("_ZN{}D2E", mangle_ident(class_name, false))
        }
        FunctionType::Normal => {
            let qualified = format!("{}::{}", class_name, decl.name);
            format!("_Z{}", mangle_ident(&qualified, true))
        }
    };

    if decl.args.is_empty() {
        mangled.push('v');
    } else {
        let mut seen = Vec::new();
        // first part of class name (before ::) is S_
        let first_part = class_name.split("::").next().unwrap_or(class_name);
        seen.push(mangle_ident(first_part, true));

        for arg in &decl.args {
            mangled.push_str(&mangle_type(&mut seen, &arg.ty.name, true, false));
        }
    }

    mangled
}
