pub mod original {
    mod gmp;
}

pub trait AKS {
    type Number;
    fn is_prime(n : Self::Number) -> bool;
}