//! Auto-generated bulk module (file 0) - exists purely to make `game_rs`
//! heavier for a hot-patch speed experiment. Real arithmetic, real generics
//! (so monomorphization isn't free), and real use of a rotating set of
//! external crates so none of them are dead code. `touch_bulk_0()` is
//! called once from `game::init` so nothing here gets stripped as unreachable.

use rand::{Rng, SeedableRng};
use sha2::{Digest, Sha256};
use noise::{NoiseFn, Perlin};
use regex::Regex;
use itertools::Itertools;
use uuid::Uuid;
use chrono::Utc;


pub mod m0 {
    use super::*;

    pub struct Accumulator0<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator0<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.274_f32 + y.sin();
        let b = y * 1.578_f32 - x.cos();
        let mut acc = Accumulator0::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.573_f32 + y.sin();
        let b = y * 8.148_f32 - x.cos();
        let mut acc = Accumulator0::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.684_f32 + y.sin();
        let b = y * 0.668_f32 - x.cos();
        let mut acc = Accumulator0::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.204_f32 + y.sin();
        let b = y * 0.942_f32 - x.cos();
        let mut acc = Accumulator0::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.785_f32 + y.sin();
        let b = y * 0.989_f32 - x.cos();
        let mut acc = Accumulator0::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.679_f32 + y.sin();
        let b = y * 5.641_f32 - x.cos();
        let mut acc = Accumulator0::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.28_f32 + y.sin();
        let b = y * 5.813_f32 - x.cos();
        let mut acc = Accumulator0::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.756_f32 + y.sin();
        let b = y * 3.987_f32 - x.cos();
        let mut acc = Accumulator0::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.557_f32 + y.sin();
        let b = y * 8.513_f32 - x.cos();
        let mut acc = Accumulator0::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.208_f32 + y.sin();
        let b = y * 5.399_f32 - x.cos();
        let mut acc = Accumulator0::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.123_f32 + y.sin();
        let b = y * 8.098_f32 - x.cos();
        let mut acc = Accumulator0::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.11_f32 + y.sin();
        let b = y * 5.698_f32 - x.cos();
        let mut acc = Accumulator0::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.749_f32 + y.sin();
        let b = y * 5.468_f32 - x.cos();
        let mut acc = Accumulator0::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.631_f32 + y.sin();
        let b = y * 6.166_f32 - x.cos();
        let mut acc = Accumulator0::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.768_f32 + y.sin();
        let b = y * 4.29_f32 - x.cos();
        let mut acc = Accumulator0::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.663_f32 + y.sin();
        let b = y * 9.15_f32 - x.cos();
        let mut acc = Accumulator0::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.038_f32 + y.sin();
        let b = y * 7.885_f32 - x.cos();
        let mut acc = Accumulator0::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.742_f32 + y.sin();
        let b = y * 0.902_f32 - x.cos();
        let mut acc = Accumulator0::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.247_f32 + y.sin();
        let b = y * 8.676_f32 - x.cos();
        let mut acc = Accumulator0::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.499_f32 + y.sin();
        let b = y * 6.068_f32 - x.cos();
        let mut acc = Accumulator0::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_0(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_0() -> f32 {
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
        total += (dep_touch_0(total as u64) % 997) as f32;
        total
    }
}

pub mod m1 {
    use super::*;

    pub struct Accumulator1<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.257_f32 + y.sin();
        let b = y * 4.198_f32 - x.cos();
        let mut acc = Accumulator1::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.452_f32 + y.sin();
        let b = y * 9.246_f32 - x.cos();
        let mut acc = Accumulator1::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.484_f32 + y.sin();
        let b = y * 6.649_f32 - x.cos();
        let mut acc = Accumulator1::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.716_f32 + y.sin();
        let b = y * 8.68_f32 - x.cos();
        let mut acc = Accumulator1::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.433_f32 + y.sin();
        let b = y * 3.532_f32 - x.cos();
        let mut acc = Accumulator1::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.783_f32 + y.sin();
        let b = y * 4.571_f32 - x.cos();
        let mut acc = Accumulator1::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.358_f32 + y.sin();
        let b = y * 4.746_f32 - x.cos();
        let mut acc = Accumulator1::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.737_f32 + y.sin();
        let b = y * 7.265_f32 - x.cos();
        let mut acc = Accumulator1::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.442_f32 + y.sin();
        let b = y * 9.832_f32 - x.cos();
        let mut acc = Accumulator1::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.889_f32 + y.sin();
        let b = y * 3.881_f32 - x.cos();
        let mut acc = Accumulator1::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.501_f32 + y.sin();
        let b = y * 9.318_f32 - x.cos();
        let mut acc = Accumulator1::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.747_f32 + y.sin();
        let b = y * 1.248_f32 - x.cos();
        let mut acc = Accumulator1::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.238_f32 + y.sin();
        let b = y * 2.917_f32 - x.cos();
        let mut acc = Accumulator1::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.527_f32 + y.sin();
        let b = y * 3.931_f32 - x.cos();
        let mut acc = Accumulator1::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.89_f32 + y.sin();
        let b = y * 4.502_f32 - x.cos();
        let mut acc = Accumulator1::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.823_f32 + y.sin();
        let b = y * 1.442_f32 - x.cos();
        let mut acc = Accumulator1::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.567_f32 + y.sin();
        let b = y * 2.829_f32 - x.cos();
        let mut acc = Accumulator1::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.767_f32 + y.sin();
        let b = y * 6.791_f32 - x.cos();
        let mut acc = Accumulator1::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.486_f32 + y.sin();
        let b = y * 1.579_f32 - x.cos();
        let mut acc = Accumulator1::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.583_f32 + y.sin();
        let b = y * 6.553_f32 - x.cos();
        let mut acc = Accumulator1::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m1-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_1() -> f32 {
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
        total += (dep_touch_1(total as u64) % 997) as f32;
        total
    }
}

pub mod m2 {
    use super::*;

    pub struct Accumulator2<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator2<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.853_f32 + y.sin();
        let b = y * 5.873_f32 - x.cos();
        let mut acc = Accumulator2::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.863_f32 + y.sin();
        let b = y * 1.528_f32 - x.cos();
        let mut acc = Accumulator2::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.719_f32 + y.sin();
        let b = y * 5.65_f32 - x.cos();
        let mut acc = Accumulator2::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.867_f32 + y.sin();
        let b = y * 5.152_f32 - x.cos();
        let mut acc = Accumulator2::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.519_f32 + y.sin();
        let b = y * 7.35_f32 - x.cos();
        let mut acc = Accumulator2::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.915_f32 + y.sin();
        let b = y * 7.744_f32 - x.cos();
        let mut acc = Accumulator2::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.919_f32 + y.sin();
        let b = y * 3.945_f32 - x.cos();
        let mut acc = Accumulator2::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.962_f32 + y.sin();
        let b = y * 4.819_f32 - x.cos();
        let mut acc = Accumulator2::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.71_f32 + y.sin();
        let b = y * 0.76_f32 - x.cos();
        let mut acc = Accumulator2::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.418_f32 + y.sin();
        let b = y * 1.177_f32 - x.cos();
        let mut acc = Accumulator2::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.615_f32 + y.sin();
        let b = y * 0.102_f32 - x.cos();
        let mut acc = Accumulator2::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.359_f32 + y.sin();
        let b = y * 9.4_f32 - x.cos();
        let mut acc = Accumulator2::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.35_f32 + y.sin();
        let b = y * 8.668_f32 - x.cos();
        let mut acc = Accumulator2::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.787_f32 + y.sin();
        let b = y * 6.317_f32 - x.cos();
        let mut acc = Accumulator2::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.002_f32 + y.sin();
        let b = y * 4.747_f32 - x.cos();
        let mut acc = Accumulator2::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.42_f32 + y.sin();
        let b = y * 9.832_f32 - x.cos();
        let mut acc = Accumulator2::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.808_f32 + y.sin();
        let b = y * 3.156_f32 - x.cos();
        let mut acc = Accumulator2::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.101_f32 + y.sin();
        let b = y * 3.458_f32 - x.cos();
        let mut acc = Accumulator2::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.79_f32 + y.sin();
        let b = y * 6.882_f32 - x.cos();
        let mut acc = Accumulator2::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.326_f32 + y.sin();
        let b = y * 9.42_f32 - x.cos();
        let mut acc = Accumulator2::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_2(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_2() -> f32 {
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
        total += (dep_touch_2(total as u64) % 997) as f32;
        total
    }
}

pub mod m3 {
    use super::*;

    pub struct Accumulator3<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator3<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.645_f32 + y.sin();
        let b = y * 6.863_f32 - x.cos();
        let mut acc = Accumulator3::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.53_f32 + y.sin();
        let b = y * 3.021_f32 - x.cos();
        let mut acc = Accumulator3::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.561_f32 + y.sin();
        let b = y * 6.923_f32 - x.cos();
        let mut acc = Accumulator3::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.18_f32 + y.sin();
        let b = y * 9.001_f32 - x.cos();
        let mut acc = Accumulator3::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.665_f32 + y.sin();
        let b = y * 5.319_f32 - x.cos();
        let mut acc = Accumulator3::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.331_f32 + y.sin();
        let b = y * 2.286_f32 - x.cos();
        let mut acc = Accumulator3::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.0_f32 + y.sin();
        let b = y * 8.12_f32 - x.cos();
        let mut acc = Accumulator3::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.973_f32 + y.sin();
        let b = y * 2.059_f32 - x.cos();
        let mut acc = Accumulator3::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.585_f32 + y.sin();
        let b = y * 0.384_f32 - x.cos();
        let mut acc = Accumulator3::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.843_f32 + y.sin();
        let b = y * 4.728_f32 - x.cos();
        let mut acc = Accumulator3::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.887_f32 + y.sin();
        let b = y * 9.474_f32 - x.cos();
        let mut acc = Accumulator3::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.024_f32 + y.sin();
        let b = y * 7.187_f32 - x.cos();
        let mut acc = Accumulator3::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.459_f32 + y.sin();
        let b = y * 3.673_f32 - x.cos();
        let mut acc = Accumulator3::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.101_f32 + y.sin();
        let b = y * 4.707_f32 - x.cos();
        let mut acc = Accumulator3::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.103_f32 + y.sin();
        let b = y * 6.216_f32 - x.cos();
        let mut acc = Accumulator3::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.336_f32 + y.sin();
        let b = y * 4.799_f32 - x.cos();
        let mut acc = Accumulator3::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.471_f32 + y.sin();
        let b = y * 6.403_f32 - x.cos();
        let mut acc = Accumulator3::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.275_f32 + y.sin();
        let b = y * 3.908_f32 - x.cos();
        let mut acc = Accumulator3::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.451_f32 + y.sin();
        let b = y * 4.785_f32 - x.cos();
        let mut acc = Accumulator3::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.352_f32 + y.sin();
        let b = y * 6.331_f32 - x.cos();
        let mut acc = Accumulator3::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_3(seed: u64) -> u64 {
        let re = Regex::new(r"m3-(\d+)").unwrap();
        let hay = format!("m3-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_3() -> f32 {
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
        total += (dep_touch_3(total as u64) % 997) as f32;
        total
    }
}

pub mod m4 {
    use super::*;

    pub struct Accumulator4<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator4<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.948_f32 + y.sin();
        let b = y * 9.622_f32 - x.cos();
        let mut acc = Accumulator4::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.639_f32 + y.sin();
        let b = y * 7.385_f32 - x.cos();
        let mut acc = Accumulator4::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.203_f32 + y.sin();
        let b = y * 1.766_f32 - x.cos();
        let mut acc = Accumulator4::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.37_f32 + y.sin();
        let b = y * 5.89_f32 - x.cos();
        let mut acc = Accumulator4::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.004_f32 + y.sin();
        let b = y * 1.533_f32 - x.cos();
        let mut acc = Accumulator4::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.707_f32 + y.sin();
        let b = y * 6.541_f32 - x.cos();
        let mut acc = Accumulator4::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.628_f32 + y.sin();
        let b = y * 5.473_f32 - x.cos();
        let mut acc = Accumulator4::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.24_f32 + y.sin();
        let b = y * 9.615_f32 - x.cos();
        let mut acc = Accumulator4::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.107_f32 + y.sin();
        let b = y * 7.445_f32 - x.cos();
        let mut acc = Accumulator4::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.351_f32 + y.sin();
        let b = y * 8.643_f32 - x.cos();
        let mut acc = Accumulator4::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.374_f32 + y.sin();
        let b = y * 2.185_f32 - x.cos();
        let mut acc = Accumulator4::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.457_f32 + y.sin();
        let b = y * 5.847_f32 - x.cos();
        let mut acc = Accumulator4::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.435_f32 + y.sin();
        let b = y * 8.275_f32 - x.cos();
        let mut acc = Accumulator4::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.018_f32 + y.sin();
        let b = y * 3.567_f32 - x.cos();
        let mut acc = Accumulator4::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.592_f32 + y.sin();
        let b = y * 8.087_f32 - x.cos();
        let mut acc = Accumulator4::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.222_f32 + y.sin();
        let b = y * 9.094_f32 - x.cos();
        let mut acc = Accumulator4::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.381_f32 + y.sin();
        let b = y * 1.588_f32 - x.cos();
        let mut acc = Accumulator4::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.283_f32 + y.sin();
        let b = y * 4.413_f32 - x.cos();
        let mut acc = Accumulator4::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.064_f32 + y.sin();
        let b = y * 7.705_f32 - x.cos();
        let mut acc = Accumulator4::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.789_f32 + y.sin();
        let b = y * 4.74_f32 - x.cos();
        let mut acc = Accumulator4::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_4(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_4() -> f32 {
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
        total += (dep_touch_4(total as u64) % 997) as f32;
        total
    }
}

pub mod m5 {
    use super::*;

