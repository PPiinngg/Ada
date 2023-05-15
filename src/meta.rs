pub static mut SAMPLERATE_VAL: f32 = 0f32;
#[inline]
pub fn samplerate() -> f32 {unsafe { SAMPLERATE_VAL }}
