use crate::number_theory::primes_less_than::{
    //sieve::Sieve,
     PrimesLessThan};

pub struct Bernstein1988 {
    f : u32
}

#[cfg(test)]
mod tests {
   
    use super::Bernstein1988;
    
    #[test]
    fn recognizes_perfect_powers(){
        use std::vec::IntoIter;
        let mut iter : IntoIter<u32>  = 
        vec![
            625,729,841,961,1089,1225,1369,1521,1681,1849,2025,2209,2401,2601,2809,3025,3249,3481,3721,3969,4225,4489,4761,5041,5329,5625,5929,6241,6561,6889,7225,7569,7921,8281,8649,9025,9409,9801,10201,10609,11025,11449,11881,12321,12769,13225,13689,14161,14641,15129,15625,16129,16641,17161,17689,18225,18769,19321,19881,20449,21025,21609,22201,22801,23409,24025,24649,25281
            ].into_iter();
        assert!(iter.all(|x| Bernstein1988::is_perfect_power(x as u64)));

    }

    #[test]
    fn recognizes_not_perfect_powers(){
        use std::vec::IntoIter;
        let mut iter : IntoIter<u32>  = 
        vec![
            627,731,843,963,1091,1227,1371,1523,1683,1851,2027,2211,2403,2603,2811,3027,3251,3483,3723,3971,4227,4491,4763,5043,5331,5627,5931,6243,6563,6891,7227,7571,7923,8283,8651,9027,9411,9803,10203,10611,11027,11451,11883,12323,12771,13227,13691,14163,14643,15131,15627,16131,16643,17163,17691,18227,18771,19323,19883,20451,21027,21611,22203,22803,23411,24027,24651,25283
            ].into_iter();
        assert!(iter.all(|x| !Bernstein1988::is_perfect_power(x as u64)));
    }
}

impl Bernstein1988 {
    fn new(f :u32) -> Self {
        Self{f}

    }

    /// Calculates log2_floor(k)
    /// # Examples
    /// ```
    /// use aks_primes::number_theory::perfect_power::bernstein::bernstein_1998::Bernstein1988;
    /// let mut test = vec![
    ///     (20, 4),
    ///     (16, 4),
    ///     (78, 6),
    ///     (556,9),
    ///     (1, 0),
    ///     (2, 1)
    /// ].into_iter();
    /// assert!(test.all(|x| Bernstein1988::log2_floor(x.0) == x.1));
    /// ```
    pub fn log2_floor(k: u64) -> u32 {
        return if k>1 {1 + Self::log2_floor(k/2)} else {0}
    }

    fn is_even(k : u32) -> bool {
        k % 2 == 0
    }

    
    fn div_ceiling(f : u32, k : u32)->u32{
        let q = f / k;
        let r = f % k;
        if r == 0 {q} else {q+1}
    }


    fn mul_2(b: u32, m: u32, k: u32) -> u32 {
        (k * m) % (2u32.pow(b))
    }
    // Unique integer d between 0 and 2^b s.t
    // (m = kd) % 2^b (m/k)%2^b
    fn div_2(b: u32, m: u32, k: u32) -> u32 {
        if b == 0 {panic!("expected b>=1, 0 given")}
        if Self::is_even(k) {panic!("expected k odd, {} given", k)}
        let exp = 2u32.pow(b);
        let m_mod = m % exp ;
        
        for d in 0..exp-1 {
            let kd_mod = (k * d) % exp;
            if m_mod == kd_mod {
                return d as u32;
            }
        }
        return (exp - 1) as u32;
    }

    // Unique integer d between 0 and 2^b s.t
    // (m = kd) % exp aka (m/k)%exp
    fn div(exp: u32, m: u32, k: u32) -> u32 {
        return (m/k) % exp;
        if Self::is_even(k) {panic!("expected k odd, {} given", k)}
        let m_mod = m % exp ;
        
        for d in 0..exp-1 {
            let kd_mod = (k * d) % exp;
            if m_mod == kd_mod {
                return d as u32;
            }
        }
        return (exp - 1) as u32;
    }

    
    // x**d mod m
    // fn power(x : u32, k :u32, m :u32) -> u32{
    //     let mut x = x;
    //     let mut k = k;
    //     let mut p =  1u32;
    //     if x == 1u32 { return p }
    //     x %= m;
    //     while k>= 1{
    //         if k%2 == 0 {
    //             p = (p*x) % m
    //         }
    //         x = (x.pow(2u32)) % m;
    //         k /= 2;
    //     }
    //     p
    // }


    // Power up to b bits
    // n**k mod 2**b
    fn pow_2(b: u32, x: u32, k: u32) -> u32 {
        x.pow(k) % 2u32.pow(b)
        //Self::power(x, k, 2u32.pow(b))
    }

    // Algorithm P
    

    // Algorithm C2
    fn n_eq_x_pow_k(&self, n: u32, x: u32, k: u32) -> bool {
        println!("{} == {}^{}", n, x, k);
        if x == 1 {
            return n == 1;
        }
        let mut b = 1;
        loop {
            let exp = 2u32.pow(b);
            let r = x.pow(k) % exp;
            if n % exp != r {
                return false;
            }
            if b >= self.f {
                return r == x.pow(k);
            }
            b = (2 * b).min(self.f);
        }
    }

