mod path_constants;

use crate::terrain_data::AttachmentLabel;
use bevy::{asset::embedded_asset, prelude::*};
use itertools::Itertools;

pub(crate) use path_constants::{
    DEFAULT_FRAGMENT_SHADER, DEFAULT_VERTEX_SHADER, DEPTH_COPY_SHADER, MIP_SHADER, PICKING_SHADER,
    PREPARE_PREPASS_SHADER, REFINE_TILES_SHADER,
};

#[derive(Default, Resource)]
pub(crate) struct InternalShaders(Vec<Handle<Shader>>);

impl InternalShaders {
    pub(crate) fn load(app: &mut App, shaders: &[&'static str]) {
        let mut shaders = shaders
            .iter()
            .map(|&shader| app.world_mut().resource_mut::<AssetServer>().load(shader))
            .collect_vec();

        let mut internal_shaders = app.world_mut().resource_mut::<InternalShaders>();
        internal_shaders.0.append(&mut shaders);
    }
}

// Todo: this could be implemented using shader defs with values
fn load_bindings_shader(app: &mut App, attachments: &[AttachmentLabel]) {
    #[cfg(feature = "wesl")]
    let source = include_str!("bindings.wesl");

    #[cfg(not(feature = "wesl"))]
    let source = include_str!("bindings.wgsl");

    let source = (0..8).fold(source.to_string(), |src, i| {
        src.replacen(
            &format!("{{{i}}}"),
            &String::from(
                &attachments
                    .get(i)
                    .cloned()
                    .unwrap_or(AttachmentLabel::Empty(i)),
            ),
            2,
        )
    });

    let mut shaders = app.world_mut().resource_mut::<Assets<Shader>>();

    #[cfg(feature = "wesl")]
    let shader = shaders.add(Shader::from_wesl(source, "bindings.wesl"));

    #[cfg(not(feature = "wesl"))]
    let shader = shaders.add(Shader::from_wgsl(source, "bindings.wgsl"));

    let mut internal_shaders = app.world_mut().resource_mut::<InternalShaders>();
    internal_shaders.0.push(shader);
}

#[cfg(feature = "wesl")]
pub(crate) fn load_terrain_shaders(app: &mut App, attachments: &[AttachmentLabel]) {
    embedded_asset!(app, "types.wesl");
    embedded_asset!(app, "attachments.wesl");
    embedded_asset!(app, "functions.wesl");
    embedded_asset!(app, "debug.wesl");
    embedded_asset!(app, "render/vertex.wesl");
    embedded_asset!(app, "render/fragment.wesl");
    embedded_asset!(app, "tiling_prepass/prepare_prepass.wesl");
    embedded_asset!(app, "tiling_prepass/refine_tiles.wesl");
    embedded_asset!(app, "picking.wesl");
    embedded_asset!(app, "depth_copy.wesl");
    embedded_asset!(app, "mipmap.wesl");

    load_bindings_shader(app, attachments);

    InternalShaders::load(
        app,
        &[
            "embedded://bevy_terrain/shaders/types.wesl",
            "embedded://bevy_terrain/shaders/attachments.wesl",
            "embedded://bevy_terrain/shaders/functions.wesl",
            "embedded://bevy_terrain/shaders/debug.wesl",
            "embedded://bevy_terrain/shaders/render/vertex.wesl",
            "embedded://bevy_terrain/shaders/render/fragment.wesl",
        ],
    );
}

#[cfg(not(feature = "wesl"))]
pub(crate) fn load_terrain_shaders(app: &mut App, attachments: &[AttachmentLabel]) {
    embedded_asset!(app, "types.wgsl");
    embedded_asset!(app, "attachments.wgsl");
    embedded_asset!(app, "functions.wgsl");
    embedded_asset!(app, "debug.wgsl");
    embedded_asset!(app, "render/vertex.wgsl");
    embedded_asset!(app, "render/fragment.wgsl");
    embedded_asset!(app, "tiling_prepass/prepare_prepass.wgsl");
    embedded_asset!(app, "tiling_prepass/refine_tiles.wgsl");
    embedded_asset!(app, "picking.wgsl");
    embedded_asset!(app, "depth_copy.wgsl");
    embedded_asset!(app, "mipmap.wgsl");

    load_bindings_shader(app, attachments);

    InternalShaders::load(
        app,
        &[
            "embedded://bevy_terrain/shaders/types.wgsl",
            "embedded://bevy_terrain/shaders/attachments.wgsl",
            "embedded://bevy_terrain/shaders/functions.wgsl",
            "embedded://bevy_terrain/shaders/debug.wgsl",
            "embedded://bevy_terrain/shaders/render/vertex.wgsl",
            "embedded://bevy_terrain/shaders/render/fragment.wgsl",
        ],
    );
}

// pub(crate) fn load_preprocess_shaders(app: &mut App) {
//     embedded_asset!(app, "preprocess/preprocessing.wgsl");
//     embedded_asset!(app, "preprocess/split.wgsl");
//     embedded_asset!(app, "preprocess/stitch.wgsl");
//     embedded_asset!(app, "preprocess/downsample.wgsl");
//
//     InternalShaders::load(
//         app,
//         &["embedded://bevy_terrain/shaders/preprocess/preprocessing.wgsl"],
//     );
// }
