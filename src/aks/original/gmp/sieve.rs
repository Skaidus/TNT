use rug::Integer;
use gmp_mpfr_sys::gmp;

struct Sieve {
    table : Integer,
    size : u32
}

impl Sieve {
    fn new() -> Sieve{
        Sieve{ table : Integer::new(), size : 2}
    }
    fn is_prime(&mut self, r : Integer) -> bool {
        let rul = r.to_u32().unwrap();
        if self.size >= rul {return !self.table.get_bit(rul)}
        self.size *= 2;
        for i in 2..=self.size{
            if !self.table.get_bit(i){
                for j in (2*i..=self.size).step_by(i as usize){
                    self.table.set_bit(j, true);
                }
            }
        }
        !self.table.get_bit(rul)
    }
}