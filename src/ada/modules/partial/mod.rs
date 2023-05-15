use crate::ada::partialbank::PartialBank;

pub mod shift;

pub trait PartialFX {
    fn tick(&mut self, bank: &mut PartialBank);
    fn reset(&mut self, bank: &mut PartialBank);
}