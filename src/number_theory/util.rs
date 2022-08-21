use super::UnsigInt;

pub fn log2_floor<T: UnsigInt>(k: T) -> u32 
    {
        return if k> T::one() {1 + log2_floor(k/(T::two()))} else {0}
        
    }

pub fn is_even<T: UnsigInt>(k : T) -> bool
{
        k % T::two() == T::zero()
}
pub fn div_ceiling<T: UnsigInt>(f : T, k : T) -> T
{
    let q = f / k;
    let r = f % k;
    if r == T::zero() {q} else {q+T::one()}
}