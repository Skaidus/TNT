pub mod polynomial;
pub mod number_theory {
    pub mod primality {
        pub mod aks {
            pub mod aks_2002;
            pub mod aks_2003;
            pub mod aks_2005;
        }
        pub mod naive;
        pub trait PrimalityTest {
            type Int;
            fn is_prime(n : Self::Int) -> bool;
        }
    }

    pub mod primes_less_than {
        pub mod sieve;
        pub mod naive;
        pub trait PrimesLessThan {
            type Int;
            fn get_primes(n : Self::Int) -> Vec<Self::Int>;
        }
    }
    
    pub mod perfect_power {
        pub mod bernstein {
            pub mod bernstein_1998;
        }
    }
}