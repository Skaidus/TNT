#[link(name = "libakstheorem", kind = "static")]
extern "C"{
    pub fn aks_theorem(n : u32, r : u32, s : u32) -> bool;
}