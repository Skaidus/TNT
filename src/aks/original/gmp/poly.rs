use rug::{Integer, Complete};

static ZERO : Integer = Integer::ZERO;
pub struct Poly {
    coef : Vec<Integer>,
    degree : usize
}

impl Poly {

    fn get_degree(&self) -> usize {
        self.coef.len() - 1
    }

    fn get_length(&self) -> usize {
        self.degree
    }

    fn get_coeficient(&self, i : usize) -> &Integer {
        if i > self.get_degree() {return &ZERO}
        self.coef.get(i).unwrap()
    }

    fn set_coeficient(&mut self, new_coef : Integer, i : usize) {
        if i<0 {panic!("coef is less than 0!")}
        if i > self.degree {
            self.coef.resize(i+1, Integer::ZERO);
            self.degree = i;
        } 
        self.coef[i] = new_coef.clone();
    }

    pub fn new() -> Poly {
        Poly {coef : Vec::with_capacity(1), degree : 0}
    }

    pub fn with_length(initial_length : usize) -> Poly {
        Poly {coef : Vec::with_capacity(initial_length), degree : initial_length - 1}
    }

    pub fn compact( &mut self) {
        let mut i = self.degree;
        while i > 0 {
            if self.coef[i] != ZERO {break}
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
        for k in 0..polymod{
            let mut sum : Integer = Integer::ZERO;
            for i in 0..k{
                sum += x.get_coeficient(i)*y.get_coeficient(k-i);
                sum +=  x.get_coeficient(i)*y.get_coeficient(k+polymod-i);
            }
            for i in k+1..=polymod{
                sum += x.get_coeficient(i)*y.get_coeficient(k+polymod-i);
            }
            (&sum % mod_).complete_into(&mut self.coef[k]);
            if k > max_deg && sum==0u32  {break}
        }
        self.compact();
    }

    pub fn assign_self_mul_mod(&mut self , y : &Self, mod_ : &Integer, polymod : usize) {
        self.clear();
        let max_deg = self.get_degree().max(y.get_degree());
        for k in 0..polymod{
            let mut sum : Integer = Integer::ZERO;
            for i in 0..k{
                sum += self.get_coeficient(i)*y.get_coeficient(k-i);
                sum +=  self.get_coeficient(i)*y.get_coeficient(k+polymod-i);
            }
            for i in k+1..=polymod{
                sum += self.get_coeficient(i)*y.get_coeficient(k+polymod-i);
            }
            (&sum % mod_).complete_into(&mut self.coef[k]);
            if k > max_deg && sum==0u32  {break}
        }
        self.compact();
    }


    pub fn assign_square_mod(&mut self ,  mod_ : &Integer, polymod : usize) {
        self.clear();
        let max_deg = self.get_degree().max(self.get_degree());
        for k in 0..polymod{
            let mut sum : Integer = Integer::ZERO;
            for i in 0..k{
                sum += self.get_coeficient(i)*self.get_coeficient(k-i);
                sum +=  self.get_coeficient(i)*self.get_coeficient(k+polymod-i);
            }
            for i in k+1..=polymod{
                sum += self.get_coeficient(i)*self.get_coeficient(k+polymod-i);
            }
            (&sum % mod_).complete_into(&mut self.coef[k]);
            if k > max_deg && sum==0u32  {break}
        }
        self.compact();
    }

    

    pub fn assign_pow_mod(&mut self , x : &Self, power : Integer, mod_ : &Integer, polymod : usize) {
        self.clear();
        self.set_coeficient(Integer::from(1u8), 0);
        for i in (0..=power.significant_bits()).rev(){
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
        let mut coef = Vec::with_capacity(self.get_length());
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