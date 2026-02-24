extern crate alloc;

extern crate wgpu_shader_types as wst;
extern crate wgpu_types as wgt;

pub mod out;
#[cfg(feature = "naga-dep")]
pub mod parse;
#[cfg(feature = "naga-dep")]
pub mod validation;

pub use wgpu_shader_types::ShaderStage;

// Re-export specific naga IR types that wgpu-core's shader validation system
// needs, without re-exporting the entire naga crate as a module.
// These are only available when the naga-dep feature is enabled (i.e., when at
// least one shader input format such as wgsl-in, glsl-in, or spv-in is active).
#[cfg(feature = "naga-dep")]
pub use naga::{
    Arena, UniqueArena, Handle,
    ImageDimension, ImageClass,
    AddressSpace, StorageAccess,
    VectorSize, Scalar, ScalarKind,
    Interpolation, Sampling,
    WithSpan,
    Module,
    Type, TypeInner,
    Binding,
};
#[cfg(feature = "naga-dep")]
pub use naga::valid::{ModuleInfo, ValidationFlags, ValidationError};
#[cfg(feature = "naga-dep")]
pub use naga::error::ShaderError;

/// Re-export of the naga WGSL frontend module, for use in shader compilation.
#[cfg(feature = "wgsl-in")]
pub use naga::front::wgsl;

/// Re-export of the naga GLSL frontend module, for use in shader compilation.
#[cfg(feature = "glsl-in")]
pub use naga::front::glsl;

/// Re-export of the naga SPIR-V frontend module, for use in shader compilation.
#[cfg(feature = "spv-in")]
pub use naga::front::spv;

use alloc::borrow::Cow;
use core::fmt;

#[derive(Debug, Clone)]
pub struct DebugSource {
    pub file_name: Cow<'static, str>,
    pub source_code: Cow<'static, str>,
}

/// Naga shader module.
#[derive(Default)]
pub struct NagaShader {
    /// Shader module IR.
    #[cfg(feature = "naga-dep")]
    pub(crate) module: Cow<'static, naga::Module>,
    /// Analysis information of the module.
    #[cfg(feature = "naga-dep")]
    pub(crate) info: naga::valid::ModuleInfo,
    /// Source codes for debug
    #[cfg(feature = "naga-dep")]
    pub(crate) debug_source: Option<DebugSource>,
    /// Private field to keep this non-constructible
    #[cfg(not(feature = "naga-dep"))]
    _p: (),
}

impl NagaShader {
    #[cfg(feature = "naga-dep")]
    pub fn from_module(
        module: Cow<'static, naga::Module>,
        info: naga::valid::ModuleInfo,
        debug_source: Option<DebugSource>,
    ) -> Self {
        Self {
            module,
            info,
            debug_source,
        }
    }

    pub fn has_overrides(&self) -> bool {
        #[cfg(feature = "naga-dep")]
        {
            !self.module.overrides.is_empty()
        }
        #[cfg(not(feature = "naga-dep"))]
        {
            unreachable!()
        }
    }
}

// Custom implementation avoids the need to generate Debug impl code
// for the whole Naga module and info.
impl fmt::Debug for NagaShader {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Naga shader")
    }
}
