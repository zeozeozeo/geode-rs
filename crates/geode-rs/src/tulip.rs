use crate::stl::{StlSharedPtr, StlVector};
use std::ffi::c_void;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct HookMetadata {
    pub m_priority: i32,
}

impl HookMetadata {
    pub const fn new(priority: i32) -> Self {
        Self {
            m_priority: priority,
        }
    }

    pub const fn default() -> Self {
        Self { m_priority: 0 }
    }
}

impl Default for HookMetadata {
    fn default() -> Self {
        Self::default()
    }
}

#[repr(C)]
pub struct HandlerMetadata {
    pub m_convention: StlSharedPtr<c_void>,
    pub m_abstract: AbstractFunction,
}

impl HandlerMetadata {
    pub fn new(convention: StlSharedPtr<c_void>, abstract_func: AbstractFunction) -> Self {
        Self {
            m_convention: convention,
            m_abstract: abstract_func,
        }
    }

    pub fn with_convention(convention: StlSharedPtr<c_void>) -> Self {
        Self {
            m_convention: convention,
            m_abstract: AbstractFunction::void_return(),
        }
    }
}

#[repr(C)]
pub struct AbstractFunction {
    pub m_return: AbstractType,
    pub m_parameters: StlVector<AbstractType>,
}

impl AbstractFunction {
    pub fn new(return_type: AbstractType, params: StlVector<AbstractType>) -> Self {
        Self {
            m_return: return_type,
            m_parameters: params,
        }
    }

    pub fn void_return() -> Self {
        Self {
            m_return: AbstractType::VOID,
            m_parameters: StlVector::new(),
        }
    }
}

impl Default for AbstractFunction {
    fn default() -> Self {
        Self::void_return()
    }
}

impl Clone for AbstractFunction {
    fn clone(&self) -> Self {
        Self {
            m_return: AbstractType {
                m_size: self.m_return.m_size,
                m_kind: self.m_return.m_kind,
            },
            m_parameters: StlVector::new(),
        }
    }
}

#[repr(C)]
pub struct AbstractType {
    pub m_size: usize,
    pub m_kind: AbstractTypeKind,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AbstractTypeKind {
    Primitive = 0,
    FloatingPoint = 1,
    Other = 2,
}

impl AbstractType {
    pub const VOID: Self = Self {
        m_size: 1,
        m_kind: AbstractTypeKind::Primitive,
    };

    pub const fn new(size: usize, kind: AbstractTypeKind) -> Self {
        Self {
            m_size: size,
            m_kind: kind,
        }
    }

    pub const fn primitive(size: usize) -> Self {
        Self::new(size, AbstractTypeKind::Primitive)
    }

    pub const fn floating_point(size: usize) -> Self {
        Self::new(size, AbstractTypeKind::FloatingPoint)
    }

    pub const fn other(size: usize) -> Self {
        Self::new(size, AbstractTypeKind::Other)
    }
}

impl Default for AbstractType {
    fn default() -> Self {
        Self::VOID
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TulipConvention {
    #[default]
    Default = 0,
    Cdecl = 1,
    Thiscall = 2,
    Fastcall = 3,
    Optcall = 4,
    Membercall = 5,
    Stdcall = 6,
}
