//! Auto-generated bulk module (file 7) - exists purely to make `game_rs`
//! heavier for a hot-patch speed experiment. Real arithmetic, real generics
//! (so monomorphization isn't free), and real use of a rotating set of
//! external crates so none of them are dead code. `touch_bulk_7()` is
//! called once from `game::init` so nothing here gets stripped as unreachable.

use rand::{Rng, SeedableRng};
use sha2::{Digest, Sha256};
use noise::{NoiseFn, Perlin};
use regex::Regex;
use itertools::Itertools;
use uuid::Uuid;
use chrono::Utc;


pub mod m700 {
    use super::*;

    pub struct Accumulator700<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator700<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.193_f32 + y.sin();
        let b = y * 1.431_f32 - x.cos();
        let mut acc = Accumulator700::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.889_f32 + y.sin();
        let b = y * 6.162_f32 - x.cos();
        let mut acc = Accumulator700::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.756_f32 + y.sin();
        let b = y * 6.687_f32 - x.cos();
        let mut acc = Accumulator700::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.42_f32 + y.sin();
        let b = y * 5.01_f32 - x.cos();
        let mut acc = Accumulator700::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.8_f32 + y.sin();
        let b = y * 3.747_f32 - x.cos();
        let mut acc = Accumulator700::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.517_f32 + y.sin();
        let b = y * 0.253_f32 - x.cos();
        let mut acc = Accumulator700::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.725_f32 + y.sin();
        let b = y * 9.11_f32 - x.cos();
        let mut acc = Accumulator700::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.212_f32 + y.sin();
        let b = y * 4.445_f32 - x.cos();
        let mut acc = Accumulator700::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.033_f32 + y.sin();
        let b = y * 2.371_f32 - x.cos();
        let mut acc = Accumulator700::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.345_f32 + y.sin();
        let b = y * 6.187_f32 - x.cos();
        let mut acc = Accumulator700::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.442_f32 + y.sin();
        let b = y * 9.743_f32 - x.cos();
        let mut acc = Accumulator700::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.521_f32 + y.sin();
        let b = y * 2.202_f32 - x.cos();
        let mut acc = Accumulator700::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.474_f32 + y.sin();
        let b = y * 2.589_f32 - x.cos();
        let mut acc = Accumulator700::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.091_f32 + y.sin();
        let b = y * 2.59_f32 - x.cos();
        let mut acc = Accumulator700::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.313_f32 + y.sin();
        let b = y * 2.746_f32 - x.cos();
        let mut acc = Accumulator700::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.668_f32 + y.sin();
        let b = y * 6.85_f32 - x.cos();
        let mut acc = Accumulator700::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.345_f32 + y.sin();
        let b = y * 4.724_f32 - x.cos();
        let mut acc = Accumulator700::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.534_f32 + y.sin();
        let b = y * 7.22_f32 - x.cos();
        let mut acc = Accumulator700::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.629_f32 + y.sin();
        let b = y * 2.622_f32 - x.cos();
        let mut acc = Accumulator700::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.33_f32 + y.sin();
        let b = y * 9.075_f32 - x.cos();
        let mut acc = Accumulator700::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_700(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_700() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_700(total as u64) % 997) as f32;
        total
    }
}

pub mod m701 {
    use super::*;

    pub struct Accumulator701<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator701<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.546_f32 + y.sin();
        let b = y * 5.458_f32 - x.cos();
        let mut acc = Accumulator701::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.152_f32 + y.sin();
        let b = y * 4.343_f32 - x.cos();
        let mut acc = Accumulator701::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.24_f32 + y.sin();
        let b = y * 8.285_f32 - x.cos();
        let mut acc = Accumulator701::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.939_f32 + y.sin();
        let b = y * 2.42_f32 - x.cos();
        let mut acc = Accumulator701::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.781_f32 + y.sin();
        let b = y * 1.402_f32 - x.cos();
        let mut acc = Accumulator701::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.649_f32 + y.sin();
        let b = y * 5.542_f32 - x.cos();
        let mut acc = Accumulator701::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.745_f32 + y.sin();
        let b = y * 6.778_f32 - x.cos();
        let mut acc = Accumulator701::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.916_f32 + y.sin();
        let b = y * 7.588_f32 - x.cos();
        let mut acc = Accumulator701::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.116_f32 + y.sin();
        let b = y * 7.182_f32 - x.cos();
        let mut acc = Accumulator701::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.22_f32 + y.sin();
        let b = y * 2.077_f32 - x.cos();
        let mut acc = Accumulator701::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.603_f32 + y.sin();
        let b = y * 1.09_f32 - x.cos();
        let mut acc = Accumulator701::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.427_f32 + y.sin();
        let b = y * 0.502_f32 - x.cos();
        let mut acc = Accumulator701::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.348_f32 + y.sin();
        let b = y * 8.42_f32 - x.cos();
        let mut acc = Accumulator701::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.216_f32 + y.sin();
        let b = y * 2.885_f32 - x.cos();
        let mut acc = Accumulator701::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.811_f32 + y.sin();
        let b = y * 9.021_f32 - x.cos();
        let mut acc = Accumulator701::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.051_f32 + y.sin();
        let b = y * 0.855_f32 - x.cos();
        let mut acc = Accumulator701::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.647_f32 + y.sin();
        let b = y * 9.008_f32 - x.cos();
        let mut acc = Accumulator701::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.382_f32 + y.sin();
        let b = y * 8.817_f32 - x.cos();
        let mut acc = Accumulator701::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.61_f32 + y.sin();
        let b = y * 1.13_f32 - x.cos();
        let mut acc = Accumulator701::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.322_f32 + y.sin();
        let b = y * 3.704_f32 - x.cos();
        let mut acc = Accumulator701::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_701(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m701-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_701() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_701(total as u64) % 997) as f32;
        total
    }
}

pub mod m702 {
    use super::*;

    pub struct Accumulator702<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator702<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.391_f32 + y.sin();
        let b = y * 3.201_f32 - x.cos();
        let mut acc = Accumulator702::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.611_f32 + y.sin();
        let b = y * 6.859_f32 - x.cos();
        let mut acc = Accumulator702::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.873_f32 + y.sin();
        let b = y * 3.852_f32 - x.cos();
        let mut acc = Accumulator702::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.788_f32 + y.sin();
        let b = y * 2.3_f32 - x.cos();
        let mut acc = Accumulator702::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.324_f32 + y.sin();
        let b = y * 9.383_f32 - x.cos();
        let mut acc = Accumulator702::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.289_f32 + y.sin();
        let b = y * 6.553_f32 - x.cos();
        let mut acc = Accumulator702::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.887_f32 + y.sin();
        let b = y * 4.118_f32 - x.cos();
        let mut acc = Accumulator702::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.952_f32 + y.sin();
        let b = y * 1.779_f32 - x.cos();
        let mut acc = Accumulator702::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.296_f32 + y.sin();
        let b = y * 5.846_f32 - x.cos();
        let mut acc = Accumulator702::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.036_f32 + y.sin();
        let b = y * 2.012_f32 - x.cos();
        let mut acc = Accumulator702::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.214_f32 + y.sin();
        let b = y * 4.666_f32 - x.cos();
        let mut acc = Accumulator702::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.602_f32 + y.sin();
        let b = y * 8.153_f32 - x.cos();
        let mut acc = Accumulator702::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.159_f32 + y.sin();
        let b = y * 6.737_f32 - x.cos();
        let mut acc = Accumulator702::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.472_f32 + y.sin();
        let b = y * 8.705_f32 - x.cos();
        let mut acc = Accumulator702::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.728_f32 + y.sin();
        let b = y * 9.026_f32 - x.cos();
        let mut acc = Accumulator702::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.096_f32 + y.sin();
        let b = y * 7.064_f32 - x.cos();
        let mut acc = Accumulator702::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.161_f32 + y.sin();
        let b = y * 2.337_f32 - x.cos();
        let mut acc = Accumulator702::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.239_f32 + y.sin();
        let b = y * 1.854_f32 - x.cos();
        let mut acc = Accumulator702::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.915_f32 + y.sin();
        let b = y * 0.974_f32 - x.cos();
        let mut acc = Accumulator702::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.005_f32 + y.sin();
        let b = y * 4.68_f32 - x.cos();
        let mut acc = Accumulator702::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_702(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_702() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_702(total as u64) % 997) as f32;
        total
    }
}

pub mod m703 {
    use super::*;

    pub struct Accumulator703<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator703<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.435_f32 + y.sin();
        let b = y * 3.315_f32 - x.cos();
        let mut acc = Accumulator703::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.736_f32 + y.sin();
        let b = y * 4.657_f32 - x.cos();
        let mut acc = Accumulator703::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.013_f32 + y.sin();
        let b = y * 1.42_f32 - x.cos();
        let mut acc = Accumulator703::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.663_f32 + y.sin();
        let b = y * 5.891_f32 - x.cos();
        let mut acc = Accumulator703::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.276_f32 + y.sin();
        let b = y * 5.899_f32 - x.cos();
        let mut acc = Accumulator703::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.67_f32 + y.sin();
        let b = y * 9.604_f32 - x.cos();
        let mut acc = Accumulator703::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.317_f32 + y.sin();
        let b = y * 2.732_f32 - x.cos();
        let mut acc = Accumulator703::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.953_f32 + y.sin();
        let b = y * 2.624_f32 - x.cos();
        let mut acc = Accumulator703::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.441_f32 + y.sin();
        let b = y * 8.594_f32 - x.cos();
        let mut acc = Accumulator703::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.53_f32 + y.sin();
        let b = y * 8.826_f32 - x.cos();
        let mut acc = Accumulator703::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.508_f32 + y.sin();
        let b = y * 7.228_f32 - x.cos();
        let mut acc = Accumulator703::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.501_f32 + y.sin();
        let b = y * 1.128_f32 - x.cos();
        let mut acc = Accumulator703::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.604_f32 + y.sin();
        let b = y * 2.42_f32 - x.cos();
        let mut acc = Accumulator703::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.632_f32 + y.sin();
        let b = y * 2.544_f32 - x.cos();
        let mut acc = Accumulator703::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.362_f32 + y.sin();
        let b = y * 1.634_f32 - x.cos();
        let mut acc = Accumulator703::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.812_f32 + y.sin();
        let b = y * 1.719_f32 - x.cos();
        let mut acc = Accumulator703::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.348_f32 + y.sin();
        let b = y * 7.457_f32 - x.cos();
        let mut acc = Accumulator703::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.374_f32 + y.sin();
        let b = y * 8.856_f32 - x.cos();
        let mut acc = Accumulator703::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.057_f32 + y.sin();
        let b = y * 9.354_f32 - x.cos();
        let mut acc = Accumulator703::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.475_f32 + y.sin();
        let b = y * 0.453_f32 - x.cos();
        let mut acc = Accumulator703::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_703(seed: u64) -> u64 {
        let re = Regex::new(r"m703-(\d+)").unwrap();
        let hay = format!("m703-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_703() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_703(total as u64) % 997) as f32;
        total
    }
}

pub mod m704 {
    use super::*;

    pub struct Accumulator704<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator704<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.767_f32 + y.sin();
        let b = y * 8.563_f32 - x.cos();
        let mut acc = Accumulator704::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.996_f32 + y.sin();
        let b = y * 9.595_f32 - x.cos();
        let mut acc = Accumulator704::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.764_f32 + y.sin();
        let b = y * 3.823_f32 - x.cos();
        let mut acc = Accumulator704::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.486_f32 + y.sin();
        let b = y * 3.696_f32 - x.cos();
        let mut acc = Accumulator704::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.029_f32 + y.sin();
        let b = y * 4.369_f32 - x.cos();
        let mut acc = Accumulator704::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.436_f32 + y.sin();
        let b = y * 2.871_f32 - x.cos();
        let mut acc = Accumulator704::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.827_f32 + y.sin();
        let b = y * 7.407_f32 - x.cos();
        let mut acc = Accumulator704::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.61_f32 + y.sin();
        let b = y * 9.08_f32 - x.cos();
        let mut acc = Accumulator704::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.959_f32 + y.sin();
        let b = y * 5.265_f32 - x.cos();
        let mut acc = Accumulator704::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.46_f32 + y.sin();
        let b = y * 2.348_f32 - x.cos();
        let mut acc = Accumulator704::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.798_f32 + y.sin();
        let b = y * 8.681_f32 - x.cos();
        let mut acc = Accumulator704::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.27_f32 + y.sin();
        let b = y * 0.346_f32 - x.cos();
        let mut acc = Accumulator704::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.893_f32 + y.sin();
        let b = y * 4.977_f32 - x.cos();
        let mut acc = Accumulator704::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.775_f32 + y.sin();
        let b = y * 3.884_f32 - x.cos();
        let mut acc = Accumulator704::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.248_f32 + y.sin();
        let b = y * 7.532_f32 - x.cos();
        let mut acc = Accumulator704::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.199_f32 + y.sin();
        let b = y * 6.734_f32 - x.cos();
        let mut acc = Accumulator704::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.411_f32 + y.sin();
        let b = y * 6.374_f32 - x.cos();
        let mut acc = Accumulator704::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.564_f32 + y.sin();
        let b = y * 1.858_f32 - x.cos();
        let mut acc = Accumulator704::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.283_f32 + y.sin();
        let b = y * 8.446_f32 - x.cos();
        let mut acc = Accumulator704::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.952_f32 + y.sin();
        let b = y * 9.558_f32 - x.cos();
        let mut acc = Accumulator704::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_704(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_704() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_704(total as u64) % 997) as f32;
        total
    }
}

pub mod m705 {
    use super::*;

    pub struct Accumulator705<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator705<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.338_f32 + y.sin();
        let b = y * 4.695_f32 - x.cos();
        let mut acc = Accumulator705::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.716_f32 + y.sin();
        let b = y * 9.47_f32 - x.cos();
        let mut acc = Accumulator705::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.276_f32 + y.sin();
        let b = y * 3.97_f32 - x.cos();
        let mut acc = Accumulator705::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.097_f32 + y.sin();
        let b = y * 4.241_f32 - x.cos();
        let mut acc = Accumulator705::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.137_f32 + y.sin();
        let b = y * 9.104_f32 - x.cos();
        let mut acc = Accumulator705::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.252_f32 + y.sin();
        let b = y * 9.679_f32 - x.cos();
        let mut acc = Accumulator705::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.231_f32 + y.sin();
        let b = y * 3.205_f32 - x.cos();
        let mut acc = Accumulator705::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.232_f32 + y.sin();
        let b = y * 9.43_f32 - x.cos();
        let mut acc = Accumulator705::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.975_f32 + y.sin();
        let b = y * 6.609_f32 - x.cos();
        let mut acc = Accumulator705::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.581_f32 + y.sin();
        let b = y * 2.774_f32 - x.cos();
        let mut acc = Accumulator705::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.261_f32 + y.sin();
        let b = y * 5.795_f32 - x.cos();
        let mut acc = Accumulator705::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.117_f32 + y.sin();
        let b = y * 2.203_f32 - x.cos();
        let mut acc = Accumulator705::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.117_f32 + y.sin();
        let b = y * 6.544_f32 - x.cos();
        let mut acc = Accumulator705::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.635_f32 + y.sin();
        let b = y * 3.567_f32 - x.cos();
        let mut acc = Accumulator705::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.36_f32 + y.sin();
        let b = y * 1.579_f32 - x.cos();
        let mut acc = Accumulator705::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.972_f32 + y.sin();
        let b = y * 1.419_f32 - x.cos();
        let mut acc = Accumulator705::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.039_f32 + y.sin();
        let b = y * 4.198_f32 - x.cos();
        let mut acc = Accumulator705::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.741_f32 + y.sin();
        let b = y * 6.592_f32 - x.cos();
        let mut acc = Accumulator705::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.403_f32 + y.sin();
        let b = y * 4.207_f32 - x.cos();
        let mut acc = Accumulator705::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.373_f32 + y.sin();
        let b = y * 1.422_f32 - x.cos();
        let mut acc = Accumulator705::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_705(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(705u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_705() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_705(total as u64) % 997) as f32;
        total
    }
}

pub mod m706 {
    use super::*;

    pub struct Accumulator706<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator706<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.09_f32 + y.sin();
        let b = y * 1.534_f32 - x.cos();
        let mut acc = Accumulator706::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.085_f32 + y.sin();
        let b = y * 1.14_f32 - x.cos();
        let mut acc = Accumulator706::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.231_f32 + y.sin();
        let b = y * 9.665_f32 - x.cos();
        let mut acc = Accumulator706::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.661_f32 + y.sin();
        let b = y * 4.11_f32 - x.cos();
        let mut acc = Accumulator706::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.521_f32 + y.sin();
        let b = y * 2.268_f32 - x.cos();
        let mut acc = Accumulator706::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.285_f32 + y.sin();
        let b = y * 5.049_f32 - x.cos();
        let mut acc = Accumulator706::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.047_f32 + y.sin();
        let b = y * 8.493_f32 - x.cos();
        let mut acc = Accumulator706::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.878_f32 + y.sin();
        let b = y * 0.883_f32 - x.cos();
        let mut acc = Accumulator706::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.994_f32 + y.sin();
        let b = y * 2.449_f32 - x.cos();
        let mut acc = Accumulator706::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.012_f32 + y.sin();
        let b = y * 9.614_f32 - x.cos();
        let mut acc = Accumulator706::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.489_f32 + y.sin();
        let b = y * 8.458_f32 - x.cos();
        let mut acc = Accumulator706::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.97_f32 + y.sin();
        let b = y * 2.38_f32 - x.cos();
        let mut acc = Accumulator706::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.348_f32 + y.sin();
        let b = y * 9.805_f32 - x.cos();
        let mut acc = Accumulator706::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.634_f32 + y.sin();
        let b = y * 2.043_f32 - x.cos();
        let mut acc = Accumulator706::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.717_f32 + y.sin();
        let b = y * 1.638_f32 - x.cos();
        let mut acc = Accumulator706::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.292_f32 + y.sin();
        let b = y * 1.836_f32 - x.cos();
        let mut acc = Accumulator706::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.984_f32 + y.sin();
        let b = y * 3.089_f32 - x.cos();
        let mut acc = Accumulator706::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.909_f32 + y.sin();
        let b = y * 2.236_f32 - x.cos();
        let mut acc = Accumulator706::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.94_f32 + y.sin();
        let b = y * 7.331_f32 - x.cos();
        let mut acc = Accumulator706::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.766_f32 + y.sin();
        let b = y * 4.873_f32 - x.cos();
        let mut acc = Accumulator706::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_706(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_706() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_706(total as u64) % 997) as f32;
        total
    }
}

pub mod m707 {
    use super::*;

    pub struct Accumulator707<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator707<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.7_f32 + y.sin();
        let b = y * 2.161_f32 - x.cos();
        let mut acc = Accumulator707::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.08_f32 + y.sin();
        let b = y * 4.997_f32 - x.cos();
        let mut acc = Accumulator707::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.013_f32 + y.sin();
        let b = y * 0.811_f32 - x.cos();
        let mut acc = Accumulator707::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.973_f32 + y.sin();
        let b = y * 1.29_f32 - x.cos();
        let mut acc = Accumulator707::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.615_f32 + y.sin();
        let b = y * 7.999_f32 - x.cos();
        let mut acc = Accumulator707::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.113_f32 + y.sin();
        let b = y * 1.192_f32 - x.cos();
        let mut acc = Accumulator707::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.488_f32 + y.sin();
        let b = y * 9.071_f32 - x.cos();
        let mut acc = Accumulator707::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.166_f32 + y.sin();
        let b = y * 2.518_f32 - x.cos();
        let mut acc = Accumulator707::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.929_f32 + y.sin();
        let b = y * 6.677_f32 - x.cos();
        let mut acc = Accumulator707::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.538_f32 + y.sin();
        let b = y * 1.096_f32 - x.cos();
        let mut acc = Accumulator707::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.481_f32 + y.sin();
        let b = y * 4.775_f32 - x.cos();
        let mut acc = Accumulator707::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.142_f32 + y.sin();
        let b = y * 3.017_f32 - x.cos();
        let mut acc = Accumulator707::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.249_f32 + y.sin();
        let b = y * 4.64_f32 - x.cos();
        let mut acc = Accumulator707::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.019_f32 + y.sin();
        let b = y * 8.334_f32 - x.cos();
        let mut acc = Accumulator707::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.406_f32 + y.sin();
        let b = y * 4.508_f32 - x.cos();
        let mut acc = Accumulator707::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.759_f32 + y.sin();
        let b = y * 4.738_f32 - x.cos();
        let mut acc = Accumulator707::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.978_f32 + y.sin();
        let b = y * 4.156_f32 - x.cos();
        let mut acc = Accumulator707::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.795_f32 + y.sin();
        let b = y * 4.791_f32 - x.cos();
        let mut acc = Accumulator707::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.885_f32 + y.sin();
        let b = y * 6.85_f32 - x.cos();
        let mut acc = Accumulator707::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.341_f32 + y.sin();
        let b = y * 3.133_f32 - x.cos();
        let mut acc = Accumulator707::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_707(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_707() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_707(total as u64) % 997) as f32;
        total
    }
}

pub mod m708 {
    use super::*;

    pub struct Accumulator708<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator708<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.861_f32 + y.sin();
        let b = y * 0.686_f32 - x.cos();
        let mut acc = Accumulator708::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.472_f32 + y.sin();
        let b = y * 0.586_f32 - x.cos();
        let mut acc = Accumulator708::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.302_f32 + y.sin();
        let b = y * 7.33_f32 - x.cos();
        let mut acc = Accumulator708::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.119_f32 + y.sin();
        let b = y * 8.836_f32 - x.cos();
        let mut acc = Accumulator708::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.505_f32 + y.sin();
        let b = y * 6.014_f32 - x.cos();
        let mut acc = Accumulator708::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.389_f32 + y.sin();
        let b = y * 9.513_f32 - x.cos();
        let mut acc = Accumulator708::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.826_f32 + y.sin();
        let b = y * 6.246_f32 - x.cos();
        let mut acc = Accumulator708::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.504_f32 + y.sin();
        let b = y * 6.396_f32 - x.cos();
        let mut acc = Accumulator708::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.164_f32 + y.sin();
        let b = y * 6.252_f32 - x.cos();
        let mut acc = Accumulator708::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.868_f32 + y.sin();
        let b = y * 5.275_f32 - x.cos();
        let mut acc = Accumulator708::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.971_f32 + y.sin();
        let b = y * 3.258_f32 - x.cos();
        let mut acc = Accumulator708::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.909_f32 + y.sin();
        let b = y * 7.168_f32 - x.cos();
        let mut acc = Accumulator708::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.225_f32 + y.sin();
        let b = y * 4.633_f32 - x.cos();
        let mut acc = Accumulator708::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.515_f32 + y.sin();
        let b = y * 6.739_f32 - x.cos();
        let mut acc = Accumulator708::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.314_f32 + y.sin();
        let b = y * 6.988_f32 - x.cos();
        let mut acc = Accumulator708::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.654_f32 + y.sin();
        let b = y * 3.966_f32 - x.cos();
        let mut acc = Accumulator708::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.942_f32 + y.sin();
        let b = y * 2.102_f32 - x.cos();
        let mut acc = Accumulator708::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.08_f32 + y.sin();
        let b = y * 0.71_f32 - x.cos();
        let mut acc = Accumulator708::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.862_f32 + y.sin();
        let b = y * 3.27_f32 - x.cos();
        let mut acc = Accumulator708::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.335_f32 + y.sin();
        let b = y * 5.573_f32 - x.cos();
        let mut acc = Accumulator708::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_708(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m708-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_708() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_708(total as u64) % 997) as f32;
        total
    }
}

pub mod m709 {
    use super::*;

    pub struct Accumulator709<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator709<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.048_f32 + y.sin();
        let b = y * 4.599_f32 - x.cos();
        let mut acc = Accumulator709::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.153_f32 + y.sin();
        let b = y * 4.879_f32 - x.cos();
        let mut acc = Accumulator709::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.383_f32 + y.sin();
        let b = y * 0.553_f32 - x.cos();
        let mut acc = Accumulator709::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.867_f32 + y.sin();
        let b = y * 4.924_f32 - x.cos();
        let mut acc = Accumulator709::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.57_f32 + y.sin();
        let b = y * 5.095_f32 - x.cos();
        let mut acc = Accumulator709::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.776_f32 + y.sin();
        let b = y * 4.071_f32 - x.cos();
        let mut acc = Accumulator709::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.09_f32 + y.sin();
        let b = y * 1.33_f32 - x.cos();
        let mut acc = Accumulator709::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.701_f32 + y.sin();
        let b = y * 8.449_f32 - x.cos();
        let mut acc = Accumulator709::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.507_f32 + y.sin();
        let b = y * 1.23_f32 - x.cos();
        let mut acc = Accumulator709::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.816_f32 + y.sin();
        let b = y * 5.926_f32 - x.cos();
        let mut acc = Accumulator709::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.483_f32 + y.sin();
        let b = y * 4.217_f32 - x.cos();
        let mut acc = Accumulator709::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.41_f32 + y.sin();
        let b = y * 2.722_f32 - x.cos();
        let mut acc = Accumulator709::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.287_f32 + y.sin();
        let b = y * 8.698_f32 - x.cos();
        let mut acc = Accumulator709::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.218_f32 + y.sin();
        let b = y * 9.854_f32 - x.cos();
        let mut acc = Accumulator709::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.248_f32 + y.sin();
        let b = y * 9.453_f32 - x.cos();
        let mut acc = Accumulator709::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.112_f32 + y.sin();
        let b = y * 7.506_f32 - x.cos();
        let mut acc = Accumulator709::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.677_f32 + y.sin();
        let b = y * 7.596_f32 - x.cos();
        let mut acc = Accumulator709::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.498_f32 + y.sin();
        let b = y * 6.029_f32 - x.cos();
        let mut acc = Accumulator709::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.732_f32 + y.sin();
        let b = y * 5.137_f32 - x.cos();
        let mut acc = Accumulator709::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.403_f32 + y.sin();
        let b = y * 9.262_f32 - x.cos();
        let mut acc = Accumulator709::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_709(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_709() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_709(total as u64) % 997) as f32;
        total
    }
}

pub mod m710 {
    use super::*;

    pub struct Accumulator710<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator710<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.845_f32 + y.sin();
        let b = y * 3.742_f32 - x.cos();
        let mut acc = Accumulator710::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.887_f32 + y.sin();
        let b = y * 0.453_f32 - x.cos();
        let mut acc = Accumulator710::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.024_f32 + y.sin();
        let b = y * 2.764_f32 - x.cos();
        let mut acc = Accumulator710::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.234_f32 + y.sin();
        let b = y * 6.456_f32 - x.cos();
        let mut acc = Accumulator710::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.442_f32 + y.sin();
        let b = y * 7.821_f32 - x.cos();
        let mut acc = Accumulator710::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.37_f32 + y.sin();
        let b = y * 7.854_f32 - x.cos();
        let mut acc = Accumulator710::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.002_f32 + y.sin();
        let b = y * 4.412_f32 - x.cos();
        let mut acc = Accumulator710::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.418_f32 + y.sin();
        let b = y * 2.346_f32 - x.cos();
        let mut acc = Accumulator710::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.17_f32 + y.sin();
        let b = y * 2.808_f32 - x.cos();
        let mut acc = Accumulator710::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.223_f32 + y.sin();
        let b = y * 5.105_f32 - x.cos();
        let mut acc = Accumulator710::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.098_f32 + y.sin();
        let b = y * 0.792_f32 - x.cos();
        let mut acc = Accumulator710::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.84_f32 + y.sin();
        let b = y * 0.899_f32 - x.cos();
        let mut acc = Accumulator710::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.942_f32 + y.sin();
        let b = y * 2.244_f32 - x.cos();
        let mut acc = Accumulator710::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.402_f32 + y.sin();
        let b = y * 0.183_f32 - x.cos();
        let mut acc = Accumulator710::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.296_f32 + y.sin();
        let b = y * 6.522_f32 - x.cos();
        let mut acc = Accumulator710::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.142_f32 + y.sin();
        let b = y * 7.646_f32 - x.cos();
        let mut acc = Accumulator710::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.383_f32 + y.sin();
        let b = y * 6.458_f32 - x.cos();
        let mut acc = Accumulator710::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.555_f32 + y.sin();
        let b = y * 9.697_f32 - x.cos();
        let mut acc = Accumulator710::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.989_f32 + y.sin();
        let b = y * 8.949_f32 - x.cos();
        let mut acc = Accumulator710::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.421_f32 + y.sin();
        let b = y * 7.149_f32 - x.cos();
        let mut acc = Accumulator710::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_710(seed: u64) -> u64 {
        let re = Regex::new(r"m710-(\d+)").unwrap();
        let hay = format!("m710-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_710() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_710(total as u64) % 997) as f32;
        total
    }
}

pub mod m711 {
    use super::*;

    pub struct Accumulator711<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator711<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.232_f32 + y.sin();
        let b = y * 1.621_f32 - x.cos();
        let mut acc = Accumulator711::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.959_f32 + y.sin();
        let b = y * 6.739_f32 - x.cos();
        let mut acc = Accumulator711::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.795_f32 + y.sin();
        let b = y * 2.899_f32 - x.cos();
        let mut acc = Accumulator711::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.135_f32 + y.sin();
        let b = y * 1.018_f32 - x.cos();
        let mut acc = Accumulator711::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.264_f32 + y.sin();
        let b = y * 1.454_f32 - x.cos();
        let mut acc = Accumulator711::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.95_f32 + y.sin();
        let b = y * 2.299_f32 - x.cos();
        let mut acc = Accumulator711::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.213_f32 + y.sin();
        let b = y * 1.345_f32 - x.cos();
        let mut acc = Accumulator711::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.004_f32 + y.sin();
        let b = y * 4.627_f32 - x.cos();
        let mut acc = Accumulator711::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.303_f32 + y.sin();
        let b = y * 3.15_f32 - x.cos();
        let mut acc = Accumulator711::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.845_f32 + y.sin();
        let b = y * 1.355_f32 - x.cos();
        let mut acc = Accumulator711::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.167_f32 + y.sin();
        let b = y * 7.863_f32 - x.cos();
        let mut acc = Accumulator711::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.234_f32 + y.sin();
        let b = y * 8.139_f32 - x.cos();
        let mut acc = Accumulator711::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.088_f32 + y.sin();
        let b = y * 1.918_f32 - x.cos();
        let mut acc = Accumulator711::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.013_f32 + y.sin();
        let b = y * 4.125_f32 - x.cos();
        let mut acc = Accumulator711::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.631_f32 + y.sin();
        let b = y * 8.255_f32 - x.cos();
        let mut acc = Accumulator711::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.278_f32 + y.sin();
        let b = y * 8.466_f32 - x.cos();
        let mut acc = Accumulator711::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.073_f32 + y.sin();
        let b = y * 7.363_f32 - x.cos();
        let mut acc = Accumulator711::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.042_f32 + y.sin();
        let b = y * 9.821_f32 - x.cos();
        let mut acc = Accumulator711::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.394_f32 + y.sin();
        let b = y * 7.861_f32 - x.cos();
        let mut acc = Accumulator711::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.198_f32 + y.sin();
        let b = y * 6.219_f32 - x.cos();
        let mut acc = Accumulator711::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_711(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_711() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_711(total as u64) % 997) as f32;
        total
    }
}

pub mod m712 {
    use super::*;

