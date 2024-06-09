pub mod Affine_point;
mod binary;
use Affine_point::prime_field_13::F13;
use Affine_point::Affine_point as point;
// this is to check if the affince point is in Tjj_13
pub fn is_point(coordinates: &point) -> bool {
    // this is the point at infinity since the point on infinity is still a point though it doesnt satisfy the equation
    if coordinates.x() == 0 && coordinates.y() == 0 {
        return true;
    }
    // this is an imaginary point , the point at infinity if you will

    // Tjj-13  (tiny jubjub13 ) this is and elliptic curve of where a = 8 and b = 8 over the the prime field of 13
    // the elliptic curve is hence y^2= x^3 + 8x + 8
    let left_side = coordinates.y_F13().sqr();
    let (a, b) = (F13::new(8), F13::new(8));
    let right_hand_side = coordinates
        .x_F13()
        .cube()
        .add(&&coordinates.x_F13().mul(&a))
        .add(&b);
    // println!("{:?} and right side is {:?}", left_side, right_hand_side);
    left_side.is_eq(&right_hand_side)
}
