#![cfg_attr(docsrs, feature(doc_cfg))]
#![no_std]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod glsl;
pub mod msl;
pub mod spv;

use alloc::string::String;

/// Create a Markdown link definition referring to the `wgpu` crate.
///
/// This macro should be used inside a `#[doc = ...]` attribute.
/// The two arguments should be string literals or macros that expand to string literals.
/// If the module in which the item using this macro is located is not the crate root,
/// use the `../` syntax.
///
/// We cannot simply use rustdoc links to `wgpu` because it is one of our dependents.
/// This link adapts to work in locally generated documentation (`cargo doc`) by default,
/// and work with `docs.rs` URL structure when building for `docs.rs`.
///
/// Note: This macro cannot be used outside this crate, because `cfg(docsrs)` will not apply.
#[cfg(not(docsrs))]
#[macro_export]
macro_rules! link_to_wgpu_docs {
    ([$reference:expr]: $url_path:expr) => {
        concat!("[", $reference, "]: ../wgpu/", $url_path)
    };

    (../ [$reference:expr]: $url_path:expr) => {
        concat!("[", $reference, "]: ../../wgpu/", $url_path)
    };
}
#[cfg(docsrs)]
#[macro_export]
macro_rules! link_to_wgpu_docs {
    ($(../)? [$reference:expr]: $url_path:expr) => {
        concat!(
            "[",
            $reference,
            // URL path will have a base URL of https://docs.rs/
            "]: /wgpu/",
            // The version of wgpu-types is not necessarily the same as the version of wgpu
            // if a patch release of either has been published, so we cannot use the full version
            // number. docs.rs will interpret this single number as a Cargo-style version
            // requirement and redirect to the latest compatible version.
            //
            // This technique would break if `wgpu` and `wgpu-types` ever switch to having distinct
            // major version numbering. An alternative would be to hardcode the corresponding `wgpu`
            // version, but that would give us another thing to forget to update.
            env!("CARGO_PKG_VERSION_MAJOR"),
            "/wgpu/",
            $url_path
        )
    };
}

/// Create a Markdown link definition referring to an item in the `wgpu` crate.
///
/// This macro should be used inside a `#[doc = ...]` attribute.
/// See [`link_to_wgpu_docs`] for more details.
#[macro_export]
macro_rules! link_to_wgpu_item {
    ($kind:ident $name:ident) => {
        $crate::link_to_wgpu_docs!(
            [concat!("`", stringify!($name), "`")]: concat!(stringify!($kind), ".", stringify!($name), ".html")
        )
    };
}

/// Stage of the programmable pipeline.
#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum ShaderStage {
    /// A vertex shader, in a render pipeline.
    Vertex,

    /// A task shader, in a mesh render pipeline.
    Task,

    /// A mesh shader, in a mesh render pipeline.
    Mesh,

    /// A fragment shader, in a render pipeline.
    Fragment,

    /// Compute pipeline shader.
    Compute,

    /// A ray generation shader, in a ray tracing pipeline.
    RayGeneration,

    /// A miss shader, in a ray tracing pipeline.
    Miss,

    /// A any hit shader, in a ray tracing pipeline.
    AnyHit,

    /// A closest hit shader, in a ray tracing pipeline.
    ClosestHit,
}

impl ShaderStage {
    pub const fn compute_like(self) -> bool {
        matches!(self, Self::Compute | Self::Task | Self::Mesh)
    }

    /// Mesh or task shader
    pub const fn mesh_like(self) -> bool {
        matches!(self, Self::Task | Self::Mesh)
    }
}

/// Hash map that is faster but not resilient to DoS attacks.
/// (Similar to rustc_hash::FxHashMap but using hashbrown::HashMap instead of alloc::collections::HashMap.)
/// To construct a new instance: `FastHashMap::default()`
pub type FastHashMap<K, T> =
    hashbrown::HashMap<K, T, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>;

