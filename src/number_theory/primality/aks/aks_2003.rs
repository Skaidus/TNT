#[cfg(test)]
mod tests {
    use super::Aks2003;
    use crate::number_theory::primality::PrimalityTest;
    use std::vec::IntoIter;
    
    #[test]
    fn recognizes_primes(){
        let mut iter : IntoIter<u32>  = vec![2, 3, 5, 7, 11//, 13, 17, 19
        //,1693, 1697, 1699, 1709, 1721, 1723, 1733, 1741, 1747, 1753, 1759, 1777, 1783, 1787, 1789, 1801, 1811, 1823, 1831, 1847, 1861, 1867, 1871, 1873, 1877, 1879, 1889, 1901, 1907, 1913, 1931, 1933, 1949, 1951, 1973, 1979, 1987, 1993, 1997, 1999, 2003, 2011, 2017, 2027, 2029, 2039, 13513, 13523, 13537, 13553, 13567, 13577, 13591, 13597, 13613, 13619, 13627, 13633, 13649, 13669, 13679, 13681, 13687, 13691, 13693, 24631, 24659, 24671, 24677, 24683, 24691, 24697, 24709, 24733, 24749, 24763, 24767, 24781, 24793, 24799, 24809, 24821, 24841, 24847, 24851, 24859, 24877, 24889, 28591, 28597, 28603, 28607, 28619, 28621, 28627, 28631, 28643, 28649, 28657, 28661
        //,776159
        //98982599
        //,9984605927,999498062999,99996460031327,9999940600088207,999999594000041207,4611685283988009527,9223371593598182327
        ].into_iter();
        assert!(iter.all(|x| Aks2003::is_prime(x)));

    }

    #[test]
    fn recognizes_composites(){
        let mut iter : IntoIter<u32> = vec![64, 65, 66, 68, 69, 70, 72, 74, 93, 94, 95, 96, 98, 99, 100
        //,1693, 1697, 1699, 1709, 1721, 1723, 1733, 1741, 1747, 1753, 1759, 1777, 1783, 1787, 1789, 1801, 1811, 1823, 1831, 1847, 1861, 1867, 1871, 1873, 1877, 1879, 1889, 1901, 1907, 1913, 1931, 1933, 1949, 1951, 1973, 1979, 1987, 1993, 1997, 1999, 2003, 2011, 2017, 2027, 2029, 2039, 13513, 13523, 13537, 13553, 13567, 13577, 13591, 13597, 13613, 13619, 13627, 13633, 13649, 13669, 13679, 13681, 13687, 13691, 13693, 24631, 24659, 24671, 24677, 24683, 24691, 24697, 24709, 24733, 24749, 24763, 24767, 24781, 24793, 24799, 24809, 24821, 24841, 24847, 24851, 24859, 24877, 24889, 28591, 28597, 28603, 28607, 28619, 28621, 28627, 28631, 28643, 28649, 28657, 28661
        //
        //,776161,98982601
        //,9984605929,999498063001,99996460031329,9999940600088209,999999594000041209,4611685283988009529,9223371593598182329
        ].into_iter();
        assert!(iter.all(|x| !Aks2003::is_prime(x)));
    }
}

use num::Integer;
use num::integer::Roots;

use crate::number_theory::primality::PrimalityTest;
use crate::number_theory::perfect_power::bernstein::bernstein_1998::Bernstein1988;
use crate::number_theory::util;
use crate::polynomial::Poly;

pub struct Aks2003 {    
    n : u32,
    logn : u32
}


impl Aks2003 {
    fn get_r(&self) -> u32{
        let max_k = self.logn.pow(2);
        let max_r = self.logn.pow(5).max(3u32);
        let mut next_r = true;
        let mut r = 2u32;
        let mut k;
        let mut n_pow_k_mod_r;
        while next_r && r < max_r {
            next_r = false;
            k = 1u32;
            while !next_r && k <= max_k {
                n_pow_k_mod_r = util::powm(self.n, k, r);
                next_r = n_pow_k_mod_r == 1u32 || n_pow_k_mod_r == 0; 
                k+=1u32;
            }
            r+=1u32;
        }
        r-1u32
    }

    fn new( n : u32) -> Aks2003{
        Aks2003 {n, logn: util::log2_floor(n)}
    }
}


impl PrimalityTest for Aks2003 {
    type Int = u32;
    
    fn is_prime(n : Self::Int) -> bool{
        if n < 3 {return true}
        if n % 2u32 == 0 {return false}
        if Bernstein1988::is_perfect_power(n){return false};
        let test = Aks2003::new(n);
        let r = test.get_r();
        for a in (2..=r).rev(){
            if n.gcd(&a) != 1 {return false};
        }
        if n <= r {return true}
        let x_n = Poly::new(r as usize, n as usize)
                .pow(n); // x^n
        for a in 1..util::phi(r).sqrt()*test.logn {
            let mut poly = Poly::new(r as usize, n as usize);
            poly -= a.try_into().unwrap();
            poly = poly.pow(n);
            poly += a.try_into().unwrap(); // (x - a)^n + a
            if poly != x_n {return false}
        }
        true
    }
}
