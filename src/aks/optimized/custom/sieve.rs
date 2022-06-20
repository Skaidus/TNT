use std::vec::IntoIter;

pub struct MySieve {
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