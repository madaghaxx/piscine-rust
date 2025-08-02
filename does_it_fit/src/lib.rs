pub mod areas_volumes;
pub use areas_volumes::*;

pub fn area_fit(
    (x, y): (usize, usize),
    kind: GeometricalShapes,
    times: usize,
    (a, b): (usize, usize)
) -> bool {
    let area_area = rectangle_area(x, y);
    match kind {
        GeometricalShapes::Square => square_area(a) * times <= area_area,
        GeometricalShapes::Circle => (circle_area(a).ceil() as usize) * times <= area_area,
        GeometricalShapes::Rectangle => rectangle_area(a, b) * times <= area_area,
        GeometricalShapes::Triangle => (triangle_area(a, b).ceil() as usize) * times <= area_area,
    }
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize)
) -> bool {
    let container_volume = x * y * z;
    match kind {
        GeometricalVolumes::Cube => cube_volume(a) * times <= container_volume,
        GeometricalVolumes::Sphere =>
            (sphere_volume(a).ceil() as usize) * times <= container_volume,
        GeometricalVolumes::Cone => (cone_volume(a, b).ceil() as usize) * times <= container_volume,
        GeometricalVolumes::TriangularPyramid => {
            (triangular_pyramid_volume(triangle_area(a, b), c).ceil() as usize) * times <=
                container_volume
        }
        GeometricalVolumes::Parallelepiped =>
            parallelepiped_volume(a, b, c) * times <= container_volume,
    }
}
