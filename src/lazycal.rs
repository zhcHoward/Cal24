use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub enum LazyCal {
    Int(i32),
    Lazy((i32, i32)),
}

pub use LazyCal::*;

impl LazyCal {
    pub fn new(dividend: i32, divisor: i32) -> Self {
        if divisor != 0 && dividend % divisor == 0 {
            Int(dividend / divisor)
        } else {
            Lazy((dividend, divisor))
        }
    }

    pub fn lazy(dividend: i32, divisor: i32) -> Self {
        Lazy((dividend, divisor))
    }

    pub fn calculate(&self) -> i32 {
        match self {
            Int(i) => *i,
            Lazy((dividend, divisor)) => {
                if *divisor == 0 || dividend % divisor != 0 {
                    0
                } else {
                    dividend / divisor
                }
            }
        }
    }
}

impl Add<i32> for LazyCal {
    type Output = LazyCal;
    fn add(self, rhs: i32) -> Self::Output {
        match self {
            Int(i) => Int(i + rhs),
            Lazy((dividend, divisor)) => LazyCal::new(dividend + divisor * rhs, divisor),
        }
    }
}

impl Add<LazyCal> for i32 {
    type Output = LazyCal;
    fn add(self, rhs: LazyCal) -> Self::Output {
        match rhs {
            Int(i) => Int(i + self),
            Lazy((dividend, divisor)) => LazyCal::new(dividend + divisor * self, divisor),
        }
    }
}

impl Add<LazyCal> for LazyCal {
    type Output = LazyCal;
    fn add(self, rhs: LazyCal) -> Self::Output {
        match self {
            Int(i) => i + rhs,
            Lazy((ld1, ld2)) => match rhs {
                Int(i) => i + self,
                Lazy((rd1, rd2)) => LazyCal::new(ld1 * rd2 + ld2 * rd1, ld2 * rd2),
            },
        }
    }
}

impl Sub<i32> for LazyCal {
    type Output = LazyCal;
    fn sub(self, rhs: i32) -> Self::Output {
        match self {
            Int(i) => Int(i - rhs),
            Lazy((dividend, divisor)) => LazyCal::new(dividend - divisor * rhs, divisor),
        }
    }
}

impl Sub<LazyCal> for i32 {
    type Output = LazyCal;
    fn sub(self, rhs: LazyCal) -> Self::Output {
        match rhs {
            Int(i) => Int(self - i),
            Lazy((dividend, divisor)) => LazyCal::new(divisor * self - dividend, divisor),
        }
    }
}

impl Sub<LazyCal> for LazyCal {
    type Output = LazyCal;
    fn sub(self, rhs: LazyCal) -> Self::Output {
        match self {
            Int(i) => i - rhs,
            Lazy((ld1, ld2)) => match rhs {
                Int(i) => self - i,
                Lazy((rd1, rd2)) => LazyCal::new(ld1 * rd2 - ld2 * rd1, ld2 * rd2),
            },
        }
    }
}

impl Mul<i32> for LazyCal {
    type Output = LazyCal;
    fn mul(self, rhs: i32) -> Self::Output {
        match self {
            Int(i) => Int(i * rhs),
            Lazy((dividend, divisor)) => LazyCal::new(dividend * rhs, divisor),
        }
    }
}

impl Mul<LazyCal> for i32 {
    type Output = LazyCal;
    fn mul(self, rhs: LazyCal) -> Self::Output {
        match rhs {
            Int(i) => Int(i * self),
            Lazy((dividend, divisor)) => LazyCal::new(dividend * self, divisor),
        }
    }
}

impl Mul<LazyCal> for LazyCal {
    type Output = LazyCal;
    fn mul(self, rhs: LazyCal) -> Self::Output {
        match self {
            Int(i) => i * rhs,
            Lazy((ld1, ld2)) => match rhs {
                Int(i) => i * self,
                Lazy((rd1, rd2)) => LazyCal::lazy(ld1 * rd1, ld2 * rd2),
            },
        }
    }
}

impl Div<i32> for LazyCal {
    type Output = LazyCal;
    fn div(self, rhs: i32) -> Self::Output {
        match self {
            Int(i) => LazyCal::new(i, rhs),
            Lazy((dividend, divisor)) => {
                if rhs != 0 && dividend % rhs == 0 {
                    LazyCal::lazy(dividend / rhs, divisor)
                } else {
                    LazyCal::lazy(dividend, divisor * rhs)
                }
            }
        }
    }
}

impl Div<LazyCal> for i32 {
    type Output = LazyCal;
    fn div(self, rhs: LazyCal) -> Self::Output {
        match rhs {
            Int(i) => LazyCal::new(self, i),
            Lazy((dividend, divisor)) => LazyCal::new(self * divisor, dividend),
        }
    }
}

impl Div<LazyCal> for LazyCal {
    type Output = LazyCal;
    fn div(self, rhs: LazyCal) -> Self::Output {
        match self {
            Int(i) => i / rhs,
            Lazy((ld1, ld2)) => match rhs {
                Int(i) => self / i,
                Lazy((rd1, rd2)) => LazyCal::new(ld1 * rd2, ld2 * rd1),
            },
        }
    }
}