/// Hash set that is faster but not resilient to DoS attacks.
/// (Similar to rustc_hash::FxHashSet but using hashbrown::HashSet instead of alloc::collections::HashMap.)
pub type FastHashSet<K> =
    hashbrown::HashSet<K, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>;

/// Specifies the values of pipeline-overridable constants in the shader module.
///
/// If an `@id` attribute was specified on the declaration,
/// the key must be the pipeline constant ID as a decimal ASCII number; if not,
/// the key must be the constant's identifier name.
///
/// The value may represent any of WGSL's concrete scalar types.
pub type PipelineConstants = hashbrown::HashMap<String, f64>;

/// Pipeline binding information for global resources.
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct ResourceBinding {
    /// The bind group index.
    pub group: u32,
    /// Binding number within the group.
    pub binding: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaskDispatchLimits {
    pub max_mesh_workgroups_per_dim: u32,
    pub max_mesh_workgroups_total: u32,
}

/// Corresponds to [WebGPU `GPUVertexFormat`](
/// https://gpuweb.github.io/gpuweb/#enumdef-gpuvertexformat).
#[repr(u32)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub enum VertexFormat {
    /// One unsigned byte (u8). `u32` in shaders.
    Uint8 = 0,
    /// Two unsigned bytes (u8). `vec2<u32>` in shaders.
    Uint8x2 = 1,
    /// Four unsigned bytes (u8). `vec4<u32>` in shaders.
    Uint8x4 = 2,
    /// One signed byte (i8). `i32` in shaders.
    Sint8 = 3,
    /// Two signed bytes (i8). `vec2<i32>` in shaders.
    Sint8x2 = 4,
    /// Four signed bytes (i8). `vec4<i32>` in shaders.
    Sint8x4 = 5,
    /// One unsigned byte (u8). [0, 255] converted to float [0, 1] `f32` in shaders.
    Unorm8 = 6,
    /// Two unsigned bytes (u8). [0, 255] converted to float [0, 1] `vec2<f32>` in shaders.
    Unorm8x2 = 7,
    /// Four unsigned bytes (u8). [0, 255] converted to float [0, 1] `vec4<f32>` in shaders.
    Unorm8x4 = 8,
    /// One signed byte (i8). [-127, 127] converted to float [-1, 1] `f32` in shaders.
    Snorm8 = 9,
    /// Two signed bytes (i8). [-127, 127] converted to float [-1, 1] `vec2<f32>` in shaders.
    Snorm8x2 = 10,
    /// Four signed bytes (i8). [-127, 127] converted to float [-1, 1] `vec4<f32>` in shaders.
    Snorm8x4 = 11,
    /// One unsigned short (u16). `u32` in shaders.
    Uint16 = 12,
    /// Two unsigned shorts (u16). `vec2<u32>` in shaders.
    Uint16x2 = 13,
    /// Four unsigned shorts (u16). `vec4<u32>` in shaders.
    Uint16x4 = 14,
    /// One signed short (u16). `i32` in shaders.
    Sint16 = 15,
    /// Two signed shorts (i16). `vec2<i32>` in shaders.
    Sint16x2 = 16,
    /// Four signed shorts (i16). `vec4<i32>` in shaders.
    Sint16x4 = 17,
    /// One unsigned short (u16). [0, 65535] converted to float [0, 1] `f32` in shaders.
    Unorm16 = 18,
    /// Two unsigned shorts (u16). [0, 65535] converted to float [0, 1] `vec2<f32>` in shaders.
    Unorm16x2 = 19,
    /// Four unsigned shorts (u16). [0, 65535] converted to float [0, 1] `vec4<f32>` in shaders.
    Unorm16x4 = 20,
    /// One signed short (i16). [-32767, 32767] converted to float [-1, 1] `f32` in shaders.
    Snorm16 = 21,
    /// Two signed shorts (i16). [-32767, 32767] converted to float [-1, 1] `vec2<f32>` in shaders.
    Snorm16x2 = 22,
    /// Four signed shorts (i16). [-32767, 32767] converted to float [-1, 1] `vec4<f32>` in shaders.
    Snorm16x4 = 23,
    /// One half-precision float (no Rust equiv). `f32` in shaders.
    Float16 = 24,
    /// Two half-precision floats (no Rust equiv). `vec2<f32>` in shaders.
    Float16x2 = 25,
    /// Four half-precision floats (no Rust equiv). `vec4<f32>` in shaders.
    Float16x4 = 26,
    /// One single-precision float (f32). `f32` in shaders.
    Float32 = 27,
    /// Two single-precision floats (f32). `vec2<f32>` in shaders.
    Float32x2 = 28,
    /// Three single-precision floats (f32). `vec3<f32>` in shaders.
    Float32x3 = 29,
    /// Four single-precision floats (f32). `vec4<f32>` in shaders.
    Float32x4 = 30,
    /// One unsigned int (u32). `u32` in shaders.
    Uint32 = 31,
    /// Two unsigned ints (u32). `vec2<u32>` in shaders.
    Uint32x2 = 32,
    /// Three unsigned ints (u32). `vec3<u32>` in shaders.
    Uint32x3 = 33,
    /// Four unsigned ints (u32). `vec4<u32>` in shaders.
    Uint32x4 = 34,
    /// One signed int (i32). `i32` in shaders.
    Sint32 = 35,
    /// Two signed ints (i32). `vec2<i32>` in shaders.
    Sint32x2 = 36,
    /// Three signed ints (i32). `vec3<i32>` in shaders.
    Sint32x3 = 37,
    /// Four signed ints (i32). `vec4<i32>` in shaders.
    Sint32x4 = 38,
    /// One double-precision float (f64). `f32` in shaders. Requires [`Features::VERTEX_ATTRIBUTE_64BIT`].
    ///
    /// [`Features::VERTEX_ATTRIBUTE_64BIT`]: ../wgpu/struct.Features.html#associatedconstant.VERTEX_ATTRIBUTE_64BIT
    Float64 = 39,
    /// Two double-precision floats (f64). `vec2<f32>` in shaders. Requires [`Features::VERTEX_ATTRIBUTE_64BIT`].
    ///
    /// [`Features::VERTEX_ATTRIBUTE_64BIT`]: ../wgpu/struct.Features.html#associatedconstant.VERTEX_ATTRIBUTE_64BIT
    Float64x2 = 40,
    /// Three double-precision floats (f64). `vec3<f32>` in shaders. Requires [`Features::VERTEX_ATTRIBUTE_64BIT`].
    ///
    /// [`Features::VERTEX_ATTRIBUTE_64BIT`]: ../wgpu/struct.Features.html#associatedconstant.VERTEX_ATTRIBUTE_64BIT
    Float64x3 = 41,
    /// Four double-precision floats (f64). `vec4<f32>` in shaders. Requires [`Features::VERTEX_ATTRIBUTE_64BIT`].
    ///
    /// [`Features::VERTEX_ATTRIBUTE_64BIT`]: ../wgpu/struct.Features.html#associatedconstant.VERTEX_ATTRIBUTE_64BIT
    Float64x4 = 42,
    /// Three unsigned 10-bit integers and one 2-bit integer, packed into a 32-bit integer (u32). [0, 1024] converted to float [0, 1] `vec4<f32>` in shaders.
    #[cfg_attr(
        any(feature = "serialize", feature = "deserialize"),
        serde(rename = "unorm10-10-10-2")
    )]
    Unorm10_10_10_2 = 43,
    /// Four unsigned 8-bit integers, packed into a 32-bit integer (u32). [0, 255] converted to float [0, 1] `vec4<f32>` in shaders.
    #[cfg_attr(
        any(feature = "serialize", feature = "deserialize"),
        serde(rename = "unorm8x4-bgra")
    )]
    Unorm8x4Bgra = 44,
}

