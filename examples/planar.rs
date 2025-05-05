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

const VIEW_DISTANCE: f64 = 100000.;

#[cfg(feature = "wesl")]
const FRAGMENT_SHADER_ASSET_PATH: &str = "shaders/planar.wesl";

#[cfg(not(feature = "wesl"))]
const FRAGMENT_SHADER_ASSET_PATH: &str = "shaders/planar.wgsl";

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
        FRAGMENT_SHADER_ASSET_PATH.into()
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
        .insert_resource(TerrainSettings::new(vec!["albedo"]))
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
                Transform::from_translation(Vec3::new(0.0, VIEW_DISTANCE as f32, 0.0))
                    .looking_to(Vec3::NEG_Y, Vec3::NEG_Z),
                DebugCameraController::new(VIEW_DISTANCE),
                OrbitalCameraController::default(),
            ))
            .id();
    });

    commands.spawn_terrain(
        asset_server.load("terrains/earth/config.tc.ron"),
        TerrainViewConfig::default(),
        CustomMaterial {
            gradient: gradient1.clone(),
            gradient_info: GradientInfo { mode: 1 },
        },
        view,
    );
}
