use rug::{Integer, ops::Pow};


struct AKS {
    n : Integer,
    log2n : Integer
}

impl AKS {
    fn new(n : usize) -> AKS{
        let n = Integer::from(n);
        AKS{
            n,
            log2n: Integer::from(n.significant_bits() - 1)
        }
    }

    fn run(&self)-> bool {
        if self.n.is_perfect_power() {return false}    
        true
    }

    fn get_r(&self){
        let max_k = self.log2n.clone().pow(2);
        let max_r = self.log2n.clone().pow(5).max(Integer::from(3));
        let mut next_r = true;
        
        
    }
}

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
    println!("Hello, world!");
}


// nat log: https://stackoverflow.com/questions/70688941/how-to-calculate-integer-logarithm-of-base-3-using-fast-bit-operations






// }

// fn mul_mod()