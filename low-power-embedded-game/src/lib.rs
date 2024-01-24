// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    
    let q = dividend / divisor;
    let r = dividend % divisor;
    (q, r)
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    // before the student has done any work.

    iter.enumerate()
    .filter(|(x, y)| x % 2 == 0)
    .map(|(z, a)| a)
    
}

pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        self.0.abs() + self.1.abs()
    }
}
