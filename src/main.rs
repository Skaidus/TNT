#![feature(int_log)]

use std::f64;

fn main() {
    println!("Hello, world!");
}

fn is_perfect_power(n : u64) -> bool{
    let epsilon  = f64::EPSILON;
    let n_float = n as f64;
    
    for b in 2..=n.log2() {
        let a = n_float.powf(1.0/(b as f64));
        if(a - a.floor()) < epsilon  {return true};
    }
    false
}

fn get_r(n : u64){
    let max_k = n.log2().pow(2);
    let max_r = 3.max(n.log2().pow(5));
    let next_r = true;
    for r in 2..=max_k{
        if !next_r {break};
        for k in 1..=max_k{
            if next_r {break};
            next_r = 
        }
    }


}

fn mul_mod()