impl VertexFormat {
    /// Returns the byte size of the format.
    #[must_use]
    pub const fn size(&self) -> u64 {
        match self {
            Self::Uint8 | Self::Sint8 | Self::Unorm8 | Self::Snorm8 => 1,
            Self::Uint8x2
            | Self::Sint8x2
            | Self::Unorm8x2
            | Self::Snorm8x2
            | Self::Uint16
            | Self::Sint16
            | Self::Unorm16
            | Self::Snorm16
            | Self::Float16 => 2,
            Self::Uint8x4
            | Self::Sint8x4
            | Self::Unorm8x4
            | Self::Snorm8x4
            | Self::Uint16x2
            | Self::Sint16x2
            | Self::Unorm16x2
            | Self::Snorm16x2
            | Self::Float16x2
            | Self::Float32
            | Self::Uint32
            | Self::Sint32
            | Self::Unorm10_10_10_2
            | Self::Unorm8x4Bgra => 4,
            Self::Uint16x4
            | Self::Sint16x4
            | Self::Unorm16x4
            | Self::Snorm16x4
            | Self::Float16x4
            | Self::Float32x2
            | Self::Uint32x2
            | Self::Sint32x2
            | Self::Float64 => 8,
            Self::Float32x3 | Self::Uint32x3 | Self::Sint32x3 => 12,
            Self::Float32x4 | Self::Uint32x4 | Self::Sint32x4 | Self::Float64x2 => 16,
            Self::Float64x3 => 24,
            Self::Float64x4 => 32,
        }
    }

