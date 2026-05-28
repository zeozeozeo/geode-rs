use broma_rs::{AccessModifier, FunctionBindField, FunctionType};

fn mangle_ident(s: &str) -> String {
    if s.contains("::") {
        let mut result = String::new();
        let mut parts: Vec<&str> = s.split("::").collect();
        parts.reverse();
        for part in parts {
            result.push_str(part);
            result.push('@');
        }
        result
    } else {
        format!("{s}@")
    }
}

fn access_token(proto: &broma_rs::MemberFunctionProto) -> char {
    if proto.is_virtual {
        match proto.access {
            AccessModifier::Private => 'E',
            AccessModifier::Protected => 'M',
            AccessModifier::Public => 'U',
        }
    } else {
        match proto.access {
            AccessModifier::Private => 'A',
            AccessModifier::Protected => 'I',
            AccessModifier::Public => 'Q',
        }
    }
}

fn split_qualified(name: &str) -> (&str, Option<&str>) {
    if let Some(index) = name.rfind("::") {
        (&name[index + 2..], Some(&name[..index]))
    } else {
        (name, None)
    }
}

fn strip_const(mut ty: &str) -> (&str, bool) {
    let mut is_const = false;
    loop {
        if let Some(rest) = ty.strip_prefix("const ") {
            ty = rest.trim();
            is_const = true;
            continue;
        }
        if let Some(rest) = ty.strip_suffix(" const") {
            ty = rest.trim();
            is_const = true;
            continue;
        }
        break;
    }
    (ty, is_const)
}

fn primitive_code(ty: &str) -> Option<&'static str> {
    match ty {
        "void" => Some("X"),
        "bool" => Some("_N"),
        "char" => Some("D"),
        "unsigned char" => Some("E"),
        "short" => Some("F"),
        "unsigned short" => Some("G"),
        "int" => Some("H"),
        "unsigned int" => Some("I"),
        "long" => Some("J"),
        "unsigned long" => Some("K"),
        "float" => Some("M"),
        "double" => Some("N"),
        _ => None,
    }
}

fn encode_class_path(current_class: &str, ty: &str) -> String {
    let (current_short, current_namespace) = split_qualified(current_class);
    let (short, namespace) = split_qualified(ty);

    if namespace == current_namespace {
        if short == current_short {
            "12@".to_string()
        } else {
            format!("{short}@2@")
        }
    } else {
        format!("{}@", mangle_ident(ty))
    }
}

fn encode_record_path(current_class: &str, ty: &str) -> String {
    let (short, namespace) = split_qualified(ty);
    let record_short = if short.starts_with("cc") {
        format!("_{short}")
    } else {
        short.to_string()
    };
    let record_ty = if let Some(namespace) = namespace {
        format!("{namespace}::{record_short}")
    } else {
        record_short
    };

    encode_class_path(current_class, &record_ty)
}

fn record_kind(ty: &str) -> char {
    let (short, _) = split_qualified(ty);
    if short.starts_with("cc") || short.starts_with('_') {
        'U'
    } else {
        'V'
    }
}

fn is_record_type(ty: &str) -> bool {
    let (short, _) = split_qualified(ty);
    short.starts_with("CC")
        || short.starts_with("cc")
        || short.starts_with("_cc")
        || short.starts_with("tCC")
        || short.starts_with("sCC")
        || matches!(short, "HSV" | "RGBA")
}

fn encode_record_value_type(current_class: &str, ty: &str) -> String {
    let path = encode_record_path(current_class, ty);
    format!("{}{path}", record_kind(ty))
}

fn encode_enum_type(current_class: &str, ty: &str) -> String {
    let (current_namespace_short, current_namespace) = split_qualified(current_class);
    let _ = current_namespace_short;
    let (short, namespace) = split_qualified(ty);

    if namespace == current_namespace {
        format!("W4{short}@2@")
    } else {
        format!("W4{}@", mangle_ident(ty))
    }
}

