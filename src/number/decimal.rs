use std::{ops::{Add, Sub, Neg, Mul, Div}, fmt::Display, cmp::max};

pub fn simplify(origin: &Decimal) -> Decimal {
    let mut int = origin.int;
    let mut point = origin.point;
    while int % 10 == 0 && point > 0 {
        int /= 10;
        point -= 1;
    }
    Decimal { int, point }
}

#[test]
fn test_decimal_simplify() {
    let origin = Decimal::_new(1140514000, 5);
    println!("{:?}\n{0}", &origin);
    let simplified = simplify(&origin);
    println!("{:?}\n{0}", &simplified);
}

#[derive(Debug)]
pub struct Decimal {
    int: i128,
    point: u8
}

impl Decimal {
    pub fn zero() -> Self {
        Self { int: 0, point: 0 }
    }

    fn is_zero(&self) -> bool {
        self.int == 0
    }

    fn is_integer(&self) -> bool {
        self.point == 0
    }

    fn is_nonnegative(&self) -> bool {
        self.int >= 0
    }

    fn _new(int: i128, point: u8) -> Self {
        Self { int, point }
    }
}

impl Decimal {  // for `pow` function
    pub fn pow(&self, rhs: &Self) -> Self {
        match rhs {
            r if r.is_nonnegative() && r.is_integer() => {
                let int = self.int.pow(rhs.int as u32);
                let point = self.point * r.int as u8;
                Self { int, point }
            },
            // todo: other kind of exponents
            _ => panic!("Only non-negative integer exponents are supported")
        }
    }
}

#[test]
fn test_decimal_pow() {
    let examples = vec![
        Decimal::from("2".to_string()).pow(&Decimal::from("8".to_string())),
        Decimal::from("0.1".to_string()).pow(&Decimal::from("4".to_string())),
        Decimal::from("0.002".to_string()).pow(&Decimal::from("4".to_string())),
        Decimal::from("2.1".to_string()).pow(&Decimal::from("4".to_string())),
        Decimal::from("0.00".to_string()).pow(&Decimal::from("4".to_string())),
        Decimal::from("0.233".to_string()).pow(&Decimal::from("1".to_string())),
        Decimal::from("0.002".to_string()).pow(&Decimal::from("0".to_string())),
    ];
    for example in examples {
        println!("Result: {}", example);
    }
}

impl From<String> for Decimal {
    fn from(num_string: String) -> Self {
        let num_vec: Vec<&str> = num_string.split('.').collect();
        let int: i128; let point: u8;
        if num_vec.len() == 1 {
            int = num_vec[0].parse().unwrap();
            point = 0;
        }
        else {
            int = (num_vec[0].to_string() + num_vec[1]).parse().unwrap();
            point = num_vec[1].len() as u8;
        };
        if int == 0 { return Decimal::zero(); }
        simplify(&Decimal { int, point })
    }
}

#[test]
fn test_decimal_from() {
    let examples = vec![
        "0.0", "1.00", "1.14514", "2.333",
    ];
    for example in examples {
        let dec = Decimal::from(example.to_string());
        println!("{:?}\n{0}\n", &dec);
    }
}

impl Display for Decimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut int_string = self.int.to_string();
        if self.int == 0 { return write!(f, "0.0"); }
        if self.point != 0 {
            while int_string.len() < self.point as usize {
                int_string.insert(0, '0');
            }
            let index = int_string.len() - self.point as usize;
            if index == 0 { int_string = "0.".to_string() + &int_string; }
            else {
                if self.int < 0 && index == 1 { int_string.insert_str(index, "0."); }
                else { int_string.insert(index, '.'); }
            }
        }
        else { int_string += ".0"; }
        write!(f, "{}", int_string)
    }
}

#[test]
fn test_decimal_display() {
    let examples = vec![
        Decimal::_new(1, 2),
        Decimal::_new(233, 2), 
        Decimal::_new(-2333, 2),
        Decimal::_new(233, 3),
        Decimal::_new(-233, 3),
    ];
    for example in examples {
        println!("{:?}\n{0}\n", &example);
    }
}

