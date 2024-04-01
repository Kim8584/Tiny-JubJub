use rand::Rng;
use std::{f32::INFINITY, io, panic::AssertUnwindSafe};
#[derive(Debug)]
pub struct Affine_point {
    x: F13,
    y: F13,
}
// // const ZERO_F13: F13 = F13::new(0);
// // const INFINITY: Affine_point = Affine_point {
// //     x: ZERO_F13,
// //     y: ZERO_F13,
// };
// this is finite field of 13
#[derive(Debug)]
struct F13 {
    element: i32,
}

impl F13 {
    fn new(element: i32) -> Self {
        loop {
            if element < 13 {
                break;
            }
            let element = element % 13;
        }
        return Self { element };
    }
    fn new_random() -> Self {
        let element = rand::thread_rng().gen_range(0..13);
        let element = F13::new(element);
        element
    }
    fn sqr(&self) -> Self {
        let ansa = self.element.pow(2);
        return F13::to_F13(ansa);
    }
    fn to_F13(num: i32) -> Self {
        if num > 1300 {
            let ansa = num - 13000;
            return F13::to_F13(ansa);
        }
        Self { element: num % 13 }
    }
    fn cube(&self) -> Self {
        let ansa = self.element.pow(3);
        return F13::to_F13(ansa);
    }
    fn add(&self, other: &F13) -> Self {
        let ansa = self.element + other.element;
        return F13::to_F13(ansa);
    }
    fn mul(&self, other: &F13) -> Self {
        let ansa = self.element * other.element;
        return F13::to_F13(ansa);
    }
    fn is_eq(&self, other: &F13) -> bool {
        self.element == other.element
    }
    fn copy(&self) -> Self {
        Self {
            element: self.element,
        }
    }
    // fermats llttle theorem is effictive for small numbers but quickly becomes a problem for large numbers euclid algorithm is much more effictive
    // for finding the multiplicative inverse of large numbers.
    // fn inverse(&self) -> Self {
    //     // fermats littler theorem a ^-1 = a ^ (13 - 2)
    //     if self.is_eq(&F13::to_F13(0)) {
    //         return F13::to_F13(0);
    //     }
    //     let ansa = self.element.pow(11);
    //     let ansa = F13::to_F13(ansa);
    //     // ensure that a ^ -1 * a = 1
    //     let one = F13::new(1);
    //     assert!(self.mul(&ansa).is_eq(&one));
    //     return ansa;
    // }

    // using euclid algorithm to find multiplicative inverse
    fn inverse(&self) -> Self {
        // a will always be 13 since were in group field of 13
        let mut a = 13;
        let mut b = self.element;
        let mut t_1 = 0;
        let mut t_2 = 1;

        loop {
            let c = a % b;
            if c == 0 {
                break;
            }
            let quotient = a / b;
            let t = EuclideAlgo::get_t(quotient, t_1, t_2);
            a = b;
            b = c;
            t_1 = t_2;
            t_2 = t;
        }
        return F13::new(t_2);
    }
    fn additive_inverse(&self) -> Self {
        if self.element == 0 {
            return Self { element: 0 };
        }
        let ansa = 13 - self.element;
        Self { element: ansa }
    }
    // for debugging purposes
    fn print_F13(&self) {
        println!("your ansa is {}", self.element);
    }
    fn sub(&self, other: &F13) -> Self {
        self.add(&other.additive_inverse())
    }
    fn div(&self, other: &F13) -> Self {
        self.mul(&other.inverse())
    }
}
pub struct Binary;
impl Binary {
    pub fn to_binary(number: i32) -> Vec<u8> {
        let mut number = number;
        let mut bin: Vec<u8> = Vec::new();
        while number > 0 {
            if number % 2 == 0 {
                bin.push(0);
            } else {
                bin.push(1);
            }
            number /= 2;
        }
        // reverse the order of items in the list
        return bin.iter().copied().rev().collect();
    }
}

impl Affine_point {
    fn new(x: F13, y: F13) -> Self {
        Self { x, y }
    }
    fn to_string(&self) -> String {
        format!("({},{})", self.x.element, self.y.element)
    }
    fn print_affine_point(&self) {
        println!("{}", self.to_string());
    }
    fn new_from_i32(x_i32: i32, y_i32: i32) -> Self {
        let x = F13::new(x_i32);
        let y = F13::new(y_i32);
        let point = Affine_point::new(x, y);
        // assert!(is_point(&point));
        point
    }
    fn copy(&self) -> Self {
        Self {
            x: self.x.copy(),
            y: self.y.copy(),
        }
    }
    fn is_eq(&self, other: &Self) -> bool {
        assert!(is_point(self));
        assert!(is_point(other));
        self.x.is_eq(&other.x) && self.y.is_eq(&other.y)
    }
    fn x(&self) -> i32 {
        self.x.element
    }
    fn y(&self) -> i32 {
        self.y.element
    }

