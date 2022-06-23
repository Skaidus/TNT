use rug::{Integer, Complete};
pub struct Poly {
    coef : Vec<Integer>,
    degree : usize
}

static ZERO : Integer = Integer::ZERO;

impl Poly {

    pub fn ceil_logk(k : &Integer) -> u32{
        k.significant_bits() - (k.is_even() as u32)
    }


    pub fn new() -> Poly {
        let mut coef = Vec::with_capacity(1);
        coef.resize(1, Integer::ZERO);
        Poly {coef, degree : 0}
    }

    pub fn with_length(initial_length : usize) -> Poly {
        let mut coef = Vec::with_capacity(initial_length+1);
        coef.resize(initial_length+1, Integer::ZERO);
        Poly {coef , degree : initial_length }
    }

    fn get_degree(&self) -> usize {
        self.degree
    }

    fn get_coeficient(&self, i : usize) -> &Integer {
        if i > self.get_degree() {return &ZERO}
        self.coef.get(i).unwrap()
    }

    pub fn set_coeficient(&mut self, new_coef : &Integer, i : usize) {
        //println!("{}", self.coef.len());
        if i > self.degree {
            self.coef.resize(i+1, Integer::ZERO);
            self.degree = i;
        } 
        self.coef[i] = new_coef.clone();
    }

    fn compact( &mut self) {
        let mut i = self.degree;
        while i > 0 {
            if self.coef[i] != Integer::ZERO {break}
            i -= 1;
        }
        if self.degree != i {
            self.coef.truncate(i + 1);
            self.coef.shrink_to(i + 1);
            self.degree = i;
        }
    }

    pub fn clear(&mut self) {
        self.coef.truncate(1);
        self.coef.shrink_to(1);
        self.degree = 0;
    }

    pub fn assign_mul_mod(&mut self , x : &Self, y : &Self, mod_ : &Integer, polymod : usize) {
        self.clear();
        let max_deg = x.get_degree().max(y.get_degree());
        let mut rem_t;
        for k in 0..polymod{
            let mut sum : Integer = Integer::ZERO;
            for i in 0..=k{
                sum += x.get_coeficient(i)*y.get_coeficient(k-i);
                sum +=  x.get_coeficient(i)*y.get_coeficient(k+polymod-i);
            }
            for i in k+1..=k+polymod{
                sum += x.get_coeficient(i)*y.get_coeficient(k+polymod-i);
            }
            rem_t = Integer::from(&sum % mod_);
            self.set_coeficient(&rem_t,k);
            if k > max_deg && sum==0u32  {break}
        }
        self.compact();
    }

    pub fn assign_self_mul_mod(&mut self , y : &Self, mod_ : &Integer, polymod : usize) {
        self.clear();
        let max_deg = self.get_degree().max(y.get_degree());
        let mut rem_t;
        for k in 0..polymod{
            let mut sum : Integer = Integer::ZERO;
            for i in 0..=k{
                sum += self.get_coeficient(i)*y.get_coeficient(k-i);
                sum +=  self.get_coeficient(i)*y.get_coeficient(k+polymod-i);
            }
            for i in k+1..=k+polymod{
                sum += self.get_coeficient(i)*y.get_coeficient(k+polymod-i);
            }
            rem_t = Integer::from(&sum % mod_);
            self.set_coeficient(&rem_t,k);
            if k > max_deg && sum==0u32  {break}
        }
        self.compact();
    }


    pub fn assign_square_mod(&mut self ,  mod_ : &Integer, polymod : usize) {
        self.clear();
        let max_deg = self.get_degree().max(self.get_degree());
        let mut rem_t;
        for k in 0..polymod{
            let mut sum : Integer =Integer::ZERO;
            for i in 0..=k{
                sum += self.get_coeficient(i)*self.get_coeficient(k-i);
                sum +=  self.get_coeficient(i)*self.get_coeficient(k+polymod-i);
            }
            for i in k+1..=k+polymod{
                sum += self.get_coeficient(i)*self.get_coeficient(k+polymod-i);
            }
            rem_t = Integer::from(&sum % mod_);
            self.set_coeficient(&rem_t,k);
            if k > max_deg && sum==0u32  {break}
        }
        self.compact();
    }

    

    pub fn assign_pow_mod(&mut self , x : &Self, power : &Integer, mod_ : &Integer, polymod : usize) {
        self.clear();
        
        self.set_coeficient(&Integer::from(1u32), 0);
        for i in (0..=Self::ceil_logk(&power)).rev(){
            self.assign_square_mod(mod_, polymod);
            if power.get_bit(i){
                self.assign_self_mul_mod(x, mod_, polymod)
            }
            if i == 0 {break}
        }
        self.compact();
    }

}

impl Clone for Poly {
    fn clone(&self) -> Poly {
        let mut coef = Vec::with_capacity(self.get_degree()+1);
        for i in 0..=self.degree{
            coef[i] = self.get_coeficient(i).clone();
        }
        Poly { coef , degree : self.degree}
    }
}

impl PartialEq for Poly {
    fn eq(&self, other: &Self) -> bool {
        if other.get_degree() != self.get_degree() { return false }
        for i in 0..=self.get_degree(){
            if other.get_coeficient(i) != self.get_coeficient(i){
                return false
            }
        }
        true
    }
}
impl Eq for Poly {}