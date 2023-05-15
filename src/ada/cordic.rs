// use lazy_static::lazy_static;
// use std::ops::Neg;

// const ACCURACY: usize = 11;
// const SIGN_MASK: u32 = 2_147_483_648;

// // currently a pretty much direct translation of matlab impl
// // https://en.wikipedia.org/wiki/CORDIC#Implementation
// // technically works but needs improving, i don't completely understand this all that well yet
// lazy_static! {
// 	static ref K: f32 = (0..ACCURACY)
// 		.map(|i| { 1f32 / (1f32 + (-2_f32 * (i as f32)).exp2()).sqrt() })
// 		.product();
// 	static ref ANGLES: Vec<f32> = (0..ACCURACY)
// 		.map(|i| { ((-(i as f32)).exp2()).atan() })
// 		.collect();
// }
// pub fn cordic_rotate(radians: &f32, cos: &mut f32, sin: &mut f32) {
// 	let mut cos_out = *cos;
// 	let mut sin_out = *sin;
// 	let mut delta = *radians;

// 	for i in 0..ACCURACY {
// 		// matrix multiplication
// 		let delta_sign: u32 = delta.to_bits() & SIGN_MASK;
// 		let cos_temp =
// 			cos_out - (sin_out * f32::from_bits(((i as f32).neg().exp2()).to_bits() ^ delta_sign));
// 		let sin_temp =
// 			sin_out + (cos_out * f32::from_bits(((i as f32).neg().exp2()).to_bits() ^ delta_sign));
// 		cos_out = cos_temp;
// 		sin_out = sin_temp;

//         delta -= f32::from_bits(ANGLES[i].to_bits() ^ delta_sign);
// 	}

// 	*cos = cos_out * *K;
// 	*sin = sin_out * *K;
// }

// pub fn cordic_sin(radians: &f32) -> f32 {
//     let mut cos = 1f32;
//     let mut sin = 0f32;
//     cordic_rotate(radians, &mut cos, &mut sin);
//     sin
// }

pub fn temp_rotate(radians: &f32, cos: &mut f32, sin: &mut f32) {
    let in_angle = sin.atan2(*cos);
    *cos = (in_angle + radians).cos();
    *sin = (in_angle + radians).sin();
}
