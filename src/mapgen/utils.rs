use noise::{NoiseFn, Simplex};
use voronoice::Point;

pub fn assign_elevation(points: &Vec<Point>, seed: u32) -> Vec<f64> {
    let simplex = Simplex::new(seed);
    let num_regions = points.len();
    let mut elevation = Vec::new();

    for i in 0..num_regions {
        let nx = points[i].x / 25.0 - 1.0 / 2.0;
        let ny = points[i].y / 25.0 - 1.0 / 2.0;

        let noise = simplex.get([nx / 0.5, ny / 0.5]) / 2.0;

        elevation.push(1.0 + noise);

        let d = 2.0 * f64::max(f64::abs(nx), f64::abs(ny));
        elevation[i] = (1.0 + elevation[i] - d) / 2.0;
    }

    elevation
}