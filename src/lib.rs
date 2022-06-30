pub mod aks;
pub mod poly;

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

