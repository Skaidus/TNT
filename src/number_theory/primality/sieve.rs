pub struct Sieve {
}

impl Sieve {
    pub fn get_primes(n : usize) -> Vec<usize>{
        let mut is_prime = vec![true;n+1];
        let mut primes = Vec::with_capacity(n/(64 - n.leading_zeros() as usize));
        is_prime[0] = false;
        is_prime[1] = false;
        let mut i = 3;
        while i * i <= n {
            if is_prime[i]{
                primes.push(i);
                let mut j = i * i;
                while j <= n {
                    is_prime[i] = false;
                    j += i;
                }
                i += 2;
            }
        }
        while i <= n {
            if is_prime[i]{
                primes.push(i);
            }
            i += 2;
        }
        primes
    }
}