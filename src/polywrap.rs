//#[link(name = "libakstheorem", kind = "static")]
// extern "C"{
//     pub fn aks_theorem(n : u32, r : u32, s : u32) -> bool;
// }

// #[cxx::bridge]
// mod ffi {
//     unsafe extern "C++" {
//         include!("aks_primes/aks_theorem/aks_theorem.h");
//         pub fn aks_theorem(n : u32, r : u32, s : u32) -> bool;
//     }
// }
