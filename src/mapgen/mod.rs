use bevy::{
    app::{Plugin, Update},
    asset::{Assets, Handle},
    color::Color,
    core_pipeline::core_2d::Camera2d,
    ecs::system::{Commands, Query, Res, ResMut, Resource},
    math::{
        primitives::Circle, Vec3,
    },
    render::mesh::{Mesh, Mesh2d, MeshBuilder, Meshable},
    sprite::{ColorMaterial, MeshMaterial2d},
    state::state::OnEnter,
    transform::components::Transform,
    window::Window,
};
use bevy_egui::egui::emath::Numeric;
use rand::{
    distr::Uniform,
    Rng, SeedableRng,
};
use rand_chacha::ChaCha8Rng;

use crate::{ui::MapgenSettings, DemoState};

#[derive(Resource)]
struct Points(Vec<Vec3>);

#[derive(Resource)]
struct RNG(ChaCha8Rng);

pub struct MapgenPlugin;

impl Plugin for MapgenPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        let seeded_rng = ChaCha8Rng::seed_from_u64(0xDEADBEEF);

        app.add_systems(OnEnter(DemoState::Mapgen), setup);

        app.insert_resource(Points(Vec::new()));
        app.insert_resource(RNG(seeded_rng));
        app.add_systems(Update, draw_circles);
        // app.add
    }
}

// const X_EXTENT: f32 = 900.;

fn draw_circles(
    mut commands: Commands,
    point_res: Res<Points>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    const CIRCLE: Circle = Circle { radius: 5.0 };
    let material: Handle<ColorMaterial> = materials.add(Color::WHITE);

    for position in &point_res.0 {
        commands.spawn((
            Mesh2d(meshes.add(CIRCLE.mesh().build())),
            MeshMaterial2d(material.clone()),
            Transform::from_translation(*position),
        ));
    }
}

// fn test(mut commands: Commands, mut gizmos: Gizmos, mut meshes: ResMut<Assets<Mesh>>,  mut materials: ResMut<Assets<ColorMaterial>>,) {
//     const POSITION: Vec2 = Vec2::new(-200.0, 0.0);
//     const POSITION_B: Vec3 = Vec3::new(200.0, 0.0, 0.0);

//     // const LINE2D: Segment2d = Segment2d { direction: Dir2::X, half_length: 100.0 };
//     // const SPHERE: Sphere = Sphere {radius: 1.0};
//     let isometry = Isometry2d::new(POSITION, Rot2::IDENTITY);
//     let color = Color::WHITE;
//     // gizmos.primitive_2d(&LINE2D, isometry, color);
//     gizmos.primitive_2d(&CRICLE, isometry, color);

//     commands.spawn((
//         Mesh2d(meshes.add((CRICLE.mesh().build()))),
//         MeshMaterial2d(material.clone()),
//         Transform::from_translation(POSITION_B)
//     ));
// }

fn setup(
    mut commands: Commands,
    mut rng_src: ResMut<RNG>,
    mut point_res: ResMut<Points>,
    map_settings: Res<MapgenSettings>
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
    // mut gizmos: Gizmos,
) {
    let n = map_settings.num_cells;
    let rng = &mut rng_src.0;

    commands.spawn(Camera2d);

    let uniform = Uniform::<f32>::new(-1.0, 1.0);

    if let Ok(distr) = uniform {
        let x_iter = rng.clone().sample_iter(distr);
        let y_iter = rng.sample_iter(distr).skip(n);

        let points_iter = x_iter
            .zip(y_iter)
            .take(n)
            .map(|(x, y)| Vec3::new(x * 300.0, y * 300.0, 0.0));

        point_res.0.extend(points_iter);
    }

    // info!("{:?}", point_res.0.len());

    // let shapes = [
    // meshes.add(Circle::new(50.0)),
    // meshes.add(CircularSector::new(50.0, 1.0)),
    // meshes.add(Segment2d::new(Dir2::new(Vec2::new(0.0, 1.0)))),
    // meshes.add(CircularSegment::new(50.0, 1.25)),
    // meshes.add(Ellipse::new(25.0, 50.0)),
    // meshes.add(Annulus::new(25.0, 50.0)),
    // meshes.add(Capsule2d::new(25.0, 50.0)),
    // meshes.add(Rhombus::new(75.0, 100.0)),
    // meshes.add(Rectangle::new(50.0, 100.0)),
    // meshes.add(RegularPolygon::new(50.0, 6)),
    // meshes.add(Triangle2d::new(
    //     Vec2::Y * 50.0,
    //     Vec2::new(-50.0, -50.0),
    //     Vec2::new(50.0, -50.0),
    // )),
    // ];
    // let num_shapes = shapes.len();

    // for (i, shape) in shapes.into_iter().enumerate() {
    //     // Distribute colors evenly across the rainbow.
    //     let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);

    //     commands.spawn((
    //         Mesh2d(shape),
    //         MeshMaterial2d(materials.add(color)),
    //         Transform::from_xyz(
    //             // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
    //             -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
    //             0.0,
    //             0.0,
    //         ),
    //     ));
    // }
}
