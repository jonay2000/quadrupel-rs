use core::mem::size_of;
use fixed::{FixedI32, FixedI64, types};

pub type FI32 = FixedI32<types::extra::U16>;
pub type FI64 = FixedI64<types::extra::U24>;

pub fn rough_isqrt(x: u32) -> u32 {
    if x == 0 {
        return x;
    }

    let n = size_of::<u32>() as u32 * 8;
    let half = (n - x.leading_zeros()) / 2;
    ((x >> half) + (1 << half)) / 2
}

pub fn sqrt_approx(x: FI32) -> FI32 {
    if x == 0 {
        return x;
    }
    assert!(x > 0);

    let bits = x.to_bits();

    let n = 16isize; // Positive bits
    let scale = n - bits.leading_zeros() as isize;
    let half = scale / 2;
    let res = if half >= 0 {
        ((bits >> half) + (1 << (16 + half))) / 2
    } else {
        let half = half.abs();
        ((bits << half) + (1 << (16 - half))) / 2
    };
    FI32::from_bits(res)
}

pub fn atan2_approx(y: FI32, x: FI32) -> FI32 {
    if x == FI32::ZERO {
        return if y < FI32::ZERO {
            -FI32::FRAC_PI_2
        } else {
            FI32::FRAC_PI_2
        }
    }

    if y == FI32::ZERO {
        return if x >= FI32::ZERO {
            FI32::ZERO
        } else {
            FI32::PI
        }
    }

    match (x < FI32::ZERO, y < FI32::ZERO) {
        (false, false) => atan_approx(y / x),
        (false, true) => -atan_approx(-y / x),
        (true, false) => FI32::PI - atan_approx(y / -x),
        (true, true) => atan_approx(y / x) - FI32::PI,
    }
}

pub fn atan_approx(x: FI32) -> FI32 {
    if x.abs() > 1.0 {
        if x > 0 {
            FI32::FRAC_PI_2 - atan_approx_raw(FI32::from_num(1)/x)
        } else {
            (-FI32::FRAC_PI_2) - atan_approx_raw(FI32::from_num(1)/x)
        }
    } else {
        atan_approx_raw(x)
    }
}

//Only works in [-1, 1]
fn atan_approx_raw(x: FI32) -> FI32 {
    // Quadratic approximation recommended in
    // http://www-labs.iro.umontreal.ca/~mignotte/IFT2425/Documents/EfficientApproximationArctgFunction.pdf.
    let n2: FI32 = FI32::from_num(0.273);
    (FI32::FRAC_PI_4 + n2 - n2 * x.abs()) * x
}