    // Algorithm N2
    // nroot_{2,b}(y,k)
    fn nroot(b: u32, y: u32, k: u32) -> u32 {
        if Self::is_even(y)  {panic!("expected y odd, {} given", y)}
        if k == 2 {return Self::nroot_2(b, y)}
        else if Self::is_even(k) {panic!("expected k odd, {} given", k)}
        println!("b = {} y = {} k = {}", b, y ,k);
        if b == 1 {
            return 1;
        }
        let mut b = b;
        let mut vec_b = Vec::with_capacity(Self::log2_floor(b as u64) as usize);
        while b > 1{
            vec_b.push(b);
            if b%2 != 0 {
                println!(" A A A A AA");
                b += 1;
            }
            b /= 2;
            println!("In end: b = {}", b);
        }
        vec_b.reverse();
        println!("{:?}", vec_b);
        let mut z = 1;
        for b in vec_b{
            let mut r2 = Self::mul_2(b, z, k + 1)/k;
            let r3 = Self::mul_2(b, y, Self::pow_2(b, z, k+1))/k;
            let exp = 2u32.pow(b);
            while r2 < r3 {
                r2 += exp;
            }
            z = (r2 - r3) % exp;//Self::div(exp, r2 - r3, k);
            println!("z = {}", z)
        }
        z
        // let z = Self::nroot(Self::div_ceiling(b, 2), y, k);
        // let mut r2 = Self::mul_2(b, z, k + 1);
        // let r3 = Self::mul_2(b, y, Self::pow_2(b, z, k+1));
        
    }
    
    // Algorithm S2
    // nroot_{2,b}(y,2)
    fn nroot_2(b: u32, y: u32) -> u32 {
        println!("b = {} y = {} k = {}", b, y ,2);
        if Self::is_even(y) {panic!("expected y odd, {} given", y)}        
        if b == 1 {
            return if y % 4 == 1 {1} else {0};
        }
        if b == 2 {
            return if y % 8 == 1 {1} else {0};
        }
        let mut b = b;
        let mut vec_b = Vec::with_capacity(Self::log2_floor(b as u64) as usize);
        while b > 2{
            println!("b = {}", b);
            vec_b.push(b);
            b = if b%2 != 0 {
                (b+1)/2
            } else {
                1 + (b/2)
            };
        }
        vec_b.reverse();
        println!("{:?}", vec_b);
        if y % 8 != 1 {return 0}
        let mut z = 1;
        for b in vec_b {
            let mut r2 = Self::mul_2(b+1, z, 3)/2;
            println!("r2 = {}", r2);
            let r3 = Self::mul_2(b+1, y, Self::pow_2(b+1, z, 3))/2;
            println!("r3 = {}", r3);
            let exp = 2u32.pow(b);
            while r2 < r3 {
               r2 += exp;
             }
            z = ((r2 - r3)) % exp;
            println!("z = {}", z);
            if z == 0 {return 0}
        }
        z
        // let z = Self::nroot_2(Self::div_ceiling(b+1, 2), y);
        // if z == 0 {return 0}
        // let mut r2 = Self::mul_2(b+1, z, 3);
        // let r3 = Self::mul_2(b+1, y, Self::pow_2(b+1, z, 3));
        // while r2 < r3 {
        //     r2 += 2u32.pow(b);
        // }
        // ((r2 - r3)/2) % (2u32.pow(b))
    }

    // Algorithm K2
    fn is_kth_power(&self, n: u32, k: u32, y: u32) -> bool {
        if Self::is_even(n) {panic!("expected n odd, {} given", n)}
        if Self::is_even(k) && k != 2 {panic!("expected k odd or k=2, {} given", k)}
        if Self::is_even(y) {panic!("expected y odd, {} given", y)}
        let b = Self::div_ceiling(self.f, k);
        println!("{} == ?^{}", n, k);
        let mut r : u32;
        if k == 2 {
            r = Self::nroot_2(b, y);
            if r == 0 {return false}
        } else {
            r = Self::nroot(b, y, k);
        }
        println!("r = {}", r);
        if self.n_eq_x_pow_k(n, r, k) {
            return true;
        }
        if k == 2 && self.n_eq_x_pow_k(n, 2u32.pow(b) - r, k){
            return true;
        }
        false
    }

    
    // Algorithm X2
    pub fn is_perfect_power(n : u64) -> bool {
        use crate::number_theory::primes_less_than::naive::NaivePlt;
        if Self::is_even(n as u32) {panic!("expected n odd, {} given", n)}
        let test = Self::new(Self::log2_floor(2 * n));
        let b = Self::div_ceiling(test.f, 2);
        let y = Self::nroot(b + 1, n as u32, 1);
        println!("f = {} b = {} y = {}", test.f, b, y);
        for p in NaivePlt::get_primes(test.f as usize) {
            if Self::is_kth_power(&test, n as u32, p as u32, y) {
                return true;
            }
        }
        false
    }
}