#[cfg(feature = "wesl")]
pub const DEFAULT_VERTEX_SHADER: &str = "embedded://bevy_terrain/shaders/render/vertex.wesl";

#[cfg(feature = "wesl")]
pub const DEFAULT_FRAGMENT_SHADER: &str = "embedded://bevy_terrain/shaders/render/fragment.wesl";

#[cfg(feature = "wesl")]
pub const PREPARE_PREPASS_SHADER: &str =
    "embedded://bevy_terrain/shaders/tiling_prepass/prepare_prepass.wesl";

#[cfg(feature = "wesl")]
pub const REFINE_TILES_SHADER: &str =
    "embedded://bevy_terrain/shaders/tiling_prepass/refine_tiles.wesl";

#[cfg(feature = "wesl")]
pub(crate) const PICKING_SHADER: &str = "embedded://bevy_terrain/shaders/picking.wesl";

#[cfg(feature = "wesl")]
pub(crate) const DEPTH_COPY_SHADER: &str = "embedded://bevy_terrain/shaders/depth_copy.wesl";

#[cfg(feature = "wesl")]
pub(crate) const MIP_SHADER: &str = "embedded://bevy_terrain/shaders/mipmap.wesl";

#[cfg(not(feature = "wesl"))]
pub const DEFAULT_VERTEX_SHADER: &str = "embedded://bevy_terrain/shaders/render/vertex.wgsl";

#[cfg(not(feature = "wesl"))]
pub const DEFAULT_FRAGMENT_SHADER: &str = "embedded://bevy_terrain/shaders/render/fragment.wgsl";

#[cfg(not(feature = "wesl"))]
pub const PREPARE_PREPASS_SHADER: &str =
    "embedded://bevy_terrain/shaders/tiling_prepass/prepare_prepass.wgsl";

#[cfg(not(feature = "wesl"))]
pub const REFINE_TILES_SHADER: &str =
    "embedded://bevy_terrain/shaders/tiling_prepass/refine_tiles.wgsl";

//#[cfg(not(feature = "wesl"))]
// pub(crate) const SPLIT_SHADER: &str = "embedded://bevy_terrain/shaders/preprocess/split.wgsl";
//
// #[cfg(not(feature = "wesl"))]
// pub(crate) const STITCH_SHADER: &str = "embedded://bevy_terrain/shaders/preprocess/stitch.wgsl";
//
// #[cfg(not(feature = "wesl"))]
// pub(crate) const DOWNSAMPLE_SHADER: &str =
//     "embedded://bevy_terrain/shaders/preprocess/downsample.wgsl";

#[cfg(not(feature = "wesl"))]
pub(crate) const PICKING_SHADER: &str = "embedded://bevy_terrain/shaders/picking.wgsl";

#[cfg(not(feature = "wesl"))]
pub(crate) const DEPTH_COPY_SHADER: &str = "embedded://bevy_terrain/shaders/depth_copy.wgsl";

#[cfg(not(feature = "wesl"))]
pub(crate) const MIP_SHADER: &str = "embedded://bevy_terrain/shaders/mipmap.wgsl";