    /// Returns the size read by an acceleration structure build of the vertex format. This is
    /// slightly different from [`Self::size`] because the alpha component of 4-component formats
    /// are not read in an acceleration structure build, allowing for a smaller stride.
    #[must_use]
    pub const fn min_acceleration_structure_vertex_stride(&self) -> u64 {
        match self {
            Self::Float16x2 | Self::Snorm16x2 => 4,
            Self::Float32x3 => 12,
            Self::Float32x2 => 8,
            // This is the minimum value from DirectX
            // > A16 component is ignored, other data can be packed there, such as setting vertex stride to 6 bytes
            //
            // https://microsoft.github.io/DirectX-Specs/d3d/Raytracing.html#d3d12_raytracing_geometry_triangles_desc
            //
            // Vulkan does not express a minimum stride.
            Self::Float16x4 | Self::Snorm16x4 => 6,
            _ => unreachable!(),
        }
    }

    /// Returns the alignment required for `wgpu::BlasTriangleGeometry::vertex_stride`
    #[must_use]
    pub const fn acceleration_structure_stride_alignment(&self) -> u64 {
        match self {
            Self::Float16x4 | Self::Float16x2 | Self::Snorm16x4 | Self::Snorm16x2 => 2,
            Self::Float32x2 | Self::Float32x3 => 4,
            _ => unreachable!(),
        }
    }
}

/// A mapping of vertex buffers and their attributes to shader
/// locations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct VertexAttribute {
    /// Shader location associated with this attribute
    pub shader_location: u32,
    /// Offset in bytes from start of vertex buffer structure
    pub offset: u64,
    /// Format code to help us unpack the attribute into the type
    /// used by the shader. Codes correspond to a 0-based index of
    /// <https://gpuweb.github.io/gpuweb/#enumdef-gpuvertexformat>.
    /// The conversion process is described by
    /// <https://gpuweb.github.io/gpuweb/#vertex-processing>.
    pub format: VertexFormat,
}

