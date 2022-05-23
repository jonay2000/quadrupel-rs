use fixed::{types, FixedI32};
use crate::library::fixed_point::{atan2_approx, FI32, sqrt_approx};

#[derive(Debug, Copy, Clone)]
pub struct Quaternion {
    pub w: FI32,
    pub x: FI32,
    pub y: FI32,
    pub z: FI32,
}

impl Quaternion {
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != 16 {
            return None;
        }
        let w =
            FixedI32::<types::extra::U30>::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let x =
            FixedI32::<types::extra::U30>::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        let y = FixedI32::<types::extra::U30>::from_be_bytes([
            bytes[8], bytes[9], bytes[10], bytes[11],
        ]);
        let z = FixedI32::<types::extra::U30>::from_be_bytes([
            bytes[12], bytes[13], bytes[14], bytes[15],
        ]);
        Some(Quaternion {
            w: FI32::from_num(w),
            x: FI32::from_num(x),
            y: FI32::from_num(y),
            z: FI32::from_num(z),
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub struct YawPitchRoll {
    /// psu
    pub yaw: FI32,
    /// theta
    pub pitch: FI32,
    /// phi
    pub roll: FI32,
}

impl YawPitchRoll {
    pub fn zero() -> Self {
        Self{ yaw: FI32::from_num(0), pitch: FI32::from_num(0), roll: FI32::from_num(0) }
    }
}

impl From<Quaternion> for YawPitchRoll {
    // Rust shows some errors in this function if using CLion, they're fake
    fn from(q: Quaternion) -> Self {
        let Quaternion { w, x, y, z } = q;

        let gx = 2 * (x * z - w * y);
        let gy = 2 * (w * x + y * z);
        let gz = w * w - x * x - y * y + z * z;

        // yaw: (about Z axis)
        let yaw = atan2_approx(
            2 * x * y - 2 * w * z,
            2 * w * w + 2 * x * x - FI32::from_num(1),
        );

        // pitch: (nose up/down, about Y axis)
        let pitch = atan2_approx(gx, sqrt_approx(gy * gy + gz * gz));

        // roll: (tilt left/right, about X axis)
        let roll = atan2_approx(gy, gz);


        Self { yaw, pitch, roll }
    }
}
