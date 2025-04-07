//! Contains a debug resource and systems controlling it to visualize different internal
//! data of the plugin.
use crate::{
    debug::{debug_camera_controller, debug_surface_approximation, orbital_camera_controller},
    terrain_data::{TileAtlas, TileTree},
    terrain_view::TerrainViewComponents,
};

use bevy::{
    prelude::*,
    render::{Extract, RenderApp, render_resource::*},
    window::PrimaryWindow,
};

mod approximation_debug;
mod camera;
mod orbital_camera;

pub(crate) use self::{approximation_debug::*, camera::*, orbital_camera::*};
pub use self::{camera::DebugCameraController, orbital_camera::OrbitalCameraController};

#[cfg(feature = "metal_capture")]
mod metal_capture;
#[cfg(feature = "metal_capture")]
pub use self::metal_capture::MetalCapturePlugin;

#[derive(Asset, AsBindGroup, TypePath, Clone, Default)]
pub struct DebugTerrainMaterial {}

impl Material for DebugTerrainMaterial {}

/// Adds a terrain debug config, a debug camera and debug control systems.
pub struct TerrainDebugPlugin;

impl Plugin for TerrainDebugPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "metal_capture")]
        app.add_plugins(MetalCapturePlugin);

        app.init_resource::<DebugTerrain>()
            .init_resource::<LoadingImages>()
            .add_systems(Startup, (debug_lighting, debug_window))
            .add_systems(
                Update,
                (
                    toggle_debug,
                    update_terrain_parameter,
                    update_view_parameter,
                    finish_loading_images,
                    orbital_camera_controller,
                    debug_camera_controller,
                ),
            )
            .add_systems(
                Last,
                debug_surface_approximation.after(TileTree::generate_surface_approximation),
            );

        app.sub_app_mut(RenderApp)
            .init_resource::<DebugTerrain>()
            .add_systems(ExtractSchedule, extract_debug);
    }
}

#[derive(Clone, Resource)]
pub struct DebugTerrain {
    pub wireframe: bool,
    pub show_data_lod: bool,
    pub show_geometry_lod: bool,
    pub show_tile_tree: bool,
    pub show_pixels: bool,
    pub show_uv: bool,
    pub show_normals: bool,
    pub morph: bool,
    pub blend: bool,
    pub tile_tree_lod: bool,
    pub lighting: bool,
    pub sample_grad: bool,
    pub high_precision: bool,
    pub freeze: bool,
    pub test1: bool,
    pub test2: bool,
    pub test3: bool,
}

impl Default for DebugTerrain {
    fn default() -> Self {
        Self {
            wireframe: false,
            show_data_lod: false,
            show_geometry_lod: false,
            show_tile_tree: false,
            show_pixels: false,
            show_uv: false,
            show_normals: false,
            morph: true,
            blend: true,
            tile_tree_lod: false,
            lighting: true,
            sample_grad: true,
            high_precision: true,
            freeze: false,
            test1: false,
            test2: false,
            test3: false,
        }
    }
}

pub fn extract_debug(mut debug: ResMut<DebugTerrain>, extracted_debug: Extract<Res<DebugTerrain>>) {
    *debug = extracted_debug.clone();
}

pub fn toggle_debug(input: Res<ButtonInput<KeyCode>>, mut debug_terrain: ResMut<DebugTerrain>) {
    if input.just_pressed(KeyCode::KeyW) {
        debug_terrain.wireframe = !debug_terrain.wireframe;
        info!(
            "Toggled the wireframe view {}.",
            if debug_terrain.wireframe { "on" } else { "off" }
        )
    }
    if input.just_pressed(KeyCode::KeyE) {
        debug_terrain.show_data_lod = !debug_terrain.show_data_lod;
        info!(
            "Toggled the terrain data LOD view {}.",
            if debug_terrain.show_data_lod {
                "on"
            } else {
                "off"
            }
        )
    }
    if input.just_pressed(KeyCode::KeyR) {
        debug_terrain.show_geometry_lod = !debug_terrain.show_geometry_lod;
        info!(
            "Toggled the terrain geometry LOD view {}.",
            if debug_terrain.show_geometry_lod {
                "on"
            } else {
                "off"
            }
        )
    }
    if input.just_pressed(KeyCode::KeyT) {
        debug_terrain.show_tile_tree = !debug_terrain.show_tile_tree;
        info!(
            "Toggled the tile tree LOD view {}.",
            if debug_terrain.show_tile_tree {
                "on"
            } else {
                "off"
            }
        )
    }
    if input.just_pressed(KeyCode::KeyY) {
        debug_terrain.show_pixels = !debug_terrain.show_pixels;
        info!(
            "Toggled the pixel view {}.",
            if debug_terrain.show_pixels {
                "on"
            } else {
                "off"
            }
        )
    }
    if input.just_pressed(KeyCode::KeyU) {
        debug_terrain.show_uv = !debug_terrain.show_uv;
        info!(
            "Toggled the uv view {}.",
            if debug_terrain.show_uv { "on" } else { "off" }
        )
    }
    if input.just_pressed(KeyCode::KeyI) {
        debug_terrain.show_normals = !debug_terrain.show_normals;
        info!(
            "Toggled the normals view {}.",
            if debug_terrain.show_normals {
                "on"
            } else {
                "off"
            }
        )
    }
    if input.just_pressed(KeyCode::KeyO) {
        debug_terrain.morph = !debug_terrain.morph;
        info!(
            "Morphing: {}.",
            if debug_terrain.morph { "on" } else { "off" }
        )
    }
    if input.just_pressed(KeyCode::KeyP) {
        debug_terrain.blend = !debug_terrain.blend;
        info!(
            "Blending: {}.",
            if debug_terrain.blend { "on" } else { "off" }
        )
    }
    if input.just_pressed(KeyCode::BracketLeft) {
        debug_terrain.tile_tree_lod = !debug_terrain.tile_tree_lod;
        info!(
            "Tile tree lod: {}.",
            if debug_terrain.tile_tree_lod {
                "on"
            } else {
                "off"
            }
        )
    }
    if input.just_pressed(KeyCode::BracketRight) {
        debug_terrain.lighting = !debug_terrain.lighting;
        info!(
            "Lighting: {}.",
            if debug_terrain.lighting { "on" } else { "off" }
        )
    }
    if input.just_pressed(KeyCode::Backslash) {
        debug_terrain.sample_grad = !debug_terrain.sample_grad;
        info!(
            "Texture sampling using gradients: {}.",
            if debug_terrain.sample_grad {
                "on"
            } else {
                "off"
            }
        )
    }
    if input.just_pressed(KeyCode::Semicolon) {
        debug_terrain.high_precision = !debug_terrain.high_precision;
        info!(
            "Toggled high precision coordinates {}.",
            if debug_terrain.high_precision {
                "on"
            } else {
                "off"
            }
        )
    }
    if input.just_pressed(KeyCode::KeyF) {
        debug_terrain.freeze = !debug_terrain.freeze;
        info!(
            "{} the view frustum.",
            if debug_terrain.freeze {
                "Froze"
            } else {
                "Unfroze"
            }
        )
    }
    if input.just_pressed(KeyCode::Digit1) {
        debug_terrain.test1 = !debug_terrain.test1;
        info!(
            "Debug flag 1: {}.",
            if debug_terrain.test1 { "on" } else { "off" }
        )
    }
    if input.just_pressed(KeyCode::Digit2) {
        debug_terrain.test2 = !debug_terrain.test2;
        info!(
            "Debug flag 2: {}.",
            if debug_terrain.test2 { "on" } else { "off" }
        )
    }
    if input.just_pressed(KeyCode::Digit3) {
        debug_terrain.test3 = !debug_terrain.test3;
        info!(
            "Debug flag 3: {}.",
            if debug_terrain.test3 { "on" } else { "off" }
        )
    }
}