/// Whether a vertex buffer is indexed by vertex or by instance.
///
/// Consider a call to [`RenderPass::draw`] like this:
///
/// ```ignore
/// render_pass.draw(vertices, instances)
/// ```
///
/// where `vertices` is a `Range<u32>` of vertex indices, and
/// `instances` is a `Range<u32>` of instance indices.
///
/// For this call, `wgpu` invokes the vertex shader entry point once
/// for every possible `(v, i)` pair, where `v` is drawn from
/// `vertices` and `i` is drawn from `instances`. These invocations
/// may happen in any order, and will usually run in parallel.
///
/// Each vertex buffer has a step mode, established by the
/// [`step_mode`] field of its [`VertexBufferLayout`], given when the
/// pipeline was created. Buffers whose step mode is [`Vertex`] use
/// `v` as the index into their contents, whereas buffers whose step
/// mode is [`Instance`] use `i`. The indicated buffer element then
/// contributes zero or more attribute values for the `(v, i)` vertex
/// shader invocation to use, based on the [`VertexBufferLayout`]'s
/// [`attributes`] list.
///
/// You can visualize the results from all these vertex shader
/// invocations as a matrix with a row for each `i` from `instances`,
/// and with a column for each `v` from `vertices`. In one sense, `v`
/// and `i` are symmetrical: both are used to index vertex buffers and
/// provide attribute values.  But the key difference between `v` and
/// `i` is that line and triangle primitives are built from the values
/// of each row, along which `i` is constant and `v` varies, not the
/// columns.
///
/// An indexed draw call works similarly:
///
/// ```ignore
/// render_pass.draw_indexed(indices, base_vertex, instances)
/// ```
///
/// The only difference is that `v` values are drawn from the contents
/// of the index buffer&mdash;specifically, the subrange of the index
/// buffer given by `indices`&mdash;instead of simply being sequential
/// integers, as they are in a `draw` call.
///
/// A non-instanced call, where `instances` is `0..1`, is simply a
/// matrix with only one row.
///
/// Corresponds to [WebGPU `GPUVertexStepMode`](
/// https://gpuweb.github.io/gpuweb/#enumdef-gpuvertexstepmode).
///
#[doc = link_to_wgpu_docs!(["`RenderPass::draw`"]: "struct.RenderPass.html#method.draw")]
#[doc = link_to_wgpu_item!(struct VertexBufferLayout)]
#[doc = link_to_wgpu_docs!(["`step_mode`"]: "struct.VertexBufferLayout.html#structfield.step_mode")]
#[doc = link_to_wgpu_docs!(["`attributes`"]: "struct.VertexBufferLayout.html#structfield.attributes")]
/// [`Vertex`]: VertexStepMode::Vertex
/// [`Instance`]: VertexStepMode::Instance
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(
    any(feature = "serialize", feature = "deserialize"),
    serde(rename_all = "kebab-case")
)]
pub enum VertexStepMode {
    /// Vertex data is advanced every vertex.
    #[default]
    Vertex = 0,
    /// Vertex data is advanced every instance.
    Instance = 1,
}

/// The output topology for a mesh shader. Note that mesh shaders don't allow things like triangle-strips.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum PrimitiveTopology {
    /// Outputs individual vertices to be rendered as points.
    Points,
    /// Outputs groups of 2 vertices to be renderedas lines .
    Lines,
    /// Outputs groups of 3 vertices to be rendered as triangles.
    Triangles,
}

