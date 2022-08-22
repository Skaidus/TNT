use num::{range, range_step, range_step_inclusive};

use crate::number_theory::UnsigInt;
pub fn log2_floor<T: UnsigInt>(k: T) -> u32 
    {
        return if k> T::one() {1 + log2_floor(k/(T::two()))} else {0}
        
    }

pub fn is_even<T: UnsigInt>(k : T) -> bool
{
        k % T::two() == T::zero()
}
pub fn div_ceiling<T: UnsigInt>(f : T, k : T) -> T
{
    let q = f / k;
    let r = f % k;
    if r == T::zero() {q} else {q+T::one()}
}

pub fn largest_prime_factor<T: UnsigInt>(n : T) -> T
{
    if n < T::two() {return T::one()};
    let mut r = n;
    let mut p = T::zero();
    if r % T::two() == T::zero() {
        p = T::two();
        loop {
            r = r/T::two();
            if r%T::two() != T::zero() {break};
        }
    }
    for i in range_step_inclusive(T::three(), r, T::two()){
        if r % i == T::zero(){
            p = i;
            loop {
                r = r/i;
                if r%i != T::zero() {break};
            }
        }
    }
    p
}