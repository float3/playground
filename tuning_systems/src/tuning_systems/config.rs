use std::sync::Mutex;

pub static OCTAVE_SIZE: Mutex<usize> = Mutex::new(12);

pub static C4: f64 = 261.6256;
pub static C0: f64 = C4 / 16.0;
pub static CN1: f64 = C4 / 32.0;

pub static A4: f64 = 440.0;
pub static A0: f64 = A4 / 16.0;
pub static AN1: f64 = A4 / 32.0;

pub static TEST: u32 = 12;
pub static TEST_LAZY: Mutex<u32> = Mutex::new(12);
