use crate::aks::AKS;

pub struct GmpAks {

}

impl AKS for GmpAks {
    type Number = u64;
    fn is_prime(n : Self::Number) -> bool{
        true
    }
}