    pub struct Accumulator712<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator712<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.264_f32 + y.sin();
        let b = y * 1.008_f32 - x.cos();
        let mut acc = Accumulator712::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.875_f32 + y.sin();
        let b = y * 2.49_f32 - x.cos();
        let mut acc = Accumulator712::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.177_f32 + y.sin();
        let b = y * 0.882_f32 - x.cos();
        let mut acc = Accumulator712::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.909_f32 + y.sin();
        let b = y * 5.281_f32 - x.cos();
        let mut acc = Accumulator712::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.85_f32 + y.sin();
        let b = y * 3.273_f32 - x.cos();
        let mut acc = Accumulator712::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.119_f32 + y.sin();
        let b = y * 9.51_f32 - x.cos();
        let mut acc = Accumulator712::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.86_f32 + y.sin();
        let b = y * 5.056_f32 - x.cos();
        let mut acc = Accumulator712::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.901_f32 + y.sin();
        let b = y * 4.321_f32 - x.cos();
        let mut acc = Accumulator712::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.134_f32 + y.sin();
        let b = y * 1.067_f32 - x.cos();
        let mut acc = Accumulator712::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.285_f32 + y.sin();
        let b = y * 1.537_f32 - x.cos();
        let mut acc = Accumulator712::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.559_f32 + y.sin();
        let b = y * 6.945_f32 - x.cos();
        let mut acc = Accumulator712::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.559_f32 + y.sin();
        let b = y * 2.287_f32 - x.cos();
        let mut acc = Accumulator712::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.581_f32 + y.sin();
        let b = y * 4.417_f32 - x.cos();
        let mut acc = Accumulator712::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.768_f32 + y.sin();
        let b = y * 0.373_f32 - x.cos();
        let mut acc = Accumulator712::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.239_f32 + y.sin();
        let b = y * 3.627_f32 - x.cos();
        let mut acc = Accumulator712::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.656_f32 + y.sin();
        let b = y * 6.439_f32 - x.cos();
        let mut acc = Accumulator712::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.971_f32 + y.sin();
        let b = y * 2.56_f32 - x.cos();
        let mut acc = Accumulator712::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.077_f32 + y.sin();
        let b = y * 8.54_f32 - x.cos();
        let mut acc = Accumulator712::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.143_f32 + y.sin();
        let b = y * 2.021_f32 - x.cos();
        let mut acc = Accumulator712::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.726_f32 + y.sin();
        let b = y * 9.784_f32 - x.cos();
        let mut acc = Accumulator712::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_712(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(712u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_712() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_712(total as u64) % 997) as f32;
        total
    }
}

pub mod m713 {
    use super::*;

    pub struct Accumulator713<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator713<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.822_f32 + y.sin();
        let b = y * 9.155_f32 - x.cos();
        let mut acc = Accumulator713::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.482_f32 + y.sin();
        let b = y * 2.68_f32 - x.cos();
        let mut acc = Accumulator713::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.792_f32 + y.sin();
        let b = y * 4.267_f32 - x.cos();
        let mut acc = Accumulator713::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.748_f32 + y.sin();
        let b = y * 6.409_f32 - x.cos();
        let mut acc = Accumulator713::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.507_f32 + y.sin();
        let b = y * 4.511_f32 - x.cos();
        let mut acc = Accumulator713::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.219_f32 + y.sin();
        let b = y * 0.374_f32 - x.cos();
        let mut acc = Accumulator713::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.592_f32 + y.sin();
        let b = y * 6.716_f32 - x.cos();
        let mut acc = Accumulator713::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.489_f32 + y.sin();
        let b = y * 6.674_f32 - x.cos();
        let mut acc = Accumulator713::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.528_f32 + y.sin();
        let b = y * 2.043_f32 - x.cos();
        let mut acc = Accumulator713::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.543_f32 + y.sin();
        let b = y * 2.098_f32 - x.cos();
        let mut acc = Accumulator713::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.202_f32 + y.sin();
        let b = y * 2.048_f32 - x.cos();
        let mut acc = Accumulator713::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.264_f32 + y.sin();
        let b = y * 5.656_f32 - x.cos();
        let mut acc = Accumulator713::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.853_f32 + y.sin();
        let b = y * 2.919_f32 - x.cos();
        let mut acc = Accumulator713::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.985_f32 + y.sin();
        let b = y * 3.47_f32 - x.cos();
        let mut acc = Accumulator713::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.169_f32 + y.sin();
        let b = y * 0.165_f32 - x.cos();
        let mut acc = Accumulator713::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.674_f32 + y.sin();
        let b = y * 6.658_f32 - x.cos();
        let mut acc = Accumulator713::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.376_f32 + y.sin();
        let b = y * 0.774_f32 - x.cos();
        let mut acc = Accumulator713::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.275_f32 + y.sin();
        let b = y * 5.989_f32 - x.cos();
        let mut acc = Accumulator713::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.038_f32 + y.sin();
        let b = y * 1.947_f32 - x.cos();
        let mut acc = Accumulator713::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.291_f32 + y.sin();
        let b = y * 6.706_f32 - x.cos();
        let mut acc = Accumulator713::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_713(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_713() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_713(total as u64) % 997) as f32;
        total
    }
}

pub mod m714 {
    use super::*;

    pub struct Accumulator714<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator714<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.719_f32 + y.sin();
        let b = y * 3.176_f32 - x.cos();
        let mut acc = Accumulator714::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.01_f32 + y.sin();
        let b = y * 4.41_f32 - x.cos();
        let mut acc = Accumulator714::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.307_f32 + y.sin();
        let b = y * 6.664_f32 - x.cos();
        let mut acc = Accumulator714::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.126_f32 + y.sin();
        let b = y * 6.572_f32 - x.cos();
        let mut acc = Accumulator714::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.8_f32 + y.sin();
        let b = y * 9.172_f32 - x.cos();
        let mut acc = Accumulator714::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.752_f32 + y.sin();
        let b = y * 6.454_f32 - x.cos();
        let mut acc = Accumulator714::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.709_f32 + y.sin();
        let b = y * 0.516_f32 - x.cos();
        let mut acc = Accumulator714::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.812_f32 + y.sin();
        let b = y * 9.851_f32 - x.cos();
        let mut acc = Accumulator714::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.515_f32 + y.sin();
        let b = y * 4.564_f32 - x.cos();
        let mut acc = Accumulator714::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.743_f32 + y.sin();
        let b = y * 6.805_f32 - x.cos();
        let mut acc = Accumulator714::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.221_f32 + y.sin();
        let b = y * 0.395_f32 - x.cos();
        let mut acc = Accumulator714::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.604_f32 + y.sin();
        let b = y * 3.403_f32 - x.cos();
        let mut acc = Accumulator714::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.194_f32 + y.sin();
        let b = y * 7.279_f32 - x.cos();
        let mut acc = Accumulator714::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.288_f32 + y.sin();
        let b = y * 0.78_f32 - x.cos();
        let mut acc = Accumulator714::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.211_f32 + y.sin();
        let b = y * 1.377_f32 - x.cos();
        let mut acc = Accumulator714::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.192_f32 + y.sin();
        let b = y * 4.487_f32 - x.cos();
        let mut acc = Accumulator714::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.681_f32 + y.sin();
        let b = y * 6.602_f32 - x.cos();
        let mut acc = Accumulator714::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.809_f32 + y.sin();
        let b = y * 2.205_f32 - x.cos();
        let mut acc = Accumulator714::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.109_f32 + y.sin();
        let b = y * 5.875_f32 - x.cos();
        let mut acc = Accumulator714::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.529_f32 + y.sin();
        let b = y * 2.525_f32 - x.cos();
        let mut acc = Accumulator714::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_714(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_714() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_714(total as u64) % 997) as f32;
        total
    }
}

pub mod m715 {
    use super::*;

    pub struct Accumulator715<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator715<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.586_f32 + y.sin();
        let b = y * 8.982_f32 - x.cos();
        let mut acc = Accumulator715::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.757_f32 + y.sin();
        let b = y * 8.214_f32 - x.cos();
        let mut acc = Accumulator715::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.657_f32 + y.sin();
        let b = y * 8.335_f32 - x.cos();
        let mut acc = Accumulator715::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.135_f32 + y.sin();
        let b = y * 4.132_f32 - x.cos();
        let mut acc = Accumulator715::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.27_f32 + y.sin();
        let b = y * 6.249_f32 - x.cos();
        let mut acc = Accumulator715::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.475_f32 + y.sin();
        let b = y * 5.768_f32 - x.cos();
        let mut acc = Accumulator715::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.172_f32 + y.sin();
        let b = y * 3.757_f32 - x.cos();
        let mut acc = Accumulator715::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.188_f32 + y.sin();
        let b = y * 2.053_f32 - x.cos();
        let mut acc = Accumulator715::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.897_f32 + y.sin();
        let b = y * 5.293_f32 - x.cos();
        let mut acc = Accumulator715::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.773_f32 + y.sin();
        let b = y * 6.575_f32 - x.cos();
        let mut acc = Accumulator715::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.824_f32 + y.sin();
        let b = y * 8.689_f32 - x.cos();
        let mut acc = Accumulator715::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.531_f32 + y.sin();
        let b = y * 4.259_f32 - x.cos();
        let mut acc = Accumulator715::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.414_f32 + y.sin();
        let b = y * 3.749_f32 - x.cos();
        let mut acc = Accumulator715::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.772_f32 + y.sin();
        let b = y * 3.547_f32 - x.cos();
        let mut acc = Accumulator715::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.996_f32 + y.sin();
        let b = y * 0.625_f32 - x.cos();
        let mut acc = Accumulator715::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.768_f32 + y.sin();
        let b = y * 1.818_f32 - x.cos();
        let mut acc = Accumulator715::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.823_f32 + y.sin();
        let b = y * 1.79_f32 - x.cos();
        let mut acc = Accumulator715::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.206_f32 + y.sin();
        let b = y * 7.048_f32 - x.cos();
        let mut acc = Accumulator715::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.047_f32 + y.sin();
        let b = y * 6.013_f32 - x.cos();
        let mut acc = Accumulator715::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.261_f32 + y.sin();
        let b = y * 8.987_f32 - x.cos();
        let mut acc = Accumulator715::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_715(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m715-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_715() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_715(total as u64) % 997) as f32;
        total
    }
}

pub mod m716 {
    use super::*;

    pub struct Accumulator716<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator716<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.245_f32 + y.sin();
        let b = y * 8.88_f32 - x.cos();
        let mut acc = Accumulator716::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.047_f32 + y.sin();
        let b = y * 7.425_f32 - x.cos();
        let mut acc = Accumulator716::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.934_f32 + y.sin();
        let b = y * 8.98_f32 - x.cos();
        let mut acc = Accumulator716::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.91_f32 + y.sin();
        let b = y * 2.332_f32 - x.cos();
        let mut acc = Accumulator716::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.289_f32 + y.sin();
        let b = y * 4.244_f32 - x.cos();
        let mut acc = Accumulator716::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.916_f32 + y.sin();
        let b = y * 1.53_f32 - x.cos();
        let mut acc = Accumulator716::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.504_f32 + y.sin();
        let b = y * 3.426_f32 - x.cos();
        let mut acc = Accumulator716::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.508_f32 + y.sin();
        let b = y * 9.168_f32 - x.cos();
        let mut acc = Accumulator716::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.698_f32 + y.sin();
        let b = y * 7.924_f32 - x.cos();
        let mut acc = Accumulator716::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.843_f32 + y.sin();
        let b = y * 2.97_f32 - x.cos();
        let mut acc = Accumulator716::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.399_f32 + y.sin();
        let b = y * 1.159_f32 - x.cos();
        let mut acc = Accumulator716::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.049_f32 + y.sin();
        let b = y * 8.719_f32 - x.cos();
        let mut acc = Accumulator716::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.053_f32 + y.sin();
        let b = y * 9.054_f32 - x.cos();
        let mut acc = Accumulator716::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.075_f32 + y.sin();
        let b = y * 6.381_f32 - x.cos();
        let mut acc = Accumulator716::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.417_f32 + y.sin();
        let b = y * 1.492_f32 - x.cos();
        let mut acc = Accumulator716::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.71_f32 + y.sin();
        let b = y * 3.504_f32 - x.cos();
        let mut acc = Accumulator716::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.962_f32 + y.sin();
        let b = y * 6.667_f32 - x.cos();
        let mut acc = Accumulator716::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.99_f32 + y.sin();
        let b = y * 5.988_f32 - x.cos();
        let mut acc = Accumulator716::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.728_f32 + y.sin();
        let b = y * 2.694_f32 - x.cos();
        let mut acc = Accumulator716::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.629_f32 + y.sin();
        let b = y * 3.291_f32 - x.cos();
        let mut acc = Accumulator716::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_716(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_716() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_716(total as u64) % 997) as f32;
        total
    }
}

pub mod m717 {
    use super::*;

    pub struct Accumulator717<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator717<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.818_f32 + y.sin();
        let b = y * 6.692_f32 - x.cos();
        let mut acc = Accumulator717::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.159_f32 + y.sin();
        let b = y * 9.382_f32 - x.cos();
        let mut acc = Accumulator717::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.809_f32 + y.sin();
        let b = y * 1.787_f32 - x.cos();
        let mut acc = Accumulator717::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.413_f32 + y.sin();
        let b = y * 4.524_f32 - x.cos();
        let mut acc = Accumulator717::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.673_f32 + y.sin();
        let b = y * 6.832_f32 - x.cos();
        let mut acc = Accumulator717::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.842_f32 + y.sin();
        let b = y * 7.783_f32 - x.cos();
        let mut acc = Accumulator717::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.359_f32 + y.sin();
        let b = y * 2.947_f32 - x.cos();
        let mut acc = Accumulator717::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.548_f32 + y.sin();
        let b = y * 2.896_f32 - x.cos();
        let mut acc = Accumulator717::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.269_f32 + y.sin();
        let b = y * 2.151_f32 - x.cos();
        let mut acc = Accumulator717::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.545_f32 + y.sin();
        let b = y * 2.92_f32 - x.cos();
        let mut acc = Accumulator717::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.407_f32 + y.sin();
        let b = y * 4.338_f32 - x.cos();
        let mut acc = Accumulator717::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.646_f32 + y.sin();
        let b = y * 5.09_f32 - x.cos();
        let mut acc = Accumulator717::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.346_f32 + y.sin();
        let b = y * 9.487_f32 - x.cos();
        let mut acc = Accumulator717::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.835_f32 + y.sin();
        let b = y * 9.137_f32 - x.cos();
        let mut acc = Accumulator717::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.747_f32 + y.sin();
        let b = y * 5.664_f32 - x.cos();
        let mut acc = Accumulator717::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.695_f32 + y.sin();
        let b = y * 4.836_f32 - x.cos();
        let mut acc = Accumulator717::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.162_f32 + y.sin();
        let b = y * 1.409_f32 - x.cos();
        let mut acc = Accumulator717::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.863_f32 + y.sin();
        let b = y * 7.888_f32 - x.cos();
        let mut acc = Accumulator717::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.42_f32 + y.sin();
        let b = y * 8.978_f32 - x.cos();
        let mut acc = Accumulator717::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.079_f32 + y.sin();
        let b = y * 0.321_f32 - x.cos();
        let mut acc = Accumulator717::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_717(seed: u64) -> u64 {
        let re = Regex::new(r"m717-(\d+)").unwrap();
        let hay = format!("m717-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_717() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_717(total as u64) % 997) as f32;
        total
    }
}

pub mod m718 {
    use super::*;

    pub struct Accumulator718<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator718<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.384_f32 + y.sin();
        let b = y * 9.081_f32 - x.cos();
        let mut acc = Accumulator718::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.545_f32 + y.sin();
        let b = y * 3.507_f32 - x.cos();
        let mut acc = Accumulator718::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.783_f32 + y.sin();
        let b = y * 1.335_f32 - x.cos();
        let mut acc = Accumulator718::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.867_f32 + y.sin();
        let b = y * 2.302_f32 - x.cos();
        let mut acc = Accumulator718::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.66_f32 + y.sin();
        let b = y * 7.27_f32 - x.cos();
        let mut acc = Accumulator718::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.813_f32 + y.sin();
        let b = y * 3.752_f32 - x.cos();
        let mut acc = Accumulator718::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.402_f32 + y.sin();
        let b = y * 7.611_f32 - x.cos();
        let mut acc = Accumulator718::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.033_f32 + y.sin();
        let b = y * 2.67_f32 - x.cos();
        let mut acc = Accumulator718::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.602_f32 + y.sin();
        let b = y * 7.985_f32 - x.cos();
        let mut acc = Accumulator718::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.553_f32 + y.sin();
        let b = y * 0.493_f32 - x.cos();
        let mut acc = Accumulator718::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.056_f32 + y.sin();
        let b = y * 0.58_f32 - x.cos();
        let mut acc = Accumulator718::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.5_f32 + y.sin();
        let b = y * 3.061_f32 - x.cos();
        let mut acc = Accumulator718::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.664_f32 + y.sin();
        let b = y * 9.063_f32 - x.cos();
        let mut acc = Accumulator718::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.386_f32 + y.sin();
        let b = y * 6.264_f32 - x.cos();
        let mut acc = Accumulator718::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.174_f32 + y.sin();
        let b = y * 7.349_f32 - x.cos();
        let mut acc = Accumulator718::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.763_f32 + y.sin();
        let b = y * 2.648_f32 - x.cos();
        let mut acc = Accumulator718::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.196_f32 + y.sin();
        let b = y * 5.732_f32 - x.cos();
        let mut acc = Accumulator718::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.16_f32 + y.sin();
        let b = y * 6.651_f32 - x.cos();
        let mut acc = Accumulator718::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.067_f32 + y.sin();
        let b = y * 3.621_f32 - x.cos();
        let mut acc = Accumulator718::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.229_f32 + y.sin();
        let b = y * 4.96_f32 - x.cos();
        let mut acc = Accumulator718::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_718(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_718() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_718(total as u64) % 997) as f32;
        total
    }
}

pub mod m719 {
    use super::*;

    pub struct Accumulator719<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator719<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.508_f32 + y.sin();
        let b = y * 2.025_f32 - x.cos();
        let mut acc = Accumulator719::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.734_f32 + y.sin();
        let b = y * 5.082_f32 - x.cos();
        let mut acc = Accumulator719::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.64_f32 + y.sin();
        let b = y * 4.997_f32 - x.cos();
        let mut acc = Accumulator719::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.503_f32 + y.sin();
        let b = y * 2.063_f32 - x.cos();
        let mut acc = Accumulator719::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.649_f32 + y.sin();
        let b = y * 5.032_f32 - x.cos();
        let mut acc = Accumulator719::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.545_f32 + y.sin();
        let b = y * 8.743_f32 - x.cos();
        let mut acc = Accumulator719::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.393_f32 + y.sin();
        let b = y * 3.209_f32 - x.cos();
        let mut acc = Accumulator719::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.704_f32 + y.sin();
        let b = y * 4.718_f32 - x.cos();
        let mut acc = Accumulator719::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.852_f32 + y.sin();
        let b = y * 6.825_f32 - x.cos();
        let mut acc = Accumulator719::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.238_f32 + y.sin();
        let b = y * 3.192_f32 - x.cos();
        let mut acc = Accumulator719::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.016_f32 + y.sin();
        let b = y * 8.411_f32 - x.cos();
        let mut acc = Accumulator719::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.365_f32 + y.sin();
        let b = y * 4.912_f32 - x.cos();
        let mut acc = Accumulator719::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.534_f32 + y.sin();
        let b = y * 4.987_f32 - x.cos();
        let mut acc = Accumulator719::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.391_f32 + y.sin();
        let b = y * 7.781_f32 - x.cos();
        let mut acc = Accumulator719::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.791_f32 + y.sin();
        let b = y * 8.436_f32 - x.cos();
        let mut acc = Accumulator719::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.019_f32 + y.sin();
        let b = y * 5.258_f32 - x.cos();
        let mut acc = Accumulator719::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.898_f32 + y.sin();
        let b = y * 6.458_f32 - x.cos();
        let mut acc = Accumulator719::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.455_f32 + y.sin();
        let b = y * 8.685_f32 - x.cos();
        let mut acc = Accumulator719::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.013_f32 + y.sin();
        let b = y * 4.813_f32 - x.cos();
        let mut acc = Accumulator719::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.346_f32 + y.sin();
        let b = y * 2.266_f32 - x.cos();
        let mut acc = Accumulator719::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_719(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(719u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_719() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_719(total as u64) % 997) as f32;
        total
    }
}

pub mod m720 {
    use super::*;

    pub struct Accumulator720<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator720<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.195_f32 + y.sin();
        let b = y * 3.356_f32 - x.cos();
        let mut acc = Accumulator720::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.594_f32 + y.sin();
        let b = y * 4.636_f32 - x.cos();
        let mut acc = Accumulator720::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.195_f32 + y.sin();
        let b = y * 6.473_f32 - x.cos();
        let mut acc = Accumulator720::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.582_f32 + y.sin();
        let b = y * 2.277_f32 - x.cos();
        let mut acc = Accumulator720::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.634_f32 + y.sin();
        let b = y * 3.604_f32 - x.cos();
        let mut acc = Accumulator720::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.652_f32 + y.sin();
        let b = y * 9.726_f32 - x.cos();
        let mut acc = Accumulator720::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.063_f32 + y.sin();
        let b = y * 5.816_f32 - x.cos();
        let mut acc = Accumulator720::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.603_f32 + y.sin();
        let b = y * 5.151_f32 - x.cos();
        let mut acc = Accumulator720::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.851_f32 + y.sin();
        let b = y * 8.264_f32 - x.cos();
        let mut acc = Accumulator720::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.418_f32 + y.sin();
        let b = y * 2.828_f32 - x.cos();
        let mut acc = Accumulator720::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.778_f32 + y.sin();
        let b = y * 7.396_f32 - x.cos();
        let mut acc = Accumulator720::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.867_f32 + y.sin();
        let b = y * 3.444_f32 - x.cos();
        let mut acc = Accumulator720::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.053_f32 + y.sin();
        let b = y * 3.06_f32 - x.cos();
        let mut acc = Accumulator720::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.336_f32 + y.sin();
        let b = y * 8.221_f32 - x.cos();
        let mut acc = Accumulator720::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.687_f32 + y.sin();
        let b = y * 1.441_f32 - x.cos();
        let mut acc = Accumulator720::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.644_f32 + y.sin();
        let b = y * 5.124_f32 - x.cos();
        let mut acc = Accumulator720::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.992_f32 + y.sin();
        let b = y * 5.478_f32 - x.cos();
        let mut acc = Accumulator720::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.146_f32 + y.sin();
        let b = y * 2.508_f32 - x.cos();
        let mut acc = Accumulator720::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.38_f32 + y.sin();
        let b = y * 5.339_f32 - x.cos();
        let mut acc = Accumulator720::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.59_f32 + y.sin();
        let b = y * 7.474_f32 - x.cos();
        let mut acc = Accumulator720::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_720(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_720() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_720(total as u64) % 997) as f32;
        total
    }
}

pub mod m721 {
    use super::*;

    pub struct Accumulator721<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator721<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.511_f32 + y.sin();
        let b = y * 5.644_f32 - x.cos();
        let mut acc = Accumulator721::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.825_f32 + y.sin();
        let b = y * 9.039_f32 - x.cos();
        let mut acc = Accumulator721::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.659_f32 + y.sin();
        let b = y * 0.624_f32 - x.cos();
        let mut acc = Accumulator721::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.024_f32 + y.sin();
        let b = y * 1.192_f32 - x.cos();
        let mut acc = Accumulator721::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.063_f32 + y.sin();
        let b = y * 9.746_f32 - x.cos();
        let mut acc = Accumulator721::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.996_f32 + y.sin();
        let b = y * 1.937_f32 - x.cos();
        let mut acc = Accumulator721::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.023_f32 + y.sin();
        let b = y * 6.599_f32 - x.cos();
        let mut acc = Accumulator721::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.215_f32 + y.sin();
        let b = y * 0.932_f32 - x.cos();
        let mut acc = Accumulator721::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.173_f32 + y.sin();
        let b = y * 6.227_f32 - x.cos();
        let mut acc = Accumulator721::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.015_f32 + y.sin();
        let b = y * 7.684_f32 - x.cos();
        let mut acc = Accumulator721::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.467_f32 + y.sin();
        let b = y * 1.418_f32 - x.cos();
        let mut acc = Accumulator721::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.713_f32 + y.sin();
        let b = y * 3.393_f32 - x.cos();
        let mut acc = Accumulator721::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.134_f32 + y.sin();
        let b = y * 0.741_f32 - x.cos();
        let mut acc = Accumulator721::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.078_f32 + y.sin();
        let b = y * 4.339_f32 - x.cos();
        let mut acc = Accumulator721::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.599_f32 + y.sin();
        let b = y * 7.872_f32 - x.cos();
        let mut acc = Accumulator721::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.877_f32 + y.sin();
        let b = y * 4.207_f32 - x.cos();
        let mut acc = Accumulator721::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.397_f32 + y.sin();
        let b = y * 1.597_f32 - x.cos();
        let mut acc = Accumulator721::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.045_f32 + y.sin();
        let b = y * 2.722_f32 - x.cos();
        let mut acc = Accumulator721::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.439_f32 + y.sin();
        let b = y * 7.931_f32 - x.cos();
        let mut acc = Accumulator721::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.641_f32 + y.sin();
        let b = y * 7.272_f32 - x.cos();
        let mut acc = Accumulator721::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_721(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_721() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_721(total as u64) % 997) as f32;
        total
    }
}

pub mod m722 {
    use super::*;

    pub struct Accumulator722<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator722<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.95_f32 + y.sin();
        let b = y * 8.713_f32 - x.cos();
        let mut acc = Accumulator722::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.755_f32 + y.sin();
        let b = y * 9.357_f32 - x.cos();
        let mut acc = Accumulator722::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.35_f32 + y.sin();
        let b = y * 5.351_f32 - x.cos();
        let mut acc = Accumulator722::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.655_f32 + y.sin();
        let b = y * 4.573_f32 - x.cos();
        let mut acc = Accumulator722::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.495_f32 + y.sin();
        let b = y * 7.931_f32 - x.cos();
        let mut acc = Accumulator722::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.335_f32 + y.sin();
        let b = y * 8.661_f32 - x.cos();
        let mut acc = Accumulator722::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.449_f32 + y.sin();
        let b = y * 4.096_f32 - x.cos();
        let mut acc = Accumulator722::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.301_f32 + y.sin();
        let b = y * 0.513_f32 - x.cos();
        let mut acc = Accumulator722::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.957_f32 + y.sin();
        let b = y * 3.0_f32 - x.cos();
        let mut acc = Accumulator722::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.994_f32 + y.sin();
        let b = y * 0.227_f32 - x.cos();
        let mut acc = Accumulator722::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.235_f32 + y.sin();
        let b = y * 3.559_f32 - x.cos();
        let mut acc = Accumulator722::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.376_f32 + y.sin();
        let b = y * 5.883_f32 - x.cos();
        let mut acc = Accumulator722::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.649_f32 + y.sin();
        let b = y * 5.979_f32 - x.cos();
        let mut acc = Accumulator722::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.524_f32 + y.sin();
        let b = y * 4.239_f32 - x.cos();
        let mut acc = Accumulator722::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.859_f32 + y.sin();
        let b = y * 8.264_f32 - x.cos();
        let mut acc = Accumulator722::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.765_f32 + y.sin();
        let b = y * 0.749_f32 - x.cos();
        let mut acc = Accumulator722::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.077_f32 + y.sin();
        let b = y * 2.966_f32 - x.cos();
        let mut acc = Accumulator722::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.69_f32 + y.sin();
        let b = y * 1.749_f32 - x.cos();
        let mut acc = Accumulator722::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.364_f32 + y.sin();
        let b = y * 7.13_f32 - x.cos();
        let mut acc = Accumulator722::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.757_f32 + y.sin();
        let b = y * 6.343_f32 - x.cos();
        let mut acc = Accumulator722::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_722(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m722-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_722() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_722(total as u64) % 997) as f32;
        total
    }
}

pub mod m723 {
    use super::*;

    pub struct Accumulator723<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator723<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.526_f32 + y.sin();
        let b = y * 2.5_f32 - x.cos();
        let mut acc = Accumulator723::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.891_f32 + y.sin();
        let b = y * 7.665_f32 - x.cos();
        let mut acc = Accumulator723::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.172_f32 + y.sin();
        let b = y * 0.308_f32 - x.cos();
        let mut acc = Accumulator723::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.192_f32 + y.sin();
        let b = y * 1.039_f32 - x.cos();
        let mut acc = Accumulator723::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.969_f32 + y.sin();
        let b = y * 6.462_f32 - x.cos();
        let mut acc = Accumulator723::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.475_f32 + y.sin();
        let b = y * 2.176_f32 - x.cos();
        let mut acc = Accumulator723::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.169_f32 + y.sin();
        let b = y * 0.376_f32 - x.cos();
        let mut acc = Accumulator723::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.334_f32 + y.sin();
        let b = y * 5.99_f32 - x.cos();
        let mut acc = Accumulator723::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.741_f32 + y.sin();
        let b = y * 9.177_f32 - x.cos();
        let mut acc = Accumulator723::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.896_f32 + y.sin();
        let b = y * 8.363_f32 - x.cos();
        let mut acc = Accumulator723::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.823_f32 + y.sin();
        let b = y * 0.901_f32 - x.cos();
        let mut acc = Accumulator723::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.104_f32 + y.sin();
        let b = y * 3.245_f32 - x.cos();
        let mut acc = Accumulator723::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.89_f32 + y.sin();
        let b = y * 0.276_f32 - x.cos();
        let mut acc = Accumulator723::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.564_f32 + y.sin();
        let b = y * 5.901_f32 - x.cos();
        let mut acc = Accumulator723::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.371_f32 + y.sin();
        let b = y * 9.052_f32 - x.cos();
        let mut acc = Accumulator723::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.618_f32 + y.sin();
        let b = y * 1.328_f32 - x.cos();
        let mut acc = Accumulator723::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.036_f32 + y.sin();
        let b = y * 0.349_f32 - x.cos();
        let mut acc = Accumulator723::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.086_f32 + y.sin();
        let b = y * 8.612_f32 - x.cos();
        let mut acc = Accumulator723::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.527_f32 + y.sin();
        let b = y * 4.499_f32 - x.cos();
        let mut acc = Accumulator723::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.041_f32 + y.sin();
        let b = y * 3.676_f32 - x.cos();
        let mut acc = Accumulator723::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_723(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_723() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_723(total as u64) % 997) as f32;
        total
    }
}

pub mod m724 {
    use super::*;

    pub struct Accumulator724<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator724<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.189_f32 + y.sin();
        let b = y * 6.911_f32 - x.cos();
        let mut acc = Accumulator724::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.764_f32 + y.sin();
        let b = y * 4.39_f32 - x.cos();
        let mut acc = Accumulator724::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.639_f32 + y.sin();
        let b = y * 4.249_f32 - x.cos();
        let mut acc = Accumulator724::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.175_f32 + y.sin();
        let b = y * 8.9_f32 - x.cos();
        let mut acc = Accumulator724::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.977_f32 + y.sin();
        let b = y * 0.246_f32 - x.cos();
        let mut acc = Accumulator724::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.826_f32 + y.sin();
        let b = y * 8.952_f32 - x.cos();
        let mut acc = Accumulator724::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.31_f32 + y.sin();
        let b = y * 5.228_f32 - x.cos();
        let mut acc = Accumulator724::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.4_f32 + y.sin();
        let b = y * 0.256_f32 - x.cos();
        let mut acc = Accumulator724::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.858_f32 + y.sin();
        let b = y * 4.085_f32 - x.cos();
        let mut acc = Accumulator724::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.306_f32 + y.sin();
        let b = y * 4.111_f32 - x.cos();
        let mut acc = Accumulator724::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.126_f32 + y.sin();
        let b = y * 2.946_f32 - x.cos();
        let mut acc = Accumulator724::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.328_f32 + y.sin();
        let b = y * 2.618_f32 - x.cos();
        let mut acc = Accumulator724::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.639_f32 + y.sin();
        let b = y * 4.005_f32 - x.cos();
        let mut acc = Accumulator724::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.821_f32 + y.sin();
        let b = y * 3.274_f32 - x.cos();
        let mut acc = Accumulator724::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.414_f32 + y.sin();
        let b = y * 4.782_f32 - x.cos();
        let mut acc = Accumulator724::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.463_f32 + y.sin();
        let b = y * 5.944_f32 - x.cos();
        let mut acc = Accumulator724::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.59_f32 + y.sin();
        let b = y * 0.933_f32 - x.cos();
        let mut acc = Accumulator724::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.156_f32 + y.sin();
        let b = y * 8.396_f32 - x.cos();
        let mut acc = Accumulator724::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.666_f32 + y.sin();
        let b = y * 7.415_f32 - x.cos();
        let mut acc = Accumulator724::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.23_f32 + y.sin();
        let b = y * 2.123_f32 - x.cos();
        let mut acc = Accumulator724::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_724(seed: u64) -> u64 {
        let re = Regex::new(r"m724-(\d+)").unwrap();
        let hay = format!("m724-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_724() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_724(total as u64) % 997) as f32;
        total
    }
}

pub mod m725 {
    use super::*;

