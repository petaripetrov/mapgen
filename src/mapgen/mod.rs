mod voronoitor;

use std::f32::consts::PI;

use bevy::{
    asset::RenderAssetUsages,
    color::palettes::css::{RED, WHITE},
    math::{vec2, vec3},
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

use delaunator::Point;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use voronoitor::Voronoitor;

use crate::state::RegenCells;

// TODO Move to Voronoitor file
#[derive(Resource, Deref, DerefMut)]
struct Points(Vec<Point>);

#[derive(Component)]
pub struct Cell;

#[derive(Resource, Reflect)]
pub struct MapgenSettings {
    rng_seed: u64,
}

pub struct MapgenPlugin;

impl Plugin for MapgenPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(Voronoitor::default());
        app.insert_resource(MapgenSettings {
            rng_seed: 0xDEADBEEF,
        });

        app.add_systems(Startup, setup);

        app.add_systems(Update, (draw_circles, draw_cells, gen_circles));
    }
}

fn draw_circles(voronoitor: Res<Voronoitor>, mut gizmos: Gizmos) {
    for Point { x, y } in &voronoitor.points {
        gizmos.arc_2d(vec2(*x as f32, *y as f32), 2.0 * PI, 0.05, RED);
    }
}

fn gen_circles(
    mut commands: Commands,
    mut events: EventReader<RegenCells>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mapgen_settings: Res<MapgenSettings>,
    query: Query<Entity, With<Cell>>,
) {
    for _ in events.read() {
        let mut rng = ChaCha8Rng::seed_from_u64(mapgen_settings.rng_seed);

        const SIZE: usize = 25;
        let mut points = Vec::<Point>::new();
        const JITTER: f64 = 0.5;

        for x in 0..SIZE {
            for y in 0..SIZE {
                let offset_x = rng.random::<(f64, f64)>();
                let offset_y = rng.random::<(f64, f64)>();

                points.push(Point {
                    x: (x as f64) + JITTER * (offset_x.0),
                    y: (y as f64) + JITTER * (offset_y.0),
                });
            }
        }

        let voronoitor = Voronoitor::new(points);
        let elevation = voronoitor.assign_elevation(mapgen_settings.rng_seed as u32);

        query
            .iter()
            .for_each(|entity| commands.entity(entity).despawn());

        voronoitor.cell_iter().iter().for_each(|voronoi_cell| {
            let mut mesh = Mesh::new(
                PrimitiveTopology::TriangleList,
                RenderAssetUsages::RENDER_WORLD,
            );

            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, voronoi_cell.vertices.clone());

            let mut indices = vec![];
            for i in 1..voronoi_cell.vertices.len() {
                indices.extend_from_slice(&[0, i as u32, i as u32 - 1]);
            }

            mesh.insert_indices(Indices::U32(indices));

            let color = if elevation[voronoi_cell.edge] < 0.5 {
                Color::hsl(240.0, 0.3, 0.5)
            } else {
                Color::hsl(90.0, 0.2, 0.5)
            };

            commands.spawn((
                Cell,
                Mesh2d(meshes.add(mesh)),
                MeshMaterial2d(materials.add(ColorMaterial::from_color(color))),
            ));
        });

        commands.insert_resource(voronoitor);
    }
}

fn draw_cells(voronoitor: Res<Voronoitor>, mut gizmos: Gizmos) {
    voronoitor.voronoi_iter().iter().for_each(|triangle| {
        gizmos.line_2d(triangle.start, triangle.end, WHITE);
    });
}

fn setup(mut commands: Commands, mut events: EventWriter<RegenCells>) {
    let mut translation = Transform::from_translation(vec3(10.0, 10.0, 0.0));
    translation.scale = vec3(0.1, 0.1, 0.1);

    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scale: 0.2,
            ..OrthographicProjection::default_2d()
        },
        Transform {
            translation: vec3(6.0, 12.0, 0.0),
            scale: vec3(0.2, 0.2, 1.0),
            ..Default::default()
        },
    ));

    events.send_default();
}
