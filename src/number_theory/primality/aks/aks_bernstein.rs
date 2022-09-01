#[cfg(test)]
mod tests {
    use super::AksBernstein;
    use crate::number_theory::primality::PrimalityTest;
    use std::vec::IntoIter;
    
    #[test]
    #[ignore = "not yet implemented"]
    fn recognizes_primes(){
        let mut iter : IntoIter<u32>  = vec![2, 3, 5, 7, 11,//, 13, 17, 19
        1693,
        1697,

        //,1693, 1697, 1699, 1709, 1721, 1723, 1733, 1741, 1747, 1753, 1759, 1777, 1783, 1787, 1789, 1801, 1811, 1823, 1831, 1847, 1861, 1867, 1871, 1873, 1877, 1879, 1889, 1901, 1907, 1913, 1931, 1933, 1949, 1951, 1973, 1979, 1987, 1993, 1997, 1999, 2003, 2011, 2017, 2027, 2029, 2039, 13513, 13523, 13537, 13553, 13567, 13577, 13591, 13597, 13613, 13619, 13627, 13633, 13649, 13669, 13679, 13681, 13687, 13691, 13693, 24631, 24659, 24671, 24677, 24683, 24691, 24697, 24709, 24733, 24749, 24763, 24767, 24781, 24793, 24799, 24809, 24821, 24841, 24847, 24851, 24859, 24877, 24889, 28591, 28597, 28603, 28607, 28619, 28621, 28627, 28631, 28643, 28649, 28657, 28661
        //,776159
        //98982599
        //,9984605927,999498062999,99996460031327,9999940600088207,999999594000041207,4611685283988009527,9223371593598182327
        ].into_iter();
        assert!(iter.all(|x| AksBernstein::is_prime(x)));

    }

    #[test]
    #[ignore = "not yet implemented"]
    fn recognizes_composites(){
        let mut iter : IntoIter<u32> = vec![64, 65, 66, 68, 69, 70, 72, 74, 93, 94, 95, 96, 98, 99, 100
        //,1693, 1697, 1699, 1709, 1721, 1723, 1733, 1741, 1747, 1753, 1759, 1777, 1783, 1787, 1789, 1801, 1811, 1823, 1831, 1847, 1861, 1867, 1871, 1873, 1877, 1879, 1889, 1901, 1907, 1913, 1931, 1933, 1949, 1951, 1973, 1979, 1987, 1993, 1997, 1999, 2003, 2011, 2017, 2027, 2029, 2039, 13513, 13523, 13537, 13553, 13567, 13577, 13591, 13597, 13613, 13619, 13627, 13633, 13649, 13669, 13679, 13681, 13687, 13691, 13693, 24631, 24659, 24671, 24677, 24683, 24691, 24697, 24709, 24733, 24749, 24763, 24767, 24781, 24793, 24799, 24809, 24821, 24841, 24847, 24851, 24859, 24877, 24889, 28591, 28597, 28603, 28607, 28619, 28621, 28627, 28631, 28643, 28649, 28657, 28661
        //
        //,776161,98982601
        //,9984605929,999498063001,99996460031329,9999940600088209,999999594000041209,4611685283988009529,9223371593598182329
        ].into_iter();
        assert!(iter.all(|x| !AksBernstein::is_prime(x)));
    }
}

use num::Integer;
use num::integer::Roots;

use crate::number_theory::primality::PrimalityTest;
use crate::number_theory::perfect_power::bernstein::bernstein_1998::Bernstein1988;
use crate::number_theory::util;

pub struct AksBernstein {    
    n : u32,
    logn_floor : u32
}


impl AksBernstein {
    fn get_aks_bound(&self) -> (u32, u32){
        let mut r = 3u32;
        let mut s = 0;
        while r < self.n{
            if self.n.gcd(&r) !=1 {return (0u32, 0u32)}
            if Naive::is_prime(r){
                let q = util::largest_prime_factor(r);
                s = 2*r.sqrt()*self.logn_floor;
                if util::n_c_k(2*q - 1, q) >= 2u32.pow(s) && self.n.pow((r-1)/q) <= 1 {
                    break;
                }
            }
            r += 1;
        }
        (r,s)
    }

    fn new( n : u32) -> AksBernstein{
        AksBernstein {n, logn_floor: util::log2_floor(n) }
    }
}

use super::aks_theorem;
use crate::number_theory::primality::naive::Naive;
impl PrimalityTest for AksBernstein {
    type Int = u32;
    
    fn is_prime(n : Self::Int) -> bool{
        if n <= 3 {return true}
        if n % 2u32 == 0 {return false}
        if Bernstein1988::is_perfect_power(n){return false};
        let test = AksBernstein::new(n);
        let res = test.get_aks_bound();
        if res.0 == 0 {return false};
        aks_theorem::ffi::aks_theorem(n, res.0, res.1)
    }
}
