mod number_theory {
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
            mod aks_2005;
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
    /// Return all prime numbers below `n`.
    /// 1 and `n` do not belong to this list.
    /// # Examples
    /// ```
    /// use tnt_lib::PrimesLessThan; // Required Trait.
    /// use tnt_lib::NaivePlt; // Struct that implements the trait.
    /// let expected : Vec<usize> = vec![2,3,5,7];
    /// assert_eq!(NaivePlt::get_primes(11), expected);
    /// ```
            fn get_primes(n : Self::Int) -> Vec<Self::Int>;
        }
    }
    
    pub mod perfect_power {
        pub mod bernstein {
            pub mod bernstein_1998;
        }
    }
    mod util;


    
}
pub use number_theory::primality::aks::aks_2002::Aks2002;
pub use number_theory::primality::aks::aks_2003::Aks2003;
pub use number_theory::primality::naive::Naive;
pub use number_theory::perfect_power::bernstein::bernstein_1998::Bernstein1988;
//pub use number_theory::primes_less_than::sieve::Sieve;
pub use number_theory::primes_less_than::naive::NaivePlt;
pub use number_theory::primes_less_than::PrimesLessThan;
pub use number_theory::primality::PrimalityTest;