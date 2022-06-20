use crate::aks::AKS;

pub struct GMP_AKS {

}

impl AKS for GMP_AKS {
    type Number = u64;
    fn is_prime(n : Self::Number) -> bool{
        true
    }
}