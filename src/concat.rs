use crate::count::Count;

pub trait Concat {
    fn concat(self, rhs: Self) -> Self;
}

impl Concat for i64 {
    fn concat(self, rhs: Self) -> Self {
        self * i64::pow(10, rhs.digits()) + rhs
    }
}