/// Built-in inputs and outputs.
#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum BuiltIn {
    /// Written in vertex/mesh shaders, read in fragment shaders
    Position { invariant: bool },
    /// Read in task, mesh, vertex, and fragment shaders
    ViewIndex,

    /// Read in vertex shaders
    BaseInstance,
    /// Read in vertex shaders
    BaseVertex,
    /// Written in vertex & mesh shaders
    ClipDistance,
    /// Written in vertex & mesh shaders
    CullDistance,
    /// Read in vertex, any- and closest-hit shaders
    InstanceIndex,
    /// Written in vertex & mesh shaders
    PointSize,
    /// Read in vertex shaders
    VertexIndex,
    /// Read in vertex & task shaders, or mesh shaders in pipelines without task shaders
    DrawIndex,

    /// Written in fragment shaders
    FragDepth,
    /// Read in fragment shaders
    PointCoord,
    /// Read in fragment shaders
    FrontFacing,
    /// Read in fragment shaders, written in mesh shaders, read in any and closest hit shaders.
    PrimitiveIndex,
    /// Read in fragment shaders
    Barycentric { perspective: bool },
    /// Read in fragment shaders
    SampleIndex,
    /// Read or written in fragment shaders
    SampleMask,

    /// Read in compute, task, and mesh shaders
    GlobalInvocationId,
    /// Read in compute, task, and mesh shaders
    LocalInvocationId,
    /// Read in compute, task, and mesh shaders
    LocalInvocationIndex,
    /// Read in compute, task, and mesh shaders
    WorkGroupId,
    /// Read in compute, task, and mesh shaders
    WorkGroupSize,
    /// Read in compute, task, and mesh shaders
    NumWorkGroups,

    /// Read in compute, task, and mesh shaders
    NumSubgroups,
    /// Read in compute, task, and mesh shaders
    SubgroupId,
    /// Read in compute, fragment, task, and mesh shaders
    SubgroupSize,
    /// Read in compute, fragment, task, and mesh shaders
    SubgroupInvocationId,

    /// Written in task shaders
    MeshTaskSize,
    /// Written in mesh shaders
    CullPrimitive,
    /// Written in mesh shaders
    PointIndex,
    /// Written in mesh shaders
    LineIndices,
    /// Written in mesh shaders
    TriangleIndices,

    /// Written to a workgroup variable in mesh shaders
    VertexCount,
    /// Written to a workgroup variable in mesh shaders
    Vertices,
    /// Written to a workgroup variable in mesh shaders
    PrimitiveCount,
    /// Written to a workgroup variable in mesh shaders
    Primitives,

    /// Read in all ray tracing pipeline shaders, the id within the number of
    /// rays that this current ray is.
    RayInvocationId,
    /// Read in all ray tracing pipeline shaders, the number of rays created.
    NumRayInvocations,
    /// Read in closest hit and any hit shaders, the custom data in the tlas
    /// instance
    InstanceCustomData,
    /// Read in closest hit and any hit shaders, the index of the geometry in
    /// the blas.
    GeometryIndex,
    /// Read in closest hit, any hit, and miss shaders, the origin of the ray.
    WorldRayOrigin,
    /// Read in closest hit, any hit, and miss shaders, the direction of the
    /// ray.
    WorldRayDirection,
    /// Read in closest hit and any hit shaders, the direction of the ray in
    /// object space.
    ObjectRayOrigin,
    /// Read in closest hit and any hit shaders, the direction of the ray in
    /// object space.
    ObjectRayDirection,
    /// Read in closest hit, any hit, and miss shaders, the t min provided by
    /// in the ray desc.
    RayTmin,
    /// Read in closest hit, any hit, and miss shaders, the final bounds at which
    /// a hit is accepted (the closest committed hit if there is one otherwise, t
    /// max provided in the ray desc).
    RayTCurrentMax,
    /// Read in closest hit and any hit shaders, the matrix for converting from
    /// object space to world space
    ObjectToWorld,
    /// Read in closest hit and any hit shaders, the matrix for converting from
    /// world space to object space
    WorldToObject,
    /// Read in closest hit and any hit shaders, the type of hit as provided by
    /// the intersection function if any, otherwise this is 254 (0xFE) for a
    /// front facing triangle and 255 (0xFF) for a back facing triangle
    HitKind,
}

impl From<PrimitiveTopology> for crate::BuiltIn {
    fn from(value: crate::PrimitiveTopology) -> Self {
        match value {
            PrimitiveTopology::Points => Self::PointIndex,
            PrimitiveTopology::Lines => Self::LineIndices,
            PrimitiveTopology::Triangles => Self::TriangleIndices,
        }
    }
}
