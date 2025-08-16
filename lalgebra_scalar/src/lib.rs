use std::ops::{Add, Sub, Mul, Div};

pub trait Scalar:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Clone
    + Sized
{
    type Item;
    fn zero() -> Self::Item;
    fn one() -> Self::Item;
}

macro_rules! impl_scalar_for {
    ($($t:ty),*) => {
        $(
            impl Scalar for $t {
                type Item = $t;
                fn zero() -> Self::Item { 0 as $t }
                fn one() -> Self::Item { 1 as $t }
            }
        )*
    };
}

impl_scalar_for!(u32, u64, i32, i64, f32, f64);
