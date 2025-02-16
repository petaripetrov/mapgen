use std::f32::consts::PI;

use bevy::{
    color::palettes::css::{RED, WHITE},
    math::{vec2, vec3},
    prelude::*,
};
use delaunator::{triangulate, Point};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::state::NumCellsUpdated;

#[derive(Resource, Deref, DerefMut)]
struct Points(Vec<Point>);

#[derive(Resource, Default)]
pub struct CellMap {
    // TODO maybe shorten this somehow
    // perpahse have only one function that handles this
    // num_triangles: usize,
    // num_edges: usize,
    halfedges: Vec<usize>,
    triangles: Vec<usize>,
    centers: Vec<Point>,
}

#[derive(Resource)]
struct RNG(ChaCha8Rng);

pub struct MapgenPlugin;

impl Plugin for MapgenPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(Points(Vec::new()));
        app.insert_resource(CellMap::default());

        app.add_event::<NumCellsUpdated>();

        app.add_systems(Startup, setup);

        app.add_systems(Update, (draw_circles, draw_cells, gen_circles));

        app.add_systems(Update, (gen_cells).run_if(resource_changed::<Points>));
        // app.add
    }
}

fn draw_circles(point_res: Res<Points>, mut gizmos: Gizmos) {
    for Point { x, y } in &point_res.0 {
        gizmos.arc_2d(vec2(*x as f32, *y as f32), 2.0 * PI, 0.05, RED);
    }
}

fn gen_circles(
    mut rng_src: ResMut<RNG>,
    mut point_res: ResMut<Points>,
    mut events: EventReader<NumCellsUpdated>,
) {
    for _ in events.read() {
        let grid_size: usize = 25;
        let mut points = Vec::<Point>::new();
        const JITTER: f64 = 0.5;

        for x in 0..grid_size {
            for y in 0..grid_size {
                let offset_x = rng_src.0.random::<(f64, f64)>();
                let offset_y = rng_src.0.random::<(f64, f64)>();

                points.push(Point {
                    x: (x as f64) + JITTER * (offset_x.0 - offset_x.1),
                    y: (y as f64) + JITTER * (offset_y.0 - offset_y.1),
                });
            }
        }

        *point_res.as_deref_mut() = points;
    }
}

#[inline(always)]
fn next_halfedge(i: usize) -> usize {
    if i % 3 == 0 {
        i + 2
    } else {
        i - 1
    }
}

fn draw_cells(cell_map: Res<CellMap>, mut gizmos: Gizmos) {
    let edges = &cell_map.halfedges;
    let triangles = &cell_map.triangles;
    let centers = &cell_map.centers;

    for i in 0..edges.len() {
        if i > edges[i] {
            let start = &centers[triangles[i]];
            let end = &centers[triangles[next_halfedge(i)]];

            gizmos.line_2d(
                Vec2 {
                    x: start.x as f32,
                    y: start.y as f32,
                },
                Vec2 {
                    x: end.x as f32,
                    y: end.y as f32,
                },
                WHITE,
            );
        }
    }
}

fn gen_cells(points: Res<Points>, mut cell_map: ResMut<CellMap>) {
    if points.len() > 0 {
        let result = triangulate(&points);

        let num_triangles = result.halfedges.len() / 3;
        let mut centroids = Vec::new();

        for i in 0..num_triangles {
            let mut sum_x = 0.0;
            let mut sum_y = 0.0;

            for j in 0..3 {
                let s = 3 * i + j;
                let p = &points[result.triangles[s]];

                sum_x += p.x as f64;
                sum_y += p.y as f64;
            }
            centroids.push(Point {
                x: sum_x / 3.0,
                y: sum_y / 3.0,
            });
        }

        *cell_map = CellMap {
            halfedges: result.halfedges,
            triangles: result.triangles,
            centers: centroids,
        };
    }
}

fn setup(mut commands: Commands, mut events: EventWriter<NumCellsUpdated>) {
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

    let seeded_rng = ChaCha8Rng::seed_from_u64(0xDEADBEEF);

    commands.insert_resource(RNG(seeded_rng));
    events.send_default();
}
