pub mod original {
    pub mod gmp;
}

pub mod optimized {
    pub mod custom;
    pub mod gmp_opt;
}

pub trait AKS {
    type Int;
    fn is_prime(n : Self::Int) -> bool;
}