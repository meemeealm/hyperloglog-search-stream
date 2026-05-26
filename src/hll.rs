use murmur3::murmur3_32;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Serialize, Deserialize, Clone)]
pub struct HyperLogLog {
    b: usize,
    m: usize,
    registers: Vec<u8>,
    alpha: f64,
}

impl HyperLogLog {
    pub fn new(b: usize) -> Self {
        let m = 1 << b;
        let alpha = match m {
            16 => 0.673,
            32 => 0.697,
            64 => 0.709,
            _ => 0.7213 / (1.0 + 1.079 / (m as f64)),
        };
        HyperLogLog {
            b,
            m,
            registers: vec![0; m],
            alpha,
        }
    }

    pub fn insert(&mut self, item: &str) {
        let mut cursor = Cursor::new(item);
        let hash = murmur3_32(&mut cursor, 0).unwrap();
        let index = (hash >> (32 - self.b)) as usize;
        let remaining_bits = (hash << self.b) | (1 << (self.b - 1));
        let leading_zeros = remaining_bits.leading_zeros() as u8 + 1;

        if leading_zeros > self.registers[index] {
            self.registers[index] = leading_zeros;
        }
    }

    pub fn estimate(&self) -> f64 {
        let mut sum = 0.0;
        for &val in &self.registers {
            sum += 2.0_f64.powi(-(val as i32));
        }
        let mut raw_estimate = self.alpha * (self.m as f64).powi(2) / sum;

        if raw_estimate <= 2.5 * (self.m as f64) {
            let zero_registers = self.registers.iter().filter(|&&x| x == 0).count();
            if zero_registers > 0 {
                raw_estimate = (self.m as f64) * ((self.m as f64) / (zero_registers as f64)).ln();
            }
        }
        raw_estimate
    }
}