    pub struct Accumulator725<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator725<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.913_f32 + y.sin();
        let b = y * 3.317_f32 - x.cos();
        let mut acc = Accumulator725::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.572_f32 + y.sin();
        let b = y * 1.781_f32 - x.cos();
        let mut acc = Accumulator725::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.389_f32 + y.sin();
        let b = y * 7.086_f32 - x.cos();
        let mut acc = Accumulator725::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.252_f32 + y.sin();
        let b = y * 9.619_f32 - x.cos();
        let mut acc = Accumulator725::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.798_f32 + y.sin();
        let b = y * 4.544_f32 - x.cos();
        let mut acc = Accumulator725::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.668_f32 + y.sin();
        let b = y * 0.824_f32 - x.cos();
        let mut acc = Accumulator725::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.528_f32 + y.sin();
        let b = y * 9.516_f32 - x.cos();
        let mut acc = Accumulator725::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.632_f32 + y.sin();
        let b = y * 8.41_f32 - x.cos();
        let mut acc = Accumulator725::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.043_f32 + y.sin();
        let b = y * 4.639_f32 - x.cos();
        let mut acc = Accumulator725::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.463_f32 + y.sin();
        let b = y * 9.377_f32 - x.cos();
        let mut acc = Accumulator725::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.02_f32 + y.sin();
        let b = y * 0.465_f32 - x.cos();
        let mut acc = Accumulator725::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.875_f32 + y.sin();
        let b = y * 7.917_f32 - x.cos();
        let mut acc = Accumulator725::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.496_f32 + y.sin();
        let b = y * 7.538_f32 - x.cos();
        let mut acc = Accumulator725::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.517_f32 + y.sin();
        let b = y * 7.687_f32 - x.cos();
        let mut acc = Accumulator725::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.202_f32 + y.sin();
        let b = y * 7.205_f32 - x.cos();
        let mut acc = Accumulator725::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.912_f32 + y.sin();
        let b = y * 5.246_f32 - x.cos();
        let mut acc = Accumulator725::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.17_f32 + y.sin();
        let b = y * 3.555_f32 - x.cos();
        let mut acc = Accumulator725::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.88_f32 + y.sin();
        let b = y * 7.639_f32 - x.cos();
        let mut acc = Accumulator725::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.41_f32 + y.sin();
        let b = y * 1.435_f32 - x.cos();
        let mut acc = Accumulator725::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.801_f32 + y.sin();
        let b = y * 3.514_f32 - x.cos();
        let mut acc = Accumulator725::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_725(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_725() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_725(total as u64) % 997) as f32;
        total
    }
}

pub mod m726 {
    use super::*;

    pub struct Accumulator726<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator726<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.543_f32 + y.sin();
        let b = y * 7.788_f32 - x.cos();
        let mut acc = Accumulator726::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.866_f32 + y.sin();
        let b = y * 1.536_f32 - x.cos();
        let mut acc = Accumulator726::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.25_f32 + y.sin();
        let b = y * 8.515_f32 - x.cos();
        let mut acc = Accumulator726::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.72_f32 + y.sin();
        let b = y * 1.786_f32 - x.cos();
        let mut acc = Accumulator726::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.243_f32 + y.sin();
        let b = y * 6.751_f32 - x.cos();
        let mut acc = Accumulator726::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.426_f32 + y.sin();
        let b = y * 0.154_f32 - x.cos();
        let mut acc = Accumulator726::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.531_f32 + y.sin();
        let b = y * 6.038_f32 - x.cos();
        let mut acc = Accumulator726::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.155_f32 + y.sin();
        let b = y * 7.823_f32 - x.cos();
        let mut acc = Accumulator726::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.558_f32 + y.sin();
        let b = y * 9.615_f32 - x.cos();
        let mut acc = Accumulator726::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.148_f32 + y.sin();
        let b = y * 1.231_f32 - x.cos();
        let mut acc = Accumulator726::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.252_f32 + y.sin();
        let b = y * 0.56_f32 - x.cos();
        let mut acc = Accumulator726::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.721_f32 + y.sin();
        let b = y * 7.175_f32 - x.cos();
        let mut acc = Accumulator726::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.114_f32 + y.sin();
        let b = y * 3.771_f32 - x.cos();
        let mut acc = Accumulator726::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.082_f32 + y.sin();
        let b = y * 3.85_f32 - x.cos();
        let mut acc = Accumulator726::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.777_f32 + y.sin();
        let b = y * 5.851_f32 - x.cos();
        let mut acc = Accumulator726::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.056_f32 + y.sin();
        let b = y * 2.088_f32 - x.cos();
        let mut acc = Accumulator726::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.16_f32 + y.sin();
        let b = y * 5.411_f32 - x.cos();
        let mut acc = Accumulator726::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.72_f32 + y.sin();
        let b = y * 9.148_f32 - x.cos();
        let mut acc = Accumulator726::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.01_f32 + y.sin();
        let b = y * 7.685_f32 - x.cos();
        let mut acc = Accumulator726::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.971_f32 + y.sin();
        let b = y * 0.912_f32 - x.cos();
        let mut acc = Accumulator726::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_726(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(726u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_726() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_726(total as u64) % 997) as f32;
        total
    }
}

pub mod m727 {
    use super::*;

    pub struct Accumulator727<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator727<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.9_f32 + y.sin();
        let b = y * 0.653_f32 - x.cos();
        let mut acc = Accumulator727::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.316_f32 + y.sin();
        let b = y * 7.833_f32 - x.cos();
        let mut acc = Accumulator727::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.444_f32 + y.sin();
        let b = y * 3.284_f32 - x.cos();
        let mut acc = Accumulator727::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.31_f32 + y.sin();
        let b = y * 2.549_f32 - x.cos();
        let mut acc = Accumulator727::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.907_f32 + y.sin();
        let b = y * 9.461_f32 - x.cos();
        let mut acc = Accumulator727::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.367_f32 + y.sin();
        let b = y * 1.997_f32 - x.cos();
        let mut acc = Accumulator727::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.753_f32 + y.sin();
        let b = y * 0.736_f32 - x.cos();
        let mut acc = Accumulator727::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.882_f32 + y.sin();
        let b = y * 9.557_f32 - x.cos();
        let mut acc = Accumulator727::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.989_f32 + y.sin();
        let b = y * 8.812_f32 - x.cos();
        let mut acc = Accumulator727::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.334_f32 + y.sin();
        let b = y * 0.593_f32 - x.cos();
        let mut acc = Accumulator727::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.311_f32 + y.sin();
        let b = y * 3.448_f32 - x.cos();
        let mut acc = Accumulator727::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.615_f32 + y.sin();
        let b = y * 7.508_f32 - x.cos();
        let mut acc = Accumulator727::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.986_f32 + y.sin();
        let b = y * 3.968_f32 - x.cos();
        let mut acc = Accumulator727::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.15_f32 + y.sin();
        let b = y * 4.684_f32 - x.cos();
        let mut acc = Accumulator727::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.197_f32 + y.sin();
        let b = y * 6.285_f32 - x.cos();
        let mut acc = Accumulator727::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.326_f32 + y.sin();
        let b = y * 8.221_f32 - x.cos();
        let mut acc = Accumulator727::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.5_f32 + y.sin();
        let b = y * 7.615_f32 - x.cos();
        let mut acc = Accumulator727::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.82_f32 + y.sin();
        let b = y * 9.812_f32 - x.cos();
        let mut acc = Accumulator727::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.812_f32 + y.sin();
        let b = y * 6.334_f32 - x.cos();
        let mut acc = Accumulator727::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.163_f32 + y.sin();
        let b = y * 0.177_f32 - x.cos();
        let mut acc = Accumulator727::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_727(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_727() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_727(total as u64) % 997) as f32;
        total
    }
}

pub mod m728 {
    use super::*;

    pub struct Accumulator728<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator728<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.526_f32 + y.sin();
        let b = y * 0.601_f32 - x.cos();
        let mut acc = Accumulator728::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.683_f32 + y.sin();
        let b = y * 0.337_f32 - x.cos();
        let mut acc = Accumulator728::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.166_f32 + y.sin();
        let b = y * 6.943_f32 - x.cos();
        let mut acc = Accumulator728::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.783_f32 + y.sin();
        let b = y * 2.202_f32 - x.cos();
        let mut acc = Accumulator728::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.454_f32 + y.sin();
        let b = y * 5.447_f32 - x.cos();
        let mut acc = Accumulator728::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.261_f32 + y.sin();
        let b = y * 2.178_f32 - x.cos();
        let mut acc = Accumulator728::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.021_f32 + y.sin();
        let b = y * 0.298_f32 - x.cos();
        let mut acc = Accumulator728::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.449_f32 + y.sin();
        let b = y * 8.697_f32 - x.cos();
        let mut acc = Accumulator728::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.933_f32 + y.sin();
        let b = y * 8.247_f32 - x.cos();
        let mut acc = Accumulator728::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.678_f32 + y.sin();
        let b = y * 7.384_f32 - x.cos();
        let mut acc = Accumulator728::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.228_f32 + y.sin();
        let b = y * 4.251_f32 - x.cos();
        let mut acc = Accumulator728::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.387_f32 + y.sin();
        let b = y * 9.317_f32 - x.cos();
        let mut acc = Accumulator728::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.227_f32 + y.sin();
        let b = y * 1.407_f32 - x.cos();
        let mut acc = Accumulator728::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.964_f32 + y.sin();
        let b = y * 3.036_f32 - x.cos();
        let mut acc = Accumulator728::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.386_f32 + y.sin();
        let b = y * 6.281_f32 - x.cos();
        let mut acc = Accumulator728::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.018_f32 + y.sin();
        let b = y * 4.766_f32 - x.cos();
        let mut acc = Accumulator728::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.929_f32 + y.sin();
        let b = y * 0.142_f32 - x.cos();
        let mut acc = Accumulator728::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.889_f32 + y.sin();
        let b = y * 4.181_f32 - x.cos();
        let mut acc = Accumulator728::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.345_f32 + y.sin();
        let b = y * 0.68_f32 - x.cos();
        let mut acc = Accumulator728::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.132_f32 + y.sin();
        let b = y * 7.785_f32 - x.cos();
        let mut acc = Accumulator728::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_728(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_728() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_728(total as u64) % 997) as f32;
        total
    }
}

pub mod m729 {
    use super::*;

    pub struct Accumulator729<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator729<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.318_f32 + y.sin();
        let b = y * 2.999_f32 - x.cos();
        let mut acc = Accumulator729::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.801_f32 + y.sin();
        let b = y * 3.292_f32 - x.cos();
        let mut acc = Accumulator729::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.095_f32 + y.sin();
        let b = y * 4.488_f32 - x.cos();
        let mut acc = Accumulator729::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.536_f32 + y.sin();
        let b = y * 5.485_f32 - x.cos();
        let mut acc = Accumulator729::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.388_f32 + y.sin();
        let b = y * 8.489_f32 - x.cos();
        let mut acc = Accumulator729::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.0_f32 + y.sin();
        let b = y * 2.919_f32 - x.cos();
        let mut acc = Accumulator729::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.036_f32 + y.sin();
        let b = y * 0.8_f32 - x.cos();
        let mut acc = Accumulator729::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.478_f32 + y.sin();
        let b = y * 4.029_f32 - x.cos();
        let mut acc = Accumulator729::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.695_f32 + y.sin();
        let b = y * 7.408_f32 - x.cos();
        let mut acc = Accumulator729::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.872_f32 + y.sin();
        let b = y * 5.91_f32 - x.cos();
        let mut acc = Accumulator729::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.763_f32 + y.sin();
        let b = y * 1.577_f32 - x.cos();
        let mut acc = Accumulator729::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.941_f32 + y.sin();
        let b = y * 5.007_f32 - x.cos();
        let mut acc = Accumulator729::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.235_f32 + y.sin();
        let b = y * 9.882_f32 - x.cos();
        let mut acc = Accumulator729::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.784_f32 + y.sin();
        let b = y * 0.445_f32 - x.cos();
        let mut acc = Accumulator729::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.381_f32 + y.sin();
        let b = y * 4.723_f32 - x.cos();
        let mut acc = Accumulator729::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.995_f32 + y.sin();
        let b = y * 7.79_f32 - x.cos();
        let mut acc = Accumulator729::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.235_f32 + y.sin();
        let b = y * 6.598_f32 - x.cos();
        let mut acc = Accumulator729::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.382_f32 + y.sin();
        let b = y * 8.304_f32 - x.cos();
        let mut acc = Accumulator729::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.762_f32 + y.sin();
        let b = y * 6.298_f32 - x.cos();
        let mut acc = Accumulator729::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.836_f32 + y.sin();
        let b = y * 1.218_f32 - x.cos();
        let mut acc = Accumulator729::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_729(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m729-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_729() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_729(total as u64) % 997) as f32;
        total
    }
}

pub mod m730 {
    use super::*;

    pub struct Accumulator730<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator730<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.486_f32 + y.sin();
        let b = y * 0.345_f32 - x.cos();
        let mut acc = Accumulator730::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.599_f32 + y.sin();
        let b = y * 4.192_f32 - x.cos();
        let mut acc = Accumulator730::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.36_f32 + y.sin();
        let b = y * 3.165_f32 - x.cos();
        let mut acc = Accumulator730::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.88_f32 + y.sin();
        let b = y * 6.854_f32 - x.cos();
        let mut acc = Accumulator730::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.399_f32 + y.sin();
        let b = y * 6.138_f32 - x.cos();
        let mut acc = Accumulator730::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.305_f32 + y.sin();
        let b = y * 3.843_f32 - x.cos();
        let mut acc = Accumulator730::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.034_f32 + y.sin();
        let b = y * 9.354_f32 - x.cos();
        let mut acc = Accumulator730::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.712_f32 + y.sin();
        let b = y * 0.993_f32 - x.cos();
        let mut acc = Accumulator730::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.63_f32 + y.sin();
        let b = y * 2.011_f32 - x.cos();
        let mut acc = Accumulator730::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.728_f32 + y.sin();
        let b = y * 5.017_f32 - x.cos();
        let mut acc = Accumulator730::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.305_f32 + y.sin();
        let b = y * 5.294_f32 - x.cos();
        let mut acc = Accumulator730::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.365_f32 + y.sin();
        let b = y * 5.763_f32 - x.cos();
        let mut acc = Accumulator730::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.529_f32 + y.sin();
        let b = y * 2.792_f32 - x.cos();
        let mut acc = Accumulator730::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.647_f32 + y.sin();
        let b = y * 6.699_f32 - x.cos();
        let mut acc = Accumulator730::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.743_f32 + y.sin();
        let b = y * 2.19_f32 - x.cos();
        let mut acc = Accumulator730::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.66_f32 + y.sin();
        let b = y * 8.032_f32 - x.cos();
        let mut acc = Accumulator730::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.276_f32 + y.sin();
        let b = y * 2.786_f32 - x.cos();
        let mut acc = Accumulator730::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.258_f32 + y.sin();
        let b = y * 8.115_f32 - x.cos();
        let mut acc = Accumulator730::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.115_f32 + y.sin();
        let b = y * 3.761_f32 - x.cos();
        let mut acc = Accumulator730::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.454_f32 + y.sin();
        let b = y * 2.026_f32 - x.cos();
        let mut acc = Accumulator730::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_730(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_730() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_730(total as u64) % 997) as f32;
        total
    }
}

pub mod m731 {
    use super::*;

    pub struct Accumulator731<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator731<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.391_f32 + y.sin();
        let b = y * 7.22_f32 - x.cos();
        let mut acc = Accumulator731::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.875_f32 + y.sin();
        let b = y * 6.751_f32 - x.cos();
        let mut acc = Accumulator731::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.272_f32 + y.sin();
        let b = y * 6.911_f32 - x.cos();
        let mut acc = Accumulator731::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.126_f32 + y.sin();
        let b = y * 8.028_f32 - x.cos();
        let mut acc = Accumulator731::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.706_f32 + y.sin();
        let b = y * 7.627_f32 - x.cos();
        let mut acc = Accumulator731::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.058_f32 + y.sin();
        let b = y * 0.169_f32 - x.cos();
        let mut acc = Accumulator731::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.515_f32 + y.sin();
        let b = y * 4.381_f32 - x.cos();
        let mut acc = Accumulator731::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.16_f32 + y.sin();
        let b = y * 8.221_f32 - x.cos();
        let mut acc = Accumulator731::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.547_f32 + y.sin();
        let b = y * 9.207_f32 - x.cos();
        let mut acc = Accumulator731::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.269_f32 + y.sin();
        let b = y * 0.289_f32 - x.cos();
        let mut acc = Accumulator731::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.856_f32 + y.sin();
        let b = y * 0.158_f32 - x.cos();
        let mut acc = Accumulator731::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.299_f32 + y.sin();
        let b = y * 8.977_f32 - x.cos();
        let mut acc = Accumulator731::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.425_f32 + y.sin();
        let b = y * 7.574_f32 - x.cos();
        let mut acc = Accumulator731::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.785_f32 + y.sin();
        let b = y * 1.339_f32 - x.cos();
        let mut acc = Accumulator731::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.024_f32 + y.sin();
        let b = y * 0.138_f32 - x.cos();
        let mut acc = Accumulator731::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.78_f32 + y.sin();
        let b = y * 7.086_f32 - x.cos();
        let mut acc = Accumulator731::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.295_f32 + y.sin();
        let b = y * 1.455_f32 - x.cos();
        let mut acc = Accumulator731::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.062_f32 + y.sin();
        let b = y * 3.755_f32 - x.cos();
        let mut acc = Accumulator731::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.181_f32 + y.sin();
        let b = y * 7.699_f32 - x.cos();
        let mut acc = Accumulator731::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.852_f32 + y.sin();
        let b = y * 6.337_f32 - x.cos();
        let mut acc = Accumulator731::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_731(seed: u64) -> u64 {
        let re = Regex::new(r"m731-(\d+)").unwrap();
        let hay = format!("m731-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_731() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_731(total as u64) % 997) as f32;
        total
    }
}

pub mod m732 {
    use super::*;

    pub struct Accumulator732<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator732<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.497_f32 + y.sin();
        let b = y * 4.938_f32 - x.cos();
        let mut acc = Accumulator732::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.59_f32 + y.sin();
        let b = y * 4.417_f32 - x.cos();
        let mut acc = Accumulator732::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.279_f32 + y.sin();
        let b = y * 1.534_f32 - x.cos();
        let mut acc = Accumulator732::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.356_f32 + y.sin();
        let b = y * 9.673_f32 - x.cos();
        let mut acc = Accumulator732::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.128_f32 + y.sin();
        let b = y * 1.21_f32 - x.cos();
        let mut acc = Accumulator732::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.073_f32 + y.sin();
        let b = y * 1.839_f32 - x.cos();
        let mut acc = Accumulator732::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.517_f32 + y.sin();
        let b = y * 6.636_f32 - x.cos();
        let mut acc = Accumulator732::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.373_f32 + y.sin();
        let b = y * 5.824_f32 - x.cos();
        let mut acc = Accumulator732::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.855_f32 + y.sin();
        let b = y * 5.94_f32 - x.cos();
        let mut acc = Accumulator732::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.198_f32 + y.sin();
        let b = y * 0.507_f32 - x.cos();
        let mut acc = Accumulator732::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.489_f32 + y.sin();
        let b = y * 4.514_f32 - x.cos();
        let mut acc = Accumulator732::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.285_f32 + y.sin();
        let b = y * 8.571_f32 - x.cos();
        let mut acc = Accumulator732::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.153_f32 + y.sin();
        let b = y * 7.43_f32 - x.cos();
        let mut acc = Accumulator732::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.838_f32 + y.sin();
        let b = y * 2.527_f32 - x.cos();
        let mut acc = Accumulator732::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.664_f32 + y.sin();
        let b = y * 8.49_f32 - x.cos();
        let mut acc = Accumulator732::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.588_f32 + y.sin();
        let b = y * 1.164_f32 - x.cos();
        let mut acc = Accumulator732::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.116_f32 + y.sin();
        let b = y * 0.448_f32 - x.cos();
        let mut acc = Accumulator732::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.145_f32 + y.sin();
        let b = y * 0.153_f32 - x.cos();
        let mut acc = Accumulator732::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.262_f32 + y.sin();
        let b = y * 5.597_f32 - x.cos();
        let mut acc = Accumulator732::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.236_f32 + y.sin();
        let b = y * 8.781_f32 - x.cos();
        let mut acc = Accumulator732::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_732(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_732() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_732(total as u64) % 997) as f32;
        total
    }
}

pub mod m733 {
    use super::*;

    pub struct Accumulator733<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator733<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.79_f32 + y.sin();
        let b = y * 1.306_f32 - x.cos();
        let mut acc = Accumulator733::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.018_f32 + y.sin();
        let b = y * 9.005_f32 - x.cos();
        let mut acc = Accumulator733::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.752_f32 + y.sin();
        let b = y * 4.603_f32 - x.cos();
        let mut acc = Accumulator733::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.757_f32 + y.sin();
        let b = y * 4.532_f32 - x.cos();
        let mut acc = Accumulator733::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.62_f32 + y.sin();
        let b = y * 9.126_f32 - x.cos();
        let mut acc = Accumulator733::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.289_f32 + y.sin();
        let b = y * 6.364_f32 - x.cos();
        let mut acc = Accumulator733::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.488_f32 + y.sin();
        let b = y * 4.82_f32 - x.cos();
        let mut acc = Accumulator733::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.127_f32 + y.sin();
        let b = y * 8.279_f32 - x.cos();
        let mut acc = Accumulator733::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.484_f32 + y.sin();
        let b = y * 1.756_f32 - x.cos();
        let mut acc = Accumulator733::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.134_f32 + y.sin();
        let b = y * 6.858_f32 - x.cos();
        let mut acc = Accumulator733::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.802_f32 + y.sin();
        let b = y * 1.872_f32 - x.cos();
        let mut acc = Accumulator733::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.886_f32 + y.sin();
        let b = y * 6.268_f32 - x.cos();
        let mut acc = Accumulator733::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.976_f32 + y.sin();
        let b = y * 2.269_f32 - x.cos();
        let mut acc = Accumulator733::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.758_f32 + y.sin();
        let b = y * 5.725_f32 - x.cos();
        let mut acc = Accumulator733::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.785_f32 + y.sin();
        let b = y * 2.033_f32 - x.cos();
        let mut acc = Accumulator733::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.306_f32 + y.sin();
        let b = y * 7.358_f32 - x.cos();
        let mut acc = Accumulator733::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.606_f32 + y.sin();
        let b = y * 5.265_f32 - x.cos();
        let mut acc = Accumulator733::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.31_f32 + y.sin();
        let b = y * 6.229_f32 - x.cos();
        let mut acc = Accumulator733::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.558_f32 + y.sin();
        let b = y * 5.442_f32 - x.cos();
        let mut acc = Accumulator733::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.824_f32 + y.sin();
        let b = y * 5.516_f32 - x.cos();
        let mut acc = Accumulator733::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_733(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(733u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_733() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_733(total as u64) % 997) as f32;
        total
    }
}

pub mod m734 {
    use super::*;

    pub struct Accumulator734<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator734<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.946_f32 + y.sin();
        let b = y * 8.326_f32 - x.cos();
        let mut acc = Accumulator734::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.244_f32 + y.sin();
        let b = y * 7.436_f32 - x.cos();
        let mut acc = Accumulator734::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.508_f32 + y.sin();
        let b = y * 7.325_f32 - x.cos();
        let mut acc = Accumulator734::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.188_f32 + y.sin();
        let b = y * 3.532_f32 - x.cos();
        let mut acc = Accumulator734::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.431_f32 + y.sin();
        let b = y * 2.513_f32 - x.cos();
        let mut acc = Accumulator734::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.242_f32 + y.sin();
        let b = y * 1.132_f32 - x.cos();
        let mut acc = Accumulator734::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.973_f32 + y.sin();
        let b = y * 3.414_f32 - x.cos();
        let mut acc = Accumulator734::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.852_f32 + y.sin();
        let b = y * 9.661_f32 - x.cos();
        let mut acc = Accumulator734::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.157_f32 + y.sin();
        let b = y * 8.382_f32 - x.cos();
        let mut acc = Accumulator734::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.244_f32 + y.sin();
        let b = y * 8.751_f32 - x.cos();
        let mut acc = Accumulator734::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.005_f32 + y.sin();
        let b = y * 1.694_f32 - x.cos();
        let mut acc = Accumulator734::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.025_f32 + y.sin();
        let b = y * 6.718_f32 - x.cos();
        let mut acc = Accumulator734::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.46_f32 + y.sin();
        let b = y * 1.785_f32 - x.cos();
        let mut acc = Accumulator734::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.234_f32 + y.sin();
        let b = y * 6.383_f32 - x.cos();
        let mut acc = Accumulator734::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.082_f32 + y.sin();
        let b = y * 7.786_f32 - x.cos();
        let mut acc = Accumulator734::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.525_f32 + y.sin();
        let b = y * 4.988_f32 - x.cos();
        let mut acc = Accumulator734::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.305_f32 + y.sin();
        let b = y * 2.188_f32 - x.cos();
        let mut acc = Accumulator734::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.704_f32 + y.sin();
        let b = y * 9.699_f32 - x.cos();
        let mut acc = Accumulator734::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.139_f32 + y.sin();
        let b = y * 8.938_f32 - x.cos();
        let mut acc = Accumulator734::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.325_f32 + y.sin();
        let b = y * 9.798_f32 - x.cos();
        let mut acc = Accumulator734::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_734(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_734() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_734(total as u64) % 997) as f32;
        total
    }
}

pub mod m735 {
    use super::*;

    pub struct Accumulator735<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator735<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.686_f32 + y.sin();
        let b = y * 4.856_f32 - x.cos();
        let mut acc = Accumulator735::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.003_f32 + y.sin();
        let b = y * 6.143_f32 - x.cos();
        let mut acc = Accumulator735::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.554_f32 + y.sin();
        let b = y * 5.894_f32 - x.cos();
        let mut acc = Accumulator735::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.448_f32 + y.sin();
        let b = y * 4.191_f32 - x.cos();
        let mut acc = Accumulator735::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.516_f32 + y.sin();
        let b = y * 1.911_f32 - x.cos();
        let mut acc = Accumulator735::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.816_f32 + y.sin();
        let b = y * 3.617_f32 - x.cos();
        let mut acc = Accumulator735::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.085_f32 + y.sin();
        let b = y * 7.685_f32 - x.cos();
        let mut acc = Accumulator735::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.953_f32 + y.sin();
        let b = y * 9.3_f32 - x.cos();
        let mut acc = Accumulator735::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.671_f32 + y.sin();
        let b = y * 3.876_f32 - x.cos();
        let mut acc = Accumulator735::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.687_f32 + y.sin();
        let b = y * 8.789_f32 - x.cos();
        let mut acc = Accumulator735::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.973_f32 + y.sin();
        let b = y * 6.217_f32 - x.cos();
        let mut acc = Accumulator735::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.588_f32 + y.sin();
        let b = y * 3.352_f32 - x.cos();
        let mut acc = Accumulator735::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.741_f32 + y.sin();
        let b = y * 9.149_f32 - x.cos();
        let mut acc = Accumulator735::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.245_f32 + y.sin();
        let b = y * 9.411_f32 - x.cos();
        let mut acc = Accumulator735::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.291_f32 + y.sin();
        let b = y * 3.546_f32 - x.cos();
        let mut acc = Accumulator735::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.104_f32 + y.sin();
        let b = y * 2.088_f32 - x.cos();
        let mut acc = Accumulator735::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.864_f32 + y.sin();
        let b = y * 2.387_f32 - x.cos();
        let mut acc = Accumulator735::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.796_f32 + y.sin();
        let b = y * 6.89_f32 - x.cos();
        let mut acc = Accumulator735::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.932_f32 + y.sin();
        let b = y * 0.707_f32 - x.cos();
        let mut acc = Accumulator735::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.674_f32 + y.sin();
        let b = y * 0.912_f32 - x.cos();
        let mut acc = Accumulator735::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_735(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_735() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_735(total as u64) % 997) as f32;
        total
    }
}

pub mod m736 {
    use super::*;

    pub struct Accumulator736<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator736<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.986_f32 + y.sin();
        let b = y * 4.25_f32 - x.cos();
        let mut acc = Accumulator736::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.538_f32 + y.sin();
        let b = y * 3.662_f32 - x.cos();
        let mut acc = Accumulator736::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.662_f32 + y.sin();
        let b = y * 4.976_f32 - x.cos();
        let mut acc = Accumulator736::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.541_f32 + y.sin();
        let b = y * 1.435_f32 - x.cos();
        let mut acc = Accumulator736::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.892_f32 + y.sin();
        let b = y * 1.184_f32 - x.cos();
        let mut acc = Accumulator736::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.502_f32 + y.sin();
        let b = y * 6.928_f32 - x.cos();
        let mut acc = Accumulator736::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.747_f32 + y.sin();
        let b = y * 0.964_f32 - x.cos();
        let mut acc = Accumulator736::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.68_f32 + y.sin();
        let b = y * 5.9_f32 - x.cos();
        let mut acc = Accumulator736::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.293_f32 + y.sin();
        let b = y * 7.509_f32 - x.cos();
        let mut acc = Accumulator736::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.92_f32 + y.sin();
        let b = y * 0.911_f32 - x.cos();
        let mut acc = Accumulator736::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.48_f32 + y.sin();
        let b = y * 9.208_f32 - x.cos();
        let mut acc = Accumulator736::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.327_f32 + y.sin();
        let b = y * 5.781_f32 - x.cos();
        let mut acc = Accumulator736::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.277_f32 + y.sin();
        let b = y * 4.265_f32 - x.cos();
        let mut acc = Accumulator736::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.236_f32 + y.sin();
        let b = y * 0.879_f32 - x.cos();
        let mut acc = Accumulator736::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.3_f32 + y.sin();
        let b = y * 2.051_f32 - x.cos();
        let mut acc = Accumulator736::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.068_f32 + y.sin();
        let b = y * 8.111_f32 - x.cos();
        let mut acc = Accumulator736::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.453_f32 + y.sin();
        let b = y * 2.907_f32 - x.cos();
        let mut acc = Accumulator736::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.984_f32 + y.sin();
        let b = y * 9.024_f32 - x.cos();
        let mut acc = Accumulator736::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.828_f32 + y.sin();
        let b = y * 4.319_f32 - x.cos();
        let mut acc = Accumulator736::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.793_f32 + y.sin();
        let b = y * 4.887_f32 - x.cos();
        let mut acc = Accumulator736::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_736(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m736-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_736() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_736(total as u64) % 997) as f32;
        total
    }
}

pub mod m737 {
    use super::*;

    pub struct Accumulator737<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator737<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.863_f32 + y.sin();
        let b = y * 6.129_f32 - x.cos();
        let mut acc = Accumulator737::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.206_f32 + y.sin();
        let b = y * 6.049_f32 - x.cos();
        let mut acc = Accumulator737::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.387_f32 + y.sin();
        let b = y * 3.77_f32 - x.cos();
        let mut acc = Accumulator737::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.808_f32 + y.sin();
        let b = y * 9.253_f32 - x.cos();
        let mut acc = Accumulator737::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.907_f32 + y.sin();
        let b = y * 9.757_f32 - x.cos();
        let mut acc = Accumulator737::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.504_f32 + y.sin();
        let b = y * 2.098_f32 - x.cos();
        let mut acc = Accumulator737::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.023_f32 + y.sin();
        let b = y * 6.731_f32 - x.cos();
        let mut acc = Accumulator737::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.448_f32 + y.sin();
        let b = y * 7.914_f32 - x.cos();
        let mut acc = Accumulator737::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.422_f32 + y.sin();
        let b = y * 3.039_f32 - x.cos();
        let mut acc = Accumulator737::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.924_f32 + y.sin();
        let b = y * 7.838_f32 - x.cos();
        let mut acc = Accumulator737::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.103_f32 + y.sin();
        let b = y * 9.887_f32 - x.cos();
        let mut acc = Accumulator737::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.016_f32 + y.sin();
        let b = y * 3.329_f32 - x.cos();
        let mut acc = Accumulator737::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.103_f32 + y.sin();
        let b = y * 7.247_f32 - x.cos();
        let mut acc = Accumulator737::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.418_f32 + y.sin();
        let b = y * 2.929_f32 - x.cos();
        let mut acc = Accumulator737::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.308_f32 + y.sin();
        let b = y * 1.684_f32 - x.cos();
        let mut acc = Accumulator737::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.568_f32 + y.sin();
        let b = y * 4.471_f32 - x.cos();
        let mut acc = Accumulator737::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.869_f32 + y.sin();
        let b = y * 9.527_f32 - x.cos();
        let mut acc = Accumulator737::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.597_f32 + y.sin();
        let b = y * 4.96_f32 - x.cos();
        let mut acc = Accumulator737::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.545_f32 + y.sin();
        let b = y * 5.629_f32 - x.cos();
        let mut acc = Accumulator737::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.879_f32 + y.sin();
        let b = y * 7.936_f32 - x.cos();
        let mut acc = Accumulator737::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_737(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_737() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_737(total as u64) % 997) as f32;
        total
    }
}

