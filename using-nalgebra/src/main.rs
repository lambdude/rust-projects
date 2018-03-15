extern crate nalgebra;
// extern crate tau;

use nalgebra::{Vector2, Vector3, Matrix2, Point2, Rotation2};

fn main() {
    // Basic operations
    let v = Vector2::new(0.0f64, 1.0);
    let rot = Matrix2::new(0.0f64, -1.0, 1.0, 0.0);
    println!("{:?}", rot * v);

    // let angle = tau::TAU / 4.0;
    // let rotated = Rotation2::new(Vector1::new(angle));
    // println!("{:?}", rotated * v);

    // let point = Point2::new(4.0f64, 4.0);
    // println!("Transate from {:?} to {:?}",
    //     &point,
    //     nalgebra::translate(&v, &point));

    // Dot and cross product
    // let v1 = Vector3::new(2.0f64, 2.0, 0.0);
    // let v2 = Vector3::new(2.0f64, -2.0, 0.0);
    // if nalgebra::approx_eq(&0.0f64, &nalgebra::dot(&v1, &v2)) {
    //     println!("v1 is orthogonal to v2");
    // }
}
