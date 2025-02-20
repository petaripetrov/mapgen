use bevy::{ecs::system::Resource, math::{vec3, Vec2, Vec3}, utils::hashbrown::HashSet};
use delaunator::{triangulate, Point, Triangulation, EMPTY};
use noise::{NoiseFn, Simplex};

use super::Points;

pub struct Triangle {
    pub(super) start: Vec2,
    pub(super) end: Vec2,
}

pub struct VoronoiCell {
    pub vertices: Vec<Vec3>,
    pub edge: usize
}

pub type Triangles = Vec<Triangle>;

#[derive(Resource, Default)]
pub struct Voronoitor {
    pub points: Vec<Point>,
    pub triangulation: Option<Triangulation>,
}

impl Voronoitor {
    pub fn new(points: Vec<Point>) -> Voronoitor {
        Voronoitor {
            points: points.clone(),
            triangulation: Some(triangulate(&points)),
        }
    }

    #[inline(always)]
    pub fn num_edges(&self) -> usize {
        if let Some(Triangulation {
            triangles: _,
            halfedges,
            hull: _,
        }) = &self.triangulation
        {
            halfedges.len()
        } else {
            0
        }
    }

    #[inline(always)]
    fn triangle_of_edges(&self, edge: usize) -> usize {
        edge / 3
    }

    #[inline(always)]
    pub fn next_halfedge(&self, edge: usize) -> usize {
        if edge % 3 == 2 {
            edge - 2
        } else {
            edge + 1
        }
    }

    // TODO Maybe call this once when creating the whole thing and reuse the value
    #[inline(always)]
    pub fn calc_centroids(&self) -> Vec<Vec2> {
        if let Some(Triangulation {
            triangles,
            halfedges,
            hull: _,
        }) = &self.triangulation
        {
            let num_triangles = halfedges.len() / 3;
            let mut centroids = vec![];

            for t in 0..num_triangles {
                let mut sum_x = 0.0f32;
                let mut sum_y = 0.0f32;

                for _ in 0..3 {
                    let s = 3 * t + 1;
                    let p = &self.points[triangles[s]];

                    sum_x += p.x as f32;
                    sum_y += p.y as f32;
                }

                centroids.push(Vec2 {
                    x: sum_x / 3.0,
                    y: sum_y / 3.0,
                });
            }

            centroids
        } else {
            Vec::new()
        }
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

    // TODO keep a copy of Points on the struct
    pub fn voronoi_iter(&self) -> Triangles {
        // TODO test against other implementations
        if let Some(Triangulation {
            triangles: _,
            halfedges,
            hull: _,
        }) = &self.triangulation
        {
            let centers = self.calc_centroids();

            (0..halfedges.len()).fold(Vec::new(), |mut acc, edge| {
                if edge < halfedges[edge] && halfedges[edge] != EMPTY {
                    let p = centers[self.triangle_of_edges(edge)];
                    let q = centers[self.triangle_of_edges(halfedges[edge])];

                    acc.push(Triangle { start: p, end: q });

                    return acc;
                }

                acc
            })
        } else {
            Vec::new()
        }
    }

    pub fn cell_iter(&self) -> Vec<VoronoiCell> {
        let mut res = Vec::new();
        let mut seen: HashSet<usize> = HashSet::new();

        if let Some(Triangulation {
            triangles,
            halfedges: _,
            hull: _,
        }) = &self.triangulation
        {
            let num_edges = self.num_edges();
            let centers = self.calc_centroids();

            for e in 0..num_edges {
                let r = triangles[self.next_halfedge(e)];

                if !seen.contains(&r) {
                    seen.insert(r);

                    let vertices: Vec<Vec3> = self
                        .edges_around_point(e)
                        .iter()
                        .map(|&edge| {
                            let Vec2 { x, y } = centers[self.triangle_of_edges(edge)];

                            vec3(x, y, 1.0)
                        })
                        .collect();

                    res.push(VoronoiCell {
                        vertices,
                        edge: r
                    });
                }
            }
        }

        res
    }

    pub fn assign_elevation(&self, seed: u32) -> Vec<f64> {
        let simplex = Simplex::new(seed);
        let num_regions = self.points.len();
        let mut elevation = Vec::new();

        for i in 0..num_regions {
            let nx = self.points[i].x / 25.0 - 1.0 / 2.0;
            let ny = self.points[i].y / 25.0 - 1.0 / 2.0;

            let noise = simplex.get([nx / 0.5, ny / 0.5]) / 2.0;

            elevation.push(1.0 + noise);

            let d = 2.0 * f64::max(f64::abs(nx), f64::abs(ny));
            elevation[i] = (1.0 + elevation[i] - d) / 2.0;
        }

        elevation
    }

    // TODO add a type for Vec<usize> Edges
    pub fn edges_around_point(&self, start: usize) -> Vec<usize> {
        let mut res = Vec::new();
        let mut incoming = start;

        if let Some(Triangulation {
            triangles: _,
            halfedges,
            hull: _,
        }) = &self.triangulation
        {
            loop {
                res.push(incoming);
                let outgoing = self.next_halfedge(incoming);
                incoming = halfedges[outgoing];

                if incoming == EMPTY || incoming == start {
                    break;
                }
            }
        }

        res
    }
}
