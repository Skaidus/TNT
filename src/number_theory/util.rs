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

// https://cp-algorithms.com/algebra/phi-function.html#properties
pub fn phi<T: UnsigInt>(n : T) -> T
{
    let mut result = n;
    let mut n = n;
    let mut i = T::two();
    while i*i <= n {
        if(n%i == T::zero()){
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

// Finds inverse of n (mod m)
pub fn inv(n : usize, m : usize) -> usize{
    let mut a = n;
    let mut b = m;
    let mut u  = 1isize;
    let mut v = 0isize;
    loop {
        let q = a / b;
        let t1 = a -q*b;
        a = b;
        b = t1;
        let t2 = u - (q as isize) * v;
        u = v;
        v = t2;
        if b != 0 {break;}
    }
    if a != 1 {u=0}
    if u < 0 {u+= m as isize}
    u as usize
}