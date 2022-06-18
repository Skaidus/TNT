use std::collections::HashMap;

struct Sieve {
    composites: HashMap<u64, Vec<u64>>,
    current: u64,
    max: u64
}

impl Sieve {

    pub fn new(max : u64) -> Sieve {
        Sieve {
            composites: HashMap::new(),
            current: 2,
            max
        }
    }

    pub fn next_prime(&mut self) -> u64 {
        while let Some(numbers) = self.composites.remove(&self.current) {
            for num in numbers {
                self.composites
                    .entry(self.current + num)
                    .or_default()
                    .push(num)
            }
            self.current += 1;
        }
        let prime = self.current;
        self.composites.insert(prime * prime, vec![prime]);
        self.current += 1;
        prime
    }
}

impl Iterator for Sieve {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.current < self.max {
            Some(self.next_prime())
        } else {
            None
        }
        
    }
}

pub struct Integer {
    n : u64,
    logn : u32
}

impl Integer {
    pub fn new(n : u64) -> Integer {
        if n==0 {
            panic!("Tested number can't be lower than 1 ")
        }
        let logn  = Integer::logk(n);
        Integer { n, logn }
    }

    fn logk(k : u64) -> u32 {
        64 - k.leading_zeros() - 1
    }

    fn mul_2(b : u32, m: u32, k: u32) -> u32{
        (k*m) % (2^b)
    }

    fn pow_2(b : u32, m: u32, k: u32) -> u32{
        (m^k) % (2^b)
    }

    fn div_2(b : u32, m: u32, k: u32) -> u32{
        let exp = 2^b;
        for d in 0..exp-1{
            if m == Integer::pow_2(b, d, k){return d}
        }
        return exp-1
    }

    fn nroot(y : u32, b : u32, k : u32) -> u32 {
        if b == 1 {return 1}
        let z = Integer::nroot(y, b / 2, k);
        let r2 = Integer::mul_2(b, z, k+1);
        let r3 = y * Integer::pow_2(b, z, k+1);
        Integer::div_2(b, r2-r3, k)
    }

    fn x_pow_k_eq_n(n : u32, x : u32, k : u32) -> bool{
        let f = Integer::logk((2*n).into());
        if x==1{return false}

    }

    fn is_kth_power(n : u32, k : u32, y : u32) -> bool{
        let f = Integer::logk((2*n).into());
        let b = f / k;
        let r = Integer::nroot(y, b, k);
        if k == 2 && r == 0 {return false}

        false 
    }



    pub fn is_perfect_power(&self) -> bool{
        let f = Integer::logk(2*self.n);
        let y = Integer::nroot(f/2 + 1, self.n as u32, 1);
        for s in Sieve::new(f.into()){
            let x = 

        }
        true


    }


}