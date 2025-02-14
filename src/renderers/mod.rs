// TODO clean this up 
// Right now, this module and the UI module are a bit of a mess of (near) constant state checks 

use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};

use crate::{ui::{LightSettings, MaterialSettings}, DemoState};

// This struct defines the data that will be passed to our shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct LambertMaterial {
    #[uniform(0)]
    color: Vec3,
    #[uniform(1)]
    light_pos: Vec3,
    #[uniform(2)]
    intensity: f32,
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for LambertMaterial {
    // Add UI input handling
    // Start adding more fun stuff
    fn fragment_shader() -> ShaderRef {
        "shaders/lambert_material.wgsl".into()
    }

    fn vertex_shader() -> ShaderRef {
        "shaders/lambert_material.wgsl".into()
    }
}

#[derive(Component)]
pub struct Light;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct LightMaterial {}

impl Material for LightMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/light.wgsl".into()
    }
}

pub struct RenderersPlugin;

impl Plugin for RenderersPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        // Load shaders
        app.add_plugins((
            MaterialPlugin::<LambertMaterial>::default(),
            MaterialPlugin::<LightMaterial>::default(),
        ));

        // Load systems
        app.add_systems(OnEnter(DemoState::Renderer), (setup, spawn_light));
        app.add_systems(Update, (set_light_pos).run_if(in_state(DemoState::Renderer)));
        app.add_systems(OnExit(DemoState::Renderer), clean_up);
    }
}

#[derive(Component)]
struct Renderer;

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LambertMaterial>>,
    ui_mat: Res<MaterialSettings>,
    _asset_server: Res<AssetServer>,
) {
    // Cube mesh
    commands.spawn((
        Renderer,
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(LambertMaterial {
            color: Vec3::from_array(ui_mat.color),
            light_pos: Vec3::new(0.8, 1.0, 0.5),
            intensity: 0.5,
        })),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    commands.spawn(PointLight {
        ..Default::default()
    });

    commands.spawn((
        Renderer,
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn clean_up(
    mut commands: Commands,
    query: Query<Entity, With<Renderer>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn spawn_light(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LightMaterial>>,
    ui_light: Res<LightSettings>,
) {
    // Light mesh
    commands.spawn((
        Renderer, // Clean up
        Light,
        Mesh3d(meshes.add(Sphere { radius: 0.1 })),
        MeshMaterial3d(materials.add(LightMaterial {})),
        Transform::from_xyz(ui_light.pos[0], ui_light.pos[1], ui_light.pos[2]),
    ));
}

fn set_light_pos(
    mut materials: ResMut<Assets<LambertMaterial>>,
    light: Res<LightSettings>,
    material_settings: Res<MaterialSettings>,
    mut query: Query<&mut Transform, With<Light>>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        for (_, material) in materials.iter_mut() {
            *transform = transform.with_translation(light.pos.into());
            material.light_pos = Vec3::from_array(light.pos);
            material.intensity = light.intensity;
            material.color = material_settings.color.into();
        }
    }
}

