mod sieve; 
use gmp_mpfr_sys::gmp;

use crate::aks::AKS;
use rug::Integer;
pub struct GmpAks {

}

impl AKS for GmpAks {
    type Int = Integer;
    fn is_prime(n : Self::Int) -> bool{
        true
    }
}