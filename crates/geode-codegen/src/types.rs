use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

thread_local! {
    static KNOWN_CLASSES: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
}

lazy_static::lazy_static! {
    static ref TYPE_ALIASES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("int", "c_int");
        m.insert("unsigned int", "c_uint");
        m.insert("short", "c_short");
        m.insert("unsigned short", "c_ushort");
        m.insert("long", "c_long");
        m.insert("unsigned long", "c_ulong");
        m.insert("long long", "c_longlong");
        m.insert("unsigned long long", "c_ulonglong");
        m.insert("char", "c_char");
        m.insert("signed char", "c_schar");
        m.insert("unsigned char", "c_uchar");
        m.insert("float", "c_float");
        m.insert("double", "c_double");
        m.insert("bool", "bool");
        m.insert("void", "()");
        m.insert("int8_t", "i8");
        m.insert("uint8_t", "u8");
        m.insert("int16_t", "i16");
        m.insert("uint16_t", "u16");
        m.insert("int32_t", "i32");
        m.insert("uint32_t", "u32");
        m.insert("int64_t", "i64");
        m.insert("uint64_t", "u64");
        m.insert("intptr_t", "isize");
        m.insert("uintptr_t", "usize");
        m.insert("size_t", "usize");
        m.insert("ssize_t", "isize");
        m
    };

    static ref COCOS_TYPES: HashSet<&'static str> = {
        let mut s = HashSet::new();
        s.insert("CCPoint");
        s.insert("CCSize");
        s.insert("CCRect");
        s.insert("ccColor3B");
        s.insert("ccColor4B");
        s.insert("ccColor4F");
        s.insert("ccBlendFunc");
        s.insert("ccVertex2F");
        s.insert("ccTex2F");
        s.insert("ccV2F_C4B_T2F");
        s.insert("ccV2F_C4F_T2F");
        s.insert("ccV3F_C4B_T2F");
        s.insert("ccArray");
        s.insert("ccCArray");
        s.insert("ccTexParams");
        s.insert("ccFontDefinition");
        s.insert("ccFontShadow");
        s.insert("ccFontStroke");
        s.insert("CCTextAlignment");
        s.insert("CCVerticalTextAlignment");
        s.insert("ccTouchesMode");
        s.insert("ccDirectorProjection");
        s.insert("GLenum");
        s.insert("GLint");
        s.insert("GLuint");
        s.insert("GLfloat");
        s.insert("GLubyte");
        s.insert("GLushort");
        s.insert("GLsizei");
        s.insert("GLbitfield");
        s.insert("GLboolean");
        s.insert("CCAffineTransform");
        s.insert("_ccArray");
        s.insert("_ccCArray");
        s.insert("_ccColor3B");
        s.insert("_ccColor4B");
        s.insert("_ccColor4F");
        s.insert("_ccVertex2F");
        s.insert("_ccVertex3F");
        s.insert("_ccTex2F");
        s.insert("_ccPointSprite");
        s.insert("_ccQuad2");
        s.insert("_ccQuad3");
        s.insert("_ccV2F_C4B_T2F");
        s.insert("_ccV2F_C4F_T2F");
        s.insert("_ccV3F_C4B_T2F");
        s.insert("_ccV2F_C4B_T2F_Triangle");
        s.insert("_ccV2F_C4B_T2F_Quad");
        s.insert("_ccV2F_C4F_T2F_Quad");
        s.insert("_ccV3F_C4B_T2F_Quad");
        s.insert("_ccV3F_C4B_T2F_Quad");
        s.insert("ccHSVValue");
        s.insert("ccBezierConfig");
        s.insert("ccBMFontDef");
        s.insert("ccBMFontPadding");
        s.insert("ccPVRTexturePixelFormatInfo");
        s.insert("ccPointSprite");
        s.insert("tCCParticle");
        s.insert("sCCParticle");
        s.insert("tCCFontDefHashElement");
        s.insert("tCCKerningHashElement");
        s.insert("CCSetIterator");
        s.insert("GLLogFunction");
        s.insert("GLshort");
        s.insert("GLclampf");
        s.insert("GLvoid");
        s.insert("GLchar");
        s.insert("BYTE");
        s.insert("UINT");
        s.insert("SHORT");
        s.insert("LONG");
        s.insert("WCHAR");
        s.insert("LPCWSTR");
        s.insert("LONGLONG");
        s.insert("WPARAM");
        s.insert("UINT_PTR");
        s.insert("cc_timeval");
        s.insert("ccColor4F");
        s.insert("ccV3F_C4B_T2F_Quad");
        s.insert("CCIMEKeyboardNotificationInfo");
        s.insert("SEL_MenuHandler");
        s.insert("SEL_CallFunc");
        s.insert("ccColor3B");
        s.insert("enumKeyCodes");
        s.insert("LanguageType");
        s.insert("CCObjectType");
        s.insert("ccGLServerState");
        s.insert("ccScriptType");
        s.insert("BorderAlignment");
        s.insert("TextureQuality");
        s.insert("PopTransition");
        s.insert("ccKeypadMSGType");
        s.insert("tOrientation");
        s.insert("tCCMenuState");
        s.insert("CCProgressTimerType");
        s.insert("eImageFormat");
        s.insert("tCCPositionType");
        s.insert("TargetPlatform");
        s.insert("ccTouchSelectorFlag");
        s.insert("ccTouchType");
        s.insert("BOOL");
        s.insert("CCControlEvent");
        s.insert("SEL_CCControlHandler");
        s.insert("CCSortableObject");
        s.insert("CCScale9Sprite");
        s.insert("CCControlColourPicker");
        s.insert("CCHttpRequest");
        s.insert("CCHttpResponse");
        s.insert("HSV");
        s.insert("RGBA");
        s.insert("EditBoxInputFlag");
        s.insert("EditBoxInputMode");
        s.insert("KeyboardReturnType");
        s.insert("CCTableViewVerticalFillOrder");
        s.insert("CCScrollViewDelegate");
        s.insert("CCTableViewDataSource");
        s.insert("CCTableViewCell");
        s.insert("CCMouseDelegate");
        s.insert("CCEvent");
        s.insert("CCTextFieldTTF");
        s.insert("CCIMEKeyboardNotificationInfo");
        s
    };
}

