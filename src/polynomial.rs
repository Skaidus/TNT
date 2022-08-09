use std::fmt;

/// A representation of a polynomial in the ring (x^r - 1, n)
pub struct Poly {
    mod_r : usize, 
    mod_n : usize,
    degree : usize,
    coefficients : Vec<usize>
}

impl fmt::Display for Poly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = Vec::with_capacity(self.coefficients.len());
        for i in 0..self.coefficients.len(){
            if self.coefficients[i] != 0 {
                output.push(format!("{}{}",
                match self.coefficients[i] {
                    1 => "".to_string(),
                    other => other.to_string()
                },
                match i {
                    0  => "".to_string(),
                    1  => "x".to_string(),
                    other => format!("x ^{}",other)
                }))
            }
            
        }
        write!(f, "({}) mod (x ^ {} - 1, {})", output.join(" + "), self.mod_r, self.mod_n)
    }
}

impl Poly {
    fn allocate()-> Poly{
        Poly { mod_r: 0, mod_n: 0, degree: 0, coefficients: Vec::new() }
    }
    /// Creates a new polynomial in the ring (x^r - 1, n) 
    /// default to x
    /// # Examples
    /// ```
    /// use aks_primes::polynomial::Poly;
    /// let poly = Poly::new(5, 6);
    /// ```
    pub fn new(mod_r : usize, mod_n : usize)-> Poly{
        Poly { mod_r, mod_n, degree: 1, coefficients: vec![0,1]  }
    }

    /// Shortcut for (x * y) % m
    fn _umulrem(x : usize, y : usize, m : usize) -> usize{
        (x * y) % m
    }
    /// Shortcut for (x * y + a) % m
    fn _umuladdrem(x : usize, y : usize, a : usize, m : usize) -> usize{
        (x * y + a) % m
    }
    /// Squares the polynomial and stores the result in 
    /// the instance. 
    /// # Examples
    /// ```
    /// use aks_primes::polynomial::Poly;
    /// let poly = Poly::new(5, 6);
    /// ```
    fn _poly_square(&mut self){
        let new_degree = self.degree + self.degree;
        let mut new_coef : Vec<usize> = vec![0; new_degree+1];
        for j in 1..=self.degree{
            let x = self.coefficients[j];
            if x != 0 {
                for i in 0..j{
                    let y = 2 * self.coefficients[i];
                    let t = Poly::_umuladdrem(x, y, new_coef[j+i], self.mod_n);
                    new_coef[j+i] = t;
                }
            }
        }
        for i in 0..=self.degree{
            let x = self.coefficients[i];
            let t = Poly::_umuladdrem(x, x, new_coef[2*i], self.mod_n);
            new_coef[2 * i] = t;
        }
        self.degree = new_degree;
        self.coefficients = new_coef;
    }
    // pub fn ceil_logk(k : &Integer) -> u32{
    //     k.significant_bits() - (k.is_even() as u32)
    // }

    // pub fn floor_logk(k : u32) -> u32{
    //     32 - k.leading_zeros()    
    // }

    // pub fn from_n_and_r(r : u32, n : usize ) -> Poly {
    //     Poly { r, n, degree: 1, coef: vec![0,1] }
    // }


    // fn get_degree(&self) -> u32 {
    //     self.degree
    // }

    // fn get_coeficient(&self, i : u32) -> usize {
    //     if i > self.get_degree() {return 0}
    //     self.coef[i as usize]
    // }

    // pub fn set_coeficient(&mut self, new_coef : &Integer, i : usize) {
    //     //println!("{}", self.coef.len());
    //     if i > self.degree {
    //         self.coef.resize(i+1, Integer::ZERO);
    //         self.degree = i;
    //     } 
    //     self.coef[i] = new_coef.clone();
    // }

    // fn compact( &mut self) {
    //     let mut i = self.degree;
        
    //     while i > 0 {
    //         if self.coef[i] != Integer::ZERO {break}
    //         i -= 1;
    //     }
    //     if self.degree != i {
    //         self.coef.truncate(i + 1);
    //         self.coef.shrink_to(i + 1);
    //         self.degree = i;
    //     }
    // }

    // fn polySquare(&mut self){