fn encode_pointer_type(current_class: &str, ty: &str) -> Option<String> {
    let inner = ty.strip_suffix('*')?.trim();
    let (inner, is_const) = strip_const(inner);

    if let Some(code) = primitive_code(inner) {
        let cv = if is_const { 'B' } else { 'A' };
        return Some(format!("PE{cv}{code}"));
    }

    let path = encode_record_path(current_class, inner);
    let cv = if is_const { 'B' } else { 'A' };
    Some(format!("PE{cv}{}{path}", record_kind(inner)))
}

fn encode_reference_type(current_class: &str, ty: &str) -> Option<String> {
    let inner = ty.strip_suffix('&')?.trim();
    let (inner, is_const) = strip_const(inner);

    if let Some(code) = primitive_code(inner) {
        let cv = if is_const { 'B' } else { 'A' };
        return Some(format!("AE{cv}{code}"));
    }

    let path = encode_record_path(current_class, inner);
    let cv = if is_const { 'B' } else { 'A' };
    Some(format!("AE{cv}{}{path}", record_kind(inner)))
}

fn encode_value_type(current_class: &str, ty: &str, allow_enum: bool) -> Option<String> {
    let (ty, is_const) = strip_const(ty);
    if is_const {
        return None;
    }

    if let Some(pointer) = encode_pointer_type(current_class, ty) {
        return Some(pointer);
    }

    if let Some(reference) = encode_reference_type(current_class, ty) {
        return Some(reference);
    }

    if let Some(code) = primitive_code(ty) {
        return Some(code.to_string());
    }

    if is_record_type(ty) {
        return Some(encode_record_value_type(current_class, ty));
    }

    if allow_enum {
        return Some(encode_enum_type(current_class, ty));
    }

    None
}

fn encode_arg_type(current_class: &str, ty: &str, seen: &mut Vec<String>) -> Option<String> {
    let encoded = encode_value_type(current_class, ty, true)?;
    if encoded.len() > 1
        && let Some(index) = seen.iter().position(|seen_ty| seen_ty == &encoded)
        && index < 10
    {
        return Some(index.to_string());
    }
    seen.push(encoded.clone());
    Some(encoded)
}

fn encode_return_type(current_class: &str, ty: &str) -> Option<String> {
    let encoded = encode_value_type(current_class, ty, false)?;

    let (ty, _) = strip_const(ty);
    if primitive_code(ty).is_none()
        && encode_pointer_type(current_class, ty).is_none()
        && encode_reference_type(current_class, ty).is_none()
    {
        return Some(format!("?A{encoded}"));
    }

    Some(encoded)
}

pub fn generate_windows_symbol(class_name: &str, func: &FunctionBindField) -> Option<String> {
    let decl = &func.prototype;
    let access = access_token(decl);

    let mut symbol = match decl.fn_type {
        FunctionType::Constructor => format!("??0{}@{access}EAA", mangle_ident(class_name)),
        FunctionType::Destructor => format!("??1{}@{access}EAA", mangle_ident(class_name)),
        FunctionType::Normal => {
            if decl.is_static {
                format!("?{}@{}@SA", decl.name, mangle_ident(class_name))
            } else {
                let constness = if decl.is_const { 'B' } else { 'A' };
                format!(
                    "?{}@{}@{access}E{constness}A",
                    decl.name,
                    mangle_ident(class_name)
                )
            }
        }
    };

    if let FunctionType::Normal = decl.fn_type {
        symbol.push_str(&encode_return_type(class_name, &decl.ret.name)?);
    } else {
        symbol.push('@');
    }

    if decl.args.is_empty() {
        symbol.push_str("XZ");
        return Some(symbol);
    }

    let mut seen = Vec::new();
    for arg in &decl.args {
        symbol.push_str(&encode_arg_type(class_name, &arg.ty.name, &mut seen)?);
    }

    symbol.push_str("@Z");
    Some(symbol)
}
