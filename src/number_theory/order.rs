
use super::{UnsigInt};

// https://cp-algorithms.com/algebra/phi-function.html#properties
pub fn mult_order<T: UnsigInt>(a : T, n : T) -> T
{
    return a*n;
}