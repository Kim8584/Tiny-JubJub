use Tjj_13::Affine_point::prime_field_13::*;
use Tjj_13::Affine_point::Affine_point as Point;
mod binary;
use binary::Binary;

// this is to get all points in Tjj_13
// F13 * F13 -> Affine_point this means that the limitation for x and y is within the range of 0..13(0..=12) and
// the technique used to get all the points is naive test for all values in F13 for both x and y and put the points in
// a vec![affince_point]
pub struct Tjj13Points;
impl Tjj13Points {
    fn all_points() -> Vec<Point> {
        let mut Tjj_13_points: Vec<Point> = Vec::new();
        // for the x cordinate which is of F13 and contains only 13 elements 0..13 (0..=12)
        for i in 0..13 {
            // for the y cordinate which is of F13 still
            for j in 0..13 {
                let point = Point::new_from_i32(i, j);
                if Tjj_13::is_point(&point) {
                    Tjj_13_points.push(point);
                }
            }
        }
        return Tjj_13_points;
    }
}
// this has euclide algorithms used in the project

fn main() {
    let a = 13;
    println!("{:?}", Binary::to_binary(a));
    let points = Tjj13Points::all_points();
    println!("the curve as {} points ", points.len());
    for i in points.iter().by_ref() {
        i.print_affine_point();
    }
    let point_a = Point::new_from_i32(8, 5);
    let point_b = Point::new_from_i32(9, 4);
    let a_plus_b = point_a.add(&point_b);
    println!(
        "{} + {} = {}",
        point_a.to_string(),
        point_b.to_string(),
        a_plus_b.to_string()
    );
    println!(
        "point a is {} and point a * 2 is {}",
        point_a.to_string(),
        point_a.double().to_string()
    );
    println!(
        "point b is {} and point b * 2 is {}",
        point_b.to_string(),
        point_b.double().to_string()
    );
    let another_point = Point::new_from_i32(1, 11);
    println!(
        "point a is {} and a * 7 = {}",
        another_point.to_string(),
        another_point.mul(7).to_string()
    );
}
