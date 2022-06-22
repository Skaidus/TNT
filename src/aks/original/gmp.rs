pub mod sieve;
pub mod poly; 
use gmp_mpfr_sys::gmp;
use sieve::Sieve;
use crate::aks::AKS;
use rug::{Integer, Complete};
pub struct GmpAks {

}

impl AKS for GmpAks {
    type Int = Integer;
    fn is_prime(n : Self::Int) -> bool{
        if n.is_perfect_power() {return false}
        let mut s = Sieve::new();
        let mut r = Integer::from(2u32);
        let logn = r.significant_bits();
        let limit = 4 * (logn * logn);
        while r < n {
            if n.is_divisible(&r){return false}
            let mut failed = false;
            if s.is_prime(&r){
                let mut i = Integer::from(1u32);
                while i <= limit{
                    if n.clone().pow_mod(&i, &r).unwrap() == 1u32{
                        failed = true;
                        break
                    }
                    i += 1u32;
                }
                if !failed {break}
            }
            r += 1;
        }
        if r == n {return true}
        // Polynomial check
        let polylimit : Integer = Integer::from(r.clone().sqrt()+1u32)*2u32 * logn;
        let mut final_size = Integer::new();
        for a in 0usize..=polylimit.to_usize().unwrap(){
            (&n % &r).complete_into(&mut final_size);

        }
        
        

        true
    }
}