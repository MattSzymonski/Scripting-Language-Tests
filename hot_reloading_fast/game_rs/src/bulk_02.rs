//! Auto-generated bulk module (file 2) - exists purely to make `game_rs`
//! heavier for a hot-patch speed experiment. Real arithmetic, real generics
//! (so monomorphization isn't free), and real use of a rotating set of
//! external crates so none of them are dead code. `touch_bulk_2()` is
//! called once from `game::init` so nothing here gets stripped as unreachable.

use rand::{Rng, SeedableRng};
use sha2::{Digest, Sha256};
use noise::{NoiseFn, Perlin};
use regex::Regex;
use itertools::Itertools;
use uuid::Uuid;
use chrono::Utc;


pub mod m200 {
    use super::*;

    pub struct Accumulator200<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator200<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.952_f32 + y.sin();
        let b = y * 3.956_f32 - x.cos();
        let mut acc = Accumulator200::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.815_f32 + y.sin();
        let b = y * 4.652_f32 - x.cos();
        let mut acc = Accumulator200::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.866_f32 + y.sin();
        let b = y * 3.16_f32 - x.cos();
        let mut acc = Accumulator200::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.041_f32 + y.sin();
        let b = y * 6.809_f32 - x.cos();
        let mut acc = Accumulator200::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.783_f32 + y.sin();
        let b = y * 3.876_f32 - x.cos();
        let mut acc = Accumulator200::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.563_f32 + y.sin();
        let b = y * 4.368_f32 - x.cos();
        let mut acc = Accumulator200::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.596_f32 + y.sin();
        let b = y * 4.001_f32 - x.cos();
        let mut acc = Accumulator200::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.701_f32 + y.sin();
        let b = y * 4.628_f32 - x.cos();
        let mut acc = Accumulator200::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.376_f32 + y.sin();
        let b = y * 1.138_f32 - x.cos();
        let mut acc = Accumulator200::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.182_f32 + y.sin();
        let b = y * 5.499_f32 - x.cos();
        let mut acc = Accumulator200::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.477_f32 + y.sin();
        let b = y * 6.625_f32 - x.cos();
        let mut acc = Accumulator200::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.064_f32 + y.sin();
        let b = y * 3.85_f32 - x.cos();
        let mut acc = Accumulator200::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.495_f32 + y.sin();
        let b = y * 9.185_f32 - x.cos();
        let mut acc = Accumulator200::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.007_f32 + y.sin();
        let b = y * 1.459_f32 - x.cos();
        let mut acc = Accumulator200::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.059_f32 + y.sin();
        let b = y * 3.684_f32 - x.cos();
        let mut acc = Accumulator200::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.594_f32 + y.sin();
        let b = y * 5.426_f32 - x.cos();
        let mut acc = Accumulator200::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.69_f32 + y.sin();
        let b = y * 7.173_f32 - x.cos();
        let mut acc = Accumulator200::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.089_f32 + y.sin();
        let b = y * 4.064_f32 - x.cos();
        let mut acc = Accumulator200::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.239_f32 + y.sin();
        let b = y * 4.712_f32 - x.cos();
        let mut acc = Accumulator200::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.653_f32 + y.sin();
        let b = y * 0.873_f32 - x.cos();
        let mut acc = Accumulator200::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_200(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_200() -> f32 {
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
        total += (dep_touch_200(total as u64) % 997) as f32;
        total
    }
}

pub mod m201 {
    use super::*;

