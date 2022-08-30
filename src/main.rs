


// use aks_primes::aks::AKS;
// use rug::Integer;
// //use aks_primes::aks::optimized::custom::Integer as MyInteger;
// use aks_primes::aks::original::gmp::sieve::Sieve;
// use aks_primes::aks::original::gmp::GmpAks;

// struct AKS {
//     n: Integer,
//     log2n: Integer,
// }

// impl AKS {
//     fn new(n: usize) -> AKS {
//         let n = Integer::from(n);
//         AKS {
//             n,
//             log2n: Integer::from(n.significant_bits() - 1),
//         }
//     }

//     fn run(&self) -> bool {
//         if self.n.is_perfect_power() {
//             return false;
//         }
//         true
//     }

//     fn get_r(&self) {
//         let max_k = (&self.log2n).pow(2).complete();
//         let max_r = (&self.log2n).pow(5).complete().max(Integer::from(3 as u8));

//         let mut next_r = true;
//         let r = Integer::from(2 as u8);
//         while r <= max_r && next_r {
//             next_r = false;
//             let mut k = Integer::from(1 as u8);
//             while k <= max_k && !next_r {
//                 next_r = ((&self.n).pow_mod(&k, &r).unwrap() == Integer::from(2 as u8))
//             }
//         }
//     }
// }

// fn get_r(n : u64){
//     let max_k = n.log2().pow(2);
//     let max_r = 3.max(n.log2().pow(5));
//     let next_r = true;
//     for r in 2..=max_k{
//         if !next_r {break};
//         for k in 1..=max_k{
//             if next_r {break};
//             next_r = 1;
//         }
//     }

use aks_primes::number_theory::primality::{aks::aks_2003, PrimalityTest};
//use aks_primes::polywrap;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("aks_primes/include/aks_theorem.h");
        fn aks_theorem(n : u32, r : u32, s : u32) -> bool;
    }
}


fn main() {
    //aks_2003::Aks2003::is_prime(2039);
    ffi::aks_theorem(1,2,3);

}

// nat log: https://stackoverflow.com/questions/70688941/how-to-calculate-integer-logarithm-of-base-3-using-fast-bit-operations

// }

// fn mul_mod()