pub fn update_terrain_parameter(
    input: Res<ButtonInput<KeyCode>>,
    mut tile_atlases: Query<&mut TileAtlas>,
) {
    for mut tile_atlas in tile_atlases.iter_mut() {
        if input.pressed(KeyCode::ShiftLeft) && input.just_pressed(KeyCode::Equal) {
            tile_atlas.height_scale += 0.1;
            info!("Heightscale: {}", tile_atlas.height_scale);
        }
        if input.just_pressed(KeyCode::Minus) {
            tile_atlas.height_scale -= 0.1;
            info!("Heightscale: {}", tile_atlas.height_scale);
        }
    }
}

pub fn update_view_parameter(
    input: Res<ButtonInput<KeyCode>>,
    mut tile_trees: ResMut<TerrainViewComponents<TileTree>>,
) {
    for tile_tree in tile_trees.values_mut() {
        let scale = tile_tree.shape.scale_f32();

        if input.just_pressed(KeyCode::KeyV) {
            tile_tree.blend_distance -= 0.25 * scale;
            info!("Blend distance: {}.", tile_tree.blend_distance / scale);
        }
        if input.just_pressed(KeyCode::KeyB) {
            tile_tree.blend_distance += 0.25 * scale;
            info!("Blend distance: {}.", tile_tree.blend_distance / scale);
        }

        if input.just_pressed(KeyCode::KeyN) {
            tile_tree.morph_distance -= 0.25 * scale;
            info!("Morph distance: {}.", tile_tree.morph_distance / scale);
        }
        if input.just_pressed(KeyCode::KeyM) {
            tile_tree.morph_distance += 0.25 * scale;
            info!("Morth distance: {}.", tile_tree.morph_distance / scale);
        }

        if input.just_pressed(KeyCode::KeyG) && tile_tree.grid_size > 2 {
            tile_tree.grid_size -= 2;
            info!("Grid size: {}.", tile_tree.grid_size);
        }
        if input.just_pressed(KeyCode::KeyH) {
            tile_tree.grid_size += 2;
            info!("Grid size {}.", tile_tree.grid_size);
        }
    }
}

pub(crate) fn debug_lighting(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            illuminance: 5000.0,
            ..default()
        },
        Transform::from_xyz(-1.0, 1.0, -3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.insert_resource(AmbientLight {
        brightness: 100.0,
        ..default()
    });
}

pub fn debug_window(mut window: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = window.single_mut().unwrap();
    window.cursor_options.visible = true; // false;
}

#[derive(Resource, Default)]
pub struct LoadingImages(Vec<(AssetId<Image>, TextureDimension, TextureFormat)>);

impl LoadingImages {
    pub fn load_image(
        &mut self,
        handle: &Handle<Image>,
        dimension: TextureDimension,
        format: TextureFormat,
    ) -> &mut Self {
        self.0.push((handle.id(), dimension, format));
        self
    }
}

fn finish_loading_images(
    asset_server: Res<AssetServer>,
    mut loading_images: ResMut<LoadingImages>,
    mut images: ResMut<Assets<Image>>,
) {
    loading_images.0.retain(|&(id, dimension, format)| {
        if asset_server.load_state(id).is_loaded() {
            let image = images.get_mut(id).unwrap();
            image.texture_descriptor.dimension = dimension;
            image.texture_descriptor.format = format;

            false
        } else {
            true
        }
    });
}
