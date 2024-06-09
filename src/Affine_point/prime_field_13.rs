use rand::Rng;
#[derive(Debug)]
pub struct F13 {
    element: i32,
}

impl F13 {
    pub fn new(element: i32) -> Self {
        loop {
            if element < 13 {
                break;
            }
            let element = element % 13;
        }
        return Self { element };
    }
    pub fn new_random() -> Self {
        let element = rand::thread_rng().gen_range(0..13);
        let element = F13::new(element);
        element
    }
    pub fn element(&self) -> i32 {
        self.element
    }
    pub fn sqr(&self) -> Self {
        let ansa = self.element.pow(2);
        return F13::to_F13(ansa);
    }
    pub fn to_F13(num: i32) -> Self {
        if num > 1300 {
            let ansa = num - 13000;
            return F13::to_F13(ansa);
        }
        Self { element: num % 13 }
    }
    pub fn cube(&self) -> Self {
        let ansa = self.element.pow(3);
        return F13::to_F13(ansa);
    }
    pub fn add(&self, other: &F13) -> Self {
        let ansa = self.element + other.element;
        return F13::to_F13(ansa);
    }
    pub fn mul(&self, other: &F13) -> Self {
        let ansa = self.element * other.element;
        return F13::to_F13(ansa);
    }
    pub fn is_eq(&self, other: &F13) -> bool {
        self.element == other.element
    }
    pub fn copy(&self) -> Self {
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
    pub fn inverse(&self) -> Self {
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
    pub fn additive_inverse(&self) -> Self {
        if self.element == 0 {
            return Self { element: 0 };
        }
        let ansa = 13 - self.element;
        Self { element: ansa }
    }
    // for debugging purposes
    pub fn print_F13(&self) {
        println!("your ansa is {}", self.element);
    }
    pub fn sub(&self, other: &F13) -> Self {
        self.add(&other.additive_inverse())
    }
    pub fn div(&self, other: &F13) -> Self {
        self.mul(&other.inverse())
    }
}
struct EuclideAlgo {}
impl EuclideAlgo {
    fn get_t(quotient: i32, T_1: i32, T_2: i32) -> i32 {
        return T_1 - (T_2 * quotient);
    }
}
