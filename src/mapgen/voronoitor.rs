use bevy::math::Vec2;
use delaunator::{triangulate, Triangulation, EMPTY};

use super::Points;

pub struct Triangle {
    pub(super) start: Vec2,
    pub(super) end: Vec2,
}

pub type Triangles = Vec<Triangle>;

pub struct Voronoitor;

impl Voronoitor {
    #[inline(always)]
    fn triangle_of_edges(&self, edge: usize) -> usize {
        edge / 3
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
    pub fn calc_centroids(
        &self,
        points: &Points,
        halfedges: &Vec<usize>,
        triangles: &Vec<usize>,
    ) -> Vec<Vec2> {
        let num_triangles = halfedges.len() / 3;
        let mut centroids = vec![];

        for t in 0..num_triangles {
            let mut sum_x = 0.0f32;
            let mut sum_y = 0.0f32;

            for _ in 0..3 {
                let s = 3 * t + 1;
                let p = &points[triangles[s]];

                sum_x += p.x as f32;
                sum_y += p.y as f32;
            }

            centroids.push(Vec2 {
                x: sum_x / 3.0,
                y: sum_y / 3.0,
            });
        }

        centroids
    }

    #[allow(unused)] // TODO TEMP add an option to visualize just the triangles
    pub fn triangle_iter(&self, points: &Points) -> Triangles {
        let Triangulation {
            triangles,
            halfedges,
            hull: _,
        } = triangulate(&points);

        (0..triangles.len()).fold(Vec::new(), |mut acc, e| {
            if e > halfedges[e] {
                let start = &points[triangles[e]];
                let end = &points[triangles[self.next_halfedge(e)]];

                acc.push(Triangle {
                    start: Vec2 {
                        x: start.x as f32,
                        y: start.y as f32,
                    },
                    end: Vec2 {
                        x: end.x as f32,
                        y: end.y as f32,
                    },
                });

                acc
            } else {
                acc
            }
        })
    }

    pub fn voronoi_iter(&self, points: &Points) -> Triangles {
        // TODO test against other implementations
        let Triangulation {
            triangles,
            halfedges,
            hull: _,
        } = triangulate(&points);

        let centers = self.calc_centroids(points, &halfedges, &triangles);

        (0..halfedges.len()).fold(Vec::new(), |mut acc, edge| {
            if edge < halfedges[edge] && halfedges[edge] != EMPTY {
                let p = centers[self.triangle_of_edges(edge)];
                let q = centers[self.triangle_of_edges(halfedges[edge])];

                acc.push(Triangle { start: p, end: q });

                return acc;
            }

            acc
        })
    }
}
