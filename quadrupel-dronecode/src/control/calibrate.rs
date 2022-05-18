// use crate::library::yaw_pitch_roll::FI32;
//
// pub struct Calibrate {
//     yaw: FI32,
//     pitch: FI32,
//     roll: FI32,
// }
//
// impl Calibrate {
//     pub fn new() -> Self {
//         Calibrate {
//             yaw: FI32::from_num(0),
//             pitch: FI32::from_num(0),
//             roll: FI32::from_num(0),
//         }
//     }
//
//     pub fn calibrate(&mut self, yaw: FI32, pitch: FI32, roll: FI32) {
//         self.yaw = self.yaw * FI32::from_num(0.99) + yaw * FI32::from_num(0.01);
//         self.pitch = self.pitch * FI32::from_num(0.99) + pitch * FI32::from_num(0.01);
//         self.roll = self.roll * FI32::from_num(0.99) + roll * FI32::from_num(0.01);
//     }
//
//     pub fn fix_yaw(&mut self, yaw: FI32) {
//         yaw - self.yaw;
//     }
//
//     pub fn fix_pitch(&mut self, pitch: FI32) {
//         pitch - self.pitch;
//     }
//
//     pub fn fix_roll(&mut self, roll: FI32) {
//         roll - self.roll;
//     }
// }
