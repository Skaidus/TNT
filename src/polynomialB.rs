use std::fmt;
use num::traits::bounds::LowerBounded;

use crate::{number_theory::util, polynomial};

/// A representation of a polynomial in the ring `(x^r - 1, n)`
pub struct Poly {
    mod_r : usize, 
    mod_n : usize,
    degree : usize,
    coefficients : Vec<usize>
}

impl Poly {
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

    fn allocate(mod_r : usize, mod_n : usize)-> Poly{
        let mut errors = Vec::with_capacity(2);
        if mod_r == 0 {
            errors.push(format!("\nExpected mod_r  > 0 ({} given)", mod_r));
        }
        if mod_n < 2 {
            errors.push(format!("\nExpected mod_n > 1 ({} given)", mod_n));
        }
        Poly {mod_r, mod_n, degree: 0, coefficients: vec![0]  }
    }

    pub fn with_capacity(initial_degree : usize, mod_r : usize, mod_n : usize)-> Poly{
        let mut errors = Vec::with_capacity(2);
        if mod_r == 0 {
            errors.push(format!("\nExpected mod_r  > 0 ({} given)", mod_r));
        }
        if mod_n < 2 {
            errors.push(format!("\nExpected mod_n > 1 ({} given)", mod_n));
        }
        Poly {mod_r, mod_n, degree: initial_degree, coefficients: vec![0; initial_degree+1] }
    }

    fn get_coef(&self, i : usize)-> usize{
        if i > self.degree {
            0
        } else {
            self.coefficients[i]
        }
    }

    pub fn set_coef(&mut self, new_coef : usize, i : usize){
        if i > self.degree {
            self.coefficients.resize(i+1, 0);
            self.degree = i;
        }
        self.coefficients[i] = new_coef;
    }

    fn compact(&mut self){
        let mut i = self.degree;
        while i > 0 {
            if self.coefficients[i] != 0 {
                break
            }
            i-=1;
        }
        self.coefficients.truncate(i+1);
        if self.degree != i {
            self.degree = i;
        }
    }

    fn clear(&mut self){
        self.coefficients.truncate(1);
        self.coefficients[0] = 0;
        self.degree = 0;
    }

    // polymod = r, mod = n
    // fn mod_mult(&mut self, x : &Self, y : &Self, mod_r : usize, mod_n : usize) {
    //     let x = x.clone();
    //     let y = y.clone();
    //     let xdeg = x.degree;
    //     let ydeg = y.degree;
    //     self.clear();
    //     let maxdeg = xdeg.max(ydeg);
    //     for k in 0..mod_r{
    //         let mut sum = 0;
    //         for i in 0..=k{
    //             sum += x.get_coef(i)*(y.get_coef(k-i) + y.get_coef(k+mod_r-i));
    //         }
    //         for i in k+1..=k+mod_r{
    //             sum += x.get_coef(i)*y.get_coef(k+mod_r-i);
    //         }
    //         self.set_coef(sum % mod_n, k);
    //         if k > maxdeg && sum == 0 {break}
    //     }
    //     self.compact();
    // }

    fn mod_mult(&mut self, y : &Self) {
        let x = self.clone();
        let y = y.clone();
        let xdeg = x.degree;
        let ydeg = y.degree;
        self.clear();
        let maxdeg = xdeg.max(ydeg);
        for k in 0..self.mod_r{
            let mut sum = 0;
            for i in 0..=k{
                sum += x.get_coef(i)*(y.get_coef(k-i) + y.get_coef(k+self.mod_r-i));
            }
            for i in k+1..=k+self.mod_r{
                sum += x.get_coef(i)*y.get_coef(k+self.mod_r-i);
            }
            self.set_coef(sum % self.mod_n, k);
            if k > maxdeg && sum == 0 {break}
        }
        self.compact();
    }

    fn mod_square(&mut self) {
        let x = self.clone();
        self.clear();
        let maxdeg = x.degree;
        for k in 0..self.mod_r{
            let mut sum = 0;
            for i in 0..=k{
                sum += x.get_coef(i)*(x.get_coef(k-i) + x.get_coef(k+self.mod_r-i));
            }
            for i in k+1..=k+self.mod_r{
                sum += x.get_coef(i)*x.get_coef(k+self.mod_r-i);
            }
            self.set_coef(sum % self.mod_n, k);
            if k > maxdeg && sum == 0 {break}
        }
        self.compact();
    }

    // pub fn mod_pow(&self, power : u32) -> Self{
    //     let mut res = Self::allocate(self.mod_r, self.mod_n);
    //     res.set_coef(1, 0);
    //     let log_pow = util::log2_floor(power);
    //     for i in (0..=log_pow).rev(){
    //         res.mod_square();
    //         if (power >> i) & 1 == 1 {
    //             res.mod_mult(self);
    //         }
    //         if i == 0 {break}
    //     }
    //     res.compact();
    //     res
    // }

    // pub fn mod_pow(&self, power : u32) -> Self{
    //     let mut res = Self::allocate(self.mod_r, self.mod_n);
    //     res.set_coef(1, 0);
    //     let mut base = self.clone();
    //     let mut power = power;
    //     while power > 0 {
    //         if power % 2 == 1 {
    //             res.mod_mult(&base);
    //         }
    //         base.mod_square();
    //         power /= 2;
    //     }
    //     res.compact();
    //     res
    pub fn mod_pow(&self, power : u32) -> Self{
        let mut res = Self::allocate(self.mod_r, self.mod_n);
        res.set_coef(1, 0);

        let mut a = self.clone();
        let mut power = power;
        while power > 0 {
            res.mod_square();
            if (power & 1) == 1 {
                res.mod_mult(&a);
            }
            power >>= 1;
        }
        res.compact();
        println!("{:?}", res.coefficients);
        res
        // let mut res = Self::allocate(self.mod_r, self.mod_n);
        // res.set_coef(1, 0);

        // let mut a = self.clone();
        // let mut power = power;
        // while power > 0 {
        //     if (power & 1) == 1 {
        //         res.mod_mult(&a);
        //     }
        //     a.mod_square();
        //     power >>= 1;
        // }
        // res.compact();
        // res
        // res.set_coef(1, 0);
        // let log_pow = util::log2_floor(power);
        // for i in (0..=log_pow).rev(){
        //     res.mod_square();
        //     if (power >> i) & 1 == 1 {
        //         res.mod_mult(self);
        //     }
        //     if i == 0 {break}
        // }
        // res.compact();
        // res
    }


    // pub fn mod_pow(&self, power : u32) -> Self{
    //     let mut res = Self::allocate(self.mod_r, self.mod_n);
    //     res.set_coef(1, 0);
    //     // let mut q = power;
    //     // while q > 0 {
    //     //     if q % 2 == 1{
    //     //         res.mod_mult(self);
    //     //     }
    //     //     res.mod_square();
    //     // }
    //     for i in (0..=log_pow).rev(){
    //         res.mod_square();
    //         if (log_pow >> i) & 1 == 1 {
    //             res.mod_mult(self);
    //         }
    //         if i == 0 {break}
    //     }
    //     res.compact();
    //     res
    // }

    


}

impl Clone for Poly {
    fn clone(&self) -> Poly {
        let mut coefficients = vec![0; self.degree + 1];
        for i in 0..=self.degree{
            coefficients[i] = self.coefficients[i];
        }
        Poly {mod_r : self.mod_n, mod_n :self.mod_n,  coefficients , degree : self.degree}
    }
}

use std::ops::{AddAssign, SubAssign}; 

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