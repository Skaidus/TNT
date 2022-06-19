use std::collections::HashMap;
const TWO : u64 = 2;
struct Sieve {
    composites: HashMap<u64, Vec<u64>>,
    current: u64,
    max: u64,
}

impl Sieve {
    pub fn new(max: u64) -> Sieve {
        Sieve {
            composites: HashMap::new(),
            current: 2,
            max,
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
    n: u64,
//    logn: u32,
}

impl Integer {
    pub fn new(n: u64) -> Integer {
        if n == 0 {
            panic!("Tested number can't be lower than 1 ")
        }
        //let logn = Integer::logk(n);
        Integer { n
            //, logn 
        }
    }

    fn ceil_div(p : u64, q : u64)->u64{
        p/q + if p % q !=0 {1} else {0}
    }

    fn pow(x : u64, y : u64) -> u64{
        if y == 0 {return 1}
        let temp = Integer::pow(x, y/2);
        if y % 2 == 0 { return temp * temp}
        x * temp * temp
    }

    fn logk(k: u64) -> u64 {
        (64 - k.leading_zeros() - 1).into()
    }

    fn mul_2(b: u64, m: u64, k: u64) -> u64 {
        println!("{}*{}%2^{}",k,m,b);
        (k * m) % (Integer::pow(2, b))
    }

    fn pow_2(b: u64, m: u64, k: u64) -> u64 {
        (Integer::pow(m, k)) % (Integer::pow(2, b))
    }

    fn div_2(b: u64, m: u64, k: u64) -> u64 {
        let exp = Integer::pow(2, b);
        for d in 0..exp - 1 {
            if m == Integer::pow_2(b, d, k) {
                return d;
            }
        }
        return exp - 1;
    }

    fn nroot(b: u64, y: u64, k: u64) -> u64 {
        if b == 1 {
            return 1;
        }
        let z = Integer::nroot(Integer::ceil_div(b , 2), y, k);
        let r2 = Integer::mul_2(b, z, k + 1);
        let r3 = (y * Integer::pow_2(b, z, k + 1)) % (2^b);
        Integer::div_2(b, r2 - r3, k)
    }

    fn x_pow_k_eq_n(n: u64, x: u64, k: u64) -> bool {
        let f = Integer::logk((2 * n).into());
        if x == 1 {
            return n == 1;
        }
        let mut b = 1;
        let exp = 2 ^ b;
        let mut x_pow_k_eq_r = false;
        while b < f {
            let r = Integer::pow_2(b, x, k);
            x_pow_k_eq_r = Integer::x_pow_k_eq_n(n, r, k);
            if n % exp != r {
                return false;
            }
            b = (2 * b).min(f);
        }
        x_pow_k_eq_r
    }

    fn is_kth_power(n: u64, k: u64, y: u64) -> bool {
        let f = Integer::logk((2 * n).into());
        let b = Integer::ceil_div(f, k);
        let r = Integer::nroot(y, b, k);
        if k == 2 && r == 0 {
            return false;
        }
        if Integer::x_pow_k_eq_n(n, r, k) {
            return true;
        }
        if k == 2 {
            return Integer::x_pow_k_eq_n(n, (2 ^ b) - r, k);
        }
        false
    }

    pub fn is_perfect_power(&self) -> bool {
        let f = Integer::logk(2 * self.n);
        let y = Integer::nroot(Integer::ceil_div(f,2) + 1, self.n, 1);
        for p in Sieve::new(f.into()) {
            if Integer::is_kth_power(self.n, p, y) {
                return true;
            }
        }
        false
    }
}