    //     let b = Self::floor_logk(self.degree+1) + 2 * Self::floor_logk(self.n.try_into().unwrap())/Self::floor_logk(2) + 1;
    //     let mut x = Integer::from(0);
    //     for i in (1..=self.degree).rev(){
    //         x += self.coef[i as usize];
    //         x *= 2_u32.pow(i);
    //     }
    //     x += self.coef[0];
    //     x = x.square();
    //     let new_degree = 2 * self.degree;
    //     let mut new_coef : Vec<usize> = Vec::with_capacity(new_degree as usize + 1);
    //     let mut t : Integer = Integer::from(0);
    //     for i in 0usize..=new_degree as usize{
    //         t = x/2u32.pow(b);
    //         t = t / self.n;
    //         new_coef[i] = t.to_usize().unwrap();
    //         x %= b;
    //     }
    //     self.degree = new_degree;
    //     self.coef = new_coef;
    // }

    // pub fn clear(&mut self) {
    //     self.coef.truncate(1);
    //     self.coef.shrink_to(1);
    //     self.degree = 0;
    // }

    // pub fn assign_mul_mod(&mut self , x : &Self, y : &Self, mod_ : &Integer, polymod : usize) {
    //     self.clear();
    //     let max_deg = x.get_degree().max(y.get_degree());
    //     let mut rem_t;
    //     for k in 0..polymod{
    //         let mut sum : Integer = Integer::ZERO;
    //         for i in 0..=k{
    //             sum += x.get_coeficient(i)*y.get_coeficient(k-i);
    //             sum +=  x.get_coeficient(i)*y.get_coeficient(k+polymod-i);
    //         }
    //         for i in k+1..=k+polymod{
    //             sum += x.get_coeficient(i)*y.get_coeficient(k+polymod-i);
    //         }
    //         rem_t = Integer::from(&sum % mod_);
    //         self.set_coeficient(&rem_t,k);
    //         if k > max_deg && sum==0u32  {break}
    //     }
    //     self.compact();
    // }

    // pub fn assign_self_mul_mod(&mut self , y : &Self, mod_ : &Integer, polymod : usize) {
    //     self.clear();
    //     let max_deg = self.get_degree().max(y.get_degree());
    //     let mut rem_t;
    //     for k in 0..polymod{
    //         let mut sum : Integer = Integer::ZERO;
    //         for i in 0..=k{
    //             sum += self.get_coeficient(i)*y.get_coeficient(k-i);
    //             sum +=  self.get_coeficient(i)*y.get_coeficient(k+polymod-i);
    //         }
    //         for i in k+1..=k+polymod{
    //             sum += self.get_coeficient(i)*y.get_coeficient(k+polymod-i);
    //         }
    //         rem_t = Integer::from(&sum % mod_);
    //         self.set_coeficient(&rem_t,k);
    //         if k > max_deg && sum==0u32  {break}
    //     }
    //     self.compact();
    // }


    // pub fn assign_square_mod(&mut self ,  mod_ : &Integer, polymod : usize) {
    //     self.clear();
    //     let max_deg = self.get_degree().max(self.get_degree());
    //     let mut rem_t;
    //     for k in 0..polymod{
    //         let mut sum : Integer =Integer::ZERO;
    //         for i in 0..=k{
    //             sum += self.get_coeficient(i)*self.get_coeficient(k-i);
    //             sum +=  self.get_coeficient(i)*self.get_coeficient(k+polymod-i);
    //         }
    //         for i in k+1..=k+polymod{
    //             sum += self.get_coeficient(i)*self.get_coeficient(k+polymod-i);
    //         }
    //         rem_t = Integer::from(&sum % mod_);
    //         self.set_coeficient(&rem_t,k);
    //         if k > max_deg && sum==0u32  {break}
    //     }
    //     self.compact();
    // }

    

    // pub fn assign_pow_mod(&mut self , x : &Self, power : &Integer, mod_ : &Integer, polymod : usize) {
    //     self.clear();
        
    //     self.set_coeficient(&Integer::from(1u32), 0);
    //     for i in (0..=Self::ceil_logk(&power)).rev(){
    //         self.assign_square_mod(mod_, polymod);
    //         if power.get_bit(i){
    //             self.assign_self_mul_mod(x, mod_, polymod)
    //         }
    //         if i == 0 {break}
    //     }
    //     self.compact();
    // }

}

impl Clone for Poly {
    fn clone(&self) -> Poly {
        let mut coefficients = Vec::with_capacity(self.degree+1);
        for i in 0..=self.degree{
            coefficients[i] = self.coefficients[i];
        }
        Poly { mod_n : self.mod_n, mod_r : self.mod_r, coefficients , degree : self.degree}
    }
}

// impl PartialEq for Poly {
//     fn eq(&self, other: &Self) -> bool {
//         if other.get_degree() != self.get_degree() { return false }
//         for i in 0..=self.get_degree(){
//             if other.get_coeficient(i) != self.get_coeficient(i){
//                 return false
//             }
//         }
//         true
//     }
// }
// impl Eq for Poly {}