pub fn register_classes(classes: &[String]) {
    KNOWN_CLASSES.with(|known| {
        let mut set = known.borrow_mut();
        for name in classes {
            set.insert(name.clone());
        }
    });
}

fn is_known_class(name: &str) -> bool {
    KNOWN_CLASSES.with(|classes| classes.borrow().contains(name))
}

fn is_cocos_type(name: &str) -> bool {
    COCOS_TYPES.contains(name)
}

#[derive(Debug, Clone, PartialEq)]
pub enum RustType {
    Primitive(String),
    KnownClass(String),
    CocosType(String),
    Opaque(String),
    Vector(Box<RustType>),
    Set(Box<RustType>),
    UnorderedMap(Box<RustType>, Box<RustType>),
    Map(Box<RustType>, Box<RustType>),
    UnorderedSet(Box<RustType>),
    Pointer(Box<RustType>, bool),
    Reference(Box<RustType>, bool),
    FunctionPtr {
        ret: Box<RustType>,
        args: Vec<RustType>,
    },
    Array(Box<RustType>, usize),
    Unknown(String),
}

impl RustType {
    pub fn to_rust_str(&self) -> String {
        match self {
            RustType::Primitive(s) => s.clone(),
            RustType::KnownClass(s) => s.clone(),
            RustType::CocosType(s) => s.clone(),
            RustType::Vector(inner) => format!("GdVector<{}>", inner.to_rust_str()),
            RustType::Set(inner) => format!("GdSet<{}>", inner.to_rust_str()),
            RustType::Map(k, v) => format!("GdMap<{}, {}>", k.to_rust_str(), v.to_rust_str()),
            RustType::UnorderedMap(k, v) => {
                format!("GdUnorderedMap<{}, {}>", k.to_rust_str(), v.to_rust_str())
            }
            RustType::UnorderedSet(inner) => format!("GdUnorderedSet<{}>", inner.to_rust_str()),
            RustType::Opaque(s) => format!("/* {s} (opaque) */ *mut c_void"),
            RustType::Pointer(inner, is_const) => {
                let inner_str = inner.to_rust_str();
                if *is_const {
                    format!("*const {inner_str}")
                } else {
                    format!("*mut {inner_str}")
                }
            }
            RustType::Reference(inner, is_const) => {
                let inner_str = inner.to_rust_str();
                if *is_const {
                    format!("*const {inner_str}")
                } else {
                    format!("*mut {inner_str}")
                }
            }
            RustType::FunctionPtr { ret, args } => {
                let args_str: Vec<String> = args.iter().map(|a| a.to_rust_str()).collect();
                let ret_str = ret.to_rust_str();
                format!(
                    "Option<unsafe extern \"C\" fn({}) -> {}>",
                    args_str.join(", "),
                    ret_str
                )
            }
            RustType::Array(inner, size) => {
                format!("[{}; {}]", inner.to_rust_str(), size)
            }
            RustType::Unknown(s) => format!("/* {s} (unk) */ c_int"), // assume C enum.. TODO: port geode additions
        }
    }
}

