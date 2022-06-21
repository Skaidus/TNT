use rug::Integer;

static ZERO : Integer = Integer::ZERO;
pub struct Poly {
    coef : Vec<Integer>
}

impl Poly {

    fn get_degree(&self) -> usize {
        self.coef.len() - 1
    }

    fn get_length(&self) -> usize {
        self.coef.len()
    }

    fn get_coeficient(&self, i : usize) -> &Integer {
        if i > self.get_degree() {return &ZERO}
        self.coef.get(i).unwrap()
    }

    pub fn new() -> Poly {
        Poly {coef : Vec::with_capacity(1)}
    }

    pub fn with_length(initial_length : usize) -> Poly {
        Poly {coef : Vec::with_capacity(initial_length)}
    }
}

impl Clone for Poly {
    fn clone(&self) -> Poly {
        let mut coef = Vec::with_capacity(self.get_length());
        for i in 0..self.get_length(){
            coef[i] = self.get_coeficient(i).clone();
        }
        Poly { coef }
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