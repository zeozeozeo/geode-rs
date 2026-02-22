use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct Platform: u32 {
        const None = 0;
        const Windows = 1 << 0;
        const Mac = (1 << 1) | (1 << 2);
        const MacIntel = 1 << 1;
        const MacArm = 1 << 2;
        const IOS = 1 << 3;
        const Android = (1 << 4) | (1 << 5);
        const Android32 = 1 << 4;
        const Android64 = 1 << 5;
        const All = Self::Windows.bits() | Self::IOS.bits() | Self::Android.bits() | Self::Mac.bits();
    }
}

impl Platform {
    pub fn new_from_str(s: &str) -> Option<Platform> {
        match s {
            "win" | "windows" => Some(Platform::Windows),
            "mac" => Some(Platform::Mac),
            "imac" => Some(Platform::MacIntel),
            "m1" => Some(Platform::MacArm),
            "ios" => Some(Platform::IOS),
            "android" => Some(Platform::Android),
            "android32" => Some(Platform::Android32),
            "android64" => Some(Platform::Android64),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PlatformNumber {
    pub win: isize,
    pub imac: isize,
    pub m1: isize,
    pub ios: isize,
    pub android32: isize,
    pub android64: isize,
}

impl PlatformNumber {
    pub const UNSPECIFIED: isize = -1;
    pub const INLINE: isize = -2;
    pub const DEFAULT: isize = -3;

    pub fn new() -> Self {
        Self {
            win: Self::UNSPECIFIED,
            imac: Self::UNSPECIFIED,
            m1: Self::UNSPECIFIED,
            ios: Self::UNSPECIFIED,
            android32: Self::UNSPECIFIED,
            android64: Self::UNSPECIFIED,
        }
    }

    pub fn set_for_platform(&mut self, platform: Platform, value: isize) {
        if platform.contains(Platform::Windows) {
            self.win = value;
        }
        if platform.contains(Platform::MacIntel) {
            self.imac = value;
        }
        if platform.contains(Platform::MacArm) {
            self.m1 = value;
        }
        if platform.contains(Platform::IOS) {
            self.ios = value;
        }
        if platform.contains(Platform::Android32) {
            self.android32 = value;
        }
        if platform.contains(Platform::Android64) {
            self.android64 = value;
        }
    }

    pub fn normalize(&mut self, has_inline: bool) {
        for addr in [
            &mut self.win,
            &mut self.imac,
            &mut self.m1,
            &mut self.ios,
            &mut self.android32,
            &mut self.android64,
        ] {
            if *addr == Self::UNSPECIFIED && has_inline {
                *addr = Self::INLINE;
            } else if *addr == Self::DEFAULT {
                *addr = Self::UNSPECIFIED;
            }
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Type {
    pub is_struct: bool,
    pub name: String,
}

impl Type {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            is_struct: false,
            name: name.into(),
        }
    }

    pub fn with_struct(mut self, is_struct: bool) -> Self {
        self.is_struct = is_struct;
        self
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Attributes {
    pub docs: String,
    pub links: Platform,
    pub missing: Platform,
    pub depends: Vec<String>,
    pub since: String,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Arg {
    pub ty: Type,
    pub name: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FunctionProto {
    pub attributes: Attributes,
    pub ret: Type,
    pub args: Vec<Arg>,
    pub name: String,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FunctionType {
    #[default]
    Normal,
    Constructor,
    Destructor,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AccessModifier {
    #[default]
    Public,
    Protected,
    Private,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MemberFunctionProto {
    pub attributes: Attributes,
    pub ret: Type,
    pub args: Vec<Arg>,
    pub name: String,
    pub fn_type: FunctionType,
    pub access: AccessModifier,
    pub is_const: bool,
    pub is_virtual: bool,
    pub is_callback: bool,
    pub is_static: bool,
}

impl MemberFunctionProto {
    pub fn signature_matches(&self, other: &MemberFunctionProto) -> bool {
        if self.name != other.name || self.args.len() != other.args.len() {
            return false;
        }
        self.args.iter().zip(&other.args).all(|(a, b)| a.ty == b.ty)
            && self.is_const == other.is_const
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MemberField {
    pub platform: Platform,
    pub name: String,
    pub ty: Type,
    pub count: usize,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PadField {
    pub amount: PlatformNumber,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FunctionBindField {
    pub prototype: MemberFunctionProto,
    pub binds: PlatformNumber,
    pub inner: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct InlineField {
    pub inner: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum FieldInner {
    Inline(InlineField),
    FunctionBind(FunctionBindField),
    Pad(PadField),
    Member(MemberField),
}

impl Default for FieldInner {
    fn default() -> Self {
        FieldInner::Member(MemberField::default())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Field {
    pub field_id: usize,
    pub parent: String,
    pub inner: FieldInner,
}

impl Field {
    pub fn as_inline(&self) -> Option<&InlineField> {
        match &self.inner {
            FieldInner::Inline(f) => Some(f),
            _ => None,
        }
    }

    pub fn as_function_bind(&self) -> Option<&FunctionBindField> {
        match &self.inner {
            FieldInner::FunctionBind(f) => Some(f),
            _ => None,
        }
    }

    pub fn as_pad(&self) -> Option<&PadField> {
        match &self.inner {
            FieldInner::Pad(f) => Some(f),
            _ => None,
        }
    }

    pub fn as_member(&self) -> Option<&MemberField> {
        match &self.inner {
            FieldInner::Member(f) => Some(f),
            _ => None,
        }
    }

    pub fn as_function_bind_mut(&mut self) -> Option<&mut FunctionBindField> {
        match &mut self.inner {
            FieldInner::FunctionBind(f) => Some(f),
            _ => None,
        }
    }

    pub fn as_member_mut(&mut self) -> Option<&mut MemberField> {
        match &mut self.inner {
            FieldInner::Member(f) => Some(f),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Class {
    pub attributes: Attributes,
    pub name: String,
    pub superclasses: Vec<String>,
    pub fields: Vec<Field>,
}

impl Class {
    pub fn find_field(&self, name: &str) -> Option<&Field> {
        self.fields.iter().find(|f| {
            if let Some(member) = f.as_member() {
                member.name == name
            } else if let Some(bind) = f.as_function_bind() {
                bind.prototype.name == name
            } else {
                false
            }
        })
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Function {
    pub prototype: FunctionProto,
    pub binds: PlatformNumber,
    pub inner: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Header {
    pub name: String,
    pub platform: Platform,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Root {
    pub headers: Vec<Header>,
    pub classes: Vec<Class>,
    pub functions: Vec<Function>,
}

impl Root {
    pub fn find_class(&self, name: &str) -> Option<&Class> {
        self.classes.iter().find(|c| c.name == name)
    }

    pub fn find_class_mut(&mut self, name: &str) -> Option<&mut Class> {
        self.classes.iter_mut().find(|c| c.name == name)
    }
}
