#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("aks_primes/include/aks_theorem.h");
        pub fn aks_theorem(n : u32, r : u32, s : u32) -> bool;
    }
}
