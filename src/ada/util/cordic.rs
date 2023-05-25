use crate::ada::{types::SimdFloats, consts::SIMD_SIZE};

pub fn cordic_rotate(radians: &SimdFloats, cos: &mut SimdFloats, sin: &mut SimdFloats) {}

pub fn temp_rotate(radians: &SimdFloats, cos: &mut SimdFloats, sin: &mut SimdFloats) {
    for i in 0..SIMD_SIZE {
        let in_angle = sin[i].atan2(cos[i]);
        cos[i] = (in_angle + radians[i]).cos();
        sin[i] = (in_angle + radians[i]).sin();
    }
}
