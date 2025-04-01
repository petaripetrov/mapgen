mod utils;

use bevy::{
    asset::RenderAssetUsages,
    math::{vec2, vec3},
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

use delaunator::Point;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use utils::assign_elevation;
use voronoice::{BoundingBox, VoronoiBuilder};

use crate::state::RegenCells;

#[derive(Component)]
pub struct Cell((usize, Handle<ColorMaterial>));

#[derive(Resource, Reflect)]
pub struct MapgenSettings {
    rng_seed: u64,
    grid_size: usize,
    jitter: f64,
    elevation_threshold: f64,
}

#[derive(Resource, Default)]
pub struct Elevation(Vec<f64>);

pub struct MapgenPlugin;

impl Plugin for MapgenPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(MapgenSettings {
            rng_seed: 0xDEADBEEF,
            grid_size: 20,
            jitter: 1.0,
            elevation_threshold: 0.65,
        });
        app.insert_resource(Elevation::default());

        app.add_systems(Startup, setup);

        app.add_systems(Update, (gen_circles, update_height_material));
    }
}

// TODO refactor
fn gen_circles(
    mut commands: Commands,
    mut events: EventReader<RegenCells>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut elevation_res: ResMut<Elevation>,
    mapgen_settings: Res<MapgenSettings>,
    query: Query<Entity, With<Cell>>,
) {
    for _ in events.read() {
        let mut rng = ChaCha8Rng::seed_from_u64(mapgen_settings.rng_seed);

        let size: usize = mapgen_settings.grid_size;
        let mut points = Vec::<Point>::new();
        let jitter = mapgen_settings.jitter;

        let center = Point {
            x: (size / 2) as f64,
            y: (size / 2) as f64,
        };

        for x in 0..size {
            for y in 0..size {
                let offset_x = rng.random::<(f64, f64)>();
                let offset_y = rng.random::<(f64, f64)>();

                let new_point = Point {
                    x: (x as f64) + jitter * (offset_x.0 - offset_x.1),
                    y: (y as f64) + jitter * (offset_y.0 - offset_y.1),
                };

                points.push(new_point);
            }
        }

        query
            .iter()
            .for_each(|entity| commands.entity(entity).despawn());

        let elevation = assign_elevation(&points, mapgen_settings.rng_seed as u32);

        let my_voronoi = VoronoiBuilder::default()
            .set_sites(points)
            .set_bounding_box(BoundingBox::new(
                center,
                mapgen_settings.grid_size as f64,
                mapgen_settings.grid_size as f64,
            ))
            .set_lloyd_relaxation_iterations(50)
            .build()
            .unwrap();

        for (idx, cell) in my_voronoi.iter_cells().enumerate() {
            let mut mesh = Mesh::new(
                PrimitiveTopology::TriangleList,
                RenderAssetUsages::RENDER_WORLD,
            );

            let vertices: Vec<Vec3> = cell
                .iter_vertices()
                .map(|vertex| Vec3 {
                    x: vertex.x as f32,
                    y: vertex.y as f32,
                    z: 0.0,
                })
                .collect();

            let mut indices = vec![];
            for i in 1..vertices.len() {
                indices.extend_from_slice(&[0, i as u32, i as u32 - 1]);
            }

            let color = if elevation[idx] < mapgen_settings.elevation_threshold {
                Color::hsl(240.0, 0.3, 0.5)
            } else {
                Color::hsl(90.0, 0.3, 0.5)
            };

            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
            mesh.insert_indices(Indices::U32(indices));

            let material_handle = materials.add(ColorMaterial::from_color(color));

            commands.spawn((
                Cell((idx, material_handle.clone())),
                Mesh2d(meshes.add(mesh)),
                MeshMaterial2d(material_handle),
            ));
        }

        elevation_res.0 = elevation;
    }
}

fn update_height_material(
    query: Query<&Cell>,
    elevation: Res<Elevation>,
    mapgen_settings: Res<MapgenSettings>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // let elevation = assign_elevation(points, seed)
    for cell in query.iter() {
        let (idx, handle) = &cell.0;
        let color = materials.get_mut(handle);

        if let Some(material) = color {

            let new_color = if elevation.0[*idx] < mapgen_settings.elevation_threshold {
                Color::hsl(240.0, 0.3, 0.5)
            } else {
                Color::hsl(90.0, 0.3, 0.5)
            };

            material.color = new_color;
        }
    }
}

fn setup(mut commands: Commands, mut events: EventWriter<RegenCells>) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scale: 0.1,
            viewport_origin: vec2(0.6, 0.5),
            ..OrthographicProjection::default_2d()
        },
        Transform {
            translation: vec3(12.5, 10.0, 0.0),
            scale: vec3(0.4, 0.4, 1.0),
            ..Default::default()
        },
    ));

    events.send_default();
}
