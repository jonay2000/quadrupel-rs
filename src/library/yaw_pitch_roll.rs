use cordic::{atan2, sqrt};
use fixed::{types, FixedI32, FixedI64};

pub type FI32O = FixedI32<types::extra::U30>;
pub type FI32 = FixedI32<types::extra::U16>;
pub type FI64 = FixedI64<types::extra::U32>;

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
        let w = FI32O::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let x = FI32O::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        let y = FI32O::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
        let z = FI32O::from_be_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]);
        Some(Quaternion { w: FI32::from_num(w), x: FI32::from_num(x), y: FI32::from_num(y), z: FI32::from_num(z) })
    }
}

pub struct YawPitchRoll {
    pub yaw: FI32,
    pub pitch: FI32,
    pub roll: FI32,
}

impl From<Quaternion> for YawPitchRoll {
    fn from(q: Quaternion) -> Self {
        let Quaternion { w, x, y, z } = q;
        
        let gx = 2 * (x * z - w * y);
        let gy = 2 * (w * x + y * z);
        let gz = w * w - x * x - y * y + z * z;

        // yaw: (about Z axis)
        let yaw = atan2(
            2 * x * y - 2 * w * z,
            2 * w * w + 2 * x * x - FI32::from_num(1),
        );

        // pitch: (nose up/down, about Y axis)
        let pitch = atan2(
            gx,
            sqrt(gy * gy + gz * gz),
        );

        // roll: (tilt left/right, about X axis)
        let roll = atan2(gy, gz);

        Self { yaw, pitch, roll }
    }
}
