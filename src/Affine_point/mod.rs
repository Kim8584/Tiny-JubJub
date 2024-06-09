pub mod prime_field_13;
use super::{binary::Binary, is_point};
pub use prime_field_13::F13;

#[derive(Debug)]
pub struct Affine_point {
    x: F13,
    y: F13,
}
impl Affine_point {
    pub fn new(x: F13, y: F13) -> Self {
        Self { x, y }
    }
    pub fn to_string(&self) -> String {
        format!("({},{})", self.x.element(), self.y.element())
    }
    pub fn print_affine_point(&self) {
        println!("{}", self.to_string());
    }
    pub fn new_from_i32(x_i32: i32, y_i32: i32) -> Self {
        let x = F13::new(x_i32);
        let y = F13::new(y_i32);
        let point = Affine_point::new(x, y);
        // assert!(is_point(&point));
        point
    }
    pub fn copy(&self) -> Self {
        Self {
            x: self.x.copy(),
            y: self.y.copy(),
        }
    }
    pub fn is_eq(&self, other: &Self) -> bool {
        assert!(is_point(self));
        assert!(is_point(other));
        self.x.is_eq(&other.x) && self.y.is_eq(&other.y)
    }
    pub fn x(&self) -> i32 {
        self.x.element()
    }
    pub fn y(&self) -> i32 {
        self.y.element()
    }
    pub fn y_F13(&self) -> F13 {
        self.y.copy()
    }
    pub fn x_F13(&self) -> F13 {
        self.x.copy()
    }
    pub fn add(&self, other: &Affine_point) -> Self {
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
    pub fn double(&self) -> Self {
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
    pub fn mul(&self, number: i32) -> Self {
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
