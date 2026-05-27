use super::{LoaderResult, raw, stl_path_to_path_buf, stl_string_to_string};
use crate::stl::{StlOptional, StlPath, StlString, StlVector, Variant3};
use std::ffi::c_void;
use std::fmt;
use std::mem::MaybeUninit;
use std::path::{Path, PathBuf};

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum VersionCompare {
    LessEq = 0,
    #[default]
    Exact = 1,
    MoreEq = 2,
    Less = 3,
    More = 4,
    Any = 5,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum VersionCompareResult {
    TooOld = 0,
    #[default]
    Match = 1,
    TooNew = 2,
    MajorMismatch = 3,
    GenericMismatch = 4,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum VersionTagType {
    #[default]
    Alpha = 0,
    Beta = 1,
    Prerelease = 2,
}

#[repr(C)]
#[derive(Clone, Default)]
pub struct VersionTag {
    pub value: VersionTagType,
    pub number: StlOptional<usize>,
}

#[repr(C)]
#[derive(Clone, Default)]
pub struct VersionInfo {
    pub major: usize,
    pub minor: usize,
    pub patch: usize,
    pub tag: StlOptional<VersionTag>,
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct ComparableVersionInfo {
    pub version: VersionInfo,
    pub compare: VersionCompare,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Severity {
    pub value: i32,
}

impl Severity {
    pub const TRACE: Self = Self { value: -1 };
    pub const DEBUG: Self = Self { value: 0 };
    pub const INFO: Self = Self { value: 1 };
    pub const WARNING: Self = Self { value: 2 };
    pub const ERROR: Self = Self { value: 3 };
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ModRequestedAction {
    #[default]
    None = 0,
    Enable = 1,
    Disable = 2,
    Uninstall = 3,
    UninstallWithSaveData = 4,
    Update = 5,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum MatJsonType {
    #[default]
    Object = 0,
    Array = 1,
    String = 2,
    Number = 3,
    Bool = 4,
    Null = 5,
}

macro_rules! opaque_cpp_owner {
    ($name:ident, $copy_ctor:path, $dtor:path) => {
        #[repr(C)]
        pub struct $name {
            storage: [usize; 1],
        }

        impl Clone for $name {
            fn clone(&self) -> Self {
                let mut out = MaybeUninit::<Self>::uninit();
                unsafe {
                    let _ = $copy_ctor(out.as_mut_ptr(), self);
                    out.assume_init()
                }
            }
        }

        impl Drop for $name {
            fn drop(&mut self) {
                let _ = unsafe { $dtor(self) };
            }
        }

        unsafe impl Send for $name {}
        unsafe impl Sync for $name {}
    };
}

opaque_cpp_owner!(
    MatJsonValue,
    raw::matjson_value_copy_ctor,
    raw::matjson_value_dtor
);
opaque_cpp_owner!(
    ModMetadata,
    raw::mod_metadata_copy_ctor,
    raw::mod_metadata_dtor
);
opaque_cpp_owner!(
    ModMetadataLinks,
    raw::mod_metadata_links_copy_ctor,
    raw::mod_metadata_links_dtor
);
opaque_cpp_owner!(
    ModMetadataDependency,
    raw::mod_metadata_dependency_copy_ctor,
    raw::mod_metadata_dependency_dtor
);
opaque_cpp_owner!(
    ModMetadataIncompatibility,
    raw::mod_metadata_incompatibility_copy_ctor,
    raw::mod_metadata_incompatibility_dtor
);
opaque_cpp_owner!(
    ModMetadataIssuesInfo,
    raw::mod_metadata_issues_info_copy_ctor,
    raw::mod_metadata_issues_info_dtor
);

#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LoadProblemType {
    #[default]
    Unknown = 0,
    InvalidGeodeFile = 1,
    MissingDependencies = 2,
    Outdated = 3,
    HasIncompatibilities = 4,
}

#[repr(C)]
pub struct LoadProblem {
    pub kind: LoadProblemType,
    pub cause: Variant3<StlPath, ModMetadata, *mut c_void>,
    pub message: StlString,
}

impl Clone for LoadProblem {
    fn clone(&self) -> Self {
        let cause = if let Some(path) = self.cause.as_val1() {
            Variant3::new_val1(path.clone())
        } else if let Some(meta) = self.cause.as_val2() {
            Variant3::new_val2(meta.clone())
        } else {
            Variant3::new_val3(*self.cause.as_val3().unwrap_or(&std::ptr::null_mut()))
        };

        Self {
            kind: self.kind,
            cause,
            message: self.message.to_string().as_str().into(),
        }
    }
}

impl fmt::Debug for LoadProblem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LoadProblem")
            .field("kind", &self.kind)
            .field("cause", &self.cause())
            .field("message", &self.message())
            .finish()
    }
}

#[derive(Clone)]
pub enum LoadProblemCause {
    Path(PathBuf),
    Metadata(ModMetadata),
    Mod(*mut c_void),
}

impl LoadProblem {
    pub fn message(&self) -> String {
        stl_string_to_string(&self.message)
    }

    pub fn cause(&self) -> LoadProblemCause {
        if let Some(path) = self.cause.as_val1() {
            LoadProblemCause::Path(stl_path_to_path_buf(path))
        } else if let Some(metadata) = self.cause.as_val2() {
            LoadProblemCause::Metadata(metadata.clone())
        } else {
            LoadProblemCause::Mod(*self.cause.as_val3().unwrap_or(&std::ptr::null_mut()))
        }
    }
}

impl fmt::Debug for VersionTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VersionTag")
            .field("value", &self.value)
            .field("number", &self.number.value())
            .finish()
    }
}

impl fmt::Debug for VersionInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VersionInfo")
            .field("major", &self.major)
            .field("minor", &self.minor)
            .field("patch", &self.patch)
            .field("tag", &self.tag.value())
            .finish()
    }
}

