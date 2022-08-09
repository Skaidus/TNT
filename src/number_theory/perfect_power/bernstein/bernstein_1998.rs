use crate::number_theory::primality::sieve::Sieve;

pub struct Bernstein1988 {

}

#[cfg(test)]
mod tests {
   
    use super::Bernstein1988;
    
    #[test]
    fn recognizes_perfect_powers(){
        use std::vec::IntoIter;
        let mut iter : IntoIter<u32>  = vec![9, 16, 25, 27, 32, 36, 49, 64, 81, 100, 121, 125].into_iter();
        assert!(iter.all(|x| Bernstein1988::is_perfect_power(x as u64)));

    }

    #[test]
    fn recognizes_not_perfect_powers(){
        use std::vec::IntoIter;
        let mut iter : IntoIter<u32>  = vec![10, 17, 26, 28, 33, 37, 44, 67, 89, 101, 123, 127].into_iter();
        assert!(iter.all(|x| !Bernstein1988::is_perfect_power(x as u64)));
    }
}

impl Bernstein1988 {

    fn logk(k: u64) -> u32 {
        64 - k.leading_zeros() - 1
    }

    fn mul_2(b: u32, m: u32, k: u32) -> u32 {
        (k * m) % (2 ^ b)
    }

    fn pow_2(b: u32, m: u32, k: u32) -> u32 {
        (m ^ k) % (2 ^ b)
    }

    fn div_2(b: u32, m: u32, k: u32) -> u32 {
        let exp = 2 ^ b;
        for d in 0..exp - 1 {
            if m == Bernstein1988::pow_2(b, d, k) {
                return d;
            }
        }
        return exp - 1;
    }

    fn nroot(y: u32, b: u32, k: u32) -> u32 {
        if b == 1 {
            return 1;
        }
        let z = Bernstein1988::nroot(y, b / 2, k);
        let r2 = Bernstein1988::mul_2(b, z, k + 1);
        let r3 = y * Bernstein1988::pow_2(b, z, k + 1);
        Bernstein1988::div_2(b, r2 - r3, k)
    }

    fn x_pow_k_eq_n(n: u32, x: u32, k: u32) -> bool {
        let f = Bernstein1988::logk((2 * n).into());
        if x == 1 {
            return n == 1;
        }
        let mut b = 1;
        let exp = 2 ^ b;
        let mut x_pow_k_eq_r = false;
        while b < f {
            let r = Bernstein1988::pow_2(b, x, k);
            x_pow_k_eq_r = Bernstein1988::x_pow_k_eq_n(n, r, k);
            if n % exp != r {
                return false;
            }
            b = (2 * b).min(f);
        }
        x_pow_k_eq_r
    }

    fn is_kth_power(n: u32, k: u32, y: u32) -> bool {
        let f = Bernstein1988::logk((2 * n).into());
        let b = f / k;
        let r = Bernstein1988::nroot(y, b, k);
        if k == 2 && r == 0 {
            return false;
        }
        if Bernstein1988::x_pow_k_eq_n(n, r, k) {
            return true;
        }
        if k == 2 {
            return Bernstein1988::x_pow_k_eq_n(n, (2 ^ b) - r, k);
        }
        false
    }

    pub fn is_perfect_power(n : u64) -> bool {
        let f = Bernstein1988::logk(2 * n);
        let y = Bernstein1988::nroot(f / 2 + 1, n as u32, 1);
        for p in Sieve::get_primes(f as usize) {
            if Bernstein1988::is_kth_power(n as u32, p as u32, y) {
                return true;
            }
        }
        false
    }
}