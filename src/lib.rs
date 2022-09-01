pub mod number_theory {
    use num::{PrimInt, Unsigned};
    use std::{fmt::Display, ops::{AddAssign, MulAssign, DivAssign, SubAssign}};
    pub trait UnsigInt : PrimInt + Unsigned + Display + AddAssign + MulAssign + DivAssign + SubAssign{
        fn two() -> Self{
            Self::one()+Self::one()
        }
        fn three() -> Self{
            Self::two()+Self::one()
        }
    }



    
    impl UnsigInt for u32{}
    impl UnsigInt for usize{}
    pub mod primality {
        pub mod aks {
            pub mod aks_2002;
            pub mod aks_2003;
            pub mod aks_bernstein;
            pub mod aks_2005;
            mod aks_theorem;
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
    pub mod util;
    
}