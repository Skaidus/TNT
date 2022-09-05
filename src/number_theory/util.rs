use num::{ range_step_inclusive, range};

use crate::number_theory::UnsigInt;
// n digits k in base 2 = floor(log2(k)) + 1 = ceil(log2(k+1))
pub fn log2_floor<T: UnsigInt>(k: T) -> u32 
    {
        return if k> T::one() {1 + log2_floor(k/(T::two()))} else {0}
        
    }

#[allow(dead_code)]
pub fn log2_ceil<T: UnsigInt>(k: T) -> u32 
{
    return log2_floor(k-T::one()) + 1
    
}

pub fn n_c_k<T: UnsigInt>(n : T, k: T) -> T 
{
    let mut res = T::one();
    let k = if k > n - k {n-k} else {k};
    for i in range(T::zero(),k){
        res *= n - i;
        res /= i + T::one();
    }
    res
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

// https://cp-algorithms.com/algebra/phi-function.html#properties
pub fn phi<T: UnsigInt>(n : T) -> T
{
    let mut result = n;
    let mut n = n;
    let mut i = T::two();
    while i*i <= n {
        if n%i == T::zero(){
            while n % i == T::zero() {
                n /= i;
            }
            result -= result/i;
        }
        i+=T::one();
    }
    if n > T::one() {result -= result/n}
    result
}

// Shortcut for (x * y) % m
pub fn umulrem<T: UnsigInt>(x : T, y : T, m : T) -> T{
    (x * y) % m
}
// Shortcut for (x * y + a) % m
#[allow(dead_code)]
pub fn umuladdrem<T: UnsigInt>(x : T, y : T, a : T, m : T) -> T{
    (x * y + a) % m
}

// Shortcut for (n ^ exp) % m
pub fn powm<T: UnsigInt>(n : T, exp : T, m : T) -> T{
    let mut r = T::one();
    let mut t = n % m;
    let mut i = exp;
    while i != T::zero() {
        if i % T::two() != T::zero() {
            r = umulrem(r, t, m);
        }
        t = umulrem(t, t, m);
        i /= T::two();
    }
    r
}
