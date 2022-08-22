use super::PrimesLessThan;
use crate::number_theory::primality::{naive::Naive, PrimalityTest};
use crate::number_theory::util;
pub struct NaivePlt {
}

impl PrimesLessThan for NaivePlt {
    type Int = usize;
    fn get_primes(n : Self::Int) -> Vec<Self::Int>{
        let mut primes = Vec::with_capacity(n/(util::log2_floor(n) as usize));
        let mut p = 3;
        primes.push(2);
        while p<n{
            if Naive::is_prime(p as u32) {
                primes.push(p);
            }
            p +=2;
        }
        primes
    }
}