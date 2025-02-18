use bevy::math::Vec2;
use delaunator::{triangulate, Triangulation, EMPTY};

use super::Points;

pub struct Triangle {
    pub(super) edge: usize,
    pub(super) start: Vec2,
    pub(super) end: Vec2,
}

pub type Triangles = Vec<Triangle>;

pub struct Voronoitor;

impl Voronoitor {
    fn gen_triangles() {}

    #[inline(always)]
    fn edges_of_triangle(self, t: usize) -> [usize; 3] {
        [3 * t, 3 * t + 1, 3 * t + 2]
    }

    #[inline(always)]
    fn triangle_of_edges(self, edge: usize) -> f64 {
        ((edge as f64) / 3.0).floor()
    }

    #[inline(always)]
    fn next_halfedge(&self, edge: usize) -> usize {
        if edge % 3 == 2 {
            edge - 2
        } else {
            edge + 1
        }
    }

    #[inline(always)]
    fn prev_halfedge(self, edge: usize) -> usize {
        if edge % 3 == 0 {
            edge + 2
        } else {
            edge - 1
        }
    }

    pub fn triangle_iter(&self, points: &Points) -> Vec<Vec2> {
        let Triangulation {
            triangles,
            halfedges,
            hull: _,
        } = triangulate(&points);

        triangles
            .iter()
            .map(|e| Vec2 {
                x: points[*e].x as f32,
                y: points[*e].y as f32,
            })
            .collect::<Vec<Vec2>>()
    }
}
