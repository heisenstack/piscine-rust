mod areas_volumes;
pub use crate::areas_volumes::*;
pub fn area_fit(
    (x, y): (usize, usize),
    kind: areas_volumes::GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool {
    match kind {
        areas_volumes::GeometricalShapes::Square => {
            let area = areas_volumes::square_area(a);
            area * times <= x * y
        }
        areas_volumes::GeometricalShapes::Circle => {
            let area = areas_volumes::circle_area(a);
            area * (times as f64) <= (x * y) as f64
        }
        areas_volumes::GeometricalShapes::Rectangle => {
            let area = areas_volumes::rectangle_area(a, b);
            area * times <= x * y
        }
        areas_volumes::GeometricalShapes::Triangle => {
            let area = areas_volumes::triangle_area(a, b);
            area * (times as f64) <= (x * y) as f64
        }
    }
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: areas_volumes::GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
    match kind {
        areas_volumes::GeometricalVolumes::Cube => {
            let volume = areas_volumes::cube_volume(a);
            volume * times <= x * y * z
        }
        areas_volumes::GeometricalVolumes::Sphere => {
            let volume = areas_volumes::sphere_volume(a);
            volume * (times as f64) <= (x * y * z) as f64
        }
        areas_volumes::GeometricalVolumes::Cone => {
            let volume = areas_volumes::cone_volume(a, b);
            volume * (times as f64) <= (x * y * z) as f64
        }
        areas_volumes::GeometricalVolumes::TriangularPyramid => {
            let base_area = areas_volumes::triangle_area(a, b);
            let volume = areas_volumes::triangular_pyramid_volume(base_area, c);
            volume * (times as f64) <= (x * y * z) as f64
        }
        areas_volumes::GeometricalVolumes::Parallelepiped => {
            let volume = areas_volumes::parallelepiped_volume(a, b, c);
            volume * times <= x * y * z
        }
    }
}