pub mod m738 {
    use super::*;

    pub struct Accumulator738<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator738<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.697_f32 + y.sin();
        let b = y * 8.702_f32 - x.cos();
        let mut acc = Accumulator738::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.295_f32 + y.sin();
        let b = y * 2.621_f32 - x.cos();
        let mut acc = Accumulator738::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.103_f32 + y.sin();
        let b = y * 4.801_f32 - x.cos();
        let mut acc = Accumulator738::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.17_f32 + y.sin();
        let b = y * 9.259_f32 - x.cos();
        let mut acc = Accumulator738::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.462_f32 + y.sin();
        let b = y * 9.043_f32 - x.cos();
        let mut acc = Accumulator738::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.129_f32 + y.sin();
        let b = y * 1.004_f32 - x.cos();
        let mut acc = Accumulator738::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.974_f32 + y.sin();
        let b = y * 4.033_f32 - x.cos();
        let mut acc = Accumulator738::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.68_f32 + y.sin();
        let b = y * 6.094_f32 - x.cos();
        let mut acc = Accumulator738::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.225_f32 + y.sin();
        let b = y * 7.0_f32 - x.cos();
        let mut acc = Accumulator738::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.769_f32 + y.sin();
        let b = y * 6.8_f32 - x.cos();
        let mut acc = Accumulator738::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.984_f32 + y.sin();
        let b = y * 5.806_f32 - x.cos();
        let mut acc = Accumulator738::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.675_f32 + y.sin();
        let b = y * 2.369_f32 - x.cos();
        let mut acc = Accumulator738::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.262_f32 + y.sin();
        let b = y * 3.468_f32 - x.cos();
        let mut acc = Accumulator738::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.289_f32 + y.sin();
        let b = y * 7.608_f32 - x.cos();
        let mut acc = Accumulator738::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.746_f32 + y.sin();
        let b = y * 6.785_f32 - x.cos();
        let mut acc = Accumulator738::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.733_f32 + y.sin();
        let b = y * 4.231_f32 - x.cos();
        let mut acc = Accumulator738::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.599_f32 + y.sin();
        let b = y * 0.899_f32 - x.cos();
        let mut acc = Accumulator738::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.775_f32 + y.sin();
        let b = y * 0.24_f32 - x.cos();
        let mut acc = Accumulator738::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.682_f32 + y.sin();
        let b = y * 7.265_f32 - x.cos();
        let mut acc = Accumulator738::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.846_f32 + y.sin();
        let b = y * 2.067_f32 - x.cos();
        let mut acc = Accumulator738::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_738(seed: u64) -> u64 {
        let re = Regex::new(r"m738-(\d+)").unwrap();
        let hay = format!("m738-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_738() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_738(total as u64) % 997) as f32;
        total
    }
}

pub mod m739 {
    use super::*;

    pub struct Accumulator739<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator739<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.896_f32 + y.sin();
        let b = y * 6.897_f32 - x.cos();
        let mut acc = Accumulator739::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.259_f32 + y.sin();
        let b = y * 3.711_f32 - x.cos();
        let mut acc = Accumulator739::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.396_f32 + y.sin();
        let b = y * 4.806_f32 - x.cos();
        let mut acc = Accumulator739::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.846_f32 + y.sin();
        let b = y * 0.937_f32 - x.cos();
        let mut acc = Accumulator739::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.977_f32 + y.sin();
        let b = y * 1.593_f32 - x.cos();
        let mut acc = Accumulator739::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.619_f32 + y.sin();
        let b = y * 9.892_f32 - x.cos();
        let mut acc = Accumulator739::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.122_f32 + y.sin();
        let b = y * 9.772_f32 - x.cos();
        let mut acc = Accumulator739::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.544_f32 + y.sin();
        let b = y * 6.319_f32 - x.cos();
        let mut acc = Accumulator739::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.097_f32 + y.sin();
        let b = y * 3.321_f32 - x.cos();
        let mut acc = Accumulator739::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.854_f32 + y.sin();
        let b = y * 5.528_f32 - x.cos();
        let mut acc = Accumulator739::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.874_f32 + y.sin();
        let b = y * 4.925_f32 - x.cos();
        let mut acc = Accumulator739::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.881_f32 + y.sin();
        let b = y * 4.559_f32 - x.cos();
        let mut acc = Accumulator739::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.158_f32 + y.sin();
        let b = y * 7.914_f32 - x.cos();
        let mut acc = Accumulator739::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.854_f32 + y.sin();
        let b = y * 8.463_f32 - x.cos();
        let mut acc = Accumulator739::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.306_f32 + y.sin();
        let b = y * 6.987_f32 - x.cos();
        let mut acc = Accumulator739::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.769_f32 + y.sin();
        let b = y * 8.986_f32 - x.cos();
        let mut acc = Accumulator739::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.975_f32 + y.sin();
        let b = y * 8.049_f32 - x.cos();
        let mut acc = Accumulator739::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.209_f32 + y.sin();
        let b = y * 9.795_f32 - x.cos();
        let mut acc = Accumulator739::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.057_f32 + y.sin();
        let b = y * 0.946_f32 - x.cos();
        let mut acc = Accumulator739::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.266_f32 + y.sin();
        let b = y * 9.257_f32 - x.cos();
        let mut acc = Accumulator739::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_739(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_739() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_739(total as u64) % 997) as f32;
        total
    }
}

pub mod m740 {
    use super::*;

    pub struct Accumulator740<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator740<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.711_f32 + y.sin();
        let b = y * 4.164_f32 - x.cos();
        let mut acc = Accumulator740::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.031_f32 + y.sin();
        let b = y * 8.54_f32 - x.cos();
        let mut acc = Accumulator740::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.8_f32 + y.sin();
        let b = y * 3.625_f32 - x.cos();
        let mut acc = Accumulator740::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.057_f32 + y.sin();
        let b = y * 6.972_f32 - x.cos();
        let mut acc = Accumulator740::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.656_f32 + y.sin();
        let b = y * 9.594_f32 - x.cos();
        let mut acc = Accumulator740::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.122_f32 + y.sin();
        let b = y * 1.539_f32 - x.cos();
        let mut acc = Accumulator740::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.393_f32 + y.sin();
        let b = y * 5.793_f32 - x.cos();
        let mut acc = Accumulator740::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.269_f32 + y.sin();
        let b = y * 6.334_f32 - x.cos();
        let mut acc = Accumulator740::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.072_f32 + y.sin();
        let b = y * 6.252_f32 - x.cos();
        let mut acc = Accumulator740::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.104_f32 + y.sin();
        let b = y * 9.837_f32 - x.cos();
        let mut acc = Accumulator740::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.45_f32 + y.sin();
        let b = y * 2.832_f32 - x.cos();
        let mut acc = Accumulator740::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.366_f32 + y.sin();
        let b = y * 2.161_f32 - x.cos();
        let mut acc = Accumulator740::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.125_f32 + y.sin();
        let b = y * 2.313_f32 - x.cos();
        let mut acc = Accumulator740::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.644_f32 + y.sin();
        let b = y * 0.25_f32 - x.cos();
        let mut acc = Accumulator740::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.083_f32 + y.sin();
        let b = y * 4.845_f32 - x.cos();
        let mut acc = Accumulator740::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.808_f32 + y.sin();
        let b = y * 7.613_f32 - x.cos();
        let mut acc = Accumulator740::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.113_f32 + y.sin();
        let b = y * 0.25_f32 - x.cos();
        let mut acc = Accumulator740::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.925_f32 + y.sin();
        let b = y * 3.874_f32 - x.cos();
        let mut acc = Accumulator740::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.271_f32 + y.sin();
        let b = y * 2.128_f32 - x.cos();
        let mut acc = Accumulator740::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.464_f32 + y.sin();
        let b = y * 4.218_f32 - x.cos();
        let mut acc = Accumulator740::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_740(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(740u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_740() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_740(total as u64) % 997) as f32;
        total
    }
}

pub mod m741 {
    use super::*;

    pub struct Accumulator741<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator741<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.112_f32 + y.sin();
        let b = y * 4.605_f32 - x.cos();
        let mut acc = Accumulator741::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.902_f32 + y.sin();
        let b = y * 8.25_f32 - x.cos();
        let mut acc = Accumulator741::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.67_f32 + y.sin();
        let b = y * 7.013_f32 - x.cos();
        let mut acc = Accumulator741::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.985_f32 + y.sin();
        let b = y * 2.899_f32 - x.cos();
        let mut acc = Accumulator741::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.835_f32 + y.sin();
        let b = y * 1.311_f32 - x.cos();
        let mut acc = Accumulator741::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.636_f32 + y.sin();
        let b = y * 0.16_f32 - x.cos();
        let mut acc = Accumulator741::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.291_f32 + y.sin();
        let b = y * 6.557_f32 - x.cos();
        let mut acc = Accumulator741::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.795_f32 + y.sin();
        let b = y * 1.422_f32 - x.cos();
        let mut acc = Accumulator741::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.467_f32 + y.sin();
        let b = y * 8.073_f32 - x.cos();
        let mut acc = Accumulator741::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.212_f32 + y.sin();
        let b = y * 8.634_f32 - x.cos();
        let mut acc = Accumulator741::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.772_f32 + y.sin();
        let b = y * 0.676_f32 - x.cos();
        let mut acc = Accumulator741::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.767_f32 + y.sin();
        let b = y * 8.819_f32 - x.cos();
        let mut acc = Accumulator741::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.879_f32 + y.sin();
        let b = y * 7.401_f32 - x.cos();
        let mut acc = Accumulator741::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.24_f32 + y.sin();
        let b = y * 2.365_f32 - x.cos();
        let mut acc = Accumulator741::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.858_f32 + y.sin();
        let b = y * 4.263_f32 - x.cos();
        let mut acc = Accumulator741::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.981_f32 + y.sin();
        let b = y * 8.467_f32 - x.cos();
        let mut acc = Accumulator741::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.399_f32 + y.sin();
        let b = y * 4.0_f32 - x.cos();
        let mut acc = Accumulator741::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.415_f32 + y.sin();
        let b = y * 7.326_f32 - x.cos();
        let mut acc = Accumulator741::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.663_f32 + y.sin();
        let b = y * 3.873_f32 - x.cos();
        let mut acc = Accumulator741::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.561_f32 + y.sin();
        let b = y * 4.448_f32 - x.cos();
        let mut acc = Accumulator741::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_741(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_741() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_741(total as u64) % 997) as f32;
        total
    }
}

pub mod m742 {
    use super::*;

    pub struct Accumulator742<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator742<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.165_f32 + y.sin();
        let b = y * 5.337_f32 - x.cos();
        let mut acc = Accumulator742::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.151_f32 + y.sin();
        let b = y * 2.64_f32 - x.cos();
        let mut acc = Accumulator742::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.57_f32 + y.sin();
        let b = y * 2.631_f32 - x.cos();
        let mut acc = Accumulator742::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.647_f32 + y.sin();
        let b = y * 9.091_f32 - x.cos();
        let mut acc = Accumulator742::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.753_f32 + y.sin();
        let b = y * 0.124_f32 - x.cos();
        let mut acc = Accumulator742::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.258_f32 + y.sin();
        let b = y * 8.119_f32 - x.cos();
        let mut acc = Accumulator742::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.442_f32 + y.sin();
        let b = y * 5.49_f32 - x.cos();
        let mut acc = Accumulator742::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.972_f32 + y.sin();
        let b = y * 3.166_f32 - x.cos();
        let mut acc = Accumulator742::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.031_f32 + y.sin();
        let b = y * 4.689_f32 - x.cos();
        let mut acc = Accumulator742::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.273_f32 + y.sin();
        let b = y * 8.402_f32 - x.cos();
        let mut acc = Accumulator742::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.453_f32 + y.sin();
        let b = y * 2.01_f32 - x.cos();
        let mut acc = Accumulator742::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.242_f32 + y.sin();
        let b = y * 2.781_f32 - x.cos();
        let mut acc = Accumulator742::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.31_f32 + y.sin();
        let b = y * 8.14_f32 - x.cos();
        let mut acc = Accumulator742::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.331_f32 + y.sin();
        let b = y * 3.981_f32 - x.cos();
        let mut acc = Accumulator742::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.501_f32 + y.sin();
        let b = y * 5.924_f32 - x.cos();
        let mut acc = Accumulator742::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.072_f32 + y.sin();
        let b = y * 7.661_f32 - x.cos();
        let mut acc = Accumulator742::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.797_f32 + y.sin();
        let b = y * 8.442_f32 - x.cos();
        let mut acc = Accumulator742::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.722_f32 + y.sin();
        let b = y * 9.355_f32 - x.cos();
        let mut acc = Accumulator742::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.586_f32 + y.sin();
        let b = y * 3.636_f32 - x.cos();
        let mut acc = Accumulator742::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.34_f32 + y.sin();
        let b = y * 0.787_f32 - x.cos();
        let mut acc = Accumulator742::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_742(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_742() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_742(total as u64) % 997) as f32;
        total
    }
}

pub mod m743 {
    use super::*;

    pub struct Accumulator743<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator743<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.084_f32 + y.sin();
        let b = y * 8.163_f32 - x.cos();
        let mut acc = Accumulator743::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.765_f32 + y.sin();
        let b = y * 9.815_f32 - x.cos();
        let mut acc = Accumulator743::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.952_f32 + y.sin();
        let b = y * 6.908_f32 - x.cos();
        let mut acc = Accumulator743::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.585_f32 + y.sin();
        let b = y * 3.284_f32 - x.cos();
        let mut acc = Accumulator743::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.239_f32 + y.sin();
        let b = y * 8.034_f32 - x.cos();
        let mut acc = Accumulator743::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.152_f32 + y.sin();
        let b = y * 4.255_f32 - x.cos();
        let mut acc = Accumulator743::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.729_f32 + y.sin();
        let b = y * 8.373_f32 - x.cos();
        let mut acc = Accumulator743::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.575_f32 + y.sin();
        let b = y * 8.943_f32 - x.cos();
        let mut acc = Accumulator743::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.459_f32 + y.sin();
        let b = y * 2.243_f32 - x.cos();
        let mut acc = Accumulator743::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.658_f32 + y.sin();
        let b = y * 2.36_f32 - x.cos();
        let mut acc = Accumulator743::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.129_f32 + y.sin();
        let b = y * 0.785_f32 - x.cos();
        let mut acc = Accumulator743::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.775_f32 + y.sin();
        let b = y * 5.911_f32 - x.cos();
        let mut acc = Accumulator743::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.786_f32 + y.sin();
        let b = y * 4.87_f32 - x.cos();
        let mut acc = Accumulator743::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.908_f32 + y.sin();
        let b = y * 9.593_f32 - x.cos();
        let mut acc = Accumulator743::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.893_f32 + y.sin();
        let b = y * 3.667_f32 - x.cos();
        let mut acc = Accumulator743::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.363_f32 + y.sin();
        let b = y * 7.383_f32 - x.cos();
        let mut acc = Accumulator743::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.465_f32 + y.sin();
        let b = y * 9.614_f32 - x.cos();
        let mut acc = Accumulator743::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.31_f32 + y.sin();
        let b = y * 2.422_f32 - x.cos();
        let mut acc = Accumulator743::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.297_f32 + y.sin();
        let b = y * 2.364_f32 - x.cos();
        let mut acc = Accumulator743::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.636_f32 + y.sin();
        let b = y * 5.979_f32 - x.cos();
        let mut acc = Accumulator743::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_743(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m743-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_743() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_743(total as u64) % 997) as f32;
        total
    }
}

pub mod m744 {
    use super::*;

    pub struct Accumulator744<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator744<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.334_f32 + y.sin();
        let b = y * 6.724_f32 - x.cos();
        let mut acc = Accumulator744::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.145_f32 + y.sin();
        let b = y * 2.338_f32 - x.cos();
        let mut acc = Accumulator744::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.304_f32 + y.sin();
        let b = y * 3.987_f32 - x.cos();
        let mut acc = Accumulator744::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.423_f32 + y.sin();
        let b = y * 4.112_f32 - x.cos();
        let mut acc = Accumulator744::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.84_f32 + y.sin();
        let b = y * 0.715_f32 - x.cos();
        let mut acc = Accumulator744::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.586_f32 + y.sin();
        let b = y * 2.541_f32 - x.cos();
        let mut acc = Accumulator744::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.236_f32 + y.sin();
        let b = y * 4.403_f32 - x.cos();
        let mut acc = Accumulator744::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.063_f32 + y.sin();
        let b = y * 6.735_f32 - x.cos();
        let mut acc = Accumulator744::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.757_f32 + y.sin();
        let b = y * 5.228_f32 - x.cos();
        let mut acc = Accumulator744::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.407_f32 + y.sin();
        let b = y * 8.615_f32 - x.cos();
        let mut acc = Accumulator744::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.536_f32 + y.sin();
        let b = y * 3.608_f32 - x.cos();
        let mut acc = Accumulator744::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.101_f32 + y.sin();
        let b = y * 5.932_f32 - x.cos();
        let mut acc = Accumulator744::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.436_f32 + y.sin();
        let b = y * 9.094_f32 - x.cos();
        let mut acc = Accumulator744::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.657_f32 + y.sin();
        let b = y * 9.823_f32 - x.cos();
        let mut acc = Accumulator744::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.668_f32 + y.sin();
        let b = y * 9.802_f32 - x.cos();
        let mut acc = Accumulator744::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.292_f32 + y.sin();
        let b = y * 3.237_f32 - x.cos();
        let mut acc = Accumulator744::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.479_f32 + y.sin();
        let b = y * 8.021_f32 - x.cos();
        let mut acc = Accumulator744::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.541_f32 + y.sin();
        let b = y * 2.652_f32 - x.cos();
        let mut acc = Accumulator744::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.47_f32 + y.sin();
        let b = y * 5.095_f32 - x.cos();
        let mut acc = Accumulator744::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.375_f32 + y.sin();
        let b = y * 0.578_f32 - x.cos();
        let mut acc = Accumulator744::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_744(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_744() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_744(total as u64) % 997) as f32;
        total
    }
}

pub mod m745 {
    use super::*;

    pub struct Accumulator745<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator745<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.165_f32 + y.sin();
        let b = y * 7.812_f32 - x.cos();
        let mut acc = Accumulator745::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.965_f32 + y.sin();
        let b = y * 7.231_f32 - x.cos();
        let mut acc = Accumulator745::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.399_f32 + y.sin();
        let b = y * 8.706_f32 - x.cos();
        let mut acc = Accumulator745::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.287_f32 + y.sin();
        let b = y * 2.839_f32 - x.cos();
        let mut acc = Accumulator745::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.249_f32 + y.sin();
        let b = y * 0.777_f32 - x.cos();
        let mut acc = Accumulator745::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.684_f32 + y.sin();
        let b = y * 1.204_f32 - x.cos();
        let mut acc = Accumulator745::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.282_f32 + y.sin();
        let b = y * 0.593_f32 - x.cos();
        let mut acc = Accumulator745::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.366_f32 + y.sin();
        let b = y * 6.384_f32 - x.cos();
        let mut acc = Accumulator745::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.08_f32 + y.sin();
        let b = y * 3.231_f32 - x.cos();
        let mut acc = Accumulator745::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.829_f32 + y.sin();
        let b = y * 6.711_f32 - x.cos();
        let mut acc = Accumulator745::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.919_f32 + y.sin();
        let b = y * 9.866_f32 - x.cos();
        let mut acc = Accumulator745::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.601_f32 + y.sin();
        let b = y * 3.9_f32 - x.cos();
        let mut acc = Accumulator745::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.679_f32 + y.sin();
        let b = y * 8.228_f32 - x.cos();
        let mut acc = Accumulator745::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.513_f32 + y.sin();
        let b = y * 0.601_f32 - x.cos();
        let mut acc = Accumulator745::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.578_f32 + y.sin();
        let b = y * 0.229_f32 - x.cos();
        let mut acc = Accumulator745::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.206_f32 + y.sin();
        let b = y * 9.531_f32 - x.cos();
        let mut acc = Accumulator745::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.482_f32 + y.sin();
        let b = y * 9.08_f32 - x.cos();
        let mut acc = Accumulator745::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.623_f32 + y.sin();
        let b = y * 2.785_f32 - x.cos();
        let mut acc = Accumulator745::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.187_f32 + y.sin();
        let b = y * 7.643_f32 - x.cos();
        let mut acc = Accumulator745::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.706_f32 + y.sin();
        let b = y * 0.228_f32 - x.cos();
        let mut acc = Accumulator745::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_745(seed: u64) -> u64 {
        let re = Regex::new(r"m745-(\d+)").unwrap();
        let hay = format!("m745-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_745() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_745(total as u64) % 997) as f32;
        total
    }
}

pub mod m746 {
    use super::*;

    pub struct Accumulator746<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator746<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.387_f32 + y.sin();
        let b = y * 4.824_f32 - x.cos();
        let mut acc = Accumulator746::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.187_f32 + y.sin();
        let b = y * 7.19_f32 - x.cos();
        let mut acc = Accumulator746::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.213_f32 + y.sin();
        let b = y * 0.626_f32 - x.cos();
        let mut acc = Accumulator746::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.067_f32 + y.sin();
        let b = y * 6.617_f32 - x.cos();
        let mut acc = Accumulator746::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.333_f32 + y.sin();
        let b = y * 4.32_f32 - x.cos();
        let mut acc = Accumulator746::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.036_f32 + y.sin();
        let b = y * 5.153_f32 - x.cos();
        let mut acc = Accumulator746::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.214_f32 + y.sin();
        let b = y * 0.903_f32 - x.cos();
        let mut acc = Accumulator746::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.819_f32 + y.sin();
        let b = y * 6.623_f32 - x.cos();
        let mut acc = Accumulator746::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.58_f32 + y.sin();
        let b = y * 5.352_f32 - x.cos();
        let mut acc = Accumulator746::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.821_f32 + y.sin();
        let b = y * 8.799_f32 - x.cos();
        let mut acc = Accumulator746::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.251_f32 + y.sin();
        let b = y * 0.577_f32 - x.cos();
        let mut acc = Accumulator746::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.24_f32 + y.sin();
        let b = y * 0.451_f32 - x.cos();
        let mut acc = Accumulator746::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.685_f32 + y.sin();
        let b = y * 9.411_f32 - x.cos();
        let mut acc = Accumulator746::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.466_f32 + y.sin();
        let b = y * 4.219_f32 - x.cos();
        let mut acc = Accumulator746::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.783_f32 + y.sin();
        let b = y * 0.991_f32 - x.cos();
        let mut acc = Accumulator746::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.565_f32 + y.sin();
        let b = y * 9.704_f32 - x.cos();
        let mut acc = Accumulator746::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.061_f32 + y.sin();
        let b = y * 7.587_f32 - x.cos();
        let mut acc = Accumulator746::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.921_f32 + y.sin();
        let b = y * 4.676_f32 - x.cos();
        let mut acc = Accumulator746::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.305_f32 + y.sin();
        let b = y * 8.674_f32 - x.cos();
        let mut acc = Accumulator746::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.357_f32 + y.sin();
        let b = y * 0.998_f32 - x.cos();
        let mut acc = Accumulator746::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_746(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_746() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_746(total as u64) % 997) as f32;
        total
    }
}

pub mod m747 {
    use super::*;

    pub struct Accumulator747<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator747<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.817_f32 + y.sin();
        let b = y * 4.218_f32 - x.cos();
        let mut acc = Accumulator747::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.087_f32 + y.sin();
        let b = y * 9.699_f32 - x.cos();
        let mut acc = Accumulator747::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.793_f32 + y.sin();
        let b = y * 6.537_f32 - x.cos();
        let mut acc = Accumulator747::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.584_f32 + y.sin();
        let b = y * 7.795_f32 - x.cos();
        let mut acc = Accumulator747::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.629_f32 + y.sin();
        let b = y * 2.57_f32 - x.cos();
        let mut acc = Accumulator747::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.12_f32 + y.sin();
        let b = y * 9.347_f32 - x.cos();
        let mut acc = Accumulator747::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.297_f32 + y.sin();
        let b = y * 1.291_f32 - x.cos();
        let mut acc = Accumulator747::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.601_f32 + y.sin();
        let b = y * 2.106_f32 - x.cos();
        let mut acc = Accumulator747::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.261_f32 + y.sin();
        let b = y * 7.381_f32 - x.cos();
        let mut acc = Accumulator747::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.713_f32 + y.sin();
        let b = y * 7.298_f32 - x.cos();
        let mut acc = Accumulator747::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.24_f32 + y.sin();
        let b = y * 8.668_f32 - x.cos();
        let mut acc = Accumulator747::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.703_f32 + y.sin();
        let b = y * 5.878_f32 - x.cos();
        let mut acc = Accumulator747::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.137_f32 + y.sin();
        let b = y * 1.558_f32 - x.cos();
        let mut acc = Accumulator747::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.83_f32 + y.sin();
        let b = y * 4.903_f32 - x.cos();
        let mut acc = Accumulator747::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.506_f32 + y.sin();
        let b = y * 4.698_f32 - x.cos();
        let mut acc = Accumulator747::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.415_f32 + y.sin();
        let b = y * 7.316_f32 - x.cos();
        let mut acc = Accumulator747::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.005_f32 + y.sin();
        let b = y * 3.568_f32 - x.cos();
        let mut acc = Accumulator747::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.579_f32 + y.sin();
        let b = y * 0.161_f32 - x.cos();
        let mut acc = Accumulator747::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.368_f32 + y.sin();
        let b = y * 1.229_f32 - x.cos();
        let mut acc = Accumulator747::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.306_f32 + y.sin();
        let b = y * 2.155_f32 - x.cos();
        let mut acc = Accumulator747::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_747(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(747u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_747() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_747(total as u64) % 997) as f32;
        total
    }
}

pub mod m748 {
    use super::*;

    pub struct Accumulator748<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator748<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.392_f32 + y.sin();
        let b = y * 1.385_f32 - x.cos();
        let mut acc = Accumulator748::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.817_f32 + y.sin();
        let b = y * 2.023_f32 - x.cos();
        let mut acc = Accumulator748::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.772_f32 + y.sin();
        let b = y * 8.252_f32 - x.cos();
        let mut acc = Accumulator748::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.083_f32 + y.sin();
        let b = y * 3.643_f32 - x.cos();
        let mut acc = Accumulator748::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.437_f32 + y.sin();
        let b = y * 2.029_f32 - x.cos();
        let mut acc = Accumulator748::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.418_f32 + y.sin();
        let b = y * 2.577_f32 - x.cos();
        let mut acc = Accumulator748::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.821_f32 + y.sin();
        let b = y * 5.016_f32 - x.cos();
        let mut acc = Accumulator748::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.126_f32 + y.sin();
        let b = y * 7.896_f32 - x.cos();
        let mut acc = Accumulator748::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.049_f32 + y.sin();
        let b = y * 5.963_f32 - x.cos();
        let mut acc = Accumulator748::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.273_f32 + y.sin();
        let b = y * 3.213_f32 - x.cos();
        let mut acc = Accumulator748::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.486_f32 + y.sin();
        let b = y * 1.965_f32 - x.cos();
        let mut acc = Accumulator748::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.975_f32 + y.sin();
        let b = y * 6.067_f32 - x.cos();
        let mut acc = Accumulator748::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.551_f32 + y.sin();
        let b = y * 8.446_f32 - x.cos();
        let mut acc = Accumulator748::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.595_f32 + y.sin();
        let b = y * 8.684_f32 - x.cos();
        let mut acc = Accumulator748::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.714_f32 + y.sin();
        let b = y * 1.952_f32 - x.cos();
        let mut acc = Accumulator748::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.161_f32 + y.sin();
        let b = y * 0.57_f32 - x.cos();
        let mut acc = Accumulator748::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.401_f32 + y.sin();
        let b = y * 3.484_f32 - x.cos();
        let mut acc = Accumulator748::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.856_f32 + y.sin();
        let b = y * 7.272_f32 - x.cos();
        let mut acc = Accumulator748::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.057_f32 + y.sin();
        let b = y * 6.376_f32 - x.cos();
        let mut acc = Accumulator748::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.512_f32 + y.sin();
        let b = y * 9.545_f32 - x.cos();
        let mut acc = Accumulator748::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_748(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_748() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_748(total as u64) % 997) as f32;
        total
    }
}

pub mod m749 {
    use super::*;

    pub struct Accumulator749<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator749<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.199_f32 + y.sin();
        let b = y * 3.134_f32 - x.cos();
        let mut acc = Accumulator749::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.977_f32 + y.sin();
        let b = y * 4.289_f32 - x.cos();
        let mut acc = Accumulator749::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.201_f32 + y.sin();
        let b = y * 4.893_f32 - x.cos();
        let mut acc = Accumulator749::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.155_f32 + y.sin();
        let b = y * 9.669_f32 - x.cos();
        let mut acc = Accumulator749::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.822_f32 + y.sin();
        let b = y * 0.13_f32 - x.cos();
        let mut acc = Accumulator749::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.605_f32 + y.sin();
        let b = y * 3.427_f32 - x.cos();
        let mut acc = Accumulator749::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.737_f32 + y.sin();
        let b = y * 0.97_f32 - x.cos();
        let mut acc = Accumulator749::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.806_f32 + y.sin();
        let b = y * 1.614_f32 - x.cos();
        let mut acc = Accumulator749::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.892_f32 + y.sin();
        let b = y * 2.065_f32 - x.cos();
        let mut acc = Accumulator749::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.534_f32 + y.sin();
        let b = y * 6.861_f32 - x.cos();
        let mut acc = Accumulator749::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.687_f32 + y.sin();
        let b = y * 1.855_f32 - x.cos();
        let mut acc = Accumulator749::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.112_f32 + y.sin();
        let b = y * 6.466_f32 - x.cos();
        let mut acc = Accumulator749::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.728_f32 + y.sin();
        let b = y * 4.813_f32 - x.cos();
        let mut acc = Accumulator749::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.855_f32 + y.sin();
        let b = y * 0.533_f32 - x.cos();
        let mut acc = Accumulator749::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.954_f32 + y.sin();
        let b = y * 5.255_f32 - x.cos();
        let mut acc = Accumulator749::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.845_f32 + y.sin();
        let b = y * 6.591_f32 - x.cos();
        let mut acc = Accumulator749::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.432_f32 + y.sin();
        let b = y * 4.42_f32 - x.cos();
        let mut acc = Accumulator749::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.406_f32 + y.sin();
        let b = y * 3.872_f32 - x.cos();
        let mut acc = Accumulator749::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.46_f32 + y.sin();
        let b = y * 4.339_f32 - x.cos();
        let mut acc = Accumulator749::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.418_f32 + y.sin();
        let b = y * 8.656_f32 - x.cos();
        let mut acc = Accumulator749::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_749(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_749() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_749(total as u64) % 997) as f32;
        total
    }
}

pub mod m750 {
    use super::*;

    pub struct Accumulator750<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator750<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.022_f32 + y.sin();
        let b = y * 4.656_f32 - x.cos();
        let mut acc = Accumulator750::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.397_f32 + y.sin();
        let b = y * 4.183_f32 - x.cos();
        let mut acc = Accumulator750::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.667_f32 + y.sin();
        let b = y * 1.891_f32 - x.cos();
        let mut acc = Accumulator750::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.338_f32 + y.sin();
        let b = y * 6.097_f32 - x.cos();
        let mut acc = Accumulator750::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.397_f32 + y.sin();
        let b = y * 6.614_f32 - x.cos();
        let mut acc = Accumulator750::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.451_f32 + y.sin();
        let b = y * 2.293_f32 - x.cos();
        let mut acc = Accumulator750::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.874_f32 + y.sin();
        let b = y * 6.486_f32 - x.cos();
        let mut acc = Accumulator750::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.204_f32 + y.sin();
        let b = y * 8.192_f32 - x.cos();
        let mut acc = Accumulator750::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.62_f32 + y.sin();
        let b = y * 1.855_f32 - x.cos();
        let mut acc = Accumulator750::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.82_f32 + y.sin();
        let b = y * 7.597_f32 - x.cos();
        let mut acc = Accumulator750::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.754_f32 + y.sin();
        let b = y * 4.312_f32 - x.cos();
        let mut acc = Accumulator750::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.269_f32 + y.sin();
        let b = y * 1.477_f32 - x.cos();
        let mut acc = Accumulator750::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.872_f32 + y.sin();
        let b = y * 9.412_f32 - x.cos();
        let mut acc = Accumulator750::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.998_f32 + y.sin();
        let b = y * 9.537_f32 - x.cos();
        let mut acc = Accumulator750::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.14_f32 + y.sin();
        let b = y * 9.279_f32 - x.cos();
        let mut acc = Accumulator750::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.396_f32 + y.sin();
        let b = y * 3.258_f32 - x.cos();
        let mut acc = Accumulator750::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.439_f32 + y.sin();
        let b = y * 9.43_f32 - x.cos();
        let mut acc = Accumulator750::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.269_f32 + y.sin();
        let b = y * 1.077_f32 - x.cos();
        let mut acc = Accumulator750::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.112_f32 + y.sin();
        let b = y * 2.839_f32 - x.cos();
        let mut acc = Accumulator750::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.024_f32 + y.sin();
        let b = y * 8.573_f32 - x.cos();
        let mut acc = Accumulator750::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_750(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m750-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_750() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_750(total as u64) % 997) as f32;
        total
    }
}

