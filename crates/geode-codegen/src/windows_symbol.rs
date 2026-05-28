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

#[cfg(test)]
mod tests {
    use super::*;
    use broma_rs::{Arg, FunctionType, MemberFunctionProto, Type};

    fn arg(name: &str, ty: &str) -> Arg {
        Arg {
            name: name.into(),
            ty: Type::new(ty),
        }
    }

    #[test]
    fn mangles_noarg_instance_method() {
        let func = FunctionBindField {
            prototype: MemberFunctionProto {
                name: "drawScene".into(),
                ret: Type::new("void"),
                ..Default::default()
            },
            ..Default::default()
        };

        assert_eq!(
            generate_windows_symbol("cocos2d::CCDirector", &func).as_deref(),
            Some("?drawScene@CCDirector@cocos2d@@QEAAXXZ")
        );
    }

    #[test]
    fn mangles_constructor() {
        let func = FunctionBindField {
            prototype: MemberFunctionProto {
                name: "CCLayerColor".into(),
                ret: Type::new("void"),
                fn_type: FunctionType::Constructor,
                ..Default::default()
            },
            ..Default::default()
        };

        assert_eq!(
            generate_windows_symbol("cocos2d::CCLayerColor", &func).as_deref(),
            Some("??0CCLayerColor@cocos2d@@QEAA@XZ")
        );
    }

    #[test]
    fn mangles_virtual_destructor() {
        let func = FunctionBindField {
            prototype: MemberFunctionProto {
                name: "~CCLayerColor".into(),
                ret: Type::new("void"),
                fn_type: FunctionType::Destructor,
                is_virtual: true,
                ..Default::default()
            },
            ..Default::default()
        };

        assert_eq!(
            generate_windows_symbol("cocos2d::CCLayerColor", &func).as_deref(),
            Some("??1CCLayerColor@cocos2d@@UEAA@XZ")
        );
    }

    #[test]
    fn mangles_static_function_with_cocos_struct_reference() {
        let func = FunctionBindField {
            prototype: MemberFunctionProto {
                name: "create".into(),
                ret: Type::new("cocos2d::CCLayerColor*"),
                is_static: true,
                args: vec![
                    arg("color", "cocos2d::ccColor4B const&"),
                    arg("width", "float"),
                    arg("height", "float"),
                ],
                ..Default::default()
            },
            ..Default::default()
        };

        assert_eq!(
            generate_windows_symbol("cocos2d::CCLayerColor", &func).as_deref(),
            Some("?create@CCLayerColor@cocos2d@@SAPEAV12@AEBU_ccColor4B@2@MM@Z")
        );
    }

    #[test]
    fn mangles_repeated_pointer_args_with_back_reference() {
        let func = FunctionBindField {
            prototype: MemberFunctionProto {
                name: "create".into(),
                ret: Type::new("cocos2d::CCLabelBMFont*"),
                is_static: true,
                args: vec![arg("str", "char const*"), arg("font", "char const*")],
                ..Default::default()
            },
            ..Default::default()
        };

        assert_eq!(
            generate_windows_symbol("cocos2d::CCLabelBMFont", &func).as_deref(),
            Some("?create@CCLabelBMFont@cocos2d@@SAPEAV12@PEBD0@Z")
        );
    }

    #[test]
    fn mangles_dispatch_insert_text() {
        let func = FunctionBindField {
            prototype: MemberFunctionProto {
                name: "dispatchInsertText".into(),
                ret: Type::new("void"),
                args: vec![
                    arg("", "char const*"),
                    arg("", "int"),
                    arg("", "cocos2d::enumKeyCodes"),
                ],
                ..Default::default()
            },
            ..Default::default()
        };

        assert_eq!(
            generate_windows_symbol("cocos2d::CCIMEDispatcher", &func).as_deref(),
            Some("?dispatchInsertText@CCIMEDispatcher@cocos2d@@QEAAXPEBDHW4enumKeyCodes@2@@Z")
        );
    }

    #[test]
    fn mangles_dispatch_keyboard_msg() {
        let func = FunctionBindField {
            prototype: MemberFunctionProto {
                name: "dispatchKeyboardMSG".into(),
                ret: Type::new("bool"),
                args: vec![
                    arg("", "cocos2d::enumKeyCodes"),
                    arg("", "bool"),
                    arg("", "bool"),
                    arg("", "double"),
                ],
                ..Default::default()
            },
            ..Default::default()
        };

        assert_eq!(
            generate_windows_symbol("cocos2d::CCKeyboardDispatcher", &func).as_deref(),
            Some(
                "?dispatchKeyboardMSG@CCKeyboardDispatcher@cocos2d@@QEAA_NW4enumKeyCodes@2@_N1N@Z"
            )
        );
    }

    #[test]
    fn mangles_dispatch_scroll_msg() {
        let func = FunctionBindField {
            prototype: MemberFunctionProto {
                name: "dispatchScrollMSG".into(),
                ret: Type::new("bool"),
                args: vec![arg("", "float"), arg("", "float")],
                ..Default::default()
            },
            ..Default::default()
        };

        assert_eq!(
            generate_windows_symbol("cocos2d::CCMouseDispatcher", &func).as_deref(),
            Some("?dispatchScrollMSG@CCMouseDispatcher@cocos2d@@QEAA_NMM@Z")
        );
    }

    #[test]
    fn mangles_touches() {
        let func = FunctionBindField {
            prototype: MemberFunctionProto {
                name: "touches".into(),
                ret: Type::new("void"),
                args: vec![
                    arg("", "cocos2d::CCSet*"),
                    arg("", "cocos2d::CCEvent*"),
                    arg("", "unsigned int"),
                ],
                ..Default::default()
            },
            ..Default::default()
        };

        assert_eq!(
            generate_windows_symbol("cocos2d::CCTouchDispatcher", &func).as_deref(),
            Some("?touches@CCTouchDispatcher@cocos2d@@QEAAXPEAVCCSet@2@PEAVCCEvent@2@I@Z")
        );
    }

    #[test]
    fn mangles_same_class_pointer_return() {
        let func = FunctionBindField {
            prototype: MemberFunctionProto {
                name: "sharedDirector".into(),
                ret: Type::new("cocos2d::CCDirector*"),
                is_static: true,
                ..Default::default()
            },
            ..Default::default()
        };

        assert_eq!(
            generate_windows_symbol("cocos2d::CCDirector", &func).as_deref(),
            Some("?sharedDirector@CCDirector@cocos2d@@SAPEAV12@XZ")
        );
    }

    #[test]
    fn mangles_record_return() {
        let func = FunctionBindField {
            prototype: MemberFunctionProto {
                name: "getWinSize".into(),
                ret: Type::new("cocos2d::CCSize"),
                ..Default::default()
            },
            ..Default::default()
        };

        assert_eq!(
            generate_windows_symbol("cocos2d::CCDirector", &func).as_deref(),
            Some("?getWinSize@CCDirector@cocos2d@@QEAA?AVCCSize@2@XZ")
        );
    }
}