    pub struct Accumulator5<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator5<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.279_f32 + y.sin();
        let b = y * 0.705_f32 - x.cos();
        let mut acc = Accumulator5::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.18_f32 + y.sin();
        let b = y * 5.543_f32 - x.cos();
        let mut acc = Accumulator5::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.756_f32 + y.sin();
        let b = y * 0.657_f32 - x.cos();
        let mut acc = Accumulator5::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.814_f32 + y.sin();
        let b = y * 7.668_f32 - x.cos();
        let mut acc = Accumulator5::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.531_f32 + y.sin();
        let b = y * 0.373_f32 - x.cos();
        let mut acc = Accumulator5::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.444_f32 + y.sin();
        let b = y * 6.103_f32 - x.cos();
        let mut acc = Accumulator5::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.04_f32 + y.sin();
        let b = y * 2.054_f32 - x.cos();
        let mut acc = Accumulator5::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.533_f32 + y.sin();
        let b = y * 5.326_f32 - x.cos();
        let mut acc = Accumulator5::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.076_f32 + y.sin();
        let b = y * 2.527_f32 - x.cos();
        let mut acc = Accumulator5::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.69_f32 + y.sin();
        let b = y * 9.333_f32 - x.cos();
        let mut acc = Accumulator5::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.143_f32 + y.sin();
        let b = y * 8.849_f32 - x.cos();
        let mut acc = Accumulator5::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.332_f32 + y.sin();
        let b = y * 1.444_f32 - x.cos();
        let mut acc = Accumulator5::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.945_f32 + y.sin();
        let b = y * 3.197_f32 - x.cos();
        let mut acc = Accumulator5::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.458_f32 + y.sin();
        let b = y * 0.817_f32 - x.cos();
        let mut acc = Accumulator5::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.067_f32 + y.sin();
        let b = y * 1.299_f32 - x.cos();
        let mut acc = Accumulator5::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.307_f32 + y.sin();
        let b = y * 6.406_f32 - x.cos();
        let mut acc = Accumulator5::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.501_f32 + y.sin();
        let b = y * 8.752_f32 - x.cos();
        let mut acc = Accumulator5::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.252_f32 + y.sin();
        let b = y * 9.435_f32 - x.cos();
        let mut acc = Accumulator5::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.772_f32 + y.sin();
        let b = y * 1.695_f32 - x.cos();
        let mut acc = Accumulator5::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.258_f32 + y.sin();
        let b = y * 1.682_f32 - x.cos();
        let mut acc = Accumulator5::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_5(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(5u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_5() -> f32 {
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
        total += (dep_touch_5(total as u64) % 997) as f32;
        total
    }
}

pub mod m6 {
    use super::*;

    pub struct Accumulator6<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator6<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.842_f32 + y.sin();
        let b = y * 4.057_f32 - x.cos();
        let mut acc = Accumulator6::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.018_f32 + y.sin();
        let b = y * 3.222_f32 - x.cos();
        let mut acc = Accumulator6::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.686_f32 + y.sin();
        let b = y * 3.412_f32 - x.cos();
        let mut acc = Accumulator6::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.416_f32 + y.sin();
        let b = y * 0.277_f32 - x.cos();
        let mut acc = Accumulator6::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.171_f32 + y.sin();
        let b = y * 2.995_f32 - x.cos();
        let mut acc = Accumulator6::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.206_f32 + y.sin();
        let b = y * 9.102_f32 - x.cos();
        let mut acc = Accumulator6::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.623_f32 + y.sin();
        let b = y * 1.127_f32 - x.cos();
        let mut acc = Accumulator6::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.765_f32 + y.sin();
        let b = y * 8.978_f32 - x.cos();
        let mut acc = Accumulator6::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.75_f32 + y.sin();
        let b = y * 1.37_f32 - x.cos();
        let mut acc = Accumulator6::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.426_f32 + y.sin();
        let b = y * 6.725_f32 - x.cos();
        let mut acc = Accumulator6::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.078_f32 + y.sin();
        let b = y * 5.359_f32 - x.cos();
        let mut acc = Accumulator6::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.692_f32 + y.sin();
        let b = y * 6.964_f32 - x.cos();
        let mut acc = Accumulator6::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.835_f32 + y.sin();
        let b = y * 7.936_f32 - x.cos();
        let mut acc = Accumulator6::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.268_f32 + y.sin();
        let b = y * 0.81_f32 - x.cos();
        let mut acc = Accumulator6::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.318_f32 + y.sin();
        let b = y * 7.956_f32 - x.cos();
        let mut acc = Accumulator6::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.06_f32 + y.sin();
        let b = y * 2.28_f32 - x.cos();
        let mut acc = Accumulator6::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.555_f32 + y.sin();
        let b = y * 4.547_f32 - x.cos();
        let mut acc = Accumulator6::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.844_f32 + y.sin();
        let b = y * 4.194_f32 - x.cos();
        let mut acc = Accumulator6::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.193_f32 + y.sin();
        let b = y * 0.523_f32 - x.cos();
        let mut acc = Accumulator6::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.437_f32 + y.sin();
        let b = y * 1.173_f32 - x.cos();
        let mut acc = Accumulator6::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_6(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_6() -> f32 {
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
        total += (dep_touch_6(total as u64) % 997) as f32;
        total
    }
}

pub mod m7 {
    use super::*;

    pub struct Accumulator7<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator7<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.667_f32 + y.sin();
        let b = y * 1.875_f32 - x.cos();
        let mut acc = Accumulator7::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.261_f32 + y.sin();
        let b = y * 5.305_f32 - x.cos();
        let mut acc = Accumulator7::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.942_f32 + y.sin();
        let b = y * 5.001_f32 - x.cos();
        let mut acc = Accumulator7::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.751_f32 + y.sin();
        let b = y * 7.976_f32 - x.cos();
        let mut acc = Accumulator7::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.462_f32 + y.sin();
        let b = y * 0.281_f32 - x.cos();
        let mut acc = Accumulator7::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.5_f32 + y.sin();
        let b = y * 1.957_f32 - x.cos();
        let mut acc = Accumulator7::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.508_f32 + y.sin();
        let b = y * 4.481_f32 - x.cos();
        let mut acc = Accumulator7::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.125_f32 + y.sin();
        let b = y * 4.335_f32 - x.cos();
        let mut acc = Accumulator7::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.45_f32 + y.sin();
        let b = y * 8.81_f32 - x.cos();
        let mut acc = Accumulator7::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.116_f32 + y.sin();
        let b = y * 2.209_f32 - x.cos();
        let mut acc = Accumulator7::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.459_f32 + y.sin();
        let b = y * 8.256_f32 - x.cos();
        let mut acc = Accumulator7::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.243_f32 + y.sin();
        let b = y * 1.469_f32 - x.cos();
        let mut acc = Accumulator7::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.722_f32 + y.sin();
        let b = y * 8.302_f32 - x.cos();
        let mut acc = Accumulator7::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.793_f32 + y.sin();
        let b = y * 7.361_f32 - x.cos();
        let mut acc = Accumulator7::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.321_f32 + y.sin();
        let b = y * 0.643_f32 - x.cos();
        let mut acc = Accumulator7::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.344_f32 + y.sin();
        let b = y * 8.631_f32 - x.cos();
        let mut acc = Accumulator7::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.615_f32 + y.sin();
        let b = y * 5.968_f32 - x.cos();
        let mut acc = Accumulator7::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.972_f32 + y.sin();
        let b = y * 4.603_f32 - x.cos();
        let mut acc = Accumulator7::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.737_f32 + y.sin();
        let b = y * 0.136_f32 - x.cos();
        let mut acc = Accumulator7::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.526_f32 + y.sin();
        let b = y * 9.632_f32 - x.cos();
        let mut acc = Accumulator7::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_7(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_7() -> f32 {
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
        total += (dep_touch_7(total as u64) % 997) as f32;
        total
    }
}

pub mod m8 {
    use super::*;

    pub struct Accumulator8<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator8<T> {
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
        let b = y * 0.438_f32 - x.cos();
        let mut acc = Accumulator8::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.235_f32 + y.sin();
        let b = y * 1.893_f32 - x.cos();
        let mut acc = Accumulator8::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.84_f32 + y.sin();
        let b = y * 4.752_f32 - x.cos();
        let mut acc = Accumulator8::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.529_f32 + y.sin();
        let b = y * 2.532_f32 - x.cos();
        let mut acc = Accumulator8::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.99_f32 + y.sin();
        let b = y * 8.107_f32 - x.cos();
        let mut acc = Accumulator8::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.015_f32 + y.sin();
        let b = y * 0.508_f32 - x.cos();
        let mut acc = Accumulator8::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.037_f32 + y.sin();
        let b = y * 6.271_f32 - x.cos();
        let mut acc = Accumulator8::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.839_f32 + y.sin();
        let b = y * 5.286_f32 - x.cos();
        let mut acc = Accumulator8::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.621_f32 + y.sin();
        let b = y * 8.849_f32 - x.cos();
        let mut acc = Accumulator8::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.917_f32 + y.sin();
        let b = y * 3.296_f32 - x.cos();
        let mut acc = Accumulator8::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.565_f32 + y.sin();
        let b = y * 7.197_f32 - x.cos();
        let mut acc = Accumulator8::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.519_f32 + y.sin();
        let b = y * 8.184_f32 - x.cos();
        let mut acc = Accumulator8::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.841_f32 + y.sin();
        let b = y * 6.248_f32 - x.cos();
        let mut acc = Accumulator8::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.97_f32 + y.sin();
        let b = y * 5.054_f32 - x.cos();
        let mut acc = Accumulator8::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.478_f32 + y.sin();
        let b = y * 5.671_f32 - x.cos();
        let mut acc = Accumulator8::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.199_f32 + y.sin();
        let b = y * 5.824_f32 - x.cos();
        let mut acc = Accumulator8::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.792_f32 + y.sin();
        let b = y * 6.895_f32 - x.cos();
        let mut acc = Accumulator8::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.934_f32 + y.sin();
        let b = y * 0.51_f32 - x.cos();
        let mut acc = Accumulator8::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.635_f32 + y.sin();
        let b = y * 1.128_f32 - x.cos();
        let mut acc = Accumulator8::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.574_f32 + y.sin();
        let b = y * 6.252_f32 - x.cos();
        let mut acc = Accumulator8::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_8(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m8-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_8() -> f32 {
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
        total += (dep_touch_8(total as u64) % 997) as f32;
        total
    }
}

pub mod m9 {
    use super::*;

    pub struct Accumulator9<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator9<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.308_f32 + y.sin();
        let b = y * 2.497_f32 - x.cos();
        let mut acc = Accumulator9::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.132_f32 + y.sin();
        let b = y * 7.917_f32 - x.cos();
        let mut acc = Accumulator9::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.239_f32 + y.sin();
        let b = y * 8.899_f32 - x.cos();
        let mut acc = Accumulator9::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.561_f32 + y.sin();
        let b = y * 0.747_f32 - x.cos();
        let mut acc = Accumulator9::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.744_f32 + y.sin();
        let b = y * 8.03_f32 - x.cos();
        let mut acc = Accumulator9::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.401_f32 + y.sin();
        let b = y * 7.513_f32 - x.cos();
        let mut acc = Accumulator9::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.35_f32 + y.sin();
        let b = y * 9.662_f32 - x.cos();
        let mut acc = Accumulator9::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.386_f32 + y.sin();
        let b = y * 0.852_f32 - x.cos();
        let mut acc = Accumulator9::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.916_f32 + y.sin();
        let b = y * 0.558_f32 - x.cos();
        let mut acc = Accumulator9::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.399_f32 + y.sin();
        let b = y * 0.859_f32 - x.cos();
        let mut acc = Accumulator9::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.351_f32 + y.sin();
        let b = y * 6.485_f32 - x.cos();
        let mut acc = Accumulator9::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.083_f32 + y.sin();
        let b = y * 5.664_f32 - x.cos();
        let mut acc = Accumulator9::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.828_f32 + y.sin();
        let b = y * 4.861_f32 - x.cos();
        let mut acc = Accumulator9::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.075_f32 + y.sin();
        let b = y * 2.233_f32 - x.cos();
        let mut acc = Accumulator9::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.95_f32 + y.sin();
        let b = y * 5.162_f32 - x.cos();
        let mut acc = Accumulator9::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.666_f32 + y.sin();
        let b = y * 7.618_f32 - x.cos();
        let mut acc = Accumulator9::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.053_f32 + y.sin();
        let b = y * 9.686_f32 - x.cos();
        let mut acc = Accumulator9::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.272_f32 + y.sin();
        let b = y * 4.598_f32 - x.cos();
        let mut acc = Accumulator9::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.587_f32 + y.sin();
        let b = y * 4.505_f32 - x.cos();
        let mut acc = Accumulator9::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.891_f32 + y.sin();
        let b = y * 9.082_f32 - x.cos();
        let mut acc = Accumulator9::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_9(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_9() -> f32 {
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
        total += (dep_touch_9(total as u64) % 997) as f32;
        total
    }
}

pub mod m10 {
    use super::*;

    pub struct Accumulator10<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator10<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.831_f32 + y.sin();
        let b = y * 0.985_f32 - x.cos();
        let mut acc = Accumulator10::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.236_f32 + y.sin();
        let b = y * 9.437_f32 - x.cos();
        let mut acc = Accumulator10::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.013_f32 + y.sin();
        let b = y * 6.29_f32 - x.cos();
        let mut acc = Accumulator10::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.791_f32 + y.sin();
        let b = y * 6.993_f32 - x.cos();
        let mut acc = Accumulator10::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.979_f32 + y.sin();
        let b = y * 8.686_f32 - x.cos();
        let mut acc = Accumulator10::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.343_f32 + y.sin();
        let b = y * 0.135_f32 - x.cos();
        let mut acc = Accumulator10::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.78_f32 + y.sin();
        let b = y * 4.073_f32 - x.cos();
        let mut acc = Accumulator10::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.479_f32 + y.sin();
        let b = y * 3.471_f32 - x.cos();
        let mut acc = Accumulator10::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.285_f32 + y.sin();
        let b = y * 3.347_f32 - x.cos();
        let mut acc = Accumulator10::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.457_f32 + y.sin();
        let b = y * 8.323_f32 - x.cos();
        let mut acc = Accumulator10::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.311_f32 + y.sin();
        let b = y * 2.018_f32 - x.cos();
        let mut acc = Accumulator10::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.935_f32 + y.sin();
        let b = y * 2.94_f32 - x.cos();
        let mut acc = Accumulator10::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.737_f32 + y.sin();
        let b = y * 3.924_f32 - x.cos();
        let mut acc = Accumulator10::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.849_f32 + y.sin();
        let b = y * 9.169_f32 - x.cos();
        let mut acc = Accumulator10::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.797_f32 + y.sin();
        let b = y * 0.573_f32 - x.cos();
        let mut acc = Accumulator10::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.606_f32 + y.sin();
        let b = y * 6.587_f32 - x.cos();
        let mut acc = Accumulator10::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.269_f32 + y.sin();
        let b = y * 2.543_f32 - x.cos();
        let mut acc = Accumulator10::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.375_f32 + y.sin();
        let b = y * 3.193_f32 - x.cos();
        let mut acc = Accumulator10::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.794_f32 + y.sin();
        let b = y * 4.292_f32 - x.cos();
        let mut acc = Accumulator10::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.057_f32 + y.sin();
        let b = y * 6.283_f32 - x.cos();
        let mut acc = Accumulator10::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_10(seed: u64) -> u64 {
        let re = Regex::new(r"m10-(\d+)").unwrap();
        let hay = format!("m10-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_10() -> f32 {
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
        total += (dep_touch_10(total as u64) % 997) as f32;
        total
    }
}

pub mod m11 {
    use super::*;

    pub struct Accumulator11<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator11<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.482_f32 + y.sin();
        let b = y * 7.152_f32 - x.cos();
        let mut acc = Accumulator11::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.248_f32 + y.sin();
        let b = y * 4.127_f32 - x.cos();
        let mut acc = Accumulator11::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.476_f32 + y.sin();
        let b = y * 6.416_f32 - x.cos();
        let mut acc = Accumulator11::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.859_f32 + y.sin();
        let b = y * 9.037_f32 - x.cos();
        let mut acc = Accumulator11::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.348_f32 + y.sin();
        let b = y * 4.727_f32 - x.cos();
        let mut acc = Accumulator11::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.861_f32 + y.sin();
        let b = y * 2.606_f32 - x.cos();
        let mut acc = Accumulator11::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.668_f32 + y.sin();
        let b = y * 2.65_f32 - x.cos();
        let mut acc = Accumulator11::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.439_f32 + y.sin();
        let b = y * 4.835_f32 - x.cos();
        let mut acc = Accumulator11::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.965_f32 + y.sin();
        let b = y * 1.74_f32 - x.cos();
        let mut acc = Accumulator11::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.837_f32 + y.sin();
        let b = y * 5.006_f32 - x.cos();
        let mut acc = Accumulator11::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.494_f32 + y.sin();
        let b = y * 4.539_f32 - x.cos();
        let mut acc = Accumulator11::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.865_f32 + y.sin();
        let b = y * 4.51_f32 - x.cos();
        let mut acc = Accumulator11::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.468_f32 + y.sin();
        let b = y * 2.492_f32 - x.cos();
        let mut acc = Accumulator11::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.451_f32 + y.sin();
        let b = y * 0.993_f32 - x.cos();
        let mut acc = Accumulator11::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.709_f32 + y.sin();
        let b = y * 8.032_f32 - x.cos();
        let mut acc = Accumulator11::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.795_f32 + y.sin();
        let b = y * 7.447_f32 - x.cos();
        let mut acc = Accumulator11::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.852_f32 + y.sin();
        let b = y * 7.409_f32 - x.cos();
        let mut acc = Accumulator11::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.793_f32 + y.sin();
        let b = y * 3.414_f32 - x.cos();
        let mut acc = Accumulator11::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.982_f32 + y.sin();
        let b = y * 5.728_f32 - x.cos();
        let mut acc = Accumulator11::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.334_f32 + y.sin();
        let b = y * 5.033_f32 - x.cos();
        let mut acc = Accumulator11::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_11(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_11() -> f32 {
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
        total += (dep_touch_11(total as u64) % 997) as f32;
        total
    }
}

pub mod m12 {
    use super::*;

    pub struct Accumulator12<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator12<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.845_f32 + y.sin();
        let b = y * 8.417_f32 - x.cos();
        let mut acc = Accumulator12::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.756_f32 + y.sin();
        let b = y * 2.535_f32 - x.cos();
        let mut acc = Accumulator12::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.429_f32 + y.sin();
        let b = y * 4.332_f32 - x.cos();
        let mut acc = Accumulator12::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.417_f32 + y.sin();
        let b = y * 8.654_f32 - x.cos();
        let mut acc = Accumulator12::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.347_f32 + y.sin();
        let b = y * 4.267_f32 - x.cos();
        let mut acc = Accumulator12::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.589_f32 + y.sin();
        let b = y * 4.9_f32 - x.cos();
        let mut acc = Accumulator12::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.937_f32 + y.sin();
        let b = y * 9.183_f32 - x.cos();
        let mut acc = Accumulator12::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.484_f32 + y.sin();
        let b = y * 9.628_f32 - x.cos();
        let mut acc = Accumulator12::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.774_f32 + y.sin();
        let b = y * 2.293_f32 - x.cos();
        let mut acc = Accumulator12::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.219_f32 + y.sin();
        let b = y * 6.784_f32 - x.cos();
        let mut acc = Accumulator12::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.97_f32 + y.sin();
        let b = y * 8.396_f32 - x.cos();
        let mut acc = Accumulator12::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.933_f32 + y.sin();
        let b = y * 7.713_f32 - x.cos();
        let mut acc = Accumulator12::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.767_f32 + y.sin();
        let b = y * 2.379_f32 - x.cos();
        let mut acc = Accumulator12::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.426_f32 + y.sin();
        let b = y * 3.077_f32 - x.cos();
        let mut acc = Accumulator12::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.239_f32 + y.sin();
        let b = y * 5.277_f32 - x.cos();
        let mut acc = Accumulator12::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.946_f32 + y.sin();
        let b = y * 1.199_f32 - x.cos();
        let mut acc = Accumulator12::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.043_f32 + y.sin();
        let b = y * 9.347_f32 - x.cos();
        let mut acc = Accumulator12::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.903_f32 + y.sin();
        let b = y * 2.291_f32 - x.cos();
        let mut acc = Accumulator12::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.111_f32 + y.sin();
        let b = y * 5.367_f32 - x.cos();
        let mut acc = Accumulator12::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.83_f32 + y.sin();
        let b = y * 3.2_f32 - x.cos();
        let mut acc = Accumulator12::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_12(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(12u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_12() -> f32 {
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
        total += (dep_touch_12(total as u64) % 997) as f32;
        total
    }
}

pub mod m13 {
    use super::*;

    pub struct Accumulator13<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator13<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.758_f32 + y.sin();
        let b = y * 2.401_f32 - x.cos();
        let mut acc = Accumulator13::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.387_f32 + y.sin();
        let b = y * 4.136_f32 - x.cos();
        let mut acc = Accumulator13::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.112_f32 + y.sin();
        let b = y * 0.314_f32 - x.cos();
        let mut acc = Accumulator13::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.772_f32 + y.sin();
        let b = y * 6.442_f32 - x.cos();
        let mut acc = Accumulator13::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.621_f32 + y.sin();
        let b = y * 6.64_f32 - x.cos();
        let mut acc = Accumulator13::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.323_f32 + y.sin();
        let b = y * 0.434_f32 - x.cos();
        let mut acc = Accumulator13::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.14_f32 + y.sin();
        let b = y * 3.651_f32 - x.cos();
        let mut acc = Accumulator13::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.041_f32 + y.sin();
        let b = y * 7.911_f32 - x.cos();
        let mut acc = Accumulator13::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.382_f32 + y.sin();
        let b = y * 0.761_f32 - x.cos();
        let mut acc = Accumulator13::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.605_f32 + y.sin();
        let b = y * 3.155_f32 - x.cos();
        let mut acc = Accumulator13::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.362_f32 + y.sin();
        let b = y * 2.27_f32 - x.cos();
        let mut acc = Accumulator13::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.168_f32 + y.sin();
        let b = y * 6.211_f32 - x.cos();
        let mut acc = Accumulator13::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.936_f32 + y.sin();
        let b = y * 2.289_f32 - x.cos();
        let mut acc = Accumulator13::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.022_f32 + y.sin();
        let b = y * 0.653_f32 - x.cos();
        let mut acc = Accumulator13::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.535_f32 + y.sin();
        let b = y * 3.956_f32 - x.cos();
        let mut acc = Accumulator13::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.332_f32 + y.sin();
        let b = y * 5.942_f32 - x.cos();
        let mut acc = Accumulator13::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.608_f32 + y.sin();
        let b = y * 0.689_f32 - x.cos();
        let mut acc = Accumulator13::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.506_f32 + y.sin();
        let b = y * 7.078_f32 - x.cos();
        let mut acc = Accumulator13::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.281_f32 + y.sin();
        let b = y * 9.876_f32 - x.cos();
        let mut acc = Accumulator13::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.327_f32 + y.sin();
        let b = y * 1.918_f32 - x.cos();
        let mut acc = Accumulator13::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_13(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_13() -> f32 {
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
        total += (dep_touch_13(total as u64) % 997) as f32;
        total
    }
}

pub mod m14 {
    use super::*;

    pub struct Accumulator14<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator14<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.414_f32 + y.sin();
        let b = y * 0.413_f32 - x.cos();
        let mut acc = Accumulator14::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.209_f32 + y.sin();
        let b = y * 8.323_f32 - x.cos();
        let mut acc = Accumulator14::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.436_f32 + y.sin();
        let b = y * 1.168_f32 - x.cos();
        let mut acc = Accumulator14::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.842_f32 + y.sin();
        let b = y * 3.544_f32 - x.cos();
        let mut acc = Accumulator14::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.599_f32 + y.sin();
        let b = y * 7.536_f32 - x.cos();
        let mut acc = Accumulator14::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.595_f32 + y.sin();
        let b = y * 8.151_f32 - x.cos();
        let mut acc = Accumulator14::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.96_f32 + y.sin();
        let b = y * 7.012_f32 - x.cos();
        let mut acc = Accumulator14::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.753_f32 + y.sin();
        let b = y * 9.111_f32 - x.cos();
        let mut acc = Accumulator14::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.268_f32 + y.sin();
        let b = y * 7.326_f32 - x.cos();
        let mut acc = Accumulator14::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.397_f32 + y.sin();
        let b = y * 4.126_f32 - x.cos();
        let mut acc = Accumulator14::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.613_f32 + y.sin();
        let b = y * 0.498_f32 - x.cos();
        let mut acc = Accumulator14::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.648_f32 + y.sin();
        let b = y * 7.973_f32 - x.cos();
        let mut acc = Accumulator14::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.619_f32 + y.sin();
        let b = y * 7.423_f32 - x.cos();
        let mut acc = Accumulator14::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.423_f32 + y.sin();
        let b = y * 2.769_f32 - x.cos();
        let mut acc = Accumulator14::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.527_f32 + y.sin();
        let b = y * 7.415_f32 - x.cos();
        let mut acc = Accumulator14::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.202_f32 + y.sin();
        let b = y * 2.801_f32 - x.cos();
        let mut acc = Accumulator14::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.171_f32 + y.sin();
        let b = y * 5.937_f32 - x.cos();
        let mut acc = Accumulator14::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.376_f32 + y.sin();
        let b = y * 0.74_f32 - x.cos();
        let mut acc = Accumulator14::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.151_f32 + y.sin();
        let b = y * 7.113_f32 - x.cos();
        let mut acc = Accumulator14::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.448_f32 + y.sin();
        let b = y * 3.888_f32 - x.cos();
        let mut acc = Accumulator14::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_14(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_14() -> f32 {
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
        total += (dep_touch_14(total as u64) % 997) as f32;
        total
    }
}

pub mod m15 {
    use super::*;

    pub struct Accumulator15<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator15<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.053_f32 + y.sin();
        let b = y * 8.085_f32 - x.cos();
        let mut acc = Accumulator15::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.195_f32 + y.sin();
        let b = y * 1.893_f32 - x.cos();
        let mut acc = Accumulator15::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.072_f32 + y.sin();
        let b = y * 6.883_f32 - x.cos();
        let mut acc = Accumulator15::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.051_f32 + y.sin();
        let b = y * 3.312_f32 - x.cos();
        let mut acc = Accumulator15::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.616_f32 + y.sin();
        let b = y * 7.782_f32 - x.cos();
        let mut acc = Accumulator15::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.874_f32 + y.sin();
        let b = y * 2.034_f32 - x.cos();
        let mut acc = Accumulator15::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.667_f32 + y.sin();
        let b = y * 4.096_f32 - x.cos();
        let mut acc = Accumulator15::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.432_f32 + y.sin();
        let b = y * 5.515_f32 - x.cos();
        let mut acc = Accumulator15::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.675_f32 + y.sin();
        let b = y * 4.28_f32 - x.cos();
        let mut acc = Accumulator15::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.781_f32 + y.sin();
        let b = y * 2.696_f32 - x.cos();
        let mut acc = Accumulator15::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.142_f32 + y.sin();
        let b = y * 4.226_f32 - x.cos();
        let mut acc = Accumulator15::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.627_f32 + y.sin();
        let b = y * 1.797_f32 - x.cos();
        let mut acc = Accumulator15::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.185_f32 + y.sin();
        let b = y * 6.179_f32 - x.cos();
        let mut acc = Accumulator15::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.402_f32 + y.sin();
        let b = y * 5.378_f32 - x.cos();
        let mut acc = Accumulator15::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.544_f32 + y.sin();
        let b = y * 7.742_f32 - x.cos();
        let mut acc = Accumulator15::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.979_f32 + y.sin();
        let b = y * 5.655_f32 - x.cos();
        let mut acc = Accumulator15::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.59_f32 + y.sin();
        let b = y * 2.651_f32 - x.cos();
        let mut acc = Accumulator15::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.525_f32 + y.sin();
        let b = y * 2.504_f32 - x.cos();
        let mut acc = Accumulator15::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.857_f32 + y.sin();
        let b = y * 8.994_f32 - x.cos();
        let mut acc = Accumulator15::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.298_f32 + y.sin();
        let b = y * 3.981_f32 - x.cos();
        let mut acc = Accumulator15::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_15(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m15-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_15() -> f32 {
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
        total += (dep_touch_15(total as u64) % 997) as f32;
        total
    }
}

pub mod m16 {
    use super::*;

    pub struct Accumulator16<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator16<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.072_f32 + y.sin();
        let b = y * 2.368_f32 - x.cos();
        let mut acc = Accumulator16::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.503_f32 + y.sin();
        let b = y * 9.811_f32 - x.cos();
        let mut acc = Accumulator16::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.144_f32 + y.sin();
        let b = y * 8.752_f32 - x.cos();
        let mut acc = Accumulator16::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.337_f32 + y.sin();
        let b = y * 9.061_f32 - x.cos();
        let mut acc = Accumulator16::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.693_f32 + y.sin();
        let b = y * 2.382_f32 - x.cos();
        let mut acc = Accumulator16::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.958_f32 + y.sin();
        let b = y * 9.635_f32 - x.cos();
        let mut acc = Accumulator16::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.003_f32 + y.sin();
        let b = y * 0.836_f32 - x.cos();
        let mut acc = Accumulator16::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.588_f32 + y.sin();
        let b = y * 4.501_f32 - x.cos();
        let mut acc = Accumulator16::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.695_f32 + y.sin();
        let b = y * 6.615_f32 - x.cos();
        let mut acc = Accumulator16::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.137_f32 + y.sin();
        let b = y * 5.942_f32 - x.cos();
        let mut acc = Accumulator16::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.527_f32 + y.sin();
        let b = y * 0.467_f32 - x.cos();
        let mut acc = Accumulator16::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.485_f32 + y.sin();
        let b = y * 2.099_f32 - x.cos();
        let mut acc = Accumulator16::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.475_f32 + y.sin();
        let b = y * 7.276_f32 - x.cos();
        let mut acc = Accumulator16::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.084_f32 + y.sin();
        let b = y * 8.125_f32 - x.cos();
        let mut acc = Accumulator16::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.748_f32 + y.sin();
        let b = y * 1.914_f32 - x.cos();
        let mut acc = Accumulator16::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.864_f32 + y.sin();
        let b = y * 0.408_f32 - x.cos();
        let mut acc = Accumulator16::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.471_f32 + y.sin();
        let b = y * 0.72_f32 - x.cos();
        let mut acc = Accumulator16::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.899_f32 + y.sin();
        let b = y * 6.607_f32 - x.cos();
        let mut acc = Accumulator16::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.364_f32 + y.sin();
        let b = y * 0.993_f32 - x.cos();
        let mut acc = Accumulator16::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.998_f32 + y.sin();
        let b = y * 2.757_f32 - x.cos();
        let mut acc = Accumulator16::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_16(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_16() -> f32 {
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
        total += (dep_touch_16(total as u64) % 997) as f32;
        total
    }
}

pub mod m17 {
    use super::*;

    pub struct Accumulator17<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator17<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.645_f32 + y.sin();
        let b = y * 4.195_f32 - x.cos();
        let mut acc = Accumulator17::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.161_f32 + y.sin();
        let b = y * 5.652_f32 - x.cos();
        let mut acc = Accumulator17::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.158_f32 + y.sin();
        let b = y * 0.278_f32 - x.cos();
        let mut acc = Accumulator17::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.416_f32 + y.sin();
        let b = y * 3.929_f32 - x.cos();
        let mut acc = Accumulator17::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.096_f32 + y.sin();
        let b = y * 0.158_f32 - x.cos();
        let mut acc = Accumulator17::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.253_f32 + y.sin();
        let b = y * 8.14_f32 - x.cos();
        let mut acc = Accumulator17::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.762_f32 + y.sin();
        let b = y * 3.674_f32 - x.cos();
        let mut acc = Accumulator17::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.374_f32 + y.sin();
        let b = y * 0.607_f32 - x.cos();
        let mut acc = Accumulator17::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.379_f32 + y.sin();
        let b = y * 9.016_f32 - x.cos();
        let mut acc = Accumulator17::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.714_f32 + y.sin();
        let b = y * 9.187_f32 - x.cos();
        let mut acc = Accumulator17::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.044_f32 + y.sin();
        let b = y * 1.53_f32 - x.cos();
        let mut acc = Accumulator17::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.686_f32 + y.sin();
        let b = y * 1.783_f32 - x.cos();
        let mut acc = Accumulator17::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.166_f32 + y.sin();
        let b = y * 4.907_f32 - x.cos();
        let mut acc = Accumulator17::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.056_f32 + y.sin();
        let b = y * 8.305_f32 - x.cos();
        let mut acc = Accumulator17::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.66_f32 + y.sin();
        let b = y * 4.831_f32 - x.cos();
        let mut acc = Accumulator17::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.055_f32 + y.sin();
        let b = y * 6.336_f32 - x.cos();
        let mut acc = Accumulator17::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.961_f32 + y.sin();
        let b = y * 6.179_f32 - x.cos();
        let mut acc = Accumulator17::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.375_f32 + y.sin();
        let b = y * 8.495_f32 - x.cos();
        let mut acc = Accumulator17::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.064_f32 + y.sin();
        let b = y * 8.394_f32 - x.cos();
        let mut acc = Accumulator17::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.893_f32 + y.sin();
        let b = y * 2.238_f32 - x.cos();
        let mut acc = Accumulator17::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_17(seed: u64) -> u64 {
        let re = Regex::new(r"m17-(\d+)").unwrap();
        let hay = format!("m17-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_17() -> f32 {
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
        total += (dep_touch_17(total as u64) % 997) as f32;
        total
    }
}

pub mod m18 {
    use super::*;

    pub struct Accumulator18<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator18<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.298_f32 + y.sin();
        let b = y * 1.633_f32 - x.cos();
        let mut acc = Accumulator18::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.306_f32 + y.sin();
        let b = y * 2.521_f32 - x.cos();
        let mut acc = Accumulator18::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.093_f32 + y.sin();
        let b = y * 1.987_f32 - x.cos();
        let mut acc = Accumulator18::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.356_f32 + y.sin();
        let b = y * 6.688_f32 - x.cos();
        let mut acc = Accumulator18::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.314_f32 + y.sin();
        let b = y * 1.254_f32 - x.cos();
        let mut acc = Accumulator18::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.566_f32 + y.sin();
        let b = y * 8.42_f32 - x.cos();
        let mut acc = Accumulator18::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.46_f32 + y.sin();
        let b = y * 3.12_f32 - x.cos();
        let mut acc = Accumulator18::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.272_f32 + y.sin();
        let b = y * 6.557_f32 - x.cos();
        let mut acc = Accumulator18::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.035_f32 + y.sin();
        let b = y * 1.852_f32 - x.cos();
        let mut acc = Accumulator18::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.165_f32 + y.sin();
        let b = y * 4.897_f32 - x.cos();
        let mut acc = Accumulator18::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.479_f32 + y.sin();
        let b = y * 6.162_f32 - x.cos();
        let mut acc = Accumulator18::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.298_f32 + y.sin();
        let b = y * 8.043_f32 - x.cos();
        let mut acc = Accumulator18::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.149_f32 + y.sin();
        let b = y * 1.359_f32 - x.cos();
        let mut acc = Accumulator18::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.68_f32 + y.sin();
        let b = y * 7.962_f32 - x.cos();
        let mut acc = Accumulator18::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.1_f32 + y.sin();
        let b = y * 0.5_f32 - x.cos();
        let mut acc = Accumulator18::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.377_f32 + y.sin();
        let b = y * 9.137_f32 - x.cos();
        let mut acc = Accumulator18::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.721_f32 + y.sin();
        let b = y * 5.113_f32 - x.cos();
        let mut acc = Accumulator18::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.47_f32 + y.sin();
        let b = y * 8.87_f32 - x.cos();
        let mut acc = Accumulator18::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.419_f32 + y.sin();
        let b = y * 1.435_f32 - x.cos();
        let mut acc = Accumulator18::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.862_f32 + y.sin();
        let b = y * 7.274_f32 - x.cos();
        let mut acc = Accumulator18::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_18(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_18() -> f32 {
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
        total += (dep_touch_18(total as u64) % 997) as f32;
        total
    }
}

pub mod m19 {
    use super::*;

    pub struct Accumulator19<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator19<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.998_f32 + y.sin();
        let b = y * 9.721_f32 - x.cos();
        let mut acc = Accumulator19::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.921_f32 + y.sin();
        let b = y * 8.048_f32 - x.cos();
        let mut acc = Accumulator19::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.824_f32 + y.sin();
        let b = y * 7.167_f32 - x.cos();
        let mut acc = Accumulator19::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.742_f32 + y.sin();
        let b = y * 3.539_f32 - x.cos();
        let mut acc = Accumulator19::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.572_f32 + y.sin();
        let b = y * 3.274_f32 - x.cos();
        let mut acc = Accumulator19::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.795_f32 + y.sin();
        let b = y * 8.093_f32 - x.cos();
        let mut acc = Accumulator19::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.591_f32 + y.sin();
        let b = y * 9.55_f32 - x.cos();
        let mut acc = Accumulator19::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.142_f32 + y.sin();
        let b = y * 2.676_f32 - x.cos();
        let mut acc = Accumulator19::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.427_f32 + y.sin();
        let b = y * 3.748_f32 - x.cos();
        let mut acc = Accumulator19::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.885_f32 + y.sin();
        let b = y * 1.68_f32 - x.cos();
        let mut acc = Accumulator19::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.761_f32 + y.sin();
        let b = y * 8.875_f32 - x.cos();
        let mut acc = Accumulator19::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.863_f32 + y.sin();
        let b = y * 2.691_f32 - x.cos();
        let mut acc = Accumulator19::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.576_f32 + y.sin();
        let b = y * 8.511_f32 - x.cos();
        let mut acc = Accumulator19::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.541_f32 + y.sin();
        let b = y * 5.784_f32 - x.cos();
        let mut acc = Accumulator19::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.57_f32 + y.sin();
        let b = y * 5.35_f32 - x.cos();
        let mut acc = Accumulator19::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.332_f32 + y.sin();
        let b = y * 3.74_f32 - x.cos();
        let mut acc = Accumulator19::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.807_f32 + y.sin();
        let b = y * 5.758_f32 - x.cos();
        let mut acc = Accumulator19::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.342_f32 + y.sin();
        let b = y * 0.898_f32 - x.cos();
        let mut acc = Accumulator19::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.832_f32 + y.sin();
        let b = y * 7.387_f32 - x.cos();
        let mut acc = Accumulator19::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.005_f32 + y.sin();
        let b = y * 5.158_f32 - x.cos();
        let mut acc = Accumulator19::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_19(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(19u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_19() -> f32 {
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
        total += (dep_touch_19(total as u64) % 997) as f32;
        total
    }
}

pub mod m20 {
    use super::*;

    pub struct Accumulator20<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator20<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.365_f32 + y.sin();
        let b = y * 9.744_f32 - x.cos();
        let mut acc = Accumulator20::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.199_f32 + y.sin();
        let b = y * 8.878_f32 - x.cos();
        let mut acc = Accumulator20::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.118_f32 + y.sin();
        let b = y * 0.431_f32 - x.cos();
        let mut acc = Accumulator20::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.952_f32 + y.sin();
        let b = y * 6.231_f32 - x.cos();
        let mut acc = Accumulator20::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.124_f32 + y.sin();
        let b = y * 8.876_f32 - x.cos();
        let mut acc = Accumulator20::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.886_f32 + y.sin();
        let b = y * 6.103_f32 - x.cos();
        let mut acc = Accumulator20::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.318_f32 + y.sin();
        let b = y * 0.126_f32 - x.cos();
        let mut acc = Accumulator20::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.077_f32 + y.sin();
        let b = y * 5.226_f32 - x.cos();
        let mut acc = Accumulator20::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.298_f32 + y.sin();
        let b = y * 5.819_f32 - x.cos();
        let mut acc = Accumulator20::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.411_f32 + y.sin();
        let b = y * 3.689_f32 - x.cos();
        let mut acc = Accumulator20::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.655_f32 + y.sin();
        let b = y * 0.238_f32 - x.cos();
        let mut acc = Accumulator20::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.033_f32 + y.sin();
        let b = y * 4.518_f32 - x.cos();
        let mut acc = Accumulator20::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.354_f32 + y.sin();
        let b = y * 8.639_f32 - x.cos();
        let mut acc = Accumulator20::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.039_f32 + y.sin();
        let b = y * 2.69_f32 - x.cos();
        let mut acc = Accumulator20::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.65_f32 + y.sin();
        let b = y * 8.145_f32 - x.cos();
        let mut acc = Accumulator20::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.928_f32 + y.sin();
        let b = y * 5.769_f32 - x.cos();
        let mut acc = Accumulator20::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.284_f32 + y.sin();
        let b = y * 7.289_f32 - x.cos();
        let mut acc = Accumulator20::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.718_f32 + y.sin();
        let b = y * 0.104_f32 - x.cos();
        let mut acc = Accumulator20::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.309_f32 + y.sin();
        let b = y * 4.079_f32 - x.cos();
        let mut acc = Accumulator20::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.66_f32 + y.sin();
        let b = y * 9.035_f32 - x.cos();
        let mut acc = Accumulator20::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_20(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_20() -> f32 {
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
        total += (dep_touch_20(total as u64) % 997) as f32;
        total
    }
}

pub mod m21 {
    use super::*;

    pub struct Accumulator21<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator21<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.221_f32 + y.sin();
        let b = y * 5.499_f32 - x.cos();
        let mut acc = Accumulator21::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.494_f32 + y.sin();
        let b = y * 2.055_f32 - x.cos();
        let mut acc = Accumulator21::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.398_f32 + y.sin();
        let b = y * 6.446_f32 - x.cos();
        let mut acc = Accumulator21::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.071_f32 + y.sin();
        let b = y * 1.811_f32 - x.cos();
        let mut acc = Accumulator21::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.725_f32 + y.sin();
        let b = y * 6.234_f32 - x.cos();
        let mut acc = Accumulator21::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.773_f32 + y.sin();
        let b = y * 7.111_f32 - x.cos();
        let mut acc = Accumulator21::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.777_f32 + y.sin();
        let b = y * 4.379_f32 - x.cos();
        let mut acc = Accumulator21::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.889_f32 + y.sin();
        let b = y * 6.524_f32 - x.cos();
        let mut acc = Accumulator21::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.314_f32 + y.sin();
        let b = y * 1.132_f32 - x.cos();
        let mut acc = Accumulator21::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.411_f32 + y.sin();
        let b = y * 1.308_f32 - x.cos();
        let mut acc = Accumulator21::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.167_f32 + y.sin();
        let b = y * 9.34_f32 - x.cos();
        let mut acc = Accumulator21::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.075_f32 + y.sin();
        let b = y * 2.707_f32 - x.cos();
        let mut acc = Accumulator21::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.757_f32 + y.sin();
        let b = y * 6.82_f32 - x.cos();
        let mut acc = Accumulator21::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.625_f32 + y.sin();
        let b = y * 2.997_f32 - x.cos();
        let mut acc = Accumulator21::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.937_f32 + y.sin();
        let b = y * 5.073_f32 - x.cos();
        let mut acc = Accumulator21::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.652_f32 + y.sin();
        let b = y * 2.414_f32 - x.cos();
        let mut acc = Accumulator21::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.087_f32 + y.sin();
        let b = y * 1.66_f32 - x.cos();
        let mut acc = Accumulator21::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.981_f32 + y.sin();
        let b = y * 3.909_f32 - x.cos();
        let mut acc = Accumulator21::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.444_f32 + y.sin();
        let b = y * 8.994_f32 - x.cos();
        let mut acc = Accumulator21::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.132_f32 + y.sin();
        let b = y * 9.72_f32 - x.cos();
        let mut acc = Accumulator21::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_21(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_21() -> f32 {
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
        total += (dep_touch_21(total as u64) % 997) as f32;
        total
    }
}

pub mod m22 {
    use super::*;

    pub struct Accumulator22<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator22<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.701_f32 + y.sin();
        let b = y * 8.329_f32 - x.cos();
        let mut acc = Accumulator22::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.163_f32 + y.sin();
        let b = y * 0.36_f32 - x.cos();
        let mut acc = Accumulator22::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.392_f32 + y.sin();
        let b = y * 8.771_f32 - x.cos();
        let mut acc = Accumulator22::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.937_f32 + y.sin();
        let b = y * 5.836_f32 - x.cos();
        let mut acc = Accumulator22::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.026_f32 + y.sin();
        let b = y * 1.517_f32 - x.cos();
        let mut acc = Accumulator22::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.197_f32 + y.sin();
        let b = y * 6.195_f32 - x.cos();
        let mut acc = Accumulator22::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.48_f32 + y.sin();
        let b = y * 1.49_f32 - x.cos();
        let mut acc = Accumulator22::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.403_f32 + y.sin();
        let b = y * 1.456_f32 - x.cos();
        let mut acc = Accumulator22::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.312_f32 + y.sin();
        let b = y * 6.931_f32 - x.cos();
        let mut acc = Accumulator22::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.558_f32 + y.sin();
        let b = y * 8.494_f32 - x.cos();
        let mut acc = Accumulator22::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.053_f32 + y.sin();
        let b = y * 9.455_f32 - x.cos();
        let mut acc = Accumulator22::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.835_f32 + y.sin();
        let b = y * 0.746_f32 - x.cos();
        let mut acc = Accumulator22::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.061_f32 + y.sin();
        let b = y * 9.354_f32 - x.cos();
        let mut acc = Accumulator22::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.516_f32 + y.sin();
        let b = y * 2.091_f32 - x.cos();
        let mut acc = Accumulator22::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.437_f32 + y.sin();
        let b = y * 8.408_f32 - x.cos();
        let mut acc = Accumulator22::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.315_f32 + y.sin();
        let b = y * 8.186_f32 - x.cos();
        let mut acc = Accumulator22::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.296_f32 + y.sin();
        let b = y * 4.776_f32 - x.cos();
        let mut acc = Accumulator22::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.059_f32 + y.sin();
        let b = y * 7.522_f32 - x.cos();
        let mut acc = Accumulator22::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.986_f32 + y.sin();
        let b = y * 3.398_f32 - x.cos();
        let mut acc = Accumulator22::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.305_f32 + y.sin();
        let b = y * 2.616_f32 - x.cos();
        let mut acc = Accumulator22::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_22(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m22-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_22() -> f32 {
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
        total += (dep_touch_22(total as u64) % 997) as f32;
        total
    }
}

pub mod m23 {
    use super::*;

    pub struct Accumulator23<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator23<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.574_f32 + y.sin();
        let b = y * 7.547_f32 - x.cos();
        let mut acc = Accumulator23::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.639_f32 + y.sin();
        let b = y * 6.0_f32 - x.cos();
        let mut acc = Accumulator23::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.443_f32 + y.sin();
        let b = y * 6.159_f32 - x.cos();
        let mut acc = Accumulator23::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.833_f32 + y.sin();
        let b = y * 0.406_f32 - x.cos();
        let mut acc = Accumulator23::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.676_f32 + y.sin();
        let b = y * 3.498_f32 - x.cos();
        let mut acc = Accumulator23::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.572_f32 + y.sin();
        let b = y * 5.648_f32 - x.cos();
        let mut acc = Accumulator23::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.55_f32 + y.sin();
        let b = y * 0.991_f32 - x.cos();
        let mut acc = Accumulator23::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.77_f32 + y.sin();
        let b = y * 0.113_f32 - x.cos();
        let mut acc = Accumulator23::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.926_f32 + y.sin();
        let b = y * 7.455_f32 - x.cos();
        let mut acc = Accumulator23::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.143_f32 + y.sin();
        let b = y * 4.91_f32 - x.cos();
        let mut acc = Accumulator23::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.913_f32 + y.sin();
        let b = y * 8.188_f32 - x.cos();
        let mut acc = Accumulator23::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.907_f32 + y.sin();
        let b = y * 9.481_f32 - x.cos();
        let mut acc = Accumulator23::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.654_f32 + y.sin();
        let b = y * 9.35_f32 - x.cos();
        let mut acc = Accumulator23::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.089_f32 + y.sin();
        let b = y * 9.295_f32 - x.cos();
        let mut acc = Accumulator23::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.983_f32 + y.sin();
        let b = y * 1.177_f32 - x.cos();
        let mut acc = Accumulator23::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.615_f32 + y.sin();
        let b = y * 4.905_f32 - x.cos();
        let mut acc = Accumulator23::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.6_f32 + y.sin();
        let b = y * 1.125_f32 - x.cos();
        let mut acc = Accumulator23::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.585_f32 + y.sin();
        let b = y * 4.032_f32 - x.cos();
        let mut acc = Accumulator23::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.84_f32 + y.sin();
        let b = y * 7.403_f32 - x.cos();
        let mut acc = Accumulator23::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.807_f32 + y.sin();
        let b = y * 0.347_f32 - x.cos();
        let mut acc = Accumulator23::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_23(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_23() -> f32 {
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
        total += (dep_touch_23(total as u64) % 997) as f32;
        total
    }
}

pub mod m24 {
    use super::*;

    pub struct Accumulator24<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator24<T> {
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
        let b = y * 4.295_f32 - x.cos();
        let mut acc = Accumulator24::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.012_f32 + y.sin();
        let b = y * 3.817_f32 - x.cos();
        let mut acc = Accumulator24::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.389_f32 + y.sin();
        let b = y * 4.617_f32 - x.cos();
        let mut acc = Accumulator24::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.922_f32 + y.sin();
        let b = y * 6.855_f32 - x.cos();
        let mut acc = Accumulator24::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.434_f32 + y.sin();
        let b = y * 3.515_f32 - x.cos();
        let mut acc = Accumulator24::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.213_f32 + y.sin();
        let b = y * 8.606_f32 - x.cos();
        let mut acc = Accumulator24::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.589_f32 + y.sin();
        let b = y * 7.371_f32 - x.cos();
        let mut acc = Accumulator24::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.639_f32 + y.sin();
        let b = y * 6.853_f32 - x.cos();
        let mut acc = Accumulator24::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.776_f32 + y.sin();
        let b = y * 1.335_f32 - x.cos();
        let mut acc = Accumulator24::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.398_f32 + y.sin();
        let b = y * 6.926_f32 - x.cos();
        let mut acc = Accumulator24::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.977_f32 + y.sin();
        let b = y * 3.055_f32 - x.cos();
        let mut acc = Accumulator24::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.2_f32 + y.sin();
        let b = y * 6.15_f32 - x.cos();
        let mut acc = Accumulator24::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.629_f32 + y.sin();
        let b = y * 2.526_f32 - x.cos();
        let mut acc = Accumulator24::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.008_f32 + y.sin();
        let b = y * 3.517_f32 - x.cos();
        let mut acc = Accumulator24::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.315_f32 + y.sin();
        let b = y * 1.955_f32 - x.cos();
        let mut acc = Accumulator24::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.85_f32 + y.sin();
        let b = y * 1.713_f32 - x.cos();
        let mut acc = Accumulator24::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.096_f32 + y.sin();
        let b = y * 3.865_f32 - x.cos();
        let mut acc = Accumulator24::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.89_f32 + y.sin();
        let b = y * 7.286_f32 - x.cos();
        let mut acc = Accumulator24::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.783_f32 + y.sin();
        let b = y * 1.171_f32 - x.cos();
        let mut acc = Accumulator24::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.852_f32 + y.sin();
        let b = y * 8.775_f32 - x.cos();
        let mut acc = Accumulator24::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_24(seed: u64) -> u64 {
        let re = Regex::new(r"m24-(\d+)").unwrap();
        let hay = format!("m24-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_24() -> f32 {
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
        total += (dep_touch_24(total as u64) % 997) as f32;
        total
    }
}

pub mod m25 {
    use super::*;

    pub struct Accumulator25<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator25<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.433_f32 + y.sin();
        let b = y * 4.01_f32 - x.cos();
        let mut acc = Accumulator25::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.896_f32 + y.sin();
        let b = y * 5.005_f32 - x.cos();
        let mut acc = Accumulator25::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.003_f32 + y.sin();
        let b = y * 0.317_f32 - x.cos();
        let mut acc = Accumulator25::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.016_f32 + y.sin();
        let b = y * 4.066_f32 - x.cos();
        let mut acc = Accumulator25::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.474_f32 + y.sin();
        let b = y * 8.458_f32 - x.cos();
        let mut acc = Accumulator25::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.725_f32 + y.sin();
        let b = y * 7.441_f32 - x.cos();
        let mut acc = Accumulator25::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.391_f32 + y.sin();
        let b = y * 6.645_f32 - x.cos();
        let mut acc = Accumulator25::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.725_f32 + y.sin();
        let b = y * 7.686_f32 - x.cos();
        let mut acc = Accumulator25::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.821_f32 + y.sin();
        let b = y * 2.34_f32 - x.cos();
        let mut acc = Accumulator25::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.387_f32 + y.sin();
        let b = y * 4.548_f32 - x.cos();
        let mut acc = Accumulator25::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.646_f32 + y.sin();
        let b = y * 6.966_f32 - x.cos();
        let mut acc = Accumulator25::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.475_f32 + y.sin();
        let b = y * 4.021_f32 - x.cos();
        let mut acc = Accumulator25::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.27_f32 + y.sin();
        let b = y * 2.551_f32 - x.cos();
        let mut acc = Accumulator25::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.831_f32 + y.sin();
        let b = y * 0.293_f32 - x.cos();
        let mut acc = Accumulator25::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.179_f32 + y.sin();
        let b = y * 6.579_f32 - x.cos();
        let mut acc = Accumulator25::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.866_f32 + y.sin();
        let b = y * 3.315_f32 - x.cos();
        let mut acc = Accumulator25::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.909_f32 + y.sin();
        let b = y * 4.9_f32 - x.cos();
        let mut acc = Accumulator25::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.474_f32 + y.sin();
        let b = y * 5.425_f32 - x.cos();
        let mut acc = Accumulator25::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.119_f32 + y.sin();
        let b = y * 9.423_f32 - x.cos();
        let mut acc = Accumulator25::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.188_f32 + y.sin();
        let b = y * 1.091_f32 - x.cos();
        let mut acc = Accumulator25::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_25(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_25() -> f32 {
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
        total += (dep_touch_25(total as u64) % 997) as f32;
        total
    }
}

pub mod m26 {
    use super::*;

    pub struct Accumulator26<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator26<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.576_f32 + y.sin();
        let b = y * 2.109_f32 - x.cos();
        let mut acc = Accumulator26::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.119_f32 + y.sin();
        let b = y * 6.365_f32 - x.cos();
        let mut acc = Accumulator26::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.213_f32 + y.sin();
        let b = y * 4.121_f32 - x.cos();
        let mut acc = Accumulator26::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.159_f32 + y.sin();
        let b = y * 6.807_f32 - x.cos();
        let mut acc = Accumulator26::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.135_f32 + y.sin();
        let b = y * 9.24_f32 - x.cos();
        let mut acc = Accumulator26::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.748_f32 + y.sin();
        let b = y * 3.584_f32 - x.cos();
        let mut acc = Accumulator26::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.574_f32 + y.sin();
        let b = y * 3.842_f32 - x.cos();
        let mut acc = Accumulator26::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.23_f32 + y.sin();
        let b = y * 4.202_f32 - x.cos();
        let mut acc = Accumulator26::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.26_f32 + y.sin();
        let b = y * 6.714_f32 - x.cos();
        let mut acc = Accumulator26::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.699_f32 + y.sin();
        let b = y * 2.299_f32 - x.cos();
        let mut acc = Accumulator26::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.025_f32 + y.sin();
        let b = y * 9.445_f32 - x.cos();
        let mut acc = Accumulator26::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.843_f32 + y.sin();
        let b = y * 9.516_f32 - x.cos();
        let mut acc = Accumulator26::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.178_f32 + y.sin();
        let b = y * 1.367_f32 - x.cos();
        let mut acc = Accumulator26::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.034_f32 + y.sin();
        let b = y * 6.316_f32 - x.cos();
        let mut acc = Accumulator26::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.394_f32 + y.sin();
        let b = y * 7.163_f32 - x.cos();
        let mut acc = Accumulator26::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.561_f32 + y.sin();
        let b = y * 6.36_f32 - x.cos();
        let mut acc = Accumulator26::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.687_f32 + y.sin();
        let b = y * 2.985_f32 - x.cos();
        let mut acc = Accumulator26::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.466_f32 + y.sin();
        let b = y * 7.742_f32 - x.cos();
        let mut acc = Accumulator26::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.577_f32 + y.sin();
        let b = y * 8.437_f32 - x.cos();
        let mut acc = Accumulator26::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.001_f32 + y.sin();
        let b = y * 6.837_f32 - x.cos();
        let mut acc = Accumulator26::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_26(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(26u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_26() -> f32 {
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
        total += (dep_touch_26(total as u64) % 997) as f32;
        total
    }
}

pub mod m27 {
    use super::*;

    pub struct Accumulator27<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator27<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.752_f32 + y.sin();
        let b = y * 4.819_f32 - x.cos();
        let mut acc = Accumulator27::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.929_f32 + y.sin();
        let b = y * 3.608_f32 - x.cos();
        let mut acc = Accumulator27::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.058_f32 + y.sin();
        let b = y * 4.8_f32 - x.cos();
        let mut acc = Accumulator27::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.209_f32 + y.sin();
        let b = y * 0.937_f32 - x.cos();
        let mut acc = Accumulator27::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.597_f32 + y.sin();
        let b = y * 3.071_f32 - x.cos();
        let mut acc = Accumulator27::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.659_f32 + y.sin();
        let b = y * 8.213_f32 - x.cos();
        let mut acc = Accumulator27::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.784_f32 + y.sin();
        let b = y * 1.476_f32 - x.cos();
        let mut acc = Accumulator27::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.305_f32 + y.sin();
        let b = y * 0.247_f32 - x.cos();
        let mut acc = Accumulator27::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.156_f32 + y.sin();
        let b = y * 0.806_f32 - x.cos();
        let mut acc = Accumulator27::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.55_f32 + y.sin();
        let b = y * 1.095_f32 - x.cos();
        let mut acc = Accumulator27::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.471_f32 + y.sin();
        let b = y * 1.92_f32 - x.cos();
        let mut acc = Accumulator27::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.495_f32 + y.sin();
        let b = y * 1.596_f32 - x.cos();
        let mut acc = Accumulator27::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.858_f32 + y.sin();
        let b = y * 1.746_f32 - x.cos();
        let mut acc = Accumulator27::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.062_f32 + y.sin();
        let b = y * 7.757_f32 - x.cos();
        let mut acc = Accumulator27::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.936_f32 + y.sin();
        let b = y * 5.475_f32 - x.cos();
        let mut acc = Accumulator27::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.32_f32 + y.sin();
        let b = y * 2.034_f32 - x.cos();
        let mut acc = Accumulator27::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.188_f32 + y.sin();
        let b = y * 0.87_f32 - x.cos();
        let mut acc = Accumulator27::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.678_f32 + y.sin();
        let b = y * 1.246_f32 - x.cos();
        let mut acc = Accumulator27::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.692_f32 + y.sin();
        let b = y * 2.395_f32 - x.cos();
        let mut acc = Accumulator27::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.738_f32 + y.sin();
        let b = y * 5.561_f32 - x.cos();
        let mut acc = Accumulator27::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_27(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_27() -> f32 {
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
        total += (dep_touch_27(total as u64) % 997) as f32;
        total
    }
}

pub mod m28 {
    use super::*;

    pub struct Accumulator28<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator28<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.678_f32 + y.sin();
        let b = y * 1.515_f32 - x.cos();
        let mut acc = Accumulator28::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.516_f32 + y.sin();
        let b = y * 1.713_f32 - x.cos();
        let mut acc = Accumulator28::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.556_f32 + y.sin();
        let b = y * 0.165_f32 - x.cos();
        let mut acc = Accumulator28::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.686_f32 + y.sin();
        let b = y * 5.613_f32 - x.cos();
        let mut acc = Accumulator28::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.009_f32 + y.sin();
        let b = y * 4.664_f32 - x.cos();
        let mut acc = Accumulator28::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.204_f32 + y.sin();
        let b = y * 9.514_f32 - x.cos();
        let mut acc = Accumulator28::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.869_f32 + y.sin();
        let b = y * 3.632_f32 - x.cos();
        let mut acc = Accumulator28::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.38_f32 + y.sin();
        let b = y * 6.075_f32 - x.cos();
        let mut acc = Accumulator28::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.318_f32 + y.sin();
        let b = y * 9.89_f32 - x.cos();
        let mut acc = Accumulator28::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.104_f32 + y.sin();
        let b = y * 4.85_f32 - x.cos();
        let mut acc = Accumulator28::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.432_f32 + y.sin();
        let b = y * 7.138_f32 - x.cos();
        let mut acc = Accumulator28::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.344_f32 + y.sin();
        let b = y * 1.026_f32 - x.cos();
        let mut acc = Accumulator28::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.688_f32 + y.sin();
        let b = y * 4.75_f32 - x.cos();
        let mut acc = Accumulator28::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.53_f32 + y.sin();
        let b = y * 9.041_f32 - x.cos();
        let mut acc = Accumulator28::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.365_f32 + y.sin();
        let b = y * 4.239_f32 - x.cos();
        let mut acc = Accumulator28::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.617_f32 + y.sin();
        let b = y * 2.934_f32 - x.cos();
        let mut acc = Accumulator28::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.212_f32 + y.sin();
        let b = y * 4.057_f32 - x.cos();
        let mut acc = Accumulator28::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.746_f32 + y.sin();
        let b = y * 8.655_f32 - x.cos();
        let mut acc = Accumulator28::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.655_f32 + y.sin();
        let b = y * 6.515_f32 - x.cos();
        let mut acc = Accumulator28::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.343_f32 + y.sin();
        let b = y * 3.208_f32 - x.cos();
        let mut acc = Accumulator28::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_28(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_28() -> f32 {
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
        total += (dep_touch_28(total as u64) % 997) as f32;
        total
    }
}

pub mod m29 {
    use super::*;

    pub struct Accumulator29<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator29<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.35_f32 + y.sin();
        let b = y * 9.633_f32 - x.cos();
        let mut acc = Accumulator29::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.785_f32 + y.sin();
        let b = y * 0.493_f32 - x.cos();
        let mut acc = Accumulator29::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.532_f32 + y.sin();
        let b = y * 4.079_f32 - x.cos();
        let mut acc = Accumulator29::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.587_f32 + y.sin();
        let b = y * 3.044_f32 - x.cos();
        let mut acc = Accumulator29::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.555_f32 + y.sin();
        let b = y * 8.155_f32 - x.cos();
        let mut acc = Accumulator29::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.065_f32 + y.sin();
        let b = y * 6.549_f32 - x.cos();
        let mut acc = Accumulator29::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.016_f32 + y.sin();
        let b = y * 6.095_f32 - x.cos();
        let mut acc = Accumulator29::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.541_f32 + y.sin();
        let b = y * 6.702_f32 - x.cos();
        let mut acc = Accumulator29::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.944_f32 + y.sin();
        let b = y * 6.774_f32 - x.cos();
        let mut acc = Accumulator29::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.487_f32 + y.sin();
        let b = y * 6.309_f32 - x.cos();
        let mut acc = Accumulator29::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.574_f32 + y.sin();
        let b = y * 1.093_f32 - x.cos();
        let mut acc = Accumulator29::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.618_f32 + y.sin();
        let b = y * 4.231_f32 - x.cos();
        let mut acc = Accumulator29::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.058_f32 + y.sin();
        let b = y * 6.526_f32 - x.cos();
        let mut acc = Accumulator29::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.645_f32 + y.sin();
        let b = y * 1.459_f32 - x.cos();
        let mut acc = Accumulator29::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.609_f32 + y.sin();
        let b = y * 2.628_f32 - x.cos();
        let mut acc = Accumulator29::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.911_f32 + y.sin();
        let b = y * 0.436_f32 - x.cos();
        let mut acc = Accumulator29::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.321_f32 + y.sin();
        let b = y * 6.389_f32 - x.cos();
        let mut acc = Accumulator29::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.978_f32 + y.sin();
        let b = y * 5.217_f32 - x.cos();
        let mut acc = Accumulator29::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.683_f32 + y.sin();
        let b = y * 4.227_f32 - x.cos();
        let mut acc = Accumulator29::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.103_f32 + y.sin();
        let b = y * 4.475_f32 - x.cos();
        let mut acc = Accumulator29::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_29(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m29-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_29() -> f32 {
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
        total += (dep_touch_29(total as u64) % 997) as f32;
        total
    }
}

pub mod m30 {
    use super::*;

    pub struct Accumulator30<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator30<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.764_f32 + y.sin();
        let b = y * 5.92_f32 - x.cos();
        let mut acc = Accumulator30::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.712_f32 + y.sin();
        let b = y * 4.759_f32 - x.cos();
        let mut acc = Accumulator30::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.478_f32 + y.sin();
        let b = y * 0.913_f32 - x.cos();
        let mut acc = Accumulator30::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.18_f32 + y.sin();
        let b = y * 1.587_f32 - x.cos();
        let mut acc = Accumulator30::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.285_f32 + y.sin();
        let b = y * 0.191_f32 - x.cos();
        let mut acc = Accumulator30::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.292_f32 + y.sin();
        let b = y * 9.57_f32 - x.cos();
        let mut acc = Accumulator30::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.239_f32 + y.sin();
        let b = y * 1.289_f32 - x.cos();
        let mut acc = Accumulator30::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.274_f32 + y.sin();
        let b = y * 7.15_f32 - x.cos();
        let mut acc = Accumulator30::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.518_f32 + y.sin();
        let b = y * 7.393_f32 - x.cos();
        let mut acc = Accumulator30::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.686_f32 + y.sin();
        let b = y * 7.423_f32 - x.cos();
        let mut acc = Accumulator30::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.484_f32 + y.sin();
        let b = y * 7.251_f32 - x.cos();
        let mut acc = Accumulator30::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.973_f32 + y.sin();
        let b = y * 5.563_f32 - x.cos();
        let mut acc = Accumulator30::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.614_f32 + y.sin();
        let b = y * 9.237_f32 - x.cos();
        let mut acc = Accumulator30::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.053_f32 + y.sin();
        let b = y * 0.616_f32 - x.cos();
        let mut acc = Accumulator30::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.212_f32 + y.sin();
        let b = y * 0.244_f32 - x.cos();
        let mut acc = Accumulator30::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.829_f32 + y.sin();
        let b = y * 6.159_f32 - x.cos();
        let mut acc = Accumulator30::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.148_f32 + y.sin();
        let b = y * 7.249_f32 - x.cos();
        let mut acc = Accumulator30::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.485_f32 + y.sin();
        let b = y * 8.282_f32 - x.cos();
        let mut acc = Accumulator30::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.686_f32 + y.sin();
        let b = y * 3.702_f32 - x.cos();
        let mut acc = Accumulator30::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.232_f32 + y.sin();
        let b = y * 4.704_f32 - x.cos();
        let mut acc = Accumulator30::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_30(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_30() -> f32 {
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
        total += (dep_touch_30(total as u64) % 997) as f32;
        total
    }
}

pub mod m31 {
    use super::*;

    pub struct Accumulator31<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator31<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.52_f32 + y.sin();
        let b = y * 7.914_f32 - x.cos();
        let mut acc = Accumulator31::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.448_f32 + y.sin();
        let b = y * 1.707_f32 - x.cos();
        let mut acc = Accumulator31::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.774_f32 + y.sin();
        let b = y * 7.725_f32 - x.cos();
        let mut acc = Accumulator31::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.36_f32 + y.sin();
        let b = y * 7.789_f32 - x.cos();
        let mut acc = Accumulator31::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.372_f32 + y.sin();
        let b = y * 2.843_f32 - x.cos();
        let mut acc = Accumulator31::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.645_f32 + y.sin();
        let b = y * 6.992_f32 - x.cos();
        let mut acc = Accumulator31::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.354_f32 + y.sin();
        let b = y * 6.037_f32 - x.cos();
        let mut acc = Accumulator31::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.247_f32 + y.sin();
        let b = y * 5.991_f32 - x.cos();
        let mut acc = Accumulator31::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.83_f32 + y.sin();
        let b = y * 9.669_f32 - x.cos();
        let mut acc = Accumulator31::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.791_f32 + y.sin();
        let b = y * 6.811_f32 - x.cos();
        let mut acc = Accumulator31::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.66_f32 + y.sin();
        let b = y * 2.397_f32 - x.cos();
        let mut acc = Accumulator31::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.876_f32 + y.sin();
        let b = y * 0.117_f32 - x.cos();
        let mut acc = Accumulator31::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.727_f32 + y.sin();
        let b = y * 1.641_f32 - x.cos();
        let mut acc = Accumulator31::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.928_f32 + y.sin();
        let b = y * 1.479_f32 - x.cos();
        let mut acc = Accumulator31::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.541_f32 + y.sin();
        let b = y * 9.659_f32 - x.cos();
        let mut acc = Accumulator31::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.809_f32 + y.sin();
        let b = y * 9.055_f32 - x.cos();
        let mut acc = Accumulator31::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.339_f32 + y.sin();
        let b = y * 5.392_f32 - x.cos();
        let mut acc = Accumulator31::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.914_f32 + y.sin();
        let b = y * 2.064_f32 - x.cos();
        let mut acc = Accumulator31::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.177_f32 + y.sin();
        let b = y * 9.726_f32 - x.cos();
        let mut acc = Accumulator31::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.048_f32 + y.sin();
        let b = y * 6.741_f32 - x.cos();
        let mut acc = Accumulator31::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_31(seed: u64) -> u64 {
        let re = Regex::new(r"m31-(\d+)").unwrap();
        let hay = format!("m31-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_31() -> f32 {
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
        total += (dep_touch_31(total as u64) % 997) as f32;
        total
    }
}

pub mod m32 {
    use super::*;

    pub struct Accumulator32<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator32<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.042_f32 + y.sin();
        let b = y * 9.175_f32 - x.cos();
        let mut acc = Accumulator32::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.461_f32 + y.sin();
        let b = y * 7.858_f32 - x.cos();
        let mut acc = Accumulator32::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.398_f32 + y.sin();
        let b = y * 5.354_f32 - x.cos();
        let mut acc = Accumulator32::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.667_f32 + y.sin();
        let b = y * 2.382_f32 - x.cos();
        let mut acc = Accumulator32::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.206_f32 + y.sin();
        let b = y * 2.644_f32 - x.cos();
        let mut acc = Accumulator32::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.246_f32 + y.sin();
        let b = y * 5.06_f32 - x.cos();
        let mut acc = Accumulator32::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.954_f32 + y.sin();
        let b = y * 1.985_f32 - x.cos();
        let mut acc = Accumulator32::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.997_f32 + y.sin();
        let b = y * 2.94_f32 - x.cos();
        let mut acc = Accumulator32::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.631_f32 + y.sin();
        let b = y * 4.044_f32 - x.cos();
        let mut acc = Accumulator32::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.498_f32 + y.sin();
        let b = y * 2.514_f32 - x.cos();
        let mut acc = Accumulator32::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.766_f32 + y.sin();
        let b = y * 1.14_f32 - x.cos();
        let mut acc = Accumulator32::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.642_f32 + y.sin();
        let b = y * 0.901_f32 - x.cos();
        let mut acc = Accumulator32::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.953_f32 + y.sin();
        let b = y * 3.48_f32 - x.cos();
        let mut acc = Accumulator32::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.05_f32 + y.sin();
        let b = y * 1.022_f32 - x.cos();
        let mut acc = Accumulator32::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.806_f32 + y.sin();
        let b = y * 8.588_f32 - x.cos();
        let mut acc = Accumulator32::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.85_f32 + y.sin();
        let b = y * 2.193_f32 - x.cos();
        let mut acc = Accumulator32::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.274_f32 + y.sin();
        let b = y * 9.376_f32 - x.cos();
        let mut acc = Accumulator32::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.125_f32 + y.sin();
        let b = y * 9.542_f32 - x.cos();
        let mut acc = Accumulator32::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.369_f32 + y.sin();
        let b = y * 3.421_f32 - x.cos();
        let mut acc = Accumulator32::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.806_f32 + y.sin();
        let b = y * 0.37_f32 - x.cos();
        let mut acc = Accumulator32::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_32(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_32() -> f32 {
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
        total += (dep_touch_32(total as u64) % 997) as f32;
        total
    }
}

pub mod m33 {
    use super::*;

    pub struct Accumulator33<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator33<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.562_f32 + y.sin();
        let b = y * 8.633_f32 - x.cos();
        let mut acc = Accumulator33::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.871_f32 + y.sin();
        let b = y * 8.387_f32 - x.cos();
        let mut acc = Accumulator33::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.557_f32 + y.sin();
        let b = y * 6.37_f32 - x.cos();
        let mut acc = Accumulator33::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.022_f32 + y.sin();
        let b = y * 0.982_f32 - x.cos();
        let mut acc = Accumulator33::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.632_f32 + y.sin();
        let b = y * 6.378_f32 - x.cos();
        let mut acc = Accumulator33::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.064_f32 + y.sin();
        let b = y * 1.89_f32 - x.cos();
        let mut acc = Accumulator33::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.735_f32 + y.sin();
        let b = y * 2.404_f32 - x.cos();
        let mut acc = Accumulator33::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.273_f32 + y.sin();
        let b = y * 0.479_f32 - x.cos();
        let mut acc = Accumulator33::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.323_f32 + y.sin();
        let b = y * 0.681_f32 - x.cos();
        let mut acc = Accumulator33::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.965_f32 + y.sin();
        let b = y * 8.305_f32 - x.cos();
        let mut acc = Accumulator33::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.627_f32 + y.sin();
        let b = y * 5.131_f32 - x.cos();
        let mut acc = Accumulator33::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.438_f32 + y.sin();
        let b = y * 9.757_f32 - x.cos();
        let mut acc = Accumulator33::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.09_f32 + y.sin();
        let b = y * 3.213_f32 - x.cos();
        let mut acc = Accumulator33::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.306_f32 + y.sin();
        let b = y * 6.734_f32 - x.cos();
        let mut acc = Accumulator33::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.88_f32 + y.sin();
        let b = y * 4.425_f32 - x.cos();
        let mut acc = Accumulator33::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.133_f32 + y.sin();
        let b = y * 3.274_f32 - x.cos();
        let mut acc = Accumulator33::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.922_f32 + y.sin();
        let b = y * 3.775_f32 - x.cos();
        let mut acc = Accumulator33::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.752_f32 + y.sin();
        let b = y * 2.437_f32 - x.cos();
        let mut acc = Accumulator33::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.06_f32 + y.sin();
        let b = y * 8.843_f32 - x.cos();
        let mut acc = Accumulator33::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.129_f32 + y.sin();
        let b = y * 2.012_f32 - x.cos();
        let mut acc = Accumulator33::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_33(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(33u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_33() -> f32 {
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
        total += (dep_touch_33(total as u64) % 997) as f32;
        total
    }
}

pub mod m34 {
    use super::*;

    pub struct Accumulator34<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator34<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.638_f32 + y.sin();
        let b = y * 8.262_f32 - x.cos();
        let mut acc = Accumulator34::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.253_f32 + y.sin();
        let b = y * 8.594_f32 - x.cos();
        let mut acc = Accumulator34::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.47_f32 + y.sin();
        let b = y * 4.483_f32 - x.cos();
        let mut acc = Accumulator34::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.174_f32 + y.sin();
        let b = y * 3.874_f32 - x.cos();
        let mut acc = Accumulator34::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.258_f32 + y.sin();
        let b = y * 4.533_f32 - x.cos();
        let mut acc = Accumulator34::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.261_f32 + y.sin();
        let b = y * 2.392_f32 - x.cos();
        let mut acc = Accumulator34::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.256_f32 + y.sin();
        let b = y * 1.499_f32 - x.cos();
        let mut acc = Accumulator34::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.313_f32 + y.sin();
        let b = y * 1.866_f32 - x.cos();
        let mut acc = Accumulator34::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.523_f32 + y.sin();
        let b = y * 1.518_f32 - x.cos();
        let mut acc = Accumulator34::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.711_f32 + y.sin();
        let b = y * 4.135_f32 - x.cos();
        let mut acc = Accumulator34::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.349_f32 + y.sin();
        let b = y * 5.696_f32 - x.cos();
        let mut acc = Accumulator34::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.378_f32 + y.sin();
        let b = y * 1.744_f32 - x.cos();
        let mut acc = Accumulator34::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.171_f32 + y.sin();
        let b = y * 4.571_f32 - x.cos();
        let mut acc = Accumulator34::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.219_f32 + y.sin();
        let b = y * 9.69_f32 - x.cos();
        let mut acc = Accumulator34::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.284_f32 + y.sin();
        let b = y * 7.819_f32 - x.cos();
        let mut acc = Accumulator34::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.587_f32 + y.sin();
        let b = y * 8.286_f32 - x.cos();
        let mut acc = Accumulator34::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.626_f32 + y.sin();
        let b = y * 2.076_f32 - x.cos();
        let mut acc = Accumulator34::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.334_f32 + y.sin();
        let b = y * 2.663_f32 - x.cos();
        let mut acc = Accumulator34::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.166_f32 + y.sin();
        let b = y * 1.056_f32 - x.cos();
        let mut acc = Accumulator34::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.173_f32 + y.sin();
        let b = y * 1.689_f32 - x.cos();
        let mut acc = Accumulator34::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_34(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_34() -> f32 {
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
        total += (dep_touch_34(total as u64) % 997) as f32;
        total
    }
}

pub mod m35 {
    use super::*;

    pub struct Accumulator35<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator35<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.69_f32 + y.sin();
        let b = y * 1.515_f32 - x.cos();
        let mut acc = Accumulator35::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.257_f32 + y.sin();
        let b = y * 8.009_f32 - x.cos();
        let mut acc = Accumulator35::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.106_f32 + y.sin();
        let b = y * 4.441_f32 - x.cos();
        let mut acc = Accumulator35::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.907_f32 + y.sin();
        let b = y * 3.629_f32 - x.cos();
        let mut acc = Accumulator35::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.037_f32 + y.sin();
        let b = y * 2.239_f32 - x.cos();
        let mut acc = Accumulator35::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.871_f32 + y.sin();
        let b = y * 8.365_f32 - x.cos();
        let mut acc = Accumulator35::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.65_f32 + y.sin();
        let b = y * 7.074_f32 - x.cos();
        let mut acc = Accumulator35::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.986_f32 + y.sin();
        let b = y * 8.224_f32 - x.cos();
        let mut acc = Accumulator35::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.262_f32 + y.sin();
        let b = y * 7.561_f32 - x.cos();
        let mut acc = Accumulator35::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.119_f32 + y.sin();
        let b = y * 6.102_f32 - x.cos();
        let mut acc = Accumulator35::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.259_f32 + y.sin();
        let b = y * 1.983_f32 - x.cos();
        let mut acc = Accumulator35::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.082_f32 + y.sin();
        let b = y * 0.744_f32 - x.cos();
        let mut acc = Accumulator35::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.192_f32 + y.sin();
        let b = y * 8.342_f32 - x.cos();
        let mut acc = Accumulator35::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.181_f32 + y.sin();
        let b = y * 3.507_f32 - x.cos();
        let mut acc = Accumulator35::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.351_f32 + y.sin();
        let b = y * 8.572_f32 - x.cos();
        let mut acc = Accumulator35::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.985_f32 + y.sin();
        let b = y * 4.113_f32 - x.cos();
        let mut acc = Accumulator35::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.406_f32 + y.sin();
        let b = y * 6.622_f32 - x.cos();
        let mut acc = Accumulator35::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.923_f32 + y.sin();
        let b = y * 8.25_f32 - x.cos();
        let mut acc = Accumulator35::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.459_f32 + y.sin();
        let b = y * 6.982_f32 - x.cos();
        let mut acc = Accumulator35::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.93_f32 + y.sin();
        let b = y * 0.145_f32 - x.cos();
        let mut acc = Accumulator35::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_35(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_35() -> f32 {
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
        total += (dep_touch_35(total as u64) % 997) as f32;
        total
    }
}

pub mod m36 {
    use super::*;

    pub struct Accumulator36<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator36<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.235_f32 + y.sin();
        let b = y * 9.594_f32 - x.cos();
        let mut acc = Accumulator36::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.284_f32 + y.sin();
        let b = y * 7.103_f32 - x.cos();
        let mut acc = Accumulator36::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.735_f32 + y.sin();
        let b = y * 8.607_f32 - x.cos();
        let mut acc = Accumulator36::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.463_f32 + y.sin();
        let b = y * 0.7_f32 - x.cos();
        let mut acc = Accumulator36::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.45_f32 + y.sin();
        let b = y * 4.949_f32 - x.cos();
        let mut acc = Accumulator36::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.351_f32 + y.sin();
        let b = y * 7.984_f32 - x.cos();
        let mut acc = Accumulator36::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.303_f32 + y.sin();
        let b = y * 9.581_f32 - x.cos();
        let mut acc = Accumulator36::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.167_f32 + y.sin();
        let b = y * 1.745_f32 - x.cos();
        let mut acc = Accumulator36::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.554_f32 + y.sin();
        let b = y * 8.108_f32 - x.cos();
        let mut acc = Accumulator36::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.291_f32 + y.sin();
        let b = y * 9.178_f32 - x.cos();
        let mut acc = Accumulator36::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.012_f32 + y.sin();
        let b = y * 0.273_f32 - x.cos();
        let mut acc = Accumulator36::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.341_f32 + y.sin();
        let b = y * 4.647_f32 - x.cos();
        let mut acc = Accumulator36::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.986_f32 + y.sin();
        let b = y * 1.108_f32 - x.cos();
        let mut acc = Accumulator36::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.128_f32 + y.sin();
        let b = y * 0.543_f32 - x.cos();
        let mut acc = Accumulator36::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.655_f32 + y.sin();
        let b = y * 5.842_f32 - x.cos();
        let mut acc = Accumulator36::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.178_f32 + y.sin();
        let b = y * 1.291_f32 - x.cos();
        let mut acc = Accumulator36::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.408_f32 + y.sin();
        let b = y * 2.329_f32 - x.cos();
        let mut acc = Accumulator36::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.543_f32 + y.sin();
        let b = y * 5.714_f32 - x.cos();
        let mut acc = Accumulator36::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.987_f32 + y.sin();
        let b = y * 9.392_f32 - x.cos();
        let mut acc = Accumulator36::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.288_f32 + y.sin();
        let b = y * 3.91_f32 - x.cos();
        let mut acc = Accumulator36::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_36(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m36-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_36() -> f32 {
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
        total += (dep_touch_36(total as u64) % 997) as f32;
        total
    }
}

pub mod m37 {
    use super::*;

    pub struct Accumulator37<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator37<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.951_f32 + y.sin();
        let b = y * 6.007_f32 - x.cos();
        let mut acc = Accumulator37::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.977_f32 + y.sin();
        let b = y * 9.325_f32 - x.cos();
        let mut acc = Accumulator37::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.418_f32 + y.sin();
        let b = y * 2.456_f32 - x.cos();
        let mut acc = Accumulator37::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.112_f32 + y.sin();
        let b = y * 8.362_f32 - x.cos();
        let mut acc = Accumulator37::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.983_f32 + y.sin();
        let b = y * 9.045_f32 - x.cos();
        let mut acc = Accumulator37::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.407_f32 + y.sin();
        let b = y * 0.625_f32 - x.cos();
        let mut acc = Accumulator37::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.537_f32 + y.sin();
        let b = y * 6.766_f32 - x.cos();
        let mut acc = Accumulator37::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.543_f32 + y.sin();
        let b = y * 4.237_f32 - x.cos();
        let mut acc = Accumulator37::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.213_f32 + y.sin();
        let b = y * 1.168_f32 - x.cos();
        let mut acc = Accumulator37::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.779_f32 + y.sin();
        let b = y * 4.344_f32 - x.cos();
        let mut acc = Accumulator37::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.658_f32 + y.sin();
        let b = y * 2.31_f32 - x.cos();
        let mut acc = Accumulator37::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.603_f32 + y.sin();
        let b = y * 7.71_f32 - x.cos();
        let mut acc = Accumulator37::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.305_f32 + y.sin();
        let b = y * 8.031_f32 - x.cos();
        let mut acc = Accumulator37::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.437_f32 + y.sin();
        let b = y * 6.387_f32 - x.cos();
        let mut acc = Accumulator37::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.096_f32 + y.sin();
        let b = y * 6.21_f32 - x.cos();
        let mut acc = Accumulator37::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.414_f32 + y.sin();
        let b = y * 9.159_f32 - x.cos();
        let mut acc = Accumulator37::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.085_f32 + y.sin();
        let b = y * 1.293_f32 - x.cos();
        let mut acc = Accumulator37::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.35_f32 + y.sin();
        let b = y * 9.418_f32 - x.cos();
        let mut acc = Accumulator37::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.208_f32 + y.sin();
        let b = y * 3.506_f32 - x.cos();
        let mut acc = Accumulator37::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.28_f32 + y.sin();
        let b = y * 5.924_f32 - x.cos();
        let mut acc = Accumulator37::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_37(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_37() -> f32 {
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
        total += (dep_touch_37(total as u64) % 997) as f32;
        total
    }
}

pub mod m38 {
    use super::*;

    pub struct Accumulator38<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator38<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.934_f32 + y.sin();
        let b = y * 0.928_f32 - x.cos();
        let mut acc = Accumulator38::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.332_f32 + y.sin();
        let b = y * 1.554_f32 - x.cos();
        let mut acc = Accumulator38::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.114_f32 + y.sin();
        let b = y * 8.775_f32 - x.cos();
        let mut acc = Accumulator38::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.758_f32 + y.sin();
        let b = y * 2.786_f32 - x.cos();
        let mut acc = Accumulator38::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.961_f32 + y.sin();
        let b = y * 5.454_f32 - x.cos();
        let mut acc = Accumulator38::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.078_f32 + y.sin();
        let b = y * 5.688_f32 - x.cos();
        let mut acc = Accumulator38::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.889_f32 + y.sin();
        let b = y * 5.476_f32 - x.cos();
        let mut acc = Accumulator38::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.617_f32 + y.sin();
        let b = y * 5.471_f32 - x.cos();
        let mut acc = Accumulator38::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.783_f32 + y.sin();
        let b = y * 8.124_f32 - x.cos();
        let mut acc = Accumulator38::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.474_f32 + y.sin();
        let b = y * 2.271_f32 - x.cos();
        let mut acc = Accumulator38::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.45_f32 + y.sin();
        let b = y * 9.602_f32 - x.cos();
        let mut acc = Accumulator38::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.216_f32 + y.sin();
        let b = y * 3.556_f32 - x.cos();
        let mut acc = Accumulator38::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.275_f32 + y.sin();
        let b = y * 3.29_f32 - x.cos();
        let mut acc = Accumulator38::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.891_f32 + y.sin();
        let b = y * 9.778_f32 - x.cos();
        let mut acc = Accumulator38::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.658_f32 + y.sin();
        let b = y * 0.314_f32 - x.cos();
        let mut acc = Accumulator38::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.755_f32 + y.sin();
        let b = y * 8.638_f32 - x.cos();
        let mut acc = Accumulator38::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.546_f32 + y.sin();
        let b = y * 5.167_f32 - x.cos();
        let mut acc = Accumulator38::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.57_f32 + y.sin();
        let b = y * 7.576_f32 - x.cos();
        let mut acc = Accumulator38::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.307_f32 + y.sin();
        let b = y * 9.501_f32 - x.cos();
        let mut acc = Accumulator38::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.253_f32 + y.sin();
        let b = y * 4.184_f32 - x.cos();
        let mut acc = Accumulator38::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_38(seed: u64) -> u64 {
        let re = Regex::new(r"m38-(\d+)").unwrap();
        let hay = format!("m38-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_38() -> f32 {
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
        total += (dep_touch_38(total as u64) % 997) as f32;
        total
    }
}

pub mod m39 {
    use super::*;

    pub struct Accumulator39<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator39<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.554_f32 + y.sin();
        let b = y * 6.718_f32 - x.cos();
        let mut acc = Accumulator39::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.085_f32 + y.sin();
        let b = y * 2.812_f32 - x.cos();
        let mut acc = Accumulator39::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.032_f32 + y.sin();
        let b = y * 8.491_f32 - x.cos();
        let mut acc = Accumulator39::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.733_f32 + y.sin();
        let b = y * 6.281_f32 - x.cos();
        let mut acc = Accumulator39::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.064_f32 + y.sin();
        let b = y * 1.347_f32 - x.cos();
        let mut acc = Accumulator39::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.142_f32 + y.sin();
        let b = y * 7.604_f32 - x.cos();
        let mut acc = Accumulator39::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.251_f32 + y.sin();
        let b = y * 3.995_f32 - x.cos();
        let mut acc = Accumulator39::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.566_f32 + y.sin();
        let b = y * 8.429_f32 - x.cos();
        let mut acc = Accumulator39::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.653_f32 + y.sin();
        let b = y * 6.052_f32 - x.cos();
        let mut acc = Accumulator39::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.447_f32 + y.sin();
        let b = y * 6.888_f32 - x.cos();
        let mut acc = Accumulator39::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.186_f32 + y.sin();
        let b = y * 2.971_f32 - x.cos();
        let mut acc = Accumulator39::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.256_f32 + y.sin();
        let b = y * 5.935_f32 - x.cos();
        let mut acc = Accumulator39::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.256_f32 + y.sin();
        let b = y * 7.813_f32 - x.cos();
        let mut acc = Accumulator39::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.831_f32 + y.sin();
        let b = y * 3.04_f32 - x.cos();
        let mut acc = Accumulator39::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.08_f32 + y.sin();
        let b = y * 1.521_f32 - x.cos();
        let mut acc = Accumulator39::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.795_f32 + y.sin();
        let b = y * 2.373_f32 - x.cos();
        let mut acc = Accumulator39::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.274_f32 + y.sin();
        let b = y * 8.365_f32 - x.cos();
        let mut acc = Accumulator39::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.496_f32 + y.sin();
        let b = y * 2.102_f32 - x.cos();
        let mut acc = Accumulator39::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.834_f32 + y.sin();
        let b = y * 9.468_f32 - x.cos();
        let mut acc = Accumulator39::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.565_f32 + y.sin();
        let b = y * 5.636_f32 - x.cos();
        let mut acc = Accumulator39::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_39(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_39() -> f32 {
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
        total += (dep_touch_39(total as u64) % 997) as f32;
        total
    }
}

pub mod m40 {
    use super::*;

    pub struct Accumulator40<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator40<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.038_f32 + y.sin();
        let b = y * 5.357_f32 - x.cos();
        let mut acc = Accumulator40::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.377_f32 + y.sin();
        let b = y * 9.884_f32 - x.cos();
        let mut acc = Accumulator40::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.192_f32 + y.sin();
        let b = y * 7.226_f32 - x.cos();
        let mut acc = Accumulator40::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.917_f32 + y.sin();
        let b = y * 3.606_f32 - x.cos();
        let mut acc = Accumulator40::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.727_f32 + y.sin();
        let b = y * 4.54_f32 - x.cos();
        let mut acc = Accumulator40::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.729_f32 + y.sin();
        let b = y * 5.247_f32 - x.cos();
        let mut acc = Accumulator40::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.113_f32 + y.sin();
        let b = y * 5.009_f32 - x.cos();
        let mut acc = Accumulator40::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.601_f32 + y.sin();
        let b = y * 5.726_f32 - x.cos();
        let mut acc = Accumulator40::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.552_f32 + y.sin();
        let b = y * 4.87_f32 - x.cos();
        let mut acc = Accumulator40::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.62_f32 + y.sin();
        let b = y * 8.912_f32 - x.cos();
        let mut acc = Accumulator40::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.464_f32 + y.sin();
        let b = y * 5.295_f32 - x.cos();
        let mut acc = Accumulator40::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.773_f32 + y.sin();
        let b = y * 3.217_f32 - x.cos();
        let mut acc = Accumulator40::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.195_f32 + y.sin();
        let b = y * 5.123_f32 - x.cos();
        let mut acc = Accumulator40::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.528_f32 + y.sin();
        let b = y * 2.99_f32 - x.cos();
        let mut acc = Accumulator40::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.141_f32 + y.sin();
        let b = y * 9.804_f32 - x.cos();
        let mut acc = Accumulator40::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.285_f32 + y.sin();
        let b = y * 5.236_f32 - x.cos();
        let mut acc = Accumulator40::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.136_f32 + y.sin();
        let b = y * 8.853_f32 - x.cos();
        let mut acc = Accumulator40::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.888_f32 + y.sin();
        let b = y * 6.275_f32 - x.cos();
        let mut acc = Accumulator40::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.145_f32 + y.sin();
        let b = y * 5.685_f32 - x.cos();
        let mut acc = Accumulator40::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.338_f32 + y.sin();
        let b = y * 0.515_f32 - x.cos();
        let mut acc = Accumulator40::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_40(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(40u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_40() -> f32 {
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
        total += (dep_touch_40(total as u64) % 997) as f32;
        total
    }
}

pub mod m41 {
    use super::*;

    pub struct Accumulator41<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator41<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.205_f32 + y.sin();
        let b = y * 0.127_f32 - x.cos();
        let mut acc = Accumulator41::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.869_f32 + y.sin();
        let b = y * 0.138_f32 - x.cos();
        let mut acc = Accumulator41::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.996_f32 + y.sin();
        let b = y * 1.065_f32 - x.cos();
        let mut acc = Accumulator41::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.647_f32 + y.sin();
        let b = y * 2.027_f32 - x.cos();
        let mut acc = Accumulator41::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.636_f32 + y.sin();
        let b = y * 5.657_f32 - x.cos();
        let mut acc = Accumulator41::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.877_f32 + y.sin();
        let b = y * 5.14_f32 - x.cos();
        let mut acc = Accumulator41::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.73_f32 + y.sin();
        let b = y * 4.129_f32 - x.cos();
        let mut acc = Accumulator41::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.524_f32 + y.sin();
        let b = y * 5.181_f32 - x.cos();
        let mut acc = Accumulator41::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.145_f32 + y.sin();
        let b = y * 1.081_f32 - x.cos();
        let mut acc = Accumulator41::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.389_f32 + y.sin();
        let b = y * 4.906_f32 - x.cos();
        let mut acc = Accumulator41::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.107_f32 + y.sin();
        let b = y * 8.005_f32 - x.cos();
        let mut acc = Accumulator41::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.471_f32 + y.sin();
        let b = y * 6.809_f32 - x.cos();
        let mut acc = Accumulator41::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.264_f32 + y.sin();
        let b = y * 7.111_f32 - x.cos();
        let mut acc = Accumulator41::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.799_f32 + y.sin();
        let b = y * 0.422_f32 - x.cos();
        let mut acc = Accumulator41::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.075_f32 + y.sin();
        let b = y * 8.958_f32 - x.cos();
        let mut acc = Accumulator41::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.718_f32 + y.sin();
        let b = y * 1.978_f32 - x.cos();
        let mut acc = Accumulator41::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.879_f32 + y.sin();
        let b = y * 0.636_f32 - x.cos();
        let mut acc = Accumulator41::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.81_f32 + y.sin();
        let b = y * 9.504_f32 - x.cos();
        let mut acc = Accumulator41::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.635_f32 + y.sin();
        let b = y * 2.435_f32 - x.cos();
        let mut acc = Accumulator41::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.531_f32 + y.sin();
        let b = y * 9.222_f32 - x.cos();
        let mut acc = Accumulator41::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_41(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_41() -> f32 {
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
        total += (dep_touch_41(total as u64) % 997) as f32;
        total
    }
}

pub mod m42 {
    use super::*;

    pub struct Accumulator42<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator42<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.185_f32 + y.sin();
        let b = y * 8.909_f32 - x.cos();
        let mut acc = Accumulator42::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.076_f32 + y.sin();
        let b = y * 6.005_f32 - x.cos();
        let mut acc = Accumulator42::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.7_f32 + y.sin();
        let b = y * 0.762_f32 - x.cos();
        let mut acc = Accumulator42::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.92_f32 + y.sin();
        let b = y * 7.141_f32 - x.cos();
        let mut acc = Accumulator42::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.152_f32 + y.sin();
        let b = y * 4.006_f32 - x.cos();
        let mut acc = Accumulator42::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.847_f32 + y.sin();
        let b = y * 7.869_f32 - x.cos();
        let mut acc = Accumulator42::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.957_f32 + y.sin();
        let b = y * 1.765_f32 - x.cos();
        let mut acc = Accumulator42::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.928_f32 + y.sin();
        let b = y * 9.621_f32 - x.cos();
        let mut acc = Accumulator42::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.981_f32 + y.sin();
        let b = y * 3.657_f32 - x.cos();
        let mut acc = Accumulator42::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.331_f32 + y.sin();
        let b = y * 3.879_f32 - x.cos();
        let mut acc = Accumulator42::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.483_f32 + y.sin();
        let b = y * 9.52_f32 - x.cos();
        let mut acc = Accumulator42::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.193_f32 + y.sin();
        let b = y * 3.542_f32 - x.cos();
        let mut acc = Accumulator42::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.896_f32 + y.sin();
        let b = y * 4.677_f32 - x.cos();
        let mut acc = Accumulator42::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.424_f32 + y.sin();
        let b = y * 0.442_f32 - x.cos();
        let mut acc = Accumulator42::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.348_f32 + y.sin();
        let b = y * 7.988_f32 - x.cos();
        let mut acc = Accumulator42::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.018_f32 + y.sin();
        let b = y * 1.008_f32 - x.cos();
        let mut acc = Accumulator42::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.44_f32 + y.sin();
        let b = y * 7.817_f32 - x.cos();
        let mut acc = Accumulator42::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.444_f32 + y.sin();
        let b = y * 8.296_f32 - x.cos();
        let mut acc = Accumulator42::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.66_f32 + y.sin();
        let b = y * 3.559_f32 - x.cos();
        let mut acc = Accumulator42::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.07_f32 + y.sin();
        let b = y * 6.268_f32 - x.cos();
        let mut acc = Accumulator42::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_42(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_42() -> f32 {
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
        total += (dep_touch_42(total as u64) % 997) as f32;
        total
    }
}

pub mod m43 {
    use super::*;

    pub struct Accumulator43<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator43<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.139_f32 + y.sin();
        let b = y * 9.419_f32 - x.cos();
        let mut acc = Accumulator43::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.104_f32 + y.sin();
        let b = y * 8.512_f32 - x.cos();
        let mut acc = Accumulator43::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.383_f32 + y.sin();
        let b = y * 7.023_f32 - x.cos();
        let mut acc = Accumulator43::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.94_f32 + y.sin();
        let b = y * 4.415_f32 - x.cos();
        let mut acc = Accumulator43::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.34_f32 + y.sin();
        let b = y * 4.061_f32 - x.cos();
        let mut acc = Accumulator43::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.183_f32 + y.sin();
        let b = y * 8.649_f32 - x.cos();
        let mut acc = Accumulator43::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.743_f32 + y.sin();
        let b = y * 0.996_f32 - x.cos();
        let mut acc = Accumulator43::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.312_f32 + y.sin();
        let b = y * 7.595_f32 - x.cos();
        let mut acc = Accumulator43::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.544_f32 + y.sin();
        let b = y * 5.663_f32 - x.cos();
        let mut acc = Accumulator43::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.247_f32 + y.sin();
        let b = y * 7.065_f32 - x.cos();
        let mut acc = Accumulator43::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.835_f32 + y.sin();
        let b = y * 8.44_f32 - x.cos();
        let mut acc = Accumulator43::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.945_f32 + y.sin();
        let b = y * 8.835_f32 - x.cos();
        let mut acc = Accumulator43::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.608_f32 + y.sin();
        let b = y * 3.643_f32 - x.cos();
        let mut acc = Accumulator43::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.533_f32 + y.sin();
        let b = y * 1.99_f32 - x.cos();
        let mut acc = Accumulator43::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.15_f32 + y.sin();
        let b = y * 2.319_f32 - x.cos();
        let mut acc = Accumulator43::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.105_f32 + y.sin();
        let b = y * 4.01_f32 - x.cos();
        let mut acc = Accumulator43::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.053_f32 + y.sin();
        let b = y * 9.007_f32 - x.cos();
        let mut acc = Accumulator43::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.751_f32 + y.sin();
        let b = y * 8.541_f32 - x.cos();
        let mut acc = Accumulator43::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.28_f32 + y.sin();
        let b = y * 1.829_f32 - x.cos();
        let mut acc = Accumulator43::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.76_f32 + y.sin();
        let b = y * 6.603_f32 - x.cos();
        let mut acc = Accumulator43::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_43(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m43-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_43() -> f32 {
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
        total += (dep_touch_43(total as u64) % 997) as f32;
        total
    }
}

pub mod m44 {
    use super::*;

    pub struct Accumulator44<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator44<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.89_f32 + y.sin();
        let b = y * 0.348_f32 - x.cos();
        let mut acc = Accumulator44::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.953_f32 + y.sin();
        let b = y * 2.535_f32 - x.cos();
        let mut acc = Accumulator44::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.551_f32 + y.sin();
        let b = y * 6.263_f32 - x.cos();
        let mut acc = Accumulator44::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.956_f32 + y.sin();
        let b = y * 2.755_f32 - x.cos();
        let mut acc = Accumulator44::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.294_f32 + y.sin();
        let b = y * 7.083_f32 - x.cos();
        let mut acc = Accumulator44::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.066_f32 + y.sin();
        let b = y * 6.064_f32 - x.cos();
        let mut acc = Accumulator44::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.041_f32 + y.sin();
        let b = y * 3.07_f32 - x.cos();
        let mut acc = Accumulator44::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.336_f32 + y.sin();
        let b = y * 5.513_f32 - x.cos();
        let mut acc = Accumulator44::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.355_f32 + y.sin();
        let b = y * 1.861_f32 - x.cos();
        let mut acc = Accumulator44::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.688_f32 + y.sin();
        let b = y * 7.123_f32 - x.cos();
        let mut acc = Accumulator44::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.172_f32 + y.sin();
        let b = y * 6.667_f32 - x.cos();
        let mut acc = Accumulator44::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.521_f32 + y.sin();
        let b = y * 0.11_f32 - x.cos();
        let mut acc = Accumulator44::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.906_f32 + y.sin();
        let b = y * 0.521_f32 - x.cos();
        let mut acc = Accumulator44::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.052_f32 + y.sin();
        let b = y * 0.564_f32 - x.cos();
        let mut acc = Accumulator44::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.774_f32 + y.sin();
        let b = y * 0.464_f32 - x.cos();
        let mut acc = Accumulator44::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.159_f32 + y.sin();
        let b = y * 9.062_f32 - x.cos();
        let mut acc = Accumulator44::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.054_f32 + y.sin();
        let b = y * 4.189_f32 - x.cos();
        let mut acc = Accumulator44::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.958_f32 + y.sin();
        let b = y * 7.426_f32 - x.cos();
        let mut acc = Accumulator44::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.855_f32 + y.sin();
        let b = y * 0.981_f32 - x.cos();
        let mut acc = Accumulator44::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.437_f32 + y.sin();
        let b = y * 3.435_f32 - x.cos();
        let mut acc = Accumulator44::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_44(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_44() -> f32 {
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
        total += (dep_touch_44(total as u64) % 997) as f32;
        total
    }
}

pub mod m45 {
    use super::*;

    pub struct Accumulator45<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator45<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.338_f32 + y.sin();
        let b = y * 8.234_f32 - x.cos();
        let mut acc = Accumulator45::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.234_f32 + y.sin();
        let b = y * 5.085_f32 - x.cos();
        let mut acc = Accumulator45::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.943_f32 + y.sin();
        let b = y * 4.298_f32 - x.cos();
        let mut acc = Accumulator45::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.396_f32 + y.sin();
        let b = y * 7.727_f32 - x.cos();
        let mut acc = Accumulator45::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.567_f32 + y.sin();
        let b = y * 0.528_f32 - x.cos();
        let mut acc = Accumulator45::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.187_f32 + y.sin();
        let b = y * 5.579_f32 - x.cos();
        let mut acc = Accumulator45::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.455_f32 + y.sin();
        let b = y * 9.6_f32 - x.cos();
        let mut acc = Accumulator45::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.413_f32 + y.sin();
        let b = y * 2.651_f32 - x.cos();
        let mut acc = Accumulator45::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.747_f32 + y.sin();
        let b = y * 3.503_f32 - x.cos();
        let mut acc = Accumulator45::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.074_f32 + y.sin();
        let b = y * 3.143_f32 - x.cos();
        let mut acc = Accumulator45::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.825_f32 + y.sin();
        let b = y * 4.867_f32 - x.cos();
        let mut acc = Accumulator45::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.431_f32 + y.sin();
        let b = y * 2.469_f32 - x.cos();
        let mut acc = Accumulator45::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.877_f32 + y.sin();
        let b = y * 1.404_f32 - x.cos();
        let mut acc = Accumulator45::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.544_f32 + y.sin();
        let b = y * 3.034_f32 - x.cos();
        let mut acc = Accumulator45::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.491_f32 + y.sin();
        let b = y * 5.62_f32 - x.cos();
        let mut acc = Accumulator45::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.268_f32 + y.sin();
        let b = y * 1.256_f32 - x.cos();
        let mut acc = Accumulator45::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.553_f32 + y.sin();
        let b = y * 1.758_f32 - x.cos();
        let mut acc = Accumulator45::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.617_f32 + y.sin();
        let b = y * 9.697_f32 - x.cos();
        let mut acc = Accumulator45::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.245_f32 + y.sin();
        let b = y * 1.222_f32 - x.cos();
        let mut acc = Accumulator45::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.221_f32 + y.sin();
        let b = y * 4.869_f32 - x.cos();
        let mut acc = Accumulator45::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_45(seed: u64) -> u64 {
        let re = Regex::new(r"m45-(\d+)").unwrap();
        let hay = format!("m45-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_45() -> f32 {
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
        total += (dep_touch_45(total as u64) % 997) as f32;
        total
    }
}

pub mod m46 {
    use super::*;

    pub struct Accumulator46<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator46<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.691_f32 + y.sin();
        let b = y * 2.853_f32 - x.cos();
        let mut acc = Accumulator46::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.184_f32 + y.sin();
        let b = y * 3.127_f32 - x.cos();
        let mut acc = Accumulator46::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.681_f32 + y.sin();
        let b = y * 4.462_f32 - x.cos();
        let mut acc = Accumulator46::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.657_f32 + y.sin();
        let b = y * 1.747_f32 - x.cos();
        let mut acc = Accumulator46::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.547_f32 + y.sin();
        let b = y * 4.691_f32 - x.cos();
        let mut acc = Accumulator46::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.858_f32 + y.sin();
        let b = y * 7.423_f32 - x.cos();
        let mut acc = Accumulator46::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.706_f32 + y.sin();
        let b = y * 5.624_f32 - x.cos();
        let mut acc = Accumulator46::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.422_f32 + y.sin();
        let b = y * 9.476_f32 - x.cos();
        let mut acc = Accumulator46::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.96_f32 + y.sin();
        let b = y * 5.422_f32 - x.cos();
        let mut acc = Accumulator46::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.621_f32 + y.sin();
        let b = y * 0.991_f32 - x.cos();
        let mut acc = Accumulator46::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.252_f32 + y.sin();
        let b = y * 9.265_f32 - x.cos();
        let mut acc = Accumulator46::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.954_f32 + y.sin();
        let b = y * 6.5_f32 - x.cos();
        let mut acc = Accumulator46::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.459_f32 + y.sin();
        let b = y * 0.371_f32 - x.cos();
        let mut acc = Accumulator46::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.328_f32 + y.sin();
        let b = y * 3.004_f32 - x.cos();
        let mut acc = Accumulator46::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.528_f32 + y.sin();
        let b = y * 5.249_f32 - x.cos();
        let mut acc = Accumulator46::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.751_f32 + y.sin();
        let b = y * 7.789_f32 - x.cos();
        let mut acc = Accumulator46::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.375_f32 + y.sin();
        let b = y * 3.301_f32 - x.cos();
        let mut acc = Accumulator46::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.444_f32 + y.sin();
        let b = y * 3.591_f32 - x.cos();
        let mut acc = Accumulator46::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.712_f32 + y.sin();
        let b = y * 5.501_f32 - x.cos();
        let mut acc = Accumulator46::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.311_f32 + y.sin();
        let b = y * 2.585_f32 - x.cos();
        let mut acc = Accumulator46::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_46(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_46() -> f32 {
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
        total += (dep_touch_46(total as u64) % 997) as f32;
        total
    }
}

pub mod m47 {
    use super::*;

    pub struct Accumulator47<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator47<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.504_f32 + y.sin();
        let b = y * 5.655_f32 - x.cos();
        let mut acc = Accumulator47::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.127_f32 + y.sin();
        let b = y * 9.899_f32 - x.cos();
        let mut acc = Accumulator47::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.971_f32 + y.sin();
        let b = y * 9.36_f32 - x.cos();
        let mut acc = Accumulator47::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.245_f32 + y.sin();
        let b = y * 7.261_f32 - x.cos();
        let mut acc = Accumulator47::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.006_f32 + y.sin();
        let b = y * 6.24_f32 - x.cos();
        let mut acc = Accumulator47::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.842_f32 + y.sin();
        let b = y * 1.704_f32 - x.cos();
        let mut acc = Accumulator47::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.34_f32 + y.sin();
        let b = y * 4.034_f32 - x.cos();
        let mut acc = Accumulator47::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.437_f32 + y.sin();
        let b = y * 4.798_f32 - x.cos();
        let mut acc = Accumulator47::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.185_f32 + y.sin();
        let b = y * 0.127_f32 - x.cos();
        let mut acc = Accumulator47::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.482_f32 + y.sin();
        let b = y * 7.812_f32 - x.cos();
        let mut acc = Accumulator47::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.503_f32 + y.sin();
        let b = y * 0.806_f32 - x.cos();
        let mut acc = Accumulator47::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.143_f32 + y.sin();
        let b = y * 4.228_f32 - x.cos();
        let mut acc = Accumulator47::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.715_f32 + y.sin();
        let b = y * 0.186_f32 - x.cos();
        let mut acc = Accumulator47::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.959_f32 + y.sin();
        let b = y * 1.712_f32 - x.cos();
        let mut acc = Accumulator47::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.141_f32 + y.sin();
        let b = y * 7.981_f32 - x.cos();
        let mut acc = Accumulator47::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.511_f32 + y.sin();
        let b = y * 2.015_f32 - x.cos();
        let mut acc = Accumulator47::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.419_f32 + y.sin();
        let b = y * 5.164_f32 - x.cos();
        let mut acc = Accumulator47::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.619_f32 + y.sin();
        let b = y * 9.005_f32 - x.cos();
        let mut acc = Accumulator47::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.649_f32 + y.sin();
        let b = y * 9.526_f32 - x.cos();
        let mut acc = Accumulator47::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.898_f32 + y.sin();
        let b = y * 8.035_f32 - x.cos();
        let mut acc = Accumulator47::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_47(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(47u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_47() -> f32 {
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
        total += (dep_touch_47(total as u64) % 997) as f32;
        total
    }
}

pub mod m48 {
    use super::*;

    pub struct Accumulator48<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator48<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.729_f32 + y.sin();
        let b = y * 6.07_f32 - x.cos();
        let mut acc = Accumulator48::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.637_f32 + y.sin();
        let b = y * 4.227_f32 - x.cos();
        let mut acc = Accumulator48::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.811_f32 + y.sin();
        let b = y * 6.444_f32 - x.cos();
        let mut acc = Accumulator48::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.58_f32 + y.sin();
        let b = y * 5.298_f32 - x.cos();
        let mut acc = Accumulator48::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.373_f32 + y.sin();
        let b = y * 1.951_f32 - x.cos();
        let mut acc = Accumulator48::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.349_f32 + y.sin();
        let b = y * 6.875_f32 - x.cos();
        let mut acc = Accumulator48::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.573_f32 + y.sin();
        let b = y * 3.746_f32 - x.cos();
        let mut acc = Accumulator48::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.343_f32 + y.sin();
        let b = y * 3.628_f32 - x.cos();
        let mut acc = Accumulator48::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.635_f32 + y.sin();
        let b = y * 3.984_f32 - x.cos();
        let mut acc = Accumulator48::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.327_f32 + y.sin();
        let b = y * 9.592_f32 - x.cos();
        let mut acc = Accumulator48::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.472_f32 + y.sin();
        let b = y * 1.2_f32 - x.cos();
        let mut acc = Accumulator48::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.467_f32 + y.sin();
        let b = y * 1.938_f32 - x.cos();
        let mut acc = Accumulator48::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.565_f32 + y.sin();
        let b = y * 4.895_f32 - x.cos();
        let mut acc = Accumulator48::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.59_f32 + y.sin();
        let b = y * 5.404_f32 - x.cos();
        let mut acc = Accumulator48::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.208_f32 + y.sin();
        let b = y * 5.129_f32 - x.cos();
        let mut acc = Accumulator48::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.655_f32 + y.sin();
        let b = y * 8.445_f32 - x.cos();
        let mut acc = Accumulator48::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.82_f32 + y.sin();
        let b = y * 4.407_f32 - x.cos();
        let mut acc = Accumulator48::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.496_f32 + y.sin();
        let b = y * 7.103_f32 - x.cos();
        let mut acc = Accumulator48::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.391_f32 + y.sin();
        let b = y * 6.241_f32 - x.cos();
        let mut acc = Accumulator48::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.149_f32 + y.sin();
        let b = y * 4.608_f32 - x.cos();
        let mut acc = Accumulator48::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_48(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_48() -> f32 {
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
        total += (dep_touch_48(total as u64) % 997) as f32;
        total
    }
}

pub mod m49 {
    use super::*;

    pub struct Accumulator49<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator49<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.941_f32 + y.sin();
        let b = y * 1.778_f32 - x.cos();
        let mut acc = Accumulator49::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.618_f32 + y.sin();
        let b = y * 7.695_f32 - x.cos();
        let mut acc = Accumulator49::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.759_f32 + y.sin();
        let b = y * 6.164_f32 - x.cos();
        let mut acc = Accumulator49::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.422_f32 + y.sin();
        let b = y * 3.749_f32 - x.cos();
        let mut acc = Accumulator49::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.979_f32 + y.sin();
        let b = y * 9.465_f32 - x.cos();
        let mut acc = Accumulator49::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.039_f32 + y.sin();
        let b = y * 7.033_f32 - x.cos();
        let mut acc = Accumulator49::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.004_f32 + y.sin();
        let b = y * 0.96_f32 - x.cos();
        let mut acc = Accumulator49::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.617_f32 + y.sin();
        let b = y * 9.092_f32 - x.cos();
        let mut acc = Accumulator49::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.746_f32 + y.sin();
        let b = y * 7.405_f32 - x.cos();
        let mut acc = Accumulator49::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.98_f32 + y.sin();
        let b = y * 7.313_f32 - x.cos();
        let mut acc = Accumulator49::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.191_f32 + y.sin();
        let b = y * 1.303_f32 - x.cos();
        let mut acc = Accumulator49::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.129_f32 + y.sin();
        let b = y * 5.242_f32 - x.cos();
        let mut acc = Accumulator49::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.173_f32 + y.sin();
        let b = y * 0.526_f32 - x.cos();
        let mut acc = Accumulator49::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.564_f32 + y.sin();
        let b = y * 3.586_f32 - x.cos();
        let mut acc = Accumulator49::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.97_f32 + y.sin();
        let b = y * 1.207_f32 - x.cos();
        let mut acc = Accumulator49::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.476_f32 + y.sin();
        let b = y * 3.573_f32 - x.cos();
        let mut acc = Accumulator49::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.478_f32 + y.sin();
        let b = y * 8.314_f32 - x.cos();
        let mut acc = Accumulator49::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.411_f32 + y.sin();
        let b = y * 7.854_f32 - x.cos();
        let mut acc = Accumulator49::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.182_f32 + y.sin();
        let b = y * 7.947_f32 - x.cos();
        let mut acc = Accumulator49::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.572_f32 + y.sin();
        let b = y * 9.225_f32 - x.cos();
        let mut acc = Accumulator49::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_49(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_49() -> f32 {
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
        total += (dep_touch_49(total as u64) % 997) as f32;
        total
    }
}

pub mod m50 {
    use super::*;

    pub struct Accumulator50<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator50<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.662_f32 + y.sin();
        let b = y * 8.295_f32 - x.cos();
        let mut acc = Accumulator50::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.679_f32 + y.sin();
        let b = y * 5.377_f32 - x.cos();
        let mut acc = Accumulator50::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.562_f32 + y.sin();
        let b = y * 2.733_f32 - x.cos();
        let mut acc = Accumulator50::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.235_f32 + y.sin();
        let b = y * 3.455_f32 - x.cos();
        let mut acc = Accumulator50::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.874_f32 + y.sin();
        let b = y * 4.843_f32 - x.cos();
        let mut acc = Accumulator50::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.944_f32 + y.sin();
        let b = y * 0.447_f32 - x.cos();
        let mut acc = Accumulator50::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.18_f32 + y.sin();
        let b = y * 6.418_f32 - x.cos();
        let mut acc = Accumulator50::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.947_f32 + y.sin();
        let b = y * 4.762_f32 - x.cos();
        let mut acc = Accumulator50::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.891_f32 + y.sin();
        let b = y * 4.496_f32 - x.cos();
        let mut acc = Accumulator50::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.656_f32 + y.sin();
        let b = y * 6.086_f32 - x.cos();
        let mut acc = Accumulator50::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.637_f32 + y.sin();
        let b = y * 5.277_f32 - x.cos();
        let mut acc = Accumulator50::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.861_f32 + y.sin();
        let b = y * 5.874_f32 - x.cos();
        let mut acc = Accumulator50::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.172_f32 + y.sin();
        let b = y * 8.128_f32 - x.cos();
        let mut acc = Accumulator50::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.684_f32 + y.sin();
        let b = y * 5.755_f32 - x.cos();
        let mut acc = Accumulator50::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.287_f32 + y.sin();
        let b = y * 3.181_f32 - x.cos();
        let mut acc = Accumulator50::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.776_f32 + y.sin();
        let b = y * 3.371_f32 - x.cos();
        let mut acc = Accumulator50::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.538_f32 + y.sin();
        let b = y * 8.688_f32 - x.cos();
        let mut acc = Accumulator50::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.545_f32 + y.sin();
        let b = y * 1.529_f32 - x.cos();
        let mut acc = Accumulator50::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.508_f32 + y.sin();
        let b = y * 3.867_f32 - x.cos();
        let mut acc = Accumulator50::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.0_f32 + y.sin();
        let b = y * 2.668_f32 - x.cos();
        let mut acc = Accumulator50::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_50(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m50-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_50() -> f32 {
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
        total += (dep_touch_50(total as u64) % 997) as f32;
        total
    }
}

pub mod m51 {
    use super::*;

    pub struct Accumulator51<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator51<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.721_f32 + y.sin();
        let b = y * 5.827_f32 - x.cos();
        let mut acc = Accumulator51::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.823_f32 + y.sin();
        let b = y * 0.434_f32 - x.cos();
        let mut acc = Accumulator51::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.946_f32 + y.sin();
        let b = y * 1.033_f32 - x.cos();
        let mut acc = Accumulator51::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.688_f32 + y.sin();
        let b = y * 6.304_f32 - x.cos();
        let mut acc = Accumulator51::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.07_f32 + y.sin();
        let b = y * 7.861_f32 - x.cos();
        let mut acc = Accumulator51::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.654_f32 + y.sin();
        let b = y * 9.299_f32 - x.cos();
        let mut acc = Accumulator51::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.806_f32 + y.sin();
        let b = y * 9.537_f32 - x.cos();
        let mut acc = Accumulator51::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.347_f32 + y.sin();
        let b = y * 5.087_f32 - x.cos();
        let mut acc = Accumulator51::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.503_f32 + y.sin();
        let b = y * 8.654_f32 - x.cos();
        let mut acc = Accumulator51::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.078_f32 + y.sin();
        let b = y * 0.692_f32 - x.cos();
        let mut acc = Accumulator51::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.683_f32 + y.sin();
        let b = y * 8.76_f32 - x.cos();
        let mut acc = Accumulator51::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.037_f32 + y.sin();
        let b = y * 8.863_f32 - x.cos();
        let mut acc = Accumulator51::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.867_f32 + y.sin();
        let b = y * 1.578_f32 - x.cos();
        let mut acc = Accumulator51::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.171_f32 + y.sin();
        let b = y * 8.636_f32 - x.cos();
        let mut acc = Accumulator51::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.069_f32 + y.sin();
        let b = y * 3.982_f32 - x.cos();
        let mut acc = Accumulator51::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.209_f32 + y.sin();
        let b = y * 5.851_f32 - x.cos();
        let mut acc = Accumulator51::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.055_f32 + y.sin();
        let b = y * 3.123_f32 - x.cos();
        let mut acc = Accumulator51::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.705_f32 + y.sin();
        let b = y * 6.557_f32 - x.cos();
        let mut acc = Accumulator51::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.82_f32 + y.sin();
        let b = y * 1.964_f32 - x.cos();
        let mut acc = Accumulator51::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.832_f32 + y.sin();
        let b = y * 3.082_f32 - x.cos();
        let mut acc = Accumulator51::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_51(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_51() -> f32 {
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
        total += (dep_touch_51(total as u64) % 997) as f32;
        total
    }
}

pub mod m52 {
    use super::*;

    pub struct Accumulator52<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator52<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.777_f32 + y.sin();
        let b = y * 3.598_f32 - x.cos();
        let mut acc = Accumulator52::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.297_f32 + y.sin();
        let b = y * 8.609_f32 - x.cos();
        let mut acc = Accumulator52::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.318_f32 + y.sin();
        let b = y * 3.229_f32 - x.cos();
        let mut acc = Accumulator52::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.804_f32 + y.sin();
        let b = y * 2.624_f32 - x.cos();
        let mut acc = Accumulator52::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.533_f32 + y.sin();
        let b = y * 6.239_f32 - x.cos();
        let mut acc = Accumulator52::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.003_f32 + y.sin();
        let b = y * 2.239_f32 - x.cos();
        let mut acc = Accumulator52::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.49_f32 + y.sin();
        let b = y * 8.85_f32 - x.cos();
        let mut acc = Accumulator52::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.568_f32 + y.sin();
        let b = y * 6.451_f32 - x.cos();
        let mut acc = Accumulator52::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.469_f32 + y.sin();
        let b = y * 0.657_f32 - x.cos();
        let mut acc = Accumulator52::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.99_f32 + y.sin();
        let b = y * 0.877_f32 - x.cos();
        let mut acc = Accumulator52::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.443_f32 + y.sin();
        let b = y * 1.439_f32 - x.cos();
        let mut acc = Accumulator52::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.752_f32 + y.sin();
        let b = y * 6.396_f32 - x.cos();
        let mut acc = Accumulator52::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.371_f32 + y.sin();
        let b = y * 9.144_f32 - x.cos();
        let mut acc = Accumulator52::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.251_f32 + y.sin();
        let b = y * 8.604_f32 - x.cos();
        let mut acc = Accumulator52::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.46_f32 + y.sin();
        let b = y * 4.072_f32 - x.cos();
        let mut acc = Accumulator52::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.944_f32 + y.sin();
        let b = y * 1.81_f32 - x.cos();
        let mut acc = Accumulator52::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.903_f32 + y.sin();
        let b = y * 0.955_f32 - x.cos();
        let mut acc = Accumulator52::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.378_f32 + y.sin();
        let b = y * 4.945_f32 - x.cos();
        let mut acc = Accumulator52::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.016_f32 + y.sin();
        let b = y * 9.314_f32 - x.cos();
        let mut acc = Accumulator52::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.352_f32 + y.sin();
        let b = y * 3.206_f32 - x.cos();
        let mut acc = Accumulator52::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_52(seed: u64) -> u64 {
        let re = Regex::new(r"m52-(\d+)").unwrap();
        let hay = format!("m52-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_52() -> f32 {
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
        total += (dep_touch_52(total as u64) % 997) as f32;
        total
    }
}

pub mod m53 {
    use super::*;

    pub struct Accumulator53<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator53<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.778_f32 + y.sin();
        let b = y * 0.649_f32 - x.cos();
        let mut acc = Accumulator53::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.059_f32 + y.sin();
        let b = y * 8.282_f32 - x.cos();
        let mut acc = Accumulator53::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.016_f32 + y.sin();
        let b = y * 1.631_f32 - x.cos();
        let mut acc = Accumulator53::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.289_f32 + y.sin();
        let b = y * 8.338_f32 - x.cos();
        let mut acc = Accumulator53::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.077_f32 + y.sin();
        let b = y * 4.248_f32 - x.cos();
        let mut acc = Accumulator53::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.765_f32 + y.sin();
        let b = y * 8.586_f32 - x.cos();
        let mut acc = Accumulator53::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.542_f32 + y.sin();
        let b = y * 5.995_f32 - x.cos();
        let mut acc = Accumulator53::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.354_f32 + y.sin();
        let b = y * 6.163_f32 - x.cos();
        let mut acc = Accumulator53::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.78_f32 + y.sin();
        let b = y * 0.41_f32 - x.cos();
        let mut acc = Accumulator53::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.131_f32 + y.sin();
        let b = y * 7.672_f32 - x.cos();
        let mut acc = Accumulator53::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.541_f32 + y.sin();
        let b = y * 2.827_f32 - x.cos();
        let mut acc = Accumulator53::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.29_f32 + y.sin();
        let b = y * 2.784_f32 - x.cos();
        let mut acc = Accumulator53::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.189_f32 + y.sin();
        let b = y * 4.762_f32 - x.cos();
        let mut acc = Accumulator53::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.03_f32 + y.sin();
        let b = y * 9.673_f32 - x.cos();
        let mut acc = Accumulator53::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.797_f32 + y.sin();
        let b = y * 2.336_f32 - x.cos();
        let mut acc = Accumulator53::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.695_f32 + y.sin();
        let b = y * 9.282_f32 - x.cos();
        let mut acc = Accumulator53::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.415_f32 + y.sin();
        let b = y * 0.69_f32 - x.cos();
        let mut acc = Accumulator53::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.108_f32 + y.sin();
        let b = y * 7.718_f32 - x.cos();
        let mut acc = Accumulator53::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.31_f32 + y.sin();
        let b = y * 3.683_f32 - x.cos();
        let mut acc = Accumulator53::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.948_f32 + y.sin();
        let b = y * 8.634_f32 - x.cos();
        let mut acc = Accumulator53::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_53(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_53() -> f32 {
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
        total += (dep_touch_53(total as u64) % 997) as f32;
        total
    }
}

pub mod m54 {
    use super::*;

    pub struct Accumulator54<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator54<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.279_f32 + y.sin();
        let b = y * 3.538_f32 - x.cos();
        let mut acc = Accumulator54::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.477_f32 + y.sin();
        let b = y * 9.727_f32 - x.cos();
        let mut acc = Accumulator54::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.189_f32 + y.sin();
        let b = y * 9.041_f32 - x.cos();
        let mut acc = Accumulator54::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.609_f32 + y.sin();
        let b = y * 3.274_f32 - x.cos();
        let mut acc = Accumulator54::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.156_f32 + y.sin();
        let b = y * 7.287_f32 - x.cos();
        let mut acc = Accumulator54::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.006_f32 + y.sin();
        let b = y * 9.054_f32 - x.cos();
        let mut acc = Accumulator54::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.568_f32 + y.sin();
        let b = y * 0.688_f32 - x.cos();
        let mut acc = Accumulator54::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.785_f32 + y.sin();
        let b = y * 8.989_f32 - x.cos();
        let mut acc = Accumulator54::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.361_f32 + y.sin();
        let b = y * 6.246_f32 - x.cos();
        let mut acc = Accumulator54::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.401_f32 + y.sin();
        let b = y * 0.303_f32 - x.cos();
        let mut acc = Accumulator54::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.351_f32 + y.sin();
        let b = y * 2.368_f32 - x.cos();
        let mut acc = Accumulator54::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.295_f32 + y.sin();
        let b = y * 4.271_f32 - x.cos();
        let mut acc = Accumulator54::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.025_f32 + y.sin();
        let b = y * 9.844_f32 - x.cos();
        let mut acc = Accumulator54::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.717_f32 + y.sin();
        let b = y * 7.85_f32 - x.cos();
        let mut acc = Accumulator54::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.558_f32 + y.sin();
        let b = y * 8.619_f32 - x.cos();
        let mut acc = Accumulator54::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.452_f32 + y.sin();
        let b = y * 1.438_f32 - x.cos();
        let mut acc = Accumulator54::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.869_f32 + y.sin();
        let b = y * 3.349_f32 - x.cos();
        let mut acc = Accumulator54::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.647_f32 + y.sin();
        let b = y * 2.547_f32 - x.cos();
        let mut acc = Accumulator54::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.791_f32 + y.sin();
        let b = y * 5.956_f32 - x.cos();
        let mut acc = Accumulator54::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.178_f32 + y.sin();
        let b = y * 0.611_f32 - x.cos();
        let mut acc = Accumulator54::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_54(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(54u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_54() -> f32 {
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
        total += (dep_touch_54(total as u64) % 997) as f32;
        total
    }
}

pub mod m55 {
    use super::*;

    pub struct Accumulator55<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator55<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.443_f32 + y.sin();
        let b = y * 7.308_f32 - x.cos();
        let mut acc = Accumulator55::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.742_f32 + y.sin();
        let b = y * 8.561_f32 - x.cos();
        let mut acc = Accumulator55::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.361_f32 + y.sin();
        let b = y * 1.47_f32 - x.cos();
        let mut acc = Accumulator55::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.815_f32 + y.sin();
        let b = y * 7.989_f32 - x.cos();
        let mut acc = Accumulator55::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.648_f32 + y.sin();
        let b = y * 0.192_f32 - x.cos();
        let mut acc = Accumulator55::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.578_f32 + y.sin();
        let b = y * 7.313_f32 - x.cos();
        let mut acc = Accumulator55::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.463_f32 + y.sin();
        let b = y * 4.652_f32 - x.cos();
        let mut acc = Accumulator55::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.984_f32 + y.sin();
        let b = y * 3.427_f32 - x.cos();
        let mut acc = Accumulator55::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.12_f32 + y.sin();
        let b = y * 8.743_f32 - x.cos();
        let mut acc = Accumulator55::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.836_f32 + y.sin();
        let b = y * 2.074_f32 - x.cos();
        let mut acc = Accumulator55::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.857_f32 + y.sin();
        let b = y * 0.471_f32 - x.cos();
        let mut acc = Accumulator55::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.933_f32 + y.sin();
        let b = y * 5.734_f32 - x.cos();
        let mut acc = Accumulator55::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.128_f32 + y.sin();
        let b = y * 0.295_f32 - x.cos();
        let mut acc = Accumulator55::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.733_f32 + y.sin();
        let b = y * 1.181_f32 - x.cos();
        let mut acc = Accumulator55::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.612_f32 + y.sin();
        let b = y * 5.249_f32 - x.cos();
        let mut acc = Accumulator55::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.854_f32 + y.sin();
        let b = y * 6.817_f32 - x.cos();
        let mut acc = Accumulator55::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.305_f32 + y.sin();
        let b = y * 5.446_f32 - x.cos();
        let mut acc = Accumulator55::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.293_f32 + y.sin();
        let b = y * 8.328_f32 - x.cos();
        let mut acc = Accumulator55::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.525_f32 + y.sin();
        let b = y * 2.208_f32 - x.cos();
        let mut acc = Accumulator55::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.268_f32 + y.sin();
        let b = y * 2.775_f32 - x.cos();
        let mut acc = Accumulator55::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_55(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_55() -> f32 {
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
        total += (dep_touch_55(total as u64) % 997) as f32;
        total
    }
}

pub mod m56 {
    use super::*;

    pub struct Accumulator56<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator56<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.249_f32 + y.sin();
        let b = y * 2.736_f32 - x.cos();
        let mut acc = Accumulator56::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.025_f32 + y.sin();
        let b = y * 0.569_f32 - x.cos();
        let mut acc = Accumulator56::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.429_f32 + y.sin();
        let b = y * 2.719_f32 - x.cos();
        let mut acc = Accumulator56::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.844_f32 + y.sin();
        let b = y * 6.501_f32 - x.cos();
        let mut acc = Accumulator56::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.865_f32 + y.sin();
        let b = y * 3.341_f32 - x.cos();
        let mut acc = Accumulator56::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.727_f32 + y.sin();
        let b = y * 8.666_f32 - x.cos();
        let mut acc = Accumulator56::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.732_f32 + y.sin();
        let b = y * 4.235_f32 - x.cos();
        let mut acc = Accumulator56::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.208_f32 + y.sin();
        let b = y * 9.635_f32 - x.cos();
        let mut acc = Accumulator56::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.557_f32 + y.sin();
        let b = y * 8.74_f32 - x.cos();
        let mut acc = Accumulator56::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.901_f32 + y.sin();
        let b = y * 6.323_f32 - x.cos();
        let mut acc = Accumulator56::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.057_f32 + y.sin();
        let b = y * 9.178_f32 - x.cos();
        let mut acc = Accumulator56::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.898_f32 + y.sin();
        let b = y * 7.254_f32 - x.cos();
        let mut acc = Accumulator56::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.19_f32 + y.sin();
        let b = y * 6.602_f32 - x.cos();
        let mut acc = Accumulator56::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.362_f32 + y.sin();
        let b = y * 7.783_f32 - x.cos();
        let mut acc = Accumulator56::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.585_f32 + y.sin();
        let b = y * 6.903_f32 - x.cos();
        let mut acc = Accumulator56::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.812_f32 + y.sin();
        let b = y * 4.436_f32 - x.cos();
        let mut acc = Accumulator56::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.193_f32 + y.sin();
        let b = y * 9.622_f32 - x.cos();
        let mut acc = Accumulator56::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.74_f32 + y.sin();
        let b = y * 6.444_f32 - x.cos();
        let mut acc = Accumulator56::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.099_f32 + y.sin();
        let b = y * 5.904_f32 - x.cos();
        let mut acc = Accumulator56::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.397_f32 + y.sin();
        let b = y * 6.269_f32 - x.cos();
        let mut acc = Accumulator56::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_56(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_56() -> f32 {
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
        total += (dep_touch_56(total as u64) % 997) as f32;
        total
    }
}

pub mod m57 {
    use super::*;

    pub struct Accumulator57<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator57<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.62_f32 + y.sin();
        let b = y * 3.581_f32 - x.cos();
        let mut acc = Accumulator57::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.957_f32 + y.sin();
        let b = y * 5.257_f32 - x.cos();
        let mut acc = Accumulator57::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.564_f32 + y.sin();
        let b = y * 8.197_f32 - x.cos();
        let mut acc = Accumulator57::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.263_f32 + y.sin();
        let b = y * 5.422_f32 - x.cos();
        let mut acc = Accumulator57::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.155_f32 + y.sin();
        let b = y * 7.601_f32 - x.cos();
        let mut acc = Accumulator57::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.006_f32 + y.sin();
        let b = y * 4.738_f32 - x.cos();
        let mut acc = Accumulator57::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.508_f32 + y.sin();
        let b = y * 5.877_f32 - x.cos();
        let mut acc = Accumulator57::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.268_f32 + y.sin();
        let b = y * 1.492_f32 - x.cos();
        let mut acc = Accumulator57::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.282_f32 + y.sin();
        let b = y * 5.235_f32 - x.cos();
        let mut acc = Accumulator57::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.758_f32 + y.sin();
        let b = y * 3.685_f32 - x.cos();
        let mut acc = Accumulator57::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.789_f32 + y.sin();
        let b = y * 8.154_f32 - x.cos();
        let mut acc = Accumulator57::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.842_f32 + y.sin();
        let b = y * 9.384_f32 - x.cos();
        let mut acc = Accumulator57::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.604_f32 + y.sin();
        let b = y * 0.524_f32 - x.cos();
        let mut acc = Accumulator57::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.645_f32 + y.sin();
        let b = y * 8.563_f32 - x.cos();
        let mut acc = Accumulator57::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.306_f32 + y.sin();
        let b = y * 1.608_f32 - x.cos();
        let mut acc = Accumulator57::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.776_f32 + y.sin();
        let b = y * 3.675_f32 - x.cos();
        let mut acc = Accumulator57::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.971_f32 + y.sin();
        let b = y * 5.209_f32 - x.cos();
        let mut acc = Accumulator57::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.59_f32 + y.sin();
        let b = y * 2.795_f32 - x.cos();
        let mut acc = Accumulator57::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.87_f32 + y.sin();
        let b = y * 6.911_f32 - x.cos();
        let mut acc = Accumulator57::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.319_f32 + y.sin();
        let b = y * 7.261_f32 - x.cos();
        let mut acc = Accumulator57::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_57(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m57-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_57() -> f32 {
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
        total += (dep_touch_57(total as u64) % 997) as f32;
        total
    }
}

pub mod m58 {
    use super::*;

    pub struct Accumulator58<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator58<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.536_f32 + y.sin();
        let b = y * 1.569_f32 - x.cos();
        let mut acc = Accumulator58::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.379_f32 + y.sin();
        let b = y * 4.89_f32 - x.cos();
        let mut acc = Accumulator58::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.429_f32 + y.sin();
        let b = y * 3.734_f32 - x.cos();
        let mut acc = Accumulator58::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.955_f32 + y.sin();
        let b = y * 2.578_f32 - x.cos();
        let mut acc = Accumulator58::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.068_f32 + y.sin();
        let b = y * 5.691_f32 - x.cos();
        let mut acc = Accumulator58::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.888_f32 + y.sin();
        let b = y * 3.104_f32 - x.cos();
        let mut acc = Accumulator58::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.791_f32 + y.sin();
        let b = y * 3.275_f32 - x.cos();
        let mut acc = Accumulator58::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.701_f32 + y.sin();
        let b = y * 4.393_f32 - x.cos();
        let mut acc = Accumulator58::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.334_f32 + y.sin();
        let b = y * 8.517_f32 - x.cos();
        let mut acc = Accumulator58::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.357_f32 + y.sin();
        let b = y * 9.492_f32 - x.cos();
        let mut acc = Accumulator58::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.155_f32 + y.sin();
        let b = y * 3.742_f32 - x.cos();
        let mut acc = Accumulator58::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.131_f32 + y.sin();
        let b = y * 3.782_f32 - x.cos();
        let mut acc = Accumulator58::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.083_f32 + y.sin();
        let b = y * 2.993_f32 - x.cos();
        let mut acc = Accumulator58::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.323_f32 + y.sin();
        let b = y * 6.053_f32 - x.cos();
        let mut acc = Accumulator58::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.553_f32 + y.sin();
        let b = y * 3.877_f32 - x.cos();
        let mut acc = Accumulator58::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.369_f32 + y.sin();
        let b = y * 6.162_f32 - x.cos();
        let mut acc = Accumulator58::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.786_f32 + y.sin();
        let b = y * 0.721_f32 - x.cos();
        let mut acc = Accumulator58::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.329_f32 + y.sin();
        let b = y * 0.794_f32 - x.cos();
        let mut acc = Accumulator58::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.466_f32 + y.sin();
        let b = y * 3.954_f32 - x.cos();
        let mut acc = Accumulator58::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.967_f32 + y.sin();
        let b = y * 8.933_f32 - x.cos();
        let mut acc = Accumulator58::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_58(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_58() -> f32 {
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
        total += (dep_touch_58(total as u64) % 997) as f32;
        total
    }
}

pub mod m59 {
    use super::*;

    pub struct Accumulator59<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator59<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.858_f32 + y.sin();
        let b = y * 1.157_f32 - x.cos();
        let mut acc = Accumulator59::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.633_f32 + y.sin();
        let b = y * 4.629_f32 - x.cos();
        let mut acc = Accumulator59::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.166_f32 + y.sin();
        let b = y * 4.741_f32 - x.cos();
        let mut acc = Accumulator59::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.41_f32 + y.sin();
        let b = y * 4.914_f32 - x.cos();
        let mut acc = Accumulator59::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.477_f32 + y.sin();
        let b = y * 0.193_f32 - x.cos();
        let mut acc = Accumulator59::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.356_f32 + y.sin();
        let b = y * 4.036_f32 - x.cos();
        let mut acc = Accumulator59::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.173_f32 + y.sin();
        let b = y * 2.981_f32 - x.cos();
        let mut acc = Accumulator59::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.638_f32 + y.sin();
        let b = y * 7.645_f32 - x.cos();
        let mut acc = Accumulator59::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.983_f32 + y.sin();
        let b = y * 8.409_f32 - x.cos();
        let mut acc = Accumulator59::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.111_f32 + y.sin();
        let b = y * 1.097_f32 - x.cos();
        let mut acc = Accumulator59::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.412_f32 + y.sin();
        let b = y * 2.213_f32 - x.cos();
        let mut acc = Accumulator59::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.639_f32 + y.sin();
        let b = y * 6.774_f32 - x.cos();
        let mut acc = Accumulator59::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.389_f32 + y.sin();
        let b = y * 8.557_f32 - x.cos();
        let mut acc = Accumulator59::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.872_f32 + y.sin();
        let b = y * 4.196_f32 - x.cos();
        let mut acc = Accumulator59::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.474_f32 + y.sin();
        let b = y * 4.088_f32 - x.cos();
        let mut acc = Accumulator59::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.645_f32 + y.sin();
        let b = y * 1.526_f32 - x.cos();
        let mut acc = Accumulator59::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.964_f32 + y.sin();
        let b = y * 9.726_f32 - x.cos();
        let mut acc = Accumulator59::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.789_f32 + y.sin();
        let b = y * 2.792_f32 - x.cos();
        let mut acc = Accumulator59::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.949_f32 + y.sin();
        let b = y * 3.86_f32 - x.cos();
        let mut acc = Accumulator59::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.517_f32 + y.sin();
        let b = y * 5.546_f32 - x.cos();
        let mut acc = Accumulator59::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_59(seed: u64) -> u64 {
        let re = Regex::new(r"m59-(\d+)").unwrap();
        let hay = format!("m59-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_59() -> f32 {
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
        total += (dep_touch_59(total as u64) % 997) as f32;
        total
    }
}

pub mod m60 {
    use super::*;

    pub struct Accumulator60<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator60<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.781_f32 + y.sin();
        let b = y * 6.774_f32 - x.cos();
        let mut acc = Accumulator60::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.084_f32 + y.sin();
        let b = y * 8.594_f32 - x.cos();
        let mut acc = Accumulator60::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.495_f32 + y.sin();
        let b = y * 2.619_f32 - x.cos();
        let mut acc = Accumulator60::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.391_f32 + y.sin();
        let b = y * 2.134_f32 - x.cos();
        let mut acc = Accumulator60::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.763_f32 + y.sin();
        let b = y * 4.649_f32 - x.cos();
        let mut acc = Accumulator60::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.057_f32 + y.sin();
        let b = y * 1.485_f32 - x.cos();
        let mut acc = Accumulator60::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.063_f32 + y.sin();
        let b = y * 9.11_f32 - x.cos();
        let mut acc = Accumulator60::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.606_f32 + y.sin();
        let b = y * 7.247_f32 - x.cos();
        let mut acc = Accumulator60::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.324_f32 + y.sin();
        let b = y * 4.108_f32 - x.cos();
        let mut acc = Accumulator60::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.169_f32 + y.sin();
        let b = y * 0.446_f32 - x.cos();
        let mut acc = Accumulator60::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.902_f32 + y.sin();
        let b = y * 2.957_f32 - x.cos();
        let mut acc = Accumulator60::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.152_f32 + y.sin();
        let b = y * 9.637_f32 - x.cos();
        let mut acc = Accumulator60::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.555_f32 + y.sin();
        let b = y * 9.262_f32 - x.cos();
        let mut acc = Accumulator60::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.098_f32 + y.sin();
        let b = y * 2.091_f32 - x.cos();
        let mut acc = Accumulator60::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.35_f32 + y.sin();
        let b = y * 6.365_f32 - x.cos();
        let mut acc = Accumulator60::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.443_f32 + y.sin();
        let b = y * 8.723_f32 - x.cos();
        let mut acc = Accumulator60::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.972_f32 + y.sin();
        let b = y * 0.239_f32 - x.cos();
        let mut acc = Accumulator60::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.598_f32 + y.sin();
        let b = y * 7.947_f32 - x.cos();
        let mut acc = Accumulator60::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.264_f32 + y.sin();
        let b = y * 7.16_f32 - x.cos();
        let mut acc = Accumulator60::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.99_f32 + y.sin();
        let b = y * 2.168_f32 - x.cos();
        let mut acc = Accumulator60::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_60(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_60() -> f32 {
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
        total += (dep_touch_60(total as u64) % 997) as f32;
        total
    }
}

pub mod m61 {
    use super::*;

    pub struct Accumulator61<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator61<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.529_f32 + y.sin();
        let b = y * 9.095_f32 - x.cos();
        let mut acc = Accumulator61::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.159_f32 + y.sin();
        let b = y * 4.663_f32 - x.cos();
        let mut acc = Accumulator61::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.786_f32 + y.sin();
        let b = y * 9.419_f32 - x.cos();
        let mut acc = Accumulator61::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.293_f32 + y.sin();
        let b = y * 8.269_f32 - x.cos();
        let mut acc = Accumulator61::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.978_f32 + y.sin();
        let b = y * 6.822_f32 - x.cos();
        let mut acc = Accumulator61::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.608_f32 + y.sin();
        let b = y * 9.154_f32 - x.cos();
        let mut acc = Accumulator61::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.509_f32 + y.sin();
        let b = y * 8.294_f32 - x.cos();
        let mut acc = Accumulator61::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.53_f32 + y.sin();
        let b = y * 8.672_f32 - x.cos();
        let mut acc = Accumulator61::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.028_f32 + y.sin();
        let b = y * 7.15_f32 - x.cos();
        let mut acc = Accumulator61::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.037_f32 + y.sin();
        let b = y * 3.279_f32 - x.cos();
        let mut acc = Accumulator61::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.589_f32 + y.sin();
        let b = y * 7.934_f32 - x.cos();
        let mut acc = Accumulator61::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.937_f32 + y.sin();
        let b = y * 0.423_f32 - x.cos();
        let mut acc = Accumulator61::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.629_f32 + y.sin();
        let b = y * 2.952_f32 - x.cos();
        let mut acc = Accumulator61::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.448_f32 + y.sin();
        let b = y * 1.017_f32 - x.cos();
        let mut acc = Accumulator61::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.559_f32 + y.sin();
        let b = y * 1.903_f32 - x.cos();
        let mut acc = Accumulator61::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.753_f32 + y.sin();
        let b = y * 1.221_f32 - x.cos();
        let mut acc = Accumulator61::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.297_f32 + y.sin();
        let b = y * 9.149_f32 - x.cos();
        let mut acc = Accumulator61::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.306_f32 + y.sin();
        let b = y * 5.258_f32 - x.cos();
        let mut acc = Accumulator61::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.901_f32 + y.sin();
        let b = y * 0.274_f32 - x.cos();
        let mut acc = Accumulator61::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.816_f32 + y.sin();
        let b = y * 9.047_f32 - x.cos();
        let mut acc = Accumulator61::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_61(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(61u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_61() -> f32 {
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
        total += (dep_touch_61(total as u64) % 997) as f32;
        total
    }
}

pub mod m62 {
    use super::*;

    pub struct Accumulator62<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator62<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.85_f32 + y.sin();
        let b = y * 8.567_f32 - x.cos();
        let mut acc = Accumulator62::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.822_f32 + y.sin();
        let b = y * 7.511_f32 - x.cos();
        let mut acc = Accumulator62::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.469_f32 + y.sin();
        let b = y * 2.758_f32 - x.cos();
        let mut acc = Accumulator62::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.772_f32 + y.sin();
        let b = y * 3.039_f32 - x.cos();
        let mut acc = Accumulator62::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.968_f32 + y.sin();
        let b = y * 9.58_f32 - x.cos();
        let mut acc = Accumulator62::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.005_f32 + y.sin();
        let b = y * 1.592_f32 - x.cos();
        let mut acc = Accumulator62::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.591_f32 + y.sin();
        let b = y * 3.365_f32 - x.cos();
        let mut acc = Accumulator62::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.814_f32 + y.sin();
        let b = y * 3.33_f32 - x.cos();
        let mut acc = Accumulator62::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.853_f32 + y.sin();
        let b = y * 7.82_f32 - x.cos();
        let mut acc = Accumulator62::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.028_f32 + y.sin();
        let b = y * 7.192_f32 - x.cos();
        let mut acc = Accumulator62::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.038_f32 + y.sin();
        let b = y * 5.505_f32 - x.cos();
        let mut acc = Accumulator62::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.937_f32 + y.sin();
        let b = y * 4.622_f32 - x.cos();
        let mut acc = Accumulator62::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.488_f32 + y.sin();
        let b = y * 5.777_f32 - x.cos();
        let mut acc = Accumulator62::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.439_f32 + y.sin();
        let b = y * 1.393_f32 - x.cos();
        let mut acc = Accumulator62::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.304_f32 + y.sin();
        let b = y * 0.847_f32 - x.cos();
        let mut acc = Accumulator62::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.599_f32 + y.sin();
        let b = y * 1.706_f32 - x.cos();
        let mut acc = Accumulator62::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.595_f32 + y.sin();
        let b = y * 0.982_f32 - x.cos();
        let mut acc = Accumulator62::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.355_f32 + y.sin();
        let b = y * 8.66_f32 - x.cos();
        let mut acc = Accumulator62::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.073_f32 + y.sin();
        let b = y * 2.661_f32 - x.cos();
        let mut acc = Accumulator62::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.715_f32 + y.sin();
        let b = y * 1.247_f32 - x.cos();
        let mut acc = Accumulator62::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_62(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_62() -> f32 {
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
        total += (dep_touch_62(total as u64) % 997) as f32;
        total
    }
}

pub mod m63 {
    use super::*;

    pub struct Accumulator63<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator63<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.751_f32 + y.sin();
        let b = y * 5.402_f32 - x.cos();
        let mut acc = Accumulator63::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.685_f32 + y.sin();
        let b = y * 1.707_f32 - x.cos();
        let mut acc = Accumulator63::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.512_f32 + y.sin();
        let b = y * 2.611_f32 - x.cos();
        let mut acc = Accumulator63::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.878_f32 + y.sin();
        let b = y * 5.542_f32 - x.cos();
        let mut acc = Accumulator63::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.002_f32 + y.sin();
        let b = y * 7.22_f32 - x.cos();
        let mut acc = Accumulator63::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.018_f32 + y.sin();
        let b = y * 8.828_f32 - x.cos();
        let mut acc = Accumulator63::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.136_f32 + y.sin();
        let b = y * 0.626_f32 - x.cos();
        let mut acc = Accumulator63::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.69_f32 + y.sin();
        let b = y * 6.852_f32 - x.cos();
        let mut acc = Accumulator63::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.953_f32 + y.sin();
        let b = y * 1.779_f32 - x.cos();
        let mut acc = Accumulator63::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.882_f32 + y.sin();
        let b = y * 4.255_f32 - x.cos();
        let mut acc = Accumulator63::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.178_f32 + y.sin();
        let b = y * 2.961_f32 - x.cos();
        let mut acc = Accumulator63::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.926_f32 + y.sin();
        let b = y * 5.769_f32 - x.cos();
        let mut acc = Accumulator63::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.487_f32 + y.sin();
        let b = y * 7.695_f32 - x.cos();
        let mut acc = Accumulator63::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.066_f32 + y.sin();
        let b = y * 0.709_f32 - x.cos();
        let mut acc = Accumulator63::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.816_f32 + y.sin();
        let b = y * 3.405_f32 - x.cos();
        let mut acc = Accumulator63::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.504_f32 + y.sin();
        let b = y * 6.159_f32 - x.cos();
        let mut acc = Accumulator63::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.812_f32 + y.sin();
        let b = y * 3.075_f32 - x.cos();
        let mut acc = Accumulator63::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.043_f32 + y.sin();
        let b = y * 4.626_f32 - x.cos();
        let mut acc = Accumulator63::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.206_f32 + y.sin();
        let b = y * 9.315_f32 - x.cos();
        let mut acc = Accumulator63::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.809_f32 + y.sin();
        let b = y * 0.416_f32 - x.cos();
        let mut acc = Accumulator63::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_63(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_63() -> f32 {
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
        total += (dep_touch_63(total as u64) % 997) as f32;
        total
    }
}

pub mod m64 {
    use super::*;

    pub struct Accumulator64<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator64<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.551_f32 + y.sin();
        let b = y * 5.112_f32 - x.cos();
        let mut acc = Accumulator64::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.582_f32 + y.sin();
        let b = y * 3.474_f32 - x.cos();
        let mut acc = Accumulator64::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.097_f32 + y.sin();
        let b = y * 9.164_f32 - x.cos();
        let mut acc = Accumulator64::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.344_f32 + y.sin();
        let b = y * 9.665_f32 - x.cos();
        let mut acc = Accumulator64::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.858_f32 + y.sin();
        let b = y * 4.801_f32 - x.cos();
        let mut acc = Accumulator64::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.25_f32 + y.sin();
        let b = y * 3.334_f32 - x.cos();
        let mut acc = Accumulator64::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.464_f32 + y.sin();
        let b = y * 6.337_f32 - x.cos();
        let mut acc = Accumulator64::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.593_f32 + y.sin();
        let b = y * 0.593_f32 - x.cos();
        let mut acc = Accumulator64::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.809_f32 + y.sin();
        let b = y * 1.005_f32 - x.cos();
        let mut acc = Accumulator64::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.619_f32 + y.sin();
        let b = y * 5.811_f32 - x.cos();
        let mut acc = Accumulator64::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.693_f32 + y.sin();
        let b = y * 7.406_f32 - x.cos();
        let mut acc = Accumulator64::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.641_f32 + y.sin();
        let b = y * 6.898_f32 - x.cos();
        let mut acc = Accumulator64::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.967_f32 + y.sin();
        let b = y * 7.401_f32 - x.cos();
        let mut acc = Accumulator64::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.886_f32 + y.sin();
        let b = y * 4.366_f32 - x.cos();
        let mut acc = Accumulator64::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.787_f32 + y.sin();
        let b = y * 8.634_f32 - x.cos();
        let mut acc = Accumulator64::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.431_f32 + y.sin();
        let b = y * 5.917_f32 - x.cos();
        let mut acc = Accumulator64::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.37_f32 + y.sin();
        let b = y * 1.235_f32 - x.cos();
        let mut acc = Accumulator64::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.457_f32 + y.sin();
        let b = y * 8.405_f32 - x.cos();
        let mut acc = Accumulator64::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.041_f32 + y.sin();
        let b = y * 4.587_f32 - x.cos();
        let mut acc = Accumulator64::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.7_f32 + y.sin();
        let b = y * 5.735_f32 - x.cos();
        let mut acc = Accumulator64::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_64(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m64-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_64() -> f32 {
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
        total += (dep_touch_64(total as u64) % 997) as f32;
        total
    }
}

pub mod m65 {
    use super::*;

    pub struct Accumulator65<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator65<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.817_f32 + y.sin();
        let b = y * 0.592_f32 - x.cos();
        let mut acc = Accumulator65::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.782_f32 + y.sin();
        let b = y * 7.878_f32 - x.cos();
        let mut acc = Accumulator65::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.684_f32 + y.sin();
        let b = y * 3.458_f32 - x.cos();
        let mut acc = Accumulator65::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.081_f32 + y.sin();
        let b = y * 0.954_f32 - x.cos();
        let mut acc = Accumulator65::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.685_f32 + y.sin();
        let b = y * 7.86_f32 - x.cos();
        let mut acc = Accumulator65::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.93_f32 + y.sin();
        let b = y * 8.296_f32 - x.cos();
        let mut acc = Accumulator65::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.144_f32 + y.sin();
        let b = y * 4.893_f32 - x.cos();
        let mut acc = Accumulator65::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.42_f32 + y.sin();
        let b = y * 8.706_f32 - x.cos();
        let mut acc = Accumulator65::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.203_f32 + y.sin();
        let b = y * 6.027_f32 - x.cos();
        let mut acc = Accumulator65::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.529_f32 + y.sin();
        let b = y * 5.445_f32 - x.cos();
        let mut acc = Accumulator65::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.566_f32 + y.sin();
        let b = y * 8.376_f32 - x.cos();
        let mut acc = Accumulator65::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.419_f32 + y.sin();
        let b = y * 3.391_f32 - x.cos();
        let mut acc = Accumulator65::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.935_f32 + y.sin();
        let b = y * 8.818_f32 - x.cos();
        let mut acc = Accumulator65::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.578_f32 + y.sin();
        let b = y * 8.01_f32 - x.cos();
        let mut acc = Accumulator65::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.22_f32 + y.sin();
        let b = y * 6.246_f32 - x.cos();
        let mut acc = Accumulator65::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.165_f32 + y.sin();
        let b = y * 1.904_f32 - x.cos();
        let mut acc = Accumulator65::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.36_f32 + y.sin();
        let b = y * 1.579_f32 - x.cos();
        let mut acc = Accumulator65::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.297_f32 + y.sin();
        let b = y * 8.821_f32 - x.cos();
        let mut acc = Accumulator65::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.743_f32 + y.sin();
        let b = y * 9.513_f32 - x.cos();
        let mut acc = Accumulator65::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.997_f32 + y.sin();
        let b = y * 9.467_f32 - x.cos();
        let mut acc = Accumulator65::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_65(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_65() -> f32 {
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
        total += (dep_touch_65(total as u64) % 997) as f32;
        total
    }
}

pub mod m66 {
    use super::*;

    pub struct Accumulator66<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator66<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.849_f32 + y.sin();
        let b = y * 6.045_f32 - x.cos();
        let mut acc = Accumulator66::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.931_f32 + y.sin();
        let b = y * 0.189_f32 - x.cos();
        let mut acc = Accumulator66::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.117_f32 + y.sin();
        let b = y * 2.51_f32 - x.cos();
        let mut acc = Accumulator66::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.032_f32 + y.sin();
        let b = y * 2.558_f32 - x.cos();
        let mut acc = Accumulator66::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.727_f32 + y.sin();
        let b = y * 9.748_f32 - x.cos();
        let mut acc = Accumulator66::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.55_f32 + y.sin();
        let b = y * 5.136_f32 - x.cos();
        let mut acc = Accumulator66::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.423_f32 + y.sin();
        let b = y * 7.483_f32 - x.cos();
        let mut acc = Accumulator66::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.63_f32 + y.sin();
        let b = y * 0.844_f32 - x.cos();
        let mut acc = Accumulator66::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.345_f32 + y.sin();
        let b = y * 2.71_f32 - x.cos();
        let mut acc = Accumulator66::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.646_f32 + y.sin();
        let b = y * 6.579_f32 - x.cos();
        let mut acc = Accumulator66::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.826_f32 + y.sin();
        let b = y * 9.291_f32 - x.cos();
        let mut acc = Accumulator66::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.685_f32 + y.sin();
        let b = y * 6.515_f32 - x.cos();
        let mut acc = Accumulator66::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.664_f32 + y.sin();
        let b = y * 9.667_f32 - x.cos();
        let mut acc = Accumulator66::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.677_f32 + y.sin();
        let b = y * 8.803_f32 - x.cos();
        let mut acc = Accumulator66::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.563_f32 + y.sin();
        let b = y * 4.488_f32 - x.cos();
        let mut acc = Accumulator66::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.122_f32 + y.sin();
        let b = y * 1.592_f32 - x.cos();
        let mut acc = Accumulator66::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.448_f32 + y.sin();
        let b = y * 0.444_f32 - x.cos();
        let mut acc = Accumulator66::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.828_f32 + y.sin();
        let b = y * 0.169_f32 - x.cos();
        let mut acc = Accumulator66::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.516_f32 + y.sin();
        let b = y * 5.858_f32 - x.cos();
        let mut acc = Accumulator66::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.078_f32 + y.sin();
        let b = y * 9.878_f32 - x.cos();
        let mut acc = Accumulator66::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_66(seed: u64) -> u64 {
        let re = Regex::new(r"m66-(\d+)").unwrap();
        let hay = format!("m66-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_66() -> f32 {
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
        total += (dep_touch_66(total as u64) % 997) as f32;
        total
    }
}

pub mod m67 {
    use super::*;

    pub struct Accumulator67<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator67<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.424_f32 + y.sin();
        let b = y * 6.387_f32 - x.cos();
        let mut acc = Accumulator67::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.247_f32 + y.sin();
        let b = y * 2.469_f32 - x.cos();
        let mut acc = Accumulator67::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.353_f32 + y.sin();
        let b = y * 5.472_f32 - x.cos();
        let mut acc = Accumulator67::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.458_f32 + y.sin();
        let b = y * 7.988_f32 - x.cos();
        let mut acc = Accumulator67::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.366_f32 + y.sin();
        let b = y * 4.931_f32 - x.cos();
        let mut acc = Accumulator67::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.63_f32 + y.sin();
        let b = y * 8.948_f32 - x.cos();
        let mut acc = Accumulator67::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.686_f32 + y.sin();
        let b = y * 5.737_f32 - x.cos();
        let mut acc = Accumulator67::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.878_f32 + y.sin();
        let b = y * 3.503_f32 - x.cos();
        let mut acc = Accumulator67::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.068_f32 + y.sin();
        let b = y * 8.013_f32 - x.cos();
        let mut acc = Accumulator67::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.689_f32 + y.sin();
        let b = y * 7.336_f32 - x.cos();
        let mut acc = Accumulator67::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.778_f32 + y.sin();
        let b = y * 8.994_f32 - x.cos();
        let mut acc = Accumulator67::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.017_f32 + y.sin();
        let b = y * 3.853_f32 - x.cos();
        let mut acc = Accumulator67::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.529_f32 + y.sin();
        let b = y * 1.934_f32 - x.cos();
        let mut acc = Accumulator67::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.834_f32 + y.sin();
        let b = y * 4.752_f32 - x.cos();
        let mut acc = Accumulator67::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.245_f32 + y.sin();
        let b = y * 4.395_f32 - x.cos();
        let mut acc = Accumulator67::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.779_f32 + y.sin();
        let b = y * 3.646_f32 - x.cos();
        let mut acc = Accumulator67::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.878_f32 + y.sin();
        let b = y * 9.543_f32 - x.cos();
        let mut acc = Accumulator67::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.202_f32 + y.sin();
        let b = y * 9.173_f32 - x.cos();
        let mut acc = Accumulator67::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.512_f32 + y.sin();
        let b = y * 8.304_f32 - x.cos();
        let mut acc = Accumulator67::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.667_f32 + y.sin();
        let b = y * 8.846_f32 - x.cos();
        let mut acc = Accumulator67::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_67(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_67() -> f32 {
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
        total += (dep_touch_67(total as u64) % 997) as f32;
        total
    }
}

pub mod m68 {
    use super::*;

    pub struct Accumulator68<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator68<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.59_f32 + y.sin();
        let b = y * 7.519_f32 - x.cos();
        let mut acc = Accumulator68::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.581_f32 + y.sin();
        let b = y * 8.411_f32 - x.cos();
        let mut acc = Accumulator68::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.476_f32 + y.sin();
        let b = y * 2.799_f32 - x.cos();
        let mut acc = Accumulator68::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.13_f32 + y.sin();
        let b = y * 3.976_f32 - x.cos();
        let mut acc = Accumulator68::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.444_f32 + y.sin();
        let b = y * 1.287_f32 - x.cos();
        let mut acc = Accumulator68::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.659_f32 + y.sin();
        let b = y * 0.505_f32 - x.cos();
        let mut acc = Accumulator68::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.651_f32 + y.sin();
        let b = y * 3.566_f32 - x.cos();
        let mut acc = Accumulator68::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.702_f32 + y.sin();
        let b = y * 2.485_f32 - x.cos();
        let mut acc = Accumulator68::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.681_f32 + y.sin();
        let b = y * 1.045_f32 - x.cos();
        let mut acc = Accumulator68::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.745_f32 + y.sin();
        let b = y * 4.145_f32 - x.cos();
        let mut acc = Accumulator68::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.19_f32 + y.sin();
        let b = y * 3.108_f32 - x.cos();
        let mut acc = Accumulator68::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.829_f32 + y.sin();
        let b = y * 7.185_f32 - x.cos();
        let mut acc = Accumulator68::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.882_f32 + y.sin();
        let b = y * 7.691_f32 - x.cos();
        let mut acc = Accumulator68::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.348_f32 + y.sin();
        let b = y * 7.84_f32 - x.cos();
        let mut acc = Accumulator68::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.02_f32 + y.sin();
        let b = y * 4.998_f32 - x.cos();
        let mut acc = Accumulator68::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.527_f32 + y.sin();
        let b = y * 1.92_f32 - x.cos();
        let mut acc = Accumulator68::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.312_f32 + y.sin();
        let b = y * 5.208_f32 - x.cos();
        let mut acc = Accumulator68::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.187_f32 + y.sin();
        let b = y * 2.93_f32 - x.cos();
        let mut acc = Accumulator68::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.42_f32 + y.sin();
        let b = y * 0.746_f32 - x.cos();
        let mut acc = Accumulator68::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.755_f32 + y.sin();
        let b = y * 0.13_f32 - x.cos();
        let mut acc = Accumulator68::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_68(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(68u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_68() -> f32 {
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
        total += (dep_touch_68(total as u64) % 997) as f32;
        total
    }
}

pub mod m69 {
    use super::*;

    pub struct Accumulator69<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator69<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.645_f32 + y.sin();
        let b = y * 5.763_f32 - x.cos();
        let mut acc = Accumulator69::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.197_f32 + y.sin();
        let b = y * 7.242_f32 - x.cos();
        let mut acc = Accumulator69::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.827_f32 + y.sin();
        let b = y * 8.316_f32 - x.cos();
        let mut acc = Accumulator69::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.277_f32 + y.sin();
        let b = y * 8.427_f32 - x.cos();
        let mut acc = Accumulator69::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.006_f32 + y.sin();
        let b = y * 8.904_f32 - x.cos();
        let mut acc = Accumulator69::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.381_f32 + y.sin();
        let b = y * 8.146_f32 - x.cos();
        let mut acc = Accumulator69::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.833_f32 + y.sin();
        let b = y * 2.917_f32 - x.cos();
        let mut acc = Accumulator69::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.0_f32 + y.sin();
        let b = y * 6.997_f32 - x.cos();
        let mut acc = Accumulator69::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.883_f32 + y.sin();
        let b = y * 7.078_f32 - x.cos();
        let mut acc = Accumulator69::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.028_f32 + y.sin();
        let b = y * 5.089_f32 - x.cos();
        let mut acc = Accumulator69::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.188_f32 + y.sin();
        let b = y * 2.294_f32 - x.cos();
        let mut acc = Accumulator69::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.625_f32 + y.sin();
        let b = y * 5.676_f32 - x.cos();
        let mut acc = Accumulator69::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.625_f32 + y.sin();
        let b = y * 6.954_f32 - x.cos();
        let mut acc = Accumulator69::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.294_f32 + y.sin();
        let b = y * 9.448_f32 - x.cos();
        let mut acc = Accumulator69::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.134_f32 + y.sin();
        let b = y * 6.267_f32 - x.cos();
        let mut acc = Accumulator69::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.479_f32 + y.sin();
        let b = y * 5.033_f32 - x.cos();
        let mut acc = Accumulator69::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.884_f32 + y.sin();
        let b = y * 5.938_f32 - x.cos();
        let mut acc = Accumulator69::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.488_f32 + y.sin();
        let b = y * 8.208_f32 - x.cos();
        let mut acc = Accumulator69::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.841_f32 + y.sin();
        let b = y * 2.983_f32 - x.cos();
        let mut acc = Accumulator69::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.554_f32 + y.sin();
        let b = y * 5.301_f32 - x.cos();
        let mut acc = Accumulator69::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_69(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_69() -> f32 {
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
        total += (dep_touch_69(total as u64) % 997) as f32;
        total
    }
}

pub mod m70 {
    use super::*;

    pub struct Accumulator70<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator70<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.235_f32 + y.sin();
        let b = y * 7.876_f32 - x.cos();
        let mut acc = Accumulator70::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.861_f32 + y.sin();
        let b = y * 9.644_f32 - x.cos();
        let mut acc = Accumulator70::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.363_f32 + y.sin();
        let b = y * 0.399_f32 - x.cos();
        let mut acc = Accumulator70::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.378_f32 + y.sin();
        let b = y * 5.046_f32 - x.cos();
        let mut acc = Accumulator70::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.769_f32 + y.sin();
        let b = y * 9.454_f32 - x.cos();
        let mut acc = Accumulator70::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.2_f32 + y.sin();
        let b = y * 5.832_f32 - x.cos();
        let mut acc = Accumulator70::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.925_f32 + y.sin();
        let b = y * 9.454_f32 - x.cos();
        let mut acc = Accumulator70::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.318_f32 + y.sin();
        let b = y * 8.603_f32 - x.cos();
        let mut acc = Accumulator70::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.981_f32 + y.sin();
        let b = y * 9.521_f32 - x.cos();
        let mut acc = Accumulator70::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.547_f32 + y.sin();
        let b = y * 2.567_f32 - x.cos();
        let mut acc = Accumulator70::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.783_f32 + y.sin();
        let b = y * 1.411_f32 - x.cos();
        let mut acc = Accumulator70::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.598_f32 + y.sin();
        let b = y * 8.728_f32 - x.cos();
        let mut acc = Accumulator70::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.556_f32 + y.sin();
        let b = y * 3.448_f32 - x.cos();
        let mut acc = Accumulator70::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.196_f32 + y.sin();
        let b = y * 1.723_f32 - x.cos();
        let mut acc = Accumulator70::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.849_f32 + y.sin();
        let b = y * 8.895_f32 - x.cos();
        let mut acc = Accumulator70::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.398_f32 + y.sin();
        let b = y * 2.037_f32 - x.cos();
        let mut acc = Accumulator70::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.382_f32 + y.sin();
        let b = y * 7.679_f32 - x.cos();
        let mut acc = Accumulator70::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.591_f32 + y.sin();
        let b = y * 6.972_f32 - x.cos();
        let mut acc = Accumulator70::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.427_f32 + y.sin();
        let b = y * 9.393_f32 - x.cos();
        let mut acc = Accumulator70::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.181_f32 + y.sin();
        let b = y * 7.151_f32 - x.cos();
        let mut acc = Accumulator70::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_70(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_70() -> f32 {
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
        total += (dep_touch_70(total as u64) % 997) as f32;
        total
    }
}

pub mod m71 {
    use super::*;

    pub struct Accumulator71<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator71<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.568_f32 + y.sin();
        let b = y * 4.196_f32 - x.cos();
        let mut acc = Accumulator71::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.175_f32 + y.sin();
        let b = y * 6.276_f32 - x.cos();
        let mut acc = Accumulator71::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.566_f32 + y.sin();
        let b = y * 6.25_f32 - x.cos();
        let mut acc = Accumulator71::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.296_f32 + y.sin();
        let b = y * 9.816_f32 - x.cos();
        let mut acc = Accumulator71::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.132_f32 + y.sin();
        let b = y * 7.346_f32 - x.cos();
        let mut acc = Accumulator71::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.582_f32 + y.sin();
        let b = y * 8.91_f32 - x.cos();
        let mut acc = Accumulator71::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.56_f32 + y.sin();
        let b = y * 1.101_f32 - x.cos();
        let mut acc = Accumulator71::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.51_f32 + y.sin();
        let b = y * 3.666_f32 - x.cos();
        let mut acc = Accumulator71::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.863_f32 + y.sin();
        let b = y * 0.911_f32 - x.cos();
        let mut acc = Accumulator71::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.889_f32 + y.sin();
        let b = y * 4.767_f32 - x.cos();
        let mut acc = Accumulator71::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.43_f32 + y.sin();
        let b = y * 5.277_f32 - x.cos();
        let mut acc = Accumulator71::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.078_f32 + y.sin();
        let b = y * 2.151_f32 - x.cos();
        let mut acc = Accumulator71::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.532_f32 + y.sin();
        let b = y * 9.288_f32 - x.cos();
        let mut acc = Accumulator71::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.06_f32 + y.sin();
        let b = y * 9.364_f32 - x.cos();
        let mut acc = Accumulator71::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.38_f32 + y.sin();
        let b = y * 7.278_f32 - x.cos();
        let mut acc = Accumulator71::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.677_f32 + y.sin();
        let b = y * 8.884_f32 - x.cos();
        let mut acc = Accumulator71::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.412_f32 + y.sin();
        let b = y * 0.226_f32 - x.cos();
        let mut acc = Accumulator71::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.233_f32 + y.sin();
        let b = y * 5.306_f32 - x.cos();
        let mut acc = Accumulator71::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.189_f32 + y.sin();
        let b = y * 8.237_f32 - x.cos();
        let mut acc = Accumulator71::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.644_f32 + y.sin();
        let b = y * 0.524_f32 - x.cos();
        let mut acc = Accumulator71::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_71(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m71-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_71() -> f32 {
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
        total += (dep_touch_71(total as u64) % 997) as f32;
        total
    }
}

pub mod m72 {
    use super::*;

    pub struct Accumulator72<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator72<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.335_f32 + y.sin();
        let b = y * 0.796_f32 - x.cos();
        let mut acc = Accumulator72::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.417_f32 + y.sin();
        let b = y * 5.584_f32 - x.cos();
        let mut acc = Accumulator72::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.848_f32 + y.sin();
        let b = y * 6.367_f32 - x.cos();
        let mut acc = Accumulator72::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.172_f32 + y.sin();
        let b = y * 2.104_f32 - x.cos();
        let mut acc = Accumulator72::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.85_f32 + y.sin();
        let b = y * 1.119_f32 - x.cos();
        let mut acc = Accumulator72::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.7_f32 + y.sin();
        let b = y * 9.359_f32 - x.cos();
        let mut acc = Accumulator72::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.573_f32 + y.sin();
        let b = y * 5.838_f32 - x.cos();
        let mut acc = Accumulator72::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.817_f32 + y.sin();
        let b = y * 9.057_f32 - x.cos();
        let mut acc = Accumulator72::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.688_f32 + y.sin();
        let b = y * 7.148_f32 - x.cos();
        let mut acc = Accumulator72::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.756_f32 + y.sin();
        let b = y * 6.489_f32 - x.cos();
        let mut acc = Accumulator72::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.801_f32 + y.sin();
        let b = y * 7.125_f32 - x.cos();
        let mut acc = Accumulator72::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.882_f32 + y.sin();
        let b = y * 4.723_f32 - x.cos();
        let mut acc = Accumulator72::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.26_f32 + y.sin();
        let b = y * 4.98_f32 - x.cos();
        let mut acc = Accumulator72::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.715_f32 + y.sin();
        let b = y * 2.438_f32 - x.cos();
        let mut acc = Accumulator72::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.148_f32 + y.sin();
        let b = y * 5.648_f32 - x.cos();
        let mut acc = Accumulator72::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.173_f32 + y.sin();
        let b = y * 6.312_f32 - x.cos();
        let mut acc = Accumulator72::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.448_f32 + y.sin();
        let b = y * 2.478_f32 - x.cos();
        let mut acc = Accumulator72::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.967_f32 + y.sin();
        let b = y * 0.473_f32 - x.cos();
        let mut acc = Accumulator72::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.039_f32 + y.sin();
        let b = y * 9.323_f32 - x.cos();
        let mut acc = Accumulator72::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.698_f32 + y.sin();
        let b = y * 0.533_f32 - x.cos();
        let mut acc = Accumulator72::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_72(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_72() -> f32 {
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
        total += (dep_touch_72(total as u64) % 997) as f32;
        total
    }
}

pub mod m73 {
    use super::*;

    pub struct Accumulator73<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator73<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.359_f32 + y.sin();
        let b = y * 9.109_f32 - x.cos();
        let mut acc = Accumulator73::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.505_f32 + y.sin();
        let b = y * 4.686_f32 - x.cos();
        let mut acc = Accumulator73::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.52_f32 + y.sin();
        let b = y * 1.117_f32 - x.cos();
        let mut acc = Accumulator73::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.046_f32 + y.sin();
        let b = y * 1.504_f32 - x.cos();
        let mut acc = Accumulator73::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.696_f32 + y.sin();
        let b = y * 5.119_f32 - x.cos();
        let mut acc = Accumulator73::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.096_f32 + y.sin();
        let b = y * 9.477_f32 - x.cos();
        let mut acc = Accumulator73::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.08_f32 + y.sin();
        let b = y * 0.122_f32 - x.cos();
        let mut acc = Accumulator73::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.548_f32 + y.sin();
        let b = y * 8.149_f32 - x.cos();
        let mut acc = Accumulator73::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.604_f32 + y.sin();
        let b = y * 6.106_f32 - x.cos();
        let mut acc = Accumulator73::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.861_f32 + y.sin();
        let b = y * 0.632_f32 - x.cos();
        let mut acc = Accumulator73::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.128_f32 + y.sin();
        let b = y * 4.579_f32 - x.cos();
        let mut acc = Accumulator73::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.175_f32 + y.sin();
        let b = y * 7.401_f32 - x.cos();
        let mut acc = Accumulator73::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.936_f32 + y.sin();
        let b = y * 5.069_f32 - x.cos();
        let mut acc = Accumulator73::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.146_f32 + y.sin();
        let b = y * 7.04_f32 - x.cos();
        let mut acc = Accumulator73::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.13_f32 + y.sin();
        let b = y * 4.305_f32 - x.cos();
        let mut acc = Accumulator73::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.104_f32 + y.sin();
        let b = y * 0.946_f32 - x.cos();
        let mut acc = Accumulator73::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.555_f32 + y.sin();
        let b = y * 1.022_f32 - x.cos();
        let mut acc = Accumulator73::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.442_f32 + y.sin();
        let b = y * 8.737_f32 - x.cos();
        let mut acc = Accumulator73::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.98_f32 + y.sin();
        let b = y * 2.785_f32 - x.cos();
        let mut acc = Accumulator73::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.572_f32 + y.sin();
        let b = y * 1.549_f32 - x.cos();
        let mut acc = Accumulator73::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_73(seed: u64) -> u64 {
        let re = Regex::new(r"m73-(\d+)").unwrap();
        let hay = format!("m73-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_73() -> f32 {
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
        total += (dep_touch_73(total as u64) % 997) as f32;
        total
    }
}

pub mod m74 {
    use super::*;

    pub struct Accumulator74<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator74<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.747_f32 + y.sin();
        let b = y * 3.382_f32 - x.cos();
        let mut acc = Accumulator74::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.168_f32 + y.sin();
        let b = y * 0.835_f32 - x.cos();
        let mut acc = Accumulator74::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.792_f32 + y.sin();
        let b = y * 7.62_f32 - x.cos();
        let mut acc = Accumulator74::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.197_f32 + y.sin();
        let b = y * 4.565_f32 - x.cos();
        let mut acc = Accumulator74::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.153_f32 + y.sin();
        let b = y * 5.73_f32 - x.cos();
        let mut acc = Accumulator74::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.081_f32 + y.sin();
        let b = y * 7.28_f32 - x.cos();
        let mut acc = Accumulator74::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.046_f32 + y.sin();
        let b = y * 8.304_f32 - x.cos();
        let mut acc = Accumulator74::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.246_f32 + y.sin();
        let b = y * 6.667_f32 - x.cos();
        let mut acc = Accumulator74::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.43_f32 + y.sin();
        let b = y * 4.322_f32 - x.cos();
        let mut acc = Accumulator74::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.862_f32 + y.sin();
        let b = y * 9.339_f32 - x.cos();
        let mut acc = Accumulator74::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.604_f32 + y.sin();
        let b = y * 1.415_f32 - x.cos();
        let mut acc = Accumulator74::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.394_f32 + y.sin();
        let b = y * 0.378_f32 - x.cos();
        let mut acc = Accumulator74::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.028_f32 + y.sin();
        let b = y * 4.44_f32 - x.cos();
        let mut acc = Accumulator74::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.528_f32 + y.sin();
        let b = y * 4.738_f32 - x.cos();
        let mut acc = Accumulator74::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.301_f32 + y.sin();
        let b = y * 9.844_f32 - x.cos();
        let mut acc = Accumulator74::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.294_f32 + y.sin();
        let b = y * 7.976_f32 - x.cos();
        let mut acc = Accumulator74::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.142_f32 + y.sin();
        let b = y * 0.305_f32 - x.cos();
        let mut acc = Accumulator74::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.431_f32 + y.sin();
        let b = y * 3.597_f32 - x.cos();
        let mut acc = Accumulator74::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.117_f32 + y.sin();
        let b = y * 7.656_f32 - x.cos();
        let mut acc = Accumulator74::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.82_f32 + y.sin();
        let b = y * 7.887_f32 - x.cos();
        let mut acc = Accumulator74::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_74(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_74() -> f32 {
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
        total += (dep_touch_74(total as u64) % 997) as f32;
        total
    }
}

pub mod m75 {
    use super::*;

    pub struct Accumulator75<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator75<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.681_f32 + y.sin();
        let b = y * 0.447_f32 - x.cos();
        let mut acc = Accumulator75::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.265_f32 + y.sin();
        let b = y * 3.402_f32 - x.cos();
        let mut acc = Accumulator75::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.365_f32 + y.sin();
        let b = y * 9.563_f32 - x.cos();
        let mut acc = Accumulator75::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.173_f32 + y.sin();
        let b = y * 0.623_f32 - x.cos();
        let mut acc = Accumulator75::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.376_f32 + y.sin();
        let b = y * 9.292_f32 - x.cos();
        let mut acc = Accumulator75::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.228_f32 + y.sin();
        let b = y * 5.185_f32 - x.cos();
        let mut acc = Accumulator75::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.978_f32 + y.sin();
        let b = y * 2.181_f32 - x.cos();
        let mut acc = Accumulator75::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.5_f32 + y.sin();
        let b = y * 8.78_f32 - x.cos();
        let mut acc = Accumulator75::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.65_f32 + y.sin();
        let b = y * 7.115_f32 - x.cos();
        let mut acc = Accumulator75::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.084_f32 + y.sin();
        let b = y * 6.12_f32 - x.cos();
        let mut acc = Accumulator75::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.868_f32 + y.sin();
        let b = y * 7.411_f32 - x.cos();
        let mut acc = Accumulator75::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.931_f32 + y.sin();
        let b = y * 3.449_f32 - x.cos();
        let mut acc = Accumulator75::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.999_f32 + y.sin();
        let b = y * 8.589_f32 - x.cos();
        let mut acc = Accumulator75::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.643_f32 + y.sin();
        let b = y * 9.55_f32 - x.cos();
        let mut acc = Accumulator75::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.357_f32 + y.sin();
        let b = y * 1.492_f32 - x.cos();
        let mut acc = Accumulator75::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.959_f32 + y.sin();
        let b = y * 6.91_f32 - x.cos();
        let mut acc = Accumulator75::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.864_f32 + y.sin();
        let b = y * 7.25_f32 - x.cos();
        let mut acc = Accumulator75::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.242_f32 + y.sin();
        let b = y * 3.643_f32 - x.cos();
        let mut acc = Accumulator75::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.562_f32 + y.sin();
        let b = y * 7.18_f32 - x.cos();
        let mut acc = Accumulator75::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.602_f32 + y.sin();
        let b = y * 6.838_f32 - x.cos();
        let mut acc = Accumulator75::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_75(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(75u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_75() -> f32 {
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
        total += (dep_touch_75(total as u64) % 997) as f32;
        total
    }
}

pub mod m76 {
    use super::*;

    pub struct Accumulator76<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator76<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.123_f32 + y.sin();
        let b = y * 4.51_f32 - x.cos();
        let mut acc = Accumulator76::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.598_f32 + y.sin();
        let b = y * 3.969_f32 - x.cos();
        let mut acc = Accumulator76::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.857_f32 + y.sin();
        let b = y * 4.46_f32 - x.cos();
        let mut acc = Accumulator76::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.705_f32 + y.sin();
        let b = y * 8.541_f32 - x.cos();
        let mut acc = Accumulator76::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.455_f32 + y.sin();
        let b = y * 8.256_f32 - x.cos();
        let mut acc = Accumulator76::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.902_f32 + y.sin();
        let b = y * 7.788_f32 - x.cos();
        let mut acc = Accumulator76::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.479_f32 + y.sin();
        let b = y * 7.968_f32 - x.cos();
        let mut acc = Accumulator76::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.388_f32 + y.sin();
        let b = y * 6.224_f32 - x.cos();
        let mut acc = Accumulator76::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.812_f32 + y.sin();
        let b = y * 0.977_f32 - x.cos();
        let mut acc = Accumulator76::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.584_f32 + y.sin();
        let b = y * 3.157_f32 - x.cos();
        let mut acc = Accumulator76::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.87_f32 + y.sin();
        let b = y * 1.508_f32 - x.cos();
        let mut acc = Accumulator76::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.301_f32 + y.sin();
        let b = y * 0.661_f32 - x.cos();
        let mut acc = Accumulator76::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.492_f32 + y.sin();
        let b = y * 5.682_f32 - x.cos();
        let mut acc = Accumulator76::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.708_f32 + y.sin();
        let b = y * 3.0_f32 - x.cos();
        let mut acc = Accumulator76::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.455_f32 + y.sin();
        let b = y * 1.375_f32 - x.cos();
        let mut acc = Accumulator76::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.664_f32 + y.sin();
        let b = y * 7.182_f32 - x.cos();
        let mut acc = Accumulator76::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.72_f32 + y.sin();
        let b = y * 7.324_f32 - x.cos();
        let mut acc = Accumulator76::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.689_f32 + y.sin();
        let b = y * 5.226_f32 - x.cos();
        let mut acc = Accumulator76::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.65_f32 + y.sin();
        let b = y * 9.011_f32 - x.cos();
        let mut acc = Accumulator76::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.896_f32 + y.sin();
        let b = y * 3.832_f32 - x.cos();
        let mut acc = Accumulator76::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_76(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_76() -> f32 {
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
        total += (dep_touch_76(total as u64) % 997) as f32;
        total
    }
}

pub mod m77 {
    use super::*;

    pub struct Accumulator77<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator77<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.295_f32 + y.sin();
        let b = y * 2.001_f32 - x.cos();
        let mut acc = Accumulator77::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.574_f32 + y.sin();
        let b = y * 8.461_f32 - x.cos();
        let mut acc = Accumulator77::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.386_f32 + y.sin();
        let b = y * 4.724_f32 - x.cos();
        let mut acc = Accumulator77::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.596_f32 + y.sin();
        let b = y * 6.604_f32 - x.cos();
        let mut acc = Accumulator77::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.401_f32 + y.sin();
        let b = y * 0.388_f32 - x.cos();
        let mut acc = Accumulator77::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.877_f32 + y.sin();
        let b = y * 1.177_f32 - x.cos();
        let mut acc = Accumulator77::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.073_f32 + y.sin();
        let b = y * 1.019_f32 - x.cos();
        let mut acc = Accumulator77::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.853_f32 + y.sin();
        let b = y * 9.146_f32 - x.cos();
        let mut acc = Accumulator77::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.273_f32 + y.sin();
        let b = y * 0.695_f32 - x.cos();
        let mut acc = Accumulator77::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.765_f32 + y.sin();
        let b = y * 3.639_f32 - x.cos();
        let mut acc = Accumulator77::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.443_f32 + y.sin();
        let b = y * 3.418_f32 - x.cos();
        let mut acc = Accumulator77::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.801_f32 + y.sin();
        let b = y * 2.28_f32 - x.cos();
        let mut acc = Accumulator77::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.215_f32 + y.sin();
        let b = y * 6.089_f32 - x.cos();
        let mut acc = Accumulator77::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.178_f32 + y.sin();
        let b = y * 9.348_f32 - x.cos();
        let mut acc = Accumulator77::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.649_f32 + y.sin();
        let b = y * 5.211_f32 - x.cos();
        let mut acc = Accumulator77::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.574_f32 + y.sin();
        let b = y * 2.181_f32 - x.cos();
        let mut acc = Accumulator77::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.778_f32 + y.sin();
        let b = y * 4.691_f32 - x.cos();
        let mut acc = Accumulator77::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.794_f32 + y.sin();
        let b = y * 1.391_f32 - x.cos();
        let mut acc = Accumulator77::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.284_f32 + y.sin();
        let b = y * 1.06_f32 - x.cos();
        let mut acc = Accumulator77::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.597_f32 + y.sin();
        let b = y * 3.64_f32 - x.cos();
        let mut acc = Accumulator77::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_77(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_77() -> f32 {
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
        total += (dep_touch_77(total as u64) % 997) as f32;
        total
    }
}

pub mod m78 {
    use super::*;

    pub struct Accumulator78<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator78<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.994_f32 + y.sin();
        let b = y * 4.755_f32 - x.cos();
        let mut acc = Accumulator78::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.864_f32 + y.sin();
        let b = y * 0.207_f32 - x.cos();
        let mut acc = Accumulator78::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.053_f32 + y.sin();
        let b = y * 8.946_f32 - x.cos();
        let mut acc = Accumulator78::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.709_f32 + y.sin();
        let b = y * 7.356_f32 - x.cos();
        let mut acc = Accumulator78::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.455_f32 + y.sin();
        let b = y * 7.671_f32 - x.cos();
        let mut acc = Accumulator78::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.028_f32 + y.sin();
        let b = y * 6.481_f32 - x.cos();
        let mut acc = Accumulator78::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.896_f32 + y.sin();
        let b = y * 8.792_f32 - x.cos();
        let mut acc = Accumulator78::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.356_f32 + y.sin();
        let b = y * 8.502_f32 - x.cos();
        let mut acc = Accumulator78::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.533_f32 + y.sin();
        let b = y * 6.478_f32 - x.cos();
        let mut acc = Accumulator78::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.324_f32 + y.sin();
        let b = y * 5.96_f32 - x.cos();
        let mut acc = Accumulator78::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.903_f32 + y.sin();
        let b = y * 2.226_f32 - x.cos();
        let mut acc = Accumulator78::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.895_f32 + y.sin();
        let b = y * 9.659_f32 - x.cos();
        let mut acc = Accumulator78::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.084_f32 + y.sin();
        let b = y * 3.177_f32 - x.cos();
        let mut acc = Accumulator78::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.307_f32 + y.sin();
        let b = y * 4.04_f32 - x.cos();
        let mut acc = Accumulator78::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.719_f32 + y.sin();
        let b = y * 3.627_f32 - x.cos();
        let mut acc = Accumulator78::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.384_f32 + y.sin();
        let b = y * 4.775_f32 - x.cos();
        let mut acc = Accumulator78::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.462_f32 + y.sin();
        let b = y * 7.376_f32 - x.cos();
        let mut acc = Accumulator78::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.82_f32 + y.sin();
        let b = y * 6.188_f32 - x.cos();
        let mut acc = Accumulator78::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.986_f32 + y.sin();
        let b = y * 4.71_f32 - x.cos();
        let mut acc = Accumulator78::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.896_f32 + y.sin();
        let b = y * 4.574_f32 - x.cos();
        let mut acc = Accumulator78::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_78(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m78-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_78() -> f32 {
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
        total += (dep_touch_78(total as u64) % 997) as f32;
        total
    }
}

pub mod m79 {
    use super::*;

    pub struct Accumulator79<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator79<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.654_f32 + y.sin();
        let b = y * 7.507_f32 - x.cos();
        let mut acc = Accumulator79::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.088_f32 + y.sin();
        let b = y * 3.463_f32 - x.cos();
        let mut acc = Accumulator79::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.048_f32 + y.sin();
        let b = y * 5.672_f32 - x.cos();
        let mut acc = Accumulator79::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.437_f32 + y.sin();
        let b = y * 8.302_f32 - x.cos();
        let mut acc = Accumulator79::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.054_f32 + y.sin();
        let b = y * 2.627_f32 - x.cos();
        let mut acc = Accumulator79::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.756_f32 + y.sin();
        let b = y * 5.469_f32 - x.cos();
        let mut acc = Accumulator79::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.449_f32 + y.sin();
        let b = y * 2.456_f32 - x.cos();
        let mut acc = Accumulator79::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.784_f32 + y.sin();
        let b = y * 4.177_f32 - x.cos();
        let mut acc = Accumulator79::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.885_f32 + y.sin();
        let b = y * 1.426_f32 - x.cos();
        let mut acc = Accumulator79::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.788_f32 + y.sin();
        let b = y * 0.658_f32 - x.cos();
        let mut acc = Accumulator79::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.244_f32 + y.sin();
        let b = y * 0.886_f32 - x.cos();
        let mut acc = Accumulator79::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.842_f32 + y.sin();
        let b = y * 2.581_f32 - x.cos();
        let mut acc = Accumulator79::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.585_f32 + y.sin();
        let b = y * 8.53_f32 - x.cos();
        let mut acc = Accumulator79::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.736_f32 + y.sin();
        let b = y * 9.332_f32 - x.cos();
        let mut acc = Accumulator79::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.029_f32 + y.sin();
        let b = y * 4.369_f32 - x.cos();
        let mut acc = Accumulator79::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.581_f32 + y.sin();
        let b = y * 8.959_f32 - x.cos();
        let mut acc = Accumulator79::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.932_f32 + y.sin();
        let b = y * 9.576_f32 - x.cos();
        let mut acc = Accumulator79::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.217_f32 + y.sin();
        let b = y * 5.288_f32 - x.cos();
        let mut acc = Accumulator79::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.009_f32 + y.sin();
        let b = y * 6.503_f32 - x.cos();
        let mut acc = Accumulator79::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.814_f32 + y.sin();
        let b = y * 4.473_f32 - x.cos();
        let mut acc = Accumulator79::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_79(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_79() -> f32 {
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
        total += (dep_touch_79(total as u64) % 997) as f32;
        total
    }
}

pub mod m80 {
    use super::*;

    pub struct Accumulator80<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator80<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.55_f32 + y.sin();
        let b = y * 5.846_f32 - x.cos();
        let mut acc = Accumulator80::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.908_f32 + y.sin();
        let b = y * 9.531_f32 - x.cos();
        let mut acc = Accumulator80::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.373_f32 + y.sin();
        let b = y * 5.903_f32 - x.cos();
        let mut acc = Accumulator80::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.689_f32 + y.sin();
        let b = y * 1.879_f32 - x.cos();
        let mut acc = Accumulator80::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.475_f32 + y.sin();
        let b = y * 6.406_f32 - x.cos();
        let mut acc = Accumulator80::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.689_f32 + y.sin();
        let b = y * 5.234_f32 - x.cos();
        let mut acc = Accumulator80::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.156_f32 + y.sin();
        let b = y * 6.97_f32 - x.cos();
        let mut acc = Accumulator80::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.218_f32 + y.sin();
        let b = y * 4.722_f32 - x.cos();
        let mut acc = Accumulator80::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.315_f32 + y.sin();
        let b = y * 9.117_f32 - x.cos();
        let mut acc = Accumulator80::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.758_f32 + y.sin();
        let b = y * 6.744_f32 - x.cos();
        let mut acc = Accumulator80::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.469_f32 + y.sin();
        let b = y * 8.82_f32 - x.cos();
        let mut acc = Accumulator80::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.493_f32 + y.sin();
        let b = y * 7.8_f32 - x.cos();
        let mut acc = Accumulator80::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.686_f32 + y.sin();
        let b = y * 0.972_f32 - x.cos();
        let mut acc = Accumulator80::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.417_f32 + y.sin();
        let b = y * 4.03_f32 - x.cos();
        let mut acc = Accumulator80::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.921_f32 + y.sin();
        let b = y * 2.378_f32 - x.cos();
        let mut acc = Accumulator80::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.041_f32 + y.sin();
        let b = y * 3.825_f32 - x.cos();
        let mut acc = Accumulator80::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.615_f32 + y.sin();
        let b = y * 1.35_f32 - x.cos();
        let mut acc = Accumulator80::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.37_f32 + y.sin();
        let b = y * 8.713_f32 - x.cos();
        let mut acc = Accumulator80::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.45_f32 + y.sin();
        let b = y * 1.433_f32 - x.cos();
        let mut acc = Accumulator80::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.137_f32 + y.sin();
        let b = y * 6.434_f32 - x.cos();
        let mut acc = Accumulator80::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_80(seed: u64) -> u64 {
        let re = Regex::new(r"m80-(\d+)").unwrap();
        let hay = format!("m80-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_80() -> f32 {
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
        total += (dep_touch_80(total as u64) % 997) as f32;
        total
    }
}

pub mod m81 {
    use super::*;

    pub struct Accumulator81<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator81<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.807_f32 + y.sin();
        let b = y * 9.335_f32 - x.cos();
        let mut acc = Accumulator81::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.421_f32 + y.sin();
        let b = y * 3.482_f32 - x.cos();
        let mut acc = Accumulator81::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.182_f32 + y.sin();
        let b = y * 8.051_f32 - x.cos();
        let mut acc = Accumulator81::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.273_f32 + y.sin();
        let b = y * 6.724_f32 - x.cos();
        let mut acc = Accumulator81::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.961_f32 + y.sin();
        let b = y * 1.248_f32 - x.cos();
        let mut acc = Accumulator81::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.614_f32 + y.sin();
        let b = y * 8.288_f32 - x.cos();
        let mut acc = Accumulator81::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.099_f32 + y.sin();
        let b = y * 2.536_f32 - x.cos();
        let mut acc = Accumulator81::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.57_f32 + y.sin();
        let b = y * 2.024_f32 - x.cos();
        let mut acc = Accumulator81::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.457_f32 + y.sin();
        let b = y * 1.701_f32 - x.cos();
        let mut acc = Accumulator81::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.991_f32 + y.sin();
        let b = y * 8.427_f32 - x.cos();
        let mut acc = Accumulator81::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.547_f32 + y.sin();
        let b = y * 8.891_f32 - x.cos();
        let mut acc = Accumulator81::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.342_f32 + y.sin();
        let b = y * 7.211_f32 - x.cos();
        let mut acc = Accumulator81::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.385_f32 + y.sin();
        let b = y * 7.924_f32 - x.cos();
        let mut acc = Accumulator81::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.941_f32 + y.sin();
        let b = y * 2.539_f32 - x.cos();
        let mut acc = Accumulator81::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.354_f32 + y.sin();
        let b = y * 2.698_f32 - x.cos();
        let mut acc = Accumulator81::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.796_f32 + y.sin();
        let b = y * 2.416_f32 - x.cos();
        let mut acc = Accumulator81::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.219_f32 + y.sin();
        let b = y * 9.259_f32 - x.cos();
        let mut acc = Accumulator81::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.556_f32 + y.sin();
        let b = y * 3.386_f32 - x.cos();
        let mut acc = Accumulator81::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.52_f32 + y.sin();
        let b = y * 2.959_f32 - x.cos();
        let mut acc = Accumulator81::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.793_f32 + y.sin();
        let b = y * 2.633_f32 - x.cos();
        let mut acc = Accumulator81::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_81(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_81() -> f32 {
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
        total += (dep_touch_81(total as u64) % 997) as f32;
        total
    }
}

pub mod m82 {
    use super::*;

    pub struct Accumulator82<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator82<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.187_f32 + y.sin();
        let b = y * 2.687_f32 - x.cos();
        let mut acc = Accumulator82::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.996_f32 + y.sin();
        let b = y * 6.216_f32 - x.cos();
        let mut acc = Accumulator82::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.641_f32 + y.sin();
        let b = y * 1.391_f32 - x.cos();
        let mut acc = Accumulator82::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.321_f32 + y.sin();
        let b = y * 5.017_f32 - x.cos();
        let mut acc = Accumulator82::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.946_f32 + y.sin();
        let b = y * 9.204_f32 - x.cos();
        let mut acc = Accumulator82::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.991_f32 + y.sin();
        let b = y * 7.462_f32 - x.cos();
        let mut acc = Accumulator82::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.244_f32 + y.sin();
        let b = y * 2.992_f32 - x.cos();
        let mut acc = Accumulator82::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.348_f32 + y.sin();
        let b = y * 7.912_f32 - x.cos();
        let mut acc = Accumulator82::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.023_f32 + y.sin();
        let b = y * 5.845_f32 - x.cos();
        let mut acc = Accumulator82::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.88_f32 + y.sin();
        let b = y * 4.531_f32 - x.cos();
        let mut acc = Accumulator82::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.854_f32 + y.sin();
        let b = y * 2.366_f32 - x.cos();
        let mut acc = Accumulator82::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.184_f32 + y.sin();
        let b = y * 4.725_f32 - x.cos();
        let mut acc = Accumulator82::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.454_f32 + y.sin();
        let b = y * 1.989_f32 - x.cos();
        let mut acc = Accumulator82::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.215_f32 + y.sin();
        let b = y * 8.331_f32 - x.cos();
        let mut acc = Accumulator82::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.461_f32 + y.sin();
        let b = y * 2.634_f32 - x.cos();
        let mut acc = Accumulator82::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.214_f32 + y.sin();
        let b = y * 3.352_f32 - x.cos();
        let mut acc = Accumulator82::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.403_f32 + y.sin();
        let b = y * 7.199_f32 - x.cos();
        let mut acc = Accumulator82::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.125_f32 + y.sin();
        let b = y * 2.172_f32 - x.cos();
        let mut acc = Accumulator82::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.874_f32 + y.sin();
        let b = y * 6.124_f32 - x.cos();
        let mut acc = Accumulator82::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.105_f32 + y.sin();
        let b = y * 3.149_f32 - x.cos();
        let mut acc = Accumulator82::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_82(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(82u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_82() -> f32 {
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
        total += (dep_touch_82(total as u64) % 997) as f32;
        total
    }
}

pub mod m83 {
    use super::*;

    pub struct Accumulator83<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator83<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.906_f32 + y.sin();
        let b = y * 1.386_f32 - x.cos();
        let mut acc = Accumulator83::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.318_f32 + y.sin();
        let b = y * 7.659_f32 - x.cos();
        let mut acc = Accumulator83::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.12_f32 + y.sin();
        let b = y * 9.495_f32 - x.cos();
        let mut acc = Accumulator83::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.986_f32 + y.sin();
        let b = y * 5.225_f32 - x.cos();
        let mut acc = Accumulator83::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.645_f32 + y.sin();
        let b = y * 6.07_f32 - x.cos();
        let mut acc = Accumulator83::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.976_f32 + y.sin();
        let b = y * 3.285_f32 - x.cos();
        let mut acc = Accumulator83::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.582_f32 + y.sin();
        let b = y * 9.23_f32 - x.cos();
        let mut acc = Accumulator83::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.625_f32 + y.sin();
        let b = y * 2.038_f32 - x.cos();
        let mut acc = Accumulator83::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.793_f32 + y.sin();
        let b = y * 7.14_f32 - x.cos();
        let mut acc = Accumulator83::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.821_f32 + y.sin();
        let b = y * 4.769_f32 - x.cos();
        let mut acc = Accumulator83::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.699_f32 + y.sin();
        let b = y * 1.141_f32 - x.cos();
        let mut acc = Accumulator83::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.213_f32 + y.sin();
        let b = y * 4.386_f32 - x.cos();
        let mut acc = Accumulator83::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.575_f32 + y.sin();
        let b = y * 6.652_f32 - x.cos();
        let mut acc = Accumulator83::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.506_f32 + y.sin();
        let b = y * 8.452_f32 - x.cos();
        let mut acc = Accumulator83::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.2_f32 + y.sin();
        let b = y * 7.011_f32 - x.cos();
        let mut acc = Accumulator83::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.479_f32 + y.sin();
        let b = y * 9.19_f32 - x.cos();
        let mut acc = Accumulator83::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.164_f32 + y.sin();
        let b = y * 1.128_f32 - x.cos();
        let mut acc = Accumulator83::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.961_f32 + y.sin();
        let b = y * 9.14_f32 - x.cos();
        let mut acc = Accumulator83::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.466_f32 + y.sin();
        let b = y * 9.715_f32 - x.cos();
        let mut acc = Accumulator83::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.364_f32 + y.sin();
        let b = y * 4.666_f32 - x.cos();
        let mut acc = Accumulator83::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_83(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_83() -> f32 {
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
        total += (dep_touch_83(total as u64) % 997) as f32;
        total
    }
}

pub mod m84 {
    use super::*;

    pub struct Accumulator84<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator84<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.444_f32 + y.sin();
        let b = y * 6.545_f32 - x.cos();
        let mut acc = Accumulator84::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.573_f32 + y.sin();
        let b = y * 3.44_f32 - x.cos();
        let mut acc = Accumulator84::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.94_f32 + y.sin();
        let b = y * 3.96_f32 - x.cos();
        let mut acc = Accumulator84::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.116_f32 + y.sin();
        let b = y * 7.313_f32 - x.cos();
        let mut acc = Accumulator84::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.923_f32 + y.sin();
        let b = y * 7.626_f32 - x.cos();
        let mut acc = Accumulator84::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.294_f32 + y.sin();
        let b = y * 6.962_f32 - x.cos();
        let mut acc = Accumulator84::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.707_f32 + y.sin();
        let b = y * 5.312_f32 - x.cos();
        let mut acc = Accumulator84::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.757_f32 + y.sin();
        let b = y * 1.802_f32 - x.cos();
        let mut acc = Accumulator84::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.037_f32 + y.sin();
        let b = y * 3.652_f32 - x.cos();
        let mut acc = Accumulator84::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.589_f32 + y.sin();
        let b = y * 5.509_f32 - x.cos();
        let mut acc = Accumulator84::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.644_f32 + y.sin();
        let b = y * 7.358_f32 - x.cos();
        let mut acc = Accumulator84::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.962_f32 + y.sin();
        let b = y * 4.733_f32 - x.cos();
        let mut acc = Accumulator84::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.02_f32 + y.sin();
        let b = y * 2.929_f32 - x.cos();
        let mut acc = Accumulator84::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.953_f32 + y.sin();
        let b = y * 2.078_f32 - x.cos();
        let mut acc = Accumulator84::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.44_f32 + y.sin();
        let b = y * 9.072_f32 - x.cos();
        let mut acc = Accumulator84::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.149_f32 + y.sin();
        let b = y * 8.071_f32 - x.cos();
        let mut acc = Accumulator84::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.961_f32 + y.sin();
        let b = y * 0.371_f32 - x.cos();
        let mut acc = Accumulator84::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.698_f32 + y.sin();
        let b = y * 9.808_f32 - x.cos();
        let mut acc = Accumulator84::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.667_f32 + y.sin();
        let b = y * 6.131_f32 - x.cos();
        let mut acc = Accumulator84::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.794_f32 + y.sin();
        let b = y * 7.398_f32 - x.cos();
        let mut acc = Accumulator84::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_84(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_84() -> f32 {
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
        total += (dep_touch_84(total as u64) % 997) as f32;
        total
    }
}

pub mod m85 {
    use super::*;

    pub struct Accumulator85<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator85<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.79_f32 + y.sin();
        let b = y * 6.565_f32 - x.cos();
        let mut acc = Accumulator85::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.152_f32 + y.sin();
        let b = y * 8.558_f32 - x.cos();
        let mut acc = Accumulator85::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.469_f32 + y.sin();
        let b = y * 7.817_f32 - x.cos();
        let mut acc = Accumulator85::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.044_f32 + y.sin();
        let b = y * 8.914_f32 - x.cos();
        let mut acc = Accumulator85::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.334_f32 + y.sin();
        let b = y * 3.214_f32 - x.cos();
        let mut acc = Accumulator85::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.406_f32 + y.sin();
        let b = y * 3.055_f32 - x.cos();
        let mut acc = Accumulator85::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.558_f32 + y.sin();
        let b = y * 3.807_f32 - x.cos();
        let mut acc = Accumulator85::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.339_f32 + y.sin();
        let b = y * 0.223_f32 - x.cos();
        let mut acc = Accumulator85::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.124_f32 + y.sin();
        let b = y * 6.331_f32 - x.cos();
        let mut acc = Accumulator85::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.505_f32 + y.sin();
        let b = y * 2.495_f32 - x.cos();
        let mut acc = Accumulator85::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.981_f32 + y.sin();
        let b = y * 9.808_f32 - x.cos();
        let mut acc = Accumulator85::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.78_f32 + y.sin();
        let b = y * 1.575_f32 - x.cos();
        let mut acc = Accumulator85::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.582_f32 + y.sin();
        let b = y * 6.528_f32 - x.cos();
        let mut acc = Accumulator85::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.383_f32 + y.sin();
        let b = y * 3.11_f32 - x.cos();
        let mut acc = Accumulator85::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.383_f32 + y.sin();
        let b = y * 3.397_f32 - x.cos();
        let mut acc = Accumulator85::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.484_f32 + y.sin();
        let b = y * 8.37_f32 - x.cos();
        let mut acc = Accumulator85::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.713_f32 + y.sin();
        let b = y * 1.463_f32 - x.cos();
        let mut acc = Accumulator85::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.657_f32 + y.sin();
        let b = y * 5.417_f32 - x.cos();
        let mut acc = Accumulator85::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.877_f32 + y.sin();
        let b = y * 8.358_f32 - x.cos();
        let mut acc = Accumulator85::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.372_f32 + y.sin();
        let b = y * 4.708_f32 - x.cos();
        let mut acc = Accumulator85::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_85(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m85-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_85() -> f32 {
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
        total += (dep_touch_85(total as u64) % 997) as f32;
        total
    }
}

pub mod m86 {
    use super::*;

    pub struct Accumulator86<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator86<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.766_f32 + y.sin();
        let b = y * 8.625_f32 - x.cos();
        let mut acc = Accumulator86::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.249_f32 + y.sin();
        let b = y * 3.637_f32 - x.cos();
        let mut acc = Accumulator86::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.084_f32 + y.sin();
        let b = y * 3.306_f32 - x.cos();
        let mut acc = Accumulator86::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.95_f32 + y.sin();
        let b = y * 0.351_f32 - x.cos();
        let mut acc = Accumulator86::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.792_f32 + y.sin();
        let b = y * 0.763_f32 - x.cos();
        let mut acc = Accumulator86::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.615_f32 + y.sin();
        let b = y * 8.527_f32 - x.cos();
        let mut acc = Accumulator86::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.038_f32 + y.sin();
        let b = y * 7.964_f32 - x.cos();
        let mut acc = Accumulator86::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.137_f32 + y.sin();
        let b = y * 6.297_f32 - x.cos();
        let mut acc = Accumulator86::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.711_f32 + y.sin();
        let b = y * 8.917_f32 - x.cos();
        let mut acc = Accumulator86::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.321_f32 + y.sin();
        let b = y * 7.343_f32 - x.cos();
        let mut acc = Accumulator86::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.718_f32 + y.sin();
        let b = y * 1.138_f32 - x.cos();
        let mut acc = Accumulator86::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.878_f32 + y.sin();
        let b = y * 8.881_f32 - x.cos();
        let mut acc = Accumulator86::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.843_f32 + y.sin();
        let b = y * 4.181_f32 - x.cos();
        let mut acc = Accumulator86::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.325_f32 + y.sin();
        let b = y * 2.143_f32 - x.cos();
        let mut acc = Accumulator86::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.66_f32 + y.sin();
        let b = y * 9.495_f32 - x.cos();
        let mut acc = Accumulator86::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.559_f32 + y.sin();
        let b = y * 6.517_f32 - x.cos();
        let mut acc = Accumulator86::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.442_f32 + y.sin();
        let b = y * 5.891_f32 - x.cos();
        let mut acc = Accumulator86::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.332_f32 + y.sin();
        let b = y * 1.384_f32 - x.cos();
        let mut acc = Accumulator86::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.901_f32 + y.sin();
        let b = y * 2.952_f32 - x.cos();
        let mut acc = Accumulator86::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.831_f32 + y.sin();
        let b = y * 3.595_f32 - x.cos();
        let mut acc = Accumulator86::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_86(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_86() -> f32 {
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
        total += (dep_touch_86(total as u64) % 997) as f32;
        total
    }
}

pub mod m87 {
    use super::*;

    pub struct Accumulator87<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator87<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.88_f32 + y.sin();
        let b = y * 6.017_f32 - x.cos();
        let mut acc = Accumulator87::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.246_f32 + y.sin();
        let b = y * 8.754_f32 - x.cos();
        let mut acc = Accumulator87::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.348_f32 + y.sin();
        let b = y * 3.83_f32 - x.cos();
        let mut acc = Accumulator87::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.855_f32 + y.sin();
        let b = y * 4.185_f32 - x.cos();
        let mut acc = Accumulator87::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.057_f32 + y.sin();
        let b = y * 3.325_f32 - x.cos();
        let mut acc = Accumulator87::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.63_f32 + y.sin();
        let b = y * 4.914_f32 - x.cos();
        let mut acc = Accumulator87::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.003_f32 + y.sin();
        let b = y * 6.651_f32 - x.cos();
        let mut acc = Accumulator87::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.028_f32 + y.sin();
        let b = y * 3.804_f32 - x.cos();
        let mut acc = Accumulator87::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.92_f32 + y.sin();
        let b = y * 1.708_f32 - x.cos();
        let mut acc = Accumulator87::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.02_f32 + y.sin();
        let b = y * 5.504_f32 - x.cos();
        let mut acc = Accumulator87::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.611_f32 + y.sin();
        let b = y * 3.645_f32 - x.cos();
        let mut acc = Accumulator87::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.132_f32 + y.sin();
        let b = y * 0.33_f32 - x.cos();
        let mut acc = Accumulator87::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.441_f32 + y.sin();
        let b = y * 7.101_f32 - x.cos();
        let mut acc = Accumulator87::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.459_f32 + y.sin();
        let b = y * 5.106_f32 - x.cos();
        let mut acc = Accumulator87::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.588_f32 + y.sin();
        let b = y * 1.508_f32 - x.cos();
        let mut acc = Accumulator87::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.395_f32 + y.sin();
        let b = y * 0.398_f32 - x.cos();
        let mut acc = Accumulator87::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.001_f32 + y.sin();
        let b = y * 2.64_f32 - x.cos();
        let mut acc = Accumulator87::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.391_f32 + y.sin();
        let b = y * 2.221_f32 - x.cos();
        let mut acc = Accumulator87::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.689_f32 + y.sin();
        let b = y * 1.005_f32 - x.cos();
        let mut acc = Accumulator87::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.966_f32 + y.sin();
        let b = y * 8.952_f32 - x.cos();
        let mut acc = Accumulator87::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_87(seed: u64) -> u64 {
        let re = Regex::new(r"m87-(\d+)").unwrap();
        let hay = format!("m87-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_87() -> f32 {
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
        total += (dep_touch_87(total as u64) % 997) as f32;
        total
    }
}

pub mod m88 {
    use super::*;

    pub struct Accumulator88<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator88<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.434_f32 + y.sin();
        let b = y * 2.423_f32 - x.cos();
        let mut acc = Accumulator88::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.374_f32 + y.sin();
        let b = y * 8.155_f32 - x.cos();
        let mut acc = Accumulator88::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.009_f32 + y.sin();
        let b = y * 8.961_f32 - x.cos();
        let mut acc = Accumulator88::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.665_f32 + y.sin();
        let b = y * 7.171_f32 - x.cos();
        let mut acc = Accumulator88::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.442_f32 + y.sin();
        let b = y * 7.078_f32 - x.cos();
        let mut acc = Accumulator88::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.215_f32 + y.sin();
        let b = y * 8.293_f32 - x.cos();
        let mut acc = Accumulator88::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.166_f32 + y.sin();
        let b = y * 0.615_f32 - x.cos();
        let mut acc = Accumulator88::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.438_f32 + y.sin();
        let b = y * 0.946_f32 - x.cos();
        let mut acc = Accumulator88::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.897_f32 + y.sin();
        let b = y * 5.581_f32 - x.cos();
        let mut acc = Accumulator88::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.493_f32 + y.sin();
        let b = y * 4.608_f32 - x.cos();
        let mut acc = Accumulator88::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.221_f32 + y.sin();
        let b = y * 5.422_f32 - x.cos();
        let mut acc = Accumulator88::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.613_f32 + y.sin();
        let b = y * 2.502_f32 - x.cos();
        let mut acc = Accumulator88::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.331_f32 + y.sin();
        let b = y * 8.619_f32 - x.cos();
        let mut acc = Accumulator88::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.158_f32 + y.sin();
        let b = y * 3.071_f32 - x.cos();
        let mut acc = Accumulator88::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.323_f32 + y.sin();
        let b = y * 4.466_f32 - x.cos();
        let mut acc = Accumulator88::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.993_f32 + y.sin();
        let b = y * 3.195_f32 - x.cos();
        let mut acc = Accumulator88::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.002_f32 + y.sin();
        let b = y * 4.193_f32 - x.cos();
        let mut acc = Accumulator88::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.633_f32 + y.sin();
        let b = y * 5.902_f32 - x.cos();
        let mut acc = Accumulator88::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.359_f32 + y.sin();
        let b = y * 4.72_f32 - x.cos();
        let mut acc = Accumulator88::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.724_f32 + y.sin();
        let b = y * 5.088_f32 - x.cos();
        let mut acc = Accumulator88::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_88(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_88() -> f32 {
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
        total += (dep_touch_88(total as u64) % 997) as f32;
        total
    }
}

pub mod m89 {
    use super::*;

    pub struct Accumulator89<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator89<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.758_f32 + y.sin();
        let b = y * 0.818_f32 - x.cos();
        let mut acc = Accumulator89::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.858_f32 + y.sin();
        let b = y * 0.782_f32 - x.cos();
        let mut acc = Accumulator89::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.57_f32 + y.sin();
        let b = y * 4.267_f32 - x.cos();
        let mut acc = Accumulator89::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.009_f32 + y.sin();
        let b = y * 8.335_f32 - x.cos();
        let mut acc = Accumulator89::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.593_f32 + y.sin();
        let b = y * 8.571_f32 - x.cos();
        let mut acc = Accumulator89::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.47_f32 + y.sin();
        let b = y * 0.508_f32 - x.cos();
        let mut acc = Accumulator89::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.773_f32 + y.sin();
        let b = y * 5.087_f32 - x.cos();
        let mut acc = Accumulator89::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.118_f32 + y.sin();
        let b = y * 4.036_f32 - x.cos();
        let mut acc = Accumulator89::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.174_f32 + y.sin();
        let b = y * 1.633_f32 - x.cos();
        let mut acc = Accumulator89::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.595_f32 + y.sin();
        let b = y * 7.303_f32 - x.cos();
        let mut acc = Accumulator89::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.524_f32 + y.sin();
        let b = y * 3.685_f32 - x.cos();
        let mut acc = Accumulator89::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.852_f32 + y.sin();
        let b = y * 5.882_f32 - x.cos();
        let mut acc = Accumulator89::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.253_f32 + y.sin();
        let b = y * 7.95_f32 - x.cos();
        let mut acc = Accumulator89::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.484_f32 + y.sin();
        let b = y * 4.045_f32 - x.cos();
        let mut acc = Accumulator89::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.899_f32 + y.sin();
        let b = y * 1.433_f32 - x.cos();
        let mut acc = Accumulator89::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.141_f32 + y.sin();
        let b = y * 8.004_f32 - x.cos();
        let mut acc = Accumulator89::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.958_f32 + y.sin();
        let b = y * 3.662_f32 - x.cos();
        let mut acc = Accumulator89::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.528_f32 + y.sin();
        let b = y * 5.313_f32 - x.cos();
        let mut acc = Accumulator89::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.364_f32 + y.sin();
        let b = y * 4.774_f32 - x.cos();
        let mut acc = Accumulator89::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.595_f32 + y.sin();
        let b = y * 6.075_f32 - x.cos();
        let mut acc = Accumulator89::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_89(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(89u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_89() -> f32 {
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
        total += (dep_touch_89(total as u64) % 997) as f32;
        total
    }
}

pub mod m90 {
    use super::*;

    pub struct Accumulator90<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator90<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.802_f32 + y.sin();
        let b = y * 2.654_f32 - x.cos();
        let mut acc = Accumulator90::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.94_f32 + y.sin();
        let b = y * 0.759_f32 - x.cos();
        let mut acc = Accumulator90::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.382_f32 + y.sin();
        let b = y * 2.803_f32 - x.cos();
        let mut acc = Accumulator90::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.922_f32 + y.sin();
        let b = y * 4.951_f32 - x.cos();
        let mut acc = Accumulator90::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.797_f32 + y.sin();
        let b = y * 0.842_f32 - x.cos();
        let mut acc = Accumulator90::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.683_f32 + y.sin();
        let b = y * 7.995_f32 - x.cos();
        let mut acc = Accumulator90::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.149_f32 + y.sin();
        let b = y * 2.249_f32 - x.cos();
        let mut acc = Accumulator90::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.686_f32 + y.sin();
        let b = y * 7.288_f32 - x.cos();
        let mut acc = Accumulator90::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.988_f32 + y.sin();
        let b = y * 1.15_f32 - x.cos();
        let mut acc = Accumulator90::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.499_f32 + y.sin();
        let b = y * 9.135_f32 - x.cos();
        let mut acc = Accumulator90::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.674_f32 + y.sin();
        let b = y * 1.556_f32 - x.cos();
        let mut acc = Accumulator90::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.491_f32 + y.sin();
        let b = y * 4.969_f32 - x.cos();
        let mut acc = Accumulator90::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.88_f32 + y.sin();
        let b = y * 1.013_f32 - x.cos();
        let mut acc = Accumulator90::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.485_f32 + y.sin();
        let b = y * 5.951_f32 - x.cos();
        let mut acc = Accumulator90::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.473_f32 + y.sin();
        let b = y * 0.648_f32 - x.cos();
        let mut acc = Accumulator90::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.482_f32 + y.sin();
        let b = y * 3.313_f32 - x.cos();
        let mut acc = Accumulator90::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.275_f32 + y.sin();
        let b = y * 1.201_f32 - x.cos();
        let mut acc = Accumulator90::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.15_f32 + y.sin();
        let b = y * 2.035_f32 - x.cos();
        let mut acc = Accumulator90::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.69_f32 + y.sin();
        let b = y * 9.602_f32 - x.cos();
        let mut acc = Accumulator90::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.285_f32 + y.sin();
        let b = y * 8.499_f32 - x.cos();
        let mut acc = Accumulator90::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_90(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_90() -> f32 {
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
        total += (dep_touch_90(total as u64) % 997) as f32;
        total
    }
}

pub mod m91 {
    use super::*;

    pub struct Accumulator91<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator91<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.931_f32 + y.sin();
        let b = y * 2.582_f32 - x.cos();
        let mut acc = Accumulator91::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.834_f32 + y.sin();
        let b = y * 0.34_f32 - x.cos();
        let mut acc = Accumulator91::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.018_f32 + y.sin();
        let b = y * 1.816_f32 - x.cos();
        let mut acc = Accumulator91::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.85_f32 + y.sin();
        let b = y * 7.878_f32 - x.cos();
        let mut acc = Accumulator91::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.324_f32 + y.sin();
        let b = y * 0.678_f32 - x.cos();
        let mut acc = Accumulator91::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.198_f32 + y.sin();
        let b = y * 8.973_f32 - x.cos();
        let mut acc = Accumulator91::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.464_f32 + y.sin();
        let b = y * 8.767_f32 - x.cos();
        let mut acc = Accumulator91::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.526_f32 + y.sin();
        let b = y * 8.737_f32 - x.cos();
        let mut acc = Accumulator91::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.307_f32 + y.sin();
        let b = y * 6.57_f32 - x.cos();
        let mut acc = Accumulator91::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.345_f32 + y.sin();
        let b = y * 8.522_f32 - x.cos();
        let mut acc = Accumulator91::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.162_f32 + y.sin();
        let b = y * 4.968_f32 - x.cos();
        let mut acc = Accumulator91::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.856_f32 + y.sin();
        let b = y * 6.869_f32 - x.cos();
        let mut acc = Accumulator91::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.003_f32 + y.sin();
        let b = y * 9.143_f32 - x.cos();
        let mut acc = Accumulator91::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.145_f32 + y.sin();
        let b = y * 0.155_f32 - x.cos();
        let mut acc = Accumulator91::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.158_f32 + y.sin();
        let b = y * 6.659_f32 - x.cos();
        let mut acc = Accumulator91::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.829_f32 + y.sin();
        let b = y * 6.594_f32 - x.cos();
        let mut acc = Accumulator91::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.507_f32 + y.sin();
        let b = y * 9.464_f32 - x.cos();
        let mut acc = Accumulator91::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.68_f32 + y.sin();
        let b = y * 8.871_f32 - x.cos();
        let mut acc = Accumulator91::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.736_f32 + y.sin();
        let b = y * 8.957_f32 - x.cos();
        let mut acc = Accumulator91::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.824_f32 + y.sin();
        let b = y * 8.148_f32 - x.cos();
        let mut acc = Accumulator91::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_91(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_91() -> f32 {
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
        total += (dep_touch_91(total as u64) % 997) as f32;
        total
    }
}

pub mod m92 {
    use super::*;

    pub struct Accumulator92<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator92<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.842_f32 + y.sin();
        let b = y * 3.219_f32 - x.cos();
        let mut acc = Accumulator92::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.587_f32 + y.sin();
        let b = y * 7.95_f32 - x.cos();
        let mut acc = Accumulator92::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.653_f32 + y.sin();
        let b = y * 8.505_f32 - x.cos();
        let mut acc = Accumulator92::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.43_f32 + y.sin();
        let b = y * 8.513_f32 - x.cos();
        let mut acc = Accumulator92::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.922_f32 + y.sin();
        let b = y * 8.221_f32 - x.cos();
        let mut acc = Accumulator92::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.561_f32 + y.sin();
        let b = y * 7.558_f32 - x.cos();
        let mut acc = Accumulator92::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.161_f32 + y.sin();
        let b = y * 2.982_f32 - x.cos();
        let mut acc = Accumulator92::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.287_f32 + y.sin();
        let b = y * 9.247_f32 - x.cos();
        let mut acc = Accumulator92::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.872_f32 + y.sin();
        let b = y * 1.409_f32 - x.cos();
        let mut acc = Accumulator92::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.811_f32 + y.sin();
        let b = y * 1.214_f32 - x.cos();
        let mut acc = Accumulator92::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.43_f32 + y.sin();
        let b = y * 7.218_f32 - x.cos();
        let mut acc = Accumulator92::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.897_f32 + y.sin();
        let b = y * 4.499_f32 - x.cos();
        let mut acc = Accumulator92::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.056_f32 + y.sin();
        let b = y * 4.94_f32 - x.cos();
        let mut acc = Accumulator92::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.033_f32 + y.sin();
        let b = y * 0.737_f32 - x.cos();
        let mut acc = Accumulator92::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.823_f32 + y.sin();
        let b = y * 3.731_f32 - x.cos();
        let mut acc = Accumulator92::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.342_f32 + y.sin();
        let b = y * 9.879_f32 - x.cos();
        let mut acc = Accumulator92::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.635_f32 + y.sin();
        let b = y * 6.888_f32 - x.cos();
        let mut acc = Accumulator92::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.534_f32 + y.sin();
        let b = y * 4.571_f32 - x.cos();
        let mut acc = Accumulator92::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.032_f32 + y.sin();
        let b = y * 9.41_f32 - x.cos();
        let mut acc = Accumulator92::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.491_f32 + y.sin();
        let b = y * 0.554_f32 - x.cos();
        let mut acc = Accumulator92::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_92(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m92-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_92() -> f32 {
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
        total += (dep_touch_92(total as u64) % 997) as f32;
        total
    }
}

pub mod m93 {
    use super::*;

    pub struct Accumulator93<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator93<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.051_f32 + y.sin();
        let b = y * 2.993_f32 - x.cos();
        let mut acc = Accumulator93::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.785_f32 + y.sin();
        let b = y * 5.401_f32 - x.cos();
        let mut acc = Accumulator93::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.288_f32 + y.sin();
        let b = y * 3.546_f32 - x.cos();
        let mut acc = Accumulator93::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.507_f32 + y.sin();
        let b = y * 2.645_f32 - x.cos();
        let mut acc = Accumulator93::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.426_f32 + y.sin();
        let b = y * 4.784_f32 - x.cos();
        let mut acc = Accumulator93::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.902_f32 + y.sin();
        let b = y * 7.853_f32 - x.cos();
        let mut acc = Accumulator93::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.211_f32 + y.sin();
        let b = y * 5.987_f32 - x.cos();
        let mut acc = Accumulator93::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.889_f32 + y.sin();
        let b = y * 2.952_f32 - x.cos();
        let mut acc = Accumulator93::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.916_f32 + y.sin();
        let b = y * 1.439_f32 - x.cos();
        let mut acc = Accumulator93::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.422_f32 + y.sin();
        let b = y * 8.316_f32 - x.cos();
        let mut acc = Accumulator93::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.396_f32 + y.sin();
        let b = y * 1.705_f32 - x.cos();
        let mut acc = Accumulator93::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.744_f32 + y.sin();
        let b = y * 7.801_f32 - x.cos();
        let mut acc = Accumulator93::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.644_f32 + y.sin();
        let b = y * 0.697_f32 - x.cos();
        let mut acc = Accumulator93::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.983_f32 + y.sin();
        let b = y * 6.106_f32 - x.cos();
        let mut acc = Accumulator93::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.282_f32 + y.sin();
        let b = y * 5.323_f32 - x.cos();
        let mut acc = Accumulator93::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.18_f32 + y.sin();
        let b = y * 4.18_f32 - x.cos();
        let mut acc = Accumulator93::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.995_f32 + y.sin();
        let b = y * 6.784_f32 - x.cos();
        let mut acc = Accumulator93::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.859_f32 + y.sin();
        let b = y * 6.495_f32 - x.cos();
        let mut acc = Accumulator93::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.165_f32 + y.sin();
        let b = y * 9.528_f32 - x.cos();
        let mut acc = Accumulator93::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.673_f32 + y.sin();
        let b = y * 6.802_f32 - x.cos();
        let mut acc = Accumulator93::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_93(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_93() -> f32 {
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
        total += (dep_touch_93(total as u64) % 997) as f32;
        total
    }
}

pub mod m94 {
    use super::*;

    pub struct Accumulator94<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator94<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.219_f32 + y.sin();
        let b = y * 8.816_f32 - x.cos();
        let mut acc = Accumulator94::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.909_f32 + y.sin();
        let b = y * 4.702_f32 - x.cos();
        let mut acc = Accumulator94::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.62_f32 + y.sin();
        let b = y * 1.069_f32 - x.cos();
        let mut acc = Accumulator94::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.566_f32 + y.sin();
        let b = y * 5.876_f32 - x.cos();
        let mut acc = Accumulator94::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.459_f32 + y.sin();
        let b = y * 1.034_f32 - x.cos();
        let mut acc = Accumulator94::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.506_f32 + y.sin();
        let b = y * 9.72_f32 - x.cos();
        let mut acc = Accumulator94::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.987_f32 + y.sin();
        let b = y * 2.87_f32 - x.cos();
        let mut acc = Accumulator94::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.873_f32 + y.sin();
        let b = y * 5.465_f32 - x.cos();
        let mut acc = Accumulator94::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.65_f32 + y.sin();
        let b = y * 3.215_f32 - x.cos();
        let mut acc = Accumulator94::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.315_f32 + y.sin();
        let b = y * 2.886_f32 - x.cos();
        let mut acc = Accumulator94::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.721_f32 + y.sin();
        let b = y * 7.724_f32 - x.cos();
        let mut acc = Accumulator94::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.916_f32 + y.sin();
        let b = y * 4.811_f32 - x.cos();
        let mut acc = Accumulator94::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.038_f32 + y.sin();
        let b = y * 9.49_f32 - x.cos();
        let mut acc = Accumulator94::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.665_f32 + y.sin();
        let b = y * 3.631_f32 - x.cos();
        let mut acc = Accumulator94::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.966_f32 + y.sin();
        let b = y * 2.972_f32 - x.cos();
        let mut acc = Accumulator94::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.493_f32 + y.sin();
        let b = y * 9.634_f32 - x.cos();
        let mut acc = Accumulator94::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.221_f32 + y.sin();
        let b = y * 2.154_f32 - x.cos();
        let mut acc = Accumulator94::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.117_f32 + y.sin();
        let b = y * 5.073_f32 - x.cos();
        let mut acc = Accumulator94::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.481_f32 + y.sin();
        let b = y * 2.425_f32 - x.cos();
        let mut acc = Accumulator94::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.806_f32 + y.sin();
        let b = y * 9.181_f32 - x.cos();
        let mut acc = Accumulator94::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_94(seed: u64) -> u64 {
        let re = Regex::new(r"m94-(\d+)").unwrap();
        let hay = format!("m94-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_94() -> f32 {
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
        total += (dep_touch_94(total as u64) % 997) as f32;
        total
    }
}

pub mod m95 {
    use super::*;

    pub struct Accumulator95<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator95<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.746_f32 + y.sin();
        let b = y * 7.089_f32 - x.cos();
        let mut acc = Accumulator95::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.712_f32 + y.sin();
        let b = y * 9.654_f32 - x.cos();
        let mut acc = Accumulator95::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.597_f32 + y.sin();
        let b = y * 3.167_f32 - x.cos();
        let mut acc = Accumulator95::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.893_f32 + y.sin();
        let b = y * 5.149_f32 - x.cos();
        let mut acc = Accumulator95::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.945_f32 + y.sin();
        let b = y * 5.877_f32 - x.cos();
        let mut acc = Accumulator95::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.228_f32 + y.sin();
        let b = y * 2.087_f32 - x.cos();
        let mut acc = Accumulator95::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.98_f32 + y.sin();
        let b = y * 2.297_f32 - x.cos();
        let mut acc = Accumulator95::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.234_f32 + y.sin();
        let b = y * 2.721_f32 - x.cos();
        let mut acc = Accumulator95::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.153_f32 + y.sin();
        let b = y * 3.27_f32 - x.cos();
        let mut acc = Accumulator95::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.07_f32 + y.sin();
        let b = y * 6.935_f32 - x.cos();
        let mut acc = Accumulator95::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.946_f32 + y.sin();
        let b = y * 6.937_f32 - x.cos();
        let mut acc = Accumulator95::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.183_f32 + y.sin();
        let b = y * 7.413_f32 - x.cos();
        let mut acc = Accumulator95::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.141_f32 + y.sin();
        let b = y * 4.293_f32 - x.cos();
        let mut acc = Accumulator95::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.316_f32 + y.sin();
        let b = y * 0.367_f32 - x.cos();
        let mut acc = Accumulator95::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.837_f32 + y.sin();
        let b = y * 6.208_f32 - x.cos();
        let mut acc = Accumulator95::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.875_f32 + y.sin();
        let b = y * 3.744_f32 - x.cos();
        let mut acc = Accumulator95::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.502_f32 + y.sin();
        let b = y * 3.618_f32 - x.cos();
        let mut acc = Accumulator95::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.6_f32 + y.sin();
        let b = y * 1.488_f32 - x.cos();
        let mut acc = Accumulator95::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.586_f32 + y.sin();
        let b = y * 1.182_f32 - x.cos();
        let mut acc = Accumulator95::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.668_f32 + y.sin();
        let b = y * 5.027_f32 - x.cos();
        let mut acc = Accumulator95::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_95(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_95() -> f32 {
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
        total += (dep_touch_95(total as u64) % 997) as f32;
        total
    }
}

pub mod m96 {
    use super::*;

    pub struct Accumulator96<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator96<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.041_f32 + y.sin();
        let b = y * 4.966_f32 - x.cos();
        let mut acc = Accumulator96::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.427_f32 + y.sin();
        let b = y * 0.248_f32 - x.cos();
        let mut acc = Accumulator96::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.415_f32 + y.sin();
        let b = y * 1.477_f32 - x.cos();
        let mut acc = Accumulator96::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.156_f32 + y.sin();
        let b = y * 8.874_f32 - x.cos();
        let mut acc = Accumulator96::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.466_f32 + y.sin();
        let b = y * 1.007_f32 - x.cos();
        let mut acc = Accumulator96::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.872_f32 + y.sin();
        let b = y * 4.308_f32 - x.cos();
        let mut acc = Accumulator96::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.598_f32 + y.sin();
        let b = y * 2.279_f32 - x.cos();
        let mut acc = Accumulator96::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.827_f32 + y.sin();
        let b = y * 0.58_f32 - x.cos();
        let mut acc = Accumulator96::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.441_f32 + y.sin();
        let b = y * 0.469_f32 - x.cos();
        let mut acc = Accumulator96::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.043_f32 + y.sin();
        let b = y * 2.646_f32 - x.cos();
        let mut acc = Accumulator96::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.494_f32 + y.sin();
        let b = y * 3.42_f32 - x.cos();
        let mut acc = Accumulator96::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.251_f32 + y.sin();
        let b = y * 3.124_f32 - x.cos();
        let mut acc = Accumulator96::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.734_f32 + y.sin();
        let b = y * 4.48_f32 - x.cos();
        let mut acc = Accumulator96::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.616_f32 + y.sin();
        let b = y * 3.092_f32 - x.cos();
        let mut acc = Accumulator96::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.229_f32 + y.sin();
        let b = y * 1.14_f32 - x.cos();
        let mut acc = Accumulator96::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.303_f32 + y.sin();
        let b = y * 1.726_f32 - x.cos();
        let mut acc = Accumulator96::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.978_f32 + y.sin();
        let b = y * 9.8_f32 - x.cos();
        let mut acc = Accumulator96::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.451_f32 + y.sin();
        let b = y * 1.635_f32 - x.cos();
        let mut acc = Accumulator96::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.844_f32 + y.sin();
        let b = y * 2.892_f32 - x.cos();
        let mut acc = Accumulator96::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.386_f32 + y.sin();
        let b = y * 1.104_f32 - x.cos();
        let mut acc = Accumulator96::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_96(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(96u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_96() -> f32 {
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
        total += (dep_touch_96(total as u64) % 997) as f32;
        total
    }
}

pub mod m97 {
    use super::*;

    pub struct Accumulator97<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator97<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.431_f32 + y.sin();
        let b = y * 1.974_f32 - x.cos();
        let mut acc = Accumulator97::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.747_f32 + y.sin();
        let b = y * 6.662_f32 - x.cos();
        let mut acc = Accumulator97::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.638_f32 + y.sin();
        let b = y * 4.548_f32 - x.cos();
        let mut acc = Accumulator97::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.945_f32 + y.sin();
        let b = y * 0.135_f32 - x.cos();
        let mut acc = Accumulator97::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.586_f32 + y.sin();
        let b = y * 1.09_f32 - x.cos();
        let mut acc = Accumulator97::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.961_f32 + y.sin();
        let b = y * 6.837_f32 - x.cos();
        let mut acc = Accumulator97::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.383_f32 + y.sin();
        let b = y * 2.709_f32 - x.cos();
        let mut acc = Accumulator97::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.336_f32 + y.sin();
        let b = y * 0.471_f32 - x.cos();
        let mut acc = Accumulator97::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.847_f32 + y.sin();
        let b = y * 4.321_f32 - x.cos();
        let mut acc = Accumulator97::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.931_f32 + y.sin();
        let b = y * 0.657_f32 - x.cos();
        let mut acc = Accumulator97::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.808_f32 + y.sin();
        let b = y * 9.115_f32 - x.cos();
        let mut acc = Accumulator97::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.848_f32 + y.sin();
        let b = y * 6.768_f32 - x.cos();
        let mut acc = Accumulator97::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.001_f32 + y.sin();
        let b = y * 6.664_f32 - x.cos();
        let mut acc = Accumulator97::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.615_f32 + y.sin();
        let b = y * 4.489_f32 - x.cos();
        let mut acc = Accumulator97::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.252_f32 + y.sin();
        let b = y * 8.082_f32 - x.cos();
        let mut acc = Accumulator97::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.344_f32 + y.sin();
        let b = y * 0.758_f32 - x.cos();
        let mut acc = Accumulator97::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.345_f32 + y.sin();
        let b = y * 6.82_f32 - x.cos();
        let mut acc = Accumulator97::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.911_f32 + y.sin();
        let b = y * 9.753_f32 - x.cos();
        let mut acc = Accumulator97::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.543_f32 + y.sin();
        let b = y * 7.11_f32 - x.cos();
        let mut acc = Accumulator97::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.206_f32 + y.sin();
        let b = y * 3.256_f32 - x.cos();
        let mut acc = Accumulator97::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_97(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_97() -> f32 {
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
        total += (dep_touch_97(total as u64) % 997) as f32;
        total
    }
}

pub mod m98 {
    use super::*;

    pub struct Accumulator98<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator98<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.266_f32 + y.sin();
        let b = y * 9.395_f32 - x.cos();
        let mut acc = Accumulator98::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.51_f32 + y.sin();
        let b = y * 6.916_f32 - x.cos();
        let mut acc = Accumulator98::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.085_f32 + y.sin();
        let b = y * 8.209_f32 - x.cos();
        let mut acc = Accumulator98::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.686_f32 + y.sin();
        let b = y * 4.392_f32 - x.cos();
        let mut acc = Accumulator98::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.394_f32 + y.sin();
        let b = y * 5.43_f32 - x.cos();
        let mut acc = Accumulator98::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.574_f32 + y.sin();
        let b = y * 6.34_f32 - x.cos();
        let mut acc = Accumulator98::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.558_f32 + y.sin();
        let b = y * 2.655_f32 - x.cos();
        let mut acc = Accumulator98::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.229_f32 + y.sin();
        let b = y * 3.83_f32 - x.cos();
        let mut acc = Accumulator98::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.266_f32 + y.sin();
        let b = y * 1.599_f32 - x.cos();
        let mut acc = Accumulator98::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.925_f32 + y.sin();
        let b = y * 3.099_f32 - x.cos();
        let mut acc = Accumulator98::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.219_f32 + y.sin();
        let b = y * 9.007_f32 - x.cos();
        let mut acc = Accumulator98::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.578_f32 + y.sin();
        let b = y * 5.865_f32 - x.cos();
        let mut acc = Accumulator98::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.793_f32 + y.sin();
        let b = y * 8.326_f32 - x.cos();
        let mut acc = Accumulator98::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.026_f32 + y.sin();
        let b = y * 5.184_f32 - x.cos();
        let mut acc = Accumulator98::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.229_f32 + y.sin();
        let b = y * 1.338_f32 - x.cos();
        let mut acc = Accumulator98::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.103_f32 + y.sin();
        let b = y * 5.863_f32 - x.cos();
        let mut acc = Accumulator98::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.171_f32 + y.sin();
        let b = y * 6.563_f32 - x.cos();
        let mut acc = Accumulator98::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.038_f32 + y.sin();
        let b = y * 4.204_f32 - x.cos();
        let mut acc = Accumulator98::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.958_f32 + y.sin();
        let b = y * 0.527_f32 - x.cos();
        let mut acc = Accumulator98::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.803_f32 + y.sin();
        let b = y * 7.646_f32 - x.cos();
        let mut acc = Accumulator98::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_98(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_98() -> f32 {
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
        total += (dep_touch_98(total as u64) % 997) as f32;
        total
    }
}

pub mod m99 {
    use super::*;

    pub struct Accumulator99<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator99<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.47_f32 + y.sin();
        let b = y * 3.775_f32 - x.cos();
        let mut acc = Accumulator99::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.54_f32 + y.sin();
        let b = y * 9.171_f32 - x.cos();
        let mut acc = Accumulator99::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.417_f32 + y.sin();
        let b = y * 5.432_f32 - x.cos();
        let mut acc = Accumulator99::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.04_f32 + y.sin();
        let b = y * 7.045_f32 - x.cos();
        let mut acc = Accumulator99::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.829_f32 + y.sin();
        let b = y * 7.265_f32 - x.cos();
        let mut acc = Accumulator99::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.247_f32 + y.sin();
        let b = y * 8.549_f32 - x.cos();
        let mut acc = Accumulator99::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.103_f32 + y.sin();
        let b = y * 3.623_f32 - x.cos();
        let mut acc = Accumulator99::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.632_f32 + y.sin();
        let b = y * 6.511_f32 - x.cos();
        let mut acc = Accumulator99::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.737_f32 + y.sin();
        let b = y * 4.335_f32 - x.cos();
        let mut acc = Accumulator99::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.726_f32 + y.sin();
        let b = y * 6.325_f32 - x.cos();
        let mut acc = Accumulator99::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.264_f32 + y.sin();
        let b = y * 5.474_f32 - x.cos();
        let mut acc = Accumulator99::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.634_f32 + y.sin();
        let b = y * 0.417_f32 - x.cos();
        let mut acc = Accumulator99::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.243_f32 + y.sin();
        let b = y * 6.668_f32 - x.cos();
        let mut acc = Accumulator99::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.983_f32 + y.sin();
        let b = y * 2.347_f32 - x.cos();
        let mut acc = Accumulator99::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.728_f32 + y.sin();
        let b = y * 7.153_f32 - x.cos();
        let mut acc = Accumulator99::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.324_f32 + y.sin();
        let b = y * 4.879_f32 - x.cos();
        let mut acc = Accumulator99::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.031_f32 + y.sin();
        let b = y * 8.884_f32 - x.cos();
        let mut acc = Accumulator99::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.078_f32 + y.sin();
        let b = y * 8.875_f32 - x.cos();
        let mut acc = Accumulator99::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.603_f32 + y.sin();
        let b = y * 4.405_f32 - x.cos();
        let mut acc = Accumulator99::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.623_f32 + y.sin();
        let b = y * 1.608_f32 - x.cos();
        let mut acc = Accumulator99::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_99(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m99-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_99() -> f32 {
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
        total += (dep_touch_99(total as u64) % 997) as f32;
        total
    }
}

pub fn touch_bulk_0() -> f32 {
    let mut total = 0.0_f32;
    total += m0::run_all_0();
    total += m1::run_all_1();
    total += m2::run_all_2();
    total += m3::run_all_3();
    total += m4::run_all_4();
    total += m5::run_all_5();
    total += m6::run_all_6();
    total += m7::run_all_7();
    total += m8::run_all_8();
    total += m9::run_all_9();
    total += m10::run_all_10();
    total += m11::run_all_11();
    total += m12::run_all_12();
    total += m13::run_all_13();
    total += m14::run_all_14();
    total += m15::run_all_15();
    total += m16::run_all_16();
    total += m17::run_all_17();
    total += m18::run_all_18();
    total += m19::run_all_19();
    total += m20::run_all_20();
    total += m21::run_all_21();
    total += m22::run_all_22();
    total += m23::run_all_23();
    total += m24::run_all_24();
    total += m25::run_all_25();
    total += m26::run_all_26();
    total += m27::run_all_27();
    total += m28::run_all_28();
    total += m29::run_all_29();
    total += m30::run_all_30();
    total += m31::run_all_31();
    total += m32::run_all_32();
    total += m33::run_all_33();
    total += m34::run_all_34();
    total += m35::run_all_35();
    total += m36::run_all_36();
    total += m37::run_all_37();
    total += m38::run_all_38();
    total += m39::run_all_39();
    total += m40::run_all_40();
    total += m41::run_all_41();
    total += m42::run_all_42();
    total += m43::run_all_43();
    total += m44::run_all_44();
    total += m45::run_all_45();
    total += m46::run_all_46();
    total += m47::run_all_47();
    total += m48::run_all_48();
    total += m49::run_all_49();
    total += m50::run_all_50();
    total += m51::run_all_51();
    total += m52::run_all_52();
    total += m53::run_all_53();
    total += m54::run_all_54();
    total += m55::run_all_55();
    total += m56::run_all_56();
    total += m57::run_all_57();
    total += m58::run_all_58();
    total += m59::run_all_59();
    total += m60::run_all_60();
    total += m61::run_all_61();
    total += m62::run_all_62();
    total += m63::run_all_63();
    total += m64::run_all_64();
    total += m65::run_all_65();
    total += m66::run_all_66();
    total += m67::run_all_67();
    total += m68::run_all_68();
    total += m69::run_all_69();
    total += m70::run_all_70();
    total += m71::run_all_71();
    total += m72::run_all_72();
    total += m73::run_all_73();
    total += m74::run_all_74();
    total += m75::run_all_75();
    total += m76::run_all_76();
    total += m77::run_all_77();
    total += m78::run_all_78();
    total += m79::run_all_79();
    total += m80::run_all_80();
    total += m81::run_all_81();
    total += m82::run_all_82();
    total += m83::run_all_83();
    total += m84::run_all_84();
    total += m85::run_all_85();
    total += m86::run_all_86();
    total += m87::run_all_87();
    total += m88::run_all_88();
    total += m89::run_all_89();
    total += m90::run_all_90();
    total += m91::run_all_91();
    total += m92::run_all_92();
    total += m93::run_all_93();
    total += m94::run_all_94();
    total += m95::run_all_95();
    total += m96::run_all_96();
    total += m97::run_all_97();
    total += m98::run_all_98();
    total += m99::run_all_99();
    total
}