pub mod m751 {
    use super::*;

    pub struct Accumulator751<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator751<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.391_f32 + y.sin();
        let b = y * 8.964_f32 - x.cos();
        let mut acc = Accumulator751::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.687_f32 + y.sin();
        let b = y * 5.009_f32 - x.cos();
        let mut acc = Accumulator751::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.159_f32 + y.sin();
        let b = y * 1.516_f32 - x.cos();
        let mut acc = Accumulator751::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.364_f32 + y.sin();
        let b = y * 8.348_f32 - x.cos();
        let mut acc = Accumulator751::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.555_f32 + y.sin();
        let b = y * 9.854_f32 - x.cos();
        let mut acc = Accumulator751::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.417_f32 + y.sin();
        let b = y * 0.743_f32 - x.cos();
        let mut acc = Accumulator751::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.025_f32 + y.sin();
        let b = y * 0.833_f32 - x.cos();
        let mut acc = Accumulator751::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.005_f32 + y.sin();
        let b = y * 5.517_f32 - x.cos();
        let mut acc = Accumulator751::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.429_f32 + y.sin();
        let b = y * 4.198_f32 - x.cos();
        let mut acc = Accumulator751::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.747_f32 + y.sin();
        let b = y * 7.077_f32 - x.cos();
        let mut acc = Accumulator751::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.76_f32 + y.sin();
        let b = y * 8.819_f32 - x.cos();
        let mut acc = Accumulator751::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.432_f32 + y.sin();
        let b = y * 9.561_f32 - x.cos();
        let mut acc = Accumulator751::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.025_f32 + y.sin();
        let b = y * 6.865_f32 - x.cos();
        let mut acc = Accumulator751::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.061_f32 + y.sin();
        let b = y * 8.161_f32 - x.cos();
        let mut acc = Accumulator751::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.822_f32 + y.sin();
        let b = y * 8.53_f32 - x.cos();
        let mut acc = Accumulator751::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.1_f32 + y.sin();
        let b = y * 1.332_f32 - x.cos();
        let mut acc = Accumulator751::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.945_f32 + y.sin();
        let b = y * 3.169_f32 - x.cos();
        let mut acc = Accumulator751::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.75_f32 + y.sin();
        let b = y * 9.802_f32 - x.cos();
        let mut acc = Accumulator751::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.468_f32 + y.sin();
        let b = y * 6.919_f32 - x.cos();
        let mut acc = Accumulator751::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.0_f32 + y.sin();
        let b = y * 5.065_f32 - x.cos();
        let mut acc = Accumulator751::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_751(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_751() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_751(total as u64) % 997) as f32;
        total
    }
}

pub mod m752 {
    use super::*;

    pub struct Accumulator752<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator752<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.267_f32 + y.sin();
        let b = y * 1.999_f32 - x.cos();
        let mut acc = Accumulator752::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.53_f32 + y.sin();
        let b = y * 4.41_f32 - x.cos();
        let mut acc = Accumulator752::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.454_f32 + y.sin();
        let b = y * 8.68_f32 - x.cos();
        let mut acc = Accumulator752::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.216_f32 + y.sin();
        let b = y * 0.902_f32 - x.cos();
        let mut acc = Accumulator752::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.136_f32 + y.sin();
        let b = y * 9.056_f32 - x.cos();
        let mut acc = Accumulator752::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.856_f32 + y.sin();
        let b = y * 2.95_f32 - x.cos();
        let mut acc = Accumulator752::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.23_f32 + y.sin();
        let b = y * 5.16_f32 - x.cos();
        let mut acc = Accumulator752::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.117_f32 + y.sin();
        let b = y * 5.552_f32 - x.cos();
        let mut acc = Accumulator752::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.956_f32 + y.sin();
        let b = y * 2.888_f32 - x.cos();
        let mut acc = Accumulator752::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.153_f32 + y.sin();
        let b = y * 3.072_f32 - x.cos();
        let mut acc = Accumulator752::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.197_f32 + y.sin();
        let b = y * 3.125_f32 - x.cos();
        let mut acc = Accumulator752::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.231_f32 + y.sin();
        let b = y * 6.108_f32 - x.cos();
        let mut acc = Accumulator752::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.654_f32 + y.sin();
        let b = y * 3.57_f32 - x.cos();
        let mut acc = Accumulator752::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.624_f32 + y.sin();
        let b = y * 2.686_f32 - x.cos();
        let mut acc = Accumulator752::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.557_f32 + y.sin();
        let b = y * 8.532_f32 - x.cos();
        let mut acc = Accumulator752::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.821_f32 + y.sin();
        let b = y * 3.894_f32 - x.cos();
        let mut acc = Accumulator752::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.731_f32 + y.sin();
        let b = y * 1.112_f32 - x.cos();
        let mut acc = Accumulator752::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.805_f32 + y.sin();
        let b = y * 7.572_f32 - x.cos();
        let mut acc = Accumulator752::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.299_f32 + y.sin();
        let b = y * 8.04_f32 - x.cos();
        let mut acc = Accumulator752::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.749_f32 + y.sin();
        let b = y * 0.914_f32 - x.cos();
        let mut acc = Accumulator752::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_752(seed: u64) -> u64 {
        let re = Regex::new(r"m752-(\d+)").unwrap();
        let hay = format!("m752-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_752() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_752(total as u64) % 997) as f32;
        total
    }
}

pub mod m753 {
    use super::*;

    pub struct Accumulator753<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator753<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.715_f32 + y.sin();
        let b = y * 0.634_f32 - x.cos();
        let mut acc = Accumulator753::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.844_f32 + y.sin();
        let b = y * 2.617_f32 - x.cos();
        let mut acc = Accumulator753::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.681_f32 + y.sin();
        let b = y * 6.713_f32 - x.cos();
        let mut acc = Accumulator753::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.129_f32 + y.sin();
        let b = y * 5.075_f32 - x.cos();
        let mut acc = Accumulator753::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.218_f32 + y.sin();
        let b = y * 7.221_f32 - x.cos();
        let mut acc = Accumulator753::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.679_f32 + y.sin();
        let b = y * 4.059_f32 - x.cos();
        let mut acc = Accumulator753::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.023_f32 + y.sin();
        let b = y * 2.57_f32 - x.cos();
        let mut acc = Accumulator753::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.526_f32 + y.sin();
        let b = y * 0.222_f32 - x.cos();
        let mut acc = Accumulator753::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.479_f32 + y.sin();
        let b = y * 6.683_f32 - x.cos();
        let mut acc = Accumulator753::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.205_f32 + y.sin();
        let b = y * 1.967_f32 - x.cos();
        let mut acc = Accumulator753::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.703_f32 + y.sin();
        let b = y * 4.075_f32 - x.cos();
        let mut acc = Accumulator753::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.067_f32 + y.sin();
        let b = y * 2.534_f32 - x.cos();
        let mut acc = Accumulator753::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.416_f32 + y.sin();
        let b = y * 6.191_f32 - x.cos();
        let mut acc = Accumulator753::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.277_f32 + y.sin();
        let b = y * 5.556_f32 - x.cos();
        let mut acc = Accumulator753::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.168_f32 + y.sin();
        let b = y * 0.206_f32 - x.cos();
        let mut acc = Accumulator753::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.096_f32 + y.sin();
        let b = y * 1.71_f32 - x.cos();
        let mut acc = Accumulator753::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.861_f32 + y.sin();
        let b = y * 2.248_f32 - x.cos();
        let mut acc = Accumulator753::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.928_f32 + y.sin();
        let b = y * 6.322_f32 - x.cos();
        let mut acc = Accumulator753::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.848_f32 + y.sin();
        let b = y * 3.791_f32 - x.cos();
        let mut acc = Accumulator753::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.812_f32 + y.sin();
        let b = y * 9.758_f32 - x.cos();
        let mut acc = Accumulator753::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_753(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_753() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_753(total as u64) % 997) as f32;
        total
    }
}

pub mod m754 {
    use super::*;

    pub struct Accumulator754<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator754<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.687_f32 + y.sin();
        let b = y * 8.173_f32 - x.cos();
        let mut acc = Accumulator754::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.17_f32 + y.sin();
        let b = y * 6.226_f32 - x.cos();
        let mut acc = Accumulator754::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.643_f32 + y.sin();
        let b = y * 8.176_f32 - x.cos();
        let mut acc = Accumulator754::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.173_f32 + y.sin();
        let b = y * 6.363_f32 - x.cos();
        let mut acc = Accumulator754::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.583_f32 + y.sin();
        let b = y * 9.58_f32 - x.cos();
        let mut acc = Accumulator754::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.025_f32 + y.sin();
        let b = y * 0.121_f32 - x.cos();
        let mut acc = Accumulator754::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.251_f32 + y.sin();
        let b = y * 3.109_f32 - x.cos();
        let mut acc = Accumulator754::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.526_f32 + y.sin();
        let b = y * 2.038_f32 - x.cos();
        let mut acc = Accumulator754::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.554_f32 + y.sin();
        let b = y * 5.372_f32 - x.cos();
        let mut acc = Accumulator754::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.41_f32 + y.sin();
        let b = y * 6.949_f32 - x.cos();
        let mut acc = Accumulator754::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.385_f32 + y.sin();
        let b = y * 0.223_f32 - x.cos();
        let mut acc = Accumulator754::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.894_f32 + y.sin();
        let b = y * 6.136_f32 - x.cos();
        let mut acc = Accumulator754::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.397_f32 + y.sin();
        let b = y * 8.724_f32 - x.cos();
        let mut acc = Accumulator754::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.426_f32 + y.sin();
        let b = y * 5.455_f32 - x.cos();
        let mut acc = Accumulator754::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.595_f32 + y.sin();
        let b = y * 8.878_f32 - x.cos();
        let mut acc = Accumulator754::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.165_f32 + y.sin();
        let b = y * 1.628_f32 - x.cos();
        let mut acc = Accumulator754::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.934_f32 + y.sin();
        let b = y * 8.196_f32 - x.cos();
        let mut acc = Accumulator754::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.852_f32 + y.sin();
        let b = y * 4.396_f32 - x.cos();
        let mut acc = Accumulator754::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.213_f32 + y.sin();
        let b = y * 3.494_f32 - x.cos();
        let mut acc = Accumulator754::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.653_f32 + y.sin();
        let b = y * 7.276_f32 - x.cos();
        let mut acc = Accumulator754::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_754(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(754u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_754() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_754(total as u64) % 997) as f32;
        total
    }
}

pub mod m755 {
    use super::*;

    pub struct Accumulator755<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator755<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.541_f32 + y.sin();
        let b = y * 9.768_f32 - x.cos();
        let mut acc = Accumulator755::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.406_f32 + y.sin();
        let b = y * 1.962_f32 - x.cos();
        let mut acc = Accumulator755::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.608_f32 + y.sin();
        let b = y * 1.563_f32 - x.cos();
        let mut acc = Accumulator755::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.584_f32 + y.sin();
        let b = y * 1.47_f32 - x.cos();
        let mut acc = Accumulator755::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.611_f32 + y.sin();
        let b = y * 9.161_f32 - x.cos();
        let mut acc = Accumulator755::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.165_f32 + y.sin();
        let b = y * 5.439_f32 - x.cos();
        let mut acc = Accumulator755::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.172_f32 + y.sin();
        let b = y * 7.759_f32 - x.cos();
        let mut acc = Accumulator755::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.119_f32 + y.sin();
        let b = y * 5.361_f32 - x.cos();
        let mut acc = Accumulator755::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.838_f32 + y.sin();
        let b = y * 1.539_f32 - x.cos();
        let mut acc = Accumulator755::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.898_f32 + y.sin();
        let b = y * 6.272_f32 - x.cos();
        let mut acc = Accumulator755::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.046_f32 + y.sin();
        let b = y * 9.666_f32 - x.cos();
        let mut acc = Accumulator755::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.555_f32 + y.sin();
        let b = y * 5.931_f32 - x.cos();
        let mut acc = Accumulator755::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.941_f32 + y.sin();
        let b = y * 5.753_f32 - x.cos();
        let mut acc = Accumulator755::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.096_f32 + y.sin();
        let b = y * 2.037_f32 - x.cos();
        let mut acc = Accumulator755::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.485_f32 + y.sin();
        let b = y * 5.128_f32 - x.cos();
        let mut acc = Accumulator755::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.864_f32 + y.sin();
        let b = y * 5.966_f32 - x.cos();
        let mut acc = Accumulator755::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.147_f32 + y.sin();
        let b = y * 7.353_f32 - x.cos();
        let mut acc = Accumulator755::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.185_f32 + y.sin();
        let b = y * 6.898_f32 - x.cos();
        let mut acc = Accumulator755::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.003_f32 + y.sin();
        let b = y * 1.641_f32 - x.cos();
        let mut acc = Accumulator755::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.547_f32 + y.sin();
        let b = y * 4.008_f32 - x.cos();
        let mut acc = Accumulator755::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_755(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_755() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_755(total as u64) % 997) as f32;
        total
    }
}

pub mod m756 {
    use super::*;

    pub struct Accumulator756<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator756<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.199_f32 + y.sin();
        let b = y * 3.458_f32 - x.cos();
        let mut acc = Accumulator756::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.44_f32 + y.sin();
        let b = y * 6.062_f32 - x.cos();
        let mut acc = Accumulator756::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.586_f32 + y.sin();
        let b = y * 1.155_f32 - x.cos();
        let mut acc = Accumulator756::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.161_f32 + y.sin();
        let b = y * 3.644_f32 - x.cos();
        let mut acc = Accumulator756::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.214_f32 + y.sin();
        let b = y * 7.307_f32 - x.cos();
        let mut acc = Accumulator756::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.13_f32 + y.sin();
        let b = y * 2.609_f32 - x.cos();
        let mut acc = Accumulator756::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.226_f32 + y.sin();
        let b = y * 4.02_f32 - x.cos();
        let mut acc = Accumulator756::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.024_f32 + y.sin();
        let b = y * 0.3_f32 - x.cos();
        let mut acc = Accumulator756::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.575_f32 + y.sin();
        let b = y * 8.155_f32 - x.cos();
        let mut acc = Accumulator756::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.005_f32 + y.sin();
        let b = y * 2.755_f32 - x.cos();
        let mut acc = Accumulator756::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.96_f32 + y.sin();
        let b = y * 4.207_f32 - x.cos();
        let mut acc = Accumulator756::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.02_f32 + y.sin();
        let b = y * 7.335_f32 - x.cos();
        let mut acc = Accumulator756::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.475_f32 + y.sin();
        let b = y * 7.802_f32 - x.cos();
        let mut acc = Accumulator756::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.113_f32 + y.sin();
        let b = y * 1.292_f32 - x.cos();
        let mut acc = Accumulator756::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.989_f32 + y.sin();
        let b = y * 8.873_f32 - x.cos();
        let mut acc = Accumulator756::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.429_f32 + y.sin();
        let b = y * 8.273_f32 - x.cos();
        let mut acc = Accumulator756::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.058_f32 + y.sin();
        let b = y * 4.946_f32 - x.cos();
        let mut acc = Accumulator756::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.664_f32 + y.sin();
        let b = y * 1.717_f32 - x.cos();
        let mut acc = Accumulator756::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.862_f32 + y.sin();
        let b = y * 9.618_f32 - x.cos();
        let mut acc = Accumulator756::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.638_f32 + y.sin();
        let b = y * 3.712_f32 - x.cos();
        let mut acc = Accumulator756::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_756(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_756() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_756(total as u64) % 997) as f32;
        total
    }
}

pub mod m757 {
    use super::*;

    pub struct Accumulator757<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator757<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.309_f32 + y.sin();
        let b = y * 9.874_f32 - x.cos();
        let mut acc = Accumulator757::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.189_f32 + y.sin();
        let b = y * 9.797_f32 - x.cos();
        let mut acc = Accumulator757::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.358_f32 + y.sin();
        let b = y * 4.036_f32 - x.cos();
        let mut acc = Accumulator757::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.486_f32 + y.sin();
        let b = y * 7.935_f32 - x.cos();
        let mut acc = Accumulator757::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.712_f32 + y.sin();
        let b = y * 0.669_f32 - x.cos();
        let mut acc = Accumulator757::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.653_f32 + y.sin();
        let b = y * 2.544_f32 - x.cos();
        let mut acc = Accumulator757::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.056_f32 + y.sin();
        let b = y * 2.001_f32 - x.cos();
        let mut acc = Accumulator757::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.449_f32 + y.sin();
        let b = y * 2.847_f32 - x.cos();
        let mut acc = Accumulator757::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.282_f32 + y.sin();
        let b = y * 2.419_f32 - x.cos();
        let mut acc = Accumulator757::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.78_f32 + y.sin();
        let b = y * 2.559_f32 - x.cos();
        let mut acc = Accumulator757::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.115_f32 + y.sin();
        let b = y * 5.068_f32 - x.cos();
        let mut acc = Accumulator757::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.454_f32 + y.sin();
        let b = y * 1.562_f32 - x.cos();
        let mut acc = Accumulator757::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.835_f32 + y.sin();
        let b = y * 1.185_f32 - x.cos();
        let mut acc = Accumulator757::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.234_f32 + y.sin();
        let b = y * 2.156_f32 - x.cos();
        let mut acc = Accumulator757::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.539_f32 + y.sin();
        let b = y * 3.634_f32 - x.cos();
        let mut acc = Accumulator757::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.353_f32 + y.sin();
        let b = y * 9.008_f32 - x.cos();
        let mut acc = Accumulator757::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.846_f32 + y.sin();
        let b = y * 0.903_f32 - x.cos();
        let mut acc = Accumulator757::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.395_f32 + y.sin();
        let b = y * 8.043_f32 - x.cos();
        let mut acc = Accumulator757::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.195_f32 + y.sin();
        let b = y * 0.417_f32 - x.cos();
        let mut acc = Accumulator757::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.009_f32 + y.sin();
        let b = y * 4.441_f32 - x.cos();
        let mut acc = Accumulator757::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_757(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m757-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_757() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_757(total as u64) % 997) as f32;
        total
    }
}

pub mod m758 {
    use super::*;

    pub struct Accumulator758<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator758<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.713_f32 + y.sin();
        let b = y * 1.471_f32 - x.cos();
        let mut acc = Accumulator758::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.822_f32 + y.sin();
        let b = y * 3.28_f32 - x.cos();
        let mut acc = Accumulator758::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.457_f32 + y.sin();
        let b = y * 1.274_f32 - x.cos();
        let mut acc = Accumulator758::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.97_f32 + y.sin();
        let b = y * 8.6_f32 - x.cos();
        let mut acc = Accumulator758::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.412_f32 + y.sin();
        let b = y * 0.21_f32 - x.cos();
        let mut acc = Accumulator758::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.016_f32 + y.sin();
        let b = y * 5.748_f32 - x.cos();
        let mut acc = Accumulator758::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.442_f32 + y.sin();
        let b = y * 7.223_f32 - x.cos();
        let mut acc = Accumulator758::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.015_f32 + y.sin();
        let b = y * 8.589_f32 - x.cos();
        let mut acc = Accumulator758::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.449_f32 + y.sin();
        let b = y * 0.935_f32 - x.cos();
        let mut acc = Accumulator758::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.278_f32 + y.sin();
        let b = y * 5.079_f32 - x.cos();
        let mut acc = Accumulator758::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.73_f32 + y.sin();
        let b = y * 8.121_f32 - x.cos();
        let mut acc = Accumulator758::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.683_f32 + y.sin();
        let b = y * 7.688_f32 - x.cos();
        let mut acc = Accumulator758::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.94_f32 + y.sin();
        let b = y * 0.723_f32 - x.cos();
        let mut acc = Accumulator758::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.718_f32 + y.sin();
        let b = y * 7.141_f32 - x.cos();
        let mut acc = Accumulator758::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.05_f32 + y.sin();
        let b = y * 1.907_f32 - x.cos();
        let mut acc = Accumulator758::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.704_f32 + y.sin();
        let b = y * 1.598_f32 - x.cos();
        let mut acc = Accumulator758::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.709_f32 + y.sin();
        let b = y * 0.131_f32 - x.cos();
        let mut acc = Accumulator758::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.393_f32 + y.sin();
        let b = y * 8.366_f32 - x.cos();
        let mut acc = Accumulator758::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.123_f32 + y.sin();
        let b = y * 2.786_f32 - x.cos();
        let mut acc = Accumulator758::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.508_f32 + y.sin();
        let b = y * 9.834_f32 - x.cos();
        let mut acc = Accumulator758::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_758(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_758() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_758(total as u64) % 997) as f32;
        total
    }
}

pub mod m759 {
    use super::*;

    pub struct Accumulator759<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator759<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.264_f32 + y.sin();
        let b = y * 6.303_f32 - x.cos();
        let mut acc = Accumulator759::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.324_f32 + y.sin();
        let b = y * 3.853_f32 - x.cos();
        let mut acc = Accumulator759::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.621_f32 + y.sin();
        let b = y * 3.758_f32 - x.cos();
        let mut acc = Accumulator759::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.138_f32 + y.sin();
        let b = y * 2.452_f32 - x.cos();
        let mut acc = Accumulator759::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.501_f32 + y.sin();
        let b = y * 2.647_f32 - x.cos();
        let mut acc = Accumulator759::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.275_f32 + y.sin();
        let b = y * 6.101_f32 - x.cos();
        let mut acc = Accumulator759::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.917_f32 + y.sin();
        let b = y * 2.584_f32 - x.cos();
        let mut acc = Accumulator759::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.761_f32 + y.sin();
        let b = y * 5.149_f32 - x.cos();
        let mut acc = Accumulator759::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.662_f32 + y.sin();
        let b = y * 6.322_f32 - x.cos();
        let mut acc = Accumulator759::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.051_f32 + y.sin();
        let b = y * 7.186_f32 - x.cos();
        let mut acc = Accumulator759::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.326_f32 + y.sin();
        let b = y * 8.601_f32 - x.cos();
        let mut acc = Accumulator759::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.446_f32 + y.sin();
        let b = y * 9.868_f32 - x.cos();
        let mut acc = Accumulator759::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.578_f32 + y.sin();
        let b = y * 4.562_f32 - x.cos();
        let mut acc = Accumulator759::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.556_f32 + y.sin();
        let b = y * 7.263_f32 - x.cos();
        let mut acc = Accumulator759::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.39_f32 + y.sin();
        let b = y * 5.953_f32 - x.cos();
        let mut acc = Accumulator759::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.681_f32 + y.sin();
        let b = y * 4.938_f32 - x.cos();
        let mut acc = Accumulator759::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.636_f32 + y.sin();
        let b = y * 6.391_f32 - x.cos();
        let mut acc = Accumulator759::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.295_f32 + y.sin();
        let b = y * 7.947_f32 - x.cos();
        let mut acc = Accumulator759::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.618_f32 + y.sin();
        let b = y * 6.09_f32 - x.cos();
        let mut acc = Accumulator759::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.111_f32 + y.sin();
        let b = y * 7.972_f32 - x.cos();
        let mut acc = Accumulator759::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_759(seed: u64) -> u64 {
        let re = Regex::new(r"m759-(\d+)").unwrap();
        let hay = format!("m759-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_759() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_759(total as u64) % 997) as f32;
        total
    }
}

pub mod m760 {
    use super::*;

    pub struct Accumulator760<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator760<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.305_f32 + y.sin();
        let b = y * 3.82_f32 - x.cos();
        let mut acc = Accumulator760::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.994_f32 + y.sin();
        let b = y * 6.491_f32 - x.cos();
        let mut acc = Accumulator760::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.715_f32 + y.sin();
        let b = y * 8.549_f32 - x.cos();
        let mut acc = Accumulator760::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.927_f32 + y.sin();
        let b = y * 8.859_f32 - x.cos();
        let mut acc = Accumulator760::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.353_f32 + y.sin();
        let b = y * 6.61_f32 - x.cos();
        let mut acc = Accumulator760::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.084_f32 + y.sin();
        let b = y * 5.817_f32 - x.cos();
        let mut acc = Accumulator760::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.97_f32 + y.sin();
        let b = y * 2.698_f32 - x.cos();
        let mut acc = Accumulator760::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.443_f32 + y.sin();
        let b = y * 0.344_f32 - x.cos();
        let mut acc = Accumulator760::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.453_f32 + y.sin();
        let b = y * 3.094_f32 - x.cos();
        let mut acc = Accumulator760::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.385_f32 + y.sin();
        let b = y * 9.652_f32 - x.cos();
        let mut acc = Accumulator760::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.766_f32 + y.sin();
        let b = y * 7.002_f32 - x.cos();
        let mut acc = Accumulator760::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.17_f32 + y.sin();
        let b = y * 8.529_f32 - x.cos();
        let mut acc = Accumulator760::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.95_f32 + y.sin();
        let b = y * 4.795_f32 - x.cos();
        let mut acc = Accumulator760::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.338_f32 + y.sin();
        let b = y * 1.056_f32 - x.cos();
        let mut acc = Accumulator760::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.171_f32 + y.sin();
        let b = y * 2.535_f32 - x.cos();
        let mut acc = Accumulator760::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.285_f32 + y.sin();
        let b = y * 3.109_f32 - x.cos();
        let mut acc = Accumulator760::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.604_f32 + y.sin();
        let b = y * 7.272_f32 - x.cos();
        let mut acc = Accumulator760::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.804_f32 + y.sin();
        let b = y * 0.53_f32 - x.cos();
        let mut acc = Accumulator760::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.636_f32 + y.sin();
        let b = y * 9.094_f32 - x.cos();
        let mut acc = Accumulator760::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.997_f32 + y.sin();
        let b = y * 6.249_f32 - x.cos();
        let mut acc = Accumulator760::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_760(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_760() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_760(total as u64) % 997) as f32;
        total
    }
}

pub mod m761 {
    use super::*;

    pub struct Accumulator761<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator761<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.236_f32 + y.sin();
        let b = y * 3.516_f32 - x.cos();
        let mut acc = Accumulator761::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.834_f32 + y.sin();
        let b = y * 1.359_f32 - x.cos();
        let mut acc = Accumulator761::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.834_f32 + y.sin();
        let b = y * 2.834_f32 - x.cos();
        let mut acc = Accumulator761::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.709_f32 + y.sin();
        let b = y * 7.293_f32 - x.cos();
        let mut acc = Accumulator761::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.181_f32 + y.sin();
        let b = y * 3.475_f32 - x.cos();
        let mut acc = Accumulator761::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.283_f32 + y.sin();
        let b = y * 3.725_f32 - x.cos();
        let mut acc = Accumulator761::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.069_f32 + y.sin();
        let b = y * 9.463_f32 - x.cos();
        let mut acc = Accumulator761::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.593_f32 + y.sin();
        let b = y * 6.19_f32 - x.cos();
        let mut acc = Accumulator761::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.975_f32 + y.sin();
        let b = y * 4.349_f32 - x.cos();
        let mut acc = Accumulator761::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.942_f32 + y.sin();
        let b = y * 2.149_f32 - x.cos();
        let mut acc = Accumulator761::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.535_f32 + y.sin();
        let b = y * 5.461_f32 - x.cos();
        let mut acc = Accumulator761::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.504_f32 + y.sin();
        let b = y * 9.092_f32 - x.cos();
        let mut acc = Accumulator761::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.194_f32 + y.sin();
        let b = y * 8.756_f32 - x.cos();
        let mut acc = Accumulator761::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.13_f32 + y.sin();
        let b = y * 6.233_f32 - x.cos();
        let mut acc = Accumulator761::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.766_f32 + y.sin();
        let b = y * 1.334_f32 - x.cos();
        let mut acc = Accumulator761::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.678_f32 + y.sin();
        let b = y * 4.709_f32 - x.cos();
        let mut acc = Accumulator761::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.008_f32 + y.sin();
        let b = y * 4.911_f32 - x.cos();
        let mut acc = Accumulator761::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.504_f32 + y.sin();
        let b = y * 7.537_f32 - x.cos();
        let mut acc = Accumulator761::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.556_f32 + y.sin();
        let b = y * 5.389_f32 - x.cos();
        let mut acc = Accumulator761::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.438_f32 + y.sin();
        let b = y * 0.688_f32 - x.cos();
        let mut acc = Accumulator761::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_761(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(761u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_761() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_761(total as u64) % 997) as f32;
        total
    }
}

pub mod m762 {
    use super::*;

    pub struct Accumulator762<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator762<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.794_f32 + y.sin();
        let b = y * 1.178_f32 - x.cos();
        let mut acc = Accumulator762::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.572_f32 + y.sin();
        let b = y * 9.583_f32 - x.cos();
        let mut acc = Accumulator762::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.77_f32 + y.sin();
        let b = y * 4.301_f32 - x.cos();
        let mut acc = Accumulator762::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.158_f32 + y.sin();
        let b = y * 2.958_f32 - x.cos();
        let mut acc = Accumulator762::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.82_f32 + y.sin();
        let b = y * 7.764_f32 - x.cos();
        let mut acc = Accumulator762::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.71_f32 + y.sin();
        let b = y * 1.997_f32 - x.cos();
        let mut acc = Accumulator762::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.621_f32 + y.sin();
        let b = y * 9.445_f32 - x.cos();
        let mut acc = Accumulator762::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.529_f32 + y.sin();
        let b = y * 7.169_f32 - x.cos();
        let mut acc = Accumulator762::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.918_f32 + y.sin();
        let b = y * 8.542_f32 - x.cos();
        let mut acc = Accumulator762::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.675_f32 + y.sin();
        let b = y * 9.47_f32 - x.cos();
        let mut acc = Accumulator762::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.452_f32 + y.sin();
        let b = y * 3.329_f32 - x.cos();
        let mut acc = Accumulator762::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.331_f32 + y.sin();
        let b = y * 1.01_f32 - x.cos();
        let mut acc = Accumulator762::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.415_f32 + y.sin();
        let b = y * 1.712_f32 - x.cos();
        let mut acc = Accumulator762::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.296_f32 + y.sin();
        let b = y * 3.91_f32 - x.cos();
        let mut acc = Accumulator762::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.23_f32 + y.sin();
        let b = y * 1.966_f32 - x.cos();
        let mut acc = Accumulator762::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.36_f32 + y.sin();
        let b = y * 3.272_f32 - x.cos();
        let mut acc = Accumulator762::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.946_f32 + y.sin();
        let b = y * 3.289_f32 - x.cos();
        let mut acc = Accumulator762::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.837_f32 + y.sin();
        let b = y * 2.557_f32 - x.cos();
        let mut acc = Accumulator762::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.099_f32 + y.sin();
        let b = y * 7.615_f32 - x.cos();
        let mut acc = Accumulator762::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.224_f32 + y.sin();
        let b = y * 5.374_f32 - x.cos();
        let mut acc = Accumulator762::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_762(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_762() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_762(total as u64) % 997) as f32;
        total
    }
}

pub mod m763 {
    use super::*;

