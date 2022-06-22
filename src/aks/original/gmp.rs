pub mod sieve;
pub mod poly; 
use sieve::Sieve;
use poly::Poly;
use crate::aks::AKS;
use rug::{Integer, Complete};

#[cfg(test)]
mod tests {
    use crate::aks::AKS;
    use rug::Integer;
    use super::GmpAks;
    use std::vec::IntoIter;
    #[test]
    fn recognizes_primes(){
        let mut iter : IntoIter<u32>  = vec![2, 3, 5, 7, 11, 13, 17, 19
        //,1693, 1697, 1699, 1709, 1721, 1723, 1733, 1741, 1747, 1753, 1759, 1777, 1783, 1787, 1789, 1801, 1811, 1823, 1831, 1847, 1861, 1867, 1871, 1873, 1877, 1879, 1889, 1901, 1907, 1913, 1931, 1933, 1949, 1951, 1973, 1979, 1987, 1993, 1997, 1999, 2003, 2011, 2017, 2027, 2029, 2039, 13513, 13523, 13537, 13553, 13567, 13577, 13591, 13597, 13613, 13619, 13627, 13633, 13649, 13669, 13679, 13681, 13687, 13691, 13693, 24631, 24659, 24671, 24677, 24683, 24691, 24697, 24709, 24733, 24749, 24763, 24767, 24781, 24793, 24799, 24809, 24821, 24841, 24847, 24851, 24859, 24877, 24889, 28591, 28597, 28603, 28607, 28619, 28621, 28627, 28631, 28643, 28649, 28657, 28661
        ,776159,98982599
        //,9984605927,999498062999,99996460031327,9999940600088207,999999594000041207,4611685283988009527,9223371593598182327
        ].into_iter();
        assert!(iter.all(|x| GmpAks::is_prime(Integer::from(x))));

    }

    #[test]
    fn recognizes_composites(){
        let mut iter : IntoIter<u32> = vec![64, 65, 66, 68, 69, 70, 72, 74, 93, 94, 95, 96, 98, 99, 100
        //,1693, 1697, 1699, 1709, 1721, 1723, 1733, 1741, 1747, 1753, 1759, 1777, 1783, 1787, 1789, 1801, 1811, 1823, 1831, 1847, 1861, 1867, 1871, 1873, 1877, 1879, 1889, 1901, 1907, 1913, 1931, 1933, 1949, 1951, 1973, 1979, 1987, 1993, 1997, 1999, 2003, 2011, 2017, 2027, 2029, 2039, 13513, 13523, 13537, 13553, 13567, 13577, 13591, 13597, 13613, 13619, 13627, 13633, 13649, 13669, 13679, 13681, 13687, 13691, 13693, 24631, 24659, 24671, 24677, 24683, 24691, 24697, 24709, 24733, 24749, 24763, 24767, 24781, 24793, 24799, 24809, 24821, 24841, 24847, 24851, 24859, 24877, 24889, 28591, 28597, 28603, 28607, 28619, 28621, 28627, 28631, 28643, 28649, 28657, 28661
        //
        ,776161,98982601
        //,9984605929,999498063001,99996460031329,9999940600088209,999999594000041209,4611685283988009529,9223371593598182329
        ].into_iter();
        assert!(iter.all(|x| !GmpAks::is_prime(Integer::from(x))));
    }
}

pub struct GmpAks {

}

impl AKS for GmpAks {
    type Int = Integer;
    fn is_prime(n : Self::Int) -> bool{
        if n.is_perfect_power() {return false}
        let mut s = Sieve::new();
        let mut r = Integer::from(2u32);
        let logn = r.significant_bits();
        let limit = 4 * (logn * logn);
        while r < n {
            if n.is_divisible(&r){return false}
            let mut failed = false;
            if s.is_prime(&r){
                let mut i = Integer::from(1u32);
                while i <= limit{
                    if n.clone().pow_mod(&i, &r).unwrap() == 1u32{
                        failed = true;
                        break
                    }
                    i += 1u32;
                }
                if !failed {break}
            }
            r += 1;
        }
        if r == n {return true}
        // Polynomial check
        let polylimit : Integer = Integer::from(r.clone().sqrt()+1u32)*2u32 * logn;
        let ui_r = r.to_usize_wrapping();
        let mut final_size = Integer::new();
        for a in 0usize..=polylimit.to_usize().unwrap(){
            (&n % &r).complete_into(&mut final_size);
            let coef = final_size.to_usize_wrapping();
            let mut compare = Poly::with_length(coef);
            compare.set_coeficient(Integer::from(1u32), coef);
            compare.set_coeficient(Integer::from(a), 0);
            let mut res = Poly::with_length(ui_r);
            let mut base = Poly::with_length(1);
            base.set_coeficient(Integer::from(a), 0);
            base.set_coeficient(Integer::from(1u32), 1);
            res.assign_pow_mod(&base, &n, &n, ui_r);
            if res != compare{return false}
        }
        true
    }
}