fn split_template_args(s: &str) -> Vec<&str> {
    let mut args = Vec::new();
    let mut depth = 0;
    let mut last_start = 0;

    for (i, c) in s.char_indices() {
        match c {
            '<' => depth += 1,
            '>' => depth -= 1,
            ',' if depth == 0 => {
                args.push(s[last_start..i].trim());
                last_start = i + 1;
            }
            _ => {}
        }
    }
    args.push(s[last_start..].trim());
    args
}

pub fn cpp_to_rust_type(cpp_type: &str) -> RustType {
    let trimmed = cpp_type.trim();

    if trimmed.is_empty() {
        return RustType::Unknown(cpp_type.to_string());
    }

    let is_const = trimmed.starts_with("const ");
    let type_str = if is_const {
        trimmed.strip_prefix("const ").unwrap().trim()
    } else {
        trimmed
    };

    if let Some(stripped) = type_str.strip_suffix('*') {
        let inner_type = stripped.trim();
        return RustType::Pointer(Box::new(cpp_to_rust_type(inner_type)), is_const);
    }

    if let Some(stripped) = type_str.strip_suffix('&') {
        let inner_type = stripped.trim();
        return RustType::Reference(Box::new(cpp_to_rust_type(inner_type)), is_const);
    }

    if let Some(alias) = TYPE_ALIASES.get(type_str) {
        return RustType::Primitive(alias.to_string());
    }

    if is_cocos_type(type_str) {
        return RustType::CocosType(type_str.to_string());
    }

    if is_known_class(type_str) {
        return RustType::KnownClass(type_str.to_string());
    }

    if let Some(name) = type_str.strip_prefix("cocos2d::extension::") {
        if is_cocos_type(name) {
            return RustType::CocosType(name.to_string());
        }
        if is_known_class(name) {
            return RustType::KnownClass(name.to_string());
        }
    }

    if let Some(name) = type_str.strip_prefix("cocos2d::") {
        if is_cocos_type(name) {
            return RustType::CocosType(name.to_string());
        }
        if is_known_class(name) {
            return RustType::KnownClass(name.to_string());
        }
        if name.ends_with('*') {
            let inner = name.strip_suffix('*').unwrap().trim();
            return RustType::Pointer(
                Box::new(cpp_to_rust_type(&format!("cocos2d::{inner}"))),
                is_const,
            );
        }
    }

    if let Some(name) = type_str.strip_prefix("gd::") {
        if let Some(inner) = name
            .strip_prefix("vector<")
            .and_then(|s| s.strip_suffix('>'))
        {
            return RustType::Vector(Box::new(cpp_to_rust_type(inner.trim())));
        }

        if let Some(inner) = name.strip_prefix("set<").and_then(|s| s.strip_suffix('>')) {
            return RustType::Set(Box::new(cpp_to_rust_type(inner.trim())));
        }

        if let Some(inner) = name
            .strip_prefix("unordered_set<")
            .and_then(|s| s.strip_suffix('>'))
        {
            return RustType::UnorderedSet(Box::new(cpp_to_rust_type(inner.trim())));
        }

        if let Some(inner) = name
            .strip_prefix("unordered_map<")
            .and_then(|s| s.strip_suffix('>'))
        {
            let args = split_template_args(inner);
            if args.len() >= 2 {
                return RustType::UnorderedMap(
                    Box::new(cpp_to_rust_type(args[0])),
                    Box::new(cpp_to_rust_type(args[1])),
                );
            }
        }

        if let Some(inner) = name.strip_prefix("map<").and_then(|s| s.strip_suffix('>')) {
            let args = split_template_args(inner);
            if args.len() >= 2 {
                return RustType::Map(
                    Box::new(cpp_to_rust_type(args[0])),
                    Box::new(cpp_to_rust_type(args[1])),
                );
            }
        }

        if name == "string" { return RustType::KnownClass("GdString".to_string()) }
    }

    if type_str.starts_with("std::") {
        let rest = type_str.strip_prefix("std::").unwrap();
        if rest.starts_with("string") {
            return RustType::KnownClass("GdString".to_string());
        }
        if let Some(inner) = rest
            .strip_prefix("array<")
            .and_then(|s| s.strip_suffix('>'))
        {
            let args = split_template_args(inner);
            if args.len() == 2 {
                let inner_type = cpp_to_rust_type(args[0].trim());
                if let Ok(size) = args[1].trim().parse::<usize>() {
                    return RustType::Array(Box::new(inner_type), size);
                }
            }
        }
        return RustType::Opaque(type_str.to_string());
    }

    RustType::Unknown(cpp_type.to_string())
}