    pub struct Accumulator763<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator763<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.123_f32 + y.sin();
        let b = y * 3.595_f32 - x.cos();
        let mut acc = Accumulator763::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.122_f32 + y.sin();
        let b = y * 2.056_f32 - x.cos();
        let mut acc = Accumulator763::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.045_f32 + y.sin();
        let b = y * 1.729_f32 - x.cos();
        let mut acc = Accumulator763::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.028_f32 + y.sin();
        let b = y * 9.815_f32 - x.cos();
        let mut acc = Accumulator763::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.23_f32 + y.sin();
        let b = y * 4.033_f32 - x.cos();
        let mut acc = Accumulator763::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.032_f32 + y.sin();
        let b = y * 1.102_f32 - x.cos();
        let mut acc = Accumulator763::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.784_f32 + y.sin();
        let b = y * 3.851_f32 - x.cos();
        let mut acc = Accumulator763::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.847_f32 + y.sin();
        let b = y * 4.527_f32 - x.cos();
        let mut acc = Accumulator763::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.587_f32 + y.sin();
        let b = y * 5.751_f32 - x.cos();
        let mut acc = Accumulator763::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.806_f32 + y.sin();
        let b = y * 4.174_f32 - x.cos();
        let mut acc = Accumulator763::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.466_f32 + y.sin();
        let b = y * 4.163_f32 - x.cos();
        let mut acc = Accumulator763::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.894_f32 + y.sin();
        let b = y * 8.914_f32 - x.cos();
        let mut acc = Accumulator763::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.047_f32 + y.sin();
        let b = y * 1.959_f32 - x.cos();
        let mut acc = Accumulator763::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.922_f32 + y.sin();
        let b = y * 9.473_f32 - x.cos();
        let mut acc = Accumulator763::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.798_f32 + y.sin();
        let b = y * 6.546_f32 - x.cos();
        let mut acc = Accumulator763::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.111_f32 + y.sin();
        let b = y * 1.187_f32 - x.cos();
        let mut acc = Accumulator763::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.209_f32 + y.sin();
        let b = y * 2.717_f32 - x.cos();
        let mut acc = Accumulator763::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.363_f32 + y.sin();
        let b = y * 8.455_f32 - x.cos();
        let mut acc = Accumulator763::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.176_f32 + y.sin();
        let b = y * 4.622_f32 - x.cos();
        let mut acc = Accumulator763::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.478_f32 + y.sin();
        let b = y * 1.059_f32 - x.cos();
        let mut acc = Accumulator763::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_763(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_763() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_763(total as u64) % 997) as f32;
        total
    }
}

pub mod m764 {
    use super::*;

    pub struct Accumulator764<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator764<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.36_f32 + y.sin();
        let b = y * 8.072_f32 - x.cos();
        let mut acc = Accumulator764::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.326_f32 + y.sin();
        let b = y * 4.716_f32 - x.cos();
        let mut acc = Accumulator764::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.945_f32 + y.sin();
        let b = y * 8.544_f32 - x.cos();
        let mut acc = Accumulator764::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.165_f32 + y.sin();
        let b = y * 8.168_f32 - x.cos();
        let mut acc = Accumulator764::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.447_f32 + y.sin();
        let b = y * 9.64_f32 - x.cos();
        let mut acc = Accumulator764::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.836_f32 + y.sin();
        let b = y * 5.741_f32 - x.cos();
        let mut acc = Accumulator764::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.87_f32 + y.sin();
        let b = y * 4.612_f32 - x.cos();
        let mut acc = Accumulator764::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.775_f32 + y.sin();
        let b = y * 1.052_f32 - x.cos();
        let mut acc = Accumulator764::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.689_f32 + y.sin();
        let b = y * 5.604_f32 - x.cos();
        let mut acc = Accumulator764::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.123_f32 + y.sin();
        let b = y * 2.349_f32 - x.cos();
        let mut acc = Accumulator764::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.181_f32 + y.sin();
        let b = y * 1.513_f32 - x.cos();
        let mut acc = Accumulator764::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.365_f32 + y.sin();
        let b = y * 8.359_f32 - x.cos();
        let mut acc = Accumulator764::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.298_f32 + y.sin();
        let b = y * 1.925_f32 - x.cos();
        let mut acc = Accumulator764::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.076_f32 + y.sin();
        let b = y * 2.591_f32 - x.cos();
        let mut acc = Accumulator764::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.367_f32 + y.sin();
        let b = y * 4.296_f32 - x.cos();
        let mut acc = Accumulator764::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.798_f32 + y.sin();
        let b = y * 1.938_f32 - x.cos();
        let mut acc = Accumulator764::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.189_f32 + y.sin();
        let b = y * 7.231_f32 - x.cos();
        let mut acc = Accumulator764::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.456_f32 + y.sin();
        let b = y * 0.432_f32 - x.cos();
        let mut acc = Accumulator764::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.436_f32 + y.sin();
        let b = y * 4.768_f32 - x.cos();
        let mut acc = Accumulator764::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.976_f32 + y.sin();
        let b = y * 1.616_f32 - x.cos();
        let mut acc = Accumulator764::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_764(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m764-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_764() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_764(total as u64) % 997) as f32;
        total
    }
}

pub mod m765 {
    use super::*;

    pub struct Accumulator765<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator765<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.738_f32 + y.sin();
        let b = y * 2.845_f32 - x.cos();
        let mut acc = Accumulator765::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.745_f32 + y.sin();
        let b = y * 9.466_f32 - x.cos();
        let mut acc = Accumulator765::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.314_f32 + y.sin();
        let b = y * 5.596_f32 - x.cos();
        let mut acc = Accumulator765::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.646_f32 + y.sin();
        let b = y * 5.093_f32 - x.cos();
        let mut acc = Accumulator765::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.614_f32 + y.sin();
        let b = y * 3.095_f32 - x.cos();
        let mut acc = Accumulator765::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.624_f32 + y.sin();
        let b = y * 8.039_f32 - x.cos();
        let mut acc = Accumulator765::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.802_f32 + y.sin();
        let b = y * 7.484_f32 - x.cos();
        let mut acc = Accumulator765::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.747_f32 + y.sin();
        let b = y * 9.013_f32 - x.cos();
        let mut acc = Accumulator765::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.001_f32 + y.sin();
        let b = y * 9.438_f32 - x.cos();
        let mut acc = Accumulator765::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.398_f32 + y.sin();
        let b = y * 3.437_f32 - x.cos();
        let mut acc = Accumulator765::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.363_f32 + y.sin();
        let b = y * 0.752_f32 - x.cos();
        let mut acc = Accumulator765::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.874_f32 + y.sin();
        let b = y * 4.121_f32 - x.cos();
        let mut acc = Accumulator765::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.268_f32 + y.sin();
        let b = y * 5.715_f32 - x.cos();
        let mut acc = Accumulator765::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.681_f32 + y.sin();
        let b = y * 2.161_f32 - x.cos();
        let mut acc = Accumulator765::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.242_f32 + y.sin();
        let b = y * 7.717_f32 - x.cos();
        let mut acc = Accumulator765::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.54_f32 + y.sin();
        let b = y * 7.862_f32 - x.cos();
        let mut acc = Accumulator765::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.3_f32 + y.sin();
        let b = y * 2.476_f32 - x.cos();
        let mut acc = Accumulator765::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.605_f32 + y.sin();
        let b = y * 7.317_f32 - x.cos();
        let mut acc = Accumulator765::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.969_f32 + y.sin();
        let b = y * 6.179_f32 - x.cos();
        let mut acc = Accumulator765::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.773_f32 + y.sin();
        let b = y * 8.271_f32 - x.cos();
        let mut acc = Accumulator765::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_765(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_765() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_765(total as u64) % 997) as f32;
        total
    }
}

pub mod m766 {
    use super::*;

    pub struct Accumulator766<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator766<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.443_f32 + y.sin();
        let b = y * 5.275_f32 - x.cos();
        let mut acc = Accumulator766::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.382_f32 + y.sin();
        let b = y * 5.595_f32 - x.cos();
        let mut acc = Accumulator766::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.478_f32 + y.sin();
        let b = y * 4.3_f32 - x.cos();
        let mut acc = Accumulator766::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.188_f32 + y.sin();
        let b = y * 8.069_f32 - x.cos();
        let mut acc = Accumulator766::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.672_f32 + y.sin();
        let b = y * 9.088_f32 - x.cos();
        let mut acc = Accumulator766::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.12_f32 + y.sin();
        let b = y * 2.018_f32 - x.cos();
        let mut acc = Accumulator766::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.467_f32 + y.sin();
        let b = y * 7.795_f32 - x.cos();
        let mut acc = Accumulator766::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.138_f32 + y.sin();
        let b = y * 1.02_f32 - x.cos();
        let mut acc = Accumulator766::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.282_f32 + y.sin();
        let b = y * 8.949_f32 - x.cos();
        let mut acc = Accumulator766::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.138_f32 + y.sin();
        let b = y * 4.303_f32 - x.cos();
        let mut acc = Accumulator766::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.917_f32 + y.sin();
        let b = y * 8.15_f32 - x.cos();
        let mut acc = Accumulator766::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.523_f32 + y.sin();
        let b = y * 9.523_f32 - x.cos();
        let mut acc = Accumulator766::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.373_f32 + y.sin();
        let b = y * 3.372_f32 - x.cos();
        let mut acc = Accumulator766::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.674_f32 + y.sin();
        let b = y * 2.203_f32 - x.cos();
        let mut acc = Accumulator766::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.905_f32 + y.sin();
        let b = y * 7.088_f32 - x.cos();
        let mut acc = Accumulator766::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.943_f32 + y.sin();
        let b = y * 2.537_f32 - x.cos();
        let mut acc = Accumulator766::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.549_f32 + y.sin();
        let b = y * 8.861_f32 - x.cos();
        let mut acc = Accumulator766::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.011_f32 + y.sin();
        let b = y * 8.241_f32 - x.cos();
        let mut acc = Accumulator766::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.387_f32 + y.sin();
        let b = y * 6.21_f32 - x.cos();
        let mut acc = Accumulator766::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.383_f32 + y.sin();
        let b = y * 4.805_f32 - x.cos();
        let mut acc = Accumulator766::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_766(seed: u64) -> u64 {
        let re = Regex::new(r"m766-(\d+)").unwrap();
        let hay = format!("m766-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_766() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_766(total as u64) % 997) as f32;
        total
    }
}

pub mod m767 {
    use super::*;

    pub struct Accumulator767<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator767<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.542_f32 + y.sin();
        let b = y * 7.051_f32 - x.cos();
        let mut acc = Accumulator767::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.01_f32 + y.sin();
        let b = y * 2.83_f32 - x.cos();
        let mut acc = Accumulator767::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.283_f32 + y.sin();
        let b = y * 8.469_f32 - x.cos();
        let mut acc = Accumulator767::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.301_f32 + y.sin();
        let b = y * 8.086_f32 - x.cos();
        let mut acc = Accumulator767::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.467_f32 + y.sin();
        let b = y * 4.493_f32 - x.cos();
        let mut acc = Accumulator767::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.036_f32 + y.sin();
        let b = y * 9.403_f32 - x.cos();
        let mut acc = Accumulator767::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.558_f32 + y.sin();
        let b = y * 5.512_f32 - x.cos();
        let mut acc = Accumulator767::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.152_f32 + y.sin();
        let b = y * 6.414_f32 - x.cos();
        let mut acc = Accumulator767::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.448_f32 + y.sin();
        let b = y * 6.507_f32 - x.cos();
        let mut acc = Accumulator767::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.532_f32 + y.sin();
        let b = y * 3.338_f32 - x.cos();
        let mut acc = Accumulator767::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.339_f32 + y.sin();
        let b = y * 3.288_f32 - x.cos();
        let mut acc = Accumulator767::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.395_f32 + y.sin();
        let b = y * 7.24_f32 - x.cos();
        let mut acc = Accumulator767::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.716_f32 + y.sin();
        let b = y * 3.868_f32 - x.cos();
        let mut acc = Accumulator767::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.524_f32 + y.sin();
        let b = y * 6.666_f32 - x.cos();
        let mut acc = Accumulator767::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.354_f32 + y.sin();
        let b = y * 7.169_f32 - x.cos();
        let mut acc = Accumulator767::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.799_f32 + y.sin();
        let b = y * 2.615_f32 - x.cos();
        let mut acc = Accumulator767::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.091_f32 + y.sin();
        let b = y * 7.385_f32 - x.cos();
        let mut acc = Accumulator767::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.331_f32 + y.sin();
        let b = y * 5.856_f32 - x.cos();
        let mut acc = Accumulator767::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.46_f32 + y.sin();
        let b = y * 8.054_f32 - x.cos();
        let mut acc = Accumulator767::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.23_f32 + y.sin();
        let b = y * 6.881_f32 - x.cos();
        let mut acc = Accumulator767::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_767(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_767() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_767(total as u64) % 997) as f32;
        total
    }
}

pub mod m768 {
    use super::*;

    pub struct Accumulator768<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator768<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.357_f32 + y.sin();
        let b = y * 2.616_f32 - x.cos();
        let mut acc = Accumulator768::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.003_f32 + y.sin();
        let b = y * 9.612_f32 - x.cos();
        let mut acc = Accumulator768::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.275_f32 + y.sin();
        let b = y * 3.861_f32 - x.cos();
        let mut acc = Accumulator768::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.471_f32 + y.sin();
        let b = y * 6.985_f32 - x.cos();
        let mut acc = Accumulator768::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.401_f32 + y.sin();
        let b = y * 3.483_f32 - x.cos();
        let mut acc = Accumulator768::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.491_f32 + y.sin();
        let b = y * 4.527_f32 - x.cos();
        let mut acc = Accumulator768::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.791_f32 + y.sin();
        let b = y * 9.154_f32 - x.cos();
        let mut acc = Accumulator768::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.448_f32 + y.sin();
        let b = y * 7.976_f32 - x.cos();
        let mut acc = Accumulator768::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.689_f32 + y.sin();
        let b = y * 6.226_f32 - x.cos();
        let mut acc = Accumulator768::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.962_f32 + y.sin();
        let b = y * 2.452_f32 - x.cos();
        let mut acc = Accumulator768::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.692_f32 + y.sin();
        let b = y * 7.079_f32 - x.cos();
        let mut acc = Accumulator768::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.907_f32 + y.sin();
        let b = y * 2.131_f32 - x.cos();
        let mut acc = Accumulator768::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.484_f32 + y.sin();
        let b = y * 3.886_f32 - x.cos();
        let mut acc = Accumulator768::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.264_f32 + y.sin();
        let b = y * 3.974_f32 - x.cos();
        let mut acc = Accumulator768::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.543_f32 + y.sin();
        let b = y * 3.615_f32 - x.cos();
        let mut acc = Accumulator768::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.732_f32 + y.sin();
        let b = y * 4.834_f32 - x.cos();
        let mut acc = Accumulator768::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.557_f32 + y.sin();
        let b = y * 6.297_f32 - x.cos();
        let mut acc = Accumulator768::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.029_f32 + y.sin();
        let b = y * 7.326_f32 - x.cos();
        let mut acc = Accumulator768::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.83_f32 + y.sin();
        let b = y * 7.157_f32 - x.cos();
        let mut acc = Accumulator768::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.264_f32 + y.sin();
        let b = y * 6.902_f32 - x.cos();
        let mut acc = Accumulator768::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_768(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(768u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_768() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_768(total as u64) % 997) as f32;
        total
    }
}

pub mod m769 {
    use super::*;

    pub struct Accumulator769<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator769<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.056_f32 + y.sin();
        let b = y * 8.851_f32 - x.cos();
        let mut acc = Accumulator769::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.298_f32 + y.sin();
        let b = y * 4.169_f32 - x.cos();
        let mut acc = Accumulator769::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.543_f32 + y.sin();
        let b = y * 7.917_f32 - x.cos();
        let mut acc = Accumulator769::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.521_f32 + y.sin();
        let b = y * 2.36_f32 - x.cos();
        let mut acc = Accumulator769::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.242_f32 + y.sin();
        let b = y * 7.491_f32 - x.cos();
        let mut acc = Accumulator769::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.377_f32 + y.sin();
        let b = y * 0.983_f32 - x.cos();
        let mut acc = Accumulator769::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.383_f32 + y.sin();
        let b = y * 4.039_f32 - x.cos();
        let mut acc = Accumulator769::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.351_f32 + y.sin();
        let b = y * 7.14_f32 - x.cos();
        let mut acc = Accumulator769::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.493_f32 + y.sin();
        let b = y * 2.358_f32 - x.cos();
        let mut acc = Accumulator769::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.3_f32 + y.sin();
        let b = y * 2.969_f32 - x.cos();
        let mut acc = Accumulator769::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.771_f32 + y.sin();
        let b = y * 6.355_f32 - x.cos();
        let mut acc = Accumulator769::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.216_f32 + y.sin();
        let b = y * 1.535_f32 - x.cos();
        let mut acc = Accumulator769::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.66_f32 + y.sin();
        let b = y * 6.433_f32 - x.cos();
        let mut acc = Accumulator769::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.234_f32 + y.sin();
        let b = y * 5.693_f32 - x.cos();
        let mut acc = Accumulator769::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.88_f32 + y.sin();
        let b = y * 8.233_f32 - x.cos();
        let mut acc = Accumulator769::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.275_f32 + y.sin();
        let b = y * 7.439_f32 - x.cos();
        let mut acc = Accumulator769::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.722_f32 + y.sin();
        let b = y * 5.315_f32 - x.cos();
        let mut acc = Accumulator769::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.662_f32 + y.sin();
        let b = y * 6.488_f32 - x.cos();
        let mut acc = Accumulator769::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.096_f32 + y.sin();
        let b = y * 6.957_f32 - x.cos();
        let mut acc = Accumulator769::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.052_f32 + y.sin();
        let b = y * 1.177_f32 - x.cos();
        let mut acc = Accumulator769::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_769(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_769() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_769(total as u64) % 997) as f32;
        total
    }
}

pub mod m770 {
    use super::*;

    pub struct Accumulator770<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator770<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.269_f32 + y.sin();
        let b = y * 4.94_f32 - x.cos();
        let mut acc = Accumulator770::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.042_f32 + y.sin();
        let b = y * 6.88_f32 - x.cos();
        let mut acc = Accumulator770::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.064_f32 + y.sin();
        let b = y * 2.59_f32 - x.cos();
        let mut acc = Accumulator770::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.153_f32 + y.sin();
        let b = y * 2.816_f32 - x.cos();
        let mut acc = Accumulator770::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.383_f32 + y.sin();
        let b = y * 7.212_f32 - x.cos();
        let mut acc = Accumulator770::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.071_f32 + y.sin();
        let b = y * 7.861_f32 - x.cos();
        let mut acc = Accumulator770::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.763_f32 + y.sin();
        let b = y * 6.534_f32 - x.cos();
        let mut acc = Accumulator770::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.768_f32 + y.sin();
        let b = y * 0.404_f32 - x.cos();
        let mut acc = Accumulator770::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.843_f32 + y.sin();
        let b = y * 2.023_f32 - x.cos();
        let mut acc = Accumulator770::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.241_f32 + y.sin();
        let b = y * 0.908_f32 - x.cos();
        let mut acc = Accumulator770::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.567_f32 + y.sin();
        let b = y * 3.206_f32 - x.cos();
        let mut acc = Accumulator770::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.021_f32 + y.sin();
        let b = y * 1.669_f32 - x.cos();
        let mut acc = Accumulator770::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.776_f32 + y.sin();
        let b = y * 8.988_f32 - x.cos();
        let mut acc = Accumulator770::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.308_f32 + y.sin();
        let b = y * 8.874_f32 - x.cos();
        let mut acc = Accumulator770::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.083_f32 + y.sin();
        let b = y * 4.323_f32 - x.cos();
        let mut acc = Accumulator770::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.279_f32 + y.sin();
        let b = y * 6.362_f32 - x.cos();
        let mut acc = Accumulator770::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.443_f32 + y.sin();
        let b = y * 0.416_f32 - x.cos();
        let mut acc = Accumulator770::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.682_f32 + y.sin();
        let b = y * 2.746_f32 - x.cos();
        let mut acc = Accumulator770::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.647_f32 + y.sin();
        let b = y * 6.892_f32 - x.cos();
        let mut acc = Accumulator770::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.379_f32 + y.sin();
        let b = y * 7.802_f32 - x.cos();
        let mut acc = Accumulator770::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_770(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_770() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_770(total as u64) % 997) as f32;
        total
    }
}

pub mod m771 {
    use super::*;

    pub struct Accumulator771<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator771<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.799_f32 + y.sin();
        let b = y * 6.942_f32 - x.cos();
        let mut acc = Accumulator771::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.828_f32 + y.sin();
        let b = y * 4.055_f32 - x.cos();
        let mut acc = Accumulator771::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.323_f32 + y.sin();
        let b = y * 5.481_f32 - x.cos();
        let mut acc = Accumulator771::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.618_f32 + y.sin();
        let b = y * 4.426_f32 - x.cos();
        let mut acc = Accumulator771::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.558_f32 + y.sin();
        let b = y * 0.889_f32 - x.cos();
        let mut acc = Accumulator771::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.523_f32 + y.sin();
        let b = y * 7.163_f32 - x.cos();
        let mut acc = Accumulator771::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.149_f32 + y.sin();
        let b = y * 8.369_f32 - x.cos();
        let mut acc = Accumulator771::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.249_f32 + y.sin();
        let b = y * 7.041_f32 - x.cos();
        let mut acc = Accumulator771::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.942_f32 + y.sin();
        let b = y * 0.594_f32 - x.cos();
        let mut acc = Accumulator771::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.06_f32 + y.sin();
        let b = y * 7.038_f32 - x.cos();
        let mut acc = Accumulator771::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.774_f32 + y.sin();
        let b = y * 6.103_f32 - x.cos();
        let mut acc = Accumulator771::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.385_f32 + y.sin();
        let b = y * 0.371_f32 - x.cos();
        let mut acc = Accumulator771::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.445_f32 + y.sin();
        let b = y * 4.386_f32 - x.cos();
        let mut acc = Accumulator771::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.66_f32 + y.sin();
        let b = y * 3.276_f32 - x.cos();
        let mut acc = Accumulator771::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.785_f32 + y.sin();
        let b = y * 1.25_f32 - x.cos();
        let mut acc = Accumulator771::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.722_f32 + y.sin();
        let b = y * 0.927_f32 - x.cos();
        let mut acc = Accumulator771::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.914_f32 + y.sin();
        let b = y * 3.3_f32 - x.cos();
        let mut acc = Accumulator771::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.327_f32 + y.sin();
        let b = y * 7.866_f32 - x.cos();
        let mut acc = Accumulator771::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.685_f32 + y.sin();
        let b = y * 9.417_f32 - x.cos();
        let mut acc = Accumulator771::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.491_f32 + y.sin();
        let b = y * 5.24_f32 - x.cos();
        let mut acc = Accumulator771::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_771(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m771-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_771() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_771(total as u64) % 997) as f32;
        total
    }
}

pub mod m772 {
    use super::*;

    pub struct Accumulator772<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator772<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.326_f32 + y.sin();
        let b = y * 0.882_f32 - x.cos();
        let mut acc = Accumulator772::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.618_f32 + y.sin();
        let b = y * 0.106_f32 - x.cos();
        let mut acc = Accumulator772::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.965_f32 + y.sin();
        let b = y * 2.469_f32 - x.cos();
        let mut acc = Accumulator772::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.922_f32 + y.sin();
        let b = y * 6.232_f32 - x.cos();
        let mut acc = Accumulator772::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.841_f32 + y.sin();
        let b = y * 7.415_f32 - x.cos();
        let mut acc = Accumulator772::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.334_f32 + y.sin();
        let b = y * 5.568_f32 - x.cos();
        let mut acc = Accumulator772::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.43_f32 + y.sin();
        let b = y * 4.928_f32 - x.cos();
        let mut acc = Accumulator772::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.301_f32 + y.sin();
        let b = y * 2.376_f32 - x.cos();
        let mut acc = Accumulator772::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.129_f32 + y.sin();
        let b = y * 8.011_f32 - x.cos();
        let mut acc = Accumulator772::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.566_f32 + y.sin();
        let b = y * 3.737_f32 - x.cos();
        let mut acc = Accumulator772::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.098_f32 + y.sin();
        let b = y * 2.862_f32 - x.cos();
        let mut acc = Accumulator772::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.856_f32 + y.sin();
        let b = y * 3.922_f32 - x.cos();
        let mut acc = Accumulator772::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.936_f32 + y.sin();
        let b = y * 9.323_f32 - x.cos();
        let mut acc = Accumulator772::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.525_f32 + y.sin();
        let b = y * 7.772_f32 - x.cos();
        let mut acc = Accumulator772::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.684_f32 + y.sin();
        let b = y * 9.694_f32 - x.cos();
        let mut acc = Accumulator772::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.917_f32 + y.sin();
        let b = y * 8.301_f32 - x.cos();
        let mut acc = Accumulator772::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.177_f32 + y.sin();
        let b = y * 6.642_f32 - x.cos();
        let mut acc = Accumulator772::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.338_f32 + y.sin();
        let b = y * 0.926_f32 - x.cos();
        let mut acc = Accumulator772::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.97_f32 + y.sin();
        let b = y * 4.916_f32 - x.cos();
        let mut acc = Accumulator772::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.522_f32 + y.sin();
        let b = y * 9.175_f32 - x.cos();
        let mut acc = Accumulator772::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_772(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_772() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_772(total as u64) % 997) as f32;
        total
    }
}

pub mod m773 {
    use super::*;

    pub struct Accumulator773<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator773<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.98_f32 + y.sin();
        let b = y * 2.065_f32 - x.cos();
        let mut acc = Accumulator773::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.826_f32 + y.sin();
        let b = y * 9.566_f32 - x.cos();
        let mut acc = Accumulator773::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.454_f32 + y.sin();
        let b = y * 3.124_f32 - x.cos();
        let mut acc = Accumulator773::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.596_f32 + y.sin();
        let b = y * 9.505_f32 - x.cos();
        let mut acc = Accumulator773::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.47_f32 + y.sin();
        let b = y * 9.307_f32 - x.cos();
        let mut acc = Accumulator773::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.569_f32 + y.sin();
        let b = y * 8.354_f32 - x.cos();
        let mut acc = Accumulator773::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.784_f32 + y.sin();
        let b = y * 1.163_f32 - x.cos();
        let mut acc = Accumulator773::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.894_f32 + y.sin();
        let b = y * 2.686_f32 - x.cos();
        let mut acc = Accumulator773::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.622_f32 + y.sin();
        let b = y * 6.363_f32 - x.cos();
        let mut acc = Accumulator773::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.724_f32 + y.sin();
        let b = y * 3.784_f32 - x.cos();
        let mut acc = Accumulator773::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.701_f32 + y.sin();
        let b = y * 8.55_f32 - x.cos();
        let mut acc = Accumulator773::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.28_f32 + y.sin();
        let b = y * 1.543_f32 - x.cos();
        let mut acc = Accumulator773::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.109_f32 + y.sin();
        let b = y * 1.878_f32 - x.cos();
        let mut acc = Accumulator773::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.941_f32 + y.sin();
        let b = y * 0.803_f32 - x.cos();
        let mut acc = Accumulator773::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.756_f32 + y.sin();
        let b = y * 3.76_f32 - x.cos();
        let mut acc = Accumulator773::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.158_f32 + y.sin();
        let b = y * 5.258_f32 - x.cos();
        let mut acc = Accumulator773::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.821_f32 + y.sin();
        let b = y * 3.866_f32 - x.cos();
        let mut acc = Accumulator773::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.662_f32 + y.sin();
        let b = y * 9.751_f32 - x.cos();
        let mut acc = Accumulator773::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.259_f32 + y.sin();
        let b = y * 8.287_f32 - x.cos();
        let mut acc = Accumulator773::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.766_f32 + y.sin();
        let b = y * 3.424_f32 - x.cos();
        let mut acc = Accumulator773::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_773(seed: u64) -> u64 {
        let re = Regex::new(r"m773-(\d+)").unwrap();
        let hay = format!("m773-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_773() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_773(total as u64) % 997) as f32;
        total
    }
}

pub mod m774 {
    use super::*;

    pub struct Accumulator774<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator774<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.376_f32 + y.sin();
        let b = y * 8.075_f32 - x.cos();
        let mut acc = Accumulator774::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.668_f32 + y.sin();
        let b = y * 4.566_f32 - x.cos();
        let mut acc = Accumulator774::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.669_f32 + y.sin();
        let b = y * 8.65_f32 - x.cos();
        let mut acc = Accumulator774::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.159_f32 + y.sin();
        let b = y * 0.96_f32 - x.cos();
        let mut acc = Accumulator774::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.29_f32 + y.sin();
        let b = y * 0.556_f32 - x.cos();
        let mut acc = Accumulator774::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.446_f32 + y.sin();
        let b = y * 1.917_f32 - x.cos();
        let mut acc = Accumulator774::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.983_f32 + y.sin();
        let b = y * 5.293_f32 - x.cos();
        let mut acc = Accumulator774::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.824_f32 + y.sin();
        let b = y * 3.978_f32 - x.cos();
        let mut acc = Accumulator774::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.403_f32 + y.sin();
        let b = y * 3.696_f32 - x.cos();
        let mut acc = Accumulator774::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.193_f32 + y.sin();
        let b = y * 5.814_f32 - x.cos();
        let mut acc = Accumulator774::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.662_f32 + y.sin();
        let b = y * 8.23_f32 - x.cos();
        let mut acc = Accumulator774::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.566_f32 + y.sin();
        let b = y * 4.228_f32 - x.cos();
        let mut acc = Accumulator774::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.179_f32 + y.sin();
        let b = y * 3.576_f32 - x.cos();
        let mut acc = Accumulator774::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.458_f32 + y.sin();
        let b = y * 0.909_f32 - x.cos();
        let mut acc = Accumulator774::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.822_f32 + y.sin();
        let b = y * 1.456_f32 - x.cos();
        let mut acc = Accumulator774::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.171_f32 + y.sin();
        let b = y * 8.444_f32 - x.cos();
        let mut acc = Accumulator774::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.252_f32 + y.sin();
        let b = y * 9.553_f32 - x.cos();
        let mut acc = Accumulator774::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.139_f32 + y.sin();
        let b = y * 6.188_f32 - x.cos();
        let mut acc = Accumulator774::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.749_f32 + y.sin();
        let b = y * 5.411_f32 - x.cos();
        let mut acc = Accumulator774::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.542_f32 + y.sin();
        let b = y * 6.941_f32 - x.cos();
        let mut acc = Accumulator774::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_774(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_774() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_774(total as u64) % 997) as f32;
        total
    }
}

pub mod m775 {
    use super::*;

    pub struct Accumulator775<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator775<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.805_f32 + y.sin();
        let b = y * 9.023_f32 - x.cos();
        let mut acc = Accumulator775::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.293_f32 + y.sin();
        let b = y * 1.21_f32 - x.cos();
        let mut acc = Accumulator775::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.888_f32 + y.sin();
        let b = y * 7.071_f32 - x.cos();
        let mut acc = Accumulator775::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.251_f32 + y.sin();
        let b = y * 0.481_f32 - x.cos();
        let mut acc = Accumulator775::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.841_f32 + y.sin();
        let b = y * 5.953_f32 - x.cos();
        let mut acc = Accumulator775::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.124_f32 + y.sin();
        let b = y * 2.462_f32 - x.cos();
        let mut acc = Accumulator775::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.369_f32 + y.sin();
        let b = y * 6.757_f32 - x.cos();
        let mut acc = Accumulator775::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.505_f32 + y.sin();
        let b = y * 0.726_f32 - x.cos();
        let mut acc = Accumulator775::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.908_f32 + y.sin();
        let b = y * 5.223_f32 - x.cos();
        let mut acc = Accumulator775::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.283_f32 + y.sin();
        let b = y * 7.546_f32 - x.cos();
        let mut acc = Accumulator775::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.122_f32 + y.sin();
        let b = y * 2.264_f32 - x.cos();
        let mut acc = Accumulator775::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.13_f32 + y.sin();
        let b = y * 7.885_f32 - x.cos();
        let mut acc = Accumulator775::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.714_f32 + y.sin();
        let b = y * 5.188_f32 - x.cos();
        let mut acc = Accumulator775::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.368_f32 + y.sin();
        let b = y * 3.805_f32 - x.cos();
        let mut acc = Accumulator775::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.01_f32 + y.sin();
        let b = y * 4.998_f32 - x.cos();
        let mut acc = Accumulator775::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.842_f32 + y.sin();
        let b = y * 4.258_f32 - x.cos();
        let mut acc = Accumulator775::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.802_f32 + y.sin();
        let b = y * 5.281_f32 - x.cos();
        let mut acc = Accumulator775::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.08_f32 + y.sin();
        let b = y * 6.875_f32 - x.cos();
        let mut acc = Accumulator775::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.967_f32 + y.sin();
        let b = y * 1.684_f32 - x.cos();
        let mut acc = Accumulator775::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.698_f32 + y.sin();
        let b = y * 5.019_f32 - x.cos();
        let mut acc = Accumulator775::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_775(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(775u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_775() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_775(total as u64) % 997) as f32;
        total
    }
}

