use std::fmt;
use crate::number_theory::util;

/// A representation of a polynomial in the ring `(x^r - 1, n)`
pub struct Poly {
    mod_r : usize, 
    mod_n : usize,
    degree : usize,
    coefficients : Vec<usize>
}

impl Poly {
    // fn allocate()-> Poly{
    //     Poly { mod_r: 0, mod_n: 0, degree: 0, coefficients: Vec::new() }
    // }

    /// Creates a new polynomial in `(x^r - 1, n)`
    /// default to `x`
    /// # Examples
    /// ```
    /// use aks_primes::polynomial::Poly;
    /// let poly = Poly::new(5, 6);
    /// assert_eq!(format!("{}",poly), "(x) mod (x^5 - 1, 6)")
    /// ```
    pub fn new(mod_r : usize, mod_n : usize)-> Poly{
        let mut errors = Vec::with_capacity(2);
        if mod_r == 0 {
            errors.push(format!("\nExpected mod_r  > 0 ({} given)", mod_r));
        }
        if mod_n < 2 {
            errors.push(format!("\nExpected mod_n > 1 ({} given)", mod_n));
        }
        if !errors.is_empty() {
            let errors = errors.join("");
            panic!("{}",errors);
        }
        Poly { mod_r, mod_n, degree: 1, coefficients: vec![0,1]  }
    }
    /// Raises the polynomial to the `exp` power in the `(x^r - 1, n)` domain
    /// # Examples
    /// ```
    ///     use aks_primes::polynomial::Poly;
    ///     let mut poly = Poly::new(5, 5);
    ///     poly += 1; // x + 1
    ///     poly = poly.pow(2);
    ///     assert_eq!(format!("{}", poly),"(x^2 + 2x + 1) mod (x^5 - 1, 5)")
    /// ```
    pub fn pow(&self, exp : u32) -> Poly{
        let mut er = 1u32;
        let mut i = exp;
        while i != 1{
            er = 2 * er + (i % 2);
            i /= 2;
        }
        let mut t = self.clone();
        i = er;
        println!("t = {}", t);
        while i != 1{
            t._poly_square();
            t._mod();
            if i % 2 != 0{
                t *= self;
                t._mod();
            }
            i /= 2;
        }
        t._norm();
        t
    }

    // fn _poly_square(&mut self){
    //     let new_degree = self.degree + self.degree;
    //     let mut new_coef : Vec<usize> = vec![0; new_degree+1];
    //     for j in 1..=self.degree{
    //         let x = self.coefficients[j];
    //         if x != 0 {
    //             for i in 0..j{
    //                 let y = 2 * self.coefficients[i];
    //                 let t = util::umuladdrem(x, y, new_coef[j+i], self.mod_n);
    //                 new_coef[j+i] = t;
    //             }
    //         }
    //     }
    //     for i in 0..=self.degree{
    //         let x = self.coefficients[i];
    //         let t = util::umuladdrem(x, x, new_coef[2*i], self.mod_n);
    //         new_coef[2 * i] = t;
    //     }
    //     self.degree = new_degree;
    //     self.coefficients = new_coef;
    // }

    fn _poly_square(&mut self){
        let new_degree = self.degree + self.degree;
        let mut new_coef : Vec<usize> = vec![0; new_degree+1];
        for j in 1..=self.degree{
            let x = self.coefficients[j];
            if x != 0 {
                for i in 0..j{
                    let y = 2 * self.coefficients[i];
                    let t = util::umuladdrem(x, y, new_coef[j+i], self.mod_n);
                    new_coef[j+i] = t;
                }
            }
        }
        for i in 0..=self.degree{
            let x = self.coefficients[i];
            let t = util::umuladdrem(x, x, new_coef[2*i], self.mod_n);
            new_coef[2 * i] = t;
        }
        self.degree = new_degree;
        self.coefficients = new_coef;
    }

    fn _mod(&mut self){
        println!("degree {}, coef {:?}, me {}", self.degree, self.coefficients,self);
        let mut new_degree = self.degree;
        while new_degree >= self.mod_r {
            self.coefficients[new_degree - self.mod_r] += self.coefficients[new_degree];
            if self.coefficients[new_degree - self.mod_r] >= self.mod_n {
                self.coefficients[new_degree - self.mod_r] -= self.mod_n;
            }
            new_degree -= 1;
            while self.coefficients[new_degree] == 0 {
                new_degree -= 1;
            }
        }
        self.degree = new_degree;
    }

    fn _norm(&mut self){
        if self.coefficients[self.degree] != 1 {
            let y = util::inv(self.coefficients[self.degree], self.mod_n);
            for i in 0..=self.degree{
                let t = self.coefficients[i];
                self.coefficients[i] = util::umulrem(t, y, self.mod_n);
            }
        }
    }

}

impl Clone for Poly {
    fn clone(&self) -> Poly {
        let mut coefficients = vec![0; self.degree + 1];
        for i in 0..=self.degree{
            coefficients[i] = self.coefficients[i];
        }
        Poly { mod_n : self.mod_n, mod_r : self.mod_r, coefficients , degree : self.degree}
    }
}

use std::ops::{AddAssign, SubAssign, MulAssign}; 

impl AddAssign<usize> for Poly {
    fn add_assign(&mut self, rhs: usize) {
        let t = rhs % self.mod_n;
        self.coefficients[0] += t;
        if self.coefficients[0] >= self.mod_n {self.coefficients[0] -= self.mod_n;}
    }
}

impl SubAssign<usize> for Poly {
    fn sub_assign(&mut self, rhs: usize) {
        let t = self.mod_n - rhs % self.mod_n;
        self.coefficients[0] += t;
        if self.coefficients[0] >= self.mod_n {self.coefficients[0] -= self.mod_n;}
    }
}

impl MulAssign<&Poly> for Poly {
    fn mul_assign(&mut self, rhs: &Self) {
        let new_degree = self.degree + rhs.degree;
        let mut new_coef : Vec<usize> = vec![0; new_degree+1];
        for j in 0..=rhs.degree{
            let x = rhs.coefficients[j];
            if x != 0 {
                for i in 0..self.degree{
                    let y = self.coefficients[i];
                    let t = util::umuladdrem(x, y, new_coef[j+i], self.mod_n);
                    new_coef[j+i] = t;
                }
            }
        }
        self.degree = new_degree;
        self.coefficients = new_coef;
    }
}

impl PartialEq for Poly {
    fn eq(&self, other: &Self) -> bool {
        if other.degree != self.degree { return false }
        for i in 0..=self.degree{
            if other.coefficients[i] != self.coefficients[i]{
                return false
            }
        }
        true
    }
}
impl Eq for Poly {}

impl fmt::Display for Poly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = Vec::with_capacity(self.coefficients.len());
        for i in (0..self.coefficients.len()).rev(){
            if self.coefficients[i] != 0 {
                output.push(format!("{}{}",
                if (self.coefficients[i]==1)&&(i>0) {
                    "".to_string()
                } else {
                    self.coefficients[i].to_string()
                },
                match i {
                    0  => "".to_string(),
                    1  => "x".to_string(),
                    other => format!("x^{}",other)
                }))
            }
            
        }
        write!(f, "({}) mod (x{} - 1, {})", 
        output.join(" + "), 
        if self.mod_r > 1 {format!("^{}",self.mod_r)} else {"".to_string()}, 
        self.mod_n)
    }
}