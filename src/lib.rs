pub mod aks;
use std::vec::IntoIter;

struct MySieve {
    primes : IntoIter<bool>,
    current : u64
}
 
impl MySieve {
    pub fn new(n: usize) -> MySieve{
        let mut primes = vec![true; n];
        let mut p : usize = 2;
        while p*p<n{
            if primes[p]{
                let mut i = 2*p;
                while i<n{
                    primes[i] = false;
                    i += p;
                }
            }
            p += 1;
        }
        let mut iterprimes = primes.into_iter();
        iterprimes.next();
        iterprimes.next();
        MySieve { primes: iterprimes, current: 2 }
    }
}

impl Iterator for MySieve {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        loop {
            match self.primes.next() {
                Some(true)=> {
                    let temp = Some(self.current);
                    self.current+=1;
                    return temp;
                },
                None => {
                    return None;
                },
                _=>{self.current+=1;}                              
            }
        }
        


    }
}

// use std::collections::HashMap;
// struct Sieve {
//     composites: HashMap<u64, Vec<u64>>,
//     current: u64,
//     max: u64,
// }

// impl Sieve {
//     pub fn new(max: u64) -> Sieve {
//         Sieve {
//             composites: HashMap::new(),
//             current: 2,
//             max,
//         }
//     }

//     pub fn next_prime(&mut self) -> u64 {
//         while let Some(numbers) = self.composites.remove(&self.current) {
//             for num in numbers {
//                 self.composites
//                     .entry(self.current + num)
//                     .or_default()
//                     .push(num)
//             }
//             self.current += 1;
//         }
//         let prime = self.current;
//         self.composites.insert(prime * prime, vec![prime]);
//         self.current += 1;
//         prime
//     }
// }

// impl Iterator for Sieve {
//     type Item = u64;

//     fn next(&mut self) -> Option<u64> {
//         if self.current < self.max {
//             Some(self.next_prime())
//         } else {
//             None
//         }
//     }
// }

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

    // fn pow(x : u64, y : u64) -> u64{
    //     if y == 0 {return 1}
    //     let temp = Integer::pow(x, y/2);
    //     if y % 2 == 0 { return temp * temp}
    //     x * temp * temp
    // }

    fn logk(k: u64) -> u64 {

        (64 - k.leading_zeros() - 1).into()
        //println!("log({}) = {}", k, a);
        
    }

    fn mul_2(b: u64, m: u64, k: u64) -> u64 {
        //println!("mul_2,{} ({},{})",b,m,k);
        (k * m) % (2u64.pow(b as u32))
    }

    fn pow_2(b: u64, m: u64, k: u64) -> u64 {
        //println!("pow_2,{} ({},{})",b,m,k);
        m.pow(k as u32) % (2u64.pow(b as u32))
    }

    fn div_2(b: u64, m: u64, k: u64) -> u64 {
        //println!("div_2,{} ({},{})",b,m,k);
        let exp = 2u64.pow(b as u32);
        for d in 0..exp - 1 {
            if (m % exp) == ((k*d) % exp) {
                return d;
            }
        }
        return exp - 1;
    }
//println!("nroot_2,{}({},{})",b,y,k);
    fn nroot(b: u64, y: u64, k: u64) -> u64 {
        //println!("nroot_2,{} ({},{})",b,y,k);
        if k == 2 {
            if b == 1 {
                return match y % 4{
                    1=> 1,
                    _=> 0
                }
            } else if b == 2 {
                return match y % 8{
                    1=> 1,
                    _=> 0
                }
            }
            let z = Integer::nroot(Integer::ceil_div(b + 1, 2), y, 2);
            if z == 0 {return 0}
            let r2 = Integer::mul_2(b+1, z, 3);
            let r3 = (y * Integer::pow_2(b+1, z, 3)) % 2u64.pow((b+1) as u32);
            ((if r2 < r3 { r2 + 2u64.pow(b as u32) - r3} 
             else {r2-r3})/2) % 2u64.pow(b as u32)
            


        } else {
            if b == 1 {
                return 1;
            }
            let z = Integer::nroot(Integer::ceil_div(b , 2), y, k);
            let r2 = Integer::mul_2(b, z, k + 1);
            let r3 = (y * Integer::pow_2(b, z, k + 1)) % 2u64.pow(b as u32);
            //println!("{} - {}",r2,r3);
            Integer::div_2(b, 
                if r2 < r3 {r2 + 2u64.pow(b as u32) -r3} else {r2-r3}
                , k)
        }
    }



    // fn n_eq_x_pow_k(n: u64, x: u64, k: u64) -> bool {
    //     //println!("{} == {}^{}?",n,x,k);
    //     let f = Integer::logk((2 * n).into());
    //     if x == 1 {
    //         return n == 1;
    //     }
    //     let mut b = 1;
    //     loop {
    //         let exp = 2u64.pow(b as u32);
    //         let r = Integer::pow_2(b, x, k);
    //         //println!("r = {}",r);
    //         if n % exp != r {
    //             return false;
    //         }
    //         if b>=f {
    //             return r == x.pow(k as u32)
    //             //return Integer::n_eq_x_pow_k(r, x, k);
    //         }
    //         b = (2 * b).min(f);
    //     }
    // }

    fn is_kth_power(n: u64, k: u64, y: u64) -> bool {
        //println!("is {} a {}th power with y={}?",n,k,y);
        let f = Integer::logk((2 * n).into());
        let b = Integer::ceil_div(f, k);
        let r = Integer::nroot(b, y, k);
        if k == 2 && r == 0 {
            return false;
        }
        if 
        //Integer::n_eq_x_pow_k(n, r, k) 
        n == r.pow(k as u32)
        {
           // println!("{} == {}^{}!!!!",n,r,k);
            return r!=0;
        }
        let temp = 2u64.pow(b as u32) - r;
        if k == 2 && 
        n == temp.pow(k as u32)
        //Integer::n_eq_x_pow_k(n, temp, k)
        {
            return temp!=0;
        }
        false
    }

    pub fn is_perfect_power(&self) -> bool {
        let f = Integer::logk(2 * self.n);
        let y = Integer::nroot(Integer::ceil_div(f,2) + 1, self.n, 1);
        for p in MySieve::new(f as usize) {
            //println!("Prime p = {}", p);
            if Integer::is_kth_power(self.n, p, y) {
                return true;
            }
        }
        false
    }
}
