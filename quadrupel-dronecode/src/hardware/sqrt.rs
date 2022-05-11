use core::mem::size_of;

pub fn rough_isqrt(x: u32) -> u32 {
    if x == 0 { return x }

    let n = size_of::<u32>() as u32 * 8;
    let scale = n - x.leading_zeros();
    let half = scale/2;
    let guess = ( (x >> half) + (1 << half) )/2;
    guess
}