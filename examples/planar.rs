use bevy::{
    prelude::{
        App, Asset, AssetServer, Commands, DefaultPlugins, Entity, Handle, Image, Material,
        PluginGroup, Res, ResMut, Startup, Transform, TransformPlugin, TypePath, Vec3, vec,
    },
    render::render_resource::{
        AsBindGroup, ShaderRef, ShaderType, TextureDimension, TextureFormat,
    },
};
use bevy_terrain::prelude::{
    BigSpaceCommands, DebugCameraController, Grid, LoadingImages, OrbitalCameraController,
    SpawnTerrainCommandsExt, TerrainDebugPlugin, TerrainMaterialPlugin, TerrainPickingPlugin,
    TerrainPlugin, TerrainSettings, TerrainViewConfig,
};

// View distance for planar terrain
const VIEW_DISTANCE: f64 = 10000.0;

#[derive(ShaderType, Clone)]
struct GradientInfo {
    mode: u32,
}

#[derive(Asset, AsBindGroup, TypePath, Clone)]
pub struct CustomMaterial {
    #[texture(0)]
    #[sampler(1)]
    gradient: Handle<Image>,
    #[uniform(2)]
    gradient_info: GradientInfo,
}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/planar.wgsl".into()
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.build().disable::<TransformPlugin>(),
            TerrainPlugin,
            TerrainMaterialPlugin::<CustomMaterial>::default(),
            TerrainDebugPlugin,
            TerrainPickingPlugin,
        ))
        .insert_resource(TerrainSettings::new(vec!["albedo"]).with_atlas_size(128))
        .add_systems(Startup, initialize)
        .run();
}

#[allow(clippy::too_many_arguments)]
fn initialize(
    mut commands: Commands,
    mut images: ResMut<LoadingImages>,
    asset_server: Res<AssetServer>,
) {
    let gradient1 = asset_server.load("textures/gradient1.png");
    images.load_image(
        &gradient1,
        TextureDimension::D2,
        TextureFormat::Rgba8UnormSrgb,
    );

    let gradient2 = asset_server.load("textures/gradient2.png");
    images.load_image(
        &gradient2,
        TextureDimension::D2,
        TextureFormat::Rgba8UnormSrgb,
    );

    let mut view = Entity::PLACEHOLDER;

    commands.spawn_big_space(Grid::default(), |root| {
        view = root
            .spawn_spatial((
                // Position the camera higher above the terrain with a wider viewing angle
                Transform::from_translation(Vec3::new(0.0, VIEW_DISTANCE as f32 * 0.5, 0.0))
                    .looking_to(Vec3::NEG_Y, Vec3::Z),
                DebugCameraController::new(VIEW_DISTANCE * 0.25),
                OrbitalCameraController::default(),
            ))
            .id();
    });

    // Create a view config with a wider tree size to load more tiles
    let view_config = TerrainViewConfig {
        tree_size: 16,           // Increase tree size to load more tiles
        geometry_tile_count: 33, // Increase geometry tiles
        view_lod: 0,             // Start at highest LOD
        grid_size: 64,           // Grid size for tile mesh
        precision_distance: 1.0,  // Distance for precision
        morph_distance: 1.5,     // Morph distance between LODs
        blend_distance: 2.0,     // Blend distance between LODs
        morph_range: 0.3,        // Range for morphing
        blend_range: 0.3,        // Range for blending
        subdivision_tolerance: 0.1, // Tolerance for subdivision
        load_tolerance: 0.5,     // Tolerance for loading
        refinement_count: 4,     // Number of refinements
        order: 0,                // Rendering order
    };

    commands.spawn_terrain(
        asset_server.load("terrains/earth/config.tc.ron"),
        view_config,
        CustomMaterial {
            gradient: gradient1.clone(),
            gradient_info: GradientInfo { mode: 1 },
        },
        view,
    );
}