impl fmt::Debug for LoadProblemCause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Path(path) => f.debug_tuple("Path").field(path).finish(),
            Self::Metadata(_) => f.write_str("Metadata(ModMetadata)"),
            Self::Mod(ptr) => f.debug_tuple("Mod").field(ptr).finish(),
        }
    }
}

impl MatJsonValue {
    pub fn dump(&self, indentation: i32) -> LoaderResult<String> {
        let dumped = unsafe { raw::matjson_value_dump(self, indentation) }
            .ok_or_else(|| "missing matjson::Value::dump".to_owned())?;
        Ok(stl_string_to_string(&dumped))
    }

    pub fn dump_compact(&self) -> LoaderResult<String> {
        self.dump(0)
    }

    pub fn value_type(&self) -> MatJsonType {
        unsafe { raw::matjson_value_type(self) }.unwrap_or_default()
    }

    pub fn size(&self) -> usize {
        unsafe { raw::matjson_value_size(self) }.unwrap_or_default()
    }

    pub fn contains(&self, key: &str) -> bool {
        unsafe { raw::matjson_value_contains(self, key.into()) }.unwrap_or(false)
    }

    pub fn as_string(&self) -> LoaderResult<String> {
        let result = unsafe { raw::matjson_value_as_string(self) }
            .ok_or_else(|| "missing matjson::Value::asString".to_owned())?;
        unsafe { result.into_rust() }.map(|value| stl_string_to_string(&value))
    }
}

impl fmt::Debug for MatJsonValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.dump_compact() {
            Ok(value) => f.write_str(&value),
            Err(_) => f.write_str("<matjson::Value>"),
        }
    }
}

impl fmt::Debug for ModMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ModMetadata")
    }
}

impl fmt::Debug for ModMetadataLinks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ModMetadataLinks")
    }
}

impl fmt::Debug for ModMetadataDependency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ModMetadataDependency")
    }
}

