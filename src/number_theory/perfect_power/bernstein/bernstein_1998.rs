// use crate::number_theory::primality::sieve::Sieve;

// pub struct Integer {
//     n: u64,
//     logn: u32,
// }

// impl Integer {
//     pub fn new(n: u64) -> Integer {
//         if n == 0 {
//             panic!("Tested number can't be lower than 1 ")
//         }
//         let logn = Integer::logk(n);
//         Integer { n, logn }
//     }

//     fn logk(k: u64) -> u32 {
//         64 - k.leading_zeros() - 1
//     }

//     fn mul_2(b: u32, m: u32, k: u32) -> u32 {
//         (k * m) % (2 ^ b)
//     }

//     fn pow_2(b: u32, m: u32, k: u32) -> u32 {
//         (m ^ k) % (2 ^ b)
//     }

//     fn div_2(b: u32, m: u32, k: u32) -> u32 {
//         let exp = 2 ^ b;
//         for d in 0..exp - 1 {
//             if m == Integer::pow_2(b, d, k) {
//                 return d;
//             }
//         }
//         return exp - 1;
//     }

//     fn nroot(y: u32, b: u32, k: u32) -> u32 {
//         if b == 1 {
//             return 1;
//         }
//         let z = Integer::nroot(y, b / 2, k);
//         let r2 = Integer::mul_2(b, z, k + 1);
//         let r3 = y * Integer::pow_2(b, z, k + 1);
//         Integer::div_2(b, r2 - r3, k)
//     }

//     fn x_pow_k_eq_n(n: u32, x: u32, k: u32) -> bool {
//         let f = Integer::logk((2 * n).into());
//         if x == 1 {
//             return n == 1;
//         }
//         let mut b = 1;
//         let exp = 2 ^ b;
//         let mut x_pow_k_eq_r = false;
//         while b < f {
//             let r = Integer::pow_2(b, x, k);
//             x_pow_k_eq_r = Integer::x_pow_k_eq_n(n, r, k);
//             if n % exp != r {
//                 return false;
//             }
//             b = (2 * b).min(f);
//         }
//         x_pow_k_eq_r
//     }

//     fn is_kth_power(n: u32, k: u32, y: u32) -> bool {
//         let f = Integer::logk((2 * n).into());
//         let b = f / k;
//         let r = Integer::nroot(y, b, k);
//         if k == 2 && r == 0 {
//             return false;
//         }
//         if Integer::x_pow_k_eq_n(n, r, k) {
//             return true;
//         }
//         if k == 2 {
//             return Integer::x_pow_k_eq_n(n, (2 ^ b) - r, k);
//         }
//         false
//     }

//     pub fn is_perfect_power(&self) -> bool {
//         let f = Integer::logk(2 * self.n);
//         let y = Integer::nroot(f / 2 + 1, self.n as u32, 1);
//         for p in Sieve::new(f.into()) {
//             if Integer::is_kth_power(self.n as u32, p as u32, y) {
//                 return true;
//             }
//         }
//         false
//     }
// }