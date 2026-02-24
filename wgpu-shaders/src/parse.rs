//! Shader parsing: source text / binary → naga IR.
//!
//! Each function in this module parses one shader source format and returns a
//! [`naga::Module`].  Error wrapping into the caller's error type is left to
//! the caller so that this module remains independent of `wgpu-core` types.

/// Parse a WGSL shader source string into a [`naga::Module`].
///
/// `capabilities` controls which naga language features the parser will accept.
#[cfg(feature = "wgsl-in")]
pub fn parse_wgsl(
    code: &str,
    capabilities: naga::valid::Capabilities,
) -> Result<naga::Module, naga::error::ShaderError<naga::front::wgsl::ParseError>> {
    let mut options = naga::front::wgsl::Options::new();
    options.capabilities = capabilities;
    let mut frontend = naga::front::wgsl::Frontend::new_with_options(options);
    frontend.parse(code).map_err(|inner| naga::error::ShaderError {
        source: code.to_string(),
        label: None,
        inner: alloc::boxed::Box::new(inner),
    })
}

/// Parse a GLSL shader source string into a [`naga::Module`].
#[cfg(feature = "glsl-in")]
pub fn parse_glsl(
    code: &str,
    options: &naga::front::glsl::Options,
) -> Result<naga::Module, naga::error::ShaderError<naga::front::glsl::ParseErrors>> {
    let mut parser = naga::front::glsl::Frontend::default();
    parser
        .parse(options, code)
        .map_err(|inner| naga::error::ShaderError {
            source: code.to_string(),
            label: None,
            inner: alloc::boxed::Box::new(inner),
        })
}

/// Parse a SPIR-V binary word stream into a [`naga::Module`].
#[cfg(feature = "spv-in")]
pub fn parse_spirv(
    words: &[u32],
    options: &naga::front::spv::Options,
) -> Result<naga::Module, naga::error::ShaderError<naga::front::spv::Error>> {
    let parser = naga::front::spv::Frontend::new(words.iter().cloned(), options);
    parser
        .parse()
        .map_err(|inner| naga::error::ShaderError {
            source: alloc::string::String::new(),
            label: None,
            inner: alloc::boxed::Box::new(inner),
        })
}