pub mod m776 {
    use super::*;

    pub struct Accumulator776<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator776<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.986_f32 + y.sin();
        let b = y * 2.845_f32 - x.cos();
        let mut acc = Accumulator776::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.112_f32 + y.sin();
        let b = y * 8.669_f32 - x.cos();
        let mut acc = Accumulator776::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.308_f32 + y.sin();
        let b = y * 8.152_f32 - x.cos();
        let mut acc = Accumulator776::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.564_f32 + y.sin();
        let b = y * 7.417_f32 - x.cos();
        let mut acc = Accumulator776::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.603_f32 + y.sin();
        let b = y * 6.818_f32 - x.cos();
        let mut acc = Accumulator776::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.342_f32 + y.sin();
        let b = y * 5.192_f32 - x.cos();
        let mut acc = Accumulator776::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.501_f32 + y.sin();
        let b = y * 3.983_f32 - x.cos();
        let mut acc = Accumulator776::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.264_f32 + y.sin();
        let b = y * 6.198_f32 - x.cos();
        let mut acc = Accumulator776::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.557_f32 + y.sin();
        let b = y * 4.583_f32 - x.cos();
        let mut acc = Accumulator776::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.134_f32 + y.sin();
        let b = y * 4.711_f32 - x.cos();
        let mut acc = Accumulator776::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.736_f32 + y.sin();
        let b = y * 5.695_f32 - x.cos();
        let mut acc = Accumulator776::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.151_f32 + y.sin();
        let b = y * 2.29_f32 - x.cos();
        let mut acc = Accumulator776::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.116_f32 + y.sin();
        let b = y * 0.164_f32 - x.cos();
        let mut acc = Accumulator776::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.107_f32 + y.sin();
        let b = y * 3.143_f32 - x.cos();
        let mut acc = Accumulator776::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.386_f32 + y.sin();
        let b = y * 4.378_f32 - x.cos();
        let mut acc = Accumulator776::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.933_f32 + y.sin();
        let b = y * 3.223_f32 - x.cos();
        let mut acc = Accumulator776::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.592_f32 + y.sin();
        let b = y * 1.936_f32 - x.cos();
        let mut acc = Accumulator776::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.26_f32 + y.sin();
        let b = y * 9.628_f32 - x.cos();
        let mut acc = Accumulator776::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.476_f32 + y.sin();
        let b = y * 1.067_f32 - x.cos();
        let mut acc = Accumulator776::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.654_f32 + y.sin();
        let b = y * 9.446_f32 - x.cos();
        let mut acc = Accumulator776::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_776(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_776() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_776(total as u64) % 997) as f32;
        total
    }
}

pub mod m777 {
    use super::*;

    pub struct Accumulator777<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator777<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.846_f32 + y.sin();
        let b = y * 3.586_f32 - x.cos();
        let mut acc = Accumulator777::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.437_f32 + y.sin();
        let b = y * 0.836_f32 - x.cos();
        let mut acc = Accumulator777::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.638_f32 + y.sin();
        let b = y * 7.869_f32 - x.cos();
        let mut acc = Accumulator777::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.682_f32 + y.sin();
        let b = y * 7.015_f32 - x.cos();
        let mut acc = Accumulator777::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.63_f32 + y.sin();
        let b = y * 1.702_f32 - x.cos();
        let mut acc = Accumulator777::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.177_f32 + y.sin();
        let b = y * 2.227_f32 - x.cos();
        let mut acc = Accumulator777::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.229_f32 + y.sin();
        let b = y * 7.848_f32 - x.cos();
        let mut acc = Accumulator777::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.589_f32 + y.sin();
        let b = y * 9.857_f32 - x.cos();
        let mut acc = Accumulator777::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.062_f32 + y.sin();
        let b = y * 7.438_f32 - x.cos();
        let mut acc = Accumulator777::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.851_f32 + y.sin();
        let b = y * 6.332_f32 - x.cos();
        let mut acc = Accumulator777::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.27_f32 + y.sin();
        let b = y * 0.664_f32 - x.cos();
        let mut acc = Accumulator777::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.777_f32 + y.sin();
        let b = y * 5.67_f32 - x.cos();
        let mut acc = Accumulator777::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.846_f32 + y.sin();
        let b = y * 3.197_f32 - x.cos();
        let mut acc = Accumulator777::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.607_f32 + y.sin();
        let b = y * 9.47_f32 - x.cos();
        let mut acc = Accumulator777::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.14_f32 + y.sin();
        let b = y * 0.493_f32 - x.cos();
        let mut acc = Accumulator777::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.602_f32 + y.sin();
        let b = y * 5.46_f32 - x.cos();
        let mut acc = Accumulator777::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.691_f32 + y.sin();
        let b = y * 3.265_f32 - x.cos();
        let mut acc = Accumulator777::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.218_f32 + y.sin();
        let b = y * 6.4_f32 - x.cos();
        let mut acc = Accumulator777::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.036_f32 + y.sin();
        let b = y * 8.167_f32 - x.cos();
        let mut acc = Accumulator777::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.284_f32 + y.sin();
        let b = y * 5.753_f32 - x.cos();
        let mut acc = Accumulator777::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_777(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_777() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_777(total as u64) % 997) as f32;
        total
    }
}

pub mod m778 {
    use super::*;

    pub struct Accumulator778<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator778<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.838_f32 + y.sin();
        let b = y * 9.022_f32 - x.cos();
        let mut acc = Accumulator778::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.754_f32 + y.sin();
        let b = y * 3.771_f32 - x.cos();
        let mut acc = Accumulator778::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.307_f32 + y.sin();
        let b = y * 2.654_f32 - x.cos();
        let mut acc = Accumulator778::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.515_f32 + y.sin();
        let b = y * 5.028_f32 - x.cos();
        let mut acc = Accumulator778::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.882_f32 + y.sin();
        let b = y * 3.1_f32 - x.cos();
        let mut acc = Accumulator778::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.691_f32 + y.sin();
        let b = y * 7.525_f32 - x.cos();
        let mut acc = Accumulator778::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.133_f32 + y.sin();
        let b = y * 2.516_f32 - x.cos();
        let mut acc = Accumulator778::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.627_f32 + y.sin();
        let b = y * 3.349_f32 - x.cos();
        let mut acc = Accumulator778::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.29_f32 + y.sin();
        let b = y * 9.391_f32 - x.cos();
        let mut acc = Accumulator778::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.629_f32 + y.sin();
        let b = y * 1.473_f32 - x.cos();
        let mut acc = Accumulator778::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.229_f32 + y.sin();
        let b = y * 5.027_f32 - x.cos();
        let mut acc = Accumulator778::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.274_f32 + y.sin();
        let b = y * 2.991_f32 - x.cos();
        let mut acc = Accumulator778::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.223_f32 + y.sin();
        let b = y * 7.115_f32 - x.cos();
        let mut acc = Accumulator778::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.926_f32 + y.sin();
        let b = y * 8.805_f32 - x.cos();
        let mut acc = Accumulator778::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.907_f32 + y.sin();
        let b = y * 4.742_f32 - x.cos();
        let mut acc = Accumulator778::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.368_f32 + y.sin();
        let b = y * 3.651_f32 - x.cos();
        let mut acc = Accumulator778::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.774_f32 + y.sin();
        let b = y * 6.47_f32 - x.cos();
        let mut acc = Accumulator778::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.051_f32 + y.sin();
        let b = y * 4.496_f32 - x.cos();
        let mut acc = Accumulator778::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.166_f32 + y.sin();
        let b = y * 8.354_f32 - x.cos();
        let mut acc = Accumulator778::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.317_f32 + y.sin();
        let b = y * 2.051_f32 - x.cos();
        let mut acc = Accumulator778::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_778(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m778-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_778() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_778(total as u64) % 997) as f32;
        total
    }
}

pub mod m779 {
    use super::*;

    pub struct Accumulator779<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator779<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.091_f32 + y.sin();
        let b = y * 8.903_f32 - x.cos();
        let mut acc = Accumulator779::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.094_f32 + y.sin();
        let b = y * 3.043_f32 - x.cos();
        let mut acc = Accumulator779::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.843_f32 + y.sin();
        let b = y * 6.202_f32 - x.cos();
        let mut acc = Accumulator779::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.256_f32 + y.sin();
        let b = y * 7.03_f32 - x.cos();
        let mut acc = Accumulator779::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.231_f32 + y.sin();
        let b = y * 2.179_f32 - x.cos();
        let mut acc = Accumulator779::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.154_f32 + y.sin();
        let b = y * 2.127_f32 - x.cos();
        let mut acc = Accumulator779::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.072_f32 + y.sin();
        let b = y * 6.119_f32 - x.cos();
        let mut acc = Accumulator779::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.567_f32 + y.sin();
        let b = y * 2.434_f32 - x.cos();
        let mut acc = Accumulator779::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.015_f32 + y.sin();
        let b = y * 2.917_f32 - x.cos();
        let mut acc = Accumulator779::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.717_f32 + y.sin();
        let b = y * 4.296_f32 - x.cos();
        let mut acc = Accumulator779::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.096_f32 + y.sin();
        let b = y * 9.753_f32 - x.cos();
        let mut acc = Accumulator779::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.543_f32 + y.sin();
        let b = y * 5.426_f32 - x.cos();
        let mut acc = Accumulator779::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.577_f32 + y.sin();
        let b = y * 2.867_f32 - x.cos();
        let mut acc = Accumulator779::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.939_f32 + y.sin();
        let b = y * 3.203_f32 - x.cos();
        let mut acc = Accumulator779::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.593_f32 + y.sin();
        let b = y * 4.033_f32 - x.cos();
        let mut acc = Accumulator779::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.546_f32 + y.sin();
        let b = y * 9.697_f32 - x.cos();
        let mut acc = Accumulator779::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.953_f32 + y.sin();
        let b = y * 5.548_f32 - x.cos();
        let mut acc = Accumulator779::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.384_f32 + y.sin();
        let b = y * 2.435_f32 - x.cos();
        let mut acc = Accumulator779::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.706_f32 + y.sin();
        let b = y * 1.404_f32 - x.cos();
        let mut acc = Accumulator779::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.559_f32 + y.sin();
        let b = y * 2.043_f32 - x.cos();
        let mut acc = Accumulator779::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_779(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_779() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_779(total as u64) % 997) as f32;
        total
    }
}

pub mod m780 {
    use super::*;

    pub struct Accumulator780<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator780<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.365_f32 + y.sin();
        let b = y * 3.612_f32 - x.cos();
        let mut acc = Accumulator780::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.473_f32 + y.sin();
        let b = y * 1.572_f32 - x.cos();
        let mut acc = Accumulator780::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.816_f32 + y.sin();
        let b = y * 3.721_f32 - x.cos();
        let mut acc = Accumulator780::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.007_f32 + y.sin();
        let b = y * 8.856_f32 - x.cos();
        let mut acc = Accumulator780::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.895_f32 + y.sin();
        let b = y * 1.728_f32 - x.cos();
        let mut acc = Accumulator780::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.355_f32 + y.sin();
        let b = y * 5.729_f32 - x.cos();
        let mut acc = Accumulator780::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.573_f32 + y.sin();
        let b = y * 6.71_f32 - x.cos();
        let mut acc = Accumulator780::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.653_f32 + y.sin();
        let b = y * 3.947_f32 - x.cos();
        let mut acc = Accumulator780::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.874_f32 + y.sin();
        let b = y * 1.25_f32 - x.cos();
        let mut acc = Accumulator780::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.669_f32 + y.sin();
        let b = y * 3.92_f32 - x.cos();
        let mut acc = Accumulator780::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.088_f32 + y.sin();
        let b = y * 6.88_f32 - x.cos();
        let mut acc = Accumulator780::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.298_f32 + y.sin();
        let b = y * 8.323_f32 - x.cos();
        let mut acc = Accumulator780::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.158_f32 + y.sin();
        let b = y * 8.931_f32 - x.cos();
        let mut acc = Accumulator780::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.927_f32 + y.sin();
        let b = y * 1.574_f32 - x.cos();
        let mut acc = Accumulator780::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.245_f32 + y.sin();
        let b = y * 3.75_f32 - x.cos();
        let mut acc = Accumulator780::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.164_f32 + y.sin();
        let b = y * 6.933_f32 - x.cos();
        let mut acc = Accumulator780::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.852_f32 + y.sin();
        let b = y * 8.743_f32 - x.cos();
        let mut acc = Accumulator780::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.604_f32 + y.sin();
        let b = y * 2.716_f32 - x.cos();
        let mut acc = Accumulator780::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.053_f32 + y.sin();
        let b = y * 5.0_f32 - x.cos();
        let mut acc = Accumulator780::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.11_f32 + y.sin();
        let b = y * 7.944_f32 - x.cos();
        let mut acc = Accumulator780::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_780(seed: u64) -> u64 {
        let re = Regex::new(r"m780-(\d+)").unwrap();
        let hay = format!("m780-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_780() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_780(total as u64) % 997) as f32;
        total
    }
}

pub mod m781 {
    use super::*;

    pub struct Accumulator781<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator781<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.072_f32 + y.sin();
        let b = y * 7.413_f32 - x.cos();
        let mut acc = Accumulator781::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.264_f32 + y.sin();
        let b = y * 5.084_f32 - x.cos();
        let mut acc = Accumulator781::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.084_f32 + y.sin();
        let b = y * 6.991_f32 - x.cos();
        let mut acc = Accumulator781::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.973_f32 + y.sin();
        let b = y * 3.847_f32 - x.cos();
        let mut acc = Accumulator781::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.433_f32 + y.sin();
        let b = y * 0.348_f32 - x.cos();
        let mut acc = Accumulator781::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.002_f32 + y.sin();
        let b = y * 3.37_f32 - x.cos();
        let mut acc = Accumulator781::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.429_f32 + y.sin();
        let b = y * 6.249_f32 - x.cos();
        let mut acc = Accumulator781::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.901_f32 + y.sin();
        let b = y * 6.873_f32 - x.cos();
        let mut acc = Accumulator781::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.133_f32 + y.sin();
        let b = y * 2.424_f32 - x.cos();
        let mut acc = Accumulator781::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.346_f32 + y.sin();
        let b = y * 1.786_f32 - x.cos();
        let mut acc = Accumulator781::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.927_f32 + y.sin();
        let b = y * 6.138_f32 - x.cos();
        let mut acc = Accumulator781::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.172_f32 + y.sin();
        let b = y * 4.363_f32 - x.cos();
        let mut acc = Accumulator781::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.92_f32 + y.sin();
        let b = y * 9.364_f32 - x.cos();
        let mut acc = Accumulator781::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.804_f32 + y.sin();
        let b = y * 2.36_f32 - x.cos();
        let mut acc = Accumulator781::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.812_f32 + y.sin();
        let b = y * 8.19_f32 - x.cos();
        let mut acc = Accumulator781::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.044_f32 + y.sin();
        let b = y * 2.403_f32 - x.cos();
        let mut acc = Accumulator781::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.348_f32 + y.sin();
        let b = y * 4.005_f32 - x.cos();
        let mut acc = Accumulator781::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.296_f32 + y.sin();
        let b = y * 0.215_f32 - x.cos();
        let mut acc = Accumulator781::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.705_f32 + y.sin();
        let b = y * 4.443_f32 - x.cos();
        let mut acc = Accumulator781::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.922_f32 + y.sin();
        let b = y * 7.913_f32 - x.cos();
        let mut acc = Accumulator781::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_781(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_781() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_781(total as u64) % 997) as f32;
        total
    }
}

pub mod m782 {
    use super::*;

    pub struct Accumulator782<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator782<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.085_f32 + y.sin();
        let b = y * 5.074_f32 - x.cos();
        let mut acc = Accumulator782::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.453_f32 + y.sin();
        let b = y * 2.973_f32 - x.cos();
        let mut acc = Accumulator782::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.051_f32 + y.sin();
        let b = y * 8.031_f32 - x.cos();
        let mut acc = Accumulator782::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.02_f32 + y.sin();
        let b = y * 4.861_f32 - x.cos();
        let mut acc = Accumulator782::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.144_f32 + y.sin();
        let b = y * 8.504_f32 - x.cos();
        let mut acc = Accumulator782::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.933_f32 + y.sin();
        let b = y * 2.59_f32 - x.cos();
        let mut acc = Accumulator782::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.546_f32 + y.sin();
        let b = y * 3.241_f32 - x.cos();
        let mut acc = Accumulator782::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.964_f32 + y.sin();
        let b = y * 5.599_f32 - x.cos();
        let mut acc = Accumulator782::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.55_f32 + y.sin();
        let b = y * 0.568_f32 - x.cos();
        let mut acc = Accumulator782::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.377_f32 + y.sin();
        let b = y * 4.85_f32 - x.cos();
        let mut acc = Accumulator782::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.029_f32 + y.sin();
        let b = y * 8.995_f32 - x.cos();
        let mut acc = Accumulator782::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.502_f32 + y.sin();
        let b = y * 6.177_f32 - x.cos();
        let mut acc = Accumulator782::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.363_f32 + y.sin();
        let b = y * 0.828_f32 - x.cos();
        let mut acc = Accumulator782::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.234_f32 + y.sin();
        let b = y * 9.503_f32 - x.cos();
        let mut acc = Accumulator782::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.531_f32 + y.sin();
        let b = y * 5.479_f32 - x.cos();
        let mut acc = Accumulator782::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.136_f32 + y.sin();
        let b = y * 6.895_f32 - x.cos();
        let mut acc = Accumulator782::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.524_f32 + y.sin();
        let b = y * 2.039_f32 - x.cos();
        let mut acc = Accumulator782::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.745_f32 + y.sin();
        let b = y * 2.048_f32 - x.cos();
        let mut acc = Accumulator782::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.757_f32 + y.sin();
        let b = y * 8.131_f32 - x.cos();
        let mut acc = Accumulator782::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.448_f32 + y.sin();
        let b = y * 7.873_f32 - x.cos();
        let mut acc = Accumulator782::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_782(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(782u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_782() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_782(total as u64) % 997) as f32;
        total
    }
}

pub mod m783 {
    use super::*;

    pub struct Accumulator783<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator783<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.408_f32 + y.sin();
        let b = y * 8.909_f32 - x.cos();
        let mut acc = Accumulator783::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.77_f32 + y.sin();
        let b = y * 8.73_f32 - x.cos();
        let mut acc = Accumulator783::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.28_f32 + y.sin();
        let b = y * 8.171_f32 - x.cos();
        let mut acc = Accumulator783::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.787_f32 + y.sin();
        let b = y * 8.713_f32 - x.cos();
        let mut acc = Accumulator783::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.549_f32 + y.sin();
        let b = y * 2.454_f32 - x.cos();
        let mut acc = Accumulator783::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.874_f32 + y.sin();
        let b = y * 5.231_f32 - x.cos();
        let mut acc = Accumulator783::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.67_f32 + y.sin();
        let b = y * 5.95_f32 - x.cos();
        let mut acc = Accumulator783::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.539_f32 + y.sin();
        let b = y * 7.512_f32 - x.cos();
        let mut acc = Accumulator783::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.503_f32 + y.sin();
        let b = y * 2.156_f32 - x.cos();
        let mut acc = Accumulator783::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.384_f32 + y.sin();
        let b = y * 7.417_f32 - x.cos();
        let mut acc = Accumulator783::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.763_f32 + y.sin();
        let b = y * 1.956_f32 - x.cos();
        let mut acc = Accumulator783::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.729_f32 + y.sin();
        let b = y * 9.101_f32 - x.cos();
        let mut acc = Accumulator783::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.079_f32 + y.sin();
        let b = y * 8.699_f32 - x.cos();
        let mut acc = Accumulator783::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.133_f32 + y.sin();
        let b = y * 2.896_f32 - x.cos();
        let mut acc = Accumulator783::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.94_f32 + y.sin();
        let b = y * 5.314_f32 - x.cos();
        let mut acc = Accumulator783::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.26_f32 + y.sin();
        let b = y * 4.041_f32 - x.cos();
        let mut acc = Accumulator783::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.655_f32 + y.sin();
        let b = y * 4.753_f32 - x.cos();
        let mut acc = Accumulator783::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.348_f32 + y.sin();
        let b = y * 4.405_f32 - x.cos();
        let mut acc = Accumulator783::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.424_f32 + y.sin();
        let b = y * 7.728_f32 - x.cos();
        let mut acc = Accumulator783::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.866_f32 + y.sin();
        let b = y * 6.024_f32 - x.cos();
        let mut acc = Accumulator783::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_783(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_783() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_783(total as u64) % 997) as f32;
        total
    }
}

pub mod m784 {
    use super::*;

    pub struct Accumulator784<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator784<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.724_f32 + y.sin();
        let b = y * 9.363_f32 - x.cos();
        let mut acc = Accumulator784::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.865_f32 + y.sin();
        let b = y * 7.078_f32 - x.cos();
        let mut acc = Accumulator784::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.188_f32 + y.sin();
        let b = y * 4.197_f32 - x.cos();
        let mut acc = Accumulator784::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.301_f32 + y.sin();
        let b = y * 9.885_f32 - x.cos();
        let mut acc = Accumulator784::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.148_f32 + y.sin();
        let b = y * 5.399_f32 - x.cos();
        let mut acc = Accumulator784::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.686_f32 + y.sin();
        let b = y * 2.528_f32 - x.cos();
        let mut acc = Accumulator784::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.574_f32 + y.sin();
        let b = y * 9.657_f32 - x.cos();
        let mut acc = Accumulator784::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.865_f32 + y.sin();
        let b = y * 8.974_f32 - x.cos();
        let mut acc = Accumulator784::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.525_f32 + y.sin();
        let b = y * 2.406_f32 - x.cos();
        let mut acc = Accumulator784::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.314_f32 + y.sin();
        let b = y * 2.983_f32 - x.cos();
        let mut acc = Accumulator784::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.489_f32 + y.sin();
        let b = y * 1.663_f32 - x.cos();
        let mut acc = Accumulator784::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.406_f32 + y.sin();
        let b = y * 7.22_f32 - x.cos();
        let mut acc = Accumulator784::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.887_f32 + y.sin();
        let b = y * 8.528_f32 - x.cos();
        let mut acc = Accumulator784::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.145_f32 + y.sin();
        let b = y * 2.022_f32 - x.cos();
        let mut acc = Accumulator784::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.386_f32 + y.sin();
        let b = y * 3.917_f32 - x.cos();
        let mut acc = Accumulator784::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.941_f32 + y.sin();
        let b = y * 6.062_f32 - x.cos();
        let mut acc = Accumulator784::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.5_f32 + y.sin();
        let b = y * 4.504_f32 - x.cos();
        let mut acc = Accumulator784::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.214_f32 + y.sin();
        let b = y * 6.269_f32 - x.cos();
        let mut acc = Accumulator784::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.553_f32 + y.sin();
        let b = y * 8.977_f32 - x.cos();
        let mut acc = Accumulator784::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.197_f32 + y.sin();
        let b = y * 2.231_f32 - x.cos();
        let mut acc = Accumulator784::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_784(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_784() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_784(total as u64) % 997) as f32;
        total
    }
}

pub mod m785 {
    use super::*;

    pub struct Accumulator785<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator785<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.018_f32 + y.sin();
        let b = y * 8.677_f32 - x.cos();
        let mut acc = Accumulator785::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.116_f32 + y.sin();
        let b = y * 1.982_f32 - x.cos();
        let mut acc = Accumulator785::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.426_f32 + y.sin();
        let b = y * 5.182_f32 - x.cos();
        let mut acc = Accumulator785::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.843_f32 + y.sin();
        let b = y * 1.588_f32 - x.cos();
        let mut acc = Accumulator785::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.471_f32 + y.sin();
        let b = y * 6.041_f32 - x.cos();
        let mut acc = Accumulator785::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.833_f32 + y.sin();
        let b = y * 0.951_f32 - x.cos();
        let mut acc = Accumulator785::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.047_f32 + y.sin();
        let b = y * 3.565_f32 - x.cos();
        let mut acc = Accumulator785::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.897_f32 + y.sin();
        let b = y * 0.529_f32 - x.cos();
        let mut acc = Accumulator785::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.702_f32 + y.sin();
        let b = y * 0.313_f32 - x.cos();
        let mut acc = Accumulator785::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.298_f32 + y.sin();
        let b = y * 9.695_f32 - x.cos();
        let mut acc = Accumulator785::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.231_f32 + y.sin();
        let b = y * 0.531_f32 - x.cos();
        let mut acc = Accumulator785::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.259_f32 + y.sin();
        let b = y * 7.898_f32 - x.cos();
        let mut acc = Accumulator785::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.359_f32 + y.sin();
        let b = y * 8.046_f32 - x.cos();
        let mut acc = Accumulator785::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.03_f32 + y.sin();
        let b = y * 1.251_f32 - x.cos();
        let mut acc = Accumulator785::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.466_f32 + y.sin();
        let b = y * 5.025_f32 - x.cos();
        let mut acc = Accumulator785::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.354_f32 + y.sin();
        let b = y * 0.583_f32 - x.cos();
        let mut acc = Accumulator785::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.62_f32 + y.sin();
        let b = y * 4.74_f32 - x.cos();
        let mut acc = Accumulator785::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.681_f32 + y.sin();
        let b = y * 1.195_f32 - x.cos();
        let mut acc = Accumulator785::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.438_f32 + y.sin();
        let b = y * 0.455_f32 - x.cos();
        let mut acc = Accumulator785::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.042_f32 + y.sin();
        let b = y * 0.922_f32 - x.cos();
        let mut acc = Accumulator785::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_785(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m785-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_785() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_785(total as u64) % 997) as f32;
        total
    }
}

pub mod m786 {
    use super::*;

    pub struct Accumulator786<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator786<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.701_f32 + y.sin();
        let b = y * 6.894_f32 - x.cos();
        let mut acc = Accumulator786::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.008_f32 + y.sin();
        let b = y * 0.291_f32 - x.cos();
        let mut acc = Accumulator786::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.303_f32 + y.sin();
        let b = y * 4.126_f32 - x.cos();
        let mut acc = Accumulator786::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.701_f32 + y.sin();
        let b = y * 8.184_f32 - x.cos();
        let mut acc = Accumulator786::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.61_f32 + y.sin();
        let b = y * 6.887_f32 - x.cos();
        let mut acc = Accumulator786::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.638_f32 + y.sin();
        let b = y * 2.107_f32 - x.cos();
        let mut acc = Accumulator786::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.742_f32 + y.sin();
        let b = y * 6.36_f32 - x.cos();
        let mut acc = Accumulator786::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.052_f32 + y.sin();
        let b = y * 9.187_f32 - x.cos();
        let mut acc = Accumulator786::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.511_f32 + y.sin();
        let b = y * 2.391_f32 - x.cos();
        let mut acc = Accumulator786::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.62_f32 + y.sin();
        let b = y * 8.361_f32 - x.cos();
        let mut acc = Accumulator786::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.922_f32 + y.sin();
        let b = y * 7.613_f32 - x.cos();
        let mut acc = Accumulator786::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.493_f32 + y.sin();
        let b = y * 8.815_f32 - x.cos();
        let mut acc = Accumulator786::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.997_f32 + y.sin();
        let b = y * 2.738_f32 - x.cos();
        let mut acc = Accumulator786::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.134_f32 + y.sin();
        let b = y * 9.809_f32 - x.cos();
        let mut acc = Accumulator786::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.184_f32 + y.sin();
        let b = y * 7.782_f32 - x.cos();
        let mut acc = Accumulator786::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.646_f32 + y.sin();
        let b = y * 3.731_f32 - x.cos();
        let mut acc = Accumulator786::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.048_f32 + y.sin();
        let b = y * 7.548_f32 - x.cos();
        let mut acc = Accumulator786::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.321_f32 + y.sin();
        let b = y * 6.809_f32 - x.cos();
        let mut acc = Accumulator786::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.537_f32 + y.sin();
        let b = y * 7.263_f32 - x.cos();
        let mut acc = Accumulator786::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.194_f32 + y.sin();
        let b = y * 1.3_f32 - x.cos();
        let mut acc = Accumulator786::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_786(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_786() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_786(total as u64) % 997) as f32;
        total
    }
}

pub mod m787 {
    use super::*;

    pub struct Accumulator787<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator787<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.317_f32 + y.sin();
        let b = y * 3.206_f32 - x.cos();
        let mut acc = Accumulator787::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.868_f32 + y.sin();
        let b = y * 3.386_f32 - x.cos();
        let mut acc = Accumulator787::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.924_f32 + y.sin();
        let b = y * 3.028_f32 - x.cos();
        let mut acc = Accumulator787::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.24_f32 + y.sin();
        let b = y * 5.152_f32 - x.cos();
        let mut acc = Accumulator787::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.065_f32 + y.sin();
        let b = y * 7.406_f32 - x.cos();
        let mut acc = Accumulator787::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.469_f32 + y.sin();
        let b = y * 7.711_f32 - x.cos();
        let mut acc = Accumulator787::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.985_f32 + y.sin();
        let b = y * 6.17_f32 - x.cos();
        let mut acc = Accumulator787::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.434_f32 + y.sin();
        let b = y * 4.734_f32 - x.cos();
        let mut acc = Accumulator787::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.962_f32 + y.sin();
        let b = y * 8.493_f32 - x.cos();
        let mut acc = Accumulator787::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.792_f32 + y.sin();
        let b = y * 9.594_f32 - x.cos();
        let mut acc = Accumulator787::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.305_f32 + y.sin();
        let b = y * 2.061_f32 - x.cos();
        let mut acc = Accumulator787::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.832_f32 + y.sin();
        let b = y * 3.014_f32 - x.cos();
        let mut acc = Accumulator787::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.645_f32 + y.sin();
        let b = y * 8.348_f32 - x.cos();
        let mut acc = Accumulator787::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.705_f32 + y.sin();
        let b = y * 3.62_f32 - x.cos();
        let mut acc = Accumulator787::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.858_f32 + y.sin();
        let b = y * 3.081_f32 - x.cos();
        let mut acc = Accumulator787::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.542_f32 + y.sin();
        let b = y * 9.884_f32 - x.cos();
        let mut acc = Accumulator787::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.733_f32 + y.sin();
        let b = y * 3.775_f32 - x.cos();
        let mut acc = Accumulator787::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.36_f32 + y.sin();
        let b = y * 3.006_f32 - x.cos();
        let mut acc = Accumulator787::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.833_f32 + y.sin();
        let b = y * 3.24_f32 - x.cos();
        let mut acc = Accumulator787::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.772_f32 + y.sin();
        let b = y * 4.165_f32 - x.cos();
        let mut acc = Accumulator787::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_787(seed: u64) -> u64 {
        let re = Regex::new(r"m787-(\d+)").unwrap();
        let hay = format!("m787-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_787() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_787(total as u64) % 997) as f32;
        total
    }
}

pub mod m788 {
    use super::*;

    pub struct Accumulator788<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator788<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.669_f32 + y.sin();
        let b = y * 5.789_f32 - x.cos();
        let mut acc = Accumulator788::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.137_f32 + y.sin();
        let b = y * 6.823_f32 - x.cos();
        let mut acc = Accumulator788::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.838_f32 + y.sin();
        let b = y * 8.357_f32 - x.cos();
        let mut acc = Accumulator788::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.53_f32 + y.sin();
        let b = y * 8.074_f32 - x.cos();
        let mut acc = Accumulator788::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.478_f32 + y.sin();
        let b = y * 3.023_f32 - x.cos();
        let mut acc = Accumulator788::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.889_f32 + y.sin();
        let b = y * 2.524_f32 - x.cos();
        let mut acc = Accumulator788::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.366_f32 + y.sin();
        let b = y * 6.925_f32 - x.cos();
        let mut acc = Accumulator788::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.791_f32 + y.sin();
        let b = y * 3.759_f32 - x.cos();
        let mut acc = Accumulator788::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.089_f32 + y.sin();
        let b = y * 3.45_f32 - x.cos();
        let mut acc = Accumulator788::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.772_f32 + y.sin();
        let b = y * 4.348_f32 - x.cos();
        let mut acc = Accumulator788::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.146_f32 + y.sin();
        let b = y * 0.715_f32 - x.cos();
        let mut acc = Accumulator788::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.803_f32 + y.sin();
        let b = y * 9.594_f32 - x.cos();
        let mut acc = Accumulator788::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.593_f32 + y.sin();
        let b = y * 7.421_f32 - x.cos();
        let mut acc = Accumulator788::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.483_f32 + y.sin();
        let b = y * 5.686_f32 - x.cos();
        let mut acc = Accumulator788::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.606_f32 + y.sin();
        let b = y * 1.086_f32 - x.cos();
        let mut acc = Accumulator788::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.276_f32 + y.sin();
        let b = y * 4.868_f32 - x.cos();
        let mut acc = Accumulator788::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.139_f32 + y.sin();
        let b = y * 0.288_f32 - x.cos();
        let mut acc = Accumulator788::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.662_f32 + y.sin();
        let b = y * 5.41_f32 - x.cos();
        let mut acc = Accumulator788::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.294_f32 + y.sin();
        let b = y * 5.588_f32 - x.cos();
        let mut acc = Accumulator788::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.095_f32 + y.sin();
        let b = y * 1.392_f32 - x.cos();
        let mut acc = Accumulator788::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_788(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_788() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_788(total as u64) % 997) as f32;
        total
    }
}

