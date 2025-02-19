mod voronoitor;

use std::f32::consts::PI;

use bevy::{
    color::palettes::css::{RED, WHITE},
    math::{vec2, vec3},
    prelude::*,
};

use delaunator::Point;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use voronoitor::Voronoitor;

use crate::state::RegenCells;

// TODO Move to Voronoitor file
#[derive(Resource, Deref, DerefMut)]
struct Points(Vec<Point>);

#[derive(Resource, Reflect)]
pub struct MapgenSettings {
    rng_seed: u64,
}

pub struct MapgenPlugin;

impl Plugin for MapgenPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(Points(Vec::new()));
        app.insert_resource(MapgenSettings {
            rng_seed: 0xDEADBEEF,
        });

        app.add_systems(Startup, setup);

        app.add_systems(Update, (draw_circles, draw_cells, gen_circles));
    }
}

fn draw_circles(point_res: Res<Points>, mut gizmos: Gizmos) {
    for Point { x, y } in &point_res.0 {
        gizmos.arc_2d(vec2(*x as f32, *y as f32), 2.0 * PI, 0.05, RED);
    }
}

fn gen_circles(
    mut point_res: ResMut<Points>,
    mut events: EventReader<RegenCells>,
    mapgen_settings: Res<MapgenSettings>
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
                    x: (x as f64) + JITTER * (offset_x.0 - offset_x.1),
                    y: (y as f64) + JITTER * (offset_y.0 - offset_y.1),
                });
            }
        }

        *point_res.as_deref_mut() = points;
    }
}

fn draw_cells(points: Res<Points>, mut gizmos: Gizmos) {
    let voronoitor = Voronoitor;

    voronoitor
        .voronoi_iter(&points)
        .iter()
        .for_each(|triangle| {
            gizmos.line_2d(triangle.start, triangle.end, WHITE);
        });
}

fn setup(
    mut commands: Commands,
    mut events: EventWriter<RegenCells>,
) {
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
