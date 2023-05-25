pub mod simd {
	use crate::ada::{consts::SIMD_SIZE, types::SimdFloats};
	pub fn cordic_rotate(radians: &SimdFloats, cos: &mut SimdFloats, sin: &mut SimdFloats) {}

	pub fn temp_rotate(radians: &SimdFloats, cos: &mut SimdFloats, sin: &mut SimdFloats) {
		for i in 0..SIMD_SIZE {
			let in_angle = sin[i].atan2(cos[i]);
			cos[i] = (in_angle + radians[i]).cos();
			sin[i] = (in_angle + radians[i]).sin();
		}
	}
}

pub mod scalar {
    pub fn cordic_rotate(radians: &f32, cos: &mut f32, sin: &mut f32) {}
    
    pub fn temp_rotate(radians: &f32, cos: &mut f32, sin: &mut f32) {
			let in_angle = sin.atan2(*cos);
			*cos = (in_angle + radians).cos();
			*sin = (in_angle + radians).sin();
	}
}
