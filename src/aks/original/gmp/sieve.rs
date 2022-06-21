use rug::Integer;

pub struct Sieve {
    table : Integer,
    pub size : u32
}

impl Sieve {
    pub fn new() -> Sieve{
        Sieve{ table : Integer::new(), size : 2}
    }
    pub fn is_prime(&mut self, r : &Integer) -> bool {
        let rul = r.to_u32().unwrap();
        if self.size >= rul {return !self.table.get_bit(rul)}
        self.size *= 2;
        for i in 2..=self.size{
            if !self.table.get_bit(i){
                let mut j = 2*i;
                while j <= self.size {
                    self.table.set_bit(j, true);
                    j += i;
                }
            }
        }
        !self.table.get_bit(rul)
    }
}