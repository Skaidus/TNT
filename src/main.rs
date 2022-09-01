use tnt_lib::number_theory::primality::{aks::aks_2003, PrimalityTest};

fn main() {
    println!("{}",aks_2003::Aks2003::is_prime(1803));
}