pub fn generate_types_mod(use_cocos_bindgen: bool) -> String {
    let mut output = String::new();

    output.push_str("#![allow(non_camel_case_types, dead_code)]\n\n");
    output.push_str("use std::ffi::c_void;\n\n");

    output.push_str("pub type c_int = i32;\n");
    output.push_str("pub type c_uint = u32;\n");
    output.push_str("pub type c_short = i16;\n");
    output.push_str("pub type c_ushort = u16;\n");
    output.push_str("pub type c_long = i64;\n");
    output.push_str("pub type c_ulong = u64;\n");
    output.push_str("pub type c_longlong = i64;\n");
    output.push_str("pub type c_ulonglong = u64;\n");
    output.push_str("pub type c_char = i8;\n");
    output.push_str("pub type c_schar = i8;\n");
    output.push_str("pub type c_uchar = u8;\n");
    output.push_str("pub type c_float = f32;\n");
    output.push_str("pub type c_double = f64;\n\n");

    output.push_str(
        r#"#[repr(C)]
pub struct GdString {
    _storage: [u8; 16],
    _size: usize,
    _capacity: usize,
}

#[repr(C)]
pub struct GdVector<T> {
    _first: *mut T,
    _last: *mut T,
    _end: *mut T,
}

#[repr(C)]
pub struct GdSet<T> {
    _head: *mut c_void,
    _size: usize,
    _marker: std::marker::PhantomData<T>,
}

#[repr(C)]
pub struct GdMap<K, V> {
    _head: *mut c_void,
    _size: usize,
    _marker: std::marker::PhantomData<(K, V)>,
}

#[repr(C)]
pub struct GdUnorderedMap<K, V> {
    #[cfg(target_os = "windows")]
    _data: [usize; 8],
    #[cfg(not(target_os = "windows"))]
    _data: [usize; 4],
    _marker: std::marker::PhantomData<(K, V)>,
}

#[repr(C)]
pub struct GdUnorderedSet<T> {
    #[cfg(target_os = "windows")]
    _storage: [u64; 8],
    #[cfg(not(target_os = "windows"))]
    _storage: [usize; 4],
    _marker: std::marker::PhantomData<T>,
}
"#,
    );

    if use_cocos_bindgen {
        output.push_str("pub use super::cocos::*;\n");
    } else {
        output.push_str("#[repr(C)]\n#[derive(Debug, Clone, Copy, Default)]\npub struct CCPoint {\n    pub x: c_float,\n    pub y: c_float,\n}\n\n");
        output.push_str("#[repr(C)]\n#[derive(Debug, Clone, Copy, Default)]\npub struct CCSize {\n    pub width: c_float,\n    pub height: c_float,\n}\n\n");
        output.push_str("#[repr(C)]\n#[derive(Debug, Clone, Copy, Default)]\npub struct CCRect {\n    pub origin: CCPoint,\n    pub size: CCSize,\n}\n\n");
    }

    output
}