impl Add for Decimal {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        if self.point == rhs.point {
            Decimal { int: self.int + rhs.int, point: self.point }
        }
        else {
            let mut this_int = self.int;
            let mut this_point = self.point;
            let mut other_int = rhs.int;
            let mut other_point = rhs.point;
            while this_point != other_point {
                if this_point < other_point {
                    this_int *= 10;
                    this_point += 1;
                }
                if this_point > other_point {
                    other_int *= 10;
                    other_point += 1;
                }
            }
            Decimal { int: this_int + other_int, point: this_point }
        }
    }
}

#[test]
fn test_decimal_add() {
    let examples = vec![
        Decimal::_new(1140000, 4) + Decimal::_new(514, 3),
        Decimal::_new(100, 1) + Decimal::_new(9, 1),
    ];
    for example in examples {
        println!("Data: {:?}\nResult: {0}\n", &example);
    }
}

impl Neg for Decimal {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Decimal { int: -self.int, point: self.point }
    }
}

impl Sub for Decimal {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

#[test]
fn test_decimal_sub() {
    let examples = vec![
        Decimal::_new(114514, 5) - Decimal::_new(114514, 5),
        Decimal::_new(2333, 3) - Decimal::_new(003, 2),
        Decimal::_new(100, 1) - Decimal::_new(9, 1),
    ];
    for example in examples {
        println!("Result: {}", &example);
    }
}

impl Mul for Decimal {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let int = self.int * rhs.int;
        let point = if self.is_integer() || rhs.is_integer() { max(self.point, rhs.point) }
            else { self.point * rhs.point + 1 };
        Decimal { int, point }
    }
}

#[test]
fn test_decimal_mul() {
    let examples = vec![
        Decimal::from(10.to_string()) * Decimal::from(2.333.to_string()),
    ];
    for example in examples {
        println!("Result: {}", &example);
    }
}

impl Div for Decimal {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        if rhs.is_zero() { panic!("Cannot divide by zero"); }
        let mut this_int = self.int;
        let mut this_point = self.point;
        while this_int as f64 / rhs.int as f64 != (this_int / rhs.int) as f64 {
            this_int *= 10;
            this_point += 1;
        }
        let mut other_int = rhs.int;
        let mut other_point = rhs.point;
        while this_int as f64 / other_int as f64 != (this_int / other_int) as f64 {
            other_int *= 10;
            other_point += 1;
        }
        let int = this_int / other_int;
        let point = this_point - other_point;
        Decimal { int, point }
    }
}

#[test]
fn test_decimal_div() {
    let _result = Decimal::from(1.14.to_string()) / Decimal::from(10.to_string());
    println!("{}", _result);
    let _result = Decimal::from(23.3.to_string()) / Decimal::from(0.1.to_string());
    println!("{}", _result);
    let _result = Decimal::from(2333.to_string()) / Decimal::from(0.2.to_string());
    println!("{}", _result);
}

#[test]
#[should_panic]
fn test_decimal_div_zero() {
    let _result = Decimal::from(1.to_string()) / Decimal::from(0.to_string());
    println!("{}", _result);
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        let simplified_self = simplify(self);
        let simplified_other = simplify(other);
        simplified_self.int == simplified_other.int
        &&
        simplified_self.point == simplified_other.point
    }
}

#[test]
fn test_decimal_partial_eq() {
    let _result = Decimal::from("1.00".to_string()) == Decimal::from("1".to_string());
    println!("Result: {}", _result);
}

#[test]
fn test_decimal() {
    let examples = vec![
        Decimal::from(0.1.to_string()) + Decimal::from(0.1.to_string()) + Decimal::from(0.1.to_string()),
    ];
    for example in examples {
        println!("Result: {}", &example);
    }
}