pub mod m789 {
    use super::*;

    pub struct Accumulator789<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator789<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.926_f32 + y.sin();
        let b = y * 1.147_f32 - x.cos();
        let mut acc = Accumulator789::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.288_f32 + y.sin();
        let b = y * 8.808_f32 - x.cos();
        let mut acc = Accumulator789::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.57_f32 + y.sin();
        let b = y * 9.098_f32 - x.cos();
        let mut acc = Accumulator789::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.503_f32 + y.sin();
        let b = y * 4.108_f32 - x.cos();
        let mut acc = Accumulator789::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.599_f32 + y.sin();
        let b = y * 3.742_f32 - x.cos();
        let mut acc = Accumulator789::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.129_f32 + y.sin();
        let b = y * 5.148_f32 - x.cos();
        let mut acc = Accumulator789::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.819_f32 + y.sin();
        let b = y * 8.36_f32 - x.cos();
        let mut acc = Accumulator789::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.954_f32 + y.sin();
        let b = y * 8.161_f32 - x.cos();
        let mut acc = Accumulator789::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.406_f32 + y.sin();
        let b = y * 1.883_f32 - x.cos();
        let mut acc = Accumulator789::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.891_f32 + y.sin();
        let b = y * 9.639_f32 - x.cos();
        let mut acc = Accumulator789::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.484_f32 + y.sin();
        let b = y * 3.344_f32 - x.cos();
        let mut acc = Accumulator789::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.274_f32 + y.sin();
        let b = y * 9.704_f32 - x.cos();
        let mut acc = Accumulator789::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.876_f32 + y.sin();
        let b = y * 1.16_f32 - x.cos();
        let mut acc = Accumulator789::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.298_f32 + y.sin();
        let b = y * 9.15_f32 - x.cos();
        let mut acc = Accumulator789::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.665_f32 + y.sin();
        let b = y * 6.851_f32 - x.cos();
        let mut acc = Accumulator789::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.064_f32 + y.sin();
        let b = y * 9.737_f32 - x.cos();
        let mut acc = Accumulator789::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.871_f32 + y.sin();
        let b = y * 9.654_f32 - x.cos();
        let mut acc = Accumulator789::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.318_f32 + y.sin();
        let b = y * 6.555_f32 - x.cos();
        let mut acc = Accumulator789::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.025_f32 + y.sin();
        let b = y * 8.808_f32 - x.cos();
        let mut acc = Accumulator789::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.111_f32 + y.sin();
        let b = y * 1.85_f32 - x.cos();
        let mut acc = Accumulator789::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_789(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(789u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_789() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_789(total as u64) % 997) as f32;
        total
    }
}

pub mod m790 {
    use super::*;

    pub struct Accumulator790<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator790<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.02_f32 + y.sin();
        let b = y * 5.468_f32 - x.cos();
        let mut acc = Accumulator790::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.924_f32 + y.sin();
        let b = y * 4.403_f32 - x.cos();
        let mut acc = Accumulator790::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.212_f32 + y.sin();
        let b = y * 1.433_f32 - x.cos();
        let mut acc = Accumulator790::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.81_f32 + y.sin();
        let b = y * 5.577_f32 - x.cos();
        let mut acc = Accumulator790::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.199_f32 + y.sin();
        let b = y * 2.616_f32 - x.cos();
        let mut acc = Accumulator790::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.823_f32 + y.sin();
        let b = y * 3.308_f32 - x.cos();
        let mut acc = Accumulator790::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.639_f32 + y.sin();
        let b = y * 7.172_f32 - x.cos();
        let mut acc = Accumulator790::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.618_f32 + y.sin();
        let b = y * 6.41_f32 - x.cos();
        let mut acc = Accumulator790::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.319_f32 + y.sin();
        let b = y * 3.457_f32 - x.cos();
        let mut acc = Accumulator790::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.424_f32 + y.sin();
        let b = y * 9.445_f32 - x.cos();
        let mut acc = Accumulator790::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.263_f32 + y.sin();
        let b = y * 4.589_f32 - x.cos();
        let mut acc = Accumulator790::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.711_f32 + y.sin();
        let b = y * 5.941_f32 - x.cos();
        let mut acc = Accumulator790::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.393_f32 + y.sin();
        let b = y * 4.867_f32 - x.cos();
        let mut acc = Accumulator790::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.741_f32 + y.sin();
        let b = y * 1.495_f32 - x.cos();
        let mut acc = Accumulator790::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.712_f32 + y.sin();
        let b = y * 7.549_f32 - x.cos();
        let mut acc = Accumulator790::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.935_f32 + y.sin();
        let b = y * 7.338_f32 - x.cos();
        let mut acc = Accumulator790::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.498_f32 + y.sin();
        let b = y * 0.827_f32 - x.cos();
        let mut acc = Accumulator790::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.528_f32 + y.sin();
        let b = y * 1.47_f32 - x.cos();
        let mut acc = Accumulator790::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.571_f32 + y.sin();
        let b = y * 2.287_f32 - x.cos();
        let mut acc = Accumulator790::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.74_f32 + y.sin();
        let b = y * 5.914_f32 - x.cos();
        let mut acc = Accumulator790::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_790(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_790() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_790(total as u64) % 997) as f32;
        total
    }
}

pub mod m791 {
    use super::*;

    pub struct Accumulator791<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator791<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.733_f32 + y.sin();
        let b = y * 1.363_f32 - x.cos();
        let mut acc = Accumulator791::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.11_f32 + y.sin();
        let b = y * 9.542_f32 - x.cos();
        let mut acc = Accumulator791::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.904_f32 + y.sin();
        let b = y * 2.06_f32 - x.cos();
        let mut acc = Accumulator791::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.043_f32 + y.sin();
        let b = y * 1.077_f32 - x.cos();
        let mut acc = Accumulator791::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.16_f32 + y.sin();
        let b = y * 3.356_f32 - x.cos();
        let mut acc = Accumulator791::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.651_f32 + y.sin();
        let b = y * 4.35_f32 - x.cos();
        let mut acc = Accumulator791::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.056_f32 + y.sin();
        let b = y * 1.217_f32 - x.cos();
        let mut acc = Accumulator791::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.7_f32 + y.sin();
        let b = y * 0.625_f32 - x.cos();
        let mut acc = Accumulator791::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.176_f32 + y.sin();
        let b = y * 8.545_f32 - x.cos();
        let mut acc = Accumulator791::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.086_f32 + y.sin();
        let b = y * 6.246_f32 - x.cos();
        let mut acc = Accumulator791::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.225_f32 + y.sin();
        let b = y * 7.1_f32 - x.cos();
        let mut acc = Accumulator791::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.382_f32 + y.sin();
        let b = y * 1.644_f32 - x.cos();
        let mut acc = Accumulator791::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.052_f32 + y.sin();
        let b = y * 9.178_f32 - x.cos();
        let mut acc = Accumulator791::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.393_f32 + y.sin();
        let b = y * 4.05_f32 - x.cos();
        let mut acc = Accumulator791::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.208_f32 + y.sin();
        let b = y * 2.896_f32 - x.cos();
        let mut acc = Accumulator791::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.219_f32 + y.sin();
        let b = y * 1.192_f32 - x.cos();
        let mut acc = Accumulator791::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.212_f32 + y.sin();
        let b = y * 2.316_f32 - x.cos();
        let mut acc = Accumulator791::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.55_f32 + y.sin();
        let b = y * 7.384_f32 - x.cos();
        let mut acc = Accumulator791::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.459_f32 + y.sin();
        let b = y * 2.693_f32 - x.cos();
        let mut acc = Accumulator791::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.779_f32 + y.sin();
        let b = y * 0.618_f32 - x.cos();
        let mut acc = Accumulator791::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_791(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_791() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_791(total as u64) % 997) as f32;
        total
    }
}

pub mod m792 {
    use super::*;

    pub struct Accumulator792<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator792<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.612_f32 + y.sin();
        let b = y * 2.914_f32 - x.cos();
        let mut acc = Accumulator792::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.563_f32 + y.sin();
        let b = y * 2.259_f32 - x.cos();
        let mut acc = Accumulator792::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.409_f32 + y.sin();
        let b = y * 6.57_f32 - x.cos();
        let mut acc = Accumulator792::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.316_f32 + y.sin();
        let b = y * 5.818_f32 - x.cos();
        let mut acc = Accumulator792::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.548_f32 + y.sin();
        let b = y * 9.19_f32 - x.cos();
        let mut acc = Accumulator792::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.19_f32 + y.sin();
        let b = y * 9.806_f32 - x.cos();
        let mut acc = Accumulator792::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.401_f32 + y.sin();
        let b = y * 6.299_f32 - x.cos();
        let mut acc = Accumulator792::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.574_f32 + y.sin();
        let b = y * 3.475_f32 - x.cos();
        let mut acc = Accumulator792::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.462_f32 + y.sin();
        let b = y * 9.827_f32 - x.cos();
        let mut acc = Accumulator792::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.608_f32 + y.sin();
        let b = y * 6.327_f32 - x.cos();
        let mut acc = Accumulator792::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.923_f32 + y.sin();
        let b = y * 8.256_f32 - x.cos();
        let mut acc = Accumulator792::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.796_f32 + y.sin();
        let b = y * 8.684_f32 - x.cos();
        let mut acc = Accumulator792::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.274_f32 + y.sin();
        let b = y * 0.885_f32 - x.cos();
        let mut acc = Accumulator792::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.887_f32 + y.sin();
        let b = y * 2.647_f32 - x.cos();
        let mut acc = Accumulator792::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.395_f32 + y.sin();
        let b = y * 3.01_f32 - x.cos();
        let mut acc = Accumulator792::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.46_f32 + y.sin();
        let b = y * 7.21_f32 - x.cos();
        let mut acc = Accumulator792::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.065_f32 + y.sin();
        let b = y * 1.076_f32 - x.cos();
        let mut acc = Accumulator792::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.108_f32 + y.sin();
        let b = y * 2.584_f32 - x.cos();
        let mut acc = Accumulator792::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.777_f32 + y.sin();
        let b = y * 8.058_f32 - x.cos();
        let mut acc = Accumulator792::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.86_f32 + y.sin();
        let b = y * 7.909_f32 - x.cos();
        let mut acc = Accumulator792::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_792(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m792-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_792() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_792(total as u64) % 997) as f32;
        total
    }
}

pub mod m793 {
    use super::*;

    pub struct Accumulator793<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator793<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.487_f32 + y.sin();
        let b = y * 0.24_f32 - x.cos();
        let mut acc = Accumulator793::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.831_f32 + y.sin();
        let b = y * 2.469_f32 - x.cos();
        let mut acc = Accumulator793::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.11_f32 + y.sin();
        let b = y * 8.976_f32 - x.cos();
        let mut acc = Accumulator793::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.058_f32 + y.sin();
        let b = y * 0.515_f32 - x.cos();
        let mut acc = Accumulator793::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.78_f32 + y.sin();
        let b = y * 0.938_f32 - x.cos();
        let mut acc = Accumulator793::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.33_f32 + y.sin();
        let b = y * 3.018_f32 - x.cos();
        let mut acc = Accumulator793::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.652_f32 + y.sin();
        let b = y * 7.294_f32 - x.cos();
        let mut acc = Accumulator793::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.43_f32 + y.sin();
        let b = y * 9.633_f32 - x.cos();
        let mut acc = Accumulator793::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.911_f32 + y.sin();
        let b = y * 7.052_f32 - x.cos();
        let mut acc = Accumulator793::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.153_f32 + y.sin();
        let b = y * 9.68_f32 - x.cos();
        let mut acc = Accumulator793::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.029_f32 + y.sin();
        let b = y * 4.928_f32 - x.cos();
        let mut acc = Accumulator793::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.607_f32 + y.sin();
        let b = y * 0.901_f32 - x.cos();
        let mut acc = Accumulator793::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.069_f32 + y.sin();
        let b = y * 2.575_f32 - x.cos();
        let mut acc = Accumulator793::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.378_f32 + y.sin();
        let b = y * 4.208_f32 - x.cos();
        let mut acc = Accumulator793::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.046_f32 + y.sin();
        let b = y * 1.187_f32 - x.cos();
        let mut acc = Accumulator793::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.739_f32 + y.sin();
        let b = y * 5.813_f32 - x.cos();
        let mut acc = Accumulator793::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.173_f32 + y.sin();
        let b = y * 7.333_f32 - x.cos();
        let mut acc = Accumulator793::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.027_f32 + y.sin();
        let b = y * 4.159_f32 - x.cos();
        let mut acc = Accumulator793::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.821_f32 + y.sin();
        let b = y * 2.38_f32 - x.cos();
        let mut acc = Accumulator793::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.417_f32 + y.sin();
        let b = y * 4.592_f32 - x.cos();
        let mut acc = Accumulator793::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_793(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_793() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_793(total as u64) % 997) as f32;
        total
    }
}

pub mod m794 {
    use super::*;

    pub struct Accumulator794<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator794<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.439_f32 + y.sin();
        let b = y * 8.38_f32 - x.cos();
        let mut acc = Accumulator794::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.629_f32 + y.sin();
        let b = y * 5.319_f32 - x.cos();
        let mut acc = Accumulator794::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.696_f32 + y.sin();
        let b = y * 8.26_f32 - x.cos();
        let mut acc = Accumulator794::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.991_f32 + y.sin();
        let b = y * 3.354_f32 - x.cos();
        let mut acc = Accumulator794::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.878_f32 + y.sin();
        let b = y * 3.249_f32 - x.cos();
        let mut acc = Accumulator794::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.369_f32 + y.sin();
        let b = y * 0.54_f32 - x.cos();
        let mut acc = Accumulator794::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.725_f32 + y.sin();
        let b = y * 6.24_f32 - x.cos();
        let mut acc = Accumulator794::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.291_f32 + y.sin();
        let b = y * 1.702_f32 - x.cos();
        let mut acc = Accumulator794::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.121_f32 + y.sin();
        let b = y * 1.778_f32 - x.cos();
        let mut acc = Accumulator794::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.312_f32 + y.sin();
        let b = y * 4.083_f32 - x.cos();
        let mut acc = Accumulator794::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.712_f32 + y.sin();
        let b = y * 0.97_f32 - x.cos();
        let mut acc = Accumulator794::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.46_f32 + y.sin();
        let b = y * 2.508_f32 - x.cos();
        let mut acc = Accumulator794::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.757_f32 + y.sin();
        let b = y * 5.55_f32 - x.cos();
        let mut acc = Accumulator794::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.047_f32 + y.sin();
        let b = y * 8.597_f32 - x.cos();
        let mut acc = Accumulator794::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.996_f32 + y.sin();
        let b = y * 1.937_f32 - x.cos();
        let mut acc = Accumulator794::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.431_f32 + y.sin();
        let b = y * 4.306_f32 - x.cos();
        let mut acc = Accumulator794::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.517_f32 + y.sin();
        let b = y * 9.152_f32 - x.cos();
        let mut acc = Accumulator794::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.067_f32 + y.sin();
        let b = y * 2.908_f32 - x.cos();
        let mut acc = Accumulator794::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.477_f32 + y.sin();
        let b = y * 9.795_f32 - x.cos();
        let mut acc = Accumulator794::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.729_f32 + y.sin();
        let b = y * 8.845_f32 - x.cos();
        let mut acc = Accumulator794::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_794(seed: u64) -> u64 {
        let re = Regex::new(r"m794-(\d+)").unwrap();
        let hay = format!("m794-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_794() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_794(total as u64) % 997) as f32;
        total
    }
}

pub mod m795 {
    use super::*;

    pub struct Accumulator795<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator795<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.815_f32 + y.sin();
        let b = y * 6.46_f32 - x.cos();
        let mut acc = Accumulator795::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.4_f32 + y.sin();
        let b = y * 8.373_f32 - x.cos();
        let mut acc = Accumulator795::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.078_f32 + y.sin();
        let b = y * 6.531_f32 - x.cos();
        let mut acc = Accumulator795::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.048_f32 + y.sin();
        let b = y * 6.027_f32 - x.cos();
        let mut acc = Accumulator795::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.492_f32 + y.sin();
        let b = y * 1.718_f32 - x.cos();
        let mut acc = Accumulator795::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.938_f32 + y.sin();
        let b = y * 5.435_f32 - x.cos();
        let mut acc = Accumulator795::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.212_f32 + y.sin();
        let b = y * 9.42_f32 - x.cos();
        let mut acc = Accumulator795::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.519_f32 + y.sin();
        let b = y * 1.947_f32 - x.cos();
        let mut acc = Accumulator795::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.132_f32 + y.sin();
        let b = y * 6.298_f32 - x.cos();
        let mut acc = Accumulator795::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.341_f32 + y.sin();
        let b = y * 3.701_f32 - x.cos();
        let mut acc = Accumulator795::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.928_f32 + y.sin();
        let b = y * 8.574_f32 - x.cos();
        let mut acc = Accumulator795::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.702_f32 + y.sin();
        let b = y * 5.749_f32 - x.cos();
        let mut acc = Accumulator795::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.123_f32 + y.sin();
        let b = y * 4.282_f32 - x.cos();
        let mut acc = Accumulator795::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.469_f32 + y.sin();
        let b = y * 1.039_f32 - x.cos();
        let mut acc = Accumulator795::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.023_f32 + y.sin();
        let b = y * 0.49_f32 - x.cos();
        let mut acc = Accumulator795::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.74_f32 + y.sin();
        let b = y * 2.409_f32 - x.cos();
        let mut acc = Accumulator795::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.986_f32 + y.sin();
        let b = y * 7.592_f32 - x.cos();
        let mut acc = Accumulator795::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.75_f32 + y.sin();
        let b = y * 8.235_f32 - x.cos();
        let mut acc = Accumulator795::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.378_f32 + y.sin();
        let b = y * 3.336_f32 - x.cos();
        let mut acc = Accumulator795::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.132_f32 + y.sin();
        let b = y * 6.236_f32 - x.cos();
        let mut acc = Accumulator795::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_795(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_795() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_795(total as u64) % 997) as f32;
        total
    }
}

pub mod m796 {
    use super::*;

    pub struct Accumulator796<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator796<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.636_f32 + y.sin();
        let b = y * 0.382_f32 - x.cos();
        let mut acc = Accumulator796::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.933_f32 + y.sin();
        let b = y * 0.539_f32 - x.cos();
        let mut acc = Accumulator796::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.127_f32 + y.sin();
        let b = y * 9.259_f32 - x.cos();
        let mut acc = Accumulator796::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.941_f32 + y.sin();
        let b = y * 2.572_f32 - x.cos();
        let mut acc = Accumulator796::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.37_f32 + y.sin();
        let b = y * 9.111_f32 - x.cos();
        let mut acc = Accumulator796::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.22_f32 + y.sin();
        let b = y * 4.009_f32 - x.cos();
        let mut acc = Accumulator796::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.801_f32 + y.sin();
        let b = y * 9.683_f32 - x.cos();
        let mut acc = Accumulator796::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.973_f32 + y.sin();
        let b = y * 0.475_f32 - x.cos();
        let mut acc = Accumulator796::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.447_f32 + y.sin();
        let b = y * 4.171_f32 - x.cos();
        let mut acc = Accumulator796::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.123_f32 + y.sin();
        let b = y * 0.48_f32 - x.cos();
        let mut acc = Accumulator796::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.637_f32 + y.sin();
        let b = y * 5.728_f32 - x.cos();
        let mut acc = Accumulator796::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.575_f32 + y.sin();
        let b = y * 9.621_f32 - x.cos();
        let mut acc = Accumulator796::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.567_f32 + y.sin();
        let b = y * 5.249_f32 - x.cos();
        let mut acc = Accumulator796::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.474_f32 + y.sin();
        let b = y * 9.847_f32 - x.cos();
        let mut acc = Accumulator796::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.904_f32 + y.sin();
        let b = y * 0.491_f32 - x.cos();
        let mut acc = Accumulator796::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.984_f32 + y.sin();
        let b = y * 5.179_f32 - x.cos();
        let mut acc = Accumulator796::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.264_f32 + y.sin();
        let b = y * 7.884_f32 - x.cos();
        let mut acc = Accumulator796::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.895_f32 + y.sin();
        let b = y * 4.655_f32 - x.cos();
        let mut acc = Accumulator796::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.429_f32 + y.sin();
        let b = y * 7.807_f32 - x.cos();
        let mut acc = Accumulator796::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.772_f32 + y.sin();
        let b = y * 6.131_f32 - x.cos();
        let mut acc = Accumulator796::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_796(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(796u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_796() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_796(total as u64) % 997) as f32;
        total
    }
}

pub mod m797 {
    use super::*;

    pub struct Accumulator797<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator797<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.844_f32 + y.sin();
        let b = y * 8.954_f32 - x.cos();
        let mut acc = Accumulator797::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.413_f32 + y.sin();
        let b = y * 0.95_f32 - x.cos();
        let mut acc = Accumulator797::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.03_f32 + y.sin();
        let b = y * 2.914_f32 - x.cos();
        let mut acc = Accumulator797::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.332_f32 + y.sin();
        let b = y * 4.897_f32 - x.cos();
        let mut acc = Accumulator797::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.08_f32 + y.sin();
        let b = y * 4.545_f32 - x.cos();
        let mut acc = Accumulator797::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.523_f32 + y.sin();
        let b = y * 8.825_f32 - x.cos();
        let mut acc = Accumulator797::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.678_f32 + y.sin();
        let b = y * 6.837_f32 - x.cos();
        let mut acc = Accumulator797::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.216_f32 + y.sin();
        let b = y * 7.028_f32 - x.cos();
        let mut acc = Accumulator797::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.669_f32 + y.sin();
        let b = y * 1.108_f32 - x.cos();
        let mut acc = Accumulator797::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.141_f32 + y.sin();
        let b = y * 0.536_f32 - x.cos();
        let mut acc = Accumulator797::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.839_f32 + y.sin();
        let b = y * 8.49_f32 - x.cos();
        let mut acc = Accumulator797::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.396_f32 + y.sin();
        let b = y * 2.943_f32 - x.cos();
        let mut acc = Accumulator797::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.495_f32 + y.sin();
        let b = y * 0.37_f32 - x.cos();
        let mut acc = Accumulator797::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.864_f32 + y.sin();
        let b = y * 6.806_f32 - x.cos();
        let mut acc = Accumulator797::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.789_f32 + y.sin();
        let b = y * 5.315_f32 - x.cos();
        let mut acc = Accumulator797::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.755_f32 + y.sin();
        let b = y * 2.27_f32 - x.cos();
        let mut acc = Accumulator797::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.057_f32 + y.sin();
        let b = y * 2.944_f32 - x.cos();
        let mut acc = Accumulator797::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.46_f32 + y.sin();
        let b = y * 1.644_f32 - x.cos();
        let mut acc = Accumulator797::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.575_f32 + y.sin();
        let b = y * 9.645_f32 - x.cos();
        let mut acc = Accumulator797::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.28_f32 + y.sin();
        let b = y * 4.533_f32 - x.cos();
        let mut acc = Accumulator797::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_797(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_797() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_797(total as u64) % 997) as f32;
        total
    }
}

pub mod m798 {
    use super::*;

    pub struct Accumulator798<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator798<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.267_f32 + y.sin();
        let b = y * 9.292_f32 - x.cos();
        let mut acc = Accumulator798::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.769_f32 + y.sin();
        let b = y * 8.588_f32 - x.cos();
        let mut acc = Accumulator798::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.289_f32 + y.sin();
        let b = y * 4.394_f32 - x.cos();
        let mut acc = Accumulator798::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.669_f32 + y.sin();
        let b = y * 6.863_f32 - x.cos();
        let mut acc = Accumulator798::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.242_f32 + y.sin();
        let b = y * 4.268_f32 - x.cos();
        let mut acc = Accumulator798::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.733_f32 + y.sin();
        let b = y * 5.681_f32 - x.cos();
        let mut acc = Accumulator798::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.443_f32 + y.sin();
        let b = y * 2.94_f32 - x.cos();
        let mut acc = Accumulator798::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.258_f32 + y.sin();
        let b = y * 7.033_f32 - x.cos();
        let mut acc = Accumulator798::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.133_f32 + y.sin();
        let b = y * 0.775_f32 - x.cos();
        let mut acc = Accumulator798::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.451_f32 + y.sin();
        let b = y * 6.697_f32 - x.cos();
        let mut acc = Accumulator798::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.191_f32 + y.sin();
        let b = y * 1.297_f32 - x.cos();
        let mut acc = Accumulator798::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.881_f32 + y.sin();
        let b = y * 2.022_f32 - x.cos();
        let mut acc = Accumulator798::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.353_f32 + y.sin();
        let b = y * 5.249_f32 - x.cos();
        let mut acc = Accumulator798::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.586_f32 + y.sin();
        let b = y * 4.645_f32 - x.cos();
        let mut acc = Accumulator798::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.198_f32 + y.sin();
        let b = y * 2.311_f32 - x.cos();
        let mut acc = Accumulator798::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.152_f32 + y.sin();
        let b = y * 7.871_f32 - x.cos();
        let mut acc = Accumulator798::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.036_f32 + y.sin();
        let b = y * 8.335_f32 - x.cos();
        let mut acc = Accumulator798::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.655_f32 + y.sin();
        let b = y * 0.628_f32 - x.cos();
        let mut acc = Accumulator798::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.259_f32 + y.sin();
        let b = y * 1.764_f32 - x.cos();
        let mut acc = Accumulator798::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.663_f32 + y.sin();
        let b = y * 0.823_f32 - x.cos();
        let mut acc = Accumulator798::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_798(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_798() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_798(total as u64) % 997) as f32;
        total
    }
}

pub mod m799 {
    use super::*;

    pub struct Accumulator799<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator799<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.071_f32 + y.sin();
        let b = y * 6.625_f32 - x.cos();
        let mut acc = Accumulator799::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.537_f32 + y.sin();
        let b = y * 5.841_f32 - x.cos();
        let mut acc = Accumulator799::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.338_f32 + y.sin();
        let b = y * 2.573_f32 - x.cos();
        let mut acc = Accumulator799::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.519_f32 + y.sin();
        let b = y * 4.351_f32 - x.cos();
        let mut acc = Accumulator799::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.922_f32 + y.sin();
        let b = y * 2.83_f32 - x.cos();
        let mut acc = Accumulator799::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.527_f32 + y.sin();
        let b = y * 7.867_f32 - x.cos();
        let mut acc = Accumulator799::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.243_f32 + y.sin();
        let b = y * 7.365_f32 - x.cos();
        let mut acc = Accumulator799::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.661_f32 + y.sin();
        let b = y * 5.39_f32 - x.cos();
        let mut acc = Accumulator799::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.721_f32 + y.sin();
        let b = y * 2.447_f32 - x.cos();
        let mut acc = Accumulator799::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.972_f32 + y.sin();
        let b = y * 5.953_f32 - x.cos();
        let mut acc = Accumulator799::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.186_f32 + y.sin();
        let b = y * 6.793_f32 - x.cos();
        let mut acc = Accumulator799::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.893_f32 + y.sin();
        let b = y * 6.267_f32 - x.cos();
        let mut acc = Accumulator799::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.447_f32 + y.sin();
        let b = y * 0.543_f32 - x.cos();
        let mut acc = Accumulator799::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.139_f32 + y.sin();
        let b = y * 8.186_f32 - x.cos();
        let mut acc = Accumulator799::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.411_f32 + y.sin();
        let b = y * 2.484_f32 - x.cos();
        let mut acc = Accumulator799::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.794_f32 + y.sin();
        let b = y * 5.689_f32 - x.cos();
        let mut acc = Accumulator799::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.896_f32 + y.sin();
        let b = y * 2.202_f32 - x.cos();
        let mut acc = Accumulator799::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.366_f32 + y.sin();
        let b = y * 5.435_f32 - x.cos();
        let mut acc = Accumulator799::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.166_f32 + y.sin();
        let b = y * 7.826_f32 - x.cos();
        let mut acc = Accumulator799::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.358_f32 + y.sin();
        let b = y * 5.338_f32 - x.cos();
        let mut acc = Accumulator799::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_799(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m799-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_799() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_799(total as u64) % 997) as f32;
        total
    }
}

pub fn touch_bulk_7() -> f32 {
    let mut total = 0.0_f32;
    total += m700::run_all_700();
    total += m701::run_all_701();
    total += m702::run_all_702();
    total += m703::run_all_703();
    total += m704::run_all_704();
    total += m705::run_all_705();
    total += m706::run_all_706();
    total += m707::run_all_707();
    total += m708::run_all_708();
    total += m709::run_all_709();
    total += m710::run_all_710();
    total += m711::run_all_711();
    total += m712::run_all_712();
    total += m713::run_all_713();
    total += m714::run_all_714();
    total += m715::run_all_715();
    total += m716::run_all_716();
    total += m717::run_all_717();
    total += m718::run_all_718();
    total += m719::run_all_719();
    total += m720::run_all_720();
    total += m721::run_all_721();
    total += m722::run_all_722();
    total += m723::run_all_723();
    total += m724::run_all_724();
    total += m725::run_all_725();
    total += m726::run_all_726();
    total += m727::run_all_727();
    total += m728::run_all_728();
    total += m729::run_all_729();
    total += m730::run_all_730();
    total += m731::run_all_731();
    total += m732::run_all_732();
    total += m733::run_all_733();
    total += m734::run_all_734();
    total += m735::run_all_735();
    total += m736::run_all_736();
    total += m737::run_all_737();
    total += m738::run_all_738();
    total += m739::run_all_739();
    total += m740::run_all_740();
    total += m741::run_all_741();
    total += m742::run_all_742();
    total += m743::run_all_743();
    total += m744::run_all_744();
    total += m745::run_all_745();
    total += m746::run_all_746();
    total += m747::run_all_747();
    total += m748::run_all_748();
    total += m749::run_all_749();
    total += m750::run_all_750();
    total += m751::run_all_751();
    total += m752::run_all_752();
    total += m753::run_all_753();
    total += m754::run_all_754();
    total += m755::run_all_755();
    total += m756::run_all_756();
    total += m757::run_all_757();
    total += m758::run_all_758();
    total += m759::run_all_759();
    total += m760::run_all_760();
    total += m761::run_all_761();
    total += m762::run_all_762();
    total += m763::run_all_763();
    total += m764::run_all_764();
    total += m765::run_all_765();
    total += m766::run_all_766();
    total += m767::run_all_767();
    total += m768::run_all_768();
    total += m769::run_all_769();
    total += m770::run_all_770();
    total += m771::run_all_771();
    total += m772::run_all_772();
    total += m773::run_all_773();
    total += m774::run_all_774();
    total += m775::run_all_775();
    total += m776::run_all_776();
    total += m777::run_all_777();
    total += m778::run_all_778();
    total += m779::run_all_779();
    total += m780::run_all_780();
    total += m781::run_all_781();
    total += m782::run_all_782();
    total += m783::run_all_783();
    total += m784::run_all_784();
    total += m785::run_all_785();
    total += m786::run_all_786();
    total += m787::run_all_787();
    total += m788::run_all_788();
    total += m789::run_all_789();
    total += m790::run_all_790();
    total += m791::run_all_791();
    total += m792::run_all_792();
    total += m793::run_all_793();
    total += m794::run_all_794();
    total += m795::run_all_795();
    total += m796::run_all_796();
    total += m797::run_all_797();
    total += m798::run_all_798();
    total += m799::run_all_799();
    total
}