    fn add(&self, other: &Affine_point) -> Self {
        assert!(is_point(self));
        assert!(is_point(other));
        if (self.x.is_eq(&other.x) && !self.y.is_eq(&other.y)) {
            return Affine_point::new_from_i32(0, 0);
        }
        if (self.y() == 0 && self.is_eq(other)) {
            return Affine_point::new_from_i32(0, 0);
        }
        // heavy on reffers to
        // (y2-y1) / (x2 - x1) which is done for both the computation of x3 and y3
        let neomerator = other.y.sub(&self.y);
        let denomenator = other.x.sub(&self.x);
        let heavy_one = neomerator.div(&denomenator);
        let x_3 = heavy_one.sqr().sub(&other.x).sub(&self.x);
        let mut y_3 = heavy_one.mul(&self.x.sub(&x_3));
        y_3 = y_3.sub(&self.y);
        Self { x: x_3, y: y_3 }
    }
    // doubling a point is just adding to itself
    fn double(&self) -> Self {
        let a = F13::new(8);
        // heavy one is 3*x + a / 2y
        // compute it separetly since it is used in both the computation of y' and x'
        let neumerator = F13::new(3).mul(&self.x.sqr()).add(&a);

        let denomenator: F13 = self.y.mul(&F13::new(2));

        let heavy_one = neumerator.div(&denomenator);

        // this is x'
        let x = heavy_one.sqr().sub(&F13::new(2).mul(&self.x));
        // this is y'
        let y = heavy_one.mul(&self.x.sub(&x)).sub(&self.y);
        Self { x, y }
    }
    // for efficiency we use the double and add method
    fn mul(&self, number: i32) -> Self {
        // doing this to reduce the amount of computation that needs to be done since
        // the number can be reduced to the prime field of 13
        let number = number % 13;
        // to get the number in binary
        let number = Binary::to_binary(number);
        let mut ansa = self.copy();
        for (i, bit) in number.iter().enumerate() {
            // we skip the first bit
            if i == 0 {
                continue;
            }
            ansa = ansa.double();
            if *bit == 1 {
                ansa = ansa.add(self);
            }
        }
        ansa
    }
}
// this is to check if the affince point is in Tjj_13
fn is_point(point: &Affine_point) -> bool {
    // this is the point at infinity since the point on infinity is still a point though it doesnt satisfy the equation
    if point.x() == 0 && point.y() == 0 {
        return true;
    }
    // this is an imaginary point , the point at infinity if you will

    // Tjj-13  (tiny jubjub13 ) this is and elliptic curve of where a = 8 and b = 8 over the the prime field of 13
    // the elliptic curve is hence y^2= x^3 + 8x + 8
    let left_side = point.y.sqr();
    let (a, b) = (F13::new(8), F13::new(8));
    let right_hand_side = point.x.cube().add(&point.x.mul(&a)).add(&b);
    // println!("{:?} and right side is {:?}", left_side, right_hand_side);
    left_side.is_eq(&right_hand_side)
}
// this is to get all points in Tjj_13
// F13 * F13 -> Affine_point this means that the limitation for x and y is within the range of 0..13(0..=12) and
// the technique used to get all the points is naive test for all values in F13 for both x and y and put the points in
// a vec![affince_point]
pub struct Tjj_13_points {}
impl Tjj_13_points {
    fn all_points() -> Vec<Affine_point> {
        let mut Tjj_13_points: Vec<Affine_point> = Vec::new();
        // for the x cordinate which is of F13 and contains only 13 elements 0..13 (0..=12)
        for i in 0..13 {
            // for the y cordinate which is of F13 still
            for j in 0..13 {
                let point = Affine_point::new_from_i32(i, j);
                if is_point(&point) {
                    Tjj_13_points.push(point);
                }
            }
        }
        return Tjj_13_points;
    }
}
// this has euclide algorithms used in the project
struct EuclideAlgo {}
impl EuclideAlgo {
    fn get_t(quotient: i32, T_1: i32, T_2: i32) -> i32 {
        return T_1 - (T_2 * quotient);
    }
}
fn main() {
    let a = 13;
    println!("{:?}", Binary::to_binary(a));
    let points = Tjj_13_points::all_points();
    println!("the curve as {} points ", points.len());
    for i in points.iter().by_ref() {
        i.print_affine_point();
    }
    let point_a = Affine_point::new_from_i32(8, 5);
    let point_b = Affine_point::new_from_i32(9, 4);
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
    let another_point = Affine_point::new_from_i32(1, 11);
    println!(
        "point a is {} and a * 7 = {}",
        another_point.to_string(),
        another_point.mul(7).to_string()
    );
}
