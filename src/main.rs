use rug::Integer;
use aks_primes::aks::optimized::custom::Integer as MyInteger;

// struct AKS {
//     n: Integer,
//     log2n: Integer,
// }

// impl AKS {
//     fn new(n: usize) -> AKS {
//         let n = Integer::from(n);
//         AKS {
//             n,
//             log2n: Integer::from(n.significant_bits() - 1),
//         }
//     }

//     fn run(&self) -> bool {
//         if self.n.is_perfect_power() {
//             return false;
//         }
//         true
//     }

//     fn get_r(&self) {
//         let max_k = (&self.log2n).pow(2).complete();
//         let max_r = (&self.log2n).pow(5).complete().max(Integer::from(3 as u8));

//         let mut next_r = true;
//         let r = Integer::from(2 as u8);
//         while r <= max_r && next_r {
//             next_r = false;
//             let mut k = Integer::from(1 as u8);
//             while k <= max_k && !next_r {
//                 next_r = ((&self.n).pow_mod(&k, &r).unwrap() == Integer::from(2 as u8))
//             }
//         }
//     }
// }

// fn get_r(n : u64){
//     let max_k = n.log2().pow(2);
//     let max_r = 3.max(n.log2().pow(5));
//     let next_r = true;
//     for r in 2..=max_k{
//         if !next_r {break};
//         for k in 1..=max_k{
//             if next_r {break};
//             next_r = 1;
//         }
//     }

fn main() {
    let n : u64 = 62764785704439251



    ;
    use std::time::Instant;
    use std::env;
    env::set_var("RUST_BACKTRACE", "1");

    let now = Instant::now();
    let test = MyInteger::new(n);
    let result = test.is_perfect_power();
    let elapsed = now.elapsed();
    println!("{} perfect power: {}", n, result);
    println!("Elapsed: {:.2?}", elapsed);

    let now = Instant::now();
    let test = Integer::from(n);
    let result = test.is_perfect_power();
    let elapsed = now.elapsed();
    println!("{} perfect power: {}", n, result);
    println!("Elapsed: {:.2?}", elapsed);
}

// nat log: https://stackoverflow.com/questions/70688941/how-to-calculate-integer-logarithm-of-base-3-using-fast-bit-operations

// }

// fn mul_mod()