    pub struct Accumulator201<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator201<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.982_f32 + y.sin();
        let b = y * 5.254_f32 - x.cos();
        let mut acc = Accumulator201::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.775_f32 + y.sin();
        let b = y * 6.134_f32 - x.cos();
        let mut acc = Accumulator201::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.317_f32 + y.sin();
        let b = y * 7.153_f32 - x.cos();
        let mut acc = Accumulator201::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.382_f32 + y.sin();
        let b = y * 3.636_f32 - x.cos();
        let mut acc = Accumulator201::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.46_f32 + y.sin();
        let b = y * 2.482_f32 - x.cos();
        let mut acc = Accumulator201::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.56_f32 + y.sin();
        let b = y * 2.835_f32 - x.cos();
        let mut acc = Accumulator201::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.369_f32 + y.sin();
        let b = y * 0.185_f32 - x.cos();
        let mut acc = Accumulator201::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.288_f32 + y.sin();
        let b = y * 5.312_f32 - x.cos();
        let mut acc = Accumulator201::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.016_f32 + y.sin();
        let b = y * 8.953_f32 - x.cos();
        let mut acc = Accumulator201::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.47_f32 + y.sin();
        let b = y * 3.292_f32 - x.cos();
        let mut acc = Accumulator201::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.161_f32 + y.sin();
        let b = y * 6.835_f32 - x.cos();
        let mut acc = Accumulator201::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.954_f32 + y.sin();
        let b = y * 3.014_f32 - x.cos();
        let mut acc = Accumulator201::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.958_f32 + y.sin();
        let b = y * 8.605_f32 - x.cos();
        let mut acc = Accumulator201::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.165_f32 + y.sin();
        let b = y * 7.393_f32 - x.cos();
        let mut acc = Accumulator201::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.312_f32 + y.sin();
        let b = y * 7.019_f32 - x.cos();
        let mut acc = Accumulator201::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.978_f32 + y.sin();
        let b = y * 5.204_f32 - x.cos();
        let mut acc = Accumulator201::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.711_f32 + y.sin();
        let b = y * 3.532_f32 - x.cos();
        let mut acc = Accumulator201::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.08_f32 + y.sin();
        let b = y * 2.99_f32 - x.cos();
        let mut acc = Accumulator201::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.594_f32 + y.sin();
        let b = y * 8.05_f32 - x.cos();
        let mut acc = Accumulator201::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.087_f32 + y.sin();
        let b = y * 9.481_f32 - x.cos();
        let mut acc = Accumulator201::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_201(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(201u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_201() -> f32 {
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
        total += (dep_touch_201(total as u64) % 997) as f32;
        total
    }
}

pub mod m202 {
    use super::*;

    pub struct Accumulator202<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator202<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.231_f32 + y.sin();
        let b = y * 8.325_f32 - x.cos();
        let mut acc = Accumulator202::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.106_f32 + y.sin();
        let b = y * 1.603_f32 - x.cos();
        let mut acc = Accumulator202::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.406_f32 + y.sin();
        let b = y * 3.8_f32 - x.cos();
        let mut acc = Accumulator202::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.514_f32 + y.sin();
        let b = y * 6.722_f32 - x.cos();
        let mut acc = Accumulator202::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.296_f32 + y.sin();
        let b = y * 9.707_f32 - x.cos();
        let mut acc = Accumulator202::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.393_f32 + y.sin();
        let b = y * 5.886_f32 - x.cos();
        let mut acc = Accumulator202::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.961_f32 + y.sin();
        let b = y * 4.996_f32 - x.cos();
        let mut acc = Accumulator202::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.601_f32 + y.sin();
        let b = y * 9.797_f32 - x.cos();
        let mut acc = Accumulator202::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.818_f32 + y.sin();
        let b = y * 2.29_f32 - x.cos();
        let mut acc = Accumulator202::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.752_f32 + y.sin();
        let b = y * 9.571_f32 - x.cos();
        let mut acc = Accumulator202::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.281_f32 + y.sin();
        let b = y * 3.105_f32 - x.cos();
        let mut acc = Accumulator202::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.455_f32 + y.sin();
        let b = y * 2.973_f32 - x.cos();
        let mut acc = Accumulator202::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.787_f32 + y.sin();
        let b = y * 8.031_f32 - x.cos();
        let mut acc = Accumulator202::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.076_f32 + y.sin();
        let b = y * 5.345_f32 - x.cos();
        let mut acc = Accumulator202::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.736_f32 + y.sin();
        let b = y * 3.745_f32 - x.cos();
        let mut acc = Accumulator202::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.173_f32 + y.sin();
        let b = y * 9.721_f32 - x.cos();
        let mut acc = Accumulator202::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.462_f32 + y.sin();
        let b = y * 1.923_f32 - x.cos();
        let mut acc = Accumulator202::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.761_f32 + y.sin();
        let b = y * 2.502_f32 - x.cos();
        let mut acc = Accumulator202::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.678_f32 + y.sin();
        let b = y * 2.761_f32 - x.cos();
        let mut acc = Accumulator202::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.901_f32 + y.sin();
        let b = y * 5.15_f32 - x.cos();
        let mut acc = Accumulator202::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_202(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_202() -> f32 {
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
        total += (dep_touch_202(total as u64) % 997) as f32;
        total
    }
}

pub mod m203 {
    use super::*;

    pub struct Accumulator203<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator203<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.132_f32 + y.sin();
        let b = y * 4.254_f32 - x.cos();
        let mut acc = Accumulator203::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.255_f32 + y.sin();
        let b = y * 7.968_f32 - x.cos();
        let mut acc = Accumulator203::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.87_f32 + y.sin();
        let b = y * 2.658_f32 - x.cos();
        let mut acc = Accumulator203::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.456_f32 + y.sin();
        let b = y * 2.692_f32 - x.cos();
        let mut acc = Accumulator203::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.358_f32 + y.sin();
        let b = y * 1.31_f32 - x.cos();
        let mut acc = Accumulator203::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.647_f32 + y.sin();
        let b = y * 4.562_f32 - x.cos();
        let mut acc = Accumulator203::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.334_f32 + y.sin();
        let b = y * 5.123_f32 - x.cos();
        let mut acc = Accumulator203::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.153_f32 + y.sin();
        let b = y * 9.307_f32 - x.cos();
        let mut acc = Accumulator203::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.424_f32 + y.sin();
        let b = y * 3.264_f32 - x.cos();
        let mut acc = Accumulator203::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.483_f32 + y.sin();
        let b = y * 7.082_f32 - x.cos();
        let mut acc = Accumulator203::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.963_f32 + y.sin();
        let b = y * 3.139_f32 - x.cos();
        let mut acc = Accumulator203::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.496_f32 + y.sin();
        let b = y * 8.462_f32 - x.cos();
        let mut acc = Accumulator203::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.996_f32 + y.sin();
        let b = y * 0.907_f32 - x.cos();
        let mut acc = Accumulator203::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.211_f32 + y.sin();
        let b = y * 0.631_f32 - x.cos();
        let mut acc = Accumulator203::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.225_f32 + y.sin();
        let b = y * 2.125_f32 - x.cos();
        let mut acc = Accumulator203::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.053_f32 + y.sin();
        let b = y * 4.275_f32 - x.cos();
        let mut acc = Accumulator203::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.972_f32 + y.sin();
        let b = y * 5.061_f32 - x.cos();
        let mut acc = Accumulator203::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.211_f32 + y.sin();
        let b = y * 7.009_f32 - x.cos();
        let mut acc = Accumulator203::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.768_f32 + y.sin();
        let b = y * 1.355_f32 - x.cos();
        let mut acc = Accumulator203::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.376_f32 + y.sin();
        let b = y * 7.771_f32 - x.cos();
        let mut acc = Accumulator203::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_203(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_203() -> f32 {
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
        total += (dep_touch_203(total as u64) % 997) as f32;
        total
    }
}

pub mod m204 {
    use super::*;

    pub struct Accumulator204<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator204<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.606_f32 + y.sin();
        let b = y * 1.101_f32 - x.cos();
        let mut acc = Accumulator204::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.057_f32 + y.sin();
        let b = y * 3.517_f32 - x.cos();
        let mut acc = Accumulator204::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.435_f32 + y.sin();
        let b = y * 6.178_f32 - x.cos();
        let mut acc = Accumulator204::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.942_f32 + y.sin();
        let b = y * 7.121_f32 - x.cos();
        let mut acc = Accumulator204::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.651_f32 + y.sin();
        let b = y * 0.72_f32 - x.cos();
        let mut acc = Accumulator204::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.123_f32 + y.sin();
        let b = y * 4.068_f32 - x.cos();
        let mut acc = Accumulator204::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.965_f32 + y.sin();
        let b = y * 9.123_f32 - x.cos();
        let mut acc = Accumulator204::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.6_f32 + y.sin();
        let b = y * 1.693_f32 - x.cos();
        let mut acc = Accumulator204::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.166_f32 + y.sin();
        let b = y * 4.306_f32 - x.cos();
        let mut acc = Accumulator204::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.61_f32 + y.sin();
        let b = y * 7.155_f32 - x.cos();
        let mut acc = Accumulator204::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.174_f32 + y.sin();
        let b = y * 2.322_f32 - x.cos();
        let mut acc = Accumulator204::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.824_f32 + y.sin();
        let b = y * 9.059_f32 - x.cos();
        let mut acc = Accumulator204::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.339_f32 + y.sin();
        let b = y * 1.666_f32 - x.cos();
        let mut acc = Accumulator204::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.127_f32 + y.sin();
        let b = y * 2.961_f32 - x.cos();
        let mut acc = Accumulator204::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.784_f32 + y.sin();
        let b = y * 2.52_f32 - x.cos();
        let mut acc = Accumulator204::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.256_f32 + y.sin();
        let b = y * 2.405_f32 - x.cos();
        let mut acc = Accumulator204::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.442_f32 + y.sin();
        let b = y * 7.131_f32 - x.cos();
        let mut acc = Accumulator204::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.209_f32 + y.sin();
        let b = y * 1.801_f32 - x.cos();
        let mut acc = Accumulator204::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.738_f32 + y.sin();
        let b = y * 2.624_f32 - x.cos();
        let mut acc = Accumulator204::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.267_f32 + y.sin();
        let b = y * 2.383_f32 - x.cos();
        let mut acc = Accumulator204::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_204(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m204-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_204() -> f32 {
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
        total += (dep_touch_204(total as u64) % 997) as f32;
        total
    }
}

pub mod m205 {
    use super::*;

    pub struct Accumulator205<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator205<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.568_f32 + y.sin();
        let b = y * 4.834_f32 - x.cos();
        let mut acc = Accumulator205::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.828_f32 + y.sin();
        let b = y * 0.182_f32 - x.cos();
        let mut acc = Accumulator205::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.509_f32 + y.sin();
        let b = y * 8.608_f32 - x.cos();
        let mut acc = Accumulator205::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.429_f32 + y.sin();
        let b = y * 4.998_f32 - x.cos();
        let mut acc = Accumulator205::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.564_f32 + y.sin();
        let b = y * 3.699_f32 - x.cos();
        let mut acc = Accumulator205::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.395_f32 + y.sin();
        let b = y * 9.474_f32 - x.cos();
        let mut acc = Accumulator205::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.78_f32 + y.sin();
        let b = y * 9.69_f32 - x.cos();
        let mut acc = Accumulator205::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.703_f32 + y.sin();
        let b = y * 8.973_f32 - x.cos();
        let mut acc = Accumulator205::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.15_f32 + y.sin();
        let b = y * 1.892_f32 - x.cos();
        let mut acc = Accumulator205::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.393_f32 + y.sin();
        let b = y * 9.599_f32 - x.cos();
        let mut acc = Accumulator205::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.338_f32 + y.sin();
        let b = y * 8.961_f32 - x.cos();
        let mut acc = Accumulator205::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.053_f32 + y.sin();
        let b = y * 4.662_f32 - x.cos();
        let mut acc = Accumulator205::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.392_f32 + y.sin();
        let b = y * 4.637_f32 - x.cos();
        let mut acc = Accumulator205::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.606_f32 + y.sin();
        let b = y * 3.742_f32 - x.cos();
        let mut acc = Accumulator205::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.422_f32 + y.sin();
        let b = y * 6.399_f32 - x.cos();
        let mut acc = Accumulator205::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.585_f32 + y.sin();
        let b = y * 4.167_f32 - x.cos();
        let mut acc = Accumulator205::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.97_f32 + y.sin();
        let b = y * 2.255_f32 - x.cos();
        let mut acc = Accumulator205::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.34_f32 + y.sin();
        let b = y * 7.914_f32 - x.cos();
        let mut acc = Accumulator205::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.785_f32 + y.sin();
        let b = y * 6.968_f32 - x.cos();
        let mut acc = Accumulator205::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.857_f32 + y.sin();
        let b = y * 0.11_f32 - x.cos();
        let mut acc = Accumulator205::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_205(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_205() -> f32 {
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
        total += (dep_touch_205(total as u64) % 997) as f32;
        total
    }
}

pub mod m206 {
    use super::*;

    pub struct Accumulator206<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator206<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.172_f32 + y.sin();
        let b = y * 3.657_f32 - x.cos();
        let mut acc = Accumulator206::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.795_f32 + y.sin();
        let b = y * 6.166_f32 - x.cos();
        let mut acc = Accumulator206::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.337_f32 + y.sin();
        let b = y * 2.23_f32 - x.cos();
        let mut acc = Accumulator206::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.796_f32 + y.sin();
        let b = y * 1.092_f32 - x.cos();
        let mut acc = Accumulator206::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.742_f32 + y.sin();
        let b = y * 4.66_f32 - x.cos();
        let mut acc = Accumulator206::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.158_f32 + y.sin();
        let b = y * 3.581_f32 - x.cos();
        let mut acc = Accumulator206::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.238_f32 + y.sin();
        let b = y * 0.138_f32 - x.cos();
        let mut acc = Accumulator206::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.338_f32 + y.sin();
        let b = y * 5.881_f32 - x.cos();
        let mut acc = Accumulator206::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.003_f32 + y.sin();
        let b = y * 4.968_f32 - x.cos();
        let mut acc = Accumulator206::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.492_f32 + y.sin();
        let b = y * 0.194_f32 - x.cos();
        let mut acc = Accumulator206::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.132_f32 + y.sin();
        let b = y * 3.574_f32 - x.cos();
        let mut acc = Accumulator206::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.281_f32 + y.sin();
        let b = y * 8.872_f32 - x.cos();
        let mut acc = Accumulator206::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.231_f32 + y.sin();
        let b = y * 1.704_f32 - x.cos();
        let mut acc = Accumulator206::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.504_f32 + y.sin();
        let b = y * 0.742_f32 - x.cos();
        let mut acc = Accumulator206::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.122_f32 + y.sin();
        let b = y * 6.819_f32 - x.cos();
        let mut acc = Accumulator206::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.961_f32 + y.sin();
        let b = y * 6.601_f32 - x.cos();
        let mut acc = Accumulator206::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.249_f32 + y.sin();
        let b = y * 7.433_f32 - x.cos();
        let mut acc = Accumulator206::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.521_f32 + y.sin();
        let b = y * 7.287_f32 - x.cos();
        let mut acc = Accumulator206::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.99_f32 + y.sin();
        let b = y * 0.681_f32 - x.cos();
        let mut acc = Accumulator206::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.509_f32 + y.sin();
        let b = y * 6.171_f32 - x.cos();
        let mut acc = Accumulator206::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_206(seed: u64) -> u64 {
        let re = Regex::new(r"m206-(\d+)").unwrap();
        let hay = format!("m206-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_206() -> f32 {
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
        total += (dep_touch_206(total as u64) % 997) as f32;
        total
    }
}

pub mod m207 {
    use super::*;

    pub struct Accumulator207<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator207<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.353_f32 + y.sin();
        let b = y * 9.399_f32 - x.cos();
        let mut acc = Accumulator207::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.338_f32 + y.sin();
        let b = y * 8.01_f32 - x.cos();
        let mut acc = Accumulator207::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.044_f32 + y.sin();
        let b = y * 6.667_f32 - x.cos();
        let mut acc = Accumulator207::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.075_f32 + y.sin();
        let b = y * 9.728_f32 - x.cos();
        let mut acc = Accumulator207::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.348_f32 + y.sin();
        let b = y * 1.779_f32 - x.cos();
        let mut acc = Accumulator207::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.291_f32 + y.sin();
        let b = y * 5.393_f32 - x.cos();
        let mut acc = Accumulator207::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.802_f32 + y.sin();
        let b = y * 4.032_f32 - x.cos();
        let mut acc = Accumulator207::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.711_f32 + y.sin();
        let b = y * 1.334_f32 - x.cos();
        let mut acc = Accumulator207::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.693_f32 + y.sin();
        let b = y * 8.861_f32 - x.cos();
        let mut acc = Accumulator207::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.281_f32 + y.sin();
        let b = y * 3.158_f32 - x.cos();
        let mut acc = Accumulator207::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.381_f32 + y.sin();
        let b = y * 0.44_f32 - x.cos();
        let mut acc = Accumulator207::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.176_f32 + y.sin();
        let b = y * 9.032_f32 - x.cos();
        let mut acc = Accumulator207::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.125_f32 + y.sin();
        let b = y * 8.401_f32 - x.cos();
        let mut acc = Accumulator207::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.502_f32 + y.sin();
        let b = y * 9.53_f32 - x.cos();
        let mut acc = Accumulator207::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.344_f32 + y.sin();
        let b = y * 7.504_f32 - x.cos();
        let mut acc = Accumulator207::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.909_f32 + y.sin();
        let b = y * 6.411_f32 - x.cos();
        let mut acc = Accumulator207::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.405_f32 + y.sin();
        let b = y * 2.558_f32 - x.cos();
        let mut acc = Accumulator207::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.134_f32 + y.sin();
        let b = y * 1.992_f32 - x.cos();
        let mut acc = Accumulator207::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.325_f32 + y.sin();
        let b = y * 4.151_f32 - x.cos();
        let mut acc = Accumulator207::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.327_f32 + y.sin();
        let b = y * 4.184_f32 - x.cos();
        let mut acc = Accumulator207::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_207(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_207() -> f32 {
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
        total += (dep_touch_207(total as u64) % 997) as f32;
        total
    }
}

pub mod m208 {
    use super::*;

    pub struct Accumulator208<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator208<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.514_f32 + y.sin();
        let b = y * 8.84_f32 - x.cos();
        let mut acc = Accumulator208::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.793_f32 + y.sin();
        let b = y * 8.518_f32 - x.cos();
        let mut acc = Accumulator208::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.22_f32 + y.sin();
        let b = y * 7.995_f32 - x.cos();
        let mut acc = Accumulator208::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.504_f32 + y.sin();
        let b = y * 7.217_f32 - x.cos();
        let mut acc = Accumulator208::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.403_f32 + y.sin();
        let b = y * 8.425_f32 - x.cos();
        let mut acc = Accumulator208::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.995_f32 + y.sin();
        let b = y * 9.637_f32 - x.cos();
        let mut acc = Accumulator208::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.211_f32 + y.sin();
        let b = y * 2.105_f32 - x.cos();
        let mut acc = Accumulator208::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.999_f32 + y.sin();
        let b = y * 7.558_f32 - x.cos();
        let mut acc = Accumulator208::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.935_f32 + y.sin();
        let b = y * 5.752_f32 - x.cos();
        let mut acc = Accumulator208::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.679_f32 + y.sin();
        let b = y * 3.865_f32 - x.cos();
        let mut acc = Accumulator208::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.116_f32 + y.sin();
        let b = y * 3.04_f32 - x.cos();
        let mut acc = Accumulator208::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.371_f32 + y.sin();
        let b = y * 3.301_f32 - x.cos();
        let mut acc = Accumulator208::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.943_f32 + y.sin();
        let b = y * 8.329_f32 - x.cos();
        let mut acc = Accumulator208::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.365_f32 + y.sin();
        let b = y * 7.839_f32 - x.cos();
        let mut acc = Accumulator208::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.322_f32 + y.sin();
        let b = y * 2.643_f32 - x.cos();
        let mut acc = Accumulator208::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.512_f32 + y.sin();
        let b = y * 3.075_f32 - x.cos();
        let mut acc = Accumulator208::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.088_f32 + y.sin();
        let b = y * 8.853_f32 - x.cos();
        let mut acc = Accumulator208::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.603_f32 + y.sin();
        let b = y * 2.023_f32 - x.cos();
        let mut acc = Accumulator208::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.568_f32 + y.sin();
        let b = y * 3.394_f32 - x.cos();
        let mut acc = Accumulator208::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.8_f32 + y.sin();
        let b = y * 2.793_f32 - x.cos();
        let mut acc = Accumulator208::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_208(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(208u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_208() -> f32 {
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
        total += (dep_touch_208(total as u64) % 997) as f32;
        total
    }
}

pub mod m209 {
    use super::*;

    pub struct Accumulator209<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator209<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.313_f32 + y.sin();
        let b = y * 2.371_f32 - x.cos();
        let mut acc = Accumulator209::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.269_f32 + y.sin();
        let b = y * 9.207_f32 - x.cos();
        let mut acc = Accumulator209::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.659_f32 + y.sin();
        let b = y * 0.382_f32 - x.cos();
        let mut acc = Accumulator209::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.309_f32 + y.sin();
        let b = y * 9.06_f32 - x.cos();
        let mut acc = Accumulator209::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.559_f32 + y.sin();
        let b = y * 5.148_f32 - x.cos();
        let mut acc = Accumulator209::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.832_f32 + y.sin();
        let b = y * 2.067_f32 - x.cos();
        let mut acc = Accumulator209::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.148_f32 + y.sin();
        let b = y * 1.91_f32 - x.cos();
        let mut acc = Accumulator209::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.175_f32 + y.sin();
        let b = y * 8.565_f32 - x.cos();
        let mut acc = Accumulator209::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.574_f32 + y.sin();
        let b = y * 1.788_f32 - x.cos();
        let mut acc = Accumulator209::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.577_f32 + y.sin();
        let b = y * 1.707_f32 - x.cos();
        let mut acc = Accumulator209::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.836_f32 + y.sin();
        let b = y * 7.933_f32 - x.cos();
        let mut acc = Accumulator209::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.701_f32 + y.sin();
        let b = y * 4.951_f32 - x.cos();
        let mut acc = Accumulator209::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.471_f32 + y.sin();
        let b = y * 8.221_f32 - x.cos();
        let mut acc = Accumulator209::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.305_f32 + y.sin();
        let b = y * 1.935_f32 - x.cos();
        let mut acc = Accumulator209::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.338_f32 + y.sin();
        let b = y * 7.356_f32 - x.cos();
        let mut acc = Accumulator209::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.208_f32 + y.sin();
        let b = y * 2.848_f32 - x.cos();
        let mut acc = Accumulator209::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.767_f32 + y.sin();
        let b = y * 7.195_f32 - x.cos();
        let mut acc = Accumulator209::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.739_f32 + y.sin();
        let b = y * 3.114_f32 - x.cos();
        let mut acc = Accumulator209::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.15_f32 + y.sin();
        let b = y * 3.95_f32 - x.cos();
        let mut acc = Accumulator209::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.586_f32 + y.sin();
        let b = y * 2.164_f32 - x.cos();
        let mut acc = Accumulator209::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_209(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_209() -> f32 {
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
        total += (dep_touch_209(total as u64) % 997) as f32;
        total
    }
}

pub mod m210 {
    use super::*;

    pub struct Accumulator210<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator210<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.38_f32 + y.sin();
        let b = y * 0.412_f32 - x.cos();
        let mut acc = Accumulator210::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.483_f32 + y.sin();
        let b = y * 3.357_f32 - x.cos();
        let mut acc = Accumulator210::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.019_f32 + y.sin();
        let b = y * 0.492_f32 - x.cos();
        let mut acc = Accumulator210::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.192_f32 + y.sin();
        let b = y * 7.864_f32 - x.cos();
        let mut acc = Accumulator210::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.14_f32 + y.sin();
        let b = y * 6.519_f32 - x.cos();
        let mut acc = Accumulator210::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.811_f32 + y.sin();
        let b = y * 1.45_f32 - x.cos();
        let mut acc = Accumulator210::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.655_f32 + y.sin();
        let b = y * 9.8_f32 - x.cos();
        let mut acc = Accumulator210::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.623_f32 + y.sin();
        let b = y * 1.979_f32 - x.cos();
        let mut acc = Accumulator210::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.812_f32 + y.sin();
        let b = y * 1.603_f32 - x.cos();
        let mut acc = Accumulator210::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.395_f32 + y.sin();
        let b = y * 6.242_f32 - x.cos();
        let mut acc = Accumulator210::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.308_f32 + y.sin();
        let b = y * 1.442_f32 - x.cos();
        let mut acc = Accumulator210::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.027_f32 + y.sin();
        let b = y * 1.128_f32 - x.cos();
        let mut acc = Accumulator210::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.012_f32 + y.sin();
        let b = y * 9.01_f32 - x.cos();
        let mut acc = Accumulator210::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.62_f32 + y.sin();
        let b = y * 6.705_f32 - x.cos();
        let mut acc = Accumulator210::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.507_f32 + y.sin();
        let b = y * 1.002_f32 - x.cos();
        let mut acc = Accumulator210::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.384_f32 + y.sin();
        let b = y * 7.235_f32 - x.cos();
        let mut acc = Accumulator210::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.989_f32 + y.sin();
        let b = y * 2.652_f32 - x.cos();
        let mut acc = Accumulator210::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.181_f32 + y.sin();
        let b = y * 4.432_f32 - x.cos();
        let mut acc = Accumulator210::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.14_f32 + y.sin();
        let b = y * 6.135_f32 - x.cos();
        let mut acc = Accumulator210::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.023_f32 + y.sin();
        let b = y * 0.575_f32 - x.cos();
        let mut acc = Accumulator210::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_210(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_210() -> f32 {
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
        total += (dep_touch_210(total as u64) % 997) as f32;
        total
    }
}

pub mod m211 {
    use super::*;

    pub struct Accumulator211<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator211<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.479_f32 + y.sin();
        let b = y * 1.252_f32 - x.cos();
        let mut acc = Accumulator211::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.762_f32 + y.sin();
        let b = y * 8.557_f32 - x.cos();
        let mut acc = Accumulator211::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.767_f32 + y.sin();
        let b = y * 2.86_f32 - x.cos();
        let mut acc = Accumulator211::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.935_f32 + y.sin();
        let b = y * 5.308_f32 - x.cos();
        let mut acc = Accumulator211::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.279_f32 + y.sin();
        let b = y * 6.012_f32 - x.cos();
        let mut acc = Accumulator211::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.189_f32 + y.sin();
        let b = y * 3.939_f32 - x.cos();
        let mut acc = Accumulator211::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.808_f32 + y.sin();
        let b = y * 4.323_f32 - x.cos();
        let mut acc = Accumulator211::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.735_f32 + y.sin();
        let b = y * 8.865_f32 - x.cos();
        let mut acc = Accumulator211::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.983_f32 + y.sin();
        let b = y * 5.88_f32 - x.cos();
        let mut acc = Accumulator211::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.691_f32 + y.sin();
        let b = y * 2.802_f32 - x.cos();
        let mut acc = Accumulator211::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.485_f32 + y.sin();
        let b = y * 4.97_f32 - x.cos();
        let mut acc = Accumulator211::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.011_f32 + y.sin();
        let b = y * 8.579_f32 - x.cos();
        let mut acc = Accumulator211::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.82_f32 + y.sin();
        let b = y * 0.331_f32 - x.cos();
        let mut acc = Accumulator211::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.772_f32 + y.sin();
        let b = y * 3.483_f32 - x.cos();
        let mut acc = Accumulator211::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.198_f32 + y.sin();
        let b = y * 0.986_f32 - x.cos();
        let mut acc = Accumulator211::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.571_f32 + y.sin();
        let b = y * 1.226_f32 - x.cos();
        let mut acc = Accumulator211::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.511_f32 + y.sin();
        let b = y * 9.358_f32 - x.cos();
        let mut acc = Accumulator211::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.288_f32 + y.sin();
        let b = y * 5.243_f32 - x.cos();
        let mut acc = Accumulator211::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.575_f32 + y.sin();
        let b = y * 3.296_f32 - x.cos();
        let mut acc = Accumulator211::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.686_f32 + y.sin();
        let b = y * 1.248_f32 - x.cos();
        let mut acc = Accumulator211::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_211(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m211-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_211() -> f32 {
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
        total += (dep_touch_211(total as u64) % 997) as f32;
        total
    }
}

pub mod m212 {
    use super::*;

    pub struct Accumulator212<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator212<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.874_f32 + y.sin();
        let b = y * 9.717_f32 - x.cos();
        let mut acc = Accumulator212::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.372_f32 + y.sin();
        let b = y * 8.044_f32 - x.cos();
        let mut acc = Accumulator212::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.267_f32 + y.sin();
        let b = y * 0.538_f32 - x.cos();
        let mut acc = Accumulator212::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.758_f32 + y.sin();
        let b = y * 9.87_f32 - x.cos();
        let mut acc = Accumulator212::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.944_f32 + y.sin();
        let b = y * 5.15_f32 - x.cos();
        let mut acc = Accumulator212::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.336_f32 + y.sin();
        let b = y * 6.174_f32 - x.cos();
        let mut acc = Accumulator212::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.399_f32 + y.sin();
        let b = y * 2.298_f32 - x.cos();
        let mut acc = Accumulator212::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.561_f32 + y.sin();
        let b = y * 1.386_f32 - x.cos();
        let mut acc = Accumulator212::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.301_f32 + y.sin();
        let b = y * 7.135_f32 - x.cos();
        let mut acc = Accumulator212::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.67_f32 + y.sin();
        let b = y * 5.541_f32 - x.cos();
        let mut acc = Accumulator212::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.551_f32 + y.sin();
        let b = y * 5.209_f32 - x.cos();
        let mut acc = Accumulator212::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.609_f32 + y.sin();
        let b = y * 1.315_f32 - x.cos();
        let mut acc = Accumulator212::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.077_f32 + y.sin();
        let b = y * 7.375_f32 - x.cos();
        let mut acc = Accumulator212::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.816_f32 + y.sin();
        let b = y * 4.255_f32 - x.cos();
        let mut acc = Accumulator212::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.264_f32 + y.sin();
        let b = y * 2.084_f32 - x.cos();
        let mut acc = Accumulator212::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.694_f32 + y.sin();
        let b = y * 9.283_f32 - x.cos();
        let mut acc = Accumulator212::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.672_f32 + y.sin();
        let b = y * 3.522_f32 - x.cos();
        let mut acc = Accumulator212::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.295_f32 + y.sin();
        let b = y * 1.305_f32 - x.cos();
        let mut acc = Accumulator212::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.694_f32 + y.sin();
        let b = y * 7.008_f32 - x.cos();
        let mut acc = Accumulator212::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.217_f32 + y.sin();
        let b = y * 2.479_f32 - x.cos();
        let mut acc = Accumulator212::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_212(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_212() -> f32 {
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
        total += (dep_touch_212(total as u64) % 997) as f32;
        total
    }
}

pub mod m213 {
    use super::*;

    pub struct Accumulator213<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator213<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.264_f32 + y.sin();
        let b = y * 7.786_f32 - x.cos();
        let mut acc = Accumulator213::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.056_f32 + y.sin();
        let b = y * 9.73_f32 - x.cos();
        let mut acc = Accumulator213::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.641_f32 + y.sin();
        let b = y * 8.391_f32 - x.cos();
        let mut acc = Accumulator213::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.683_f32 + y.sin();
        let b = y * 1.698_f32 - x.cos();
        let mut acc = Accumulator213::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.259_f32 + y.sin();
        let b = y * 2.57_f32 - x.cos();
        let mut acc = Accumulator213::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.212_f32 + y.sin();
        let b = y * 2.761_f32 - x.cos();
        let mut acc = Accumulator213::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.165_f32 + y.sin();
        let b = y * 4.075_f32 - x.cos();
        let mut acc = Accumulator213::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.107_f32 + y.sin();
        let b = y * 9.519_f32 - x.cos();
        let mut acc = Accumulator213::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.436_f32 + y.sin();
        let b = y * 1.889_f32 - x.cos();
        let mut acc = Accumulator213::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.038_f32 + y.sin();
        let b = y * 2.5_f32 - x.cos();
        let mut acc = Accumulator213::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.123_f32 + y.sin();
        let b = y * 2.775_f32 - x.cos();
        let mut acc = Accumulator213::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.343_f32 + y.sin();
        let b = y * 2.886_f32 - x.cos();
        let mut acc = Accumulator213::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.636_f32 + y.sin();
        let b = y * 8.573_f32 - x.cos();
        let mut acc = Accumulator213::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.39_f32 + y.sin();
        let b = y * 5.15_f32 - x.cos();
        let mut acc = Accumulator213::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.169_f32 + y.sin();
        let b = y * 9.09_f32 - x.cos();
        let mut acc = Accumulator213::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.464_f32 + y.sin();
        let b = y * 9.378_f32 - x.cos();
        let mut acc = Accumulator213::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.316_f32 + y.sin();
        let b = y * 6.84_f32 - x.cos();
        let mut acc = Accumulator213::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.609_f32 + y.sin();
        let b = y * 1.168_f32 - x.cos();
        let mut acc = Accumulator213::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.058_f32 + y.sin();
        let b = y * 5.321_f32 - x.cos();
        let mut acc = Accumulator213::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.326_f32 + y.sin();
        let b = y * 1.731_f32 - x.cos();
        let mut acc = Accumulator213::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_213(seed: u64) -> u64 {
        let re = Regex::new(r"m213-(\d+)").unwrap();
        let hay = format!("m213-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_213() -> f32 {
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
        total += (dep_touch_213(total as u64) % 997) as f32;
        total
    }
}

pub mod m214 {
    use super::*;

    pub struct Accumulator214<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator214<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.081_f32 + y.sin();
        let b = y * 7.979_f32 - x.cos();
        let mut acc = Accumulator214::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.083_f32 + y.sin();
        let b = y * 0.137_f32 - x.cos();
        let mut acc = Accumulator214::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.394_f32 + y.sin();
        let b = y * 3.076_f32 - x.cos();
        let mut acc = Accumulator214::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.54_f32 + y.sin();
        let b = y * 3.921_f32 - x.cos();
        let mut acc = Accumulator214::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.539_f32 + y.sin();
        let b = y * 1.619_f32 - x.cos();
        let mut acc = Accumulator214::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.379_f32 + y.sin();
        let b = y * 7.867_f32 - x.cos();
        let mut acc = Accumulator214::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.081_f32 + y.sin();
        let b = y * 1.42_f32 - x.cos();
        let mut acc = Accumulator214::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.339_f32 + y.sin();
        let b = y * 7.178_f32 - x.cos();
        let mut acc = Accumulator214::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.963_f32 + y.sin();
        let b = y * 4.008_f32 - x.cos();
        let mut acc = Accumulator214::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.359_f32 + y.sin();
        let b = y * 9.596_f32 - x.cos();
        let mut acc = Accumulator214::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.547_f32 + y.sin();
        let b = y * 4.74_f32 - x.cos();
        let mut acc = Accumulator214::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.272_f32 + y.sin();
        let b = y * 8.808_f32 - x.cos();
        let mut acc = Accumulator214::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.526_f32 + y.sin();
        let b = y * 7.555_f32 - x.cos();
        let mut acc = Accumulator214::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.5_f32 + y.sin();
        let b = y * 2.547_f32 - x.cos();
        let mut acc = Accumulator214::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.066_f32 + y.sin();
        let b = y * 4.193_f32 - x.cos();
        let mut acc = Accumulator214::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.065_f32 + y.sin();
        let b = y * 9.379_f32 - x.cos();
        let mut acc = Accumulator214::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.22_f32 + y.sin();
        let b = y * 1.265_f32 - x.cos();
        let mut acc = Accumulator214::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.818_f32 + y.sin();
        let b = y * 8.971_f32 - x.cos();
        let mut acc = Accumulator214::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.249_f32 + y.sin();
        let b = y * 9.22_f32 - x.cos();
        let mut acc = Accumulator214::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.94_f32 + y.sin();
        let b = y * 0.307_f32 - x.cos();
        let mut acc = Accumulator214::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_214(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_214() -> f32 {
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
        total += (dep_touch_214(total as u64) % 997) as f32;
        total
    }
}

pub mod m215 {
    use super::*;

    pub struct Accumulator215<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator215<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.779_f32 + y.sin();
        let b = y * 3.668_f32 - x.cos();
        let mut acc = Accumulator215::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.865_f32 + y.sin();
        let b = y * 4.533_f32 - x.cos();
        let mut acc = Accumulator215::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.382_f32 + y.sin();
        let b = y * 4.686_f32 - x.cos();
        let mut acc = Accumulator215::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.045_f32 + y.sin();
        let b = y * 1.695_f32 - x.cos();
        let mut acc = Accumulator215::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.063_f32 + y.sin();
        let b = y * 8.941_f32 - x.cos();
        let mut acc = Accumulator215::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.883_f32 + y.sin();
        let b = y * 7.719_f32 - x.cos();
        let mut acc = Accumulator215::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.877_f32 + y.sin();
        let b = y * 8.795_f32 - x.cos();
        let mut acc = Accumulator215::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.423_f32 + y.sin();
        let b = y * 0.8_f32 - x.cos();
        let mut acc = Accumulator215::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.946_f32 + y.sin();
        let b = y * 1.482_f32 - x.cos();
        let mut acc = Accumulator215::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.598_f32 + y.sin();
        let b = y * 3.83_f32 - x.cos();
        let mut acc = Accumulator215::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.59_f32 + y.sin();
        let b = y * 8.147_f32 - x.cos();
        let mut acc = Accumulator215::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.243_f32 + y.sin();
        let b = y * 6.498_f32 - x.cos();
        let mut acc = Accumulator215::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.452_f32 + y.sin();
        let b = y * 6.875_f32 - x.cos();
        let mut acc = Accumulator215::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.878_f32 + y.sin();
        let b = y * 1.244_f32 - x.cos();
        let mut acc = Accumulator215::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.589_f32 + y.sin();
        let b = y * 3.014_f32 - x.cos();
        let mut acc = Accumulator215::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.97_f32 + y.sin();
        let b = y * 3.639_f32 - x.cos();
        let mut acc = Accumulator215::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.543_f32 + y.sin();
        let b = y * 8.801_f32 - x.cos();
        let mut acc = Accumulator215::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.139_f32 + y.sin();
        let b = y * 1.887_f32 - x.cos();
        let mut acc = Accumulator215::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.299_f32 + y.sin();
        let b = y * 9.872_f32 - x.cos();
        let mut acc = Accumulator215::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.437_f32 + y.sin();
        let b = y * 6.313_f32 - x.cos();
        let mut acc = Accumulator215::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_215(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(215u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_215() -> f32 {
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
        total += (dep_touch_215(total as u64) % 997) as f32;
        total
    }
}

pub mod m216 {
    use super::*;

    pub struct Accumulator216<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator216<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.311_f32 + y.sin();
        let b = y * 4.191_f32 - x.cos();
        let mut acc = Accumulator216::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.429_f32 + y.sin();
        let b = y * 1.621_f32 - x.cos();
        let mut acc = Accumulator216::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.71_f32 + y.sin();
        let b = y * 4.204_f32 - x.cos();
        let mut acc = Accumulator216::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.511_f32 + y.sin();
        let b = y * 3.255_f32 - x.cos();
        let mut acc = Accumulator216::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.148_f32 + y.sin();
        let b = y * 6.706_f32 - x.cos();
        let mut acc = Accumulator216::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.126_f32 + y.sin();
        let b = y * 2.054_f32 - x.cos();
        let mut acc = Accumulator216::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.364_f32 + y.sin();
        let b = y * 7.913_f32 - x.cos();
        let mut acc = Accumulator216::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.288_f32 + y.sin();
        let b = y * 3.245_f32 - x.cos();
        let mut acc = Accumulator216::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.498_f32 + y.sin();
        let b = y * 1.879_f32 - x.cos();
        let mut acc = Accumulator216::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.137_f32 + y.sin();
        let b = y * 3.343_f32 - x.cos();
        let mut acc = Accumulator216::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.84_f32 + y.sin();
        let b = y * 1.724_f32 - x.cos();
        let mut acc = Accumulator216::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.102_f32 + y.sin();
        let b = y * 6.894_f32 - x.cos();
        let mut acc = Accumulator216::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.747_f32 + y.sin();
        let b = y * 5.826_f32 - x.cos();
        let mut acc = Accumulator216::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.487_f32 + y.sin();
        let b = y * 2.181_f32 - x.cos();
        let mut acc = Accumulator216::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.668_f32 + y.sin();
        let b = y * 3.65_f32 - x.cos();
        let mut acc = Accumulator216::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.95_f32 + y.sin();
        let b = y * 0.92_f32 - x.cos();
        let mut acc = Accumulator216::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.966_f32 + y.sin();
        let b = y * 2.556_f32 - x.cos();
        let mut acc = Accumulator216::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.402_f32 + y.sin();
        let b = y * 8.375_f32 - x.cos();
        let mut acc = Accumulator216::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.84_f32 + y.sin();
        let b = y * 1.772_f32 - x.cos();
        let mut acc = Accumulator216::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.432_f32 + y.sin();
        let b = y * 7.898_f32 - x.cos();
        let mut acc = Accumulator216::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_216(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_216() -> f32 {
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
        total += (dep_touch_216(total as u64) % 997) as f32;
        total
    }
}

pub mod m217 {
    use super::*;

    pub struct Accumulator217<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator217<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.767_f32 + y.sin();
        let b = y * 0.971_f32 - x.cos();
        let mut acc = Accumulator217::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.558_f32 + y.sin();
        let b = y * 9.184_f32 - x.cos();
        let mut acc = Accumulator217::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.848_f32 + y.sin();
        let b = y * 6.865_f32 - x.cos();
        let mut acc = Accumulator217::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.312_f32 + y.sin();
        let b = y * 9.146_f32 - x.cos();
        let mut acc = Accumulator217::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.105_f32 + y.sin();
        let b = y * 1.465_f32 - x.cos();
        let mut acc = Accumulator217::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.704_f32 + y.sin();
        let b = y * 4.076_f32 - x.cos();
        let mut acc = Accumulator217::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.967_f32 + y.sin();
        let b = y * 2.503_f32 - x.cos();
        let mut acc = Accumulator217::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.309_f32 + y.sin();
        let b = y * 3.553_f32 - x.cos();
        let mut acc = Accumulator217::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.413_f32 + y.sin();
        let b = y * 1.92_f32 - x.cos();
        let mut acc = Accumulator217::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.895_f32 + y.sin();
        let b = y * 3.651_f32 - x.cos();
        let mut acc = Accumulator217::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.388_f32 + y.sin();
        let b = y * 5.44_f32 - x.cos();
        let mut acc = Accumulator217::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.109_f32 + y.sin();
        let b = y * 8.645_f32 - x.cos();
        let mut acc = Accumulator217::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.222_f32 + y.sin();
        let b = y * 2.239_f32 - x.cos();
        let mut acc = Accumulator217::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.693_f32 + y.sin();
        let b = y * 7.879_f32 - x.cos();
        let mut acc = Accumulator217::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.438_f32 + y.sin();
        let b = y * 6.856_f32 - x.cos();
        let mut acc = Accumulator217::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.881_f32 + y.sin();
        let b = y * 5.046_f32 - x.cos();
        let mut acc = Accumulator217::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.039_f32 + y.sin();
        let b = y * 8.74_f32 - x.cos();
        let mut acc = Accumulator217::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.701_f32 + y.sin();
        let b = y * 6.726_f32 - x.cos();
        let mut acc = Accumulator217::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.96_f32 + y.sin();
        let b = y * 6.871_f32 - x.cos();
        let mut acc = Accumulator217::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.676_f32 + y.sin();
        let b = y * 2.611_f32 - x.cos();
        let mut acc = Accumulator217::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_217(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_217() -> f32 {
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
        total += (dep_touch_217(total as u64) % 997) as f32;
        total
    }
}

pub mod m218 {
    use super::*;

    pub struct Accumulator218<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator218<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.061_f32 + y.sin();
        let b = y * 0.162_f32 - x.cos();
        let mut acc = Accumulator218::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.634_f32 + y.sin();
        let b = y * 8.123_f32 - x.cos();
        let mut acc = Accumulator218::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.697_f32 + y.sin();
        let b = y * 3.471_f32 - x.cos();
        let mut acc = Accumulator218::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.804_f32 + y.sin();
        let b = y * 4.052_f32 - x.cos();
        let mut acc = Accumulator218::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.317_f32 + y.sin();
        let b = y * 2.754_f32 - x.cos();
        let mut acc = Accumulator218::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.068_f32 + y.sin();
        let b = y * 7.249_f32 - x.cos();
        let mut acc = Accumulator218::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.84_f32 + y.sin();
        let b = y * 7.092_f32 - x.cos();
        let mut acc = Accumulator218::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.29_f32 + y.sin();
        let b = y * 2.996_f32 - x.cos();
        let mut acc = Accumulator218::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.177_f32 + y.sin();
        let b = y * 4.539_f32 - x.cos();
        let mut acc = Accumulator218::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.609_f32 + y.sin();
        let b = y * 2.64_f32 - x.cos();
        let mut acc = Accumulator218::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.522_f32 + y.sin();
        let b = y * 0.245_f32 - x.cos();
        let mut acc = Accumulator218::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.214_f32 + y.sin();
        let b = y * 4.889_f32 - x.cos();
        let mut acc = Accumulator218::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.526_f32 + y.sin();
        let b = y * 3.789_f32 - x.cos();
        let mut acc = Accumulator218::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.292_f32 + y.sin();
        let b = y * 0.457_f32 - x.cos();
        let mut acc = Accumulator218::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.786_f32 + y.sin();
        let b = y * 6.646_f32 - x.cos();
        let mut acc = Accumulator218::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.882_f32 + y.sin();
        let b = y * 6.278_f32 - x.cos();
        let mut acc = Accumulator218::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.809_f32 + y.sin();
        let b = y * 9.357_f32 - x.cos();
        let mut acc = Accumulator218::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.563_f32 + y.sin();
        let b = y * 0.247_f32 - x.cos();
        let mut acc = Accumulator218::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.981_f32 + y.sin();
        let b = y * 8.69_f32 - x.cos();
        let mut acc = Accumulator218::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.571_f32 + y.sin();
        let b = y * 0.431_f32 - x.cos();
        let mut acc = Accumulator218::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_218(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m218-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_218() -> f32 {
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
        total += (dep_touch_218(total as u64) % 997) as f32;
        total
    }
}

pub mod m219 {
    use super::*;

    pub struct Accumulator219<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator219<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.915_f32 + y.sin();
        let b = y * 4.236_f32 - x.cos();
        let mut acc = Accumulator219::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.89_f32 + y.sin();
        let b = y * 2.832_f32 - x.cos();
        let mut acc = Accumulator219::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.036_f32 + y.sin();
        let b = y * 6.662_f32 - x.cos();
        let mut acc = Accumulator219::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.984_f32 + y.sin();
        let b = y * 6.819_f32 - x.cos();
        let mut acc = Accumulator219::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.832_f32 + y.sin();
        let b = y * 9.188_f32 - x.cos();
        let mut acc = Accumulator219::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.475_f32 + y.sin();
        let b = y * 2.553_f32 - x.cos();
        let mut acc = Accumulator219::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.252_f32 + y.sin();
        let b = y * 9.49_f32 - x.cos();
        let mut acc = Accumulator219::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.447_f32 + y.sin();
        let b = y * 9.369_f32 - x.cos();
        let mut acc = Accumulator219::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.33_f32 + y.sin();
        let b = y * 7.39_f32 - x.cos();
        let mut acc = Accumulator219::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.722_f32 + y.sin();
        let b = y * 7.523_f32 - x.cos();
        let mut acc = Accumulator219::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.042_f32 + y.sin();
        let b = y * 9.138_f32 - x.cos();
        let mut acc = Accumulator219::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.199_f32 + y.sin();
        let b = y * 0.719_f32 - x.cos();
        let mut acc = Accumulator219::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.407_f32 + y.sin();
        let b = y * 9.328_f32 - x.cos();
        let mut acc = Accumulator219::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.884_f32 + y.sin();
        let b = y * 5.812_f32 - x.cos();
        let mut acc = Accumulator219::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.748_f32 + y.sin();
        let b = y * 3.426_f32 - x.cos();
        let mut acc = Accumulator219::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.984_f32 + y.sin();
        let b = y * 0.397_f32 - x.cos();
        let mut acc = Accumulator219::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.23_f32 + y.sin();
        let b = y * 1.058_f32 - x.cos();
        let mut acc = Accumulator219::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.008_f32 + y.sin();
        let b = y * 9.412_f32 - x.cos();
        let mut acc = Accumulator219::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.74_f32 + y.sin();
        let b = y * 2.117_f32 - x.cos();
        let mut acc = Accumulator219::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.539_f32 + y.sin();
        let b = y * 2.535_f32 - x.cos();
        let mut acc = Accumulator219::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_219(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_219() -> f32 {
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
        total += (dep_touch_219(total as u64) % 997) as f32;
        total
    }
}

pub mod m220 {
    use super::*;

    pub struct Accumulator220<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator220<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.787_f32 + y.sin();
        let b = y * 0.595_f32 - x.cos();
        let mut acc = Accumulator220::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.678_f32 + y.sin();
        let b = y * 2.073_f32 - x.cos();
        let mut acc = Accumulator220::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.875_f32 + y.sin();
        let b = y * 9.703_f32 - x.cos();
        let mut acc = Accumulator220::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.777_f32 + y.sin();
        let b = y * 1.934_f32 - x.cos();
        let mut acc = Accumulator220::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.768_f32 + y.sin();
        let b = y * 7.065_f32 - x.cos();
        let mut acc = Accumulator220::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.57_f32 + y.sin();
        let b = y * 0.998_f32 - x.cos();
        let mut acc = Accumulator220::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.839_f32 + y.sin();
        let b = y * 2.996_f32 - x.cos();
        let mut acc = Accumulator220::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.138_f32 + y.sin();
        let b = y * 3.577_f32 - x.cos();
        let mut acc = Accumulator220::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.601_f32 + y.sin();
        let b = y * 1.397_f32 - x.cos();
        let mut acc = Accumulator220::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.345_f32 + y.sin();
        let b = y * 9.599_f32 - x.cos();
        let mut acc = Accumulator220::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.39_f32 + y.sin();
        let b = y * 7.802_f32 - x.cos();
        let mut acc = Accumulator220::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.076_f32 + y.sin();
        let b = y * 6.81_f32 - x.cos();
        let mut acc = Accumulator220::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.326_f32 + y.sin();
        let b = y * 1.054_f32 - x.cos();
        let mut acc = Accumulator220::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.072_f32 + y.sin();
        let b = y * 6.521_f32 - x.cos();
        let mut acc = Accumulator220::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.737_f32 + y.sin();
        let b = y * 2.032_f32 - x.cos();
        let mut acc = Accumulator220::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.359_f32 + y.sin();
        let b = y * 3.226_f32 - x.cos();
        let mut acc = Accumulator220::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.803_f32 + y.sin();
        let b = y * 0.352_f32 - x.cos();
        let mut acc = Accumulator220::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.081_f32 + y.sin();
        let b = y * 3.148_f32 - x.cos();
        let mut acc = Accumulator220::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.869_f32 + y.sin();
        let b = y * 9.385_f32 - x.cos();
        let mut acc = Accumulator220::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.434_f32 + y.sin();
        let b = y * 2.032_f32 - x.cos();
        let mut acc = Accumulator220::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_220(seed: u64) -> u64 {
        let re = Regex::new(r"m220-(\d+)").unwrap();
        let hay = format!("m220-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_220() -> f32 {
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
        total += (dep_touch_220(total as u64) % 997) as f32;
        total
    }
}

pub mod m221 {
    use super::*;

    pub struct Accumulator221<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator221<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.985_f32 + y.sin();
        let b = y * 5.01_f32 - x.cos();
        let mut acc = Accumulator221::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.254_f32 + y.sin();
        let b = y * 8.354_f32 - x.cos();
        let mut acc = Accumulator221::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.834_f32 + y.sin();
        let b = y * 1.259_f32 - x.cos();
        let mut acc = Accumulator221::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.192_f32 + y.sin();
        let b = y * 2.46_f32 - x.cos();
        let mut acc = Accumulator221::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.17_f32 + y.sin();
        let b = y * 6.544_f32 - x.cos();
        let mut acc = Accumulator221::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.527_f32 + y.sin();
        let b = y * 2.572_f32 - x.cos();
        let mut acc = Accumulator221::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.061_f32 + y.sin();
        let b = y * 2.525_f32 - x.cos();
        let mut acc = Accumulator221::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.606_f32 + y.sin();
        let b = y * 7.184_f32 - x.cos();
        let mut acc = Accumulator221::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.823_f32 + y.sin();
        let b = y * 4.422_f32 - x.cos();
        let mut acc = Accumulator221::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.376_f32 + y.sin();
        let b = y * 7.04_f32 - x.cos();
        let mut acc = Accumulator221::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.878_f32 + y.sin();
        let b = y * 5.858_f32 - x.cos();
        let mut acc = Accumulator221::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.84_f32 + y.sin();
        let b = y * 1.923_f32 - x.cos();
        let mut acc = Accumulator221::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.981_f32 + y.sin();
        let b = y * 4.195_f32 - x.cos();
        let mut acc = Accumulator221::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.757_f32 + y.sin();
        let b = y * 9.82_f32 - x.cos();
        let mut acc = Accumulator221::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.924_f32 + y.sin();
        let b = y * 4.564_f32 - x.cos();
        let mut acc = Accumulator221::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.038_f32 + y.sin();
        let b = y * 9.157_f32 - x.cos();
        let mut acc = Accumulator221::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.818_f32 + y.sin();
        let b = y * 0.219_f32 - x.cos();
        let mut acc = Accumulator221::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.311_f32 + y.sin();
        let b = y * 8.454_f32 - x.cos();
        let mut acc = Accumulator221::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.587_f32 + y.sin();
        let b = y * 4.999_f32 - x.cos();
        let mut acc = Accumulator221::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.518_f32 + y.sin();
        let b = y * 0.493_f32 - x.cos();
        let mut acc = Accumulator221::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_221(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_221() -> f32 {
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
        total += (dep_touch_221(total as u64) % 997) as f32;
        total
    }
}

pub mod m222 {
    use super::*;

    pub struct Accumulator222<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator222<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.999_f32 + y.sin();
        let b = y * 8.711_f32 - x.cos();
        let mut acc = Accumulator222::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.975_f32 + y.sin();
        let b = y * 2.256_f32 - x.cos();
        let mut acc = Accumulator222::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.773_f32 + y.sin();
        let b = y * 0.871_f32 - x.cos();
        let mut acc = Accumulator222::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.709_f32 + y.sin();
        let b = y * 8.831_f32 - x.cos();
        let mut acc = Accumulator222::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.865_f32 + y.sin();
        let b = y * 8.309_f32 - x.cos();
        let mut acc = Accumulator222::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.441_f32 + y.sin();
        let b = y * 6.168_f32 - x.cos();
        let mut acc = Accumulator222::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.507_f32 + y.sin();
        let b = y * 4.871_f32 - x.cos();
        let mut acc = Accumulator222::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.021_f32 + y.sin();
        let b = y * 2.185_f32 - x.cos();
        let mut acc = Accumulator222::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.415_f32 + y.sin();
        let b = y * 7.911_f32 - x.cos();
        let mut acc = Accumulator222::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.734_f32 + y.sin();
        let b = y * 0.564_f32 - x.cos();
        let mut acc = Accumulator222::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.455_f32 + y.sin();
        let b = y * 9.469_f32 - x.cos();
        let mut acc = Accumulator222::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.517_f32 + y.sin();
        let b = y * 5.979_f32 - x.cos();
        let mut acc = Accumulator222::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.784_f32 + y.sin();
        let b = y * 2.862_f32 - x.cos();
        let mut acc = Accumulator222::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.64_f32 + y.sin();
        let b = y * 2.52_f32 - x.cos();
        let mut acc = Accumulator222::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.062_f32 + y.sin();
        let b = y * 3.913_f32 - x.cos();
        let mut acc = Accumulator222::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.711_f32 + y.sin();
        let b = y * 5.162_f32 - x.cos();
        let mut acc = Accumulator222::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.291_f32 + y.sin();
        let b = y * 1.732_f32 - x.cos();
        let mut acc = Accumulator222::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.641_f32 + y.sin();
        let b = y * 0.557_f32 - x.cos();
        let mut acc = Accumulator222::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.259_f32 + y.sin();
        let b = y * 5.836_f32 - x.cos();
        let mut acc = Accumulator222::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.505_f32 + y.sin();
        let b = y * 3.66_f32 - x.cos();
        let mut acc = Accumulator222::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_222(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(222u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_222() -> f32 {
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
        total += (dep_touch_222(total as u64) % 997) as f32;
        total
    }
}

pub mod m223 {
    use super::*;

    pub struct Accumulator223<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator223<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.719_f32 + y.sin();
        let b = y * 8.907_f32 - x.cos();
        let mut acc = Accumulator223::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.834_f32 + y.sin();
        let b = y * 4.116_f32 - x.cos();
        let mut acc = Accumulator223::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.801_f32 + y.sin();
        let b = y * 1.648_f32 - x.cos();
        let mut acc = Accumulator223::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.447_f32 + y.sin();
        let b = y * 9.333_f32 - x.cos();
        let mut acc = Accumulator223::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.849_f32 + y.sin();
        let b = y * 5.082_f32 - x.cos();
        let mut acc = Accumulator223::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.232_f32 + y.sin();
        let b = y * 7.419_f32 - x.cos();
        let mut acc = Accumulator223::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.221_f32 + y.sin();
        let b = y * 3.664_f32 - x.cos();
        let mut acc = Accumulator223::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.241_f32 + y.sin();
        let b = y * 9.898_f32 - x.cos();
        let mut acc = Accumulator223::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.268_f32 + y.sin();
        let b = y * 1.331_f32 - x.cos();
        let mut acc = Accumulator223::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.623_f32 + y.sin();
        let b = y * 2.645_f32 - x.cos();
        let mut acc = Accumulator223::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.157_f32 + y.sin();
        let b = y * 2.681_f32 - x.cos();
        let mut acc = Accumulator223::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.056_f32 + y.sin();
        let b = y * 3.237_f32 - x.cos();
        let mut acc = Accumulator223::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.072_f32 + y.sin();
        let b = y * 4.961_f32 - x.cos();
        let mut acc = Accumulator223::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.523_f32 + y.sin();
        let b = y * 3.368_f32 - x.cos();
        let mut acc = Accumulator223::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.121_f32 + y.sin();
        let b = y * 5.227_f32 - x.cos();
        let mut acc = Accumulator223::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.976_f32 + y.sin();
        let b = y * 2.587_f32 - x.cos();
        let mut acc = Accumulator223::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.326_f32 + y.sin();
        let b = y * 3.663_f32 - x.cos();
        let mut acc = Accumulator223::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.98_f32 + y.sin();
        let b = y * 4.604_f32 - x.cos();
        let mut acc = Accumulator223::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.652_f32 + y.sin();
        let b = y * 2.882_f32 - x.cos();
        let mut acc = Accumulator223::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.478_f32 + y.sin();
        let b = y * 9.628_f32 - x.cos();
        let mut acc = Accumulator223::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_223(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_223() -> f32 {
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
        total += (dep_touch_223(total as u64) % 997) as f32;
        total
    }
}

pub mod m224 {
    use super::*;

    pub struct Accumulator224<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator224<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.829_f32 + y.sin();
        let b = y * 0.444_f32 - x.cos();
        let mut acc = Accumulator224::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.161_f32 + y.sin();
        let b = y * 7.808_f32 - x.cos();
        let mut acc = Accumulator224::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.33_f32 + y.sin();
        let b = y * 2.32_f32 - x.cos();
        let mut acc = Accumulator224::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.277_f32 + y.sin();
        let b = y * 1.901_f32 - x.cos();
        let mut acc = Accumulator224::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.179_f32 + y.sin();
        let b = y * 3.324_f32 - x.cos();
        let mut acc = Accumulator224::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.898_f32 + y.sin();
        let b = y * 4.115_f32 - x.cos();
        let mut acc = Accumulator224::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.006_f32 + y.sin();
        let b = y * 7.677_f32 - x.cos();
        let mut acc = Accumulator224::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.133_f32 + y.sin();
        let b = y * 0.838_f32 - x.cos();
        let mut acc = Accumulator224::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.334_f32 + y.sin();
        let b = y * 1.766_f32 - x.cos();
        let mut acc = Accumulator224::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.67_f32 + y.sin();
        let b = y * 5.128_f32 - x.cos();
        let mut acc = Accumulator224::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.091_f32 + y.sin();
        let b = y * 8.955_f32 - x.cos();
        let mut acc = Accumulator224::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.593_f32 + y.sin();
        let b = y * 0.803_f32 - x.cos();
        let mut acc = Accumulator224::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.793_f32 + y.sin();
        let b = y * 1.925_f32 - x.cos();
        let mut acc = Accumulator224::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.22_f32 + y.sin();
        let b = y * 8.591_f32 - x.cos();
        let mut acc = Accumulator224::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.241_f32 + y.sin();
        let b = y * 5.263_f32 - x.cos();
        let mut acc = Accumulator224::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.83_f32 + y.sin();
        let b = y * 3.493_f32 - x.cos();
        let mut acc = Accumulator224::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.844_f32 + y.sin();
        let b = y * 6.89_f32 - x.cos();
        let mut acc = Accumulator224::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.235_f32 + y.sin();
        let b = y * 4.747_f32 - x.cos();
        let mut acc = Accumulator224::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.654_f32 + y.sin();
        let b = y * 5.708_f32 - x.cos();
        let mut acc = Accumulator224::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.293_f32 + y.sin();
        let b = y * 0.715_f32 - x.cos();
        let mut acc = Accumulator224::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_224(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_224() -> f32 {
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
        total += (dep_touch_224(total as u64) % 997) as f32;
        total
    }
}

pub mod m225 {
    use super::*;

    pub struct Accumulator225<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator225<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.992_f32 + y.sin();
        let b = y * 4.476_f32 - x.cos();
        let mut acc = Accumulator225::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.558_f32 + y.sin();
        let b = y * 2.347_f32 - x.cos();
        let mut acc = Accumulator225::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.785_f32 + y.sin();
        let b = y * 6.431_f32 - x.cos();
        let mut acc = Accumulator225::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.284_f32 + y.sin();
        let b = y * 2.339_f32 - x.cos();
        let mut acc = Accumulator225::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.951_f32 + y.sin();
        let b = y * 2.61_f32 - x.cos();
        let mut acc = Accumulator225::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.671_f32 + y.sin();
        let b = y * 6.617_f32 - x.cos();
        let mut acc = Accumulator225::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.335_f32 + y.sin();
        let b = y * 6.982_f32 - x.cos();
        let mut acc = Accumulator225::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.209_f32 + y.sin();
        let b = y * 1.198_f32 - x.cos();
        let mut acc = Accumulator225::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.205_f32 + y.sin();
        let b = y * 3.265_f32 - x.cos();
        let mut acc = Accumulator225::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.945_f32 + y.sin();
        let b = y * 1.589_f32 - x.cos();
        let mut acc = Accumulator225::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.639_f32 + y.sin();
        let b = y * 1.595_f32 - x.cos();
        let mut acc = Accumulator225::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.074_f32 + y.sin();
        let b = y * 3.194_f32 - x.cos();
        let mut acc = Accumulator225::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.189_f32 + y.sin();
        let b = y * 6.283_f32 - x.cos();
        let mut acc = Accumulator225::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.894_f32 + y.sin();
        let b = y * 4.072_f32 - x.cos();
        let mut acc = Accumulator225::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.48_f32 + y.sin();
        let b = y * 6.465_f32 - x.cos();
        let mut acc = Accumulator225::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.258_f32 + y.sin();
        let b = y * 7.431_f32 - x.cos();
        let mut acc = Accumulator225::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.658_f32 + y.sin();
        let b = y * 5.44_f32 - x.cos();
        let mut acc = Accumulator225::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.006_f32 + y.sin();
        let b = y * 6.853_f32 - x.cos();
        let mut acc = Accumulator225::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.258_f32 + y.sin();
        let b = y * 9.822_f32 - x.cos();
        let mut acc = Accumulator225::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.28_f32 + y.sin();
        let b = y * 9.602_f32 - x.cos();
        let mut acc = Accumulator225::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_225(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m225-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_225() -> f32 {
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
        total += (dep_touch_225(total as u64) % 997) as f32;
        total
    }
}

pub mod m226 {
    use super::*;

    pub struct Accumulator226<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator226<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.391_f32 + y.sin();
        let b = y * 9.472_f32 - x.cos();
        let mut acc = Accumulator226::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.979_f32 + y.sin();
        let b = y * 5.756_f32 - x.cos();
        let mut acc = Accumulator226::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.978_f32 + y.sin();
        let b = y * 5.962_f32 - x.cos();
        let mut acc = Accumulator226::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.659_f32 + y.sin();
        let b = y * 1.135_f32 - x.cos();
        let mut acc = Accumulator226::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.937_f32 + y.sin();
        let b = y * 5.272_f32 - x.cos();
        let mut acc = Accumulator226::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.311_f32 + y.sin();
        let b = y * 8.647_f32 - x.cos();
        let mut acc = Accumulator226::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.134_f32 + y.sin();
        let b = y * 6.755_f32 - x.cos();
        let mut acc = Accumulator226::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.651_f32 + y.sin();
        let b = y * 9.294_f32 - x.cos();
        let mut acc = Accumulator226::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.405_f32 + y.sin();
        let b = y * 0.378_f32 - x.cos();
        let mut acc = Accumulator226::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.348_f32 + y.sin();
        let b = y * 8.345_f32 - x.cos();
        let mut acc = Accumulator226::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.996_f32 + y.sin();
        let b = y * 8.92_f32 - x.cos();
        let mut acc = Accumulator226::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.792_f32 + y.sin();
        let b = y * 6.179_f32 - x.cos();
        let mut acc = Accumulator226::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.871_f32 + y.sin();
        let b = y * 8.823_f32 - x.cos();
        let mut acc = Accumulator226::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.976_f32 + y.sin();
        let b = y * 2.853_f32 - x.cos();
        let mut acc = Accumulator226::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.012_f32 + y.sin();
        let b = y * 4.138_f32 - x.cos();
        let mut acc = Accumulator226::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.547_f32 + y.sin();
        let b = y * 4.449_f32 - x.cos();
        let mut acc = Accumulator226::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.878_f32 + y.sin();
        let b = y * 3.059_f32 - x.cos();
        let mut acc = Accumulator226::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.312_f32 + y.sin();
        let b = y * 6.273_f32 - x.cos();
        let mut acc = Accumulator226::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.411_f32 + y.sin();
        let b = y * 8.744_f32 - x.cos();
        let mut acc = Accumulator226::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.36_f32 + y.sin();
        let b = y * 9.888_f32 - x.cos();
        let mut acc = Accumulator226::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_226(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_226() -> f32 {
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
        total += (dep_touch_226(total as u64) % 997) as f32;
        total
    }
}

pub mod m227 {
    use super::*;

    pub struct Accumulator227<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator227<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.866_f32 + y.sin();
        let b = y * 6.78_f32 - x.cos();
        let mut acc = Accumulator227::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.037_f32 + y.sin();
        let b = y * 6.365_f32 - x.cos();
        let mut acc = Accumulator227::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.462_f32 + y.sin();
        let b = y * 1.456_f32 - x.cos();
        let mut acc = Accumulator227::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.304_f32 + y.sin();
        let b = y * 9.699_f32 - x.cos();
        let mut acc = Accumulator227::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.833_f32 + y.sin();
        let b = y * 3.296_f32 - x.cos();
        let mut acc = Accumulator227::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.204_f32 + y.sin();
        let b = y * 5.44_f32 - x.cos();
        let mut acc = Accumulator227::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.171_f32 + y.sin();
        let b = y * 0.536_f32 - x.cos();
        let mut acc = Accumulator227::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.968_f32 + y.sin();
        let b = y * 6.391_f32 - x.cos();
        let mut acc = Accumulator227::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.197_f32 + y.sin();
        let b = y * 2.801_f32 - x.cos();
        let mut acc = Accumulator227::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.888_f32 + y.sin();
        let b = y * 5.575_f32 - x.cos();
        let mut acc = Accumulator227::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.224_f32 + y.sin();
        let b = y * 7.444_f32 - x.cos();
        let mut acc = Accumulator227::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.215_f32 + y.sin();
        let b = y * 2.243_f32 - x.cos();
        let mut acc = Accumulator227::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.265_f32 + y.sin();
        let b = y * 7.101_f32 - x.cos();
        let mut acc = Accumulator227::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.505_f32 + y.sin();
        let b = y * 9.855_f32 - x.cos();
        let mut acc = Accumulator227::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.196_f32 + y.sin();
        let b = y * 5.11_f32 - x.cos();
        let mut acc = Accumulator227::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.426_f32 + y.sin();
        let b = y * 3.831_f32 - x.cos();
        let mut acc = Accumulator227::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.431_f32 + y.sin();
        let b = y * 3.03_f32 - x.cos();
        let mut acc = Accumulator227::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.84_f32 + y.sin();
        let b = y * 5.236_f32 - x.cos();
        let mut acc = Accumulator227::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.222_f32 + y.sin();
        let b = y * 7.835_f32 - x.cos();
        let mut acc = Accumulator227::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.515_f32 + y.sin();
        let b = y * 3.536_f32 - x.cos();
        let mut acc = Accumulator227::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_227(seed: u64) -> u64 {
        let re = Regex::new(r"m227-(\d+)").unwrap();
        let hay = format!("m227-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_227() -> f32 {
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
        total += (dep_touch_227(total as u64) % 997) as f32;
        total
    }
}

pub mod m228 {
    use super::*;

    pub struct Accumulator228<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator228<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.902_f32 + y.sin();
        let b = y * 9.006_f32 - x.cos();
        let mut acc = Accumulator228::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.091_f32 + y.sin();
        let b = y * 5.657_f32 - x.cos();
        let mut acc = Accumulator228::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.783_f32 + y.sin();
        let b = y * 4.473_f32 - x.cos();
        let mut acc = Accumulator228::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.988_f32 + y.sin();
        let b = y * 2.496_f32 - x.cos();
        let mut acc = Accumulator228::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.213_f32 + y.sin();
        let b = y * 4.408_f32 - x.cos();
        let mut acc = Accumulator228::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.027_f32 + y.sin();
        let b = y * 5.956_f32 - x.cos();
        let mut acc = Accumulator228::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.071_f32 + y.sin();
        let b = y * 1.221_f32 - x.cos();
        let mut acc = Accumulator228::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.802_f32 + y.sin();
        let b = y * 7.334_f32 - x.cos();
        let mut acc = Accumulator228::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.015_f32 + y.sin();
        let b = y * 5.336_f32 - x.cos();
        let mut acc = Accumulator228::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.123_f32 + y.sin();
        let b = y * 2.943_f32 - x.cos();
        let mut acc = Accumulator228::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.278_f32 + y.sin();
        let b = y * 8.218_f32 - x.cos();
        let mut acc = Accumulator228::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.657_f32 + y.sin();
        let b = y * 9.254_f32 - x.cos();
        let mut acc = Accumulator228::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.144_f32 + y.sin();
        let b = y * 5.203_f32 - x.cos();
        let mut acc = Accumulator228::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.763_f32 + y.sin();
        let b = y * 1.285_f32 - x.cos();
        let mut acc = Accumulator228::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.753_f32 + y.sin();
        let b = y * 5.54_f32 - x.cos();
        let mut acc = Accumulator228::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.013_f32 + y.sin();
        let b = y * 9.223_f32 - x.cos();
        let mut acc = Accumulator228::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.683_f32 + y.sin();
        let b = y * 1.376_f32 - x.cos();
        let mut acc = Accumulator228::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.47_f32 + y.sin();
        let b = y * 4.421_f32 - x.cos();
        let mut acc = Accumulator228::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.839_f32 + y.sin();
        let b = y * 9.472_f32 - x.cos();
        let mut acc = Accumulator228::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.705_f32 + y.sin();
        let b = y * 2.01_f32 - x.cos();
        let mut acc = Accumulator228::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_228(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_228() -> f32 {
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
        total += (dep_touch_228(total as u64) % 997) as f32;
        total
    }
}

pub mod m229 {
    use super::*;

    pub struct Accumulator229<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator229<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.707_f32 + y.sin();
        let b = y * 3.767_f32 - x.cos();
        let mut acc = Accumulator229::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.6_f32 + y.sin();
        let b = y * 4.021_f32 - x.cos();
        let mut acc = Accumulator229::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.574_f32 + y.sin();
        let b = y * 6.264_f32 - x.cos();
        let mut acc = Accumulator229::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.27_f32 + y.sin();
        let b = y * 9.182_f32 - x.cos();
        let mut acc = Accumulator229::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.256_f32 + y.sin();
        let b = y * 8.041_f32 - x.cos();
        let mut acc = Accumulator229::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.885_f32 + y.sin();
        let b = y * 1.37_f32 - x.cos();
        let mut acc = Accumulator229::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.898_f32 + y.sin();
        let b = y * 4.544_f32 - x.cos();
        let mut acc = Accumulator229::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.642_f32 + y.sin();
        let b = y * 5.085_f32 - x.cos();
        let mut acc = Accumulator229::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.479_f32 + y.sin();
        let b = y * 4.147_f32 - x.cos();
        let mut acc = Accumulator229::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.317_f32 + y.sin();
        let b = y * 2.329_f32 - x.cos();
        let mut acc = Accumulator229::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.285_f32 + y.sin();
        let b = y * 2.972_f32 - x.cos();
        let mut acc = Accumulator229::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.382_f32 + y.sin();
        let b = y * 3.69_f32 - x.cos();
        let mut acc = Accumulator229::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.422_f32 + y.sin();
        let b = y * 9.669_f32 - x.cos();
        let mut acc = Accumulator229::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.581_f32 + y.sin();
        let b = y * 2.018_f32 - x.cos();
        let mut acc = Accumulator229::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.661_f32 + y.sin();
        let b = y * 8.821_f32 - x.cos();
        let mut acc = Accumulator229::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.707_f32 + y.sin();
        let b = y * 7.099_f32 - x.cos();
        let mut acc = Accumulator229::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.752_f32 + y.sin();
        let b = y * 4.416_f32 - x.cos();
        let mut acc = Accumulator229::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.646_f32 + y.sin();
        let b = y * 7.73_f32 - x.cos();
        let mut acc = Accumulator229::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.439_f32 + y.sin();
        let b = y * 8.735_f32 - x.cos();
        let mut acc = Accumulator229::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.435_f32 + y.sin();
        let b = y * 9.763_f32 - x.cos();
        let mut acc = Accumulator229::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_229(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(229u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_229() -> f32 {
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
        total += (dep_touch_229(total as u64) % 997) as f32;
        total
    }
}

pub mod m230 {
    use super::*;

    pub struct Accumulator230<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator230<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.005_f32 + y.sin();
        let b = y * 7.098_f32 - x.cos();
        let mut acc = Accumulator230::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.736_f32 + y.sin();
        let b = y * 3.631_f32 - x.cos();
        let mut acc = Accumulator230::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.65_f32 + y.sin();
        let b = y * 6.794_f32 - x.cos();
        let mut acc = Accumulator230::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.031_f32 + y.sin();
        let b = y * 3.208_f32 - x.cos();
        let mut acc = Accumulator230::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.167_f32 + y.sin();
        let b = y * 5.02_f32 - x.cos();
        let mut acc = Accumulator230::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.913_f32 + y.sin();
        let b = y * 3.294_f32 - x.cos();
        let mut acc = Accumulator230::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.132_f32 + y.sin();
        let b = y * 7.383_f32 - x.cos();
        let mut acc = Accumulator230::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.027_f32 + y.sin();
        let b = y * 2.07_f32 - x.cos();
        let mut acc = Accumulator230::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.025_f32 + y.sin();
        let b = y * 1.574_f32 - x.cos();
        let mut acc = Accumulator230::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.622_f32 + y.sin();
        let b = y * 7.069_f32 - x.cos();
        let mut acc = Accumulator230::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.613_f32 + y.sin();
        let b = y * 3.772_f32 - x.cos();
        let mut acc = Accumulator230::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.277_f32 + y.sin();
        let b = y * 1.254_f32 - x.cos();
        let mut acc = Accumulator230::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.959_f32 + y.sin();
        let b = y * 8.626_f32 - x.cos();
        let mut acc = Accumulator230::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.594_f32 + y.sin();
        let b = y * 5.118_f32 - x.cos();
        let mut acc = Accumulator230::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.499_f32 + y.sin();
        let b = y * 6.292_f32 - x.cos();
        let mut acc = Accumulator230::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.074_f32 + y.sin();
        let b = y * 4.625_f32 - x.cos();
        let mut acc = Accumulator230::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.877_f32 + y.sin();
        let b = y * 3.142_f32 - x.cos();
        let mut acc = Accumulator230::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.723_f32 + y.sin();
        let b = y * 0.245_f32 - x.cos();
        let mut acc = Accumulator230::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.886_f32 + y.sin();
        let b = y * 7.3_f32 - x.cos();
        let mut acc = Accumulator230::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.665_f32 + y.sin();
        let b = y * 5.824_f32 - x.cos();
        let mut acc = Accumulator230::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_230(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_230() -> f32 {
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
        total += (dep_touch_230(total as u64) % 997) as f32;
        total
    }
}

pub mod m231 {
    use super::*;

    pub struct Accumulator231<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator231<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.692_f32 + y.sin();
        let b = y * 9.596_f32 - x.cos();
        let mut acc = Accumulator231::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.364_f32 + y.sin();
        let b = y * 9.657_f32 - x.cos();
        let mut acc = Accumulator231::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.565_f32 + y.sin();
        let b = y * 0.614_f32 - x.cos();
        let mut acc = Accumulator231::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.5_f32 + y.sin();
        let b = y * 6.573_f32 - x.cos();
        let mut acc = Accumulator231::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.34_f32 + y.sin();
        let b = y * 8.373_f32 - x.cos();
        let mut acc = Accumulator231::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.685_f32 + y.sin();
        let b = y * 3.316_f32 - x.cos();
        let mut acc = Accumulator231::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.69_f32 + y.sin();
        let b = y * 5.549_f32 - x.cos();
        let mut acc = Accumulator231::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.311_f32 + y.sin();
        let b = y * 3.804_f32 - x.cos();
        let mut acc = Accumulator231::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.289_f32 + y.sin();
        let b = y * 2.383_f32 - x.cos();
        let mut acc = Accumulator231::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.606_f32 + y.sin();
        let b = y * 3.641_f32 - x.cos();
        let mut acc = Accumulator231::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.401_f32 + y.sin();
        let b = y * 5.352_f32 - x.cos();
        let mut acc = Accumulator231::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.344_f32 + y.sin();
        let b = y * 0.826_f32 - x.cos();
        let mut acc = Accumulator231::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.541_f32 + y.sin();
        let b = y * 2.6_f32 - x.cos();
        let mut acc = Accumulator231::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.312_f32 + y.sin();
        let b = y * 8.513_f32 - x.cos();
        let mut acc = Accumulator231::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.172_f32 + y.sin();
        let b = y * 3.691_f32 - x.cos();
        let mut acc = Accumulator231::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.254_f32 + y.sin();
        let b = y * 0.956_f32 - x.cos();
        let mut acc = Accumulator231::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.067_f32 + y.sin();
        let b = y * 7.733_f32 - x.cos();
        let mut acc = Accumulator231::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.107_f32 + y.sin();
        let b = y * 4.952_f32 - x.cos();
        let mut acc = Accumulator231::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.333_f32 + y.sin();
        let b = y * 8.624_f32 - x.cos();
        let mut acc = Accumulator231::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.61_f32 + y.sin();
        let b = y * 7.69_f32 - x.cos();
        let mut acc = Accumulator231::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_231(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_231() -> f32 {
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
        total += (dep_touch_231(total as u64) % 997) as f32;
        total
    }
}

pub mod m232 {
    use super::*;

    pub struct Accumulator232<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator232<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.194_f32 + y.sin();
        let b = y * 4.432_f32 - x.cos();
        let mut acc = Accumulator232::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.916_f32 + y.sin();
        let b = y * 5.698_f32 - x.cos();
        let mut acc = Accumulator232::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.558_f32 + y.sin();
        let b = y * 7.64_f32 - x.cos();
        let mut acc = Accumulator232::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.468_f32 + y.sin();
        let b = y * 2.204_f32 - x.cos();
        let mut acc = Accumulator232::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.795_f32 + y.sin();
        let b = y * 8.616_f32 - x.cos();
        let mut acc = Accumulator232::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.129_f32 + y.sin();
        let b = y * 1.6_f32 - x.cos();
        let mut acc = Accumulator232::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.001_f32 + y.sin();
        let b = y * 5.514_f32 - x.cos();
        let mut acc = Accumulator232::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.467_f32 + y.sin();
        let b = y * 1.681_f32 - x.cos();
        let mut acc = Accumulator232::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.671_f32 + y.sin();
        let b = y * 9.202_f32 - x.cos();
        let mut acc = Accumulator232::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.986_f32 + y.sin();
        let b = y * 4.992_f32 - x.cos();
        let mut acc = Accumulator232::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.583_f32 + y.sin();
        let b = y * 4.956_f32 - x.cos();
        let mut acc = Accumulator232::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.835_f32 + y.sin();
        let b = y * 0.73_f32 - x.cos();
        let mut acc = Accumulator232::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.356_f32 + y.sin();
        let b = y * 6.536_f32 - x.cos();
        let mut acc = Accumulator232::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.079_f32 + y.sin();
        let b = y * 6.9_f32 - x.cos();
        let mut acc = Accumulator232::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.615_f32 + y.sin();
        let b = y * 2.507_f32 - x.cos();
        let mut acc = Accumulator232::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.246_f32 + y.sin();
        let b = y * 1.852_f32 - x.cos();
        let mut acc = Accumulator232::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.516_f32 + y.sin();
        let b = y * 5.488_f32 - x.cos();
        let mut acc = Accumulator232::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.617_f32 + y.sin();
        let b = y * 5.41_f32 - x.cos();
        let mut acc = Accumulator232::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.783_f32 + y.sin();
        let b = y * 1.822_f32 - x.cos();
        let mut acc = Accumulator232::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.516_f32 + y.sin();
        let b = y * 6.683_f32 - x.cos();
        let mut acc = Accumulator232::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_232(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m232-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_232() -> f32 {
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
        total += (dep_touch_232(total as u64) % 997) as f32;
        total
    }
}

pub mod m233 {
    use super::*;

    pub struct Accumulator233<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator233<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.049_f32 + y.sin();
        let b = y * 8.609_f32 - x.cos();
        let mut acc = Accumulator233::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.246_f32 + y.sin();
        let b = y * 8.63_f32 - x.cos();
        let mut acc = Accumulator233::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.892_f32 + y.sin();
        let b = y * 4.331_f32 - x.cos();
        let mut acc = Accumulator233::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.284_f32 + y.sin();
        let b = y * 7.073_f32 - x.cos();
        let mut acc = Accumulator233::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.593_f32 + y.sin();
        let b = y * 7.113_f32 - x.cos();
        let mut acc = Accumulator233::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.311_f32 + y.sin();
        let b = y * 4.071_f32 - x.cos();
        let mut acc = Accumulator233::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.29_f32 + y.sin();
        let b = y * 0.29_f32 - x.cos();
        let mut acc = Accumulator233::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.485_f32 + y.sin();
        let b = y * 0.875_f32 - x.cos();
        let mut acc = Accumulator233::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.757_f32 + y.sin();
        let b = y * 2.541_f32 - x.cos();
        let mut acc = Accumulator233::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.162_f32 + y.sin();
        let b = y * 6.849_f32 - x.cos();
        let mut acc = Accumulator233::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.558_f32 + y.sin();
        let b = y * 4.791_f32 - x.cos();
        let mut acc = Accumulator233::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.431_f32 + y.sin();
        let b = y * 1.639_f32 - x.cos();
        let mut acc = Accumulator233::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.545_f32 + y.sin();
        let b = y * 9.196_f32 - x.cos();
        let mut acc = Accumulator233::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.702_f32 + y.sin();
        let b = y * 8.091_f32 - x.cos();
        let mut acc = Accumulator233::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.344_f32 + y.sin();
        let b = y * 8.585_f32 - x.cos();
        let mut acc = Accumulator233::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.835_f32 + y.sin();
        let b = y * 5.684_f32 - x.cos();
        let mut acc = Accumulator233::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.555_f32 + y.sin();
        let b = y * 9.873_f32 - x.cos();
        let mut acc = Accumulator233::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.235_f32 + y.sin();
        let b = y * 7.096_f32 - x.cos();
        let mut acc = Accumulator233::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.711_f32 + y.sin();
        let b = y * 5.59_f32 - x.cos();
        let mut acc = Accumulator233::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.762_f32 + y.sin();
        let b = y * 7.438_f32 - x.cos();
        let mut acc = Accumulator233::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_233(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_233() -> f32 {
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
        total += (dep_touch_233(total as u64) % 997) as f32;
        total
    }
}

pub mod m234 {
    use super::*;

    pub struct Accumulator234<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator234<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.72_f32 + y.sin();
        let b = y * 1.104_f32 - x.cos();
        let mut acc = Accumulator234::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.105_f32 + y.sin();
        let b = y * 1.057_f32 - x.cos();
        let mut acc = Accumulator234::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.972_f32 + y.sin();
        let b = y * 3.608_f32 - x.cos();
        let mut acc = Accumulator234::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.59_f32 + y.sin();
        let b = y * 7.621_f32 - x.cos();
        let mut acc = Accumulator234::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.784_f32 + y.sin();
        let b = y * 2.536_f32 - x.cos();
        let mut acc = Accumulator234::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.846_f32 + y.sin();
        let b = y * 1.84_f32 - x.cos();
        let mut acc = Accumulator234::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.067_f32 + y.sin();
        let b = y * 1.533_f32 - x.cos();
        let mut acc = Accumulator234::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.387_f32 + y.sin();
        let b = y * 3.27_f32 - x.cos();
        let mut acc = Accumulator234::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.579_f32 + y.sin();
        let b = y * 4.16_f32 - x.cos();
        let mut acc = Accumulator234::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.75_f32 + y.sin();
        let b = y * 9.488_f32 - x.cos();
        let mut acc = Accumulator234::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.062_f32 + y.sin();
        let b = y * 7.721_f32 - x.cos();
        let mut acc = Accumulator234::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.14_f32 + y.sin();
        let b = y * 8.624_f32 - x.cos();
        let mut acc = Accumulator234::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.014_f32 + y.sin();
        let b = y * 6.152_f32 - x.cos();
        let mut acc = Accumulator234::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.277_f32 + y.sin();
        let b = y * 2.323_f32 - x.cos();
        let mut acc = Accumulator234::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.58_f32 + y.sin();
        let b = y * 4.904_f32 - x.cos();
        let mut acc = Accumulator234::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.27_f32 + y.sin();
        let b = y * 7.085_f32 - x.cos();
        let mut acc = Accumulator234::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.708_f32 + y.sin();
        let b = y * 8.741_f32 - x.cos();
        let mut acc = Accumulator234::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.597_f32 + y.sin();
        let b = y * 6.446_f32 - x.cos();
        let mut acc = Accumulator234::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.363_f32 + y.sin();
        let b = y * 9.848_f32 - x.cos();
        let mut acc = Accumulator234::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.543_f32 + y.sin();
        let b = y * 5.457_f32 - x.cos();
        let mut acc = Accumulator234::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_234(seed: u64) -> u64 {
        let re = Regex::new(r"m234-(\d+)").unwrap();
        let hay = format!("m234-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_234() -> f32 {
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
        total += (dep_touch_234(total as u64) % 997) as f32;
        total
    }
}

pub mod m235 {
    use super::*;

    pub struct Accumulator235<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator235<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.514_f32 + y.sin();
        let b = y * 2.982_f32 - x.cos();
        let mut acc = Accumulator235::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.747_f32 + y.sin();
        let b = y * 4.003_f32 - x.cos();
        let mut acc = Accumulator235::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.347_f32 + y.sin();
        let b = y * 5.038_f32 - x.cos();
        let mut acc = Accumulator235::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.426_f32 + y.sin();
        let b = y * 6.294_f32 - x.cos();
        let mut acc = Accumulator235::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.271_f32 + y.sin();
        let b = y * 1.176_f32 - x.cos();
        let mut acc = Accumulator235::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.641_f32 + y.sin();
        let b = y * 8.798_f32 - x.cos();
        let mut acc = Accumulator235::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.849_f32 + y.sin();
        let b = y * 7.361_f32 - x.cos();
        let mut acc = Accumulator235::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.848_f32 + y.sin();
        let b = y * 1.573_f32 - x.cos();
        let mut acc = Accumulator235::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.346_f32 + y.sin();
        let b = y * 9.574_f32 - x.cos();
        let mut acc = Accumulator235::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.711_f32 + y.sin();
        let b = y * 0.888_f32 - x.cos();
        let mut acc = Accumulator235::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.247_f32 + y.sin();
        let b = y * 8.029_f32 - x.cos();
        let mut acc = Accumulator235::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.175_f32 + y.sin();
        let b = y * 0.903_f32 - x.cos();
        let mut acc = Accumulator235::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.956_f32 + y.sin();
        let b = y * 9.876_f32 - x.cos();
        let mut acc = Accumulator235::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.123_f32 + y.sin();
        let b = y * 6.696_f32 - x.cos();
        let mut acc = Accumulator235::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.004_f32 + y.sin();
        let b = y * 4.032_f32 - x.cos();
        let mut acc = Accumulator235::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.64_f32 + y.sin();
        let b = y * 4.886_f32 - x.cos();
        let mut acc = Accumulator235::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.515_f32 + y.sin();
        let b = y * 4.873_f32 - x.cos();
        let mut acc = Accumulator235::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.994_f32 + y.sin();
        let b = y * 3.808_f32 - x.cos();
        let mut acc = Accumulator235::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.07_f32 + y.sin();
        let b = y * 0.448_f32 - x.cos();
        let mut acc = Accumulator235::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.18_f32 + y.sin();
        let b = y * 4.323_f32 - x.cos();
        let mut acc = Accumulator235::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_235(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_235() -> f32 {
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
        total += (dep_touch_235(total as u64) % 997) as f32;
        total
    }
}

pub mod m236 {
    use super::*;

    pub struct Accumulator236<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator236<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.322_f32 + y.sin();
        let b = y * 3.063_f32 - x.cos();
        let mut acc = Accumulator236::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.322_f32 + y.sin();
        let b = y * 6.327_f32 - x.cos();
        let mut acc = Accumulator236::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.545_f32 + y.sin();
        let b = y * 3.086_f32 - x.cos();
        let mut acc = Accumulator236::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.615_f32 + y.sin();
        let b = y * 5.813_f32 - x.cos();
        let mut acc = Accumulator236::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.913_f32 + y.sin();
        let b = y * 5.451_f32 - x.cos();
        let mut acc = Accumulator236::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.402_f32 + y.sin();
        let b = y * 9.418_f32 - x.cos();
        let mut acc = Accumulator236::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.164_f32 + y.sin();
        let b = y * 5.881_f32 - x.cos();
        let mut acc = Accumulator236::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.142_f32 + y.sin();
        let b = y * 9.785_f32 - x.cos();
        let mut acc = Accumulator236::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.299_f32 + y.sin();
        let b = y * 5.855_f32 - x.cos();
        let mut acc = Accumulator236::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.275_f32 + y.sin();
        let b = y * 1.191_f32 - x.cos();
        let mut acc = Accumulator236::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.378_f32 + y.sin();
        let b = y * 6.118_f32 - x.cos();
        let mut acc = Accumulator236::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.957_f32 + y.sin();
        let b = y * 1.691_f32 - x.cos();
        let mut acc = Accumulator236::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.725_f32 + y.sin();
        let b = y * 7.939_f32 - x.cos();
        let mut acc = Accumulator236::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.899_f32 + y.sin();
        let b = y * 0.918_f32 - x.cos();
        let mut acc = Accumulator236::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.054_f32 + y.sin();
        let b = y * 7.64_f32 - x.cos();
        let mut acc = Accumulator236::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.163_f32 + y.sin();
        let b = y * 7.064_f32 - x.cos();
        let mut acc = Accumulator236::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.965_f32 + y.sin();
        let b = y * 4.651_f32 - x.cos();
        let mut acc = Accumulator236::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.254_f32 + y.sin();
        let b = y * 3.219_f32 - x.cos();
        let mut acc = Accumulator236::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.744_f32 + y.sin();
        let b = y * 8.464_f32 - x.cos();
        let mut acc = Accumulator236::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.475_f32 + y.sin();
        let b = y * 7.684_f32 - x.cos();
        let mut acc = Accumulator236::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_236(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(236u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_236() -> f32 {
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
        total += (dep_touch_236(total as u64) % 997) as f32;
        total
    }
}

pub mod m237 {
    use super::*;

    pub struct Accumulator237<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator237<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.955_f32 + y.sin();
        let b = y * 4.331_f32 - x.cos();
        let mut acc = Accumulator237::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.061_f32 + y.sin();
        let b = y * 3.698_f32 - x.cos();
        let mut acc = Accumulator237::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.919_f32 + y.sin();
        let b = y * 9.591_f32 - x.cos();
        let mut acc = Accumulator237::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.251_f32 + y.sin();
        let b = y * 6.994_f32 - x.cos();
        let mut acc = Accumulator237::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.531_f32 + y.sin();
        let b = y * 0.827_f32 - x.cos();
        let mut acc = Accumulator237::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.54_f32 + y.sin();
        let b = y * 0.62_f32 - x.cos();
        let mut acc = Accumulator237::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.746_f32 + y.sin();
        let b = y * 9.572_f32 - x.cos();
        let mut acc = Accumulator237::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.235_f32 + y.sin();
        let b = y * 1.156_f32 - x.cos();
        let mut acc = Accumulator237::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.674_f32 + y.sin();
        let b = y * 0.263_f32 - x.cos();
        let mut acc = Accumulator237::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.809_f32 + y.sin();
        let b = y * 6.588_f32 - x.cos();
        let mut acc = Accumulator237::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.227_f32 + y.sin();
        let b = y * 1.573_f32 - x.cos();
        let mut acc = Accumulator237::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.201_f32 + y.sin();
        let b = y * 0.611_f32 - x.cos();
        let mut acc = Accumulator237::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.81_f32 + y.sin();
        let b = y * 6.053_f32 - x.cos();
        let mut acc = Accumulator237::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.297_f32 + y.sin();
        let b = y * 1.507_f32 - x.cos();
        let mut acc = Accumulator237::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.644_f32 + y.sin();
        let b = y * 7.569_f32 - x.cos();
        let mut acc = Accumulator237::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.792_f32 + y.sin();
        let b = y * 2.712_f32 - x.cos();
        let mut acc = Accumulator237::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.499_f32 + y.sin();
        let b = y * 0.296_f32 - x.cos();
        let mut acc = Accumulator237::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.818_f32 + y.sin();
        let b = y * 7.76_f32 - x.cos();
        let mut acc = Accumulator237::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.799_f32 + y.sin();
        let b = y * 5.826_f32 - x.cos();
        let mut acc = Accumulator237::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.009_f32 + y.sin();
        let b = y * 8.939_f32 - x.cos();
        let mut acc = Accumulator237::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_237(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_237() -> f32 {
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
        total += (dep_touch_237(total as u64) % 997) as f32;
        total
    }
}

pub mod m238 {
    use super::*;

    pub struct Accumulator238<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator238<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.454_f32 + y.sin();
        let b = y * 5.43_f32 - x.cos();
        let mut acc = Accumulator238::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.738_f32 + y.sin();
        let b = y * 2.447_f32 - x.cos();
        let mut acc = Accumulator238::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.795_f32 + y.sin();
        let b = y * 4.939_f32 - x.cos();
        let mut acc = Accumulator238::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.715_f32 + y.sin();
        let b = y * 9.828_f32 - x.cos();
        let mut acc = Accumulator238::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.747_f32 + y.sin();
        let b = y * 9.737_f32 - x.cos();
        let mut acc = Accumulator238::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.248_f32 + y.sin();
        let b = y * 9.027_f32 - x.cos();
        let mut acc = Accumulator238::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.793_f32 + y.sin();
        let b = y * 0.786_f32 - x.cos();
        let mut acc = Accumulator238::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.626_f32 + y.sin();
        let b = y * 5.895_f32 - x.cos();
        let mut acc = Accumulator238::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.139_f32 + y.sin();
        let b = y * 0.7_f32 - x.cos();
        let mut acc = Accumulator238::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.125_f32 + y.sin();
        let b = y * 1.93_f32 - x.cos();
        let mut acc = Accumulator238::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.963_f32 + y.sin();
        let b = y * 8.819_f32 - x.cos();
        let mut acc = Accumulator238::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.072_f32 + y.sin();
        let b = y * 9.052_f32 - x.cos();
        let mut acc = Accumulator238::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.573_f32 + y.sin();
        let b = y * 1.151_f32 - x.cos();
        let mut acc = Accumulator238::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.118_f32 + y.sin();
        let b = y * 4.066_f32 - x.cos();
        let mut acc = Accumulator238::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.525_f32 + y.sin();
        let b = y * 9.281_f32 - x.cos();
        let mut acc = Accumulator238::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.347_f32 + y.sin();
        let b = y * 6.978_f32 - x.cos();
        let mut acc = Accumulator238::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.72_f32 + y.sin();
        let b = y * 5.048_f32 - x.cos();
        let mut acc = Accumulator238::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.391_f32 + y.sin();
        let b = y * 0.695_f32 - x.cos();
        let mut acc = Accumulator238::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.168_f32 + y.sin();
        let b = y * 2.072_f32 - x.cos();
        let mut acc = Accumulator238::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.948_f32 + y.sin();
        let b = y * 3.864_f32 - x.cos();
        let mut acc = Accumulator238::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_238(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_238() -> f32 {
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
        total += (dep_touch_238(total as u64) % 997) as f32;
        total
    }
}

pub mod m239 {
    use super::*;

    pub struct Accumulator239<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator239<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.172_f32 + y.sin();
        let b = y * 1.922_f32 - x.cos();
        let mut acc = Accumulator239::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.082_f32 + y.sin();
        let b = y * 0.797_f32 - x.cos();
        let mut acc = Accumulator239::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.339_f32 + y.sin();
        let b = y * 3.205_f32 - x.cos();
        let mut acc = Accumulator239::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.858_f32 + y.sin();
        let b = y * 6.149_f32 - x.cos();
        let mut acc = Accumulator239::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.292_f32 + y.sin();
        let b = y * 7.696_f32 - x.cos();
        let mut acc = Accumulator239::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.508_f32 + y.sin();
        let b = y * 1.466_f32 - x.cos();
        let mut acc = Accumulator239::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.192_f32 + y.sin();
        let b = y * 6.411_f32 - x.cos();
        let mut acc = Accumulator239::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.868_f32 + y.sin();
        let b = y * 6.245_f32 - x.cos();
        let mut acc = Accumulator239::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.892_f32 + y.sin();
        let b = y * 3.388_f32 - x.cos();
        let mut acc = Accumulator239::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.711_f32 + y.sin();
        let b = y * 1.697_f32 - x.cos();
        let mut acc = Accumulator239::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.139_f32 + y.sin();
        let b = y * 9.644_f32 - x.cos();
        let mut acc = Accumulator239::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.339_f32 + y.sin();
        let b = y * 4.682_f32 - x.cos();
        let mut acc = Accumulator239::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.663_f32 + y.sin();
        let b = y * 4.179_f32 - x.cos();
        let mut acc = Accumulator239::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.437_f32 + y.sin();
        let b = y * 2.316_f32 - x.cos();
        let mut acc = Accumulator239::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.729_f32 + y.sin();
        let b = y * 7.181_f32 - x.cos();
        let mut acc = Accumulator239::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.241_f32 + y.sin();
        let b = y * 4.013_f32 - x.cos();
        let mut acc = Accumulator239::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.599_f32 + y.sin();
        let b = y * 1.365_f32 - x.cos();
        let mut acc = Accumulator239::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.409_f32 + y.sin();
        let b = y * 4.664_f32 - x.cos();
        let mut acc = Accumulator239::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.862_f32 + y.sin();
        let b = y * 4.655_f32 - x.cos();
        let mut acc = Accumulator239::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.084_f32 + y.sin();
        let b = y * 0.769_f32 - x.cos();
        let mut acc = Accumulator239::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_239(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m239-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_239() -> f32 {
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
        total += (dep_touch_239(total as u64) % 997) as f32;
        total
    }
}

pub mod m240 {
    use super::*;

    pub struct Accumulator240<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator240<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.963_f32 + y.sin();
        let b = y * 5.265_f32 - x.cos();
        let mut acc = Accumulator240::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.602_f32 + y.sin();
        let b = y * 0.307_f32 - x.cos();
        let mut acc = Accumulator240::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.305_f32 + y.sin();
        let b = y * 9.26_f32 - x.cos();
        let mut acc = Accumulator240::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.759_f32 + y.sin();
        let b = y * 4.286_f32 - x.cos();
        let mut acc = Accumulator240::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.348_f32 + y.sin();
        let b = y * 5.949_f32 - x.cos();
        let mut acc = Accumulator240::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.136_f32 + y.sin();
        let b = y * 4.304_f32 - x.cos();
        let mut acc = Accumulator240::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.428_f32 + y.sin();
        let b = y * 6.899_f32 - x.cos();
        let mut acc = Accumulator240::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.681_f32 + y.sin();
        let b = y * 3.09_f32 - x.cos();
        let mut acc = Accumulator240::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.711_f32 + y.sin();
        let b = y * 2.193_f32 - x.cos();
        let mut acc = Accumulator240::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.18_f32 + y.sin();
        let b = y * 7.496_f32 - x.cos();
        let mut acc = Accumulator240::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.311_f32 + y.sin();
        let b = y * 1.974_f32 - x.cos();
        let mut acc = Accumulator240::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.946_f32 + y.sin();
        let b = y * 9.371_f32 - x.cos();
        let mut acc = Accumulator240::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.953_f32 + y.sin();
        let b = y * 3.852_f32 - x.cos();
        let mut acc = Accumulator240::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.898_f32 + y.sin();
        let b = y * 9.291_f32 - x.cos();
        let mut acc = Accumulator240::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.786_f32 + y.sin();
        let b = y * 6.548_f32 - x.cos();
        let mut acc = Accumulator240::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.736_f32 + y.sin();
        let b = y * 7.162_f32 - x.cos();
        let mut acc = Accumulator240::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.496_f32 + y.sin();
        let b = y * 0.112_f32 - x.cos();
        let mut acc = Accumulator240::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.164_f32 + y.sin();
        let b = y * 8.979_f32 - x.cos();
        let mut acc = Accumulator240::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.344_f32 + y.sin();
        let b = y * 2.594_f32 - x.cos();
        let mut acc = Accumulator240::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.425_f32 + y.sin();
        let b = y * 5.955_f32 - x.cos();
        let mut acc = Accumulator240::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_240(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_240() -> f32 {
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
        total += (dep_touch_240(total as u64) % 997) as f32;
        total
    }
}

pub mod m241 {
    use super::*;

    pub struct Accumulator241<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator241<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.565_f32 + y.sin();
        let b = y * 8.905_f32 - x.cos();
        let mut acc = Accumulator241::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.406_f32 + y.sin();
        let b = y * 2.075_f32 - x.cos();
        let mut acc = Accumulator241::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.044_f32 + y.sin();
        let b = y * 0.608_f32 - x.cos();
        let mut acc = Accumulator241::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.106_f32 + y.sin();
        let b = y * 4.774_f32 - x.cos();
        let mut acc = Accumulator241::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.364_f32 + y.sin();
        let b = y * 9.611_f32 - x.cos();
        let mut acc = Accumulator241::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.205_f32 + y.sin();
        let b = y * 5.785_f32 - x.cos();
        let mut acc = Accumulator241::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.974_f32 + y.sin();
        let b = y * 1.259_f32 - x.cos();
        let mut acc = Accumulator241::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.641_f32 + y.sin();
        let b = y * 5.386_f32 - x.cos();
        let mut acc = Accumulator241::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.651_f32 + y.sin();
        let b = y * 2.335_f32 - x.cos();
        let mut acc = Accumulator241::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.115_f32 + y.sin();
        let b = y * 1.896_f32 - x.cos();
        let mut acc = Accumulator241::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.046_f32 + y.sin();
        let b = y * 3.248_f32 - x.cos();
        let mut acc = Accumulator241::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.673_f32 + y.sin();
        let b = y * 2.473_f32 - x.cos();
        let mut acc = Accumulator241::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.5_f32 + y.sin();
        let b = y * 8.912_f32 - x.cos();
        let mut acc = Accumulator241::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.99_f32 + y.sin();
        let b = y * 0.745_f32 - x.cos();
        let mut acc = Accumulator241::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.831_f32 + y.sin();
        let b = y * 3.165_f32 - x.cos();
        let mut acc = Accumulator241::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.91_f32 + y.sin();
        let b = y * 3.045_f32 - x.cos();
        let mut acc = Accumulator241::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.614_f32 + y.sin();
        let b = y * 7.129_f32 - x.cos();
        let mut acc = Accumulator241::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.103_f32 + y.sin();
        let b = y * 7.764_f32 - x.cos();
        let mut acc = Accumulator241::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.252_f32 + y.sin();
        let b = y * 3.821_f32 - x.cos();
        let mut acc = Accumulator241::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.54_f32 + y.sin();
        let b = y * 2.757_f32 - x.cos();
        let mut acc = Accumulator241::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_241(seed: u64) -> u64 {
        let re = Regex::new(r"m241-(\d+)").unwrap();
        let hay = format!("m241-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_241() -> f32 {
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
        total += (dep_touch_241(total as u64) % 997) as f32;
        total
    }
}

pub mod m242 {
    use super::*;

    pub struct Accumulator242<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator242<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.46_f32 + y.sin();
        let b = y * 9.746_f32 - x.cos();
        let mut acc = Accumulator242::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.499_f32 + y.sin();
        let b = y * 0.35_f32 - x.cos();
        let mut acc = Accumulator242::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.145_f32 + y.sin();
        let b = y * 0.179_f32 - x.cos();
        let mut acc = Accumulator242::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.423_f32 + y.sin();
        let b = y * 4.834_f32 - x.cos();
        let mut acc = Accumulator242::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.079_f32 + y.sin();
        let b = y * 6.259_f32 - x.cos();
        let mut acc = Accumulator242::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.334_f32 + y.sin();
        let b = y * 1.53_f32 - x.cos();
        let mut acc = Accumulator242::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.663_f32 + y.sin();
        let b = y * 1.966_f32 - x.cos();
        let mut acc = Accumulator242::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.209_f32 + y.sin();
        let b = y * 6.91_f32 - x.cos();
        let mut acc = Accumulator242::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.431_f32 + y.sin();
        let b = y * 6.261_f32 - x.cos();
        let mut acc = Accumulator242::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.156_f32 + y.sin();
        let b = y * 5.782_f32 - x.cos();
        let mut acc = Accumulator242::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.005_f32 + y.sin();
        let b = y * 7.823_f32 - x.cos();
        let mut acc = Accumulator242::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.267_f32 + y.sin();
        let b = y * 3.499_f32 - x.cos();
        let mut acc = Accumulator242::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.909_f32 + y.sin();
        let b = y * 0.351_f32 - x.cos();
        let mut acc = Accumulator242::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.372_f32 + y.sin();
        let b = y * 9.84_f32 - x.cos();
        let mut acc = Accumulator242::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.634_f32 + y.sin();
        let b = y * 4.423_f32 - x.cos();
        let mut acc = Accumulator242::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.403_f32 + y.sin();
        let b = y * 4.839_f32 - x.cos();
        let mut acc = Accumulator242::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.336_f32 + y.sin();
        let b = y * 8.615_f32 - x.cos();
        let mut acc = Accumulator242::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.566_f32 + y.sin();
        let b = y * 1.686_f32 - x.cos();
        let mut acc = Accumulator242::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.145_f32 + y.sin();
        let b = y * 9.057_f32 - x.cos();
        let mut acc = Accumulator242::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.974_f32 + y.sin();
        let b = y * 2.996_f32 - x.cos();
        let mut acc = Accumulator242::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_242(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_242() -> f32 {
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
        total += (dep_touch_242(total as u64) % 997) as f32;
        total
    }
}

pub mod m243 {
    use super::*;

    pub struct Accumulator243<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator243<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.618_f32 + y.sin();
        let b = y * 5.531_f32 - x.cos();
        let mut acc = Accumulator243::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.234_f32 + y.sin();
        let b = y * 8.107_f32 - x.cos();
        let mut acc = Accumulator243::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.752_f32 + y.sin();
        let b = y * 2.379_f32 - x.cos();
        let mut acc = Accumulator243::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.259_f32 + y.sin();
        let b = y * 5.706_f32 - x.cos();
        let mut acc = Accumulator243::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.665_f32 + y.sin();
        let b = y * 2.596_f32 - x.cos();
        let mut acc = Accumulator243::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.692_f32 + y.sin();
        let b = y * 3.964_f32 - x.cos();
        let mut acc = Accumulator243::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.254_f32 + y.sin();
        let b = y * 3.24_f32 - x.cos();
        let mut acc = Accumulator243::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.532_f32 + y.sin();
        let b = y * 6.226_f32 - x.cos();
        let mut acc = Accumulator243::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.883_f32 + y.sin();
        let b = y * 4.762_f32 - x.cos();
        let mut acc = Accumulator243::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.906_f32 + y.sin();
        let b = y * 9.364_f32 - x.cos();
        let mut acc = Accumulator243::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.385_f32 + y.sin();
        let b = y * 4.728_f32 - x.cos();
        let mut acc = Accumulator243::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.023_f32 + y.sin();
        let b = y * 9.356_f32 - x.cos();
        let mut acc = Accumulator243::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.235_f32 + y.sin();
        let b = y * 1.42_f32 - x.cos();
        let mut acc = Accumulator243::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.259_f32 + y.sin();
        let b = y * 7.642_f32 - x.cos();
        let mut acc = Accumulator243::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.469_f32 + y.sin();
        let b = y * 5.087_f32 - x.cos();
        let mut acc = Accumulator243::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.253_f32 + y.sin();
        let b = y * 7.226_f32 - x.cos();
        let mut acc = Accumulator243::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.692_f32 + y.sin();
        let b = y * 4.87_f32 - x.cos();
        let mut acc = Accumulator243::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.894_f32 + y.sin();
        let b = y * 1.094_f32 - x.cos();
        let mut acc = Accumulator243::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.755_f32 + y.sin();
        let b = y * 6.324_f32 - x.cos();
        let mut acc = Accumulator243::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.96_f32 + y.sin();
        let b = y * 3.214_f32 - x.cos();
        let mut acc = Accumulator243::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_243(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(243u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_243() -> f32 {
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
        total += (dep_touch_243(total as u64) % 997) as f32;
        total
    }
}

pub mod m244 {
    use super::*;

    pub struct Accumulator244<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator244<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.398_f32 + y.sin();
        let b = y * 7.934_f32 - x.cos();
        let mut acc = Accumulator244::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.731_f32 + y.sin();
        let b = y * 3.118_f32 - x.cos();
        let mut acc = Accumulator244::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.392_f32 + y.sin();
        let b = y * 4.03_f32 - x.cos();
        let mut acc = Accumulator244::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.256_f32 + y.sin();
        let b = y * 6.95_f32 - x.cos();
        let mut acc = Accumulator244::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.324_f32 + y.sin();
        let b = y * 0.739_f32 - x.cos();
        let mut acc = Accumulator244::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.763_f32 + y.sin();
        let b = y * 6.469_f32 - x.cos();
        let mut acc = Accumulator244::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.247_f32 + y.sin();
        let b = y * 0.684_f32 - x.cos();
        let mut acc = Accumulator244::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.348_f32 + y.sin();
        let b = y * 5.935_f32 - x.cos();
        let mut acc = Accumulator244::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.405_f32 + y.sin();
        let b = y * 4.032_f32 - x.cos();
        let mut acc = Accumulator244::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.835_f32 + y.sin();
        let b = y * 0.337_f32 - x.cos();
        let mut acc = Accumulator244::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.544_f32 + y.sin();
        let b = y * 5.14_f32 - x.cos();
        let mut acc = Accumulator244::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.958_f32 + y.sin();
        let b = y * 9.533_f32 - x.cos();
        let mut acc = Accumulator244::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.618_f32 + y.sin();
        let b = y * 0.897_f32 - x.cos();
        let mut acc = Accumulator244::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.276_f32 + y.sin();
        let b = y * 5.333_f32 - x.cos();
        let mut acc = Accumulator244::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.087_f32 + y.sin();
        let b = y * 8.158_f32 - x.cos();
        let mut acc = Accumulator244::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.901_f32 + y.sin();
        let b = y * 4.875_f32 - x.cos();
        let mut acc = Accumulator244::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.798_f32 + y.sin();
        let b = y * 6.671_f32 - x.cos();
        let mut acc = Accumulator244::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.11_f32 + y.sin();
        let b = y * 3.258_f32 - x.cos();
        let mut acc = Accumulator244::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.043_f32 + y.sin();
        let b = y * 5.94_f32 - x.cos();
        let mut acc = Accumulator244::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.757_f32 + y.sin();
        let b = y * 7.561_f32 - x.cos();
        let mut acc = Accumulator244::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_244(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_244() -> f32 {
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
        total += (dep_touch_244(total as u64) % 997) as f32;
        total
    }
}

pub mod m245 {
    use super::*;

    pub struct Accumulator245<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator245<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.006_f32 + y.sin();
        let b = y * 6.079_f32 - x.cos();
        let mut acc = Accumulator245::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.818_f32 + y.sin();
        let b = y * 4.8_f32 - x.cos();
        let mut acc = Accumulator245::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.006_f32 + y.sin();
        let b = y * 6.746_f32 - x.cos();
        let mut acc = Accumulator245::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.83_f32 + y.sin();
        let b = y * 3.71_f32 - x.cos();
        let mut acc = Accumulator245::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.271_f32 + y.sin();
        let b = y * 5.003_f32 - x.cos();
        let mut acc = Accumulator245::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.038_f32 + y.sin();
        let b = y * 6.408_f32 - x.cos();
        let mut acc = Accumulator245::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.558_f32 + y.sin();
        let b = y * 1.338_f32 - x.cos();
        let mut acc = Accumulator245::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.437_f32 + y.sin();
        let b = y * 8.248_f32 - x.cos();
        let mut acc = Accumulator245::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.211_f32 + y.sin();
        let b = y * 7.153_f32 - x.cos();
        let mut acc = Accumulator245::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.991_f32 + y.sin();
        let b = y * 0.149_f32 - x.cos();
        let mut acc = Accumulator245::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.571_f32 + y.sin();
        let b = y * 1.891_f32 - x.cos();
        let mut acc = Accumulator245::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.981_f32 + y.sin();
        let b = y * 2.897_f32 - x.cos();
        let mut acc = Accumulator245::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.912_f32 + y.sin();
        let b = y * 1.102_f32 - x.cos();
        let mut acc = Accumulator245::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.13_f32 + y.sin();
        let b = y * 3.445_f32 - x.cos();
        let mut acc = Accumulator245::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.292_f32 + y.sin();
        let b = y * 2.739_f32 - x.cos();
        let mut acc = Accumulator245::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.959_f32 + y.sin();
        let b = y * 2.131_f32 - x.cos();
        let mut acc = Accumulator245::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.96_f32 + y.sin();
        let b = y * 2.809_f32 - x.cos();
        let mut acc = Accumulator245::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.131_f32 + y.sin();
        let b = y * 1.966_f32 - x.cos();
        let mut acc = Accumulator245::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.234_f32 + y.sin();
        let b = y * 0.246_f32 - x.cos();
        let mut acc = Accumulator245::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.643_f32 + y.sin();
        let b = y * 0.683_f32 - x.cos();
        let mut acc = Accumulator245::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_245(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_245() -> f32 {
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
        total += (dep_touch_245(total as u64) % 997) as f32;
        total
    }
}

pub mod m246 {
    use super::*;

    pub struct Accumulator246<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator246<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.818_f32 + y.sin();
        let b = y * 2.366_f32 - x.cos();
        let mut acc = Accumulator246::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.999_f32 + y.sin();
        let b = y * 4.423_f32 - x.cos();
        let mut acc = Accumulator246::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.753_f32 + y.sin();
        let b = y * 2.423_f32 - x.cos();
        let mut acc = Accumulator246::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.925_f32 + y.sin();
        let b = y * 6.32_f32 - x.cos();
        let mut acc = Accumulator246::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.242_f32 + y.sin();
        let b = y * 3.613_f32 - x.cos();
        let mut acc = Accumulator246::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.45_f32 + y.sin();
        let b = y * 8.438_f32 - x.cos();
        let mut acc = Accumulator246::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.018_f32 + y.sin();
        let b = y * 7.202_f32 - x.cos();
        let mut acc = Accumulator246::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.478_f32 + y.sin();
        let b = y * 2.458_f32 - x.cos();
        let mut acc = Accumulator246::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.116_f32 + y.sin();
        let b = y * 6.116_f32 - x.cos();
        let mut acc = Accumulator246::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.298_f32 + y.sin();
        let b = y * 8.896_f32 - x.cos();
        let mut acc = Accumulator246::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.794_f32 + y.sin();
        let b = y * 2.006_f32 - x.cos();
        let mut acc = Accumulator246::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.151_f32 + y.sin();
        let b = y * 5.886_f32 - x.cos();
        let mut acc = Accumulator246::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.232_f32 + y.sin();
        let b = y * 4.901_f32 - x.cos();
        let mut acc = Accumulator246::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.697_f32 + y.sin();
        let b = y * 1.462_f32 - x.cos();
        let mut acc = Accumulator246::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.03_f32 + y.sin();
        let b = y * 1.595_f32 - x.cos();
        let mut acc = Accumulator246::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.825_f32 + y.sin();
        let b = y * 1.662_f32 - x.cos();
        let mut acc = Accumulator246::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.279_f32 + y.sin();
        let b = y * 1.937_f32 - x.cos();
        let mut acc = Accumulator246::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.604_f32 + y.sin();
        let b = y * 5.176_f32 - x.cos();
        let mut acc = Accumulator246::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.952_f32 + y.sin();
        let b = y * 0.798_f32 - x.cos();
        let mut acc = Accumulator246::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.734_f32 + y.sin();
        let b = y * 7.411_f32 - x.cos();
        let mut acc = Accumulator246::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_246(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m246-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_246() -> f32 {
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
        total += (dep_touch_246(total as u64) % 997) as f32;
        total
    }
}

pub mod m247 {
    use super::*;

    pub struct Accumulator247<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator247<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.833_f32 + y.sin();
        let b = y * 6.265_f32 - x.cos();
        let mut acc = Accumulator247::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.77_f32 + y.sin();
        let b = y * 9.013_f32 - x.cos();
        let mut acc = Accumulator247::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.14_f32 + y.sin();
        let b = y * 3.406_f32 - x.cos();
        let mut acc = Accumulator247::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.058_f32 + y.sin();
        let b = y * 8.375_f32 - x.cos();
        let mut acc = Accumulator247::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.927_f32 + y.sin();
        let b = y * 4.253_f32 - x.cos();
        let mut acc = Accumulator247::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.447_f32 + y.sin();
        let b = y * 4.404_f32 - x.cos();
        let mut acc = Accumulator247::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.565_f32 + y.sin();
        let b = y * 9.407_f32 - x.cos();
        let mut acc = Accumulator247::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.564_f32 + y.sin();
        let b = y * 1.276_f32 - x.cos();
        let mut acc = Accumulator247::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.933_f32 + y.sin();
        let b = y * 8.448_f32 - x.cos();
        let mut acc = Accumulator247::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.138_f32 + y.sin();
        let b = y * 9.378_f32 - x.cos();
        let mut acc = Accumulator247::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.0_f32 + y.sin();
        let b = y * 1.077_f32 - x.cos();
        let mut acc = Accumulator247::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.881_f32 + y.sin();
        let b = y * 9.107_f32 - x.cos();
        let mut acc = Accumulator247::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.925_f32 + y.sin();
        let b = y * 3.603_f32 - x.cos();
        let mut acc = Accumulator247::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.628_f32 + y.sin();
        let b = y * 0.367_f32 - x.cos();
        let mut acc = Accumulator247::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.543_f32 + y.sin();
        let b = y * 8.879_f32 - x.cos();
        let mut acc = Accumulator247::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.087_f32 + y.sin();
        let b = y * 4.296_f32 - x.cos();
        let mut acc = Accumulator247::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.876_f32 + y.sin();
        let b = y * 8.667_f32 - x.cos();
        let mut acc = Accumulator247::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.299_f32 + y.sin();
        let b = y * 3.658_f32 - x.cos();
        let mut acc = Accumulator247::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.153_f32 + y.sin();
        let b = y * 0.426_f32 - x.cos();
        let mut acc = Accumulator247::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.758_f32 + y.sin();
        let b = y * 0.973_f32 - x.cos();
        let mut acc = Accumulator247::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_247(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_247() -> f32 {
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
        total += (dep_touch_247(total as u64) % 997) as f32;
        total
    }
}

pub mod m248 {
    use super::*;

    pub struct Accumulator248<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator248<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.663_f32 + y.sin();
        let b = y * 0.998_f32 - x.cos();
        let mut acc = Accumulator248::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.983_f32 + y.sin();
        let b = y * 4.308_f32 - x.cos();
        let mut acc = Accumulator248::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.731_f32 + y.sin();
        let b = y * 2.648_f32 - x.cos();
        let mut acc = Accumulator248::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.226_f32 + y.sin();
        let b = y * 9.07_f32 - x.cos();
        let mut acc = Accumulator248::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.641_f32 + y.sin();
        let b = y * 4.962_f32 - x.cos();
        let mut acc = Accumulator248::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.93_f32 + y.sin();
        let b = y * 4.368_f32 - x.cos();
        let mut acc = Accumulator248::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.939_f32 + y.sin();
        let b = y * 9.275_f32 - x.cos();
        let mut acc = Accumulator248::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.642_f32 + y.sin();
        let b = y * 1.58_f32 - x.cos();
        let mut acc = Accumulator248::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.99_f32 + y.sin();
        let b = y * 2.156_f32 - x.cos();
        let mut acc = Accumulator248::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.902_f32 + y.sin();
        let b = y * 2.283_f32 - x.cos();
        let mut acc = Accumulator248::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.985_f32 + y.sin();
        let b = y * 3.7_f32 - x.cos();
        let mut acc = Accumulator248::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.237_f32 + y.sin();
        let b = y * 8.2_f32 - x.cos();
        let mut acc = Accumulator248::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.488_f32 + y.sin();
        let b = y * 7.865_f32 - x.cos();
        let mut acc = Accumulator248::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.479_f32 + y.sin();
        let b = y * 0.453_f32 - x.cos();
        let mut acc = Accumulator248::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.278_f32 + y.sin();
        let b = y * 8.215_f32 - x.cos();
        let mut acc = Accumulator248::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.685_f32 + y.sin();
        let b = y * 1.475_f32 - x.cos();
        let mut acc = Accumulator248::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.656_f32 + y.sin();
        let b = y * 9.32_f32 - x.cos();
        let mut acc = Accumulator248::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.076_f32 + y.sin();
        let b = y * 6.083_f32 - x.cos();
        let mut acc = Accumulator248::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.179_f32 + y.sin();
        let b = y * 5.004_f32 - x.cos();
        let mut acc = Accumulator248::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.281_f32 + y.sin();
        let b = y * 8.895_f32 - x.cos();
        let mut acc = Accumulator248::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_248(seed: u64) -> u64 {
        let re = Regex::new(r"m248-(\d+)").unwrap();
        let hay = format!("m248-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_248() -> f32 {
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
        total += (dep_touch_248(total as u64) % 997) as f32;
        total
    }
}

pub mod m249 {
    use super::*;

    pub struct Accumulator249<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator249<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.73_f32 + y.sin();
        let b = y * 1.882_f32 - x.cos();
        let mut acc = Accumulator249::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.402_f32 + y.sin();
        let b = y * 2.563_f32 - x.cos();
        let mut acc = Accumulator249::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.708_f32 + y.sin();
        let b = y * 0.74_f32 - x.cos();
        let mut acc = Accumulator249::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.899_f32 + y.sin();
        let b = y * 3.508_f32 - x.cos();
        let mut acc = Accumulator249::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.467_f32 + y.sin();
        let b = y * 1.38_f32 - x.cos();
        let mut acc = Accumulator249::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.725_f32 + y.sin();
        let b = y * 8.021_f32 - x.cos();
        let mut acc = Accumulator249::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.81_f32 + y.sin();
        let b = y * 9.41_f32 - x.cos();
        let mut acc = Accumulator249::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.205_f32 + y.sin();
        let b = y * 1.419_f32 - x.cos();
        let mut acc = Accumulator249::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.457_f32 + y.sin();
        let b = y * 4.34_f32 - x.cos();
        let mut acc = Accumulator249::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.809_f32 + y.sin();
        let b = y * 0.641_f32 - x.cos();
        let mut acc = Accumulator249::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.536_f32 + y.sin();
        let b = y * 3.219_f32 - x.cos();
        let mut acc = Accumulator249::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.067_f32 + y.sin();
        let b = y * 0.549_f32 - x.cos();
        let mut acc = Accumulator249::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.527_f32 + y.sin();
        let b = y * 8.157_f32 - x.cos();
        let mut acc = Accumulator249::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.827_f32 + y.sin();
        let b = y * 1.837_f32 - x.cos();
        let mut acc = Accumulator249::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.052_f32 + y.sin();
        let b = y * 6.453_f32 - x.cos();
        let mut acc = Accumulator249::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.27_f32 + y.sin();
        let b = y * 8.33_f32 - x.cos();
        let mut acc = Accumulator249::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.679_f32 + y.sin();
        let b = y * 7.211_f32 - x.cos();
        let mut acc = Accumulator249::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.853_f32 + y.sin();
        let b = y * 6.444_f32 - x.cos();
        let mut acc = Accumulator249::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.035_f32 + y.sin();
        let b = y * 3.447_f32 - x.cos();
        let mut acc = Accumulator249::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.479_f32 + y.sin();
        let b = y * 4.588_f32 - x.cos();
        let mut acc = Accumulator249::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_249(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_249() -> f32 {
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
        total += (dep_touch_249(total as u64) % 997) as f32;
        total
    }
}

pub mod m250 {
    use super::*;

    pub struct Accumulator250<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator250<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.913_f32 + y.sin();
        let b = y * 7.828_f32 - x.cos();
        let mut acc = Accumulator250::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.277_f32 + y.sin();
        let b = y * 2.134_f32 - x.cos();
        let mut acc = Accumulator250::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.947_f32 + y.sin();
        let b = y * 5.387_f32 - x.cos();
        let mut acc = Accumulator250::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.29_f32 + y.sin();
        let b = y * 8.315_f32 - x.cos();
        let mut acc = Accumulator250::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.533_f32 + y.sin();
        let b = y * 8.841_f32 - x.cos();
        let mut acc = Accumulator250::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.942_f32 + y.sin();
        let b = y * 1.929_f32 - x.cos();
        let mut acc = Accumulator250::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.329_f32 + y.sin();
        let b = y * 1.844_f32 - x.cos();
        let mut acc = Accumulator250::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.534_f32 + y.sin();
        let b = y * 3.228_f32 - x.cos();
        let mut acc = Accumulator250::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.316_f32 + y.sin();
        let b = y * 2.591_f32 - x.cos();
        let mut acc = Accumulator250::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.719_f32 + y.sin();
        let b = y * 0.755_f32 - x.cos();
        let mut acc = Accumulator250::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.163_f32 + y.sin();
        let b = y * 9.489_f32 - x.cos();
        let mut acc = Accumulator250::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.839_f32 + y.sin();
        let b = y * 1.984_f32 - x.cos();
        let mut acc = Accumulator250::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.093_f32 + y.sin();
        let b = y * 8.394_f32 - x.cos();
        let mut acc = Accumulator250::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.281_f32 + y.sin();
        let b = y * 4.709_f32 - x.cos();
        let mut acc = Accumulator250::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.764_f32 + y.sin();
        let b = y * 8.101_f32 - x.cos();
        let mut acc = Accumulator250::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.629_f32 + y.sin();
        let b = y * 7.225_f32 - x.cos();
        let mut acc = Accumulator250::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.183_f32 + y.sin();
        let b = y * 3.848_f32 - x.cos();
        let mut acc = Accumulator250::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.918_f32 + y.sin();
        let b = y * 8.339_f32 - x.cos();
        let mut acc = Accumulator250::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.703_f32 + y.sin();
        let b = y * 1.798_f32 - x.cos();
        let mut acc = Accumulator250::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.963_f32 + y.sin();
        let b = y * 5.549_f32 - x.cos();
        let mut acc = Accumulator250::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_250(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(250u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_250() -> f32 {
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
        total += (dep_touch_250(total as u64) % 997) as f32;
        total
    }
}

pub mod m251 {
    use super::*;

    pub struct Accumulator251<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator251<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.919_f32 + y.sin();
        let b = y * 2.657_f32 - x.cos();
        let mut acc = Accumulator251::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.51_f32 + y.sin();
        let b = y * 7.701_f32 - x.cos();
        let mut acc = Accumulator251::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.252_f32 + y.sin();
        let b = y * 4.026_f32 - x.cos();
        let mut acc = Accumulator251::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.871_f32 + y.sin();
        let b = y * 7.665_f32 - x.cos();
        let mut acc = Accumulator251::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.7_f32 + y.sin();
        let b = y * 0.521_f32 - x.cos();
        let mut acc = Accumulator251::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.903_f32 + y.sin();
        let b = y * 7.448_f32 - x.cos();
        let mut acc = Accumulator251::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.716_f32 + y.sin();
        let b = y * 3.314_f32 - x.cos();
        let mut acc = Accumulator251::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.356_f32 + y.sin();
        let b = y * 5.549_f32 - x.cos();
        let mut acc = Accumulator251::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.657_f32 + y.sin();
        let b = y * 1.722_f32 - x.cos();
        let mut acc = Accumulator251::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.552_f32 + y.sin();
        let b = y * 2.351_f32 - x.cos();
        let mut acc = Accumulator251::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.806_f32 + y.sin();
        let b = y * 8.783_f32 - x.cos();
        let mut acc = Accumulator251::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.562_f32 + y.sin();
        let b = y * 3.131_f32 - x.cos();
        let mut acc = Accumulator251::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.355_f32 + y.sin();
        let b = y * 3.893_f32 - x.cos();
        let mut acc = Accumulator251::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.318_f32 + y.sin();
        let b = y * 2.464_f32 - x.cos();
        let mut acc = Accumulator251::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.921_f32 + y.sin();
        let b = y * 5.669_f32 - x.cos();
        let mut acc = Accumulator251::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.862_f32 + y.sin();
        let b = y * 1.28_f32 - x.cos();
        let mut acc = Accumulator251::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.576_f32 + y.sin();
        let b = y * 4.229_f32 - x.cos();
        let mut acc = Accumulator251::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.071_f32 + y.sin();
        let b = y * 2.67_f32 - x.cos();
        let mut acc = Accumulator251::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.84_f32 + y.sin();
        let b = y * 4.192_f32 - x.cos();
        let mut acc = Accumulator251::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.287_f32 + y.sin();
        let b = y * 3.349_f32 - x.cos();
        let mut acc = Accumulator251::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_251(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_251() -> f32 {
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
        total += (dep_touch_251(total as u64) % 997) as f32;
        total
    }
}

pub mod m252 {
    use super::*;

    pub struct Accumulator252<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator252<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.073_f32 + y.sin();
        let b = y * 5.201_f32 - x.cos();
        let mut acc = Accumulator252::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.419_f32 + y.sin();
        let b = y * 6.221_f32 - x.cos();
        let mut acc = Accumulator252::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.278_f32 + y.sin();
        let b = y * 0.882_f32 - x.cos();
        let mut acc = Accumulator252::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.419_f32 + y.sin();
        let b = y * 6.949_f32 - x.cos();
        let mut acc = Accumulator252::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.63_f32 + y.sin();
        let b = y * 0.366_f32 - x.cos();
        let mut acc = Accumulator252::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.642_f32 + y.sin();
        let b = y * 4.757_f32 - x.cos();
        let mut acc = Accumulator252::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.013_f32 + y.sin();
        let b = y * 3.15_f32 - x.cos();
        let mut acc = Accumulator252::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.796_f32 + y.sin();
        let b = y * 4.097_f32 - x.cos();
        let mut acc = Accumulator252::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.146_f32 + y.sin();
        let b = y * 2.061_f32 - x.cos();
        let mut acc = Accumulator252::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.271_f32 + y.sin();
        let b = y * 3.344_f32 - x.cos();
        let mut acc = Accumulator252::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.985_f32 + y.sin();
        let b = y * 7.853_f32 - x.cos();
        let mut acc = Accumulator252::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.065_f32 + y.sin();
        let b = y * 3.77_f32 - x.cos();
        let mut acc = Accumulator252::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.847_f32 + y.sin();
        let b = y * 2.073_f32 - x.cos();
        let mut acc = Accumulator252::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.283_f32 + y.sin();
        let b = y * 4.721_f32 - x.cos();
        let mut acc = Accumulator252::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.665_f32 + y.sin();
        let b = y * 3.067_f32 - x.cos();
        let mut acc = Accumulator252::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.672_f32 + y.sin();
        let b = y * 8.213_f32 - x.cos();
        let mut acc = Accumulator252::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.499_f32 + y.sin();
        let b = y * 5.93_f32 - x.cos();
        let mut acc = Accumulator252::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.884_f32 + y.sin();
        let b = y * 5.268_f32 - x.cos();
        let mut acc = Accumulator252::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.426_f32 + y.sin();
        let b = y * 0.576_f32 - x.cos();
        let mut acc = Accumulator252::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.271_f32 + y.sin();
        let b = y * 4.63_f32 - x.cos();
        let mut acc = Accumulator252::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_252(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_252() -> f32 {
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
        total += (dep_touch_252(total as u64) % 997) as f32;
        total
    }
}

pub mod m253 {
    use super::*;

    pub struct Accumulator253<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator253<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.042_f32 + y.sin();
        let b = y * 9.633_f32 - x.cos();
        let mut acc = Accumulator253::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.515_f32 + y.sin();
        let b = y * 1.693_f32 - x.cos();
        let mut acc = Accumulator253::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.032_f32 + y.sin();
        let b = y * 0.301_f32 - x.cos();
        let mut acc = Accumulator253::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.069_f32 + y.sin();
        let b = y * 0.423_f32 - x.cos();
        let mut acc = Accumulator253::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.96_f32 + y.sin();
        let b = y * 7.303_f32 - x.cos();
        let mut acc = Accumulator253::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.414_f32 + y.sin();
        let b = y * 0.143_f32 - x.cos();
        let mut acc = Accumulator253::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.7_f32 + y.sin();
        let b = y * 1.152_f32 - x.cos();
        let mut acc = Accumulator253::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.194_f32 + y.sin();
        let b = y * 8.58_f32 - x.cos();
        let mut acc = Accumulator253::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.457_f32 + y.sin();
        let b = y * 6.972_f32 - x.cos();
        let mut acc = Accumulator253::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.544_f32 + y.sin();
        let b = y * 5.613_f32 - x.cos();
        let mut acc = Accumulator253::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.186_f32 + y.sin();
        let b = y * 3.142_f32 - x.cos();
        let mut acc = Accumulator253::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.044_f32 + y.sin();
        let b = y * 7.591_f32 - x.cos();
        let mut acc = Accumulator253::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.674_f32 + y.sin();
        let b = y * 1.289_f32 - x.cos();
        let mut acc = Accumulator253::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.442_f32 + y.sin();
        let b = y * 1.737_f32 - x.cos();
        let mut acc = Accumulator253::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.224_f32 + y.sin();
        let b = y * 5.23_f32 - x.cos();
        let mut acc = Accumulator253::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.424_f32 + y.sin();
        let b = y * 9.322_f32 - x.cos();
        let mut acc = Accumulator253::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.187_f32 + y.sin();
        let b = y * 8.796_f32 - x.cos();
        let mut acc = Accumulator253::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.744_f32 + y.sin();
        let b = y * 8.545_f32 - x.cos();
        let mut acc = Accumulator253::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.969_f32 + y.sin();
        let b = y * 1.953_f32 - x.cos();
        let mut acc = Accumulator253::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.177_f32 + y.sin();
        let b = y * 1.508_f32 - x.cos();
        let mut acc = Accumulator253::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_253(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m253-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_253() -> f32 {
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
        total += (dep_touch_253(total as u64) % 997) as f32;
        total
    }
}

pub mod m254 {
    use super::*;

    pub struct Accumulator254<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator254<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.287_f32 + y.sin();
        let b = y * 1.229_f32 - x.cos();
        let mut acc = Accumulator254::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.211_f32 + y.sin();
        let b = y * 4.464_f32 - x.cos();
        let mut acc = Accumulator254::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.125_f32 + y.sin();
        let b = y * 7.019_f32 - x.cos();
        let mut acc = Accumulator254::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.215_f32 + y.sin();
        let b = y * 0.735_f32 - x.cos();
        let mut acc = Accumulator254::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.771_f32 + y.sin();
        let b = y * 6.962_f32 - x.cos();
        let mut acc = Accumulator254::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.118_f32 + y.sin();
        let b = y * 9.145_f32 - x.cos();
        let mut acc = Accumulator254::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.62_f32 + y.sin();
        let b = y * 7.208_f32 - x.cos();
        let mut acc = Accumulator254::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.336_f32 + y.sin();
        let b = y * 5.331_f32 - x.cos();
        let mut acc = Accumulator254::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.479_f32 + y.sin();
        let b = y * 8.647_f32 - x.cos();
        let mut acc = Accumulator254::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.611_f32 + y.sin();
        let b = y * 9.872_f32 - x.cos();
        let mut acc = Accumulator254::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.753_f32 + y.sin();
        let b = y * 0.976_f32 - x.cos();
        let mut acc = Accumulator254::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.234_f32 + y.sin();
        let b = y * 8.783_f32 - x.cos();
        let mut acc = Accumulator254::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.862_f32 + y.sin();
        let b = y * 4.437_f32 - x.cos();
        let mut acc = Accumulator254::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.657_f32 + y.sin();
        let b = y * 6.223_f32 - x.cos();
        let mut acc = Accumulator254::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.726_f32 + y.sin();
        let b = y * 3.998_f32 - x.cos();
        let mut acc = Accumulator254::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.94_f32 + y.sin();
        let b = y * 7.037_f32 - x.cos();
        let mut acc = Accumulator254::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.603_f32 + y.sin();
        let b = y * 8.062_f32 - x.cos();
        let mut acc = Accumulator254::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.499_f32 + y.sin();
        let b = y * 2.116_f32 - x.cos();
        let mut acc = Accumulator254::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.892_f32 + y.sin();
        let b = y * 7.196_f32 - x.cos();
        let mut acc = Accumulator254::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.078_f32 + y.sin();
        let b = y * 5.141_f32 - x.cos();
        let mut acc = Accumulator254::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_254(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_254() -> f32 {
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
        total += (dep_touch_254(total as u64) % 997) as f32;
        total
    }
}

pub mod m255 {
    use super::*;

    pub struct Accumulator255<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator255<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.917_f32 + y.sin();
        let b = y * 3.494_f32 - x.cos();
        let mut acc = Accumulator255::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.226_f32 + y.sin();
        let b = y * 2.748_f32 - x.cos();
        let mut acc = Accumulator255::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.793_f32 + y.sin();
        let b = y * 2.083_f32 - x.cos();
        let mut acc = Accumulator255::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.157_f32 + y.sin();
        let b = y * 8.063_f32 - x.cos();
        let mut acc = Accumulator255::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.989_f32 + y.sin();
        let b = y * 2.087_f32 - x.cos();
        let mut acc = Accumulator255::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.45_f32 + y.sin();
        let b = y * 5.822_f32 - x.cos();
        let mut acc = Accumulator255::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.685_f32 + y.sin();
        let b = y * 4.112_f32 - x.cos();
        let mut acc = Accumulator255::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.421_f32 + y.sin();
        let b = y * 3.665_f32 - x.cos();
        let mut acc = Accumulator255::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.559_f32 + y.sin();
        let b = y * 3.138_f32 - x.cos();
        let mut acc = Accumulator255::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.13_f32 + y.sin();
        let b = y * 3.886_f32 - x.cos();
        let mut acc = Accumulator255::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.351_f32 + y.sin();
        let b = y * 8.455_f32 - x.cos();
        let mut acc = Accumulator255::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.349_f32 + y.sin();
        let b = y * 2.816_f32 - x.cos();
        let mut acc = Accumulator255::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.551_f32 + y.sin();
        let b = y * 4.428_f32 - x.cos();
        let mut acc = Accumulator255::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.805_f32 + y.sin();
        let b = y * 0.45_f32 - x.cos();
        let mut acc = Accumulator255::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.797_f32 + y.sin();
        let b = y * 6.982_f32 - x.cos();
        let mut acc = Accumulator255::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.406_f32 + y.sin();
        let b = y * 0.16_f32 - x.cos();
        let mut acc = Accumulator255::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.725_f32 + y.sin();
        let b = y * 0.824_f32 - x.cos();
        let mut acc = Accumulator255::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.625_f32 + y.sin();
        let b = y * 5.45_f32 - x.cos();
        let mut acc = Accumulator255::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.475_f32 + y.sin();
        let b = y * 6.825_f32 - x.cos();
        let mut acc = Accumulator255::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.523_f32 + y.sin();
        let b = y * 2.041_f32 - x.cos();
        let mut acc = Accumulator255::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_255(seed: u64) -> u64 {
        let re = Regex::new(r"m255-(\d+)").unwrap();
        let hay = format!("m255-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_255() -> f32 {
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
        total += (dep_touch_255(total as u64) % 997) as f32;
        total
    }
}

pub mod m256 {
    use super::*;

    pub struct Accumulator256<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator256<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.459_f32 + y.sin();
        let b = y * 2.924_f32 - x.cos();
        let mut acc = Accumulator256::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.502_f32 + y.sin();
        let b = y * 3.956_f32 - x.cos();
        let mut acc = Accumulator256::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.513_f32 + y.sin();
        let b = y * 7.706_f32 - x.cos();
        let mut acc = Accumulator256::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.598_f32 + y.sin();
        let b = y * 8.862_f32 - x.cos();
        let mut acc = Accumulator256::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.43_f32 + y.sin();
        let b = y * 4.581_f32 - x.cos();
        let mut acc = Accumulator256::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.944_f32 + y.sin();
        let b = y * 2.908_f32 - x.cos();
        let mut acc = Accumulator256::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.795_f32 + y.sin();
        let b = y * 7.706_f32 - x.cos();
        let mut acc = Accumulator256::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.72_f32 + y.sin();
        let b = y * 8.791_f32 - x.cos();
        let mut acc = Accumulator256::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.868_f32 + y.sin();
        let b = y * 8.614_f32 - x.cos();
        let mut acc = Accumulator256::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.695_f32 + y.sin();
        let b = y * 3.666_f32 - x.cos();
        let mut acc = Accumulator256::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.89_f32 + y.sin();
        let b = y * 2.85_f32 - x.cos();
        let mut acc = Accumulator256::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.209_f32 + y.sin();
        let b = y * 9.248_f32 - x.cos();
        let mut acc = Accumulator256::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.742_f32 + y.sin();
        let b = y * 0.964_f32 - x.cos();
        let mut acc = Accumulator256::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.615_f32 + y.sin();
        let b = y * 6.476_f32 - x.cos();
        let mut acc = Accumulator256::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.223_f32 + y.sin();
        let b = y * 8.424_f32 - x.cos();
        let mut acc = Accumulator256::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.272_f32 + y.sin();
        let b = y * 8.225_f32 - x.cos();
        let mut acc = Accumulator256::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.913_f32 + y.sin();
        let b = y * 8.651_f32 - x.cos();
        let mut acc = Accumulator256::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.845_f32 + y.sin();
        let b = y * 4.968_f32 - x.cos();
        let mut acc = Accumulator256::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.554_f32 + y.sin();
        let b = y * 8.034_f32 - x.cos();
        let mut acc = Accumulator256::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.817_f32 + y.sin();
        let b = y * 0.744_f32 - x.cos();
        let mut acc = Accumulator256::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_256(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_256() -> f32 {
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
        total += (dep_touch_256(total as u64) % 997) as f32;
        total
    }
}

pub mod m257 {
    use super::*;

    pub struct Accumulator257<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator257<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.868_f32 + y.sin();
        let b = y * 9.517_f32 - x.cos();
        let mut acc = Accumulator257::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.337_f32 + y.sin();
        let b = y * 1.657_f32 - x.cos();
        let mut acc = Accumulator257::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.887_f32 + y.sin();
        let b = y * 3.243_f32 - x.cos();
        let mut acc = Accumulator257::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.407_f32 + y.sin();
        let b = y * 6.846_f32 - x.cos();
        let mut acc = Accumulator257::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.137_f32 + y.sin();
        let b = y * 5.707_f32 - x.cos();
        let mut acc = Accumulator257::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.805_f32 + y.sin();
        let b = y * 6.895_f32 - x.cos();
        let mut acc = Accumulator257::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.593_f32 + y.sin();
        let b = y * 2.268_f32 - x.cos();
        let mut acc = Accumulator257::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.84_f32 + y.sin();
        let b = y * 6.115_f32 - x.cos();
        let mut acc = Accumulator257::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.19_f32 + y.sin();
        let b = y * 8.615_f32 - x.cos();
        let mut acc = Accumulator257::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.714_f32 + y.sin();
        let b = y * 3.294_f32 - x.cos();
        let mut acc = Accumulator257::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.708_f32 + y.sin();
        let b = y * 8.641_f32 - x.cos();
        let mut acc = Accumulator257::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.344_f32 + y.sin();
        let b = y * 0.677_f32 - x.cos();
        let mut acc = Accumulator257::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.868_f32 + y.sin();
        let b = y * 0.106_f32 - x.cos();
        let mut acc = Accumulator257::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.107_f32 + y.sin();
        let b = y * 1.278_f32 - x.cos();
        let mut acc = Accumulator257::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.385_f32 + y.sin();
        let b = y * 8.812_f32 - x.cos();
        let mut acc = Accumulator257::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.335_f32 + y.sin();
        let b = y * 1.782_f32 - x.cos();
        let mut acc = Accumulator257::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.878_f32 + y.sin();
        let b = y * 2.288_f32 - x.cos();
        let mut acc = Accumulator257::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.096_f32 + y.sin();
        let b = y * 5.045_f32 - x.cos();
        let mut acc = Accumulator257::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.609_f32 + y.sin();
        let b = y * 5.354_f32 - x.cos();
        let mut acc = Accumulator257::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.919_f32 + y.sin();
        let b = y * 3.645_f32 - x.cos();
        let mut acc = Accumulator257::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_257(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(257u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_257() -> f32 {
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
        total += (dep_touch_257(total as u64) % 997) as f32;
        total
    }
}

pub mod m258 {
    use super::*;

    pub struct Accumulator258<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator258<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.066_f32 + y.sin();
        let b = y * 1.085_f32 - x.cos();
        let mut acc = Accumulator258::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.101_f32 + y.sin();
        let b = y * 4.205_f32 - x.cos();
        let mut acc = Accumulator258::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.966_f32 + y.sin();
        let b = y * 3.155_f32 - x.cos();
        let mut acc = Accumulator258::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.664_f32 + y.sin();
        let b = y * 7.709_f32 - x.cos();
        let mut acc = Accumulator258::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.916_f32 + y.sin();
        let b = y * 8.426_f32 - x.cos();
        let mut acc = Accumulator258::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.321_f32 + y.sin();
        let b = y * 4.534_f32 - x.cos();
        let mut acc = Accumulator258::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.777_f32 + y.sin();
        let b = y * 7.547_f32 - x.cos();
        let mut acc = Accumulator258::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.586_f32 + y.sin();
        let b = y * 6.2_f32 - x.cos();
        let mut acc = Accumulator258::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.778_f32 + y.sin();
        let b = y * 0.992_f32 - x.cos();
        let mut acc = Accumulator258::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.158_f32 + y.sin();
        let b = y * 0.92_f32 - x.cos();
        let mut acc = Accumulator258::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.39_f32 + y.sin();
        let b = y * 0.897_f32 - x.cos();
        let mut acc = Accumulator258::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.861_f32 + y.sin();
        let b = y * 5.36_f32 - x.cos();
        let mut acc = Accumulator258::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.24_f32 + y.sin();
        let b = y * 2.377_f32 - x.cos();
        let mut acc = Accumulator258::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.198_f32 + y.sin();
        let b = y * 3.321_f32 - x.cos();
        let mut acc = Accumulator258::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.113_f32 + y.sin();
        let b = y * 1.229_f32 - x.cos();
        let mut acc = Accumulator258::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.275_f32 + y.sin();
        let b = y * 7.605_f32 - x.cos();
        let mut acc = Accumulator258::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.86_f32 + y.sin();
        let b = y * 0.952_f32 - x.cos();
        let mut acc = Accumulator258::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.479_f32 + y.sin();
        let b = y * 7.23_f32 - x.cos();
        let mut acc = Accumulator258::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.444_f32 + y.sin();
        let b = y * 2.399_f32 - x.cos();
        let mut acc = Accumulator258::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.73_f32 + y.sin();
        let b = y * 7.085_f32 - x.cos();
        let mut acc = Accumulator258::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_258(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_258() -> f32 {
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
        total += (dep_touch_258(total as u64) % 997) as f32;
        total
    }
}

pub mod m259 {
    use super::*;

    pub struct Accumulator259<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator259<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.869_f32 + y.sin();
        let b = y * 1.615_f32 - x.cos();
        let mut acc = Accumulator259::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.249_f32 + y.sin();
        let b = y * 6.437_f32 - x.cos();
        let mut acc = Accumulator259::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.685_f32 + y.sin();
        let b = y * 3.825_f32 - x.cos();
        let mut acc = Accumulator259::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.138_f32 + y.sin();
        let b = y * 5.438_f32 - x.cos();
        let mut acc = Accumulator259::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.125_f32 + y.sin();
        let b = y * 3.364_f32 - x.cos();
        let mut acc = Accumulator259::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.877_f32 + y.sin();
        let b = y * 6.524_f32 - x.cos();
        let mut acc = Accumulator259::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.884_f32 + y.sin();
        let b = y * 1.284_f32 - x.cos();
        let mut acc = Accumulator259::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.205_f32 + y.sin();
        let b = y * 4.488_f32 - x.cos();
        let mut acc = Accumulator259::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.186_f32 + y.sin();
        let b = y * 1.36_f32 - x.cos();
        let mut acc = Accumulator259::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.885_f32 + y.sin();
        let b = y * 4.442_f32 - x.cos();
        let mut acc = Accumulator259::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.449_f32 + y.sin();
        let b = y * 8.414_f32 - x.cos();
        let mut acc = Accumulator259::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.621_f32 + y.sin();
        let b = y * 1.582_f32 - x.cos();
        let mut acc = Accumulator259::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.812_f32 + y.sin();
        let b = y * 6.219_f32 - x.cos();
        let mut acc = Accumulator259::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.124_f32 + y.sin();
        let b = y * 6.16_f32 - x.cos();
        let mut acc = Accumulator259::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.493_f32 + y.sin();
        let b = y * 1.055_f32 - x.cos();
        let mut acc = Accumulator259::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.183_f32 + y.sin();
        let b = y * 6.042_f32 - x.cos();
        let mut acc = Accumulator259::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.644_f32 + y.sin();
        let b = y * 4.885_f32 - x.cos();
        let mut acc = Accumulator259::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.797_f32 + y.sin();
        let b = y * 2.814_f32 - x.cos();
        let mut acc = Accumulator259::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.968_f32 + y.sin();
        let b = y * 7.777_f32 - x.cos();
        let mut acc = Accumulator259::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.992_f32 + y.sin();
        let b = y * 2.063_f32 - x.cos();
        let mut acc = Accumulator259::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_259(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_259() -> f32 {
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
        total += (dep_touch_259(total as u64) % 997) as f32;
        total
    }
}

pub mod m260 {
    use super::*;

    pub struct Accumulator260<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator260<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.488_f32 + y.sin();
        let b = y * 3.147_f32 - x.cos();
        let mut acc = Accumulator260::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.341_f32 + y.sin();
        let b = y * 8.211_f32 - x.cos();
        let mut acc = Accumulator260::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.844_f32 + y.sin();
        let b = y * 8.659_f32 - x.cos();
        let mut acc = Accumulator260::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.221_f32 + y.sin();
        let b = y * 1.857_f32 - x.cos();
        let mut acc = Accumulator260::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.717_f32 + y.sin();
        let b = y * 4.969_f32 - x.cos();
        let mut acc = Accumulator260::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.029_f32 + y.sin();
        let b = y * 7.736_f32 - x.cos();
        let mut acc = Accumulator260::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.038_f32 + y.sin();
        let b = y * 3.681_f32 - x.cos();
        let mut acc = Accumulator260::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.843_f32 + y.sin();
        let b = y * 7.711_f32 - x.cos();
        let mut acc = Accumulator260::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.066_f32 + y.sin();
        let b = y * 1.349_f32 - x.cos();
        let mut acc = Accumulator260::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.219_f32 + y.sin();
        let b = y * 1.366_f32 - x.cos();
        let mut acc = Accumulator260::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.837_f32 + y.sin();
        let b = y * 8.232_f32 - x.cos();
        let mut acc = Accumulator260::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.045_f32 + y.sin();
        let b = y * 3.586_f32 - x.cos();
        let mut acc = Accumulator260::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.38_f32 + y.sin();
        let b = y * 4.977_f32 - x.cos();
        let mut acc = Accumulator260::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.341_f32 + y.sin();
        let b = y * 5.951_f32 - x.cos();
        let mut acc = Accumulator260::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.658_f32 + y.sin();
        let b = y * 8.634_f32 - x.cos();
        let mut acc = Accumulator260::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.007_f32 + y.sin();
        let b = y * 4.669_f32 - x.cos();
        let mut acc = Accumulator260::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.389_f32 + y.sin();
        let b = y * 7.196_f32 - x.cos();
        let mut acc = Accumulator260::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.696_f32 + y.sin();
        let b = y * 6.212_f32 - x.cos();
        let mut acc = Accumulator260::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.866_f32 + y.sin();
        let b = y * 4.967_f32 - x.cos();
        let mut acc = Accumulator260::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.007_f32 + y.sin();
        let b = y * 0.79_f32 - x.cos();
        let mut acc = Accumulator260::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_260(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m260-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_260() -> f32 {
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
        total += (dep_touch_260(total as u64) % 997) as f32;
        total
    }
}

pub mod m261 {
    use super::*;

    pub struct Accumulator261<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator261<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.108_f32 + y.sin();
        let b = y * 0.17_f32 - x.cos();
        let mut acc = Accumulator261::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.844_f32 + y.sin();
        let b = y * 6.398_f32 - x.cos();
        let mut acc = Accumulator261::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.695_f32 + y.sin();
        let b = y * 5.403_f32 - x.cos();
        let mut acc = Accumulator261::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.392_f32 + y.sin();
        let b = y * 1.05_f32 - x.cos();
        let mut acc = Accumulator261::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.277_f32 + y.sin();
        let b = y * 8.006_f32 - x.cos();
        let mut acc = Accumulator261::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.443_f32 + y.sin();
        let b = y * 2.294_f32 - x.cos();
        let mut acc = Accumulator261::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.517_f32 + y.sin();
        let b = y * 6.661_f32 - x.cos();
        let mut acc = Accumulator261::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.579_f32 + y.sin();
        let b = y * 3.092_f32 - x.cos();
        let mut acc = Accumulator261::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.721_f32 + y.sin();
        let b = y * 5.892_f32 - x.cos();
        let mut acc = Accumulator261::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.652_f32 + y.sin();
        let b = y * 9.144_f32 - x.cos();
        let mut acc = Accumulator261::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.653_f32 + y.sin();
        let b = y * 0.448_f32 - x.cos();
        let mut acc = Accumulator261::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.77_f32 + y.sin();
        let b = y * 5.439_f32 - x.cos();
        let mut acc = Accumulator261::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.501_f32 + y.sin();
        let b = y * 6.787_f32 - x.cos();
        let mut acc = Accumulator261::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.534_f32 + y.sin();
        let b = y * 9.287_f32 - x.cos();
        let mut acc = Accumulator261::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.486_f32 + y.sin();
        let b = y * 2.793_f32 - x.cos();
        let mut acc = Accumulator261::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.817_f32 + y.sin();
        let b = y * 4.353_f32 - x.cos();
        let mut acc = Accumulator261::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.095_f32 + y.sin();
        let b = y * 8.359_f32 - x.cos();
        let mut acc = Accumulator261::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.282_f32 + y.sin();
        let b = y * 1.327_f32 - x.cos();
        let mut acc = Accumulator261::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.347_f32 + y.sin();
        let b = y * 2.833_f32 - x.cos();
        let mut acc = Accumulator261::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.128_f32 + y.sin();
        let b = y * 9.878_f32 - x.cos();
        let mut acc = Accumulator261::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_261(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_261() -> f32 {
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
        total += (dep_touch_261(total as u64) % 997) as f32;
        total
    }
}

pub mod m262 {
    use super::*;

    pub struct Accumulator262<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator262<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.647_f32 + y.sin();
        let b = y * 0.334_f32 - x.cos();
        let mut acc = Accumulator262::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.156_f32 + y.sin();
        let b = y * 4.699_f32 - x.cos();
        let mut acc = Accumulator262::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.89_f32 + y.sin();
        let b = y * 8.889_f32 - x.cos();
        let mut acc = Accumulator262::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.524_f32 + y.sin();
        let b = y * 7.801_f32 - x.cos();
        let mut acc = Accumulator262::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.316_f32 + y.sin();
        let b = y * 7.989_f32 - x.cos();
        let mut acc = Accumulator262::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.893_f32 + y.sin();
        let b = y * 9.084_f32 - x.cos();
        let mut acc = Accumulator262::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.861_f32 + y.sin();
        let b = y * 1.422_f32 - x.cos();
        let mut acc = Accumulator262::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.676_f32 + y.sin();
        let b = y * 5.1_f32 - x.cos();
        let mut acc = Accumulator262::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.151_f32 + y.sin();
        let b = y * 1.59_f32 - x.cos();
        let mut acc = Accumulator262::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.09_f32 + y.sin();
        let b = y * 7.879_f32 - x.cos();
        let mut acc = Accumulator262::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.187_f32 + y.sin();
        let b = y * 2.831_f32 - x.cos();
        let mut acc = Accumulator262::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.762_f32 + y.sin();
        let b = y * 9.368_f32 - x.cos();
        let mut acc = Accumulator262::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.108_f32 + y.sin();
        let b = y * 4.315_f32 - x.cos();
        let mut acc = Accumulator262::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.979_f32 + y.sin();
        let b = y * 3.845_f32 - x.cos();
        let mut acc = Accumulator262::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.882_f32 + y.sin();
        let b = y * 4.046_f32 - x.cos();
        let mut acc = Accumulator262::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.542_f32 + y.sin();
        let b = y * 1.233_f32 - x.cos();
        let mut acc = Accumulator262::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.707_f32 + y.sin();
        let b = y * 9.153_f32 - x.cos();
        let mut acc = Accumulator262::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.287_f32 + y.sin();
        let b = y * 8.257_f32 - x.cos();
        let mut acc = Accumulator262::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.828_f32 + y.sin();
        let b = y * 7.537_f32 - x.cos();
        let mut acc = Accumulator262::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.468_f32 + y.sin();
        let b = y * 5.149_f32 - x.cos();
        let mut acc = Accumulator262::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_262(seed: u64) -> u64 {
        let re = Regex::new(r"m262-(\d+)").unwrap();
        let hay = format!("m262-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_262() -> f32 {
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
        total += (dep_touch_262(total as u64) % 997) as f32;
        total
    }
}

pub mod m263 {
    use super::*;

    pub struct Accumulator263<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator263<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.184_f32 + y.sin();
        let b = y * 4.28_f32 - x.cos();
        let mut acc = Accumulator263::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.482_f32 + y.sin();
        let b = y * 8.178_f32 - x.cos();
        let mut acc = Accumulator263::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.455_f32 + y.sin();
        let b = y * 6.926_f32 - x.cos();
        let mut acc = Accumulator263::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.551_f32 + y.sin();
        let b = y * 4.899_f32 - x.cos();
        let mut acc = Accumulator263::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.301_f32 + y.sin();
        let b = y * 4.697_f32 - x.cos();
        let mut acc = Accumulator263::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.144_f32 + y.sin();
        let b = y * 2.837_f32 - x.cos();
        let mut acc = Accumulator263::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.205_f32 + y.sin();
        let b = y * 7.985_f32 - x.cos();
        let mut acc = Accumulator263::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.965_f32 + y.sin();
        let b = y * 2.628_f32 - x.cos();
        let mut acc = Accumulator263::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.954_f32 + y.sin();
        let b = y * 5.314_f32 - x.cos();
        let mut acc = Accumulator263::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.176_f32 + y.sin();
        let b = y * 0.257_f32 - x.cos();
        let mut acc = Accumulator263::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.725_f32 + y.sin();
        let b = y * 0.567_f32 - x.cos();
        let mut acc = Accumulator263::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.717_f32 + y.sin();
        let b = y * 3.285_f32 - x.cos();
        let mut acc = Accumulator263::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.949_f32 + y.sin();
        let b = y * 7.622_f32 - x.cos();
        let mut acc = Accumulator263::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.52_f32 + y.sin();
        let b = y * 4.281_f32 - x.cos();
        let mut acc = Accumulator263::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.804_f32 + y.sin();
        let b = y * 6.976_f32 - x.cos();
        let mut acc = Accumulator263::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.033_f32 + y.sin();
        let b = y * 5.189_f32 - x.cos();
        let mut acc = Accumulator263::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.508_f32 + y.sin();
        let b = y * 2.475_f32 - x.cos();
        let mut acc = Accumulator263::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.864_f32 + y.sin();
        let b = y * 1.127_f32 - x.cos();
        let mut acc = Accumulator263::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.299_f32 + y.sin();
        let b = y * 1.471_f32 - x.cos();
        let mut acc = Accumulator263::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.846_f32 + y.sin();
        let b = y * 8.941_f32 - x.cos();
        let mut acc = Accumulator263::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_263(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_263() -> f32 {
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
        total += (dep_touch_263(total as u64) % 997) as f32;
        total
    }
}

pub mod m264 {
    use super::*;

    pub struct Accumulator264<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator264<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.969_f32 + y.sin();
        let b = y * 0.4_f32 - x.cos();
        let mut acc = Accumulator264::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.443_f32 + y.sin();
        let b = y * 4.716_f32 - x.cos();
        let mut acc = Accumulator264::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.368_f32 + y.sin();
        let b = y * 3.113_f32 - x.cos();
        let mut acc = Accumulator264::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.816_f32 + y.sin();
        let b = y * 9.537_f32 - x.cos();
        let mut acc = Accumulator264::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.797_f32 + y.sin();
        let b = y * 4.04_f32 - x.cos();
        let mut acc = Accumulator264::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.069_f32 + y.sin();
        let b = y * 3.522_f32 - x.cos();
        let mut acc = Accumulator264::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.251_f32 + y.sin();
        let b = y * 5.969_f32 - x.cos();
        let mut acc = Accumulator264::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.014_f32 + y.sin();
        let b = y * 7.938_f32 - x.cos();
        let mut acc = Accumulator264::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.148_f32 + y.sin();
        let b = y * 1.637_f32 - x.cos();
        let mut acc = Accumulator264::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.264_f32 + y.sin();
        let b = y * 0.99_f32 - x.cos();
        let mut acc = Accumulator264::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.799_f32 + y.sin();
        let b = y * 3.986_f32 - x.cos();
        let mut acc = Accumulator264::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.416_f32 + y.sin();
        let b = y * 6.514_f32 - x.cos();
        let mut acc = Accumulator264::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.669_f32 + y.sin();
        let b = y * 1.414_f32 - x.cos();
        let mut acc = Accumulator264::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.869_f32 + y.sin();
        let b = y * 2.671_f32 - x.cos();
        let mut acc = Accumulator264::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.456_f32 + y.sin();
        let b = y * 7.68_f32 - x.cos();
        let mut acc = Accumulator264::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.601_f32 + y.sin();
        let b = y * 8.427_f32 - x.cos();
        let mut acc = Accumulator264::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.718_f32 + y.sin();
        let b = y * 2.804_f32 - x.cos();
        let mut acc = Accumulator264::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.776_f32 + y.sin();
        let b = y * 3.55_f32 - x.cos();
        let mut acc = Accumulator264::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.354_f32 + y.sin();
        let b = y * 3.861_f32 - x.cos();
        let mut acc = Accumulator264::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.693_f32 + y.sin();
        let b = y * 4.061_f32 - x.cos();
        let mut acc = Accumulator264::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_264(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(264u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_264() -> f32 {
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
        total += (dep_touch_264(total as u64) % 997) as f32;
        total
    }
}

pub mod m265 {
    use super::*;

    pub struct Accumulator265<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator265<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.576_f32 + y.sin();
        let b = y * 5.206_f32 - x.cos();
        let mut acc = Accumulator265::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.993_f32 + y.sin();
        let b = y * 8.592_f32 - x.cos();
        let mut acc = Accumulator265::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.685_f32 + y.sin();
        let b = y * 7.052_f32 - x.cos();
        let mut acc = Accumulator265::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.486_f32 + y.sin();
        let b = y * 3.566_f32 - x.cos();
        let mut acc = Accumulator265::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.501_f32 + y.sin();
        let b = y * 5.045_f32 - x.cos();
        let mut acc = Accumulator265::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.218_f32 + y.sin();
        let b = y * 7.156_f32 - x.cos();
        let mut acc = Accumulator265::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.99_f32 + y.sin();
        let b = y * 6.948_f32 - x.cos();
        let mut acc = Accumulator265::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.365_f32 + y.sin();
        let b = y * 4.597_f32 - x.cos();
        let mut acc = Accumulator265::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.016_f32 + y.sin();
        let b = y * 2.169_f32 - x.cos();
        let mut acc = Accumulator265::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.862_f32 + y.sin();
        let b = y * 2.152_f32 - x.cos();
        let mut acc = Accumulator265::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.761_f32 + y.sin();
        let b = y * 5.31_f32 - x.cos();
        let mut acc = Accumulator265::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.778_f32 + y.sin();
        let b = y * 6.47_f32 - x.cos();
        let mut acc = Accumulator265::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.911_f32 + y.sin();
        let b = y * 4.315_f32 - x.cos();
        let mut acc = Accumulator265::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.452_f32 + y.sin();
        let b = y * 6.914_f32 - x.cos();
        let mut acc = Accumulator265::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.151_f32 + y.sin();
        let b = y * 8.096_f32 - x.cos();
        let mut acc = Accumulator265::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.324_f32 + y.sin();
        let b = y * 4.07_f32 - x.cos();
        let mut acc = Accumulator265::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.735_f32 + y.sin();
        let b = y * 1.044_f32 - x.cos();
        let mut acc = Accumulator265::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.268_f32 + y.sin();
        let b = y * 1.74_f32 - x.cos();
        let mut acc = Accumulator265::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.987_f32 + y.sin();
        let b = y * 0.14_f32 - x.cos();
        let mut acc = Accumulator265::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.565_f32 + y.sin();
        let b = y * 6.696_f32 - x.cos();
        let mut acc = Accumulator265::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_265(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_265() -> f32 {
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
        total += (dep_touch_265(total as u64) % 997) as f32;
        total
    }
}

pub mod m266 {
    use super::*;

    pub struct Accumulator266<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator266<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.541_f32 + y.sin();
        let b = y * 1.525_f32 - x.cos();
        let mut acc = Accumulator266::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.549_f32 + y.sin();
        let b = y * 5.244_f32 - x.cos();
        let mut acc = Accumulator266::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.676_f32 + y.sin();
        let b = y * 0.111_f32 - x.cos();
        let mut acc = Accumulator266::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.416_f32 + y.sin();
        let b = y * 6.373_f32 - x.cos();
        let mut acc = Accumulator266::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.401_f32 + y.sin();
        let b = y * 3.042_f32 - x.cos();
        let mut acc = Accumulator266::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.438_f32 + y.sin();
        let b = y * 8.361_f32 - x.cos();
        let mut acc = Accumulator266::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.282_f32 + y.sin();
        let b = y * 8.038_f32 - x.cos();
        let mut acc = Accumulator266::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.574_f32 + y.sin();
        let b = y * 4.464_f32 - x.cos();
        let mut acc = Accumulator266::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.574_f32 + y.sin();
        let b = y * 5.131_f32 - x.cos();
        let mut acc = Accumulator266::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.408_f32 + y.sin();
        let b = y * 3.572_f32 - x.cos();
        let mut acc = Accumulator266::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.711_f32 + y.sin();
        let b = y * 5.667_f32 - x.cos();
        let mut acc = Accumulator266::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.524_f32 + y.sin();
        let b = y * 1.608_f32 - x.cos();
        let mut acc = Accumulator266::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.39_f32 + y.sin();
        let b = y * 0.941_f32 - x.cos();
        let mut acc = Accumulator266::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.54_f32 + y.sin();
        let b = y * 6.073_f32 - x.cos();
        let mut acc = Accumulator266::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.332_f32 + y.sin();
        let b = y * 1.463_f32 - x.cos();
        let mut acc = Accumulator266::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.839_f32 + y.sin();
        let b = y * 7.288_f32 - x.cos();
        let mut acc = Accumulator266::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.52_f32 + y.sin();
        let b = y * 8.271_f32 - x.cos();
        let mut acc = Accumulator266::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.77_f32 + y.sin();
        let b = y * 5.473_f32 - x.cos();
        let mut acc = Accumulator266::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.662_f32 + y.sin();
        let b = y * 9.273_f32 - x.cos();
        let mut acc = Accumulator266::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.529_f32 + y.sin();
        let b = y * 5.893_f32 - x.cos();
        let mut acc = Accumulator266::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_266(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_266() -> f32 {
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
        total += (dep_touch_266(total as u64) % 997) as f32;
        total
    }
}

pub mod m267 {
    use super::*;

    pub struct Accumulator267<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator267<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.858_f32 + y.sin();
        let b = y * 1.674_f32 - x.cos();
        let mut acc = Accumulator267::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.032_f32 + y.sin();
        let b = y * 3.088_f32 - x.cos();
        let mut acc = Accumulator267::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.44_f32 + y.sin();
        let b = y * 9.868_f32 - x.cos();
        let mut acc = Accumulator267::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.244_f32 + y.sin();
        let b = y * 2.841_f32 - x.cos();
        let mut acc = Accumulator267::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.264_f32 + y.sin();
        let b = y * 6.439_f32 - x.cos();
        let mut acc = Accumulator267::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.158_f32 + y.sin();
        let b = y * 4.109_f32 - x.cos();
        let mut acc = Accumulator267::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.971_f32 + y.sin();
        let b = y * 7.128_f32 - x.cos();
        let mut acc = Accumulator267::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.133_f32 + y.sin();
        let b = y * 3.409_f32 - x.cos();
        let mut acc = Accumulator267::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.574_f32 + y.sin();
        let b = y * 7.554_f32 - x.cos();
        let mut acc = Accumulator267::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.15_f32 + y.sin();
        let b = y * 8.088_f32 - x.cos();
        let mut acc = Accumulator267::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.647_f32 + y.sin();
        let b = y * 4.715_f32 - x.cos();
        let mut acc = Accumulator267::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.381_f32 + y.sin();
        let b = y * 7.423_f32 - x.cos();
        let mut acc = Accumulator267::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.692_f32 + y.sin();
        let b = y * 6.604_f32 - x.cos();
        let mut acc = Accumulator267::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.261_f32 + y.sin();
        let b = y * 1.372_f32 - x.cos();
        let mut acc = Accumulator267::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.672_f32 + y.sin();
        let b = y * 8.023_f32 - x.cos();
        let mut acc = Accumulator267::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.001_f32 + y.sin();
        let b = y * 2.488_f32 - x.cos();
        let mut acc = Accumulator267::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.973_f32 + y.sin();
        let b = y * 5.777_f32 - x.cos();
        let mut acc = Accumulator267::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.335_f32 + y.sin();
        let b = y * 8.125_f32 - x.cos();
        let mut acc = Accumulator267::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.504_f32 + y.sin();
        let b = y * 1.924_f32 - x.cos();
        let mut acc = Accumulator267::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.255_f32 + y.sin();
        let b = y * 9.176_f32 - x.cos();
        let mut acc = Accumulator267::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_267(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m267-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_267() -> f32 {
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
        total += (dep_touch_267(total as u64) % 997) as f32;
        total
    }
}

pub mod m268 {
    use super::*;

    pub struct Accumulator268<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator268<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.48_f32 + y.sin();
        let b = y * 6.356_f32 - x.cos();
        let mut acc = Accumulator268::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.122_f32 + y.sin();
        let b = y * 8.544_f32 - x.cos();
        let mut acc = Accumulator268::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.867_f32 + y.sin();
        let b = y * 2.836_f32 - x.cos();
        let mut acc = Accumulator268::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.009_f32 + y.sin();
        let b = y * 5.73_f32 - x.cos();
        let mut acc = Accumulator268::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.682_f32 + y.sin();
        let b = y * 9.484_f32 - x.cos();
        let mut acc = Accumulator268::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.043_f32 + y.sin();
        let b = y * 8.456_f32 - x.cos();
        let mut acc = Accumulator268::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.658_f32 + y.sin();
        let b = y * 5.091_f32 - x.cos();
        let mut acc = Accumulator268::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.314_f32 + y.sin();
        let b = y * 2.661_f32 - x.cos();
        let mut acc = Accumulator268::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.587_f32 + y.sin();
        let b = y * 8.178_f32 - x.cos();
        let mut acc = Accumulator268::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.188_f32 + y.sin();
        let b = y * 2.56_f32 - x.cos();
        let mut acc = Accumulator268::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.51_f32 + y.sin();
        let b = y * 0.458_f32 - x.cos();
        let mut acc = Accumulator268::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.278_f32 + y.sin();
        let b = y * 9.818_f32 - x.cos();
        let mut acc = Accumulator268::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.295_f32 + y.sin();
        let b = y * 6.863_f32 - x.cos();
        let mut acc = Accumulator268::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.186_f32 + y.sin();
        let b = y * 1.492_f32 - x.cos();
        let mut acc = Accumulator268::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.165_f32 + y.sin();
        let b = y * 8.423_f32 - x.cos();
        let mut acc = Accumulator268::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.554_f32 + y.sin();
        let b = y * 3.668_f32 - x.cos();
        let mut acc = Accumulator268::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.462_f32 + y.sin();
        let b = y * 2.841_f32 - x.cos();
        let mut acc = Accumulator268::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.24_f32 + y.sin();
        let b = y * 2.11_f32 - x.cos();
        let mut acc = Accumulator268::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.966_f32 + y.sin();
        let b = y * 7.721_f32 - x.cos();
        let mut acc = Accumulator268::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.096_f32 + y.sin();
        let b = y * 1.859_f32 - x.cos();
        let mut acc = Accumulator268::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_268(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_268() -> f32 {
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
        total += (dep_touch_268(total as u64) % 997) as f32;
        total
    }
}

pub mod m269 {
    use super::*;

    pub struct Accumulator269<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator269<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.825_f32 + y.sin();
        let b = y * 8.489_f32 - x.cos();
        let mut acc = Accumulator269::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.269_f32 + y.sin();
        let b = y * 9.284_f32 - x.cos();
        let mut acc = Accumulator269::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.172_f32 + y.sin();
        let b = y * 4.718_f32 - x.cos();
        let mut acc = Accumulator269::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.571_f32 + y.sin();
        let b = y * 5.997_f32 - x.cos();
        let mut acc = Accumulator269::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.341_f32 + y.sin();
        let b = y * 9.625_f32 - x.cos();
        let mut acc = Accumulator269::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.931_f32 + y.sin();
        let b = y * 6.258_f32 - x.cos();
        let mut acc = Accumulator269::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.308_f32 + y.sin();
        let b = y * 7.602_f32 - x.cos();
        let mut acc = Accumulator269::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.227_f32 + y.sin();
        let b = y * 1.323_f32 - x.cos();
        let mut acc = Accumulator269::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.221_f32 + y.sin();
        let b = y * 4.838_f32 - x.cos();
        let mut acc = Accumulator269::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.638_f32 + y.sin();
        let b = y * 9.787_f32 - x.cos();
        let mut acc = Accumulator269::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.565_f32 + y.sin();
        let b = y * 3.983_f32 - x.cos();
        let mut acc = Accumulator269::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.336_f32 + y.sin();
        let b = y * 7.914_f32 - x.cos();
        let mut acc = Accumulator269::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.876_f32 + y.sin();
        let b = y * 3.574_f32 - x.cos();
        let mut acc = Accumulator269::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.673_f32 + y.sin();
        let b = y * 6.45_f32 - x.cos();
        let mut acc = Accumulator269::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.567_f32 + y.sin();
        let b = y * 1.125_f32 - x.cos();
        let mut acc = Accumulator269::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.15_f32 + y.sin();
        let b = y * 8.604_f32 - x.cos();
        let mut acc = Accumulator269::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.687_f32 + y.sin();
        let b = y * 9.629_f32 - x.cos();
        let mut acc = Accumulator269::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.075_f32 + y.sin();
        let b = y * 1.486_f32 - x.cos();
        let mut acc = Accumulator269::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.794_f32 + y.sin();
        let b = y * 1.885_f32 - x.cos();
        let mut acc = Accumulator269::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.014_f32 + y.sin();
        let b = y * 4.425_f32 - x.cos();
        let mut acc = Accumulator269::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_269(seed: u64) -> u64 {
        let re = Regex::new(r"m269-(\d+)").unwrap();
        let hay = format!("m269-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_269() -> f32 {
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
        total += (dep_touch_269(total as u64) % 997) as f32;
        total
    }
}

pub mod m270 {
    use super::*;

    pub struct Accumulator270<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator270<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.238_f32 + y.sin();
        let b = y * 5.014_f32 - x.cos();
        let mut acc = Accumulator270::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.455_f32 + y.sin();
        let b = y * 7.492_f32 - x.cos();
        let mut acc = Accumulator270::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.237_f32 + y.sin();
        let b = y * 3.21_f32 - x.cos();
        let mut acc = Accumulator270::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.251_f32 + y.sin();
        let b = y * 1.446_f32 - x.cos();
        let mut acc = Accumulator270::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.614_f32 + y.sin();
        let b = y * 0.206_f32 - x.cos();
        let mut acc = Accumulator270::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.647_f32 + y.sin();
        let b = y * 9.189_f32 - x.cos();
        let mut acc = Accumulator270::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.472_f32 + y.sin();
        let b = y * 2.812_f32 - x.cos();
        let mut acc = Accumulator270::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.326_f32 + y.sin();
        let b = y * 9.489_f32 - x.cos();
        let mut acc = Accumulator270::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.936_f32 + y.sin();
        let b = y * 4.81_f32 - x.cos();
        let mut acc = Accumulator270::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.462_f32 + y.sin();
        let b = y * 8.046_f32 - x.cos();
        let mut acc = Accumulator270::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.169_f32 + y.sin();
        let b = y * 3.171_f32 - x.cos();
        let mut acc = Accumulator270::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.934_f32 + y.sin();
        let b = y * 2.239_f32 - x.cos();
        let mut acc = Accumulator270::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.247_f32 + y.sin();
        let b = y * 4.339_f32 - x.cos();
        let mut acc = Accumulator270::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.655_f32 + y.sin();
        let b = y * 4.104_f32 - x.cos();
        let mut acc = Accumulator270::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.742_f32 + y.sin();
        let b = y * 8.763_f32 - x.cos();
        let mut acc = Accumulator270::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.577_f32 + y.sin();
        let b = y * 4.062_f32 - x.cos();
        let mut acc = Accumulator270::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.836_f32 + y.sin();
        let b = y * 3.732_f32 - x.cos();
        let mut acc = Accumulator270::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.353_f32 + y.sin();
        let b = y * 0.583_f32 - x.cos();
        let mut acc = Accumulator270::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.703_f32 + y.sin();
        let b = y * 6.009_f32 - x.cos();
        let mut acc = Accumulator270::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.249_f32 + y.sin();
        let b = y * 3.449_f32 - x.cos();
        let mut acc = Accumulator270::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_270(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_270() -> f32 {
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
        total += (dep_touch_270(total as u64) % 997) as f32;
        total
    }
}

pub mod m271 {
    use super::*;

    pub struct Accumulator271<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator271<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.617_f32 + y.sin();
        let b = y * 0.139_f32 - x.cos();
        let mut acc = Accumulator271::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.268_f32 + y.sin();
        let b = y * 9.28_f32 - x.cos();
        let mut acc = Accumulator271::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.222_f32 + y.sin();
        let b = y * 7.678_f32 - x.cos();
        let mut acc = Accumulator271::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.257_f32 + y.sin();
        let b = y * 0.431_f32 - x.cos();
        let mut acc = Accumulator271::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.734_f32 + y.sin();
        let b = y * 0.83_f32 - x.cos();
        let mut acc = Accumulator271::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.711_f32 + y.sin();
        let b = y * 1.92_f32 - x.cos();
        let mut acc = Accumulator271::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.062_f32 + y.sin();
        let b = y * 6.685_f32 - x.cos();
        let mut acc = Accumulator271::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.833_f32 + y.sin();
        let b = y * 1.546_f32 - x.cos();
        let mut acc = Accumulator271::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.33_f32 + y.sin();
        let b = y * 5.075_f32 - x.cos();
        let mut acc = Accumulator271::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.692_f32 + y.sin();
        let b = y * 1.609_f32 - x.cos();
        let mut acc = Accumulator271::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.959_f32 + y.sin();
        let b = y * 2.438_f32 - x.cos();
        let mut acc = Accumulator271::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.207_f32 + y.sin();
        let b = y * 5.304_f32 - x.cos();
        let mut acc = Accumulator271::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.603_f32 + y.sin();
        let b = y * 5.114_f32 - x.cos();
        let mut acc = Accumulator271::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.528_f32 + y.sin();
        let b = y * 3.339_f32 - x.cos();
        let mut acc = Accumulator271::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.443_f32 + y.sin();
        let b = y * 7.464_f32 - x.cos();
        let mut acc = Accumulator271::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.368_f32 + y.sin();
        let b = y * 4.894_f32 - x.cos();
        let mut acc = Accumulator271::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.979_f32 + y.sin();
        let b = y * 8.752_f32 - x.cos();
        let mut acc = Accumulator271::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.144_f32 + y.sin();
        let b = y * 1.545_f32 - x.cos();
        let mut acc = Accumulator271::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.254_f32 + y.sin();
        let b = y * 9.072_f32 - x.cos();
        let mut acc = Accumulator271::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.718_f32 + y.sin();
        let b = y * 3.597_f32 - x.cos();
        let mut acc = Accumulator271::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_271(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(271u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_271() -> f32 {
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
        total += (dep_touch_271(total as u64) % 997) as f32;
        total
    }
}

pub mod m272 {
    use super::*;

    pub struct Accumulator272<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator272<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.113_f32 + y.sin();
        let b = y * 0.836_f32 - x.cos();
        let mut acc = Accumulator272::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.34_f32 + y.sin();
        let b = y * 8.71_f32 - x.cos();
        let mut acc = Accumulator272::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.916_f32 + y.sin();
        let b = y * 2.879_f32 - x.cos();
        let mut acc = Accumulator272::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.852_f32 + y.sin();
        let b = y * 0.687_f32 - x.cos();
        let mut acc = Accumulator272::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.254_f32 + y.sin();
        let b = y * 4.238_f32 - x.cos();
        let mut acc = Accumulator272::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.135_f32 + y.sin();
        let b = y * 3.927_f32 - x.cos();
        let mut acc = Accumulator272::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.563_f32 + y.sin();
        let b = y * 9.527_f32 - x.cos();
        let mut acc = Accumulator272::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.909_f32 + y.sin();
        let b = y * 7.741_f32 - x.cos();
        let mut acc = Accumulator272::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.348_f32 + y.sin();
        let b = y * 2.661_f32 - x.cos();
        let mut acc = Accumulator272::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.98_f32 + y.sin();
        let b = y * 2.492_f32 - x.cos();
        let mut acc = Accumulator272::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.309_f32 + y.sin();
        let b = y * 5.089_f32 - x.cos();
        let mut acc = Accumulator272::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.556_f32 + y.sin();
        let b = y * 4.425_f32 - x.cos();
        let mut acc = Accumulator272::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.325_f32 + y.sin();
        let b = y * 0.408_f32 - x.cos();
        let mut acc = Accumulator272::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.206_f32 + y.sin();
        let b = y * 6.08_f32 - x.cos();
        let mut acc = Accumulator272::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.183_f32 + y.sin();
        let b = y * 1.828_f32 - x.cos();
        let mut acc = Accumulator272::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.97_f32 + y.sin();
        let b = y * 2.22_f32 - x.cos();
        let mut acc = Accumulator272::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.482_f32 + y.sin();
        let b = y * 0.525_f32 - x.cos();
        let mut acc = Accumulator272::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.85_f32 + y.sin();
        let b = y * 9.893_f32 - x.cos();
        let mut acc = Accumulator272::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.032_f32 + y.sin();
        let b = y * 6.989_f32 - x.cos();
        let mut acc = Accumulator272::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.558_f32 + y.sin();
        let b = y * 5.121_f32 - x.cos();
        let mut acc = Accumulator272::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_272(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_272() -> f32 {
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
        total += (dep_touch_272(total as u64) % 997) as f32;
        total
    }
}

pub mod m273 {
    use super::*;

    pub struct Accumulator273<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator273<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.671_f32 + y.sin();
        let b = y * 0.653_f32 - x.cos();
        let mut acc = Accumulator273::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.496_f32 + y.sin();
        let b = y * 7.072_f32 - x.cos();
        let mut acc = Accumulator273::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.538_f32 + y.sin();
        let b = y * 0.766_f32 - x.cos();
        let mut acc = Accumulator273::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.85_f32 + y.sin();
        let b = y * 5.847_f32 - x.cos();
        let mut acc = Accumulator273::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.312_f32 + y.sin();
        let b = y * 0.841_f32 - x.cos();
        let mut acc = Accumulator273::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.676_f32 + y.sin();
        let b = y * 3.24_f32 - x.cos();
        let mut acc = Accumulator273::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.192_f32 + y.sin();
        let b = y * 2.415_f32 - x.cos();
        let mut acc = Accumulator273::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.423_f32 + y.sin();
        let b = y * 4.167_f32 - x.cos();
        let mut acc = Accumulator273::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.279_f32 + y.sin();
        let b = y * 5.432_f32 - x.cos();
        let mut acc = Accumulator273::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.943_f32 + y.sin();
        let b = y * 5.003_f32 - x.cos();
        let mut acc = Accumulator273::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.112_f32 + y.sin();
        let b = y * 4.203_f32 - x.cos();
        let mut acc = Accumulator273::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.703_f32 + y.sin();
        let b = y * 1.368_f32 - x.cos();
        let mut acc = Accumulator273::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.899_f32 + y.sin();
        let b = y * 4.885_f32 - x.cos();
        let mut acc = Accumulator273::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.527_f32 + y.sin();
        let b = y * 8.165_f32 - x.cos();
        let mut acc = Accumulator273::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.05_f32 + y.sin();
        let b = y * 3.682_f32 - x.cos();
        let mut acc = Accumulator273::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.096_f32 + y.sin();
        let b = y * 2.845_f32 - x.cos();
        let mut acc = Accumulator273::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.471_f32 + y.sin();
        let b = y * 2.667_f32 - x.cos();
        let mut acc = Accumulator273::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.409_f32 + y.sin();
        let b = y * 6.981_f32 - x.cos();
        let mut acc = Accumulator273::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.564_f32 + y.sin();
        let b = y * 0.266_f32 - x.cos();
        let mut acc = Accumulator273::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.362_f32 + y.sin();
        let b = y * 0.504_f32 - x.cos();
        let mut acc = Accumulator273::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_273(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_273() -> f32 {
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
        total += (dep_touch_273(total as u64) % 997) as f32;
        total
    }
}

pub mod m274 {
    use super::*;

    pub struct Accumulator274<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator274<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.132_f32 + y.sin();
        let b = y * 4.383_f32 - x.cos();
        let mut acc = Accumulator274::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.875_f32 + y.sin();
        let b = y * 2.278_f32 - x.cos();
        let mut acc = Accumulator274::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.455_f32 + y.sin();
        let b = y * 1.477_f32 - x.cos();
        let mut acc = Accumulator274::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.787_f32 + y.sin();
        let b = y * 1.968_f32 - x.cos();
        let mut acc = Accumulator274::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.678_f32 + y.sin();
        let b = y * 1.433_f32 - x.cos();
        let mut acc = Accumulator274::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.011_f32 + y.sin();
        let b = y * 5.277_f32 - x.cos();
        let mut acc = Accumulator274::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.788_f32 + y.sin();
        let b = y * 3.691_f32 - x.cos();
        let mut acc = Accumulator274::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.304_f32 + y.sin();
        let b = y * 8.218_f32 - x.cos();
        let mut acc = Accumulator274::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.447_f32 + y.sin();
        let b = y * 8.856_f32 - x.cos();
        let mut acc = Accumulator274::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.134_f32 + y.sin();
        let b = y * 9.373_f32 - x.cos();
        let mut acc = Accumulator274::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.207_f32 + y.sin();
        let b = y * 9.326_f32 - x.cos();
        let mut acc = Accumulator274::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.007_f32 + y.sin();
        let b = y * 2.15_f32 - x.cos();
        let mut acc = Accumulator274::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.71_f32 + y.sin();
        let b = y * 1.026_f32 - x.cos();
        let mut acc = Accumulator274::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.183_f32 + y.sin();
        let b = y * 7.99_f32 - x.cos();
        let mut acc = Accumulator274::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.426_f32 + y.sin();
        let b = y * 9.365_f32 - x.cos();
        let mut acc = Accumulator274::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.916_f32 + y.sin();
        let b = y * 7.526_f32 - x.cos();
        let mut acc = Accumulator274::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.229_f32 + y.sin();
        let b = y * 0.815_f32 - x.cos();
        let mut acc = Accumulator274::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.621_f32 + y.sin();
        let b = y * 2.368_f32 - x.cos();
        let mut acc = Accumulator274::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.865_f32 + y.sin();
        let b = y * 3.857_f32 - x.cos();
        let mut acc = Accumulator274::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.143_f32 + y.sin();
        let b = y * 3.273_f32 - x.cos();
        let mut acc = Accumulator274::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_274(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m274-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_274() -> f32 {
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
        total += (dep_touch_274(total as u64) % 997) as f32;
        total
    }
}

pub mod m275 {
    use super::*;

    pub struct Accumulator275<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator275<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.23_f32 + y.sin();
        let b = y * 1.988_f32 - x.cos();
        let mut acc = Accumulator275::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.649_f32 + y.sin();
        let b = y * 3.508_f32 - x.cos();
        let mut acc = Accumulator275::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.406_f32 + y.sin();
        let b = y * 8.284_f32 - x.cos();
        let mut acc = Accumulator275::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.677_f32 + y.sin();
        let b = y * 0.513_f32 - x.cos();
        let mut acc = Accumulator275::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.099_f32 + y.sin();
        let b = y * 4.532_f32 - x.cos();
        let mut acc = Accumulator275::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.783_f32 + y.sin();
        let b = y * 5.241_f32 - x.cos();
        let mut acc = Accumulator275::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.498_f32 + y.sin();
        let b = y * 4.214_f32 - x.cos();
        let mut acc = Accumulator275::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.776_f32 + y.sin();
        let b = y * 3.402_f32 - x.cos();
        let mut acc = Accumulator275::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.893_f32 + y.sin();
        let b = y * 6.139_f32 - x.cos();
        let mut acc = Accumulator275::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.04_f32 + y.sin();
        let b = y * 7.225_f32 - x.cos();
        let mut acc = Accumulator275::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.895_f32 + y.sin();
        let b = y * 7.57_f32 - x.cos();
        let mut acc = Accumulator275::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.284_f32 + y.sin();
        let b = y * 7.216_f32 - x.cos();
        let mut acc = Accumulator275::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.332_f32 + y.sin();
        let b = y * 9.112_f32 - x.cos();
        let mut acc = Accumulator275::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.252_f32 + y.sin();
        let b = y * 9.124_f32 - x.cos();
        let mut acc = Accumulator275::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.804_f32 + y.sin();
        let b = y * 1.913_f32 - x.cos();
        let mut acc = Accumulator275::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.46_f32 + y.sin();
        let b = y * 9.448_f32 - x.cos();
        let mut acc = Accumulator275::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.876_f32 + y.sin();
        let b = y * 8.756_f32 - x.cos();
        let mut acc = Accumulator275::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.756_f32 + y.sin();
        let b = y * 8.021_f32 - x.cos();
        let mut acc = Accumulator275::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.052_f32 + y.sin();
        let b = y * 9.464_f32 - x.cos();
        let mut acc = Accumulator275::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.28_f32 + y.sin();
        let b = y * 2.876_f32 - x.cos();
        let mut acc = Accumulator275::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_275(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_275() -> f32 {
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
        total += (dep_touch_275(total as u64) % 997) as f32;
        total
    }
}

pub mod m276 {
    use super::*;

    pub struct Accumulator276<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator276<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.977_f32 + y.sin();
        let b = y * 6.909_f32 - x.cos();
        let mut acc = Accumulator276::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.88_f32 + y.sin();
        let b = y * 7.144_f32 - x.cos();
        let mut acc = Accumulator276::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.503_f32 + y.sin();
        let b = y * 1.438_f32 - x.cos();
        let mut acc = Accumulator276::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.012_f32 + y.sin();
        let b = y * 8.891_f32 - x.cos();
        let mut acc = Accumulator276::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.687_f32 + y.sin();
        let b = y * 2.751_f32 - x.cos();
        let mut acc = Accumulator276::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.246_f32 + y.sin();
        let b = y * 3.497_f32 - x.cos();
        let mut acc = Accumulator276::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.339_f32 + y.sin();
        let b = y * 7.304_f32 - x.cos();
        let mut acc = Accumulator276::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.974_f32 + y.sin();
        let b = y * 4.753_f32 - x.cos();
        let mut acc = Accumulator276::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.054_f32 + y.sin();
        let b = y * 9.138_f32 - x.cos();
        let mut acc = Accumulator276::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.337_f32 + y.sin();
        let b = y * 7.179_f32 - x.cos();
        let mut acc = Accumulator276::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.135_f32 + y.sin();
        let b = y * 3.167_f32 - x.cos();
        let mut acc = Accumulator276::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.99_f32 + y.sin();
        let b = y * 2.391_f32 - x.cos();
        let mut acc = Accumulator276::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.892_f32 + y.sin();
        let b = y * 4.981_f32 - x.cos();
        let mut acc = Accumulator276::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.794_f32 + y.sin();
        let b = y * 6.863_f32 - x.cos();
        let mut acc = Accumulator276::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.779_f32 + y.sin();
        let b = y * 3.44_f32 - x.cos();
        let mut acc = Accumulator276::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.556_f32 + y.sin();
        let b = y * 9.626_f32 - x.cos();
        let mut acc = Accumulator276::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.302_f32 + y.sin();
        let b = y * 5.13_f32 - x.cos();
        let mut acc = Accumulator276::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.776_f32 + y.sin();
        let b = y * 5.816_f32 - x.cos();
        let mut acc = Accumulator276::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.087_f32 + y.sin();
        let b = y * 9.631_f32 - x.cos();
        let mut acc = Accumulator276::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.519_f32 + y.sin();
        let b = y * 4.603_f32 - x.cos();
        let mut acc = Accumulator276::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_276(seed: u64) -> u64 {
        let re = Regex::new(r"m276-(\d+)").unwrap();
        let hay = format!("m276-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_276() -> f32 {
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
        total += (dep_touch_276(total as u64) % 997) as f32;
        total
    }
}

pub mod m277 {
    use super::*;

    pub struct Accumulator277<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator277<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.694_f32 + y.sin();
        let b = y * 5.018_f32 - x.cos();
        let mut acc = Accumulator277::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.834_f32 + y.sin();
        let b = y * 3.866_f32 - x.cos();
        let mut acc = Accumulator277::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.371_f32 + y.sin();
        let b = y * 3.386_f32 - x.cos();
        let mut acc = Accumulator277::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.165_f32 + y.sin();
        let b = y * 3.247_f32 - x.cos();
        let mut acc = Accumulator277::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.564_f32 + y.sin();
        let b = y * 5.312_f32 - x.cos();
        let mut acc = Accumulator277::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.761_f32 + y.sin();
        let b = y * 9.76_f32 - x.cos();
        let mut acc = Accumulator277::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.181_f32 + y.sin();
        let b = y * 6.07_f32 - x.cos();
        let mut acc = Accumulator277::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.365_f32 + y.sin();
        let b = y * 3.284_f32 - x.cos();
        let mut acc = Accumulator277::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.689_f32 + y.sin();
        let b = y * 2.663_f32 - x.cos();
        let mut acc = Accumulator277::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.428_f32 + y.sin();
        let b = y * 7.008_f32 - x.cos();
        let mut acc = Accumulator277::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.465_f32 + y.sin();
        let b = y * 5.556_f32 - x.cos();
        let mut acc = Accumulator277::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.917_f32 + y.sin();
        let b = y * 6.073_f32 - x.cos();
        let mut acc = Accumulator277::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.04_f32 + y.sin();
        let b = y * 1.813_f32 - x.cos();
        let mut acc = Accumulator277::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.776_f32 + y.sin();
        let b = y * 4.272_f32 - x.cos();
        let mut acc = Accumulator277::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.383_f32 + y.sin();
        let b = y * 3.704_f32 - x.cos();
        let mut acc = Accumulator277::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.181_f32 + y.sin();
        let b = y * 6.35_f32 - x.cos();
        let mut acc = Accumulator277::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.609_f32 + y.sin();
        let b = y * 2.728_f32 - x.cos();
        let mut acc = Accumulator277::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.513_f32 + y.sin();
        let b = y * 5.034_f32 - x.cos();
        let mut acc = Accumulator277::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.895_f32 + y.sin();
        let b = y * 5.33_f32 - x.cos();
        let mut acc = Accumulator277::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.516_f32 + y.sin();
        let b = y * 2.203_f32 - x.cos();
        let mut acc = Accumulator277::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_277(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_277() -> f32 {
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
        total += (dep_touch_277(total as u64) % 997) as f32;
        total
    }
}

pub mod m278 {
    use super::*;

    pub struct Accumulator278<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator278<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.77_f32 + y.sin();
        let b = y * 8.911_f32 - x.cos();
        let mut acc = Accumulator278::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.688_f32 + y.sin();
        let b = y * 1.799_f32 - x.cos();
        let mut acc = Accumulator278::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.697_f32 + y.sin();
        let b = y * 3.71_f32 - x.cos();
        let mut acc = Accumulator278::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.722_f32 + y.sin();
        let b = y * 7.844_f32 - x.cos();
        let mut acc = Accumulator278::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.088_f32 + y.sin();
        let b = y * 3.512_f32 - x.cos();
        let mut acc = Accumulator278::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.653_f32 + y.sin();
        let b = y * 9.231_f32 - x.cos();
        let mut acc = Accumulator278::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.585_f32 + y.sin();
        let b = y * 4.461_f32 - x.cos();
        let mut acc = Accumulator278::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.143_f32 + y.sin();
        let b = y * 7.59_f32 - x.cos();
        let mut acc = Accumulator278::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.845_f32 + y.sin();
        let b = y * 2.366_f32 - x.cos();
        let mut acc = Accumulator278::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.854_f32 + y.sin();
        let b = y * 0.562_f32 - x.cos();
        let mut acc = Accumulator278::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.95_f32 + y.sin();
        let b = y * 2.18_f32 - x.cos();
        let mut acc = Accumulator278::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.645_f32 + y.sin();
        let b = y * 5.835_f32 - x.cos();
        let mut acc = Accumulator278::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.442_f32 + y.sin();
        let b = y * 2.855_f32 - x.cos();
        let mut acc = Accumulator278::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.878_f32 + y.sin();
        let b = y * 6.874_f32 - x.cos();
        let mut acc = Accumulator278::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.787_f32 + y.sin();
        let b = y * 6.049_f32 - x.cos();
        let mut acc = Accumulator278::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.859_f32 + y.sin();
        let b = y * 1.332_f32 - x.cos();
        let mut acc = Accumulator278::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.289_f32 + y.sin();
        let b = y * 8.731_f32 - x.cos();
        let mut acc = Accumulator278::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.062_f32 + y.sin();
        let b = y * 8.825_f32 - x.cos();
        let mut acc = Accumulator278::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.689_f32 + y.sin();
        let b = y * 2.643_f32 - x.cos();
        let mut acc = Accumulator278::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.913_f32 + y.sin();
        let b = y * 3.535_f32 - x.cos();
        let mut acc = Accumulator278::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_278(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(278u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_278() -> f32 {
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
        total += (dep_touch_278(total as u64) % 997) as f32;
        total
    }
}

pub mod m279 {
    use super::*;

    pub struct Accumulator279<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator279<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.491_f32 + y.sin();
        let b = y * 6.943_f32 - x.cos();
        let mut acc = Accumulator279::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.16_f32 + y.sin();
        let b = y * 3.662_f32 - x.cos();
        let mut acc = Accumulator279::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.857_f32 + y.sin();
        let b = y * 9.701_f32 - x.cos();
        let mut acc = Accumulator279::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.076_f32 + y.sin();
        let b = y * 3.339_f32 - x.cos();
        let mut acc = Accumulator279::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.922_f32 + y.sin();
        let b = y * 2.137_f32 - x.cos();
        let mut acc = Accumulator279::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.265_f32 + y.sin();
        let b = y * 7.858_f32 - x.cos();
        let mut acc = Accumulator279::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.912_f32 + y.sin();
        let b = y * 0.97_f32 - x.cos();
        let mut acc = Accumulator279::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.637_f32 + y.sin();
        let b = y * 8.135_f32 - x.cos();
        let mut acc = Accumulator279::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.251_f32 + y.sin();
        let b = y * 0.365_f32 - x.cos();
        let mut acc = Accumulator279::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.547_f32 + y.sin();
        let b = y * 2.542_f32 - x.cos();
        let mut acc = Accumulator279::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.173_f32 + y.sin();
        let b = y * 8.529_f32 - x.cos();
        let mut acc = Accumulator279::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.641_f32 + y.sin();
        let b = y * 4.937_f32 - x.cos();
        let mut acc = Accumulator279::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.436_f32 + y.sin();
        let b = y * 1.493_f32 - x.cos();
        let mut acc = Accumulator279::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.263_f32 + y.sin();
        let b = y * 3.805_f32 - x.cos();
        let mut acc = Accumulator279::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.079_f32 + y.sin();
        let b = y * 9.676_f32 - x.cos();
        let mut acc = Accumulator279::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.857_f32 + y.sin();
        let b = y * 8.741_f32 - x.cos();
        let mut acc = Accumulator279::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.023_f32 + y.sin();
        let b = y * 4.253_f32 - x.cos();
        let mut acc = Accumulator279::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.037_f32 + y.sin();
        let b = y * 9.112_f32 - x.cos();
        let mut acc = Accumulator279::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.463_f32 + y.sin();
        let b = y * 0.768_f32 - x.cos();
        let mut acc = Accumulator279::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.107_f32 + y.sin();
        let b = y * 6.406_f32 - x.cos();
        let mut acc = Accumulator279::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_279(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_279() -> f32 {
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
        total += (dep_touch_279(total as u64) % 997) as f32;
        total
    }
}

pub mod m280 {
    use super::*;

    pub struct Accumulator280<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator280<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.937_f32 + y.sin();
        let b = y * 8.734_f32 - x.cos();
        let mut acc = Accumulator280::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.875_f32 + y.sin();
        let b = y * 4.99_f32 - x.cos();
        let mut acc = Accumulator280::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.407_f32 + y.sin();
        let b = y * 9.023_f32 - x.cos();
        let mut acc = Accumulator280::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.17_f32 + y.sin();
        let b = y * 0.117_f32 - x.cos();
        let mut acc = Accumulator280::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.465_f32 + y.sin();
        let b = y * 7.583_f32 - x.cos();
        let mut acc = Accumulator280::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.221_f32 + y.sin();
        let b = y * 0.139_f32 - x.cos();
        let mut acc = Accumulator280::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.413_f32 + y.sin();
        let b = y * 2.658_f32 - x.cos();
        let mut acc = Accumulator280::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.752_f32 + y.sin();
        let b = y * 7.6_f32 - x.cos();
        let mut acc = Accumulator280::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.524_f32 + y.sin();
        let b = y * 7.994_f32 - x.cos();
        let mut acc = Accumulator280::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.72_f32 + y.sin();
        let b = y * 3.84_f32 - x.cos();
        let mut acc = Accumulator280::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.043_f32 + y.sin();
        let b = y * 0.252_f32 - x.cos();
        let mut acc = Accumulator280::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.535_f32 + y.sin();
        let b = y * 1.457_f32 - x.cos();
        let mut acc = Accumulator280::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.913_f32 + y.sin();
        let b = y * 3.893_f32 - x.cos();
        let mut acc = Accumulator280::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.911_f32 + y.sin();
        let b = y * 0.694_f32 - x.cos();
        let mut acc = Accumulator280::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.368_f32 + y.sin();
        let b = y * 6.713_f32 - x.cos();
        let mut acc = Accumulator280::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.58_f32 + y.sin();
        let b = y * 2.519_f32 - x.cos();
        let mut acc = Accumulator280::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.161_f32 + y.sin();
        let b = y * 2.457_f32 - x.cos();
        let mut acc = Accumulator280::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.818_f32 + y.sin();
        let b = y * 9.485_f32 - x.cos();
        let mut acc = Accumulator280::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.877_f32 + y.sin();
        let b = y * 2.994_f32 - x.cos();
        let mut acc = Accumulator280::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.883_f32 + y.sin();
        let b = y * 2.767_f32 - x.cos();
        let mut acc = Accumulator280::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_280(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_280() -> f32 {
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
        total += (dep_touch_280(total as u64) % 997) as f32;
        total
    }
}

pub mod m281 {
    use super::*;

    pub struct Accumulator281<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator281<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.047_f32 + y.sin();
        let b = y * 7.911_f32 - x.cos();
        let mut acc = Accumulator281::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.078_f32 + y.sin();
        let b = y * 4.688_f32 - x.cos();
        let mut acc = Accumulator281::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.443_f32 + y.sin();
        let b = y * 9.216_f32 - x.cos();
        let mut acc = Accumulator281::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.505_f32 + y.sin();
        let b = y * 6.485_f32 - x.cos();
        let mut acc = Accumulator281::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.097_f32 + y.sin();
        let b = y * 1.126_f32 - x.cos();
        let mut acc = Accumulator281::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.918_f32 + y.sin();
        let b = y * 6.717_f32 - x.cos();
        let mut acc = Accumulator281::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.088_f32 + y.sin();
        let b = y * 3.973_f32 - x.cos();
        let mut acc = Accumulator281::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.397_f32 + y.sin();
        let b = y * 9.244_f32 - x.cos();
        let mut acc = Accumulator281::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.663_f32 + y.sin();
        let b = y * 6.541_f32 - x.cos();
        let mut acc = Accumulator281::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.112_f32 + y.sin();
        let b = y * 6.002_f32 - x.cos();
        let mut acc = Accumulator281::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.042_f32 + y.sin();
        let b = y * 6.246_f32 - x.cos();
        let mut acc = Accumulator281::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.506_f32 + y.sin();
        let b = y * 7.493_f32 - x.cos();
        let mut acc = Accumulator281::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.202_f32 + y.sin();
        let b = y * 9.385_f32 - x.cos();
        let mut acc = Accumulator281::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.256_f32 + y.sin();
        let b = y * 1.007_f32 - x.cos();
        let mut acc = Accumulator281::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.419_f32 + y.sin();
        let b = y * 7.57_f32 - x.cos();
        let mut acc = Accumulator281::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.348_f32 + y.sin();
        let b = y * 6.217_f32 - x.cos();
        let mut acc = Accumulator281::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.427_f32 + y.sin();
        let b = y * 4.876_f32 - x.cos();
        let mut acc = Accumulator281::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.819_f32 + y.sin();
        let b = y * 3.521_f32 - x.cos();
        let mut acc = Accumulator281::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.861_f32 + y.sin();
        let b = y * 4.496_f32 - x.cos();
        let mut acc = Accumulator281::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.002_f32 + y.sin();
        let b = y * 6.494_f32 - x.cos();
        let mut acc = Accumulator281::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_281(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m281-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_281() -> f32 {
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
        total += (dep_touch_281(total as u64) % 997) as f32;
        total
    }
}

pub mod m282 {
    use super::*;

    pub struct Accumulator282<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator282<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.685_f32 + y.sin();
        let b = y * 4.825_f32 - x.cos();
        let mut acc = Accumulator282::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.579_f32 + y.sin();
        let b = y * 1.876_f32 - x.cos();
        let mut acc = Accumulator282::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.261_f32 + y.sin();
        let b = y * 4.564_f32 - x.cos();
        let mut acc = Accumulator282::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.22_f32 + y.sin();
        let b = y * 9.716_f32 - x.cos();
        let mut acc = Accumulator282::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.306_f32 + y.sin();
        let b = y * 1.316_f32 - x.cos();
        let mut acc = Accumulator282::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.598_f32 + y.sin();
        let b = y * 1.855_f32 - x.cos();
        let mut acc = Accumulator282::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.88_f32 + y.sin();
        let b = y * 9.734_f32 - x.cos();
        let mut acc = Accumulator282::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.041_f32 + y.sin();
        let b = y * 8.011_f32 - x.cos();
        let mut acc = Accumulator282::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.457_f32 + y.sin();
        let b = y * 8.916_f32 - x.cos();
        let mut acc = Accumulator282::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.994_f32 + y.sin();
        let b = y * 4.457_f32 - x.cos();
        let mut acc = Accumulator282::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.095_f32 + y.sin();
        let b = y * 2.513_f32 - x.cos();
        let mut acc = Accumulator282::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.406_f32 + y.sin();
        let b = y * 4.985_f32 - x.cos();
        let mut acc = Accumulator282::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.182_f32 + y.sin();
        let b = y * 4.322_f32 - x.cos();
        let mut acc = Accumulator282::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.857_f32 + y.sin();
        let b = y * 3.162_f32 - x.cos();
        let mut acc = Accumulator282::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.426_f32 + y.sin();
        let b = y * 0.917_f32 - x.cos();
        let mut acc = Accumulator282::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.227_f32 + y.sin();
        let b = y * 6.626_f32 - x.cos();
        let mut acc = Accumulator282::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.864_f32 + y.sin();
        let b = y * 4.275_f32 - x.cos();
        let mut acc = Accumulator282::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.555_f32 + y.sin();
        let b = y * 2.349_f32 - x.cos();
        let mut acc = Accumulator282::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.826_f32 + y.sin();
        let b = y * 4.264_f32 - x.cos();
        let mut acc = Accumulator282::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.966_f32 + y.sin();
        let b = y * 1.767_f32 - x.cos();
        let mut acc = Accumulator282::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_282(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_282() -> f32 {
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
        total += (dep_touch_282(total as u64) % 997) as f32;
        total
    }
}

pub mod m283 {
    use super::*;

    pub struct Accumulator283<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator283<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.271_f32 + y.sin();
        let b = y * 7.495_f32 - x.cos();
        let mut acc = Accumulator283::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.359_f32 + y.sin();
        let b = y * 2.193_f32 - x.cos();
        let mut acc = Accumulator283::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.16_f32 + y.sin();
        let b = y * 1.839_f32 - x.cos();
        let mut acc = Accumulator283::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.799_f32 + y.sin();
        let b = y * 5.784_f32 - x.cos();
        let mut acc = Accumulator283::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.473_f32 + y.sin();
        let b = y * 1.895_f32 - x.cos();
        let mut acc = Accumulator283::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.844_f32 + y.sin();
        let b = y * 8.09_f32 - x.cos();
        let mut acc = Accumulator283::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.09_f32 + y.sin();
        let b = y * 3.315_f32 - x.cos();
        let mut acc = Accumulator283::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.087_f32 + y.sin();
        let b = y * 6.681_f32 - x.cos();
        let mut acc = Accumulator283::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.797_f32 + y.sin();
        let b = y * 3.046_f32 - x.cos();
        let mut acc = Accumulator283::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.0_f32 + y.sin();
        let b = y * 2.453_f32 - x.cos();
        let mut acc = Accumulator283::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.729_f32 + y.sin();
        let b = y * 0.558_f32 - x.cos();
        let mut acc = Accumulator283::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.536_f32 + y.sin();
        let b = y * 4.313_f32 - x.cos();
        let mut acc = Accumulator283::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.335_f32 + y.sin();
        let b = y * 2.346_f32 - x.cos();
        let mut acc = Accumulator283::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.552_f32 + y.sin();
        let b = y * 5.083_f32 - x.cos();
        let mut acc = Accumulator283::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.181_f32 + y.sin();
        let b = y * 5.65_f32 - x.cos();
        let mut acc = Accumulator283::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.821_f32 + y.sin();
        let b = y * 3.466_f32 - x.cos();
        let mut acc = Accumulator283::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.699_f32 + y.sin();
        let b = y * 6.283_f32 - x.cos();
        let mut acc = Accumulator283::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.339_f32 + y.sin();
        let b = y * 5.87_f32 - x.cos();
        let mut acc = Accumulator283::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.208_f32 + y.sin();
        let b = y * 0.358_f32 - x.cos();
        let mut acc = Accumulator283::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.463_f32 + y.sin();
        let b = y * 7.388_f32 - x.cos();
        let mut acc = Accumulator283::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_283(seed: u64) -> u64 {
        let re = Regex::new(r"m283-(\d+)").unwrap();
        let hay = format!("m283-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_283() -> f32 {
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
        total += (dep_touch_283(total as u64) % 997) as f32;
        total
    }
}

pub mod m284 {
    use super::*;

    pub struct Accumulator284<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator284<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.195_f32 + y.sin();
        let b = y * 0.556_f32 - x.cos();
        let mut acc = Accumulator284::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.036_f32 + y.sin();
        let b = y * 5.791_f32 - x.cos();
        let mut acc = Accumulator284::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.844_f32 + y.sin();
        let b = y * 9.13_f32 - x.cos();
        let mut acc = Accumulator284::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.369_f32 + y.sin();
        let b = y * 9.418_f32 - x.cos();
        let mut acc = Accumulator284::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.71_f32 + y.sin();
        let b = y * 4.197_f32 - x.cos();
        let mut acc = Accumulator284::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.701_f32 + y.sin();
        let b = y * 0.458_f32 - x.cos();
        let mut acc = Accumulator284::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.63_f32 + y.sin();
        let b = y * 9.098_f32 - x.cos();
        let mut acc = Accumulator284::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.071_f32 + y.sin();
        let b = y * 8.176_f32 - x.cos();
        let mut acc = Accumulator284::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.076_f32 + y.sin();
        let b = y * 5.803_f32 - x.cos();
        let mut acc = Accumulator284::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.089_f32 + y.sin();
        let b = y * 4.944_f32 - x.cos();
        let mut acc = Accumulator284::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.578_f32 + y.sin();
        let b = y * 2.824_f32 - x.cos();
        let mut acc = Accumulator284::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.61_f32 + y.sin();
        let b = y * 1.344_f32 - x.cos();
        let mut acc = Accumulator284::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.361_f32 + y.sin();
        let b = y * 5.195_f32 - x.cos();
        let mut acc = Accumulator284::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.696_f32 + y.sin();
        let b = y * 9.143_f32 - x.cos();
        let mut acc = Accumulator284::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.512_f32 + y.sin();
        let b = y * 6.967_f32 - x.cos();
        let mut acc = Accumulator284::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.087_f32 + y.sin();
        let b = y * 6.033_f32 - x.cos();
        let mut acc = Accumulator284::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.4_f32 + y.sin();
        let b = y * 3.846_f32 - x.cos();
        let mut acc = Accumulator284::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.089_f32 + y.sin();
        let b = y * 0.221_f32 - x.cos();
        let mut acc = Accumulator284::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.798_f32 + y.sin();
        let b = y * 9.147_f32 - x.cos();
        let mut acc = Accumulator284::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.919_f32 + y.sin();
        let b = y * 3.417_f32 - x.cos();
        let mut acc = Accumulator284::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_284(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_284() -> f32 {
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
        total += (dep_touch_284(total as u64) % 997) as f32;
        total
    }
}

pub mod m285 {
    use super::*;

    pub struct Accumulator285<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator285<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.951_f32 + y.sin();
        let b = y * 8.964_f32 - x.cos();
        let mut acc = Accumulator285::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.637_f32 + y.sin();
        let b = y * 2.283_f32 - x.cos();
        let mut acc = Accumulator285::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.328_f32 + y.sin();
        let b = y * 4.967_f32 - x.cos();
        let mut acc = Accumulator285::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.775_f32 + y.sin();
        let b = y * 5.306_f32 - x.cos();
        let mut acc = Accumulator285::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.306_f32 + y.sin();
        let b = y * 0.72_f32 - x.cos();
        let mut acc = Accumulator285::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.531_f32 + y.sin();
        let b = y * 4.938_f32 - x.cos();
        let mut acc = Accumulator285::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.398_f32 + y.sin();
        let b = y * 0.539_f32 - x.cos();
        let mut acc = Accumulator285::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.358_f32 + y.sin();
        let b = y * 0.18_f32 - x.cos();
        let mut acc = Accumulator285::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.905_f32 + y.sin();
        let b = y * 8.331_f32 - x.cos();
        let mut acc = Accumulator285::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.485_f32 + y.sin();
        let b = y * 8.963_f32 - x.cos();
        let mut acc = Accumulator285::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.253_f32 + y.sin();
        let b = y * 8.377_f32 - x.cos();
        let mut acc = Accumulator285::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.412_f32 + y.sin();
        let b = y * 8.703_f32 - x.cos();
        let mut acc = Accumulator285::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.304_f32 + y.sin();
        let b = y * 4.32_f32 - x.cos();
        let mut acc = Accumulator285::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.386_f32 + y.sin();
        let b = y * 2.491_f32 - x.cos();
        let mut acc = Accumulator285::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.604_f32 + y.sin();
        let b = y * 3.188_f32 - x.cos();
        let mut acc = Accumulator285::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.929_f32 + y.sin();
        let b = y * 0.498_f32 - x.cos();
        let mut acc = Accumulator285::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.427_f32 + y.sin();
        let b = y * 5.231_f32 - x.cos();
        let mut acc = Accumulator285::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.916_f32 + y.sin();
        let b = y * 4.156_f32 - x.cos();
        let mut acc = Accumulator285::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.154_f32 + y.sin();
        let b = y * 6.533_f32 - x.cos();
        let mut acc = Accumulator285::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.868_f32 + y.sin();
        let b = y * 5.831_f32 - x.cos();
        let mut acc = Accumulator285::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_285(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(285u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_285() -> f32 {
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
        total += (dep_touch_285(total as u64) % 997) as f32;
        total
    }
}

pub mod m286 {
    use super::*;

    pub struct Accumulator286<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator286<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.06_f32 + y.sin();
        let b = y * 1.696_f32 - x.cos();
        let mut acc = Accumulator286::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.683_f32 + y.sin();
        let b = y * 5.338_f32 - x.cos();
        let mut acc = Accumulator286::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.393_f32 + y.sin();
        let b = y * 2.243_f32 - x.cos();
        let mut acc = Accumulator286::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.065_f32 + y.sin();
        let b = y * 0.779_f32 - x.cos();
        let mut acc = Accumulator286::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.793_f32 + y.sin();
        let b = y * 0.131_f32 - x.cos();
        let mut acc = Accumulator286::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.783_f32 + y.sin();
        let b = y * 9.205_f32 - x.cos();
        let mut acc = Accumulator286::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.664_f32 + y.sin();
        let b = y * 3.498_f32 - x.cos();
        let mut acc = Accumulator286::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.126_f32 + y.sin();
        let b = y * 7.622_f32 - x.cos();
        let mut acc = Accumulator286::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.763_f32 + y.sin();
        let b = y * 7.668_f32 - x.cos();
        let mut acc = Accumulator286::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.293_f32 + y.sin();
        let b = y * 6.833_f32 - x.cos();
        let mut acc = Accumulator286::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.929_f32 + y.sin();
        let b = y * 1.953_f32 - x.cos();
        let mut acc = Accumulator286::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.851_f32 + y.sin();
        let b = y * 7.465_f32 - x.cos();
        let mut acc = Accumulator286::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.252_f32 + y.sin();
        let b = y * 1.278_f32 - x.cos();
        let mut acc = Accumulator286::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.499_f32 + y.sin();
        let b = y * 3.121_f32 - x.cos();
        let mut acc = Accumulator286::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.61_f32 + y.sin();
        let b = y * 5.108_f32 - x.cos();
        let mut acc = Accumulator286::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.653_f32 + y.sin();
        let b = y * 0.975_f32 - x.cos();
        let mut acc = Accumulator286::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.804_f32 + y.sin();
        let b = y * 9.155_f32 - x.cos();
        let mut acc = Accumulator286::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.902_f32 + y.sin();
        let b = y * 5.0_f32 - x.cos();
        let mut acc = Accumulator286::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.554_f32 + y.sin();
        let b = y * 4.421_f32 - x.cos();
        let mut acc = Accumulator286::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.392_f32 + y.sin();
        let b = y * 1.461_f32 - x.cos();
        let mut acc = Accumulator286::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_286(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_286() -> f32 {
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
        total += (dep_touch_286(total as u64) % 997) as f32;
        total
    }
}

pub mod m287 {
    use super::*;

    pub struct Accumulator287<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator287<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.604_f32 + y.sin();
        let b = y * 2.423_f32 - x.cos();
        let mut acc = Accumulator287::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.676_f32 + y.sin();
        let b = y * 3.173_f32 - x.cos();
        let mut acc = Accumulator287::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.158_f32 + y.sin();
        let b = y * 3.632_f32 - x.cos();
        let mut acc = Accumulator287::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.264_f32 + y.sin();
        let b = y * 7.384_f32 - x.cos();
        let mut acc = Accumulator287::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.052_f32 + y.sin();
        let b = y * 3.425_f32 - x.cos();
        let mut acc = Accumulator287::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.916_f32 + y.sin();
        let b = y * 0.976_f32 - x.cos();
        let mut acc = Accumulator287::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.237_f32 + y.sin();
        let b = y * 3.047_f32 - x.cos();
        let mut acc = Accumulator287::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.222_f32 + y.sin();
        let b = y * 7.223_f32 - x.cos();
        let mut acc = Accumulator287::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.251_f32 + y.sin();
        let b = y * 0.203_f32 - x.cos();
        let mut acc = Accumulator287::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.258_f32 + y.sin();
        let b = y * 2.302_f32 - x.cos();
        let mut acc = Accumulator287::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.668_f32 + y.sin();
        let b = y * 9.483_f32 - x.cos();
        let mut acc = Accumulator287::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.808_f32 + y.sin();
        let b = y * 7.997_f32 - x.cos();
        let mut acc = Accumulator287::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.95_f32 + y.sin();
        let b = y * 5.846_f32 - x.cos();
        let mut acc = Accumulator287::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.974_f32 + y.sin();
        let b = y * 4.818_f32 - x.cos();
        let mut acc = Accumulator287::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.608_f32 + y.sin();
        let b = y * 5.556_f32 - x.cos();
        let mut acc = Accumulator287::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.126_f32 + y.sin();
        let b = y * 8.918_f32 - x.cos();
        let mut acc = Accumulator287::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.306_f32 + y.sin();
        let b = y * 4.391_f32 - x.cos();
        let mut acc = Accumulator287::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.811_f32 + y.sin();
        let b = y * 6.229_f32 - x.cos();
        let mut acc = Accumulator287::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.693_f32 + y.sin();
        let b = y * 1.725_f32 - x.cos();
        let mut acc = Accumulator287::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.242_f32 + y.sin();
        let b = y * 2.35_f32 - x.cos();
        let mut acc = Accumulator287::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_287(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_287() -> f32 {
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
        total += (dep_touch_287(total as u64) % 997) as f32;
        total
    }
}

pub mod m288 {
    use super::*;

    pub struct Accumulator288<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator288<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.422_f32 + y.sin();
        let b = y * 9.205_f32 - x.cos();
        let mut acc = Accumulator288::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.134_f32 + y.sin();
        let b = y * 5.237_f32 - x.cos();
        let mut acc = Accumulator288::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.013_f32 + y.sin();
        let b = y * 9.085_f32 - x.cos();
        let mut acc = Accumulator288::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.564_f32 + y.sin();
        let b = y * 8.359_f32 - x.cos();
        let mut acc = Accumulator288::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.003_f32 + y.sin();
        let b = y * 6.317_f32 - x.cos();
        let mut acc = Accumulator288::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.387_f32 + y.sin();
        let b = y * 4.479_f32 - x.cos();
        let mut acc = Accumulator288::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.687_f32 + y.sin();
        let b = y * 5.146_f32 - x.cos();
        let mut acc = Accumulator288::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.903_f32 + y.sin();
        let b = y * 7.002_f32 - x.cos();
        let mut acc = Accumulator288::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.421_f32 + y.sin();
        let b = y * 5.984_f32 - x.cos();
        let mut acc = Accumulator288::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.815_f32 + y.sin();
        let b = y * 2.561_f32 - x.cos();
        let mut acc = Accumulator288::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.661_f32 + y.sin();
        let b = y * 0.484_f32 - x.cos();
        let mut acc = Accumulator288::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.701_f32 + y.sin();
        let b = y * 6.118_f32 - x.cos();
        let mut acc = Accumulator288::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.365_f32 + y.sin();
        let b = y * 1.698_f32 - x.cos();
        let mut acc = Accumulator288::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.433_f32 + y.sin();
        let b = y * 3.451_f32 - x.cos();
        let mut acc = Accumulator288::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.227_f32 + y.sin();
        let b = y * 9.332_f32 - x.cos();
        let mut acc = Accumulator288::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.574_f32 + y.sin();
        let b = y * 9.894_f32 - x.cos();
        let mut acc = Accumulator288::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.244_f32 + y.sin();
        let b = y * 6.859_f32 - x.cos();
        let mut acc = Accumulator288::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.098_f32 + y.sin();
        let b = y * 8.819_f32 - x.cos();
        let mut acc = Accumulator288::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.076_f32 + y.sin();
        let b = y * 7.172_f32 - x.cos();
        let mut acc = Accumulator288::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.851_f32 + y.sin();
        let b = y * 8.269_f32 - x.cos();
        let mut acc = Accumulator288::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_288(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m288-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_288() -> f32 {
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
        total += (dep_touch_288(total as u64) % 997) as f32;
        total
    }
}

pub mod m289 {
    use super::*;

    pub struct Accumulator289<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator289<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.646_f32 + y.sin();
        let b = y * 4.311_f32 - x.cos();
        let mut acc = Accumulator289::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.133_f32 + y.sin();
        let b = y * 6.071_f32 - x.cos();
        let mut acc = Accumulator289::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.392_f32 + y.sin();
        let b = y * 5.559_f32 - x.cos();
        let mut acc = Accumulator289::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.953_f32 + y.sin();
        let b = y * 5.857_f32 - x.cos();
        let mut acc = Accumulator289::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.163_f32 + y.sin();
        let b = y * 8.708_f32 - x.cos();
        let mut acc = Accumulator289::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.868_f32 + y.sin();
        let b = y * 5.882_f32 - x.cos();
        let mut acc = Accumulator289::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.356_f32 + y.sin();
        let b = y * 3.033_f32 - x.cos();
        let mut acc = Accumulator289::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.595_f32 + y.sin();
        let b = y * 3.263_f32 - x.cos();
        let mut acc = Accumulator289::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.637_f32 + y.sin();
        let b = y * 9.394_f32 - x.cos();
        let mut acc = Accumulator289::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.069_f32 + y.sin();
        let b = y * 6.981_f32 - x.cos();
        let mut acc = Accumulator289::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.989_f32 + y.sin();
        let b = y * 6.78_f32 - x.cos();
        let mut acc = Accumulator289::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.304_f32 + y.sin();
        let b = y * 6.248_f32 - x.cos();
        let mut acc = Accumulator289::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.44_f32 + y.sin();
        let b = y * 9.364_f32 - x.cos();
        let mut acc = Accumulator289::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.593_f32 + y.sin();
        let b = y * 5.556_f32 - x.cos();
        let mut acc = Accumulator289::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.992_f32 + y.sin();
        let b = y * 9.187_f32 - x.cos();
        let mut acc = Accumulator289::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.127_f32 + y.sin();
        let b = y * 6.066_f32 - x.cos();
        let mut acc = Accumulator289::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.148_f32 + y.sin();
        let b = y * 9.092_f32 - x.cos();
        let mut acc = Accumulator289::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.127_f32 + y.sin();
        let b = y * 8.78_f32 - x.cos();
        let mut acc = Accumulator289::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.572_f32 + y.sin();
        let b = y * 6.937_f32 - x.cos();
        let mut acc = Accumulator289::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.59_f32 + y.sin();
        let b = y * 0.215_f32 - x.cos();
        let mut acc = Accumulator289::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_289(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_289() -> f32 {
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
        total += (dep_touch_289(total as u64) % 997) as f32;
        total
    }
}

pub mod m290 {
    use super::*;

    pub struct Accumulator290<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator290<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.146_f32 + y.sin();
        let b = y * 7.969_f32 - x.cos();
        let mut acc = Accumulator290::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.671_f32 + y.sin();
        let b = y * 6.724_f32 - x.cos();
        let mut acc = Accumulator290::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.102_f32 + y.sin();
        let b = y * 3.37_f32 - x.cos();
        let mut acc = Accumulator290::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.273_f32 + y.sin();
        let b = y * 1.187_f32 - x.cos();
        let mut acc = Accumulator290::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.872_f32 + y.sin();
        let b = y * 6.604_f32 - x.cos();
        let mut acc = Accumulator290::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.281_f32 + y.sin();
        let b = y * 5.044_f32 - x.cos();
        let mut acc = Accumulator290::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.554_f32 + y.sin();
        let b = y * 1.004_f32 - x.cos();
        let mut acc = Accumulator290::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.518_f32 + y.sin();
        let b = y * 2.141_f32 - x.cos();
        let mut acc = Accumulator290::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.032_f32 + y.sin();
        let b = y * 1.63_f32 - x.cos();
        let mut acc = Accumulator290::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.097_f32 + y.sin();
        let b = y * 1.323_f32 - x.cos();
        let mut acc = Accumulator290::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.665_f32 + y.sin();
        let b = y * 2.666_f32 - x.cos();
        let mut acc = Accumulator290::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.201_f32 + y.sin();
        let b = y * 9.182_f32 - x.cos();
        let mut acc = Accumulator290::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.552_f32 + y.sin();
        let b = y * 6.712_f32 - x.cos();
        let mut acc = Accumulator290::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.994_f32 + y.sin();
        let b = y * 1.584_f32 - x.cos();
        let mut acc = Accumulator290::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.959_f32 + y.sin();
        let b = y * 9.319_f32 - x.cos();
        let mut acc = Accumulator290::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.585_f32 + y.sin();
        let b = y * 1.301_f32 - x.cos();
        let mut acc = Accumulator290::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.651_f32 + y.sin();
        let b = y * 8.975_f32 - x.cos();
        let mut acc = Accumulator290::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.139_f32 + y.sin();
        let b = y * 4.191_f32 - x.cos();
        let mut acc = Accumulator290::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.017_f32 + y.sin();
        let b = y * 3.897_f32 - x.cos();
        let mut acc = Accumulator290::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.107_f32 + y.sin();
        let b = y * 2.156_f32 - x.cos();
        let mut acc = Accumulator290::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_290(seed: u64) -> u64 {
        let re = Regex::new(r"m290-(\d+)").unwrap();
        let hay = format!("m290-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_290() -> f32 {
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
        total += (dep_touch_290(total as u64) % 997) as f32;
        total
    }
}

pub mod m291 {
    use super::*;

    pub struct Accumulator291<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator291<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.913_f32 + y.sin();
        let b = y * 8.833_f32 - x.cos();
        let mut acc = Accumulator291::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.334_f32 + y.sin();
        let b = y * 2.347_f32 - x.cos();
        let mut acc = Accumulator291::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.738_f32 + y.sin();
        let b = y * 1.579_f32 - x.cos();
        let mut acc = Accumulator291::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.34_f32 + y.sin();
        let b = y * 5.497_f32 - x.cos();
        let mut acc = Accumulator291::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.404_f32 + y.sin();
        let b = y * 7.686_f32 - x.cos();
        let mut acc = Accumulator291::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.324_f32 + y.sin();
        let b = y * 3.755_f32 - x.cos();
        let mut acc = Accumulator291::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.028_f32 + y.sin();
        let b = y * 2.421_f32 - x.cos();
        let mut acc = Accumulator291::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.89_f32 + y.sin();
        let b = y * 8.885_f32 - x.cos();
        let mut acc = Accumulator291::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.976_f32 + y.sin();
        let b = y * 6.09_f32 - x.cos();
        let mut acc = Accumulator291::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.62_f32 + y.sin();
        let b = y * 5.366_f32 - x.cos();
        let mut acc = Accumulator291::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.608_f32 + y.sin();
        let b = y * 7.392_f32 - x.cos();
        let mut acc = Accumulator291::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.39_f32 + y.sin();
        let b = y * 8.075_f32 - x.cos();
        let mut acc = Accumulator291::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.466_f32 + y.sin();
        let b = y * 1.713_f32 - x.cos();
        let mut acc = Accumulator291::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.255_f32 + y.sin();
        let b = y * 2.913_f32 - x.cos();
        let mut acc = Accumulator291::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.844_f32 + y.sin();
        let b = y * 7.193_f32 - x.cos();
        let mut acc = Accumulator291::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.671_f32 + y.sin();
        let b = y * 3.794_f32 - x.cos();
        let mut acc = Accumulator291::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.324_f32 + y.sin();
        let b = y * 7.067_f32 - x.cos();
        let mut acc = Accumulator291::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.148_f32 + y.sin();
        let b = y * 4.765_f32 - x.cos();
        let mut acc = Accumulator291::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.333_f32 + y.sin();
        let b = y * 6.651_f32 - x.cos();
        let mut acc = Accumulator291::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.913_f32 + y.sin();
        let b = y * 3.971_f32 - x.cos();
        let mut acc = Accumulator291::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_291(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_291() -> f32 {
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
        total += (dep_touch_291(total as u64) % 997) as f32;
        total
    }
}

pub mod m292 {
    use super::*;

    pub struct Accumulator292<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator292<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.895_f32 + y.sin();
        let b = y * 4.232_f32 - x.cos();
        let mut acc = Accumulator292::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.899_f32 + y.sin();
        let b = y * 2.268_f32 - x.cos();
        let mut acc = Accumulator292::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.912_f32 + y.sin();
        let b = y * 8.746_f32 - x.cos();
        let mut acc = Accumulator292::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.616_f32 + y.sin();
        let b = y * 7.536_f32 - x.cos();
        let mut acc = Accumulator292::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.774_f32 + y.sin();
        let b = y * 7.479_f32 - x.cos();
        let mut acc = Accumulator292::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.999_f32 + y.sin();
        let b = y * 6.288_f32 - x.cos();
        let mut acc = Accumulator292::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.81_f32 + y.sin();
        let b = y * 1.441_f32 - x.cos();
        let mut acc = Accumulator292::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.435_f32 + y.sin();
        let b = y * 1.408_f32 - x.cos();
        let mut acc = Accumulator292::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.328_f32 + y.sin();
        let b = y * 9.781_f32 - x.cos();
        let mut acc = Accumulator292::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.35_f32 + y.sin();
        let b = y * 7.681_f32 - x.cos();
        let mut acc = Accumulator292::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.319_f32 + y.sin();
        let b = y * 9.745_f32 - x.cos();
        let mut acc = Accumulator292::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.216_f32 + y.sin();
        let b = y * 9.611_f32 - x.cos();
        let mut acc = Accumulator292::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.238_f32 + y.sin();
        let b = y * 5.875_f32 - x.cos();
        let mut acc = Accumulator292::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.088_f32 + y.sin();
        let b = y * 4.644_f32 - x.cos();
        let mut acc = Accumulator292::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.796_f32 + y.sin();
        let b = y * 1.164_f32 - x.cos();
        let mut acc = Accumulator292::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.873_f32 + y.sin();
        let b = y * 3.005_f32 - x.cos();
        let mut acc = Accumulator292::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.003_f32 + y.sin();
        let b = y * 1.854_f32 - x.cos();
        let mut acc = Accumulator292::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.12_f32 + y.sin();
        let b = y * 0.502_f32 - x.cos();
        let mut acc = Accumulator292::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.04_f32 + y.sin();
        let b = y * 8.156_f32 - x.cos();
        let mut acc = Accumulator292::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.48_f32 + y.sin();
        let b = y * 8.42_f32 - x.cos();
        let mut acc = Accumulator292::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_292(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(292u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_292() -> f32 {
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
        total += (dep_touch_292(total as u64) % 997) as f32;
        total
    }
}

pub mod m293 {
    use super::*;

    pub struct Accumulator293<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator293<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.715_f32 + y.sin();
        let b = y * 1.63_f32 - x.cos();
        let mut acc = Accumulator293::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.296_f32 + y.sin();
        let b = y * 5.167_f32 - x.cos();
        let mut acc = Accumulator293::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.894_f32 + y.sin();
        let b = y * 0.158_f32 - x.cos();
        let mut acc = Accumulator293::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.185_f32 + y.sin();
        let b = y * 1.463_f32 - x.cos();
        let mut acc = Accumulator293::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.113_f32 + y.sin();
        let b = y * 0.661_f32 - x.cos();
        let mut acc = Accumulator293::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.382_f32 + y.sin();
        let b = y * 9.302_f32 - x.cos();
        let mut acc = Accumulator293::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.404_f32 + y.sin();
        let b = y * 9.849_f32 - x.cos();
        let mut acc = Accumulator293::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.331_f32 + y.sin();
        let b = y * 6.275_f32 - x.cos();
        let mut acc = Accumulator293::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.441_f32 + y.sin();
        let b = y * 5.283_f32 - x.cos();
        let mut acc = Accumulator293::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.825_f32 + y.sin();
        let b = y * 2.443_f32 - x.cos();
        let mut acc = Accumulator293::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.974_f32 + y.sin();
        let b = y * 3.002_f32 - x.cos();
        let mut acc = Accumulator293::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.295_f32 + y.sin();
        let b = y * 9.464_f32 - x.cos();
        let mut acc = Accumulator293::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.001_f32 + y.sin();
        let b = y * 8.734_f32 - x.cos();
        let mut acc = Accumulator293::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.18_f32 + y.sin();
        let b = y * 3.511_f32 - x.cos();
        let mut acc = Accumulator293::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.069_f32 + y.sin();
        let b = y * 2.41_f32 - x.cos();
        let mut acc = Accumulator293::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.538_f32 + y.sin();
        let b = y * 5.244_f32 - x.cos();
        let mut acc = Accumulator293::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.156_f32 + y.sin();
        let b = y * 2.566_f32 - x.cos();
        let mut acc = Accumulator293::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.117_f32 + y.sin();
        let b = y * 9.614_f32 - x.cos();
        let mut acc = Accumulator293::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.493_f32 + y.sin();
        let b = y * 6.637_f32 - x.cos();
        let mut acc = Accumulator293::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.708_f32 + y.sin();
        let b = y * 7.753_f32 - x.cos();
        let mut acc = Accumulator293::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_293(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_293() -> f32 {
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
        total += (dep_touch_293(total as u64) % 997) as f32;
        total
    }
}

pub mod m294 {
    use super::*;

    pub struct Accumulator294<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator294<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.641_f32 + y.sin();
        let b = y * 1.893_f32 - x.cos();
        let mut acc = Accumulator294::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.714_f32 + y.sin();
        let b = y * 4.058_f32 - x.cos();
        let mut acc = Accumulator294::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.246_f32 + y.sin();
        let b = y * 9.515_f32 - x.cos();
        let mut acc = Accumulator294::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.197_f32 + y.sin();
        let b = y * 6.028_f32 - x.cos();
        let mut acc = Accumulator294::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.818_f32 + y.sin();
        let b = y * 4.321_f32 - x.cos();
        let mut acc = Accumulator294::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.569_f32 + y.sin();
        let b = y * 0.777_f32 - x.cos();
        let mut acc = Accumulator294::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.626_f32 + y.sin();
        let b = y * 7.867_f32 - x.cos();
        let mut acc = Accumulator294::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.229_f32 + y.sin();
        let b = y * 2.002_f32 - x.cos();
        let mut acc = Accumulator294::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.356_f32 + y.sin();
        let b = y * 7.551_f32 - x.cos();
        let mut acc = Accumulator294::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.691_f32 + y.sin();
        let b = y * 8.782_f32 - x.cos();
        let mut acc = Accumulator294::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.411_f32 + y.sin();
        let b = y * 5.595_f32 - x.cos();
        let mut acc = Accumulator294::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.623_f32 + y.sin();
        let b = y * 5.12_f32 - x.cos();
        let mut acc = Accumulator294::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.991_f32 + y.sin();
        let b = y * 7.093_f32 - x.cos();
        let mut acc = Accumulator294::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.156_f32 + y.sin();
        let b = y * 5.732_f32 - x.cos();
        let mut acc = Accumulator294::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.407_f32 + y.sin();
        let b = y * 2.968_f32 - x.cos();
        let mut acc = Accumulator294::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.136_f32 + y.sin();
        let b = y * 2.137_f32 - x.cos();
        let mut acc = Accumulator294::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.388_f32 + y.sin();
        let b = y * 5.497_f32 - x.cos();
        let mut acc = Accumulator294::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.791_f32 + y.sin();
        let b = y * 1.244_f32 - x.cos();
        let mut acc = Accumulator294::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.24_f32 + y.sin();
        let b = y * 9.545_f32 - x.cos();
        let mut acc = Accumulator294::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.961_f32 + y.sin();
        let b = y * 6.452_f32 - x.cos();
        let mut acc = Accumulator294::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_294(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_294() -> f32 {
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
        total += (dep_touch_294(total as u64) % 997) as f32;
        total
    }
}

pub mod m295 {
    use super::*;

    pub struct Accumulator295<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator295<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.736_f32 + y.sin();
        let b = y * 9.05_f32 - x.cos();
        let mut acc = Accumulator295::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.625_f32 + y.sin();
        let b = y * 5.284_f32 - x.cos();
        let mut acc = Accumulator295::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.465_f32 + y.sin();
        let b = y * 5.396_f32 - x.cos();
        let mut acc = Accumulator295::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.793_f32 + y.sin();
        let b = y * 7.531_f32 - x.cos();
        let mut acc = Accumulator295::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.544_f32 + y.sin();
        let b = y * 3.368_f32 - x.cos();
        let mut acc = Accumulator295::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.44_f32 + y.sin();
        let b = y * 1.807_f32 - x.cos();
        let mut acc = Accumulator295::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.917_f32 + y.sin();
        let b = y * 6.095_f32 - x.cos();
        let mut acc = Accumulator295::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.366_f32 + y.sin();
        let b = y * 0.238_f32 - x.cos();
        let mut acc = Accumulator295::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.226_f32 + y.sin();
        let b = y * 0.707_f32 - x.cos();
        let mut acc = Accumulator295::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.372_f32 + y.sin();
        let b = y * 4.189_f32 - x.cos();
        let mut acc = Accumulator295::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.269_f32 + y.sin();
        let b = y * 6.703_f32 - x.cos();
        let mut acc = Accumulator295::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.916_f32 + y.sin();
        let b = y * 9.106_f32 - x.cos();
        let mut acc = Accumulator295::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.294_f32 + y.sin();
        let b = y * 2.172_f32 - x.cos();
        let mut acc = Accumulator295::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.511_f32 + y.sin();
        let b = y * 1.659_f32 - x.cos();
        let mut acc = Accumulator295::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.094_f32 + y.sin();
        let b = y * 2.31_f32 - x.cos();
        let mut acc = Accumulator295::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.883_f32 + y.sin();
        let b = y * 5.308_f32 - x.cos();
        let mut acc = Accumulator295::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.279_f32 + y.sin();
        let b = y * 7.118_f32 - x.cos();
        let mut acc = Accumulator295::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.047_f32 + y.sin();
        let b = y * 3.577_f32 - x.cos();
        let mut acc = Accumulator295::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.227_f32 + y.sin();
        let b = y * 3.914_f32 - x.cos();
        let mut acc = Accumulator295::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.138_f32 + y.sin();
        let b = y * 5.58_f32 - x.cos();
        let mut acc = Accumulator295::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_295(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m295-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_295() -> f32 {
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
        total += (dep_touch_295(total as u64) % 997) as f32;
        total
    }
}

pub mod m296 {
    use super::*;

    pub struct Accumulator296<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator296<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.696_f32 + y.sin();
        let b = y * 5.86_f32 - x.cos();
        let mut acc = Accumulator296::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.381_f32 + y.sin();
        let b = y * 6.822_f32 - x.cos();
        let mut acc = Accumulator296::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.442_f32 + y.sin();
        let b = y * 6.349_f32 - x.cos();
        let mut acc = Accumulator296::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.821_f32 + y.sin();
        let b = y * 1.47_f32 - x.cos();
        let mut acc = Accumulator296::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.63_f32 + y.sin();
        let b = y * 0.586_f32 - x.cos();
        let mut acc = Accumulator296::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.609_f32 + y.sin();
        let b = y * 3.755_f32 - x.cos();
        let mut acc = Accumulator296::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.291_f32 + y.sin();
        let b = y * 8.603_f32 - x.cos();
        let mut acc = Accumulator296::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.456_f32 + y.sin();
        let b = y * 7.611_f32 - x.cos();
        let mut acc = Accumulator296::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.401_f32 + y.sin();
        let b = y * 4.648_f32 - x.cos();
        let mut acc = Accumulator296::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.909_f32 + y.sin();
        let b = y * 1.273_f32 - x.cos();
        let mut acc = Accumulator296::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.432_f32 + y.sin();
        let b = y * 8.173_f32 - x.cos();
        let mut acc = Accumulator296::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.387_f32 + y.sin();
        let b = y * 3.527_f32 - x.cos();
        let mut acc = Accumulator296::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.781_f32 + y.sin();
        let b = y * 5.936_f32 - x.cos();
        let mut acc = Accumulator296::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.577_f32 + y.sin();
        let b = y * 8.982_f32 - x.cos();
        let mut acc = Accumulator296::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.484_f32 + y.sin();
        let b = y * 1.711_f32 - x.cos();
        let mut acc = Accumulator296::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.609_f32 + y.sin();
        let b = y * 9.052_f32 - x.cos();
        let mut acc = Accumulator296::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.674_f32 + y.sin();
        let b = y * 3.325_f32 - x.cos();
        let mut acc = Accumulator296::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.089_f32 + y.sin();
        let b = y * 9.169_f32 - x.cos();
        let mut acc = Accumulator296::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.074_f32 + y.sin();
        let b = y * 5.72_f32 - x.cos();
        let mut acc = Accumulator296::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.835_f32 + y.sin();
        let b = y * 4.984_f32 - x.cos();
        let mut acc = Accumulator296::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_296(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_296() -> f32 {
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
        total += (dep_touch_296(total as u64) % 997) as f32;
        total
    }
}

pub mod m297 {
    use super::*;

    pub struct Accumulator297<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator297<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.172_f32 + y.sin();
        let b = y * 4.601_f32 - x.cos();
        let mut acc = Accumulator297::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.722_f32 + y.sin();
        let b = y * 1.797_f32 - x.cos();
        let mut acc = Accumulator297::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.005_f32 + y.sin();
        let b = y * 3.317_f32 - x.cos();
        let mut acc = Accumulator297::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.268_f32 + y.sin();
        let b = y * 9.802_f32 - x.cos();
        let mut acc = Accumulator297::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.476_f32 + y.sin();
        let b = y * 2.916_f32 - x.cos();
        let mut acc = Accumulator297::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.073_f32 + y.sin();
        let b = y * 6.592_f32 - x.cos();
        let mut acc = Accumulator297::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.659_f32 + y.sin();
        let b = y * 6.185_f32 - x.cos();
        let mut acc = Accumulator297::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.748_f32 + y.sin();
        let b = y * 2.997_f32 - x.cos();
        let mut acc = Accumulator297::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.043_f32 + y.sin();
        let b = y * 2.35_f32 - x.cos();
        let mut acc = Accumulator297::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.849_f32 + y.sin();
        let b = y * 4.143_f32 - x.cos();
        let mut acc = Accumulator297::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.568_f32 + y.sin();
        let b = y * 4.667_f32 - x.cos();
        let mut acc = Accumulator297::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.876_f32 + y.sin();
        let b = y * 2.662_f32 - x.cos();
        let mut acc = Accumulator297::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.412_f32 + y.sin();
        let b = y * 7.579_f32 - x.cos();
        let mut acc = Accumulator297::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.282_f32 + y.sin();
        let b = y * 2.09_f32 - x.cos();
        let mut acc = Accumulator297::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.737_f32 + y.sin();
        let b = y * 0.361_f32 - x.cos();
        let mut acc = Accumulator297::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.721_f32 + y.sin();
        let b = y * 2.908_f32 - x.cos();
        let mut acc = Accumulator297::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.603_f32 + y.sin();
        let b = y * 5.859_f32 - x.cos();
        let mut acc = Accumulator297::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.882_f32 + y.sin();
        let b = y * 0.41_f32 - x.cos();
        let mut acc = Accumulator297::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.356_f32 + y.sin();
        let b = y * 0.29_f32 - x.cos();
        let mut acc = Accumulator297::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.053_f32 + y.sin();
        let b = y * 9.564_f32 - x.cos();
        let mut acc = Accumulator297::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_297(seed: u64) -> u64 {
        let re = Regex::new(r"m297-(\d+)").unwrap();
        let hay = format!("m297-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_297() -> f32 {
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
        total += (dep_touch_297(total as u64) % 997) as f32;
        total
    }
}

pub mod m298 {
    use super::*;

    pub struct Accumulator298<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator298<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.307_f32 + y.sin();
        let b = y * 9.798_f32 - x.cos();
        let mut acc = Accumulator298::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.572_f32 + y.sin();
        let b = y * 5.738_f32 - x.cos();
        let mut acc = Accumulator298::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.942_f32 + y.sin();
        let b = y * 2.214_f32 - x.cos();
        let mut acc = Accumulator298::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.319_f32 + y.sin();
        let b = y * 0.195_f32 - x.cos();
        let mut acc = Accumulator298::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.098_f32 + y.sin();
        let b = y * 5.249_f32 - x.cos();
        let mut acc = Accumulator298::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.152_f32 + y.sin();
        let b = y * 3.077_f32 - x.cos();
        let mut acc = Accumulator298::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.404_f32 + y.sin();
        let b = y * 8.295_f32 - x.cos();
        let mut acc = Accumulator298::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.177_f32 + y.sin();
        let b = y * 0.829_f32 - x.cos();
        let mut acc = Accumulator298::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.213_f32 + y.sin();
        let b = y * 5.107_f32 - x.cos();
        let mut acc = Accumulator298::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.716_f32 + y.sin();
        let b = y * 0.187_f32 - x.cos();
        let mut acc = Accumulator298::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.883_f32 + y.sin();
        let b = y * 1.926_f32 - x.cos();
        let mut acc = Accumulator298::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.611_f32 + y.sin();
        let b = y * 8.822_f32 - x.cos();
        let mut acc = Accumulator298::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.987_f32 + y.sin();
        let b = y * 8.554_f32 - x.cos();
        let mut acc = Accumulator298::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.014_f32 + y.sin();
        let b = y * 9.442_f32 - x.cos();
        let mut acc = Accumulator298::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.473_f32 + y.sin();
        let b = y * 8.82_f32 - x.cos();
        let mut acc = Accumulator298::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.68_f32 + y.sin();
        let b = y * 1.572_f32 - x.cos();
        let mut acc = Accumulator298::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.328_f32 + y.sin();
        let b = y * 8.871_f32 - x.cos();
        let mut acc = Accumulator298::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.048_f32 + y.sin();
        let b = y * 2.92_f32 - x.cos();
        let mut acc = Accumulator298::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.822_f32 + y.sin();
        let b = y * 5.789_f32 - x.cos();
        let mut acc = Accumulator298::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.864_f32 + y.sin();
        let b = y * 0.981_f32 - x.cos();
        let mut acc = Accumulator298::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_298(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_298() -> f32 {
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
        total += (dep_touch_298(total as u64) % 997) as f32;
        total
    }
}

pub mod m299 {
    use super::*;

    pub struct Accumulator299<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator299<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.699_f32 + y.sin();
        let b = y * 6.54_f32 - x.cos();
        let mut acc = Accumulator299::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.085_f32 + y.sin();
        let b = y * 6.614_f32 - x.cos();
        let mut acc = Accumulator299::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.24_f32 + y.sin();
        let b = y * 9.172_f32 - x.cos();
        let mut acc = Accumulator299::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.102_f32 + y.sin();
        let b = y * 9.308_f32 - x.cos();
        let mut acc = Accumulator299::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.939_f32 + y.sin();
        let b = y * 4.586_f32 - x.cos();
        let mut acc = Accumulator299::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.121_f32 + y.sin();
        let b = y * 5.214_f32 - x.cos();
        let mut acc = Accumulator299::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.385_f32 + y.sin();
        let b = y * 3.744_f32 - x.cos();
        let mut acc = Accumulator299::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.793_f32 + y.sin();
        let b = y * 4.942_f32 - x.cos();
        let mut acc = Accumulator299::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.101_f32 + y.sin();
        let b = y * 9.048_f32 - x.cos();
        let mut acc = Accumulator299::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.61_f32 + y.sin();
        let b = y * 6.982_f32 - x.cos();
        let mut acc = Accumulator299::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.85_f32 + y.sin();
        let b = y * 2.799_f32 - x.cos();
        let mut acc = Accumulator299::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.989_f32 + y.sin();
        let b = y * 8.223_f32 - x.cos();
        let mut acc = Accumulator299::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.836_f32 + y.sin();
        let b = y * 7.37_f32 - x.cos();
        let mut acc = Accumulator299::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.059_f32 + y.sin();
        let b = y * 8.548_f32 - x.cos();
        let mut acc = Accumulator299::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.353_f32 + y.sin();
        let b = y * 7.244_f32 - x.cos();
        let mut acc = Accumulator299::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.702_f32 + y.sin();
        let b = y * 6.587_f32 - x.cos();
        let mut acc = Accumulator299::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.591_f32 + y.sin();
        let b = y * 3.933_f32 - x.cos();
        let mut acc = Accumulator299::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.903_f32 + y.sin();
        let b = y * 7.839_f32 - x.cos();
        let mut acc = Accumulator299::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.124_f32 + y.sin();
        let b = y * 8.391_f32 - x.cos();
        let mut acc = Accumulator299::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.906_f32 + y.sin();
        let b = y * 3.379_f32 - x.cos();
        let mut acc = Accumulator299::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_299(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(299u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_299() -> f32 {
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
        total += (dep_touch_299(total as u64) % 997) as f32;
        total
    }
}

pub fn touch_bulk_2() -> f32 {
    let mut total = 0.0_f32;
    total += m200::run_all_200();
    total += m201::run_all_201();
    total += m202::run_all_202();
    total += m203::run_all_203();
    total += m204::run_all_204();
    total += m205::run_all_205();
    total += m206::run_all_206();
    total += m207::run_all_207();
    total += m208::run_all_208();
    total += m209::run_all_209();
    total += m210::run_all_210();
    total += m211::run_all_211();
    total += m212::run_all_212();
    total += m213::run_all_213();
    total += m214::run_all_214();
    total += m215::run_all_215();
    total += m216::run_all_216();
    total += m217::run_all_217();
    total += m218::run_all_218();
    total += m219::run_all_219();
    total += m220::run_all_220();
    total += m221::run_all_221();
    total += m222::run_all_222();
    total += m223::run_all_223();
    total += m224::run_all_224();
    total += m225::run_all_225();
    total += m226::run_all_226();
    total += m227::run_all_227();
    total += m228::run_all_228();
    total += m229::run_all_229();
    total += m230::run_all_230();
    total += m231::run_all_231();
    total += m232::run_all_232();
    total += m233::run_all_233();
    total += m234::run_all_234();
    total += m235::run_all_235();
    total += m236::run_all_236();
    total += m237::run_all_237();
    total += m238::run_all_238();
    total += m239::run_all_239();
    total += m240::run_all_240();
    total += m241::run_all_241();
    total += m242::run_all_242();
    total += m243::run_all_243();
    total += m244::run_all_244();
    total += m245::run_all_245();
    total += m246::run_all_246();
    total += m247::run_all_247();
    total += m248::run_all_248();
    total += m249::run_all_249();
    total += m250::run_all_250();
    total += m251::run_all_251();
    total += m252::run_all_252();
    total += m253::run_all_253();
    total += m254::run_all_254();
    total += m255::run_all_255();
    total += m256::run_all_256();
    total += m257::run_all_257();
    total += m258::run_all_258();
    total += m259::run_all_259();
    total += m260::run_all_260();
    total += m261::run_all_261();
    total += m262::run_all_262();
    total += m263::run_all_263();
    total += m264::run_all_264();
    total += m265::run_all_265();
    total += m266::run_all_266();
    total += m267::run_all_267();
    total += m268::run_all_268();
    total += m269::run_all_269();
    total += m270::run_all_270();
    total += m271::run_all_271();
    total += m272::run_all_272();
    total += m273::run_all_273();
    total += m274::run_all_274();
    total += m275::run_all_275();
    total += m276::run_all_276();
    total += m277::run_all_277();
    total += m278::run_all_278();
    total += m279::run_all_279();
    total += m280::run_all_280();
    total += m281::run_all_281();
    total += m282::run_all_282();
    total += m283::run_all_283();
    total += m284::run_all_284();
    total += m285::run_all_285();
    total += m286::run_all_286();
    total += m287::run_all_287();
    total += m288::run_all_288();
    total += m289::run_all_289();
    total += m290::run_all_290();
    total += m291::run_all_291();
    total += m292::run_all_292();
    total += m293::run_all_293();
    total += m294::run_all_294();
    total += m295::run_all_295();
    total += m296::run_all_296();
    total += m297::run_all_297();
    total += m298::run_all_298();
    total += m299::run_all_299();
    total
}
