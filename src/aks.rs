pub mod original {
    pub mod gmp;
}

pub mod optimized {
    pub mod custom;
}

pub trait AKS {
    type Number;
    fn is_prime(n : Self::Number) -> bool;
}