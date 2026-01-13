#[cfg(not(feature = "wesl"))]
use bevy::shader::ShaderRef;
use bevy::{prelude::*, reflect::TypePath, render::render_resource::*, window::WindowResolution};
use bevy_terrain::prelude::{
    BigSpaceCommands, DebugCameraController, Grid, LoadingImages, OrbitalCameraController,
    SpawnTerrainCommandsExt, TerrainDebugPlugin, TerrainMaterialPlugin, TerrainPickingPlugin,
    TerrainPlugin, TerrainSettings, TerrainViewConfig,
};

const RADIUS: f64 = 6371000.0;

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
    #[cfg(not(feature = "wesl"))]
    fn fragment_shader() -> ShaderRef {
        "shaders/spherical.wgsl".into()
    }

    #[cfg(feature = "wesl")]
    fn fragment_shader() -> ShaderRef {
        "shaders/spherical.wesl".into()
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(1920, 1080),
                        ..default()
                    }),
                    ..default()
                })
                .build()
                .disable::<TransformPlugin>(),
            TerrainPlugin,
            TerrainMaterialPlugin::<CustomMaterial>::default(),
            TerrainDebugPlugin,
            TerrainPickingPlugin,
        ))
        .insert_resource(TerrainSettings::new(vec!["albedo"]))
        // .insert_resource(ClearColor(Color::WHITE))
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
                Transform::from_translation(-Vec3::X * RADIUS as f32 * 3.0)
                    .looking_to(Vec3::X, Vec3::Y),
                DebugCameraController::new(RADIUS),
                OrbitalCameraController::default(),
            ))
            .id();
    });

    commands.spawn_terrain(
        asset_server.load("terrains/earth/config.tc.ron"),
        TerrainViewConfig::default(),
        CustomMaterial {
            gradient: gradient1.clone(),
            gradient_info: GradientInfo { mode: 2 },
        },
        view,
    );
}
