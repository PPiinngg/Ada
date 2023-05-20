// use lazy_static::lazy_static;

// //////////////////////////////////////

// const ACCURACY: usize = 8;

// //////////////////////////////////////

// const SIGN_MASK: u32 = 0x80000000u32;
// const F32_EXP: u32 = 0x7f800000u32;
// const F32_NOT_EXP: u32 = !F32_EXP;

// lazy_static! {
// 	static ref K: f32 = (0..ACCURACY)
// 		.map(|i| { 1f32 / (1f32 + (-2f32 * (i as f32)).exp2()).sqrt() })
// 		.product();
// 	static ref ANGLES: Vec<f32> = (0..ACCURACY)
// 		.map(|i| { ((-(i as f32)).exp2()).atan() })
// 		.collect();
// }

// pub fn cordic_rotate(radians: &f32, cos: &mut f32, sin: &mut f32) {
// 	let mut cos_out: f32 = *cos;
// 	let mut sin_out: f32 = *sin;
// 	let mut delta: f32 = *radians;


// 	for i in 0..ACCURACY {
// 		let sign_flip: u32 = delta.to_bits() & SIGN_MASK;
// 		delta = f32::from_bits(delta.to_bits() ^ sign_flip);

// 		let cos_uint: u32 = cos_out.to_bits();
// 		let sin_uint: u32 = sin_out.to_bits();
// 		cos_out += f32::from_bits(
// 			(cos_uint & F32_NOT_EXP) | ((cos_uint & F32_EXP) - ((i as u32) << 23)) ^ sign_flip,
// 		); // LINE 53 IN C++ IMPL
// 		sin_out -= f32::from_bits(
// 			(sin_uint & F32_NOT_EXP) | ((sin_uint & F32_EXP) - ((i as u32) << 23)) ^ sign_flip,
// 		); // LINE 52 IN C++ IMPL

// 		delta -= ANGLES[i];
// 	}
// }

pub fn temp_rotate(radians: &f32, cos: &mut f32, sin: &mut f32) {
	let in_angle = sin.atan2(*cos);
	*cos = (in_angle + radians).cos();
	*sin = (in_angle + radians).sin();
}
