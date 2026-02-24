#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CallingConvention {
    #[default]
    Default = 0,
    Cdecl = 1,
    Thiscall = 2,
    Fastcall = 3,
    Optcall = 4,
    Membercall = 5,
    Stdcall = 6,
}

impl CallingConvention {
    pub fn for_member_function(is_static: bool) -> Self {
        if is_static {
            Self::Default
        } else {
            #[cfg(target_os = "windows")]
            {
                #[cfg(target_arch = "x86")]
                {
                    Self::Thiscall
                }
                #[cfg(not(target_arch = "x86"))]
                {
                    Self::Default
                }
            }
            #[cfg(not(target_os = "windows"))]
            {
                Self::Default
            }
        }
    }
}

impl From<CallingConvention> for crate::tulip::TulipConvention {
    fn from(val: CallingConvention) -> Self {
        match val {
            CallingConvention::Default => crate::tulip::TulipConvention::Default,
            CallingConvention::Cdecl => crate::tulip::TulipConvention::Cdecl,
            CallingConvention::Thiscall => crate::tulip::TulipConvention::Thiscall,
            CallingConvention::Fastcall => crate::tulip::TulipConvention::Fastcall,
            CallingConvention::Optcall => crate::tulip::TulipConvention::Optcall,
            CallingConvention::Membercall => crate::tulip::TulipConvention::Membercall,
            CallingConvention::Stdcall => crate::tulip::TulipConvention::Stdcall,
        }
    }
}