impl fmt::Debug for ModMetadataIncompatibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ModMetadataIncompatibility")
    }
}

impl fmt::Debug for ModMetadataIssuesInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ModMetadataIssuesInfo")
    }
}

impl ModMetadataLinks {
    pub fn homepage_url(&self) -> Option<String> {
        unsafe { raw::mod_metadata_links_get_homepage_url(self) }
            .and_then(optional_string_to_option)
    }

    pub fn source_url(&self) -> Option<String> {
        unsafe { raw::mod_metadata_links_get_source_url(self) }.and_then(optional_string_to_option)
    }

    pub fn community_url(&self) -> Option<String> {
        unsafe { raw::mod_metadata_links_get_community_url(self) }
            .and_then(optional_string_to_option)
    }
}

impl ModMetadataIssuesInfo {
    pub fn info(&self) -> String {
        unsafe { raw::mod_metadata_issues_info_get_info(self) }
            .and_then(|value| unsafe { value.as_ref() })
            .map(stl_string_to_string)
            .unwrap_or_default()
    }

    pub fn url(&self) -> Option<String> {
        unsafe { raw::mod_metadata_issues_info_get_url(self) }
            .and_then(|value| unsafe { value.as_ref() })
            .and_then(optional_string_ref_to_option)
    }
}

impl ModMetadataDependency {
    pub fn id(&self) -> String {
        unsafe { raw::mod_metadata_dependency_get_id(self) }
            .and_then(|value| unsafe { value.as_ref() })
            .map(stl_string_to_string)
            .unwrap_or_default()
    }

    pub fn version(&self) -> ComparableVersionInfo {
        unsafe { raw::mod_metadata_dependency_get_version(self) }
            .and_then(|value| unsafe { value.as_ref() })
            .cloned()
            .unwrap_or_default()
    }

    pub fn is_required(&self) -> bool {
        unsafe { raw::mod_metadata_dependency_is_required(self) }.unwrap_or(false)
    }

    pub fn mod_ptr(&self) -> *mut c_void {
        unsafe { raw::mod_metadata_dependency_get_mod(self) }.unwrap_or(std::ptr::null_mut())
    }

    pub fn settings(&self) -> Option<MatJsonValue> {
        unsafe { raw::mod_metadata_dependency_get_settings(self) }
            .and_then(|value| unsafe { value.as_ref() })
            .cloned()
    }

    pub fn is_resolved(&self) -> bool {
        unsafe { raw::mod_metadata_dependency_is_resolved(self) }.unwrap_or(false)
    }
}

impl ModMetadataIncompatibility {
    pub fn id(&self) -> String {
        unsafe { raw::mod_metadata_incompatibility_get_id(self) }
            .and_then(|value| unsafe { value.as_ref() })
            .map(stl_string_to_string)
            .unwrap_or_default()
    }

    pub fn version(&self) -> ComparableVersionInfo {
        unsafe { raw::mod_metadata_incompatibility_get_version(self) }
            .and_then(|value| unsafe { value.as_ref() })
            .cloned()
            .unwrap_or_default()
    }

    pub fn is_breaking(&self) -> bool {
        unsafe { raw::mod_metadata_incompatibility_is_breaking(self) }.unwrap_or(false)
    }

    pub fn mod_ptr(&self) -> *mut c_void {
        unsafe { raw::mod_metadata_incompatibility_get_mod(self) }.unwrap_or(std::ptr::null_mut())
    }

    pub fn is_resolved(&self) -> bool {
        unsafe { raw::mod_metadata_incompatibility_is_resolved(self) }.unwrap_or(false)
    }
}

impl ModMetadata {
    pub fn path(&self) -> Option<PathBuf> {
        unsafe { raw::mod_metadata_get_path(self) }
            .and_then(|value| unsafe { value.as_ref() })
            .map(stl_path_to_path_buf)
    }

    pub fn binary_name(&self) -> String {
        unsafe { raw::mod_metadata_get_binary_name(self) }
            .map(|value| value.to_string_lossy())
            .unwrap_or_default()
    }

    pub fn version(&self) -> Option<VersionInfo> {
        unsafe { raw::mod_metadata_get_version(self) }
    }

    pub fn id(&self) -> String {
        unsafe { raw::mod_metadata_get_id(self) }
            .map(|value| value.to_string_lossy())
            .unwrap_or_default()
    }

    pub fn name(&self) -> String {
        unsafe { raw::mod_metadata_get_name(self) }
            .map(|value| value.to_string_lossy())
            .unwrap_or_default()
    }

    pub fn developers(&self) -> Vec<String> {
        unsafe { raw::mod_metadata_get_developers(self) }
            .and_then(|value| unsafe { value.as_ref() })
            .map(|value| value.iter().map(stl_string_to_string).collect())
            .unwrap_or_default()
    }

    pub fn description(&self) -> Option<String> {
        unsafe { raw::mod_metadata_get_description(self) }
            .and_then(|value| unsafe { value.as_ref() })
            .and_then(optional_string_ref_to_option)
    }

    pub fn details(&self) -> Option<String> {
        unsafe { raw::mod_metadata_get_details(self) }
            .and_then(|value| unsafe { value.as_ref() })
            .and_then(optional_string_ref_to_option)
    }

    pub fn links(&self) -> Option<ModMetadataLinks> {
        unsafe { raw::mod_metadata_get_links(self) }
            .and_then(|value| unsafe { value.as_ref() })
            .cloned()
    }

    pub fn issues(&self) -> Option<ModMetadataIssuesInfo> {
        unsafe { raw::mod_metadata_get_issues(self) }
            .and_then(|value| unsafe { value.as_ref() })
            .and_then(|value| value.value().cloned())
    }

    pub fn dependencies(&self) -> Vec<ModMetadataDependency> {
        unsafe { raw::mod_metadata_get_dependencies(self) }
            .and_then(|value| unsafe { value.as_ref() })
            .map(|value| value.iter().cloned().collect())
            .unwrap_or_default()
    }

    pub fn incompatibilities(&self) -> Vec<ModMetadataIncompatibility> {
        unsafe { raw::mod_metadata_get_incompatibilities(self) }
            .and_then(|value| unsafe { value.as_ref() })
            .map(|value| value.iter().cloned().collect())
            .unwrap_or_default()
    }

    pub fn load_priority(&self) -> i32 {
        unsafe { raw::mod_metadata_get_load_priority(self) }.unwrap_or_default()
    }

    pub fn raw_json(&self) -> Option<MatJsonValue> {
        unsafe { raw::mod_metadata_get_raw_json(self) }
    }

    pub fn to_json(&self) -> Option<MatJsonValue> {
        unsafe { raw::mod_metadata_to_json(self) }
    }

    pub fn create_from_geode_file(path: &Path) -> Option<Self> {
        unsafe { raw::mod_metadata_create_from_geode_file(&super::path_to_stl_path(path)) }
    }

    pub fn create(json: &MatJsonValue) -> Option<Self> {
        unsafe { raw::mod_metadata_create(json) }
    }

    pub fn validate_id(id: &str) -> bool {
        unsafe { raw::mod_metadata_validate_id(id.into()) }.unwrap_or(false)
    }

    pub fn format_developer_display_string(developers: &[String]) -> Option<String> {
        let mut vec = StlVector::new();
        for value in developers {
            vec.push_back(value.as_str().into());
        }
        unsafe { raw::mod_metadata_format_developer_display_string(&vec) }
            .map(|value| stl_string_to_string(&value))
    }
}

fn optional_string_to_option(value: StlOptional<StlString>) -> Option<String> {
    Option::<StlString>::from(value).map(|value| stl_string_to_string(&value))
}

fn optional_string_ref_to_option(value: &StlOptional<StlString>) -> Option<String> {
    value.value().map(stl_string_to_string)
}
