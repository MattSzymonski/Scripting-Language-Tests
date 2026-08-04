//! Auto-generated bulk module (file 11) - exists purely to make `game_rs`
//! heavier for a hot-patch speed experiment. Real arithmetic, real generics
//! (so monomorphization isn't free), and real use of a rotating set of
//! external crates so none of them are dead code. `touch_bulk_11()` is
//! called once from `game::init` so nothing here gets stripped as unreachable.

use rand::{Rng, SeedableRng};
use sha2::{Digest, Sha256};
use noise::{NoiseFn, Perlin};
use regex::Regex;
use itertools::Itertools;
use uuid::Uuid;
use chrono::Utc;


pub mod m1100 {
    use super::*;

    pub struct Accumulator1100<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1100<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.349_f32 + y.sin();
        let b = y * 8.469_f32 - x.cos();
        let mut acc = Accumulator1100::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.781_f32 + y.sin();
        let b = y * 1.545_f32 - x.cos();
        let mut acc = Accumulator1100::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.733_f32 + y.sin();
        let b = y * 3.116_f32 - x.cos();
        let mut acc = Accumulator1100::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.156_f32 + y.sin();
        let b = y * 9.639_f32 - x.cos();
        let mut acc = Accumulator1100::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.458_f32 + y.sin();
        let b = y * 1.084_f32 - x.cos();
        let mut acc = Accumulator1100::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.423_f32 + y.sin();
        let b = y * 4.26_f32 - x.cos();
        let mut acc = Accumulator1100::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.186_f32 + y.sin();
        let b = y * 1.388_f32 - x.cos();
        let mut acc = Accumulator1100::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.73_f32 + y.sin();
        let b = y * 2.605_f32 - x.cos();
        let mut acc = Accumulator1100::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.143_f32 + y.sin();
        let b = y * 2.245_f32 - x.cos();
        let mut acc = Accumulator1100::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.126_f32 + y.sin();
        let b = y * 3.01_f32 - x.cos();
        let mut acc = Accumulator1100::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.517_f32 + y.sin();
        let b = y * 5.775_f32 - x.cos();
        let mut acc = Accumulator1100::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.114_f32 + y.sin();
        let b = y * 7.837_f32 - x.cos();
        let mut acc = Accumulator1100::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.287_f32 + y.sin();
        let b = y * 4.735_f32 - x.cos();
        let mut acc = Accumulator1100::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.556_f32 + y.sin();
        let b = y * 5.115_f32 - x.cos();
        let mut acc = Accumulator1100::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.629_f32 + y.sin();
        let b = y * 1.268_f32 - x.cos();
        let mut acc = Accumulator1100::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.405_f32 + y.sin();
        let b = y * 8.394_f32 - x.cos();
        let mut acc = Accumulator1100::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.647_f32 + y.sin();
        let b = y * 5.575_f32 - x.cos();
        let mut acc = Accumulator1100::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.227_f32 + y.sin();
        let b = y * 4.005_f32 - x.cos();
        let mut acc = Accumulator1100::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.228_f32 + y.sin();
        let b = y * 3.325_f32 - x.cos();
        let mut acc = Accumulator1100::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.67_f32 + y.sin();
        let b = y * 1.852_f32 - x.cos();
        let mut acc = Accumulator1100::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1100(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m1100-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_1100() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1100(total as u64) % 997) as f32;
        total
    }
}

pub mod m1101 {
    use super::*;

    pub struct Accumulator1101<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1101<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.867_f32 + y.sin();
        let b = y * 6.093_f32 - x.cos();
        let mut acc = Accumulator1101::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.615_f32 + y.sin();
        let b = y * 8.738_f32 - x.cos();
        let mut acc = Accumulator1101::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.976_f32 + y.sin();
        let b = y * 7.624_f32 - x.cos();
        let mut acc = Accumulator1101::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.944_f32 + y.sin();
        let b = y * 6.515_f32 - x.cos();
        let mut acc = Accumulator1101::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.003_f32 + y.sin();
        let b = y * 8.171_f32 - x.cos();
        let mut acc = Accumulator1101::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.234_f32 + y.sin();
        let b = y * 4.837_f32 - x.cos();
        let mut acc = Accumulator1101::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.337_f32 + y.sin();
        let b = y * 1.241_f32 - x.cos();
        let mut acc = Accumulator1101::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.289_f32 + y.sin();
        let b = y * 3.413_f32 - x.cos();
        let mut acc = Accumulator1101::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.311_f32 + y.sin();
        let b = y * 6.811_f32 - x.cos();
        let mut acc = Accumulator1101::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.553_f32 + y.sin();
        let b = y * 9.503_f32 - x.cos();
        let mut acc = Accumulator1101::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.416_f32 + y.sin();
        let b = y * 8.227_f32 - x.cos();
        let mut acc = Accumulator1101::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.425_f32 + y.sin();
        let b = y * 1.606_f32 - x.cos();
        let mut acc = Accumulator1101::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.381_f32 + y.sin();
        let b = y * 6.709_f32 - x.cos();
        let mut acc = Accumulator1101::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.548_f32 + y.sin();
        let b = y * 6.257_f32 - x.cos();
        let mut acc = Accumulator1101::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.578_f32 + y.sin();
        let b = y * 0.631_f32 - x.cos();
        let mut acc = Accumulator1101::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.369_f32 + y.sin();
        let b = y * 0.126_f32 - x.cos();
        let mut acc = Accumulator1101::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.774_f32 + y.sin();
        let b = y * 1.581_f32 - x.cos();
        let mut acc = Accumulator1101::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.35_f32 + y.sin();
        let b = y * 1.407_f32 - x.cos();
        let mut acc = Accumulator1101::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.818_f32 + y.sin();
        let b = y * 4.089_f32 - x.cos();
        let mut acc = Accumulator1101::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.681_f32 + y.sin();
        let b = y * 8.587_f32 - x.cos();
        let mut acc = Accumulator1101::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1101(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_1101() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1101(total as u64) % 997) as f32;
        total
    }
}

pub mod m1102 {
    use super::*;

    pub struct Accumulator1102<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1102<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.684_f32 + y.sin();
        let b = y * 4.669_f32 - x.cos();
        let mut acc = Accumulator1102::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.174_f32 + y.sin();
        let b = y * 6.424_f32 - x.cos();
        let mut acc = Accumulator1102::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.552_f32 + y.sin();
        let b = y * 8.468_f32 - x.cos();
        let mut acc = Accumulator1102::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.943_f32 + y.sin();
        let b = y * 2.396_f32 - x.cos();
        let mut acc = Accumulator1102::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.019_f32 + y.sin();
        let b = y * 3.658_f32 - x.cos();
        let mut acc = Accumulator1102::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.989_f32 + y.sin();
        let b = y * 5.731_f32 - x.cos();
        let mut acc = Accumulator1102::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.301_f32 + y.sin();
        let b = y * 2.78_f32 - x.cos();
        let mut acc = Accumulator1102::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.718_f32 + y.sin();
        let b = y * 4.237_f32 - x.cos();
        let mut acc = Accumulator1102::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.765_f32 + y.sin();
        let b = y * 2.454_f32 - x.cos();
        let mut acc = Accumulator1102::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.346_f32 + y.sin();
        let b = y * 4.349_f32 - x.cos();
        let mut acc = Accumulator1102::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.413_f32 + y.sin();
        let b = y * 2.912_f32 - x.cos();
        let mut acc = Accumulator1102::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.043_f32 + y.sin();
        let b = y * 9.185_f32 - x.cos();
        let mut acc = Accumulator1102::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.577_f32 + y.sin();
        let b = y * 2.708_f32 - x.cos();
        let mut acc = Accumulator1102::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.685_f32 + y.sin();
        let b = y * 4.075_f32 - x.cos();
        let mut acc = Accumulator1102::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.4_f32 + y.sin();
        let b = y * 5.989_f32 - x.cos();
        let mut acc = Accumulator1102::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.485_f32 + y.sin();
        let b = y * 4.359_f32 - x.cos();
        let mut acc = Accumulator1102::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.13_f32 + y.sin();
        let b = y * 5.247_f32 - x.cos();
        let mut acc = Accumulator1102::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.532_f32 + y.sin();
        let b = y * 7.689_f32 - x.cos();
        let mut acc = Accumulator1102::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.001_f32 + y.sin();
        let b = y * 2.507_f32 - x.cos();
        let mut acc = Accumulator1102::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.319_f32 + y.sin();
        let b = y * 5.387_f32 - x.cos();
        let mut acc = Accumulator1102::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1102(seed: u64) -> u64 {
        let re = Regex::new(r"m1102-(\d+)").unwrap();
        let hay = format!("m1102-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_1102() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1102(total as u64) % 997) as f32;
        total
    }
}

pub mod m1103 {
    use super::*;

    pub struct Accumulator1103<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1103<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.522_f32 + y.sin();
        let b = y * 1.386_f32 - x.cos();
        let mut acc = Accumulator1103::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.577_f32 + y.sin();
        let b = y * 0.996_f32 - x.cos();
        let mut acc = Accumulator1103::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.266_f32 + y.sin();
        let b = y * 6.84_f32 - x.cos();
        let mut acc = Accumulator1103::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.823_f32 + y.sin();
        let b = y * 4.727_f32 - x.cos();
        let mut acc = Accumulator1103::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.347_f32 + y.sin();
        let b = y * 8.681_f32 - x.cos();
        let mut acc = Accumulator1103::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.308_f32 + y.sin();
        let b = y * 1.313_f32 - x.cos();
        let mut acc = Accumulator1103::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.062_f32 + y.sin();
        let b = y * 6.361_f32 - x.cos();
        let mut acc = Accumulator1103::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.173_f32 + y.sin();
        let b = y * 8.362_f32 - x.cos();
        let mut acc = Accumulator1103::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.164_f32 + y.sin();
        let b = y * 9.204_f32 - x.cos();
        let mut acc = Accumulator1103::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.211_f32 + y.sin();
        let b = y * 3.884_f32 - x.cos();
        let mut acc = Accumulator1103::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.122_f32 + y.sin();
        let b = y * 0.249_f32 - x.cos();
        let mut acc = Accumulator1103::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.43_f32 + y.sin();
        let b = y * 8.518_f32 - x.cos();
        let mut acc = Accumulator1103::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.319_f32 + y.sin();
        let b = y * 6.632_f32 - x.cos();
        let mut acc = Accumulator1103::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.141_f32 + y.sin();
        let b = y * 1.035_f32 - x.cos();
        let mut acc = Accumulator1103::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.24_f32 + y.sin();
        let b = y * 7.475_f32 - x.cos();
        let mut acc = Accumulator1103::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.612_f32 + y.sin();
        let b = y * 2.533_f32 - x.cos();
        let mut acc = Accumulator1103::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.473_f32 + y.sin();
        let b = y * 6.288_f32 - x.cos();
        let mut acc = Accumulator1103::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.014_f32 + y.sin();
        let b = y * 2.983_f32 - x.cos();
        let mut acc = Accumulator1103::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.17_f32 + y.sin();
        let b = y * 5.292_f32 - x.cos();
        let mut acc = Accumulator1103::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.703_f32 + y.sin();
        let b = y * 6.263_f32 - x.cos();
        let mut acc = Accumulator1103::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1103(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_1103() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1103(total as u64) % 997) as f32;
        total
    }
}

pub mod m1104 {
    use super::*;

    pub struct Accumulator1104<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1104<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.755_f32 + y.sin();
        let b = y * 3.456_f32 - x.cos();
        let mut acc = Accumulator1104::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.679_f32 + y.sin();
        let b = y * 4.445_f32 - x.cos();
        let mut acc = Accumulator1104::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.251_f32 + y.sin();
        let b = y * 9.332_f32 - x.cos();
        let mut acc = Accumulator1104::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.129_f32 + y.sin();
        let b = y * 6.682_f32 - x.cos();
        let mut acc = Accumulator1104::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.032_f32 + y.sin();
        let b = y * 4.17_f32 - x.cos();
        let mut acc = Accumulator1104::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.595_f32 + y.sin();
        let b = y * 2.877_f32 - x.cos();
        let mut acc = Accumulator1104::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.087_f32 + y.sin();
        let b = y * 0.668_f32 - x.cos();
        let mut acc = Accumulator1104::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.969_f32 + y.sin();
        let b = y * 4.437_f32 - x.cos();
        let mut acc = Accumulator1104::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.121_f32 + y.sin();
        let b = y * 3.486_f32 - x.cos();
        let mut acc = Accumulator1104::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.635_f32 + y.sin();
        let b = y * 3.469_f32 - x.cos();
        let mut acc = Accumulator1104::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.239_f32 + y.sin();
        let b = y * 2.971_f32 - x.cos();
        let mut acc = Accumulator1104::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.346_f32 + y.sin();
        let b = y * 4.495_f32 - x.cos();
        let mut acc = Accumulator1104::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.539_f32 + y.sin();
        let b = y * 1.311_f32 - x.cos();
        let mut acc = Accumulator1104::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.792_f32 + y.sin();
        let b = y * 7.766_f32 - x.cos();
        let mut acc = Accumulator1104::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.546_f32 + y.sin();
        let b = y * 5.389_f32 - x.cos();
        let mut acc = Accumulator1104::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.848_f32 + y.sin();
        let b = y * 9.741_f32 - x.cos();
        let mut acc = Accumulator1104::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.63_f32 + y.sin();
        let b = y * 9.54_f32 - x.cos();
        let mut acc = Accumulator1104::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.726_f32 + y.sin();
        let b = y * 6.874_f32 - x.cos();
        let mut acc = Accumulator1104::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.986_f32 + y.sin();
        let b = y * 9.846_f32 - x.cos();
        let mut acc = Accumulator1104::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.684_f32 + y.sin();
        let b = y * 0.217_f32 - x.cos();
        let mut acc = Accumulator1104::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1104(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(1104u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_1104() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1104(total as u64) % 997) as f32;
        total
    }
}

pub mod m1105 {
    use super::*;

    pub struct Accumulator1105<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1105<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.268_f32 + y.sin();
        let b = y * 3.687_f32 - x.cos();
        let mut acc = Accumulator1105::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.47_f32 + y.sin();
        let b = y * 2.664_f32 - x.cos();
        let mut acc = Accumulator1105::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.849_f32 + y.sin();
        let b = y * 1.177_f32 - x.cos();
        let mut acc = Accumulator1105::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.021_f32 + y.sin();
        let b = y * 7.521_f32 - x.cos();
        let mut acc = Accumulator1105::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.794_f32 + y.sin();
        let b = y * 8.844_f32 - x.cos();
        let mut acc = Accumulator1105::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.4_f32 + y.sin();
        let b = y * 4.177_f32 - x.cos();
        let mut acc = Accumulator1105::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.457_f32 + y.sin();
        let b = y * 5.224_f32 - x.cos();
        let mut acc = Accumulator1105::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.462_f32 + y.sin();
        let b = y * 9.529_f32 - x.cos();
        let mut acc = Accumulator1105::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.906_f32 + y.sin();
        let b = y * 2.787_f32 - x.cos();
        let mut acc = Accumulator1105::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.542_f32 + y.sin();
        let b = y * 3.723_f32 - x.cos();
        let mut acc = Accumulator1105::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.656_f32 + y.sin();
        let b = y * 5.291_f32 - x.cos();
        let mut acc = Accumulator1105::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.269_f32 + y.sin();
        let b = y * 3.096_f32 - x.cos();
        let mut acc = Accumulator1105::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.685_f32 + y.sin();
        let b = y * 4.143_f32 - x.cos();
        let mut acc = Accumulator1105::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.26_f32 + y.sin();
        let b = y * 8.8_f32 - x.cos();
        let mut acc = Accumulator1105::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.476_f32 + y.sin();
        let b = y * 2.763_f32 - x.cos();
        let mut acc = Accumulator1105::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.927_f32 + y.sin();
        let b = y * 7.278_f32 - x.cos();
        let mut acc = Accumulator1105::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.964_f32 + y.sin();
        let b = y * 9.293_f32 - x.cos();
        let mut acc = Accumulator1105::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.077_f32 + y.sin();
        let b = y * 1.033_f32 - x.cos();
        let mut acc = Accumulator1105::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.71_f32 + y.sin();
        let b = y * 7.246_f32 - x.cos();
        let mut acc = Accumulator1105::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.478_f32 + y.sin();
        let b = y * 5.976_f32 - x.cos();
        let mut acc = Accumulator1105::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1105(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_1105() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1105(total as u64) % 997) as f32;
        total
    }
}

pub mod m1106 {
    use super::*;

    pub struct Accumulator1106<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1106<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.197_f32 + y.sin();
        let b = y * 1.502_f32 - x.cos();
        let mut acc = Accumulator1106::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.717_f32 + y.sin();
        let b = y * 1.995_f32 - x.cos();
        let mut acc = Accumulator1106::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.594_f32 + y.sin();
        let b = y * 3.049_f32 - x.cos();
        let mut acc = Accumulator1106::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.99_f32 + y.sin();
        let b = y * 7.211_f32 - x.cos();
        let mut acc = Accumulator1106::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.821_f32 + y.sin();
        let b = y * 6.947_f32 - x.cos();
        let mut acc = Accumulator1106::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.912_f32 + y.sin();
        let b = y * 5.829_f32 - x.cos();
        let mut acc = Accumulator1106::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.695_f32 + y.sin();
        let b = y * 0.177_f32 - x.cos();
        let mut acc = Accumulator1106::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.242_f32 + y.sin();
        let b = y * 6.41_f32 - x.cos();
        let mut acc = Accumulator1106::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.605_f32 + y.sin();
        let b = y * 7.387_f32 - x.cos();
        let mut acc = Accumulator1106::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.314_f32 + y.sin();
        let b = y * 7.777_f32 - x.cos();
        let mut acc = Accumulator1106::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.908_f32 + y.sin();
        let b = y * 8.788_f32 - x.cos();
        let mut acc = Accumulator1106::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.477_f32 + y.sin();
        let b = y * 1.065_f32 - x.cos();
        let mut acc = Accumulator1106::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.374_f32 + y.sin();
        let b = y * 5.659_f32 - x.cos();
        let mut acc = Accumulator1106::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.443_f32 + y.sin();
        let b = y * 5.093_f32 - x.cos();
        let mut acc = Accumulator1106::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.549_f32 + y.sin();
        let b = y * 3.967_f32 - x.cos();
        let mut acc = Accumulator1106::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.3_f32 + y.sin();
        let b = y * 3.922_f32 - x.cos();
        let mut acc = Accumulator1106::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.862_f32 + y.sin();
        let b = y * 9.648_f32 - x.cos();
        let mut acc = Accumulator1106::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.195_f32 + y.sin();
        let b = y * 4.207_f32 - x.cos();
        let mut acc = Accumulator1106::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.696_f32 + y.sin();
        let b = y * 3.129_f32 - x.cos();
        let mut acc = Accumulator1106::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.306_f32 + y.sin();
        let b = y * 3.336_f32 - x.cos();
        let mut acc = Accumulator1106::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1106(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_1106() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1106(total as u64) % 997) as f32;
        total
    }
}

pub mod m1107 {
    use super::*;

    pub struct Accumulator1107<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1107<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.621_f32 + y.sin();
        let b = y * 1.816_f32 - x.cos();
        let mut acc = Accumulator1107::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.864_f32 + y.sin();
        let b = y * 2.931_f32 - x.cos();
        let mut acc = Accumulator1107::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.707_f32 + y.sin();
        let b = y * 0.666_f32 - x.cos();
        let mut acc = Accumulator1107::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.203_f32 + y.sin();
        let b = y * 5.609_f32 - x.cos();
        let mut acc = Accumulator1107::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.646_f32 + y.sin();
        let b = y * 4.82_f32 - x.cos();
        let mut acc = Accumulator1107::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.627_f32 + y.sin();
        let b = y * 1.577_f32 - x.cos();
        let mut acc = Accumulator1107::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.564_f32 + y.sin();
        let b = y * 9.001_f32 - x.cos();
        let mut acc = Accumulator1107::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.965_f32 + y.sin();
        let b = y * 3.587_f32 - x.cos();
        let mut acc = Accumulator1107::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.619_f32 + y.sin();
        let b = y * 5.307_f32 - x.cos();
        let mut acc = Accumulator1107::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.141_f32 + y.sin();
        let b = y * 2.732_f32 - x.cos();
        let mut acc = Accumulator1107::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.256_f32 + y.sin();
        let b = y * 6.198_f32 - x.cos();
        let mut acc = Accumulator1107::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.725_f32 + y.sin();
        let b = y * 6.989_f32 - x.cos();
        let mut acc = Accumulator1107::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.586_f32 + y.sin();
        let b = y * 0.883_f32 - x.cos();
        let mut acc = Accumulator1107::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.506_f32 + y.sin();
        let b = y * 0.113_f32 - x.cos();
        let mut acc = Accumulator1107::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.94_f32 + y.sin();
        let b = y * 6.503_f32 - x.cos();
        let mut acc = Accumulator1107::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.218_f32 + y.sin();
        let b = y * 6.272_f32 - x.cos();
        let mut acc = Accumulator1107::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.738_f32 + y.sin();
        let b = y * 6.679_f32 - x.cos();
        let mut acc = Accumulator1107::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.102_f32 + y.sin();
        let b = y * 3.657_f32 - x.cos();
        let mut acc = Accumulator1107::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.03_f32 + y.sin();
        let b = y * 4.463_f32 - x.cos();
        let mut acc = Accumulator1107::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.406_f32 + y.sin();
        let b = y * 4.016_f32 - x.cos();
        let mut acc = Accumulator1107::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1107(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m1107-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_1107() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1107(total as u64) % 997) as f32;
        total
    }
}

pub mod m1108 {
    use super::*;

    pub struct Accumulator1108<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1108<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.963_f32 + y.sin();
        let b = y * 6.966_f32 - x.cos();
        let mut acc = Accumulator1108::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.772_f32 + y.sin();
        let b = y * 9.038_f32 - x.cos();
        let mut acc = Accumulator1108::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.809_f32 + y.sin();
        let b = y * 2.781_f32 - x.cos();
        let mut acc = Accumulator1108::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.55_f32 + y.sin();
        let b = y * 4.606_f32 - x.cos();
        let mut acc = Accumulator1108::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.126_f32 + y.sin();
        let b = y * 2.289_f32 - x.cos();
        let mut acc = Accumulator1108::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.554_f32 + y.sin();
        let b = y * 8.393_f32 - x.cos();
        let mut acc = Accumulator1108::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.023_f32 + y.sin();
        let b = y * 2.861_f32 - x.cos();
        let mut acc = Accumulator1108::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.446_f32 + y.sin();
        let b = y * 8.423_f32 - x.cos();
        let mut acc = Accumulator1108::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.591_f32 + y.sin();
        let b = y * 1.567_f32 - x.cos();
        let mut acc = Accumulator1108::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.363_f32 + y.sin();
        let b = y * 2.403_f32 - x.cos();
        let mut acc = Accumulator1108::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.531_f32 + y.sin();
        let b = y * 7.603_f32 - x.cos();
        let mut acc = Accumulator1108::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.006_f32 + y.sin();
        let b = y * 6.103_f32 - x.cos();
        let mut acc = Accumulator1108::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.808_f32 + y.sin();
        let b = y * 6.884_f32 - x.cos();
        let mut acc = Accumulator1108::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.224_f32 + y.sin();
        let b = y * 0.872_f32 - x.cos();
        let mut acc = Accumulator1108::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.016_f32 + y.sin();
        let b = y * 9.086_f32 - x.cos();
        let mut acc = Accumulator1108::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.862_f32 + y.sin();
        let b = y * 5.047_f32 - x.cos();
        let mut acc = Accumulator1108::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.468_f32 + y.sin();
        let b = y * 8.22_f32 - x.cos();
        let mut acc = Accumulator1108::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.982_f32 + y.sin();
        let b = y * 2.601_f32 - x.cos();
        let mut acc = Accumulator1108::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.142_f32 + y.sin();
        let b = y * 2.155_f32 - x.cos();
        let mut acc = Accumulator1108::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.308_f32 + y.sin();
        let b = y * 1.789_f32 - x.cos();
        let mut acc = Accumulator1108::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1108(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_1108() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1108(total as u64) % 997) as f32;
        total
    }
}

pub mod m1109 {
    use super::*;

    pub struct Accumulator1109<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1109<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.501_f32 + y.sin();
        let b = y * 9.718_f32 - x.cos();
        let mut acc = Accumulator1109::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.524_f32 + y.sin();
        let b = y * 2.355_f32 - x.cos();
        let mut acc = Accumulator1109::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.15_f32 + y.sin();
        let b = y * 6.771_f32 - x.cos();
        let mut acc = Accumulator1109::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.267_f32 + y.sin();
        let b = y * 6.236_f32 - x.cos();
        let mut acc = Accumulator1109::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.331_f32 + y.sin();
        let b = y * 7.984_f32 - x.cos();
        let mut acc = Accumulator1109::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.201_f32 + y.sin();
        let b = y * 4.348_f32 - x.cos();
        let mut acc = Accumulator1109::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.083_f32 + y.sin();
        let b = y * 3.579_f32 - x.cos();
        let mut acc = Accumulator1109::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.001_f32 + y.sin();
        let b = y * 7.729_f32 - x.cos();
        let mut acc = Accumulator1109::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.445_f32 + y.sin();
        let b = y * 4.332_f32 - x.cos();
        let mut acc = Accumulator1109::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.378_f32 + y.sin();
        let b = y * 8.006_f32 - x.cos();
        let mut acc = Accumulator1109::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.56_f32 + y.sin();
        let b = y * 7.668_f32 - x.cos();
        let mut acc = Accumulator1109::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.909_f32 + y.sin();
        let b = y * 0.587_f32 - x.cos();
        let mut acc = Accumulator1109::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.139_f32 + y.sin();
        let b = y * 4.656_f32 - x.cos();
        let mut acc = Accumulator1109::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.976_f32 + y.sin();
        let b = y * 5.513_f32 - x.cos();
        let mut acc = Accumulator1109::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.428_f32 + y.sin();
        let b = y * 5.697_f32 - x.cos();
        let mut acc = Accumulator1109::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.994_f32 + y.sin();
        let b = y * 8.221_f32 - x.cos();
        let mut acc = Accumulator1109::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.229_f32 + y.sin();
        let b = y * 8.333_f32 - x.cos();
        let mut acc = Accumulator1109::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.514_f32 + y.sin();
        let b = y * 9.694_f32 - x.cos();
        let mut acc = Accumulator1109::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.166_f32 + y.sin();
        let b = y * 9.83_f32 - x.cos();
        let mut acc = Accumulator1109::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.34_f32 + y.sin();
        let b = y * 1.837_f32 - x.cos();
        let mut acc = Accumulator1109::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1109(seed: u64) -> u64 {
        let re = Regex::new(r"m1109-(\d+)").unwrap();
        let hay = format!("m1109-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_1109() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1109(total as u64) % 997) as f32;
        total
    }
}

pub mod m1110 {
    use super::*;

    pub struct Accumulator1110<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1110<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.483_f32 + y.sin();
        let b = y * 2.462_f32 - x.cos();
        let mut acc = Accumulator1110::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.565_f32 + y.sin();
        let b = y * 2.307_f32 - x.cos();
        let mut acc = Accumulator1110::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.508_f32 + y.sin();
        let b = y * 4.17_f32 - x.cos();
        let mut acc = Accumulator1110::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.049_f32 + y.sin();
        let b = y * 5.735_f32 - x.cos();
        let mut acc = Accumulator1110::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.372_f32 + y.sin();
        let b = y * 5.782_f32 - x.cos();
        let mut acc = Accumulator1110::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.507_f32 + y.sin();
        let b = y * 1.149_f32 - x.cos();
        let mut acc = Accumulator1110::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.727_f32 + y.sin();
        let b = y * 4.161_f32 - x.cos();
        let mut acc = Accumulator1110::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.206_f32 + y.sin();
        let b = y * 7.796_f32 - x.cos();
        let mut acc = Accumulator1110::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.426_f32 + y.sin();
        let b = y * 7.561_f32 - x.cos();
        let mut acc = Accumulator1110::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.385_f32 + y.sin();
        let b = y * 4.213_f32 - x.cos();
        let mut acc = Accumulator1110::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.56_f32 + y.sin();
        let b = y * 7.407_f32 - x.cos();
        let mut acc = Accumulator1110::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.444_f32 + y.sin();
        let b = y * 6.785_f32 - x.cos();
        let mut acc = Accumulator1110::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.302_f32 + y.sin();
        let b = y * 5.132_f32 - x.cos();
        let mut acc = Accumulator1110::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.695_f32 + y.sin();
        let b = y * 7.899_f32 - x.cos();
        let mut acc = Accumulator1110::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.487_f32 + y.sin();
        let b = y * 2.256_f32 - x.cos();
        let mut acc = Accumulator1110::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.857_f32 + y.sin();
        let b = y * 3.417_f32 - x.cos();
        let mut acc = Accumulator1110::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.771_f32 + y.sin();
        let b = y * 9.696_f32 - x.cos();
        let mut acc = Accumulator1110::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.193_f32 + y.sin();
        let b = y * 4.299_f32 - x.cos();
        let mut acc = Accumulator1110::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.232_f32 + y.sin();
        let b = y * 4.944_f32 - x.cos();
        let mut acc = Accumulator1110::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.197_f32 + y.sin();
        let b = y * 1.935_f32 - x.cos();
        let mut acc = Accumulator1110::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1110(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_1110() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1110(total as u64) % 997) as f32;
        total
    }
}

pub mod m1111 {
    use super::*;

    pub struct Accumulator1111<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1111<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.519_f32 + y.sin();
        let b = y * 0.492_f32 - x.cos();
        let mut acc = Accumulator1111::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.936_f32 + y.sin();
        let b = y * 7.461_f32 - x.cos();
        let mut acc = Accumulator1111::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.204_f32 + y.sin();
        let b = y * 5.408_f32 - x.cos();
        let mut acc = Accumulator1111::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.033_f32 + y.sin();
        let b = y * 6.656_f32 - x.cos();
        let mut acc = Accumulator1111::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.318_f32 + y.sin();
        let b = y * 5.168_f32 - x.cos();
        let mut acc = Accumulator1111::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.613_f32 + y.sin();
        let b = y * 7.279_f32 - x.cos();
        let mut acc = Accumulator1111::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.345_f32 + y.sin();
        let b = y * 8.968_f32 - x.cos();
        let mut acc = Accumulator1111::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.762_f32 + y.sin();
        let b = y * 4.474_f32 - x.cos();
        let mut acc = Accumulator1111::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.14_f32 + y.sin();
        let b = y * 5.104_f32 - x.cos();
        let mut acc = Accumulator1111::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.993_f32 + y.sin();
        let b = y * 0.297_f32 - x.cos();
        let mut acc = Accumulator1111::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.305_f32 + y.sin();
        let b = y * 3.359_f32 - x.cos();
        let mut acc = Accumulator1111::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.042_f32 + y.sin();
        let b = y * 2.176_f32 - x.cos();
        let mut acc = Accumulator1111::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.857_f32 + y.sin();
        let b = y * 0.377_f32 - x.cos();
        let mut acc = Accumulator1111::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.082_f32 + y.sin();
        let b = y * 3.289_f32 - x.cos();
        let mut acc = Accumulator1111::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.68_f32 + y.sin();
        let b = y * 3.378_f32 - x.cos();
        let mut acc = Accumulator1111::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.297_f32 + y.sin();
        let b = y * 1.27_f32 - x.cos();
        let mut acc = Accumulator1111::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.549_f32 + y.sin();
        let b = y * 8.102_f32 - x.cos();
        let mut acc = Accumulator1111::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.425_f32 + y.sin();
        let b = y * 9.675_f32 - x.cos();
        let mut acc = Accumulator1111::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.096_f32 + y.sin();
        let b = y * 1.592_f32 - x.cos();
        let mut acc = Accumulator1111::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.937_f32 + y.sin();
        let b = y * 9.528_f32 - x.cos();
        let mut acc = Accumulator1111::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1111(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(1111u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_1111() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1111(total as u64) % 997) as f32;
        total
    }
}

pub mod m1112 {
    use super::*;

    pub struct Accumulator1112<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1112<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.824_f32 + y.sin();
        let b = y * 7.416_f32 - x.cos();
        let mut acc = Accumulator1112::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.148_f32 + y.sin();
        let b = y * 1.57_f32 - x.cos();
        let mut acc = Accumulator1112::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.164_f32 + y.sin();
        let b = y * 8.105_f32 - x.cos();
        let mut acc = Accumulator1112::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.508_f32 + y.sin();
        let b = y * 2.704_f32 - x.cos();
        let mut acc = Accumulator1112::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.609_f32 + y.sin();
        let b = y * 9.115_f32 - x.cos();
        let mut acc = Accumulator1112::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.16_f32 + y.sin();
        let b = y * 3.17_f32 - x.cos();
        let mut acc = Accumulator1112::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.7_f32 + y.sin();
        let b = y * 3.599_f32 - x.cos();
        let mut acc = Accumulator1112::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.583_f32 + y.sin();
        let b = y * 1.629_f32 - x.cos();
        let mut acc = Accumulator1112::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.366_f32 + y.sin();
        let b = y * 4.329_f32 - x.cos();
        let mut acc = Accumulator1112::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.616_f32 + y.sin();
        let b = y * 0.167_f32 - x.cos();
        let mut acc = Accumulator1112::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.412_f32 + y.sin();
        let b = y * 3.1_f32 - x.cos();
        let mut acc = Accumulator1112::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.646_f32 + y.sin();
        let b = y * 7.531_f32 - x.cos();
        let mut acc = Accumulator1112::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.333_f32 + y.sin();
        let b = y * 2.209_f32 - x.cos();
        let mut acc = Accumulator1112::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.698_f32 + y.sin();
        let b = y * 1.792_f32 - x.cos();
        let mut acc = Accumulator1112::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.692_f32 + y.sin();
        let b = y * 6.076_f32 - x.cos();
        let mut acc = Accumulator1112::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.823_f32 + y.sin();
        let b = y * 3.18_f32 - x.cos();
        let mut acc = Accumulator1112::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.538_f32 + y.sin();
        let b = y * 5.917_f32 - x.cos();
        let mut acc = Accumulator1112::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.111_f32 + y.sin();
        let b = y * 6.098_f32 - x.cos();
        let mut acc = Accumulator1112::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.848_f32 + y.sin();
        let b = y * 0.963_f32 - x.cos();
        let mut acc = Accumulator1112::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.269_f32 + y.sin();
        let b = y * 3.035_f32 - x.cos();
        let mut acc = Accumulator1112::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1112(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_1112() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1112(total as u64) % 997) as f32;
        total
    }
}

pub mod m1113 {
    use super::*;

    pub struct Accumulator1113<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1113<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.189_f32 + y.sin();
        let b = y * 8.146_f32 - x.cos();
        let mut acc = Accumulator1113::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.446_f32 + y.sin();
        let b = y * 0.487_f32 - x.cos();
        let mut acc = Accumulator1113::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.721_f32 + y.sin();
        let b = y * 1.26_f32 - x.cos();
        let mut acc = Accumulator1113::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.489_f32 + y.sin();
        let b = y * 3.024_f32 - x.cos();
        let mut acc = Accumulator1113::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.233_f32 + y.sin();
        let b = y * 9.495_f32 - x.cos();
        let mut acc = Accumulator1113::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.367_f32 + y.sin();
        let b = y * 8.208_f32 - x.cos();
        let mut acc = Accumulator1113::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.123_f32 + y.sin();
        let b = y * 0.995_f32 - x.cos();
        let mut acc = Accumulator1113::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.245_f32 + y.sin();
        let b = y * 0.328_f32 - x.cos();
        let mut acc = Accumulator1113::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.698_f32 + y.sin();
        let b = y * 2.143_f32 - x.cos();
        let mut acc = Accumulator1113::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.514_f32 + y.sin();
        let b = y * 2.523_f32 - x.cos();
        let mut acc = Accumulator1113::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.36_f32 + y.sin();
        let b = y * 1.335_f32 - x.cos();
        let mut acc = Accumulator1113::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.384_f32 + y.sin();
        let b = y * 9.344_f32 - x.cos();
        let mut acc = Accumulator1113::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.161_f32 + y.sin();
        let b = y * 7.677_f32 - x.cos();
        let mut acc = Accumulator1113::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.264_f32 + y.sin();
        let b = y * 0.816_f32 - x.cos();
        let mut acc = Accumulator1113::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.805_f32 + y.sin();
        let b = y * 2.5_f32 - x.cos();
        let mut acc = Accumulator1113::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.303_f32 + y.sin();
        let b = y * 5.334_f32 - x.cos();
        let mut acc = Accumulator1113::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.184_f32 + y.sin();
        let b = y * 8.448_f32 - x.cos();
        let mut acc = Accumulator1113::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.197_f32 + y.sin();
        let b = y * 1.569_f32 - x.cos();
        let mut acc = Accumulator1113::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.793_f32 + y.sin();
        let b = y * 1.459_f32 - x.cos();
        let mut acc = Accumulator1113::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.006_f32 + y.sin();
        let b = y * 5.739_f32 - x.cos();
        let mut acc = Accumulator1113::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1113(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_1113() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1113(total as u64) % 997) as f32;
        total
    }
}

pub mod m1114 {
    use super::*;

    pub struct Accumulator1114<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1114<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.984_f32 + y.sin();
        let b = y * 6.498_f32 - x.cos();
        let mut acc = Accumulator1114::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.119_f32 + y.sin();
        let b = y * 3.155_f32 - x.cos();
        let mut acc = Accumulator1114::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.225_f32 + y.sin();
        let b = y * 1.338_f32 - x.cos();
        let mut acc = Accumulator1114::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.7_f32 + y.sin();
        let b = y * 1.984_f32 - x.cos();
        let mut acc = Accumulator1114::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.941_f32 + y.sin();
        let b = y * 6.145_f32 - x.cos();
        let mut acc = Accumulator1114::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.113_f32 + y.sin();
        let b = y * 4.142_f32 - x.cos();
        let mut acc = Accumulator1114::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.943_f32 + y.sin();
        let b = y * 3.704_f32 - x.cos();
        let mut acc = Accumulator1114::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.409_f32 + y.sin();
        let b = y * 6.595_f32 - x.cos();
        let mut acc = Accumulator1114::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.879_f32 + y.sin();
        let b = y * 8.588_f32 - x.cos();
        let mut acc = Accumulator1114::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.836_f32 + y.sin();
        let b = y * 5.224_f32 - x.cos();
        let mut acc = Accumulator1114::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.04_f32 + y.sin();
        let b = y * 9.205_f32 - x.cos();
        let mut acc = Accumulator1114::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.563_f32 + y.sin();
        let b = y * 9.099_f32 - x.cos();
        let mut acc = Accumulator1114::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.422_f32 + y.sin();
        let b = y * 8.875_f32 - x.cos();
        let mut acc = Accumulator1114::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.168_f32 + y.sin();
        let b = y * 6.652_f32 - x.cos();
        let mut acc = Accumulator1114::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.948_f32 + y.sin();
        let b = y * 2.63_f32 - x.cos();
        let mut acc = Accumulator1114::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.857_f32 + y.sin();
        let b = y * 2.651_f32 - x.cos();
        let mut acc = Accumulator1114::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.744_f32 + y.sin();
        let b = y * 8.842_f32 - x.cos();
        let mut acc = Accumulator1114::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.844_f32 + y.sin();
        let b = y * 2.426_f32 - x.cos();
        let mut acc = Accumulator1114::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.528_f32 + y.sin();
        let b = y * 9.486_f32 - x.cos();
        let mut acc = Accumulator1114::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.328_f32 + y.sin();
        let b = y * 7.329_f32 - x.cos();
        let mut acc = Accumulator1114::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1114(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m1114-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_1114() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1114(total as u64) % 997) as f32;
        total
    }
}

pub mod m1115 {
    use super::*;

    pub struct Accumulator1115<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1115<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.796_f32 + y.sin();
        let b = y * 2.576_f32 - x.cos();
        let mut acc = Accumulator1115::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.835_f32 + y.sin();
        let b = y * 5.291_f32 - x.cos();
        let mut acc = Accumulator1115::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.277_f32 + y.sin();
        let b = y * 5.769_f32 - x.cos();
        let mut acc = Accumulator1115::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.48_f32 + y.sin();
        let b = y * 2.13_f32 - x.cos();
        let mut acc = Accumulator1115::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.813_f32 + y.sin();
        let b = y * 7.995_f32 - x.cos();
        let mut acc = Accumulator1115::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.61_f32 + y.sin();
        let b = y * 2.697_f32 - x.cos();
        let mut acc = Accumulator1115::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.638_f32 + y.sin();
        let b = y * 8.767_f32 - x.cos();
        let mut acc = Accumulator1115::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.789_f32 + y.sin();
        let b = y * 5.951_f32 - x.cos();
        let mut acc = Accumulator1115::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.209_f32 + y.sin();
        let b = y * 6.753_f32 - x.cos();
        let mut acc = Accumulator1115::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.831_f32 + y.sin();
        let b = y * 8.295_f32 - x.cos();
        let mut acc = Accumulator1115::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.305_f32 + y.sin();
        let b = y * 6.739_f32 - x.cos();
        let mut acc = Accumulator1115::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.353_f32 + y.sin();
        let b = y * 2.272_f32 - x.cos();
        let mut acc = Accumulator1115::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.077_f32 + y.sin();
        let b = y * 6.546_f32 - x.cos();
        let mut acc = Accumulator1115::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.505_f32 + y.sin();
        let b = y * 0.803_f32 - x.cos();
        let mut acc = Accumulator1115::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.646_f32 + y.sin();
        let b = y * 6.4_f32 - x.cos();
        let mut acc = Accumulator1115::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.568_f32 + y.sin();
        let b = y * 7.754_f32 - x.cos();
        let mut acc = Accumulator1115::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.305_f32 + y.sin();
        let b = y * 0.915_f32 - x.cos();
        let mut acc = Accumulator1115::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.001_f32 + y.sin();
        let b = y * 2.249_f32 - x.cos();
        let mut acc = Accumulator1115::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.854_f32 + y.sin();
        let b = y * 9.411_f32 - x.cos();
        let mut acc = Accumulator1115::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.828_f32 + y.sin();
        let b = y * 6.353_f32 - x.cos();
        let mut acc = Accumulator1115::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1115(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_1115() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1115(total as u64) % 997) as f32;
        total
    }
}

pub mod m1116 {
    use super::*;

    pub struct Accumulator1116<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1116<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.072_f32 + y.sin();
        let b = y * 2.865_f32 - x.cos();
        let mut acc = Accumulator1116::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.487_f32 + y.sin();
        let b = y * 2.455_f32 - x.cos();
        let mut acc = Accumulator1116::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.552_f32 + y.sin();
        let b = y * 3.186_f32 - x.cos();
        let mut acc = Accumulator1116::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.235_f32 + y.sin();
        let b = y * 4.598_f32 - x.cos();
        let mut acc = Accumulator1116::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.026_f32 + y.sin();
        let b = y * 7.841_f32 - x.cos();
        let mut acc = Accumulator1116::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.749_f32 + y.sin();
        let b = y * 5.845_f32 - x.cos();
        let mut acc = Accumulator1116::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.578_f32 + y.sin();
        let b = y * 7.979_f32 - x.cos();
        let mut acc = Accumulator1116::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.103_f32 + y.sin();
        let b = y * 4.135_f32 - x.cos();
        let mut acc = Accumulator1116::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.254_f32 + y.sin();
        let b = y * 5.954_f32 - x.cos();
        let mut acc = Accumulator1116::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.715_f32 + y.sin();
        let b = y * 6.773_f32 - x.cos();
        let mut acc = Accumulator1116::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.879_f32 + y.sin();
        let b = y * 1.271_f32 - x.cos();
        let mut acc = Accumulator1116::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.815_f32 + y.sin();
        let b = y * 5.69_f32 - x.cos();
        let mut acc = Accumulator1116::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.302_f32 + y.sin();
        let b = y * 5.207_f32 - x.cos();
        let mut acc = Accumulator1116::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.863_f32 + y.sin();
        let b = y * 8.279_f32 - x.cos();
        let mut acc = Accumulator1116::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.843_f32 + y.sin();
        let b = y * 1.15_f32 - x.cos();
        let mut acc = Accumulator1116::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.835_f32 + y.sin();
        let b = y * 5.15_f32 - x.cos();
        let mut acc = Accumulator1116::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.416_f32 + y.sin();
        let b = y * 1.382_f32 - x.cos();
        let mut acc = Accumulator1116::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.777_f32 + y.sin();
        let b = y * 2.051_f32 - x.cos();
        let mut acc = Accumulator1116::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.912_f32 + y.sin();
        let b = y * 4.717_f32 - x.cos();
        let mut acc = Accumulator1116::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.573_f32 + y.sin();
        let b = y * 9.028_f32 - x.cos();
        let mut acc = Accumulator1116::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1116(seed: u64) -> u64 {
        let re = Regex::new(r"m1116-(\d+)").unwrap();
        let hay = format!("m1116-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_1116() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1116(total as u64) % 997) as f32;
        total
    }
}

pub mod m1117 {
    use super::*;

    pub struct Accumulator1117<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1117<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.382_f32 + y.sin();
        let b = y * 6.327_f32 - x.cos();
        let mut acc = Accumulator1117::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.958_f32 + y.sin();
        let b = y * 6.48_f32 - x.cos();
        let mut acc = Accumulator1117::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.856_f32 + y.sin();
        let b = y * 2.827_f32 - x.cos();
        let mut acc = Accumulator1117::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.836_f32 + y.sin();
        let b = y * 1.387_f32 - x.cos();
        let mut acc = Accumulator1117::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.805_f32 + y.sin();
        let b = y * 6.529_f32 - x.cos();
        let mut acc = Accumulator1117::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.546_f32 + y.sin();
        let b = y * 9.633_f32 - x.cos();
        let mut acc = Accumulator1117::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.049_f32 + y.sin();
        let b = y * 9.293_f32 - x.cos();
        let mut acc = Accumulator1117::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.621_f32 + y.sin();
        let b = y * 7.309_f32 - x.cos();
        let mut acc = Accumulator1117::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.927_f32 + y.sin();
        let b = y * 1.966_f32 - x.cos();
        let mut acc = Accumulator1117::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.957_f32 + y.sin();
        let b = y * 3.373_f32 - x.cos();
        let mut acc = Accumulator1117::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.001_f32 + y.sin();
        let b = y * 4.711_f32 - x.cos();
        let mut acc = Accumulator1117::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.879_f32 + y.sin();
        let b = y * 9.169_f32 - x.cos();
        let mut acc = Accumulator1117::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.355_f32 + y.sin();
        let b = y * 4.287_f32 - x.cos();
        let mut acc = Accumulator1117::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.877_f32 + y.sin();
        let b = y * 8.545_f32 - x.cos();
        let mut acc = Accumulator1117::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.485_f32 + y.sin();
        let b = y * 4.239_f32 - x.cos();
        let mut acc = Accumulator1117::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.285_f32 + y.sin();
        let b = y * 4.957_f32 - x.cos();
        let mut acc = Accumulator1117::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.073_f32 + y.sin();
        let b = y * 8.199_f32 - x.cos();
        let mut acc = Accumulator1117::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.241_f32 + y.sin();
        let b = y * 9.56_f32 - x.cos();
        let mut acc = Accumulator1117::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.38_f32 + y.sin();
        let b = y * 6.509_f32 - x.cos();
        let mut acc = Accumulator1117::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.483_f32 + y.sin();
        let b = y * 7.653_f32 - x.cos();
        let mut acc = Accumulator1117::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1117(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_1117() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1117(total as u64) % 997) as f32;
        total
    }
}

pub mod m1118 {
    use super::*;

    pub struct Accumulator1118<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1118<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.18_f32 + y.sin();
        let b = y * 6.569_f32 - x.cos();
        let mut acc = Accumulator1118::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.46_f32 + y.sin();
        let b = y * 5.844_f32 - x.cos();
        let mut acc = Accumulator1118::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.498_f32 + y.sin();
        let b = y * 8.971_f32 - x.cos();
        let mut acc = Accumulator1118::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.264_f32 + y.sin();
        let b = y * 4.684_f32 - x.cos();
        let mut acc = Accumulator1118::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.399_f32 + y.sin();
        let b = y * 4.102_f32 - x.cos();
        let mut acc = Accumulator1118::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.211_f32 + y.sin();
        let b = y * 0.928_f32 - x.cos();
        let mut acc = Accumulator1118::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.697_f32 + y.sin();
        let b = y * 7.64_f32 - x.cos();
        let mut acc = Accumulator1118::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.11_f32 + y.sin();
        let b = y * 3.734_f32 - x.cos();
        let mut acc = Accumulator1118::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.275_f32 + y.sin();
        let b = y * 0.282_f32 - x.cos();
        let mut acc = Accumulator1118::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.014_f32 + y.sin();
        let b = y * 8.373_f32 - x.cos();
        let mut acc = Accumulator1118::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.037_f32 + y.sin();
        let b = y * 7.416_f32 - x.cos();
        let mut acc = Accumulator1118::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.249_f32 + y.sin();
        let b = y * 0.652_f32 - x.cos();
        let mut acc = Accumulator1118::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.153_f32 + y.sin();
        let b = y * 4.526_f32 - x.cos();
        let mut acc = Accumulator1118::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.181_f32 + y.sin();
        let b = y * 0.942_f32 - x.cos();
        let mut acc = Accumulator1118::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.075_f32 + y.sin();
        let b = y * 1.08_f32 - x.cos();
        let mut acc = Accumulator1118::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.508_f32 + y.sin();
        let b = y * 7.049_f32 - x.cos();
        let mut acc = Accumulator1118::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.053_f32 + y.sin();
        let b = y * 2.647_f32 - x.cos();
        let mut acc = Accumulator1118::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.851_f32 + y.sin();
        let b = y * 4.512_f32 - x.cos();
        let mut acc = Accumulator1118::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.529_f32 + y.sin();
        let b = y * 1.721_f32 - x.cos();
        let mut acc = Accumulator1118::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.591_f32 + y.sin();
        let b = y * 1.015_f32 - x.cos();
        let mut acc = Accumulator1118::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1118(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(1118u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_1118() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1118(total as u64) % 997) as f32;
        total
    }
}

pub mod m1119 {
    use super::*;

    pub struct Accumulator1119<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1119<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.066_f32 + y.sin();
        let b = y * 7.679_f32 - x.cos();
        let mut acc = Accumulator1119::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.218_f32 + y.sin();
        let b = y * 1.975_f32 - x.cos();
        let mut acc = Accumulator1119::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.543_f32 + y.sin();
        let b = y * 2.43_f32 - x.cos();
        let mut acc = Accumulator1119::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.409_f32 + y.sin();
        let b = y * 9.786_f32 - x.cos();
        let mut acc = Accumulator1119::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.055_f32 + y.sin();
        let b = y * 5.82_f32 - x.cos();
        let mut acc = Accumulator1119::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.275_f32 + y.sin();
        let b = y * 7.264_f32 - x.cos();
        let mut acc = Accumulator1119::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.591_f32 + y.sin();
        let b = y * 0.398_f32 - x.cos();
        let mut acc = Accumulator1119::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.686_f32 + y.sin();
        let b = y * 1.307_f32 - x.cos();
        let mut acc = Accumulator1119::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.349_f32 + y.sin();
        let b = y * 3.793_f32 - x.cos();
        let mut acc = Accumulator1119::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.27_f32 + y.sin();
        let b = y * 0.202_f32 - x.cos();
        let mut acc = Accumulator1119::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.492_f32 + y.sin();
        let b = y * 5.686_f32 - x.cos();
        let mut acc = Accumulator1119::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.662_f32 + y.sin();
        let b = y * 2.268_f32 - x.cos();
        let mut acc = Accumulator1119::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.42_f32 + y.sin();
        let b = y * 3.962_f32 - x.cos();
        let mut acc = Accumulator1119::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.191_f32 + y.sin();
        let b = y * 2.702_f32 - x.cos();
        let mut acc = Accumulator1119::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.358_f32 + y.sin();
        let b = y * 5.332_f32 - x.cos();
        let mut acc = Accumulator1119::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.88_f32 + y.sin();
        let b = y * 7.677_f32 - x.cos();
        let mut acc = Accumulator1119::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.095_f32 + y.sin();
        let b = y * 8.243_f32 - x.cos();
        let mut acc = Accumulator1119::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.238_f32 + y.sin();
        let b = y * 9.821_f32 - x.cos();
        let mut acc = Accumulator1119::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.417_f32 + y.sin();
        let b = y * 4.509_f32 - x.cos();
        let mut acc = Accumulator1119::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.711_f32 + y.sin();
        let b = y * 3.765_f32 - x.cos();
        let mut acc = Accumulator1119::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1119(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_1119() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1119(total as u64) % 997) as f32;
        total
    }
}

pub mod m1120 {
    use super::*;

    pub struct Accumulator1120<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1120<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.562_f32 + y.sin();
        let b = y * 1.283_f32 - x.cos();
        let mut acc = Accumulator1120::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.03_f32 + y.sin();
        let b = y * 1.867_f32 - x.cos();
        let mut acc = Accumulator1120::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.966_f32 + y.sin();
        let b = y * 1.8_f32 - x.cos();
        let mut acc = Accumulator1120::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.222_f32 + y.sin();
        let b = y * 8.734_f32 - x.cos();
        let mut acc = Accumulator1120::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.78_f32 + y.sin();
        let b = y * 7.814_f32 - x.cos();
        let mut acc = Accumulator1120::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.101_f32 + y.sin();
        let b = y * 6.072_f32 - x.cos();
        let mut acc = Accumulator1120::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.771_f32 + y.sin();
        let b = y * 3.621_f32 - x.cos();
        let mut acc = Accumulator1120::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.42_f32 + y.sin();
        let b = y * 1.405_f32 - x.cos();
        let mut acc = Accumulator1120::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.914_f32 + y.sin();
        let b = y * 2.278_f32 - x.cos();
        let mut acc = Accumulator1120::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.851_f32 + y.sin();
        let b = y * 7.617_f32 - x.cos();
        let mut acc = Accumulator1120::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.23_f32 + y.sin();
        let b = y * 4.516_f32 - x.cos();
        let mut acc = Accumulator1120::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.175_f32 + y.sin();
        let b = y * 8.388_f32 - x.cos();
        let mut acc = Accumulator1120::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.253_f32 + y.sin();
        let b = y * 6.819_f32 - x.cos();
        let mut acc = Accumulator1120::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.514_f32 + y.sin();
        let b = y * 8.842_f32 - x.cos();
        let mut acc = Accumulator1120::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.282_f32 + y.sin();
        let b = y * 0.817_f32 - x.cos();
        let mut acc = Accumulator1120::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.3_f32 + y.sin();
        let b = y * 4.591_f32 - x.cos();
        let mut acc = Accumulator1120::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.746_f32 + y.sin();
        let b = y * 4.531_f32 - x.cos();
        let mut acc = Accumulator1120::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.11_f32 + y.sin();
        let b = y * 6.187_f32 - x.cos();
        let mut acc = Accumulator1120::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.256_f32 + y.sin();
        let b = y * 6.063_f32 - x.cos();
        let mut acc = Accumulator1120::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.579_f32 + y.sin();
        let b = y * 5.193_f32 - x.cos();
        let mut acc = Accumulator1120::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1120(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_1120() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1120(total as u64) % 997) as f32;
        total
    }
}

pub mod m1121 {
    use super::*;

    pub struct Accumulator1121<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1121<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.384_f32 + y.sin();
        let b = y * 9.561_f32 - x.cos();
        let mut acc = Accumulator1121::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.607_f32 + y.sin();
        let b = y * 0.304_f32 - x.cos();
        let mut acc = Accumulator1121::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.191_f32 + y.sin();
        let b = y * 2.091_f32 - x.cos();
        let mut acc = Accumulator1121::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.059_f32 + y.sin();
        let b = y * 0.57_f32 - x.cos();
        let mut acc = Accumulator1121::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.627_f32 + y.sin();
        let b = y * 3.782_f32 - x.cos();
        let mut acc = Accumulator1121::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.45_f32 + y.sin();
        let b = y * 2.093_f32 - x.cos();
        let mut acc = Accumulator1121::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.116_f32 + y.sin();
        let b = y * 5.382_f32 - x.cos();
        let mut acc = Accumulator1121::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.543_f32 + y.sin();
        let b = y * 5.858_f32 - x.cos();
        let mut acc = Accumulator1121::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.431_f32 + y.sin();
        let b = y * 4.182_f32 - x.cos();
        let mut acc = Accumulator1121::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.995_f32 + y.sin();
        let b = y * 3.701_f32 - x.cos();
        let mut acc = Accumulator1121::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.811_f32 + y.sin();
        let b = y * 4.69_f32 - x.cos();
        let mut acc = Accumulator1121::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.775_f32 + y.sin();
        let b = y * 2.651_f32 - x.cos();
        let mut acc = Accumulator1121::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.995_f32 + y.sin();
        let b = y * 0.683_f32 - x.cos();
        let mut acc = Accumulator1121::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.205_f32 + y.sin();
        let b = y * 9.271_f32 - x.cos();
        let mut acc = Accumulator1121::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.605_f32 + y.sin();
        let b = y * 4.458_f32 - x.cos();
        let mut acc = Accumulator1121::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.331_f32 + y.sin();
        let b = y * 8.955_f32 - x.cos();
        let mut acc = Accumulator1121::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.542_f32 + y.sin();
        let b = y * 0.836_f32 - x.cos();
        let mut acc = Accumulator1121::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.25_f32 + y.sin();
        let b = y * 7.232_f32 - x.cos();
        let mut acc = Accumulator1121::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.512_f32 + y.sin();
        let b = y * 6.36_f32 - x.cos();
        let mut acc = Accumulator1121::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.783_f32 + y.sin();
        let b = y * 9.873_f32 - x.cos();
        let mut acc = Accumulator1121::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1121(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m1121-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_1121() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1121(total as u64) % 997) as f32;
        total
    }
}

pub mod m1122 {
    use super::*;

    pub struct Accumulator1122<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1122<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.876_f32 + y.sin();
        let b = y * 2.702_f32 - x.cos();
        let mut acc = Accumulator1122::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.906_f32 + y.sin();
        let b = y * 7.035_f32 - x.cos();
        let mut acc = Accumulator1122::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.028_f32 + y.sin();
        let b = y * 2.169_f32 - x.cos();
        let mut acc = Accumulator1122::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.593_f32 + y.sin();
        let b = y * 5.793_f32 - x.cos();
        let mut acc = Accumulator1122::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.195_f32 + y.sin();
        let b = y * 4.474_f32 - x.cos();
        let mut acc = Accumulator1122::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.789_f32 + y.sin();
        let b = y * 3.226_f32 - x.cos();
        let mut acc = Accumulator1122::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.485_f32 + y.sin();
        let b = y * 6.875_f32 - x.cos();
        let mut acc = Accumulator1122::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.891_f32 + y.sin();
        let b = y * 1.928_f32 - x.cos();
        let mut acc = Accumulator1122::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.048_f32 + y.sin();
        let b = y * 7.824_f32 - x.cos();
        let mut acc = Accumulator1122::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.865_f32 + y.sin();
        let b = y * 9.539_f32 - x.cos();
        let mut acc = Accumulator1122::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.243_f32 + y.sin();
        let b = y * 7.551_f32 - x.cos();
        let mut acc = Accumulator1122::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.245_f32 + y.sin();
        let b = y * 6.333_f32 - x.cos();
        let mut acc = Accumulator1122::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.974_f32 + y.sin();
        let b = y * 9.409_f32 - x.cos();
        let mut acc = Accumulator1122::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.447_f32 + y.sin();
        let b = y * 5.333_f32 - x.cos();
        let mut acc = Accumulator1122::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.309_f32 + y.sin();
        let b = y * 7.924_f32 - x.cos();
        let mut acc = Accumulator1122::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.218_f32 + y.sin();
        let b = y * 7.04_f32 - x.cos();
        let mut acc = Accumulator1122::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.477_f32 + y.sin();
        let b = y * 8.325_f32 - x.cos();
        let mut acc = Accumulator1122::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.467_f32 + y.sin();
        let b = y * 4.428_f32 - x.cos();
        let mut acc = Accumulator1122::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.771_f32 + y.sin();
        let b = y * 2.884_f32 - x.cos();
        let mut acc = Accumulator1122::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.597_f32 + y.sin();
        let b = y * 7.646_f32 - x.cos();
        let mut acc = Accumulator1122::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1122(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_1122() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1122(total as u64) % 997) as f32;
        total
    }
}

pub mod m1123 {
    use super::*;

    pub struct Accumulator1123<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1123<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.612_f32 + y.sin();
        let b = y * 8.219_f32 - x.cos();
        let mut acc = Accumulator1123::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.791_f32 + y.sin();
        let b = y * 5.507_f32 - x.cos();
        let mut acc = Accumulator1123::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.765_f32 + y.sin();
        let b = y * 8.122_f32 - x.cos();
        let mut acc = Accumulator1123::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.727_f32 + y.sin();
        let b = y * 3.594_f32 - x.cos();
        let mut acc = Accumulator1123::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.004_f32 + y.sin();
        let b = y * 1.232_f32 - x.cos();
        let mut acc = Accumulator1123::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.127_f32 + y.sin();
        let b = y * 0.749_f32 - x.cos();
        let mut acc = Accumulator1123::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.574_f32 + y.sin();
        let b = y * 7.762_f32 - x.cos();
        let mut acc = Accumulator1123::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.818_f32 + y.sin();
        let b = y * 8.97_f32 - x.cos();
        let mut acc = Accumulator1123::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.122_f32 + y.sin();
        let b = y * 7.249_f32 - x.cos();
        let mut acc = Accumulator1123::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.493_f32 + y.sin();
        let b = y * 2.245_f32 - x.cos();
        let mut acc = Accumulator1123::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.484_f32 + y.sin();
        let b = y * 2.699_f32 - x.cos();
        let mut acc = Accumulator1123::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.358_f32 + y.sin();
        let b = y * 2.561_f32 - x.cos();
        let mut acc = Accumulator1123::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.181_f32 + y.sin();
        let b = y * 6.581_f32 - x.cos();
        let mut acc = Accumulator1123::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.292_f32 + y.sin();
        let b = y * 2.563_f32 - x.cos();
        let mut acc = Accumulator1123::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.7_f32 + y.sin();
        let b = y * 2.435_f32 - x.cos();
        let mut acc = Accumulator1123::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.6_f32 + y.sin();
        let b = y * 3.709_f32 - x.cos();
        let mut acc = Accumulator1123::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.805_f32 + y.sin();
        let b = y * 9.316_f32 - x.cos();
        let mut acc = Accumulator1123::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.924_f32 + y.sin();
        let b = y * 5.125_f32 - x.cos();
        let mut acc = Accumulator1123::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.847_f32 + y.sin();
        let b = y * 4.12_f32 - x.cos();
        let mut acc = Accumulator1123::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.924_f32 + y.sin();
        let b = y * 2.972_f32 - x.cos();
        let mut acc = Accumulator1123::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1123(seed: u64) -> u64 {
        let re = Regex::new(r"m1123-(\d+)").unwrap();
        let hay = format!("m1123-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_1123() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1123(total as u64) % 997) as f32;
        total
    }
}

pub mod m1124 {
    use super::*;

    pub struct Accumulator1124<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1124<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.827_f32 + y.sin();
        let b = y * 4.116_f32 - x.cos();
        let mut acc = Accumulator1124::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.219_f32 + y.sin();
        let b = y * 0.956_f32 - x.cos();
        let mut acc = Accumulator1124::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.112_f32 + y.sin();
        let b = y * 8.643_f32 - x.cos();
        let mut acc = Accumulator1124::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.816_f32 + y.sin();
        let b = y * 6.155_f32 - x.cos();
        let mut acc = Accumulator1124::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.608_f32 + y.sin();
        let b = y * 2.173_f32 - x.cos();
        let mut acc = Accumulator1124::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.501_f32 + y.sin();
        let b = y * 6.004_f32 - x.cos();
        let mut acc = Accumulator1124::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.019_f32 + y.sin();
        let b = y * 8.276_f32 - x.cos();
        let mut acc = Accumulator1124::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.437_f32 + y.sin();
        let b = y * 7.464_f32 - x.cos();
        let mut acc = Accumulator1124::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.824_f32 + y.sin();
        let b = y * 2.874_f32 - x.cos();
        let mut acc = Accumulator1124::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.154_f32 + y.sin();
        let b = y * 2.684_f32 - x.cos();
        let mut acc = Accumulator1124::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.875_f32 + y.sin();
        let b = y * 2.21_f32 - x.cos();
        let mut acc = Accumulator1124::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.105_f32 + y.sin();
        let b = y * 5.329_f32 - x.cos();
        let mut acc = Accumulator1124::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.267_f32 + y.sin();
        let b = y * 8.01_f32 - x.cos();
        let mut acc = Accumulator1124::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.989_f32 + y.sin();
        let b = y * 9.27_f32 - x.cos();
        let mut acc = Accumulator1124::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.075_f32 + y.sin();
        let b = y * 8.933_f32 - x.cos();
        let mut acc = Accumulator1124::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.675_f32 + y.sin();
        let b = y * 6.335_f32 - x.cos();
        let mut acc = Accumulator1124::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.173_f32 + y.sin();
        let b = y * 1.175_f32 - x.cos();
        let mut acc = Accumulator1124::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.095_f32 + y.sin();
        let b = y * 8.052_f32 - x.cos();
        let mut acc = Accumulator1124::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.038_f32 + y.sin();
        let b = y * 3.82_f32 - x.cos();
        let mut acc = Accumulator1124::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.178_f32 + y.sin();
        let b = y * 5.766_f32 - x.cos();
        let mut acc = Accumulator1124::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1124(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_1124() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1124(total as u64) % 997) as f32;
        total
    }
}

pub mod m1125 {
    use super::*;

    pub struct Accumulator1125<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1125<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.829_f32 + y.sin();
        let b = y * 2.7_f32 - x.cos();
        let mut acc = Accumulator1125::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.011_f32 + y.sin();
        let b = y * 6.218_f32 - x.cos();
        let mut acc = Accumulator1125::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.778_f32 + y.sin();
        let b = y * 2.086_f32 - x.cos();
        let mut acc = Accumulator1125::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.516_f32 + y.sin();
        let b = y * 3.75_f32 - x.cos();
        let mut acc = Accumulator1125::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.181_f32 + y.sin();
        let b = y * 5.223_f32 - x.cos();
        let mut acc = Accumulator1125::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.749_f32 + y.sin();
        let b = y * 7.984_f32 - x.cos();
        let mut acc = Accumulator1125::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.751_f32 + y.sin();
        let b = y * 5.43_f32 - x.cos();
        let mut acc = Accumulator1125::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.878_f32 + y.sin();
        let b = y * 4.044_f32 - x.cos();
        let mut acc = Accumulator1125::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.455_f32 + y.sin();
        let b = y * 2.336_f32 - x.cos();
        let mut acc = Accumulator1125::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.3_f32 + y.sin();
        let b = y * 8.384_f32 - x.cos();
        let mut acc = Accumulator1125::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.273_f32 + y.sin();
        let b = y * 4.218_f32 - x.cos();
        let mut acc = Accumulator1125::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.352_f32 + y.sin();
        let b = y * 6.914_f32 - x.cos();
        let mut acc = Accumulator1125::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.245_f32 + y.sin();
        let b = y * 0.887_f32 - x.cos();
        let mut acc = Accumulator1125::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.592_f32 + y.sin();
        let b = y * 7.292_f32 - x.cos();
        let mut acc = Accumulator1125::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.526_f32 + y.sin();
        let b = y * 5.44_f32 - x.cos();
        let mut acc = Accumulator1125::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.994_f32 + y.sin();
        let b = y * 9.107_f32 - x.cos();
        let mut acc = Accumulator1125::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.213_f32 + y.sin();
        let b = y * 1.08_f32 - x.cos();
        let mut acc = Accumulator1125::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.435_f32 + y.sin();
        let b = y * 3.737_f32 - x.cos();
        let mut acc = Accumulator1125::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.249_f32 + y.sin();
        let b = y * 5.185_f32 - x.cos();
        let mut acc = Accumulator1125::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.898_f32 + y.sin();
        let b = y * 9.29_f32 - x.cos();
        let mut acc = Accumulator1125::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1125(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(1125u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_1125() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1125(total as u64) % 997) as f32;
        total
    }
}

pub mod m1126 {
    use super::*;

    pub struct Accumulator1126<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1126<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.67_f32 + y.sin();
        let b = y * 1.08_f32 - x.cos();
        let mut acc = Accumulator1126::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.053_f32 + y.sin();
        let b = y * 9.095_f32 - x.cos();
        let mut acc = Accumulator1126::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.153_f32 + y.sin();
        let b = y * 2.696_f32 - x.cos();
        let mut acc = Accumulator1126::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.447_f32 + y.sin();
        let b = y * 0.619_f32 - x.cos();
        let mut acc = Accumulator1126::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.2_f32 + y.sin();
        let b = y * 6.737_f32 - x.cos();
        let mut acc = Accumulator1126::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.24_f32 + y.sin();
        let b = y * 2.426_f32 - x.cos();
        let mut acc = Accumulator1126::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.398_f32 + y.sin();
        let b = y * 9.446_f32 - x.cos();
        let mut acc = Accumulator1126::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.112_f32 + y.sin();
        let b = y * 8.986_f32 - x.cos();
        let mut acc = Accumulator1126::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.979_f32 + y.sin();
        let b = y * 9.66_f32 - x.cos();
        let mut acc = Accumulator1126::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.65_f32 + y.sin();
        let b = y * 9.871_f32 - x.cos();
        let mut acc = Accumulator1126::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.834_f32 + y.sin();
        let b = y * 5.163_f32 - x.cos();
        let mut acc = Accumulator1126::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.446_f32 + y.sin();
        let b = y * 7.966_f32 - x.cos();
        let mut acc = Accumulator1126::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.21_f32 + y.sin();
        let b = y * 0.991_f32 - x.cos();
        let mut acc = Accumulator1126::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.532_f32 + y.sin();
        let b = y * 2.364_f32 - x.cos();
        let mut acc = Accumulator1126::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.706_f32 + y.sin();
        let b = y * 2.888_f32 - x.cos();
        let mut acc = Accumulator1126::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.079_f32 + y.sin();
        let b = y * 6.259_f32 - x.cos();
        let mut acc = Accumulator1126::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.428_f32 + y.sin();
        let b = y * 7.765_f32 - x.cos();
        let mut acc = Accumulator1126::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.217_f32 + y.sin();
        let b = y * 9.363_f32 - x.cos();
        let mut acc = Accumulator1126::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.252_f32 + y.sin();
        let b = y * 8.385_f32 - x.cos();
        let mut acc = Accumulator1126::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.03_f32 + y.sin();
        let b = y * 9.245_f32 - x.cos();
        let mut acc = Accumulator1126::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1126(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_1126() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1126(total as u64) % 997) as f32;
        total
    }
}

pub mod m1127 {
    use super::*;

    pub struct Accumulator1127<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1127<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.945_f32 + y.sin();
        let b = y * 0.426_f32 - x.cos();
        let mut acc = Accumulator1127::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.279_f32 + y.sin();
        let b = y * 1.45_f32 - x.cos();
        let mut acc = Accumulator1127::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.982_f32 + y.sin();
        let b = y * 9.664_f32 - x.cos();
        let mut acc = Accumulator1127::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.402_f32 + y.sin();
        let b = y * 3.697_f32 - x.cos();
        let mut acc = Accumulator1127::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.893_f32 + y.sin();
        let b = y * 1.095_f32 - x.cos();
        let mut acc = Accumulator1127::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.935_f32 + y.sin();
        let b = y * 8.687_f32 - x.cos();
        let mut acc = Accumulator1127::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.821_f32 + y.sin();
        let b = y * 9.314_f32 - x.cos();
        let mut acc = Accumulator1127::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.245_f32 + y.sin();
        let b = y * 4.329_f32 - x.cos();
        let mut acc = Accumulator1127::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.304_f32 + y.sin();
        let b = y * 4.217_f32 - x.cos();
        let mut acc = Accumulator1127::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.145_f32 + y.sin();
        let b = y * 7.065_f32 - x.cos();
        let mut acc = Accumulator1127::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.86_f32 + y.sin();
        let b = y * 0.503_f32 - x.cos();
        let mut acc = Accumulator1127::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.564_f32 + y.sin();
        let b = y * 3.412_f32 - x.cos();
        let mut acc = Accumulator1127::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.886_f32 + y.sin();
        let b = y * 7.565_f32 - x.cos();
        let mut acc = Accumulator1127::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.851_f32 + y.sin();
        let b = y * 9.218_f32 - x.cos();
        let mut acc = Accumulator1127::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.512_f32 + y.sin();
        let b = y * 6.799_f32 - x.cos();
        let mut acc = Accumulator1127::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.449_f32 + y.sin();
        let b = y * 6.058_f32 - x.cos();
        let mut acc = Accumulator1127::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.443_f32 + y.sin();
        let b = y * 0.236_f32 - x.cos();
        let mut acc = Accumulator1127::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.371_f32 + y.sin();
        let b = y * 0.771_f32 - x.cos();
        let mut acc = Accumulator1127::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.278_f32 + y.sin();
        let b = y * 3.525_f32 - x.cos();
        let mut acc = Accumulator1127::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.4_f32 + y.sin();
        let b = y * 6.456_f32 - x.cos();
        let mut acc = Accumulator1127::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1127(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_1127() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1127(total as u64) % 997) as f32;
        total
    }
}

pub mod m1128 {
    use super::*;

    pub struct Accumulator1128<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1128<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.731_f32 + y.sin();
        let b = y * 9.04_f32 - x.cos();
        let mut acc = Accumulator1128::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.555_f32 + y.sin();
        let b = y * 5.155_f32 - x.cos();
        let mut acc = Accumulator1128::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.463_f32 + y.sin();
        let b = y * 8.594_f32 - x.cos();
        let mut acc = Accumulator1128::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.656_f32 + y.sin();
        let b = y * 9.401_f32 - x.cos();
        let mut acc = Accumulator1128::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.628_f32 + y.sin();
        let b = y * 7.083_f32 - x.cos();
        let mut acc = Accumulator1128::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.194_f32 + y.sin();
        let b = y * 7.092_f32 - x.cos();
        let mut acc = Accumulator1128::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.376_f32 + y.sin();
        let b = y * 8.404_f32 - x.cos();
        let mut acc = Accumulator1128::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.542_f32 + y.sin();
        let b = y * 3.039_f32 - x.cos();
        let mut acc = Accumulator1128::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.823_f32 + y.sin();
        let b = y * 0.558_f32 - x.cos();
        let mut acc = Accumulator1128::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.11_f32 + y.sin();
        let b = y * 2.218_f32 - x.cos();
        let mut acc = Accumulator1128::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.24_f32 + y.sin();
        let b = y * 7.204_f32 - x.cos();
        let mut acc = Accumulator1128::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.645_f32 + y.sin();
        let b = y * 6.706_f32 - x.cos();
        let mut acc = Accumulator1128::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.439_f32 + y.sin();
        let b = y * 8.518_f32 - x.cos();
        let mut acc = Accumulator1128::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.156_f32 + y.sin();
        let b = y * 9.038_f32 - x.cos();
        let mut acc = Accumulator1128::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.207_f32 + y.sin();
        let b = y * 2.681_f32 - x.cos();
        let mut acc = Accumulator1128::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.404_f32 + y.sin();
        let b = y * 3.339_f32 - x.cos();
        let mut acc = Accumulator1128::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.964_f32 + y.sin();
        let b = y * 4.677_f32 - x.cos();
        let mut acc = Accumulator1128::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.787_f32 + y.sin();
        let b = y * 0.543_f32 - x.cos();
        let mut acc = Accumulator1128::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.823_f32 + y.sin();
        let b = y * 5.668_f32 - x.cos();
        let mut acc = Accumulator1128::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.949_f32 + y.sin();
        let b = y * 1.171_f32 - x.cos();
        let mut acc = Accumulator1128::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1128(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m1128-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_1128() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1128(total as u64) % 997) as f32;
        total
    }
}

pub mod m1129 {
    use super::*;

    pub struct Accumulator1129<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1129<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.217_f32 + y.sin();
        let b = y * 6.572_f32 - x.cos();
        let mut acc = Accumulator1129::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.541_f32 + y.sin();
        let b = y * 6.3_f32 - x.cos();
        let mut acc = Accumulator1129::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.942_f32 + y.sin();
        let b = y * 4.712_f32 - x.cos();
        let mut acc = Accumulator1129::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.257_f32 + y.sin();
        let b = y * 5.717_f32 - x.cos();
        let mut acc = Accumulator1129::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.811_f32 + y.sin();
        let b = y * 2.064_f32 - x.cos();
        let mut acc = Accumulator1129::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.681_f32 + y.sin();
        let b = y * 8.087_f32 - x.cos();
        let mut acc = Accumulator1129::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.544_f32 + y.sin();
        let b = y * 7.244_f32 - x.cos();
        let mut acc = Accumulator1129::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.501_f32 + y.sin();
        let b = y * 9.683_f32 - x.cos();
        let mut acc = Accumulator1129::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.103_f32 + y.sin();
        let b = y * 7.189_f32 - x.cos();
        let mut acc = Accumulator1129::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.483_f32 + y.sin();
        let b = y * 6.523_f32 - x.cos();
        let mut acc = Accumulator1129::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.176_f32 + y.sin();
        let b = y * 7.827_f32 - x.cos();
        let mut acc = Accumulator1129::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.737_f32 + y.sin();
        let b = y * 2.994_f32 - x.cos();
        let mut acc = Accumulator1129::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.184_f32 + y.sin();
        let b = y * 7.219_f32 - x.cos();
        let mut acc = Accumulator1129::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.369_f32 + y.sin();
        let b = y * 9.499_f32 - x.cos();
        let mut acc = Accumulator1129::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.151_f32 + y.sin();
        let b = y * 4.573_f32 - x.cos();
        let mut acc = Accumulator1129::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.172_f32 + y.sin();
        let b = y * 5.446_f32 - x.cos();
        let mut acc = Accumulator1129::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.271_f32 + y.sin();
        let b = y * 0.781_f32 - x.cos();
        let mut acc = Accumulator1129::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.791_f32 + y.sin();
        let b = y * 5.305_f32 - x.cos();
        let mut acc = Accumulator1129::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.356_f32 + y.sin();
        let b = y * 0.532_f32 - x.cos();
        let mut acc = Accumulator1129::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.586_f32 + y.sin();
        let b = y * 6.058_f32 - x.cos();
        let mut acc = Accumulator1129::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1129(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_1129() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1129(total as u64) % 997) as f32;
        total
    }
}

pub mod m1130 {
    use super::*;

    pub struct Accumulator1130<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1130<T> {
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
        let b = y * 7.187_f32 - x.cos();
        let mut acc = Accumulator1130::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.881_f32 + y.sin();
        let b = y * 2.172_f32 - x.cos();
        let mut acc = Accumulator1130::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.283_f32 + y.sin();
        let b = y * 0.521_f32 - x.cos();
        let mut acc = Accumulator1130::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.855_f32 + y.sin();
        let b = y * 0.504_f32 - x.cos();
        let mut acc = Accumulator1130::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.164_f32 + y.sin();
        let b = y * 4.525_f32 - x.cos();
        let mut acc = Accumulator1130::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.859_f32 + y.sin();
        let b = y * 2.547_f32 - x.cos();
        let mut acc = Accumulator1130::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.313_f32 + y.sin();
        let b = y * 6.004_f32 - x.cos();
        let mut acc = Accumulator1130::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.408_f32 + y.sin();
        let b = y * 8.98_f32 - x.cos();
        let mut acc = Accumulator1130::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.987_f32 + y.sin();
        let b = y * 1.061_f32 - x.cos();
        let mut acc = Accumulator1130::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.015_f32 + y.sin();
        let b = y * 8.297_f32 - x.cos();
        let mut acc = Accumulator1130::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.022_f32 + y.sin();
        let b = y * 7.811_f32 - x.cos();
        let mut acc = Accumulator1130::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.918_f32 + y.sin();
        let b = y * 1.051_f32 - x.cos();
        let mut acc = Accumulator1130::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.966_f32 + y.sin();
        let b = y * 6.7_f32 - x.cos();
        let mut acc = Accumulator1130::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.495_f32 + y.sin();
        let b = y * 2.364_f32 - x.cos();
        let mut acc = Accumulator1130::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.692_f32 + y.sin();
        let b = y * 9.35_f32 - x.cos();
        let mut acc = Accumulator1130::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.02_f32 + y.sin();
        let b = y * 5.428_f32 - x.cos();
        let mut acc = Accumulator1130::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.219_f32 + y.sin();
        let b = y * 0.62_f32 - x.cos();
        let mut acc = Accumulator1130::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.076_f32 + y.sin();
        let b = y * 6.219_f32 - x.cos();
        let mut acc = Accumulator1130::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.115_f32 + y.sin();
        let b = y * 9.739_f32 - x.cos();
        let mut acc = Accumulator1130::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.879_f32 + y.sin();
        let b = y * 9.805_f32 - x.cos();
        let mut acc = Accumulator1130::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1130(seed: u64) -> u64 {
        let re = Regex::new(r"m1130-(\d+)").unwrap();
        let hay = format!("m1130-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_1130() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1130(total as u64) % 997) as f32;
        total
    }
}

pub mod m1131 {
    use super::*;

    pub struct Accumulator1131<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1131<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.505_f32 + y.sin();
        let b = y * 1.735_f32 - x.cos();
        let mut acc = Accumulator1131::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.546_f32 + y.sin();
        let b = y * 9.532_f32 - x.cos();
        let mut acc = Accumulator1131::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.228_f32 + y.sin();
        let b = y * 1.111_f32 - x.cos();
        let mut acc = Accumulator1131::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.845_f32 + y.sin();
        let b = y * 9.018_f32 - x.cos();
        let mut acc = Accumulator1131::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.246_f32 + y.sin();
        let b = y * 9.704_f32 - x.cos();
        let mut acc = Accumulator1131::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.274_f32 + y.sin();
        let b = y * 6.83_f32 - x.cos();
        let mut acc = Accumulator1131::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.826_f32 + y.sin();
        let b = y * 7.061_f32 - x.cos();
        let mut acc = Accumulator1131::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.807_f32 + y.sin();
        let b = y * 9.544_f32 - x.cos();
        let mut acc = Accumulator1131::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.062_f32 + y.sin();
        let b = y * 3.647_f32 - x.cos();
        let mut acc = Accumulator1131::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.152_f32 + y.sin();
        let b = y * 3.199_f32 - x.cos();
        let mut acc = Accumulator1131::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.083_f32 + y.sin();
        let b = y * 8.888_f32 - x.cos();
        let mut acc = Accumulator1131::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.059_f32 + y.sin();
        let b = y * 3.184_f32 - x.cos();
        let mut acc = Accumulator1131::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.473_f32 + y.sin();
        let b = y * 8.944_f32 - x.cos();
        let mut acc = Accumulator1131::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.258_f32 + y.sin();
        let b = y * 7.248_f32 - x.cos();
        let mut acc = Accumulator1131::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.503_f32 + y.sin();
        let b = y * 4.081_f32 - x.cos();
        let mut acc = Accumulator1131::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.031_f32 + y.sin();
        let b = y * 5.842_f32 - x.cos();
        let mut acc = Accumulator1131::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.674_f32 + y.sin();
        let b = y * 2.211_f32 - x.cos();
        let mut acc = Accumulator1131::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.194_f32 + y.sin();
        let b = y * 4.592_f32 - x.cos();
        let mut acc = Accumulator1131::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.564_f32 + y.sin();
        let b = y * 5.56_f32 - x.cos();
        let mut acc = Accumulator1131::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.983_f32 + y.sin();
        let b = y * 2.01_f32 - x.cos();
        let mut acc = Accumulator1131::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1131(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_1131() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1131(total as u64) % 997) as f32;
        total
    }
}

pub mod m1132 {
    use super::*;

    pub struct Accumulator1132<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1132<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.348_f32 + y.sin();
        let b = y * 7.604_f32 - x.cos();
        let mut acc = Accumulator1132::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.971_f32 + y.sin();
        let b = y * 9.796_f32 - x.cos();
        let mut acc = Accumulator1132::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.889_f32 + y.sin();
        let b = y * 5.379_f32 - x.cos();
        let mut acc = Accumulator1132::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.847_f32 + y.sin();
        let b = y * 1.153_f32 - x.cos();
        let mut acc = Accumulator1132::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.423_f32 + y.sin();
        let b = y * 8.038_f32 - x.cos();
        let mut acc = Accumulator1132::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.601_f32 + y.sin();
        let b = y * 3.05_f32 - x.cos();
        let mut acc = Accumulator1132::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.401_f32 + y.sin();
        let b = y * 7.464_f32 - x.cos();
        let mut acc = Accumulator1132::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.928_f32 + y.sin();
        let b = y * 9.729_f32 - x.cos();
        let mut acc = Accumulator1132::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.089_f32 + y.sin();
        let b = y * 9.222_f32 - x.cos();
        let mut acc = Accumulator1132::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.813_f32 + y.sin();
        let b = y * 7.949_f32 - x.cos();
        let mut acc = Accumulator1132::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.331_f32 + y.sin();
        let b = y * 4.68_f32 - x.cos();
        let mut acc = Accumulator1132::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.797_f32 + y.sin();
        let b = y * 9.035_f32 - x.cos();
        let mut acc = Accumulator1132::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.077_f32 + y.sin();
        let b = y * 8.655_f32 - x.cos();
        let mut acc = Accumulator1132::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.287_f32 + y.sin();
        let b = y * 3.939_f32 - x.cos();
        let mut acc = Accumulator1132::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.407_f32 + y.sin();
        let b = y * 3.572_f32 - x.cos();
        let mut acc = Accumulator1132::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.627_f32 + y.sin();
        let b = y * 4.159_f32 - x.cos();
        let mut acc = Accumulator1132::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.405_f32 + y.sin();
        let b = y * 4.06_f32 - x.cos();
        let mut acc = Accumulator1132::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.718_f32 + y.sin();
        let b = y * 4.979_f32 - x.cos();
        let mut acc = Accumulator1132::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.542_f32 + y.sin();
        let b = y * 5.108_f32 - x.cos();
        let mut acc = Accumulator1132::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.348_f32 + y.sin();
        let b = y * 2.673_f32 - x.cos();
        let mut acc = Accumulator1132::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1132(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(1132u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_1132() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1132(total as u64) % 997) as f32;
        total
    }
}

pub mod m1133 {
    use super::*;

    pub struct Accumulator1133<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1133<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.634_f32 + y.sin();
        let b = y * 7.836_f32 - x.cos();
        let mut acc = Accumulator1133::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.351_f32 + y.sin();
        let b = y * 2.872_f32 - x.cos();
        let mut acc = Accumulator1133::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.817_f32 + y.sin();
        let b = y * 8.067_f32 - x.cos();
        let mut acc = Accumulator1133::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.345_f32 + y.sin();
        let b = y * 5.721_f32 - x.cos();
        let mut acc = Accumulator1133::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.676_f32 + y.sin();
        let b = y * 3.464_f32 - x.cos();
        let mut acc = Accumulator1133::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.303_f32 + y.sin();
        let b = y * 8.226_f32 - x.cos();
        let mut acc = Accumulator1133::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.223_f32 + y.sin();
        let b = y * 5.214_f32 - x.cos();
        let mut acc = Accumulator1133::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.604_f32 + y.sin();
        let b = y * 7.962_f32 - x.cos();
        let mut acc = Accumulator1133::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.744_f32 + y.sin();
        let b = y * 4.709_f32 - x.cos();
        let mut acc = Accumulator1133::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.776_f32 + y.sin();
        let b = y * 0.502_f32 - x.cos();
        let mut acc = Accumulator1133::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.745_f32 + y.sin();
        let b = y * 2.138_f32 - x.cos();
        let mut acc = Accumulator1133::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.534_f32 + y.sin();
        let b = y * 2.521_f32 - x.cos();
        let mut acc = Accumulator1133::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.584_f32 + y.sin();
        let b = y * 6.836_f32 - x.cos();
        let mut acc = Accumulator1133::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.127_f32 + y.sin();
        let b = y * 3.532_f32 - x.cos();
        let mut acc = Accumulator1133::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.99_f32 + y.sin();
        let b = y * 7.259_f32 - x.cos();
        let mut acc = Accumulator1133::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.225_f32 + y.sin();
        let b = y * 2.065_f32 - x.cos();
        let mut acc = Accumulator1133::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.647_f32 + y.sin();
        let b = y * 1.226_f32 - x.cos();
        let mut acc = Accumulator1133::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.658_f32 + y.sin();
        let b = y * 7.366_f32 - x.cos();
        let mut acc = Accumulator1133::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.025_f32 + y.sin();
        let b = y * 4.836_f32 - x.cos();
        let mut acc = Accumulator1133::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.328_f32 + y.sin();
        let b = y * 9.802_f32 - x.cos();
        let mut acc = Accumulator1133::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1133(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_1133() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1133(total as u64) % 997) as f32;
        total
    }
}

pub mod m1134 {
    use super::*;

    pub struct Accumulator1134<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1134<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.135_f32 + y.sin();
        let b = y * 9.735_f32 - x.cos();
        let mut acc = Accumulator1134::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.872_f32 + y.sin();
        let b = y * 8.251_f32 - x.cos();
        let mut acc = Accumulator1134::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.426_f32 + y.sin();
        let b = y * 2.039_f32 - x.cos();
        let mut acc = Accumulator1134::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.329_f32 + y.sin();
        let b = y * 4.67_f32 - x.cos();
        let mut acc = Accumulator1134::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.782_f32 + y.sin();
        let b = y * 3.709_f32 - x.cos();
        let mut acc = Accumulator1134::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.05_f32 + y.sin();
        let b = y * 1.308_f32 - x.cos();
        let mut acc = Accumulator1134::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.859_f32 + y.sin();
        let b = y * 0.677_f32 - x.cos();
        let mut acc = Accumulator1134::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.329_f32 + y.sin();
        let b = y * 9.426_f32 - x.cos();
        let mut acc = Accumulator1134::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.473_f32 + y.sin();
        let b = y * 3.361_f32 - x.cos();
        let mut acc = Accumulator1134::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.514_f32 + y.sin();
        let b = y * 9.541_f32 - x.cos();
        let mut acc = Accumulator1134::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.525_f32 + y.sin();
        let b = y * 1.539_f32 - x.cos();
        let mut acc = Accumulator1134::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.522_f32 + y.sin();
        let b = y * 8.558_f32 - x.cos();
        let mut acc = Accumulator1134::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.617_f32 + y.sin();
        let b = y * 5.067_f32 - x.cos();
        let mut acc = Accumulator1134::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.044_f32 + y.sin();
        let b = y * 2.598_f32 - x.cos();
        let mut acc = Accumulator1134::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.513_f32 + y.sin();
        let b = y * 2.929_f32 - x.cos();
        let mut acc = Accumulator1134::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.487_f32 + y.sin();
        let b = y * 6.388_f32 - x.cos();
        let mut acc = Accumulator1134::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.589_f32 + y.sin();
        let b = y * 5.613_f32 - x.cos();
        let mut acc = Accumulator1134::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.68_f32 + y.sin();
        let b = y * 8.248_f32 - x.cos();
        let mut acc = Accumulator1134::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.964_f32 + y.sin();
        let b = y * 7.815_f32 - x.cos();
        let mut acc = Accumulator1134::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.27_f32 + y.sin();
        let b = y * 9.21_f32 - x.cos();
        let mut acc = Accumulator1134::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1134(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_1134() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1134(total as u64) % 997) as f32;
        total
    }
}

pub mod m1135 {
    use super::*;

    pub struct Accumulator1135<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1135<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.991_f32 + y.sin();
        let b = y * 1.221_f32 - x.cos();
        let mut acc = Accumulator1135::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.176_f32 + y.sin();
        let b = y * 4.352_f32 - x.cos();
        let mut acc = Accumulator1135::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.86_f32 + y.sin();
        let b = y * 3.465_f32 - x.cos();
        let mut acc = Accumulator1135::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.001_f32 + y.sin();
        let b = y * 2.921_f32 - x.cos();
        let mut acc = Accumulator1135::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.397_f32 + y.sin();
        let b = y * 5.318_f32 - x.cos();
        let mut acc = Accumulator1135::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.09_f32 + y.sin();
        let b = y * 3.984_f32 - x.cos();
        let mut acc = Accumulator1135::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.642_f32 + y.sin();
        let b = y * 6.258_f32 - x.cos();
        let mut acc = Accumulator1135::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.796_f32 + y.sin();
        let b = y * 0.846_f32 - x.cos();
        let mut acc = Accumulator1135::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.547_f32 + y.sin();
        let b = y * 9.482_f32 - x.cos();
        let mut acc = Accumulator1135::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.622_f32 + y.sin();
        let b = y * 4.453_f32 - x.cos();
        let mut acc = Accumulator1135::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.486_f32 + y.sin();
        let b = y * 7.026_f32 - x.cos();
        let mut acc = Accumulator1135::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.334_f32 + y.sin();
        let b = y * 5.512_f32 - x.cos();
        let mut acc = Accumulator1135::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.988_f32 + y.sin();
        let b = y * 4.183_f32 - x.cos();
        let mut acc = Accumulator1135::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.507_f32 + y.sin();
        let b = y * 8.085_f32 - x.cos();
        let mut acc = Accumulator1135::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.439_f32 + y.sin();
        let b = y * 7.051_f32 - x.cos();
        let mut acc = Accumulator1135::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.665_f32 + y.sin();
        let b = y * 5.458_f32 - x.cos();
        let mut acc = Accumulator1135::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.234_f32 + y.sin();
        let b = y * 3.362_f32 - x.cos();
        let mut acc = Accumulator1135::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.401_f32 + y.sin();
        let b = y * 7.08_f32 - x.cos();
        let mut acc = Accumulator1135::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.491_f32 + y.sin();
        let b = y * 2.645_f32 - x.cos();
        let mut acc = Accumulator1135::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.401_f32 + y.sin();
        let b = y * 0.392_f32 - x.cos();
        let mut acc = Accumulator1135::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1135(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m1135-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_1135() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1135(total as u64) % 997) as f32;
        total
    }
}

pub mod m1136 {
    use super::*;

    pub struct Accumulator1136<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1136<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.158_f32 + y.sin();
        let b = y * 0.795_f32 - x.cos();
        let mut acc = Accumulator1136::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.814_f32 + y.sin();
        let b = y * 9.238_f32 - x.cos();
        let mut acc = Accumulator1136::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.771_f32 + y.sin();
        let b = y * 1.095_f32 - x.cos();
        let mut acc = Accumulator1136::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.735_f32 + y.sin();
        let b = y * 0.18_f32 - x.cos();
        let mut acc = Accumulator1136::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.397_f32 + y.sin();
        let b = y * 8.305_f32 - x.cos();
        let mut acc = Accumulator1136::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.916_f32 + y.sin();
        let b = y * 6.491_f32 - x.cos();
        let mut acc = Accumulator1136::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.262_f32 + y.sin();
        let b = y * 7.154_f32 - x.cos();
        let mut acc = Accumulator1136::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.09_f32 + y.sin();
        let b = y * 5.342_f32 - x.cos();
        let mut acc = Accumulator1136::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.008_f32 + y.sin();
        let b = y * 7.53_f32 - x.cos();
        let mut acc = Accumulator1136::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.671_f32 + y.sin();
        let b = y * 6.57_f32 - x.cos();
        let mut acc = Accumulator1136::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.797_f32 + y.sin();
        let b = y * 8.718_f32 - x.cos();
        let mut acc = Accumulator1136::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.005_f32 + y.sin();
        let b = y * 0.358_f32 - x.cos();
        let mut acc = Accumulator1136::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.479_f32 + y.sin();
        let b = y * 6.295_f32 - x.cos();
        let mut acc = Accumulator1136::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.726_f32 + y.sin();
        let b = y * 2.95_f32 - x.cos();
        let mut acc = Accumulator1136::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.673_f32 + y.sin();
        let b = y * 6.488_f32 - x.cos();
        let mut acc = Accumulator1136::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.149_f32 + y.sin();
        let b = y * 7.129_f32 - x.cos();
        let mut acc = Accumulator1136::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.43_f32 + y.sin();
        let b = y * 4.753_f32 - x.cos();
        let mut acc = Accumulator1136::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.992_f32 + y.sin();
        let b = y * 6.222_f32 - x.cos();
        let mut acc = Accumulator1136::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.922_f32 + y.sin();
        let b = y * 2.942_f32 - x.cos();
        let mut acc = Accumulator1136::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.927_f32 + y.sin();
        let b = y * 3.637_f32 - x.cos();
        let mut acc = Accumulator1136::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1136(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_1136() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1136(total as u64) % 997) as f32;
        total
    }
}

pub mod m1137 {
    use super::*;

    pub struct Accumulator1137<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1137<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.296_f32 + y.sin();
        let b = y * 3.153_f32 - x.cos();
        let mut acc = Accumulator1137::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.853_f32 + y.sin();
        let b = y * 8.203_f32 - x.cos();
        let mut acc = Accumulator1137::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.374_f32 + y.sin();
        let b = y * 8.238_f32 - x.cos();
        let mut acc = Accumulator1137::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.87_f32 + y.sin();
        let b = y * 7.264_f32 - x.cos();
        let mut acc = Accumulator1137::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.354_f32 + y.sin();
        let b = y * 5.756_f32 - x.cos();
        let mut acc = Accumulator1137::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.538_f32 + y.sin();
        let b = y * 5.642_f32 - x.cos();
        let mut acc = Accumulator1137::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.836_f32 + y.sin();
        let b = y * 1.486_f32 - x.cos();
        let mut acc = Accumulator1137::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.914_f32 + y.sin();
        let b = y * 7.767_f32 - x.cos();
        let mut acc = Accumulator1137::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.136_f32 + y.sin();
        let b = y * 7.45_f32 - x.cos();
        let mut acc = Accumulator1137::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.764_f32 + y.sin();
        let b = y * 3.057_f32 - x.cos();
        let mut acc = Accumulator1137::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.228_f32 + y.sin();
        let b = y * 7.627_f32 - x.cos();
        let mut acc = Accumulator1137::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.283_f32 + y.sin();
        let b = y * 7.668_f32 - x.cos();
        let mut acc = Accumulator1137::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.093_f32 + y.sin();
        let b = y * 9.757_f32 - x.cos();
        let mut acc = Accumulator1137::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.285_f32 + y.sin();
        let b = y * 1.356_f32 - x.cos();
        let mut acc = Accumulator1137::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.405_f32 + y.sin();
        let b = y * 7.958_f32 - x.cos();
        let mut acc = Accumulator1137::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.317_f32 + y.sin();
        let b = y * 5.99_f32 - x.cos();
        let mut acc = Accumulator1137::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.575_f32 + y.sin();
        let b = y * 4.122_f32 - x.cos();
        let mut acc = Accumulator1137::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.889_f32 + y.sin();
        let b = y * 7.835_f32 - x.cos();
        let mut acc = Accumulator1137::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.028_f32 + y.sin();
        let b = y * 8.845_f32 - x.cos();
        let mut acc = Accumulator1137::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.528_f32 + y.sin();
        let b = y * 1.35_f32 - x.cos();
        let mut acc = Accumulator1137::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1137(seed: u64) -> u64 {
        let re = Regex::new(r"m1137-(\d+)").unwrap();
        let hay = format!("m1137-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_1137() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1137(total as u64) % 997) as f32;
        total
    }
}

pub mod m1138 {
    use super::*;

    pub struct Accumulator1138<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1138<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.772_f32 + y.sin();
        let b = y * 2.518_f32 - x.cos();
        let mut acc = Accumulator1138::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.656_f32 + y.sin();
        let b = y * 2.373_f32 - x.cos();
        let mut acc = Accumulator1138::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.166_f32 + y.sin();
        let b = y * 7.9_f32 - x.cos();
        let mut acc = Accumulator1138::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.708_f32 + y.sin();
        let b = y * 7.067_f32 - x.cos();
        let mut acc = Accumulator1138::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.546_f32 + y.sin();
        let b = y * 5.623_f32 - x.cos();
        let mut acc = Accumulator1138::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.259_f32 + y.sin();
        let b = y * 0.287_f32 - x.cos();
        let mut acc = Accumulator1138::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.431_f32 + y.sin();
        let b = y * 6.497_f32 - x.cos();
        let mut acc = Accumulator1138::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.758_f32 + y.sin();
        let b = y * 6.783_f32 - x.cos();
        let mut acc = Accumulator1138::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.731_f32 + y.sin();
        let b = y * 2.474_f32 - x.cos();
        let mut acc = Accumulator1138::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.437_f32 + y.sin();
        let b = y * 5.157_f32 - x.cos();
        let mut acc = Accumulator1138::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.493_f32 + y.sin();
        let b = y * 9.476_f32 - x.cos();
        let mut acc = Accumulator1138::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.304_f32 + y.sin();
        let b = y * 5.926_f32 - x.cos();
        let mut acc = Accumulator1138::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.674_f32 + y.sin();
        let b = y * 6.028_f32 - x.cos();
        let mut acc = Accumulator1138::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.816_f32 + y.sin();
        let b = y * 5.462_f32 - x.cos();
        let mut acc = Accumulator1138::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.048_f32 + y.sin();
        let b = y * 2.529_f32 - x.cos();
        let mut acc = Accumulator1138::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.168_f32 + y.sin();
        let b = y * 3.66_f32 - x.cos();
        let mut acc = Accumulator1138::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.642_f32 + y.sin();
        let b = y * 2.715_f32 - x.cos();
        let mut acc = Accumulator1138::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.582_f32 + y.sin();
        let b = y * 1.579_f32 - x.cos();
        let mut acc = Accumulator1138::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.394_f32 + y.sin();
        let b = y * 1.046_f32 - x.cos();
        let mut acc = Accumulator1138::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.838_f32 + y.sin();
        let b = y * 6.753_f32 - x.cos();
        let mut acc = Accumulator1138::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1138(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_1138() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1138(total as u64) % 997) as f32;
        total
    }
}

pub mod m1139 {
    use super::*;

    pub struct Accumulator1139<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1139<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.212_f32 + y.sin();
        let b = y * 8.553_f32 - x.cos();
        let mut acc = Accumulator1139::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.656_f32 + y.sin();
        let b = y * 1.892_f32 - x.cos();
        let mut acc = Accumulator1139::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.938_f32 + y.sin();
        let b = y * 8.695_f32 - x.cos();
        let mut acc = Accumulator1139::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.632_f32 + y.sin();
        let b = y * 1.549_f32 - x.cos();
        let mut acc = Accumulator1139::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.478_f32 + y.sin();
        let b = y * 7.403_f32 - x.cos();
        let mut acc = Accumulator1139::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.257_f32 + y.sin();
        let b = y * 1.572_f32 - x.cos();
        let mut acc = Accumulator1139::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.554_f32 + y.sin();
        let b = y * 2.782_f32 - x.cos();
        let mut acc = Accumulator1139::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.307_f32 + y.sin();
        let b = y * 5.994_f32 - x.cos();
        let mut acc = Accumulator1139::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.339_f32 + y.sin();
        let b = y * 4.965_f32 - x.cos();
        let mut acc = Accumulator1139::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.116_f32 + y.sin();
        let b = y * 0.396_f32 - x.cos();
        let mut acc = Accumulator1139::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.518_f32 + y.sin();
        let b = y * 7.523_f32 - x.cos();
        let mut acc = Accumulator1139::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.846_f32 + y.sin();
        let b = y * 0.322_f32 - x.cos();
        let mut acc = Accumulator1139::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.462_f32 + y.sin();
        let b = y * 1.037_f32 - x.cos();
        let mut acc = Accumulator1139::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.318_f32 + y.sin();
        let b = y * 0.988_f32 - x.cos();
        let mut acc = Accumulator1139::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.917_f32 + y.sin();
        let b = y * 3.75_f32 - x.cos();
        let mut acc = Accumulator1139::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.805_f32 + y.sin();
        let b = y * 3.12_f32 - x.cos();
        let mut acc = Accumulator1139::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.937_f32 + y.sin();
        let b = y * 4.72_f32 - x.cos();
        let mut acc = Accumulator1139::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.79_f32 + y.sin();
        let b = y * 5.596_f32 - x.cos();
        let mut acc = Accumulator1139::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.275_f32 + y.sin();
        let b = y * 8.585_f32 - x.cos();
        let mut acc = Accumulator1139::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.427_f32 + y.sin();
        let b = y * 4.419_f32 - x.cos();
        let mut acc = Accumulator1139::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1139(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(1139u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_1139() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1139(total as u64) % 997) as f32;
        total
    }
}

pub mod m1140 {
    use super::*;

    pub struct Accumulator1140<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1140<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.857_f32 + y.sin();
        let b = y * 7.675_f32 - x.cos();
        let mut acc = Accumulator1140::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.918_f32 + y.sin();
        let b = y * 2.444_f32 - x.cos();
        let mut acc = Accumulator1140::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.082_f32 + y.sin();
        let b = y * 8.76_f32 - x.cos();
        let mut acc = Accumulator1140::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.358_f32 + y.sin();
        let b = y * 8.079_f32 - x.cos();
        let mut acc = Accumulator1140::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.885_f32 + y.sin();
        let b = y * 3.012_f32 - x.cos();
        let mut acc = Accumulator1140::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.932_f32 + y.sin();
        let b = y * 8.21_f32 - x.cos();
        let mut acc = Accumulator1140::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.801_f32 + y.sin();
        let b = y * 8.442_f32 - x.cos();
        let mut acc = Accumulator1140::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.978_f32 + y.sin();
        let b = y * 7.089_f32 - x.cos();
        let mut acc = Accumulator1140::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.87_f32 + y.sin();
        let b = y * 2.542_f32 - x.cos();
        let mut acc = Accumulator1140::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.45_f32 + y.sin();
        let b = y * 4.344_f32 - x.cos();
        let mut acc = Accumulator1140::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.46_f32 + y.sin();
        let b = y * 5.391_f32 - x.cos();
        let mut acc = Accumulator1140::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.569_f32 + y.sin();
        let b = y * 9.061_f32 - x.cos();
        let mut acc = Accumulator1140::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.533_f32 + y.sin();
        let b = y * 8.109_f32 - x.cos();
        let mut acc = Accumulator1140::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.569_f32 + y.sin();
        let b = y * 2.977_f32 - x.cos();
        let mut acc = Accumulator1140::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.557_f32 + y.sin();
        let b = y * 1.87_f32 - x.cos();
        let mut acc = Accumulator1140::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.579_f32 + y.sin();
        let b = y * 0.538_f32 - x.cos();
        let mut acc = Accumulator1140::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.152_f32 + y.sin();
        let b = y * 8.344_f32 - x.cos();
        let mut acc = Accumulator1140::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.687_f32 + y.sin();
        let b = y * 4.384_f32 - x.cos();
        let mut acc = Accumulator1140::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.7_f32 + y.sin();
        let b = y * 6.937_f32 - x.cos();
        let mut acc = Accumulator1140::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.752_f32 + y.sin();
        let b = y * 0.682_f32 - x.cos();
        let mut acc = Accumulator1140::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1140(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_1140() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1140(total as u64) % 997) as f32;
        total
    }
}

pub mod m1141 {
    use super::*;

    pub struct Accumulator1141<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1141<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.836_f32 + y.sin();
        let b = y * 0.128_f32 - x.cos();
        let mut acc = Accumulator1141::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.605_f32 + y.sin();
        let b = y * 4.379_f32 - x.cos();
        let mut acc = Accumulator1141::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.741_f32 + y.sin();
        let b = y * 8.849_f32 - x.cos();
        let mut acc = Accumulator1141::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.624_f32 + y.sin();
        let b = y * 4.443_f32 - x.cos();
        let mut acc = Accumulator1141::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.081_f32 + y.sin();
        let b = y * 7.949_f32 - x.cos();
        let mut acc = Accumulator1141::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.088_f32 + y.sin();
        let b = y * 0.907_f32 - x.cos();
        let mut acc = Accumulator1141::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.607_f32 + y.sin();
        let b = y * 2.746_f32 - x.cos();
        let mut acc = Accumulator1141::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.568_f32 + y.sin();
        let b = y * 6.352_f32 - x.cos();
        let mut acc = Accumulator1141::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.532_f32 + y.sin();
        let b = y * 6.026_f32 - x.cos();
        let mut acc = Accumulator1141::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.775_f32 + y.sin();
        let b = y * 1.76_f32 - x.cos();
        let mut acc = Accumulator1141::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.082_f32 + y.sin();
        let b = y * 3.447_f32 - x.cos();
        let mut acc = Accumulator1141::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.293_f32 + y.sin();
        let b = y * 9.565_f32 - x.cos();
        let mut acc = Accumulator1141::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.179_f32 + y.sin();
        let b = y * 3.565_f32 - x.cos();
        let mut acc = Accumulator1141::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.512_f32 + y.sin();
        let b = y * 4.978_f32 - x.cos();
        let mut acc = Accumulator1141::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.159_f32 + y.sin();
        let b = y * 8.72_f32 - x.cos();
        let mut acc = Accumulator1141::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.6_f32 + y.sin();
        let b = y * 4.754_f32 - x.cos();
        let mut acc = Accumulator1141::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.626_f32 + y.sin();
        let b = y * 2.593_f32 - x.cos();
        let mut acc = Accumulator1141::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.95_f32 + y.sin();
        let b = y * 3.369_f32 - x.cos();
        let mut acc = Accumulator1141::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.87_f32 + y.sin();
        let b = y * 2.487_f32 - x.cos();
        let mut acc = Accumulator1141::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.849_f32 + y.sin();
        let b = y * 2.929_f32 - x.cos();
        let mut acc = Accumulator1141::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1141(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_1141() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1141(total as u64) % 997) as f32;
        total
    }
}

pub mod m1142 {
    use super::*;

    pub struct Accumulator1142<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1142<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.84_f32 + y.sin();
        let b = y * 5.204_f32 - x.cos();
        let mut acc = Accumulator1142::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.457_f32 + y.sin();
        let b = y * 5.593_f32 - x.cos();
        let mut acc = Accumulator1142::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.186_f32 + y.sin();
        let b = y * 5.129_f32 - x.cos();
        let mut acc = Accumulator1142::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.086_f32 + y.sin();
        let b = y * 7.758_f32 - x.cos();
        let mut acc = Accumulator1142::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.32_f32 + y.sin();
        let b = y * 3.933_f32 - x.cos();
        let mut acc = Accumulator1142::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.624_f32 + y.sin();
        let b = y * 1.683_f32 - x.cos();
        let mut acc = Accumulator1142::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.899_f32 + y.sin();
        let b = y * 5.982_f32 - x.cos();
        let mut acc = Accumulator1142::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.877_f32 + y.sin();
        let b = y * 7.374_f32 - x.cos();
        let mut acc = Accumulator1142::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.676_f32 + y.sin();
        let b = y * 5.561_f32 - x.cos();
        let mut acc = Accumulator1142::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.486_f32 + y.sin();
        let b = y * 9.673_f32 - x.cos();
        let mut acc = Accumulator1142::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.909_f32 + y.sin();
        let b = y * 5.007_f32 - x.cos();
        let mut acc = Accumulator1142::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.637_f32 + y.sin();
        let b = y * 8.245_f32 - x.cos();
        let mut acc = Accumulator1142::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.163_f32 + y.sin();
        let b = y * 4.945_f32 - x.cos();
        let mut acc = Accumulator1142::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.407_f32 + y.sin();
        let b = y * 8.77_f32 - x.cos();
        let mut acc = Accumulator1142::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.405_f32 + y.sin();
        let b = y * 6.457_f32 - x.cos();
        let mut acc = Accumulator1142::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.018_f32 + y.sin();
        let b = y * 7.52_f32 - x.cos();
        let mut acc = Accumulator1142::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.462_f32 + y.sin();
        let b = y * 1.368_f32 - x.cos();
        let mut acc = Accumulator1142::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.761_f32 + y.sin();
        let b = y * 6.143_f32 - x.cos();
        let mut acc = Accumulator1142::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.299_f32 + y.sin();
        let b = y * 2.642_f32 - x.cos();
        let mut acc = Accumulator1142::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.922_f32 + y.sin();
        let b = y * 9.825_f32 - x.cos();
        let mut acc = Accumulator1142::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1142(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m1142-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_1142() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1142(total as u64) % 997) as f32;
        total
    }
}

pub mod m1143 {
    use super::*;

    pub struct Accumulator1143<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1143<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.943_f32 + y.sin();
        let b = y * 2.754_f32 - x.cos();
        let mut acc = Accumulator1143::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.141_f32 + y.sin();
        let b = y * 6.62_f32 - x.cos();
        let mut acc = Accumulator1143::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.481_f32 + y.sin();
        let b = y * 2.474_f32 - x.cos();
        let mut acc = Accumulator1143::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.38_f32 + y.sin();
        let b = y * 2.381_f32 - x.cos();
        let mut acc = Accumulator1143::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.452_f32 + y.sin();
        let b = y * 9.345_f32 - x.cos();
        let mut acc = Accumulator1143::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.491_f32 + y.sin();
        let b = y * 4.633_f32 - x.cos();
        let mut acc = Accumulator1143::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.582_f32 + y.sin();
        let b = y * 0.26_f32 - x.cos();
        let mut acc = Accumulator1143::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.903_f32 + y.sin();
        let b = y * 2.816_f32 - x.cos();
        let mut acc = Accumulator1143::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.96_f32 + y.sin();
        let b = y * 0.25_f32 - x.cos();
        let mut acc = Accumulator1143::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.38_f32 + y.sin();
        let b = y * 2.908_f32 - x.cos();
        let mut acc = Accumulator1143::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.124_f32 + y.sin();
        let b = y * 8.37_f32 - x.cos();
        let mut acc = Accumulator1143::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.942_f32 + y.sin();
        let b = y * 7.257_f32 - x.cos();
        let mut acc = Accumulator1143::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.588_f32 + y.sin();
        let b = y * 2.626_f32 - x.cos();
        let mut acc = Accumulator1143::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.717_f32 + y.sin();
        let b = y * 9.008_f32 - x.cos();
        let mut acc = Accumulator1143::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.497_f32 + y.sin();
        let b = y * 6.871_f32 - x.cos();
        let mut acc = Accumulator1143::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.695_f32 + y.sin();
        let b = y * 7.483_f32 - x.cos();
        let mut acc = Accumulator1143::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.681_f32 + y.sin();
        let b = y * 8.796_f32 - x.cos();
        let mut acc = Accumulator1143::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.991_f32 + y.sin();
        let b = y * 3.915_f32 - x.cos();
        let mut acc = Accumulator1143::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.088_f32 + y.sin();
        let b = y * 8.036_f32 - x.cos();
        let mut acc = Accumulator1143::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.76_f32 + y.sin();
        let b = y * 9.153_f32 - x.cos();
        let mut acc = Accumulator1143::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1143(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_1143() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1143(total as u64) % 997) as f32;
        total
    }
}

pub mod m1144 {
    use super::*;

    pub struct Accumulator1144<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1144<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.884_f32 + y.sin();
        let b = y * 2.937_f32 - x.cos();
        let mut acc = Accumulator1144::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.405_f32 + y.sin();
        let b = y * 1.21_f32 - x.cos();
        let mut acc = Accumulator1144::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.254_f32 + y.sin();
        let b = y * 9.69_f32 - x.cos();
        let mut acc = Accumulator1144::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.112_f32 + y.sin();
        let b = y * 4.967_f32 - x.cos();
        let mut acc = Accumulator1144::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.401_f32 + y.sin();
        let b = y * 8.393_f32 - x.cos();
        let mut acc = Accumulator1144::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.045_f32 + y.sin();
        let b = y * 0.838_f32 - x.cos();
        let mut acc = Accumulator1144::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.598_f32 + y.sin();
        let b = y * 4.984_f32 - x.cos();
        let mut acc = Accumulator1144::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.989_f32 + y.sin();
        let b = y * 8.619_f32 - x.cos();
        let mut acc = Accumulator1144::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.592_f32 + y.sin();
        let b = y * 7.236_f32 - x.cos();
        let mut acc = Accumulator1144::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.398_f32 + y.sin();
        let b = y * 4.727_f32 - x.cos();
        let mut acc = Accumulator1144::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.631_f32 + y.sin();
        let b = y * 8.421_f32 - x.cos();
        let mut acc = Accumulator1144::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.21_f32 + y.sin();
        let b = y * 8.948_f32 - x.cos();
        let mut acc = Accumulator1144::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.575_f32 + y.sin();
        let b = y * 8.961_f32 - x.cos();
        let mut acc = Accumulator1144::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.124_f32 + y.sin();
        let b = y * 1.447_f32 - x.cos();
        let mut acc = Accumulator1144::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.687_f32 + y.sin();
        let b = y * 9.813_f32 - x.cos();
        let mut acc = Accumulator1144::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.354_f32 + y.sin();
        let b = y * 9.411_f32 - x.cos();
        let mut acc = Accumulator1144::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.808_f32 + y.sin();
        let b = y * 6.257_f32 - x.cos();
        let mut acc = Accumulator1144::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.716_f32 + y.sin();
        let b = y * 5.164_f32 - x.cos();
        let mut acc = Accumulator1144::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.707_f32 + y.sin();
        let b = y * 6.817_f32 - x.cos();
        let mut acc = Accumulator1144::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.954_f32 + y.sin();
        let b = y * 6.02_f32 - x.cos();
        let mut acc = Accumulator1144::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1144(seed: u64) -> u64 {
        let re = Regex::new(r"m1144-(\d+)").unwrap();
        let hay = format!("m1144-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_1144() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1144(total as u64) % 997) as f32;
        total
    }
}

pub mod m1145 {
    use super::*;

    pub struct Accumulator1145<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1145<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.495_f32 + y.sin();
        let b = y * 8.849_f32 - x.cos();
        let mut acc = Accumulator1145::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.344_f32 + y.sin();
        let b = y * 6.848_f32 - x.cos();
        let mut acc = Accumulator1145::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.875_f32 + y.sin();
        let b = y * 6.241_f32 - x.cos();
        let mut acc = Accumulator1145::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.71_f32 + y.sin();
        let b = y * 8.59_f32 - x.cos();
        let mut acc = Accumulator1145::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.053_f32 + y.sin();
        let b = y * 3.07_f32 - x.cos();
        let mut acc = Accumulator1145::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.516_f32 + y.sin();
        let b = y * 4.409_f32 - x.cos();
        let mut acc = Accumulator1145::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.991_f32 + y.sin();
        let b = y * 6.062_f32 - x.cos();
        let mut acc = Accumulator1145::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.915_f32 + y.sin();
        let b = y * 3.578_f32 - x.cos();
        let mut acc = Accumulator1145::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.422_f32 + y.sin();
        let b = y * 3.769_f32 - x.cos();
        let mut acc = Accumulator1145::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.1_f32 + y.sin();
        let b = y * 8.493_f32 - x.cos();
        let mut acc = Accumulator1145::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.083_f32 + y.sin();
        let b = y * 6.854_f32 - x.cos();
        let mut acc = Accumulator1145::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.983_f32 + y.sin();
        let b = y * 6.531_f32 - x.cos();
        let mut acc = Accumulator1145::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.762_f32 + y.sin();
        let b = y * 4.475_f32 - x.cos();
        let mut acc = Accumulator1145::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.619_f32 + y.sin();
        let b = y * 2.785_f32 - x.cos();
        let mut acc = Accumulator1145::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.341_f32 + y.sin();
        let b = y * 7.771_f32 - x.cos();
        let mut acc = Accumulator1145::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.496_f32 + y.sin();
        let b = y * 4.385_f32 - x.cos();
        let mut acc = Accumulator1145::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.177_f32 + y.sin();
        let b = y * 2.377_f32 - x.cos();
        let mut acc = Accumulator1145::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.079_f32 + y.sin();
        let b = y * 1.944_f32 - x.cos();
        let mut acc = Accumulator1145::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.533_f32 + y.sin();
        let b = y * 0.661_f32 - x.cos();
        let mut acc = Accumulator1145::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.682_f32 + y.sin();
        let b = y * 7.967_f32 - x.cos();
        let mut acc = Accumulator1145::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1145(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_1145() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1145(total as u64) % 997) as f32;
        total
    }
}

pub mod m1146 {
    use super::*;

    pub struct Accumulator1146<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1146<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.846_f32 + y.sin();
        let b = y * 0.621_f32 - x.cos();
        let mut acc = Accumulator1146::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.293_f32 + y.sin();
        let b = y * 8.768_f32 - x.cos();
        let mut acc = Accumulator1146::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.457_f32 + y.sin();
        let b = y * 2.068_f32 - x.cos();
        let mut acc = Accumulator1146::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.131_f32 + y.sin();
        let b = y * 0.768_f32 - x.cos();
        let mut acc = Accumulator1146::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.637_f32 + y.sin();
        let b = y * 8.885_f32 - x.cos();
        let mut acc = Accumulator1146::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.545_f32 + y.sin();
        let b = y * 7.329_f32 - x.cos();
        let mut acc = Accumulator1146::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.802_f32 + y.sin();
        let b = y * 3.244_f32 - x.cos();
        let mut acc = Accumulator1146::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.352_f32 + y.sin();
        let b = y * 6.735_f32 - x.cos();
        let mut acc = Accumulator1146::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.325_f32 + y.sin();
        let b = y * 5.419_f32 - x.cos();
        let mut acc = Accumulator1146::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.287_f32 + y.sin();
        let b = y * 9.807_f32 - x.cos();
        let mut acc = Accumulator1146::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.887_f32 + y.sin();
        let b = y * 2.776_f32 - x.cos();
        let mut acc = Accumulator1146::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.456_f32 + y.sin();
        let b = y * 8.385_f32 - x.cos();
        let mut acc = Accumulator1146::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.017_f32 + y.sin();
        let b = y * 2.865_f32 - x.cos();
        let mut acc = Accumulator1146::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.133_f32 + y.sin();
        let b = y * 9.366_f32 - x.cos();
        let mut acc = Accumulator1146::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.635_f32 + y.sin();
        let b = y * 5.57_f32 - x.cos();
        let mut acc = Accumulator1146::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.278_f32 + y.sin();
        let b = y * 6.844_f32 - x.cos();
        let mut acc = Accumulator1146::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.491_f32 + y.sin();
        let b = y * 4.03_f32 - x.cos();
        let mut acc = Accumulator1146::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.269_f32 + y.sin();
        let b = y * 1.53_f32 - x.cos();
        let mut acc = Accumulator1146::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.334_f32 + y.sin();
        let b = y * 0.87_f32 - x.cos();
        let mut acc = Accumulator1146::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.255_f32 + y.sin();
        let b = y * 1.677_f32 - x.cos();
        let mut acc = Accumulator1146::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1146(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(1146u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_1146() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1146(total as u64) % 997) as f32;
        total
    }
}

pub mod m1147 {
    use super::*;

    pub struct Accumulator1147<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1147<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.491_f32 + y.sin();
        let b = y * 0.778_f32 - x.cos();
        let mut acc = Accumulator1147::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.418_f32 + y.sin();
        let b = y * 9.133_f32 - x.cos();
        let mut acc = Accumulator1147::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.924_f32 + y.sin();
        let b = y * 0.22_f32 - x.cos();
        let mut acc = Accumulator1147::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.959_f32 + y.sin();
        let b = y * 8.054_f32 - x.cos();
        let mut acc = Accumulator1147::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.234_f32 + y.sin();
        let b = y * 5.12_f32 - x.cos();
        let mut acc = Accumulator1147::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.399_f32 + y.sin();
        let b = y * 8.179_f32 - x.cos();
        let mut acc = Accumulator1147::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.791_f32 + y.sin();
        let b = y * 2.357_f32 - x.cos();
        let mut acc = Accumulator1147::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.234_f32 + y.sin();
        let b = y * 6.38_f32 - x.cos();
        let mut acc = Accumulator1147::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.335_f32 + y.sin();
        let b = y * 8.151_f32 - x.cos();
        let mut acc = Accumulator1147::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.509_f32 + y.sin();
        let b = y * 1.084_f32 - x.cos();
        let mut acc = Accumulator1147::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.228_f32 + y.sin();
        let b = y * 1.64_f32 - x.cos();
        let mut acc = Accumulator1147::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.828_f32 + y.sin();
        let b = y * 9.063_f32 - x.cos();
        let mut acc = Accumulator1147::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.526_f32 + y.sin();
        let b = y * 8.496_f32 - x.cos();
        let mut acc = Accumulator1147::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.432_f32 + y.sin();
        let b = y * 9.165_f32 - x.cos();
        let mut acc = Accumulator1147::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.511_f32 + y.sin();
        let b = y * 1.752_f32 - x.cos();
        let mut acc = Accumulator1147::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.31_f32 + y.sin();
        let b = y * 9.86_f32 - x.cos();
        let mut acc = Accumulator1147::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.729_f32 + y.sin();
        let b = y * 8.049_f32 - x.cos();
        let mut acc = Accumulator1147::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.615_f32 + y.sin();
        let b = y * 8.181_f32 - x.cos();
        let mut acc = Accumulator1147::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.976_f32 + y.sin();
        let b = y * 1.418_f32 - x.cos();
        let mut acc = Accumulator1147::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.701_f32 + y.sin();
        let b = y * 3.23_f32 - x.cos();
        let mut acc = Accumulator1147::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1147(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_1147() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1147(total as u64) % 997) as f32;
        total
    }
}

pub mod m1148 {
    use super::*;

    pub struct Accumulator1148<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1148<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.411_f32 + y.sin();
        let b = y * 1.084_f32 - x.cos();
        let mut acc = Accumulator1148::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.205_f32 + y.sin();
        let b = y * 9.35_f32 - x.cos();
        let mut acc = Accumulator1148::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.07_f32 + y.sin();
        let b = y * 5.45_f32 - x.cos();
        let mut acc = Accumulator1148::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.107_f32 + y.sin();
        let b = y * 8.322_f32 - x.cos();
        let mut acc = Accumulator1148::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.447_f32 + y.sin();
        let b = y * 5.248_f32 - x.cos();
        let mut acc = Accumulator1148::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.473_f32 + y.sin();
        let b = y * 8.395_f32 - x.cos();
        let mut acc = Accumulator1148::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.731_f32 + y.sin();
        let b = y * 4.365_f32 - x.cos();
        let mut acc = Accumulator1148::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.149_f32 + y.sin();
        let b = y * 0.155_f32 - x.cos();
        let mut acc = Accumulator1148::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.119_f32 + y.sin();
        let b = y * 3.627_f32 - x.cos();
        let mut acc = Accumulator1148::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.021_f32 + y.sin();
        let b = y * 1.262_f32 - x.cos();
        let mut acc = Accumulator1148::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.42_f32 + y.sin();
        let b = y * 3.457_f32 - x.cos();
        let mut acc = Accumulator1148::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.579_f32 + y.sin();
        let b = y * 8.994_f32 - x.cos();
        let mut acc = Accumulator1148::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.8_f32 + y.sin();
        let b = y * 1.941_f32 - x.cos();
        let mut acc = Accumulator1148::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.339_f32 + y.sin();
        let b = y * 2.146_f32 - x.cos();
        let mut acc = Accumulator1148::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.767_f32 + y.sin();
        let b = y * 8.556_f32 - x.cos();
        let mut acc = Accumulator1148::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.988_f32 + y.sin();
        let b = y * 9.325_f32 - x.cos();
        let mut acc = Accumulator1148::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.571_f32 + y.sin();
        let b = y * 9.849_f32 - x.cos();
        let mut acc = Accumulator1148::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.642_f32 + y.sin();
        let b = y * 5.764_f32 - x.cos();
        let mut acc = Accumulator1148::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.138_f32 + y.sin();
        let b = y * 5.705_f32 - x.cos();
        let mut acc = Accumulator1148::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.861_f32 + y.sin();
        let b = y * 9.831_f32 - x.cos();
        let mut acc = Accumulator1148::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1148(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_1148() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1148(total as u64) % 997) as f32;
        total
    }
}

pub mod m1149 {
    use super::*;

    pub struct Accumulator1149<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1149<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.1_f32 + y.sin();
        let b = y * 7.522_f32 - x.cos();
        let mut acc = Accumulator1149::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.758_f32 + y.sin();
        let b = y * 4.112_f32 - x.cos();
        let mut acc = Accumulator1149::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.759_f32 + y.sin();
        let b = y * 3.078_f32 - x.cos();
        let mut acc = Accumulator1149::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.838_f32 + y.sin();
        let b = y * 0.234_f32 - x.cos();
        let mut acc = Accumulator1149::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.386_f32 + y.sin();
        let b = y * 6.158_f32 - x.cos();
        let mut acc = Accumulator1149::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.368_f32 + y.sin();
        let b = y * 7.46_f32 - x.cos();
        let mut acc = Accumulator1149::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.734_f32 + y.sin();
        let b = y * 8.283_f32 - x.cos();
        let mut acc = Accumulator1149::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.271_f32 + y.sin();
        let b = y * 2.035_f32 - x.cos();
        let mut acc = Accumulator1149::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.958_f32 + y.sin();
        let b = y * 6.928_f32 - x.cos();
        let mut acc = Accumulator1149::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.148_f32 + y.sin();
        let b = y * 9.838_f32 - x.cos();
        let mut acc = Accumulator1149::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.758_f32 + y.sin();
        let b = y * 7.707_f32 - x.cos();
        let mut acc = Accumulator1149::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.496_f32 + y.sin();
        let b = y * 6.456_f32 - x.cos();
        let mut acc = Accumulator1149::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.316_f32 + y.sin();
        let b = y * 3.783_f32 - x.cos();
        let mut acc = Accumulator1149::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.243_f32 + y.sin();
        let b = y * 8.821_f32 - x.cos();
        let mut acc = Accumulator1149::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.728_f32 + y.sin();
        let b = y * 7.413_f32 - x.cos();
        let mut acc = Accumulator1149::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.688_f32 + y.sin();
        let b = y * 3.7_f32 - x.cos();
        let mut acc = Accumulator1149::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.026_f32 + y.sin();
        let b = y * 7.533_f32 - x.cos();
        let mut acc = Accumulator1149::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.896_f32 + y.sin();
        let b = y * 5.644_f32 - x.cos();
        let mut acc = Accumulator1149::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.418_f32 + y.sin();
        let b = y * 4.076_f32 - x.cos();
        let mut acc = Accumulator1149::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.109_f32 + y.sin();
        let b = y * 2.581_f32 - x.cos();
        let mut acc = Accumulator1149::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1149(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m1149-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_1149() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1149(total as u64) % 997) as f32;
        total
    }
}

pub mod m1150 {
    use super::*;

    pub struct Accumulator1150<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1150<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.296_f32 + y.sin();
        let b = y * 5.087_f32 - x.cos();
        let mut acc = Accumulator1150::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.639_f32 + y.sin();
        let b = y * 8.317_f32 - x.cos();
        let mut acc = Accumulator1150::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.6_f32 + y.sin();
        let b = y * 7.873_f32 - x.cos();
        let mut acc = Accumulator1150::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.749_f32 + y.sin();
        let b = y * 1.777_f32 - x.cos();
        let mut acc = Accumulator1150::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.299_f32 + y.sin();
        let b = y * 0.407_f32 - x.cos();
        let mut acc = Accumulator1150::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.537_f32 + y.sin();
        let b = y * 3.88_f32 - x.cos();
        let mut acc = Accumulator1150::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.321_f32 + y.sin();
        let b = y * 5.585_f32 - x.cos();
        let mut acc = Accumulator1150::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.079_f32 + y.sin();
        let b = y * 6.464_f32 - x.cos();
        let mut acc = Accumulator1150::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.823_f32 + y.sin();
        let b = y * 3.986_f32 - x.cos();
        let mut acc = Accumulator1150::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.69_f32 + y.sin();
        let b = y * 2.936_f32 - x.cos();
        let mut acc = Accumulator1150::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.196_f32 + y.sin();
        let b = y * 4.712_f32 - x.cos();
        let mut acc = Accumulator1150::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.312_f32 + y.sin();
        let b = y * 6.637_f32 - x.cos();
        let mut acc = Accumulator1150::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.962_f32 + y.sin();
        let b = y * 6.127_f32 - x.cos();
        let mut acc = Accumulator1150::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.191_f32 + y.sin();
        let b = y * 1.033_f32 - x.cos();
        let mut acc = Accumulator1150::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.907_f32 + y.sin();
        let b = y * 4.399_f32 - x.cos();
        let mut acc = Accumulator1150::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.868_f32 + y.sin();
        let b = y * 1.435_f32 - x.cos();
        let mut acc = Accumulator1150::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.405_f32 + y.sin();
        let b = y * 9.43_f32 - x.cos();
        let mut acc = Accumulator1150::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.076_f32 + y.sin();
        let b = y * 3.318_f32 - x.cos();
        let mut acc = Accumulator1150::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.081_f32 + y.sin();
        let b = y * 3.512_f32 - x.cos();
        let mut acc = Accumulator1150::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.627_f32 + y.sin();
        let b = y * 1.023_f32 - x.cos();
        let mut acc = Accumulator1150::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1150(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_1150() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1150(total as u64) % 997) as f32;
        total
    }
}

pub mod m1151 {
    use super::*;

    pub struct Accumulator1151<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1151<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.474_f32 + y.sin();
        let b = y * 6.638_f32 - x.cos();
        let mut acc = Accumulator1151::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.179_f32 + y.sin();
        let b = y * 3.739_f32 - x.cos();
        let mut acc = Accumulator1151::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.428_f32 + y.sin();
        let b = y * 2.537_f32 - x.cos();
        let mut acc = Accumulator1151::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.451_f32 + y.sin();
        let b = y * 4.576_f32 - x.cos();
        let mut acc = Accumulator1151::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.454_f32 + y.sin();
        let b = y * 1.728_f32 - x.cos();
        let mut acc = Accumulator1151::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.369_f32 + y.sin();
        let b = y * 2.81_f32 - x.cos();
        let mut acc = Accumulator1151::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.695_f32 + y.sin();
        let b = y * 0.278_f32 - x.cos();
        let mut acc = Accumulator1151::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.626_f32 + y.sin();
        let b = y * 3.507_f32 - x.cos();
        let mut acc = Accumulator1151::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.504_f32 + y.sin();
        let b = y * 0.226_f32 - x.cos();
        let mut acc = Accumulator1151::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.719_f32 + y.sin();
        let b = y * 9.73_f32 - x.cos();
        let mut acc = Accumulator1151::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.61_f32 + y.sin();
        let b = y * 2.968_f32 - x.cos();
        let mut acc = Accumulator1151::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.785_f32 + y.sin();
        let b = y * 8.248_f32 - x.cos();
        let mut acc = Accumulator1151::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.865_f32 + y.sin();
        let b = y * 2.502_f32 - x.cos();
        let mut acc = Accumulator1151::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.007_f32 + y.sin();
        let b = y * 4.119_f32 - x.cos();
        let mut acc = Accumulator1151::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.694_f32 + y.sin();
        let b = y * 7.387_f32 - x.cos();
        let mut acc = Accumulator1151::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.258_f32 + y.sin();
        let b = y * 4.181_f32 - x.cos();
        let mut acc = Accumulator1151::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.379_f32 + y.sin();
        let b = y * 6.63_f32 - x.cos();
        let mut acc = Accumulator1151::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.178_f32 + y.sin();
        let b = y * 8.508_f32 - x.cos();
        let mut acc = Accumulator1151::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.15_f32 + y.sin();
        let b = y * 4.836_f32 - x.cos();
        let mut acc = Accumulator1151::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.938_f32 + y.sin();
        let b = y * 7.547_f32 - x.cos();
        let mut acc = Accumulator1151::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1151(seed: u64) -> u64 {
        let re = Regex::new(r"m1151-(\d+)").unwrap();
        let hay = format!("m1151-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_1151() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1151(total as u64) % 997) as f32;
        total
    }
}

pub mod m1152 {
    use super::*;

    pub struct Accumulator1152<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1152<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.557_f32 + y.sin();
        let b = y * 1.339_f32 - x.cos();
        let mut acc = Accumulator1152::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.428_f32 + y.sin();
        let b = y * 7.214_f32 - x.cos();
        let mut acc = Accumulator1152::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.923_f32 + y.sin();
        let b = y * 4.087_f32 - x.cos();
        let mut acc = Accumulator1152::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.41_f32 + y.sin();
        let b = y * 7.121_f32 - x.cos();
        let mut acc = Accumulator1152::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.691_f32 + y.sin();
        let b = y * 9.702_f32 - x.cos();
        let mut acc = Accumulator1152::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.038_f32 + y.sin();
        let b = y * 1.335_f32 - x.cos();
        let mut acc = Accumulator1152::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.064_f32 + y.sin();
        let b = y * 5.443_f32 - x.cos();
        let mut acc = Accumulator1152::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.516_f32 + y.sin();
        let b = y * 6.369_f32 - x.cos();
        let mut acc = Accumulator1152::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.797_f32 + y.sin();
        let b = y * 0.809_f32 - x.cos();
        let mut acc = Accumulator1152::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.538_f32 + y.sin();
        let b = y * 4.593_f32 - x.cos();
        let mut acc = Accumulator1152::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.604_f32 + y.sin();
        let b = y * 0.596_f32 - x.cos();
        let mut acc = Accumulator1152::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.245_f32 + y.sin();
        let b = y * 3.15_f32 - x.cos();
        let mut acc = Accumulator1152::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.309_f32 + y.sin();
        let b = y * 4.146_f32 - x.cos();
        let mut acc = Accumulator1152::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.585_f32 + y.sin();
        let b = y * 9.637_f32 - x.cos();
        let mut acc = Accumulator1152::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.406_f32 + y.sin();
        let b = y * 4.919_f32 - x.cos();
        let mut acc = Accumulator1152::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.167_f32 + y.sin();
        let b = y * 7.241_f32 - x.cos();
        let mut acc = Accumulator1152::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.009_f32 + y.sin();
        let b = y * 1.303_f32 - x.cos();
        let mut acc = Accumulator1152::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.95_f32 + y.sin();
        let b = y * 3.668_f32 - x.cos();
        let mut acc = Accumulator1152::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.848_f32 + y.sin();
        let b = y * 8.58_f32 - x.cos();
        let mut acc = Accumulator1152::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.216_f32 + y.sin();
        let b = y * 0.814_f32 - x.cos();
        let mut acc = Accumulator1152::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1152(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_1152() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1152(total as u64) % 997) as f32;
        total
    }
}

pub mod m1153 {
    use super::*;

    pub struct Accumulator1153<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1153<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.548_f32 + y.sin();
        let b = y * 7.986_f32 - x.cos();
        let mut acc = Accumulator1153::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.322_f32 + y.sin();
        let b = y * 5.145_f32 - x.cos();
        let mut acc = Accumulator1153::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.484_f32 + y.sin();
        let b = y * 2.315_f32 - x.cos();
        let mut acc = Accumulator1153::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.341_f32 + y.sin();
        let b = y * 0.224_f32 - x.cos();
        let mut acc = Accumulator1153::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.352_f32 + y.sin();
        let b = y * 3.042_f32 - x.cos();
        let mut acc = Accumulator1153::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.942_f32 + y.sin();
        let b = y * 8.77_f32 - x.cos();
        let mut acc = Accumulator1153::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.473_f32 + y.sin();
        let b = y * 9.641_f32 - x.cos();
        let mut acc = Accumulator1153::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.987_f32 + y.sin();
        let b = y * 4.891_f32 - x.cos();
        let mut acc = Accumulator1153::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.924_f32 + y.sin();
        let b = y * 8.19_f32 - x.cos();
        let mut acc = Accumulator1153::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.318_f32 + y.sin();
        let b = y * 5.154_f32 - x.cos();
        let mut acc = Accumulator1153::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.323_f32 + y.sin();
        let b = y * 9.65_f32 - x.cos();
        let mut acc = Accumulator1153::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.791_f32 + y.sin();
        let b = y * 7.371_f32 - x.cos();
        let mut acc = Accumulator1153::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.756_f32 + y.sin();
        let b = y * 9.279_f32 - x.cos();
        let mut acc = Accumulator1153::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.191_f32 + y.sin();
        let b = y * 3.796_f32 - x.cos();
        let mut acc = Accumulator1153::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.26_f32 + y.sin();
        let b = y * 1.054_f32 - x.cos();
        let mut acc = Accumulator1153::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.785_f32 + y.sin();
        let b = y * 3.946_f32 - x.cos();
        let mut acc = Accumulator1153::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.224_f32 + y.sin();
        let b = y * 5.46_f32 - x.cos();
        let mut acc = Accumulator1153::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.811_f32 + y.sin();
        let b = y * 9.056_f32 - x.cos();
        let mut acc = Accumulator1153::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.977_f32 + y.sin();
        let b = y * 6.152_f32 - x.cos();
        let mut acc = Accumulator1153::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.465_f32 + y.sin();
        let b = y * 8.209_f32 - x.cos();
        let mut acc = Accumulator1153::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1153(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(1153u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_1153() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1153(total as u64) % 997) as f32;
        total
    }
}

pub mod m1154 {
    use super::*;

    pub struct Accumulator1154<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1154<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.748_f32 + y.sin();
        let b = y * 1.252_f32 - x.cos();
        let mut acc = Accumulator1154::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.052_f32 + y.sin();
        let b = y * 1.099_f32 - x.cos();
        let mut acc = Accumulator1154::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.862_f32 + y.sin();
        let b = y * 5.092_f32 - x.cos();
        let mut acc = Accumulator1154::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.2_f32 + y.sin();
        let b = y * 8.12_f32 - x.cos();
        let mut acc = Accumulator1154::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.797_f32 + y.sin();
        let b = y * 7.253_f32 - x.cos();
        let mut acc = Accumulator1154::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.889_f32 + y.sin();
        let b = y * 2.741_f32 - x.cos();
        let mut acc = Accumulator1154::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.495_f32 + y.sin();
        let b = y * 7.114_f32 - x.cos();
        let mut acc = Accumulator1154::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.888_f32 + y.sin();
        let b = y * 3.743_f32 - x.cos();
        let mut acc = Accumulator1154::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.76_f32 + y.sin();
        let b = y * 5.982_f32 - x.cos();
        let mut acc = Accumulator1154::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.873_f32 + y.sin();
        let b = y * 2.918_f32 - x.cos();
        let mut acc = Accumulator1154::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.183_f32 + y.sin();
        let b = y * 1.358_f32 - x.cos();
        let mut acc = Accumulator1154::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.518_f32 + y.sin();
        let b = y * 0.648_f32 - x.cos();
        let mut acc = Accumulator1154::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.577_f32 + y.sin();
        let b = y * 0.155_f32 - x.cos();
        let mut acc = Accumulator1154::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.696_f32 + y.sin();
        let b = y * 4.446_f32 - x.cos();
        let mut acc = Accumulator1154::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.815_f32 + y.sin();
        let b = y * 4.387_f32 - x.cos();
        let mut acc = Accumulator1154::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.175_f32 + y.sin();
        let b = y * 9.264_f32 - x.cos();
        let mut acc = Accumulator1154::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.091_f32 + y.sin();
        let b = y * 1.698_f32 - x.cos();
        let mut acc = Accumulator1154::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.442_f32 + y.sin();
        let b = y * 5.941_f32 - x.cos();
        let mut acc = Accumulator1154::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.655_f32 + y.sin();
        let b = y * 0.287_f32 - x.cos();
        let mut acc = Accumulator1154::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.901_f32 + y.sin();
        let b = y * 6.571_f32 - x.cos();
        let mut acc = Accumulator1154::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1154(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_1154() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1154(total as u64) % 997) as f32;
        total
    }
}

pub mod m1155 {
    use super::*;

    pub struct Accumulator1155<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1155<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.587_f32 + y.sin();
        let b = y * 5.659_f32 - x.cos();
        let mut acc = Accumulator1155::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.802_f32 + y.sin();
        let b = y * 4.413_f32 - x.cos();
        let mut acc = Accumulator1155::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.853_f32 + y.sin();
        let b = y * 8.8_f32 - x.cos();
        let mut acc = Accumulator1155::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.899_f32 + y.sin();
        let b = y * 6.218_f32 - x.cos();
        let mut acc = Accumulator1155::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.689_f32 + y.sin();
        let b = y * 6.156_f32 - x.cos();
        let mut acc = Accumulator1155::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.71_f32 + y.sin();
        let b = y * 7.185_f32 - x.cos();
        let mut acc = Accumulator1155::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.556_f32 + y.sin();
        let b = y * 5.032_f32 - x.cos();
        let mut acc = Accumulator1155::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.206_f32 + y.sin();
        let b = y * 5.465_f32 - x.cos();
        let mut acc = Accumulator1155::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.984_f32 + y.sin();
        let b = y * 1.859_f32 - x.cos();
        let mut acc = Accumulator1155::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.672_f32 + y.sin();
        let b = y * 8.385_f32 - x.cos();
        let mut acc = Accumulator1155::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.796_f32 + y.sin();
        let b = y * 2.33_f32 - x.cos();
        let mut acc = Accumulator1155::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.694_f32 + y.sin();
        let b = y * 6.141_f32 - x.cos();
        let mut acc = Accumulator1155::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.203_f32 + y.sin();
        let b = y * 1.888_f32 - x.cos();
        let mut acc = Accumulator1155::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.913_f32 + y.sin();
        let b = y * 8.573_f32 - x.cos();
        let mut acc = Accumulator1155::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.668_f32 + y.sin();
        let b = y * 8.456_f32 - x.cos();
        let mut acc = Accumulator1155::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.411_f32 + y.sin();
        let b = y * 3.896_f32 - x.cos();
        let mut acc = Accumulator1155::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.47_f32 + y.sin();
        let b = y * 1.811_f32 - x.cos();
        let mut acc = Accumulator1155::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.489_f32 + y.sin();
        let b = y * 6.14_f32 - x.cos();
        let mut acc = Accumulator1155::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.691_f32 + y.sin();
        let b = y * 1.341_f32 - x.cos();
        let mut acc = Accumulator1155::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.109_f32 + y.sin();
        let b = y * 4.309_f32 - x.cos();
        let mut acc = Accumulator1155::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1155(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_1155() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1155(total as u64) % 997) as f32;
        total
    }
}

pub mod m1156 {
    use super::*;

    pub struct Accumulator1156<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1156<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.005_f32 + y.sin();
        let b = y * 0.347_f32 - x.cos();
        let mut acc = Accumulator1156::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.688_f32 + y.sin();
        let b = y * 6.763_f32 - x.cos();
        let mut acc = Accumulator1156::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.491_f32 + y.sin();
        let b = y * 4.612_f32 - x.cos();
        let mut acc = Accumulator1156::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.46_f32 + y.sin();
        let b = y * 5.415_f32 - x.cos();
        let mut acc = Accumulator1156::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.51_f32 + y.sin();
        let b = y * 4.222_f32 - x.cos();
        let mut acc = Accumulator1156::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.281_f32 + y.sin();
        let b = y * 1.617_f32 - x.cos();
        let mut acc = Accumulator1156::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.339_f32 + y.sin();
        let b = y * 6.512_f32 - x.cos();
        let mut acc = Accumulator1156::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.777_f32 + y.sin();
        let b = y * 3.065_f32 - x.cos();
        let mut acc = Accumulator1156::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.084_f32 + y.sin();
        let b = y * 7.242_f32 - x.cos();
        let mut acc = Accumulator1156::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.075_f32 + y.sin();
        let b = y * 9.88_f32 - x.cos();
        let mut acc = Accumulator1156::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.166_f32 + y.sin();
        let b = y * 8.083_f32 - x.cos();
        let mut acc = Accumulator1156::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.038_f32 + y.sin();
        let b = y * 1.44_f32 - x.cos();
        let mut acc = Accumulator1156::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.643_f32 + y.sin();
        let b = y * 1.952_f32 - x.cos();
        let mut acc = Accumulator1156::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.968_f32 + y.sin();
        let b = y * 0.175_f32 - x.cos();
        let mut acc = Accumulator1156::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.143_f32 + y.sin();
        let b = y * 8.122_f32 - x.cos();
        let mut acc = Accumulator1156::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.197_f32 + y.sin();
        let b = y * 5.103_f32 - x.cos();
        let mut acc = Accumulator1156::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.073_f32 + y.sin();
        let b = y * 8.095_f32 - x.cos();
        let mut acc = Accumulator1156::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.435_f32 + y.sin();
        let b = y * 8.757_f32 - x.cos();
        let mut acc = Accumulator1156::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.931_f32 + y.sin();
        let b = y * 8.203_f32 - x.cos();
        let mut acc = Accumulator1156::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.689_f32 + y.sin();
        let b = y * 8.002_f32 - x.cos();
        let mut acc = Accumulator1156::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1156(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m1156-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_1156() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1156(total as u64) % 997) as f32;
        total
    }
}

pub mod m1157 {
    use super::*;

    pub struct Accumulator1157<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1157<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.944_f32 + y.sin();
        let b = y * 1.192_f32 - x.cos();
        let mut acc = Accumulator1157::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.884_f32 + y.sin();
        let b = y * 4.387_f32 - x.cos();
        let mut acc = Accumulator1157::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.592_f32 + y.sin();
        let b = y * 7.465_f32 - x.cos();
        let mut acc = Accumulator1157::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.708_f32 + y.sin();
        let b = y * 8.254_f32 - x.cos();
        let mut acc = Accumulator1157::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.929_f32 + y.sin();
        let b = y * 7.538_f32 - x.cos();
        let mut acc = Accumulator1157::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.797_f32 + y.sin();
        let b = y * 3.843_f32 - x.cos();
        let mut acc = Accumulator1157::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.292_f32 + y.sin();
        let b = y * 5.953_f32 - x.cos();
        let mut acc = Accumulator1157::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.886_f32 + y.sin();
        let b = y * 0.974_f32 - x.cos();
        let mut acc = Accumulator1157::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.084_f32 + y.sin();
        let b = y * 2.77_f32 - x.cos();
        let mut acc = Accumulator1157::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.649_f32 + y.sin();
        let b = y * 5.694_f32 - x.cos();
        let mut acc = Accumulator1157::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.041_f32 + y.sin();
        let b = y * 8.403_f32 - x.cos();
        let mut acc = Accumulator1157::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.12_f32 + y.sin();
        let b = y * 5.672_f32 - x.cos();
        let mut acc = Accumulator1157::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.959_f32 + y.sin();
        let b = y * 0.555_f32 - x.cos();
        let mut acc = Accumulator1157::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.728_f32 + y.sin();
        let b = y * 1.582_f32 - x.cos();
        let mut acc = Accumulator1157::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.934_f32 + y.sin();
        let b = y * 8.549_f32 - x.cos();
        let mut acc = Accumulator1157::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.143_f32 + y.sin();
        let b = y * 7.487_f32 - x.cos();
        let mut acc = Accumulator1157::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.86_f32 + y.sin();
        let b = y * 8.824_f32 - x.cos();
        let mut acc = Accumulator1157::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.641_f32 + y.sin();
        let b = y * 6.785_f32 - x.cos();
        let mut acc = Accumulator1157::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.542_f32 + y.sin();
        let b = y * 5.287_f32 - x.cos();
        let mut acc = Accumulator1157::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.839_f32 + y.sin();
        let b = y * 9.638_f32 - x.cos();
        let mut acc = Accumulator1157::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1157(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_1157() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1157(total as u64) % 997) as f32;
        total
    }
}

pub mod m1158 {
    use super::*;

    pub struct Accumulator1158<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1158<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.452_f32 + y.sin();
        let b = y * 1.781_f32 - x.cos();
        let mut acc = Accumulator1158::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.245_f32 + y.sin();
        let b = y * 9.793_f32 - x.cos();
        let mut acc = Accumulator1158::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.417_f32 + y.sin();
        let b = y * 3.666_f32 - x.cos();
        let mut acc = Accumulator1158::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.478_f32 + y.sin();
        let b = y * 0.995_f32 - x.cos();
        let mut acc = Accumulator1158::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.083_f32 + y.sin();
        let b = y * 8.99_f32 - x.cos();
        let mut acc = Accumulator1158::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.798_f32 + y.sin();
        let b = y * 0.686_f32 - x.cos();
        let mut acc = Accumulator1158::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.69_f32 + y.sin();
        let b = y * 7.857_f32 - x.cos();
        let mut acc = Accumulator1158::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.922_f32 + y.sin();
        let b = y * 3.436_f32 - x.cos();
        let mut acc = Accumulator1158::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.517_f32 + y.sin();
        let b = y * 3.174_f32 - x.cos();
        let mut acc = Accumulator1158::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.045_f32 + y.sin();
        let b = y * 8.197_f32 - x.cos();
        let mut acc = Accumulator1158::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.291_f32 + y.sin();
        let b = y * 9.685_f32 - x.cos();
        let mut acc = Accumulator1158::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.143_f32 + y.sin();
        let b = y * 8.014_f32 - x.cos();
        let mut acc = Accumulator1158::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.796_f32 + y.sin();
        let b = y * 0.555_f32 - x.cos();
        let mut acc = Accumulator1158::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.442_f32 + y.sin();
        let b = y * 6.648_f32 - x.cos();
        let mut acc = Accumulator1158::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.019_f32 + y.sin();
        let b = y * 5.068_f32 - x.cos();
        let mut acc = Accumulator1158::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.648_f32 + y.sin();
        let b = y * 9.305_f32 - x.cos();
        let mut acc = Accumulator1158::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.261_f32 + y.sin();
        let b = y * 5.574_f32 - x.cos();
        let mut acc = Accumulator1158::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.046_f32 + y.sin();
        let b = y * 1.275_f32 - x.cos();
        let mut acc = Accumulator1158::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.495_f32 + y.sin();
        let b = y * 3.402_f32 - x.cos();
        let mut acc = Accumulator1158::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.913_f32 + y.sin();
        let b = y * 5.671_f32 - x.cos();
        let mut acc = Accumulator1158::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1158(seed: u64) -> u64 {
        let re = Regex::new(r"m1158-(\d+)").unwrap();
        let hay = format!("m1158-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_1158() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1158(total as u64) % 997) as f32;
        total
    }
}

pub mod m1159 {
    use super::*;

    pub struct Accumulator1159<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1159<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.183_f32 + y.sin();
        let b = y * 5.182_f32 - x.cos();
        let mut acc = Accumulator1159::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.423_f32 + y.sin();
        let b = y * 8.405_f32 - x.cos();
        let mut acc = Accumulator1159::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.642_f32 + y.sin();
        let b = y * 2.275_f32 - x.cos();
        let mut acc = Accumulator1159::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.17_f32 + y.sin();
        let b = y * 1.729_f32 - x.cos();
        let mut acc = Accumulator1159::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.442_f32 + y.sin();
        let b = y * 3.179_f32 - x.cos();
        let mut acc = Accumulator1159::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.816_f32 + y.sin();
        let b = y * 2.942_f32 - x.cos();
        let mut acc = Accumulator1159::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.182_f32 + y.sin();
        let b = y * 9.027_f32 - x.cos();
        let mut acc = Accumulator1159::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.372_f32 + y.sin();
        let b = y * 3.62_f32 - x.cos();
        let mut acc = Accumulator1159::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.918_f32 + y.sin();
        let b = y * 0.369_f32 - x.cos();
        let mut acc = Accumulator1159::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.833_f32 + y.sin();
        let b = y * 4.196_f32 - x.cos();
        let mut acc = Accumulator1159::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.865_f32 + y.sin();
        let b = y * 6.004_f32 - x.cos();
        let mut acc = Accumulator1159::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.955_f32 + y.sin();
        let b = y * 5.375_f32 - x.cos();
        let mut acc = Accumulator1159::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.603_f32 + y.sin();
        let b = y * 0.12_f32 - x.cos();
        let mut acc = Accumulator1159::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.29_f32 + y.sin();
        let b = y * 9.804_f32 - x.cos();
        let mut acc = Accumulator1159::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.92_f32 + y.sin();
        let b = y * 3.42_f32 - x.cos();
        let mut acc = Accumulator1159::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.536_f32 + y.sin();
        let b = y * 1.256_f32 - x.cos();
        let mut acc = Accumulator1159::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.097_f32 + y.sin();
        let b = y * 6.411_f32 - x.cos();
        let mut acc = Accumulator1159::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.249_f32 + y.sin();
        let b = y * 2.947_f32 - x.cos();
        let mut acc = Accumulator1159::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.204_f32 + y.sin();
        let b = y * 1.487_f32 - x.cos();
        let mut acc = Accumulator1159::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.611_f32 + y.sin();
        let b = y * 0.466_f32 - x.cos();
        let mut acc = Accumulator1159::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1159(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_1159() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1159(total as u64) % 997) as f32;
        total
    }
}

pub mod m1160 {
    use super::*;

    pub struct Accumulator1160<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1160<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.486_f32 + y.sin();
        let b = y * 2.657_f32 - x.cos();
        let mut acc = Accumulator1160::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.266_f32 + y.sin();
        let b = y * 8.363_f32 - x.cos();
        let mut acc = Accumulator1160::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.861_f32 + y.sin();
        let b = y * 1.804_f32 - x.cos();
        let mut acc = Accumulator1160::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.188_f32 + y.sin();
        let b = y * 6.779_f32 - x.cos();
        let mut acc = Accumulator1160::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.74_f32 + y.sin();
        let b = y * 4.672_f32 - x.cos();
        let mut acc = Accumulator1160::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.705_f32 + y.sin();
        let b = y * 4.823_f32 - x.cos();
        let mut acc = Accumulator1160::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.222_f32 + y.sin();
        let b = y * 5.887_f32 - x.cos();
        let mut acc = Accumulator1160::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.766_f32 + y.sin();
        let b = y * 5.765_f32 - x.cos();
        let mut acc = Accumulator1160::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.889_f32 + y.sin();
        let b = y * 5.044_f32 - x.cos();
        let mut acc = Accumulator1160::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.397_f32 + y.sin();
        let b = y * 3.494_f32 - x.cos();
        let mut acc = Accumulator1160::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.811_f32 + y.sin();
        let b = y * 9.125_f32 - x.cos();
        let mut acc = Accumulator1160::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.446_f32 + y.sin();
        let b = y * 2.377_f32 - x.cos();
        let mut acc = Accumulator1160::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.857_f32 + y.sin();
        let b = y * 0.964_f32 - x.cos();
        let mut acc = Accumulator1160::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.792_f32 + y.sin();
        let b = y * 8.499_f32 - x.cos();
        let mut acc = Accumulator1160::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.628_f32 + y.sin();
        let b = y * 2.421_f32 - x.cos();
        let mut acc = Accumulator1160::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.016_f32 + y.sin();
        let b = y * 5.618_f32 - x.cos();
        let mut acc = Accumulator1160::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.83_f32 + y.sin();
        let b = y * 8.812_f32 - x.cos();
        let mut acc = Accumulator1160::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.119_f32 + y.sin();
        let b = y * 2.647_f32 - x.cos();
        let mut acc = Accumulator1160::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.229_f32 + y.sin();
        let b = y * 2.417_f32 - x.cos();
        let mut acc = Accumulator1160::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.97_f32 + y.sin();
        let b = y * 8.255_f32 - x.cos();
        let mut acc = Accumulator1160::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1160(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(1160u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_1160() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1160(total as u64) % 997) as f32;
        total
    }
}

pub mod m1161 {
    use super::*;

    pub struct Accumulator1161<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1161<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.475_f32 + y.sin();
        let b = y * 1.939_f32 - x.cos();
        let mut acc = Accumulator1161::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.065_f32 + y.sin();
        let b = y * 8.436_f32 - x.cos();
        let mut acc = Accumulator1161::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.554_f32 + y.sin();
        let b = y * 0.162_f32 - x.cos();
        let mut acc = Accumulator1161::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.789_f32 + y.sin();
        let b = y * 6.718_f32 - x.cos();
        let mut acc = Accumulator1161::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.731_f32 + y.sin();
        let b = y * 8.811_f32 - x.cos();
        let mut acc = Accumulator1161::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.735_f32 + y.sin();
        let b = y * 8.16_f32 - x.cos();
        let mut acc = Accumulator1161::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.932_f32 + y.sin();
        let b = y * 7.904_f32 - x.cos();
        let mut acc = Accumulator1161::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.119_f32 + y.sin();
        let b = y * 2.481_f32 - x.cos();
        let mut acc = Accumulator1161::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.181_f32 + y.sin();
        let b = y * 1.753_f32 - x.cos();
        let mut acc = Accumulator1161::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.667_f32 + y.sin();
        let b = y * 8.604_f32 - x.cos();
        let mut acc = Accumulator1161::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.653_f32 + y.sin();
        let b = y * 9.021_f32 - x.cos();
        let mut acc = Accumulator1161::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.901_f32 + y.sin();
        let b = y * 7.08_f32 - x.cos();
        let mut acc = Accumulator1161::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.992_f32 + y.sin();
        let b = y * 1.573_f32 - x.cos();
        let mut acc = Accumulator1161::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.864_f32 + y.sin();
        let b = y * 4.436_f32 - x.cos();
        let mut acc = Accumulator1161::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.881_f32 + y.sin();
        let b = y * 5.559_f32 - x.cos();
        let mut acc = Accumulator1161::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.114_f32 + y.sin();
        let b = y * 7.635_f32 - x.cos();
        let mut acc = Accumulator1161::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.92_f32 + y.sin();
        let b = y * 6.445_f32 - x.cos();
        let mut acc = Accumulator1161::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.144_f32 + y.sin();
        let b = y * 4.545_f32 - x.cos();
        let mut acc = Accumulator1161::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.863_f32 + y.sin();
        let b = y * 9.128_f32 - x.cos();
        let mut acc = Accumulator1161::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.312_f32 + y.sin();
        let b = y * 0.93_f32 - x.cos();
        let mut acc = Accumulator1161::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1161(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_1161() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1161(total as u64) % 997) as f32;
        total
    }
}

pub mod m1162 {
    use super::*;

    pub struct Accumulator1162<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1162<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.297_f32 + y.sin();
        let b = y * 9.385_f32 - x.cos();
        let mut acc = Accumulator1162::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.542_f32 + y.sin();
        let b = y * 3.778_f32 - x.cos();
        let mut acc = Accumulator1162::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.372_f32 + y.sin();
        let b = y * 1.152_f32 - x.cos();
        let mut acc = Accumulator1162::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.752_f32 + y.sin();
        let b = y * 9.643_f32 - x.cos();
        let mut acc = Accumulator1162::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.411_f32 + y.sin();
        let b = y * 2.249_f32 - x.cos();
        let mut acc = Accumulator1162::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.209_f32 + y.sin();
        let b = y * 2.769_f32 - x.cos();
        let mut acc = Accumulator1162::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.889_f32 + y.sin();
        let b = y * 9.744_f32 - x.cos();
        let mut acc = Accumulator1162::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.319_f32 + y.sin();
        let b = y * 2.32_f32 - x.cos();
        let mut acc = Accumulator1162::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.937_f32 + y.sin();
        let b = y * 8.375_f32 - x.cos();
        let mut acc = Accumulator1162::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.233_f32 + y.sin();
        let b = y * 5.625_f32 - x.cos();
        let mut acc = Accumulator1162::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.148_f32 + y.sin();
        let b = y * 0.755_f32 - x.cos();
        let mut acc = Accumulator1162::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.423_f32 + y.sin();
        let b = y * 8.632_f32 - x.cos();
        let mut acc = Accumulator1162::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.692_f32 + y.sin();
        let b = y * 8.669_f32 - x.cos();
        let mut acc = Accumulator1162::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.106_f32 + y.sin();
        let b = y * 2.224_f32 - x.cos();
        let mut acc = Accumulator1162::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.913_f32 + y.sin();
        let b = y * 5.372_f32 - x.cos();
        let mut acc = Accumulator1162::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.889_f32 + y.sin();
        let b = y * 2.225_f32 - x.cos();
        let mut acc = Accumulator1162::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.729_f32 + y.sin();
        let b = y * 7.477_f32 - x.cos();
        let mut acc = Accumulator1162::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.75_f32 + y.sin();
        let b = y * 4.796_f32 - x.cos();
        let mut acc = Accumulator1162::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.38_f32 + y.sin();
        let b = y * 4.549_f32 - x.cos();
        let mut acc = Accumulator1162::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.797_f32 + y.sin();
        let b = y * 8.815_f32 - x.cos();
        let mut acc = Accumulator1162::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1162(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_1162() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1162(total as u64) % 997) as f32;
        total
    }
}

pub mod m1163 {
    use super::*;

    pub struct Accumulator1163<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1163<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.899_f32 + y.sin();
        let b = y * 8.282_f32 - x.cos();
        let mut acc = Accumulator1163::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.644_f32 + y.sin();
        let b = y * 2.752_f32 - x.cos();
        let mut acc = Accumulator1163::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.25_f32 + y.sin();
        let b = y * 2.833_f32 - x.cos();
        let mut acc = Accumulator1163::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.839_f32 + y.sin();
        let b = y * 5.353_f32 - x.cos();
        let mut acc = Accumulator1163::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.949_f32 + y.sin();
        let b = y * 2.88_f32 - x.cos();
        let mut acc = Accumulator1163::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.873_f32 + y.sin();
        let b = y * 5.58_f32 - x.cos();
        let mut acc = Accumulator1163::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.491_f32 + y.sin();
        let b = y * 7.164_f32 - x.cos();
        let mut acc = Accumulator1163::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.133_f32 + y.sin();
        let b = y * 9.24_f32 - x.cos();
        let mut acc = Accumulator1163::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.79_f32 + y.sin();
        let b = y * 6.63_f32 - x.cos();
        let mut acc = Accumulator1163::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.366_f32 + y.sin();
        let b = y * 9.293_f32 - x.cos();
        let mut acc = Accumulator1163::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.274_f32 + y.sin();
        let b = y * 7.991_f32 - x.cos();
        let mut acc = Accumulator1163::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.15_f32 + y.sin();
        let b = y * 7.252_f32 - x.cos();
        let mut acc = Accumulator1163::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.464_f32 + y.sin();
        let b = y * 7.888_f32 - x.cos();
        let mut acc = Accumulator1163::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.565_f32 + y.sin();
        let b = y * 4.766_f32 - x.cos();
        let mut acc = Accumulator1163::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.011_f32 + y.sin();
        let b = y * 2.784_f32 - x.cos();
        let mut acc = Accumulator1163::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.571_f32 + y.sin();
        let b = y * 9.479_f32 - x.cos();
        let mut acc = Accumulator1163::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.464_f32 + y.sin();
        let b = y * 2.66_f32 - x.cos();
        let mut acc = Accumulator1163::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.113_f32 + y.sin();
        let b = y * 9.711_f32 - x.cos();
        let mut acc = Accumulator1163::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.801_f32 + y.sin();
        let b = y * 6.082_f32 - x.cos();
        let mut acc = Accumulator1163::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.225_f32 + y.sin();
        let b = y * 3.624_f32 - x.cos();
        let mut acc = Accumulator1163::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1163(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m1163-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_1163() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1163(total as u64) % 997) as f32;
        total
    }
}

pub mod m1164 {
    use super::*;

    pub struct Accumulator1164<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1164<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.372_f32 + y.sin();
        let b = y * 8.839_f32 - x.cos();
        let mut acc = Accumulator1164::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.799_f32 + y.sin();
        let b = y * 4.197_f32 - x.cos();
        let mut acc = Accumulator1164::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.676_f32 + y.sin();
        let b = y * 7.017_f32 - x.cos();
        let mut acc = Accumulator1164::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.947_f32 + y.sin();
        let b = y * 6.977_f32 - x.cos();
        let mut acc = Accumulator1164::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.621_f32 + y.sin();
        let b = y * 2.128_f32 - x.cos();
        let mut acc = Accumulator1164::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.022_f32 + y.sin();
        let b = y * 3.065_f32 - x.cos();
        let mut acc = Accumulator1164::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.232_f32 + y.sin();
        let b = y * 5.051_f32 - x.cos();
        let mut acc = Accumulator1164::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.806_f32 + y.sin();
        let b = y * 1.726_f32 - x.cos();
        let mut acc = Accumulator1164::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.742_f32 + y.sin();
        let b = y * 8.878_f32 - x.cos();
        let mut acc = Accumulator1164::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.339_f32 + y.sin();
        let b = y * 5.972_f32 - x.cos();
        let mut acc = Accumulator1164::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.182_f32 + y.sin();
        let b = y * 1.727_f32 - x.cos();
        let mut acc = Accumulator1164::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.218_f32 + y.sin();
        let b = y * 7.274_f32 - x.cos();
        let mut acc = Accumulator1164::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.336_f32 + y.sin();
        let b = y * 2.633_f32 - x.cos();
        let mut acc = Accumulator1164::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.658_f32 + y.sin();
        let b = y * 6.799_f32 - x.cos();
        let mut acc = Accumulator1164::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.962_f32 + y.sin();
        let b = y * 8.351_f32 - x.cos();
        let mut acc = Accumulator1164::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.704_f32 + y.sin();
        let b = y * 5.353_f32 - x.cos();
        let mut acc = Accumulator1164::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.569_f32 + y.sin();
        let b = y * 7.988_f32 - x.cos();
        let mut acc = Accumulator1164::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.529_f32 + y.sin();
        let b = y * 3.315_f32 - x.cos();
        let mut acc = Accumulator1164::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.469_f32 + y.sin();
        let b = y * 5.235_f32 - x.cos();
        let mut acc = Accumulator1164::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.498_f32 + y.sin();
        let b = y * 0.766_f32 - x.cos();
        let mut acc = Accumulator1164::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1164(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_1164() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1164(total as u64) % 997) as f32;
        total
    }
}

pub mod m1165 {
    use super::*;

    pub struct Accumulator1165<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1165<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.946_f32 + y.sin();
        let b = y * 7.817_f32 - x.cos();
        let mut acc = Accumulator1165::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.665_f32 + y.sin();
        let b = y * 3.543_f32 - x.cos();
        let mut acc = Accumulator1165::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.041_f32 + y.sin();
        let b = y * 3.666_f32 - x.cos();
        let mut acc = Accumulator1165::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.582_f32 + y.sin();
        let b = y * 3.394_f32 - x.cos();
        let mut acc = Accumulator1165::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.849_f32 + y.sin();
        let b = y * 6.079_f32 - x.cos();
        let mut acc = Accumulator1165::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.989_f32 + y.sin();
        let b = y * 4.601_f32 - x.cos();
        let mut acc = Accumulator1165::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.121_f32 + y.sin();
        let b = y * 0.592_f32 - x.cos();
        let mut acc = Accumulator1165::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.585_f32 + y.sin();
        let b = y * 7.356_f32 - x.cos();
        let mut acc = Accumulator1165::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.31_f32 + y.sin();
        let b = y * 3.535_f32 - x.cos();
        let mut acc = Accumulator1165::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.369_f32 + y.sin();
        let b = y * 4.425_f32 - x.cos();
        let mut acc = Accumulator1165::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.188_f32 + y.sin();
        let b = y * 0.113_f32 - x.cos();
        let mut acc = Accumulator1165::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.611_f32 + y.sin();
        let b = y * 0.491_f32 - x.cos();
        let mut acc = Accumulator1165::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.909_f32 + y.sin();
        let b = y * 6.441_f32 - x.cos();
        let mut acc = Accumulator1165::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.608_f32 + y.sin();
        let b = y * 4.173_f32 - x.cos();
        let mut acc = Accumulator1165::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.804_f32 + y.sin();
        let b = y * 0.409_f32 - x.cos();
        let mut acc = Accumulator1165::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.64_f32 + y.sin();
        let b = y * 2.177_f32 - x.cos();
        let mut acc = Accumulator1165::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.534_f32 + y.sin();
        let b = y * 1.25_f32 - x.cos();
        let mut acc = Accumulator1165::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.075_f32 + y.sin();
        let b = y * 5.159_f32 - x.cos();
        let mut acc = Accumulator1165::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.274_f32 + y.sin();
        let b = y * 1.987_f32 - x.cos();
        let mut acc = Accumulator1165::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.284_f32 + y.sin();
        let b = y * 1.797_f32 - x.cos();
        let mut acc = Accumulator1165::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1165(seed: u64) -> u64 {
        let re = Regex::new(r"m1165-(\d+)").unwrap();
        let hay = format!("m1165-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_1165() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1165(total as u64) % 997) as f32;
        total
    }
}

pub mod m1166 {
    use super::*;

    pub struct Accumulator1166<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1166<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.749_f32 + y.sin();
        let b = y * 8.906_f32 - x.cos();
        let mut acc = Accumulator1166::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.559_f32 + y.sin();
        let b = y * 5.504_f32 - x.cos();
        let mut acc = Accumulator1166::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.41_f32 + y.sin();
        let b = y * 4.998_f32 - x.cos();
        let mut acc = Accumulator1166::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.679_f32 + y.sin();
        let b = y * 9.672_f32 - x.cos();
        let mut acc = Accumulator1166::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.429_f32 + y.sin();
        let b = y * 2.644_f32 - x.cos();
        let mut acc = Accumulator1166::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.413_f32 + y.sin();
        let b = y * 5.186_f32 - x.cos();
        let mut acc = Accumulator1166::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.595_f32 + y.sin();
        let b = y * 1.944_f32 - x.cos();
        let mut acc = Accumulator1166::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.496_f32 + y.sin();
        let b = y * 1.889_f32 - x.cos();
        let mut acc = Accumulator1166::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.408_f32 + y.sin();
        let b = y * 6.706_f32 - x.cos();
        let mut acc = Accumulator1166::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.075_f32 + y.sin();
        let b = y * 9.705_f32 - x.cos();
        let mut acc = Accumulator1166::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.452_f32 + y.sin();
        let b = y * 4.326_f32 - x.cos();
        let mut acc = Accumulator1166::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.425_f32 + y.sin();
        let b = y * 5.085_f32 - x.cos();
        let mut acc = Accumulator1166::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.172_f32 + y.sin();
        let b = y * 6.025_f32 - x.cos();
        let mut acc = Accumulator1166::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.424_f32 + y.sin();
        let b = y * 6.67_f32 - x.cos();
        let mut acc = Accumulator1166::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.422_f32 + y.sin();
        let b = y * 8.443_f32 - x.cos();
        let mut acc = Accumulator1166::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.385_f32 + y.sin();
        let b = y * 4.904_f32 - x.cos();
        let mut acc = Accumulator1166::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.406_f32 + y.sin();
        let b = y * 2.443_f32 - x.cos();
        let mut acc = Accumulator1166::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.565_f32 + y.sin();
        let b = y * 6.549_f32 - x.cos();
        let mut acc = Accumulator1166::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.661_f32 + y.sin();
        let b = y * 5.15_f32 - x.cos();
        let mut acc = Accumulator1166::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.447_f32 + y.sin();
        let b = y * 8.255_f32 - x.cos();
        let mut acc = Accumulator1166::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1166(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_1166() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1166(total as u64) % 997) as f32;
        total
    }
}

pub mod m1167 {
    use super::*;

    pub struct Accumulator1167<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1167<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.589_f32 + y.sin();
        let b = y * 4.648_f32 - x.cos();
        let mut acc = Accumulator1167::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.369_f32 + y.sin();
        let b = y * 0.649_f32 - x.cos();
        let mut acc = Accumulator1167::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.344_f32 + y.sin();
        let b = y * 5.982_f32 - x.cos();
        let mut acc = Accumulator1167::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.382_f32 + y.sin();
        let b = y * 6.777_f32 - x.cos();
        let mut acc = Accumulator1167::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.05_f32 + y.sin();
        let b = y * 3.332_f32 - x.cos();
        let mut acc = Accumulator1167::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.3_f32 + y.sin();
        let b = y * 8.12_f32 - x.cos();
        let mut acc = Accumulator1167::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.252_f32 + y.sin();
        let b = y * 2.798_f32 - x.cos();
        let mut acc = Accumulator1167::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.334_f32 + y.sin();
        let b = y * 4.875_f32 - x.cos();
        let mut acc = Accumulator1167::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.94_f32 + y.sin();
        let b = y * 4.502_f32 - x.cos();
        let mut acc = Accumulator1167::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.339_f32 + y.sin();
        let b = y * 0.334_f32 - x.cos();
        let mut acc = Accumulator1167::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.886_f32 + y.sin();
        let b = y * 1.88_f32 - x.cos();
        let mut acc = Accumulator1167::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.869_f32 + y.sin();
        let b = y * 4.354_f32 - x.cos();
        let mut acc = Accumulator1167::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.651_f32 + y.sin();
        let b = y * 5.948_f32 - x.cos();
        let mut acc = Accumulator1167::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.308_f32 + y.sin();
        let b = y * 3.118_f32 - x.cos();
        let mut acc = Accumulator1167::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.334_f32 + y.sin();
        let b = y * 5.461_f32 - x.cos();
        let mut acc = Accumulator1167::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.474_f32 + y.sin();
        let b = y * 3.387_f32 - x.cos();
        let mut acc = Accumulator1167::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.082_f32 + y.sin();
        let b = y * 2.792_f32 - x.cos();
        let mut acc = Accumulator1167::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.616_f32 + y.sin();
        let b = y * 0.379_f32 - x.cos();
        let mut acc = Accumulator1167::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.258_f32 + y.sin();
        let b = y * 6.132_f32 - x.cos();
        let mut acc = Accumulator1167::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.144_f32 + y.sin();
        let b = y * 4.618_f32 - x.cos();
        let mut acc = Accumulator1167::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1167(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(1167u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_1167() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1167(total as u64) % 997) as f32;
        total
    }
}

pub mod m1168 {
    use super::*;

    pub struct Accumulator1168<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1168<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.748_f32 + y.sin();
        let b = y * 8.796_f32 - x.cos();
        let mut acc = Accumulator1168::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.618_f32 + y.sin();
        let b = y * 1.56_f32 - x.cos();
        let mut acc = Accumulator1168::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.857_f32 + y.sin();
        let b = y * 2.126_f32 - x.cos();
        let mut acc = Accumulator1168::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.782_f32 + y.sin();
        let b = y * 6.889_f32 - x.cos();
        let mut acc = Accumulator1168::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.645_f32 + y.sin();
        let b = y * 3.686_f32 - x.cos();
        let mut acc = Accumulator1168::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.644_f32 + y.sin();
        let b = y * 2.676_f32 - x.cos();
        let mut acc = Accumulator1168::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.647_f32 + y.sin();
        let b = y * 9.198_f32 - x.cos();
        let mut acc = Accumulator1168::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.787_f32 + y.sin();
        let b = y * 0.33_f32 - x.cos();
        let mut acc = Accumulator1168::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.317_f32 + y.sin();
        let b = y * 9.145_f32 - x.cos();
        let mut acc = Accumulator1168::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.934_f32 + y.sin();
        let b = y * 2.768_f32 - x.cos();
        let mut acc = Accumulator1168::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.343_f32 + y.sin();
        let b = y * 2.681_f32 - x.cos();
        let mut acc = Accumulator1168::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.631_f32 + y.sin();
        let b = y * 7.505_f32 - x.cos();
        let mut acc = Accumulator1168::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.493_f32 + y.sin();
        let b = y * 3.241_f32 - x.cos();
        let mut acc = Accumulator1168::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.219_f32 + y.sin();
        let b = y * 2.825_f32 - x.cos();
        let mut acc = Accumulator1168::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.037_f32 + y.sin();
        let b = y * 6.103_f32 - x.cos();
        let mut acc = Accumulator1168::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.086_f32 + y.sin();
        let b = y * 4.24_f32 - x.cos();
        let mut acc = Accumulator1168::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.784_f32 + y.sin();
        let b = y * 8.972_f32 - x.cos();
        let mut acc = Accumulator1168::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.904_f32 + y.sin();
        let b = y * 7.183_f32 - x.cos();
        let mut acc = Accumulator1168::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.405_f32 + y.sin();
        let b = y * 6.565_f32 - x.cos();
        let mut acc = Accumulator1168::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.936_f32 + y.sin();
        let b = y * 5.642_f32 - x.cos();
        let mut acc = Accumulator1168::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1168(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_1168() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1168(total as u64) % 997) as f32;
        total
    }
}

pub mod m1169 {
    use super::*;

    pub struct Accumulator1169<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1169<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.056_f32 + y.sin();
        let b = y * 7.238_f32 - x.cos();
        let mut acc = Accumulator1169::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.805_f32 + y.sin();
        let b = y * 0.615_f32 - x.cos();
        let mut acc = Accumulator1169::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.732_f32 + y.sin();
        let b = y * 2.666_f32 - x.cos();
        let mut acc = Accumulator1169::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.748_f32 + y.sin();
        let b = y * 2.028_f32 - x.cos();
        let mut acc = Accumulator1169::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.461_f32 + y.sin();
        let b = y * 7.179_f32 - x.cos();
        let mut acc = Accumulator1169::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.09_f32 + y.sin();
        let b = y * 8.187_f32 - x.cos();
        let mut acc = Accumulator1169::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.54_f32 + y.sin();
        let b = y * 2.481_f32 - x.cos();
        let mut acc = Accumulator1169::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.368_f32 + y.sin();
        let b = y * 9.741_f32 - x.cos();
        let mut acc = Accumulator1169::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.482_f32 + y.sin();
        let b = y * 4.591_f32 - x.cos();
        let mut acc = Accumulator1169::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.888_f32 + y.sin();
        let b = y * 3.312_f32 - x.cos();
        let mut acc = Accumulator1169::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.22_f32 + y.sin();
        let b = y * 4.151_f32 - x.cos();
        let mut acc = Accumulator1169::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.531_f32 + y.sin();
        let b = y * 8.154_f32 - x.cos();
        let mut acc = Accumulator1169::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.121_f32 + y.sin();
        let b = y * 4.521_f32 - x.cos();
        let mut acc = Accumulator1169::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.594_f32 + y.sin();
        let b = y * 7.224_f32 - x.cos();
        let mut acc = Accumulator1169::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.534_f32 + y.sin();
        let b = y * 0.577_f32 - x.cos();
        let mut acc = Accumulator1169::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.027_f32 + y.sin();
        let b = y * 8.857_f32 - x.cos();
        let mut acc = Accumulator1169::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.517_f32 + y.sin();
        let b = y * 7.644_f32 - x.cos();
        let mut acc = Accumulator1169::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.038_f32 + y.sin();
        let b = y * 6.078_f32 - x.cos();
        let mut acc = Accumulator1169::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.129_f32 + y.sin();
        let b = y * 4.33_f32 - x.cos();
        let mut acc = Accumulator1169::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.904_f32 + y.sin();
        let b = y * 1.939_f32 - x.cos();
        let mut acc = Accumulator1169::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1169(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_1169() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1169(total as u64) % 997) as f32;
        total
    }
}

pub mod m1170 {
    use super::*;

    pub struct Accumulator1170<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1170<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.605_f32 + y.sin();
        let b = y * 0.641_f32 - x.cos();
        let mut acc = Accumulator1170::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.711_f32 + y.sin();
        let b = y * 3.185_f32 - x.cos();
        let mut acc = Accumulator1170::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.076_f32 + y.sin();
        let b = y * 4.093_f32 - x.cos();
        let mut acc = Accumulator1170::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.65_f32 + y.sin();
        let b = y * 1.765_f32 - x.cos();
        let mut acc = Accumulator1170::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.804_f32 + y.sin();
        let b = y * 0.694_f32 - x.cos();
        let mut acc = Accumulator1170::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.823_f32 + y.sin();
        let b = y * 9.406_f32 - x.cos();
        let mut acc = Accumulator1170::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.063_f32 + y.sin();
        let b = y * 2.785_f32 - x.cos();
        let mut acc = Accumulator1170::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.298_f32 + y.sin();
        let b = y * 6.323_f32 - x.cos();
        let mut acc = Accumulator1170::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.855_f32 + y.sin();
        let b = y * 4.668_f32 - x.cos();
        let mut acc = Accumulator1170::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.565_f32 + y.sin();
        let b = y * 3.777_f32 - x.cos();
        let mut acc = Accumulator1170::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.608_f32 + y.sin();
        let b = y * 6.348_f32 - x.cos();
        let mut acc = Accumulator1170::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.819_f32 + y.sin();
        let b = y * 9.399_f32 - x.cos();
        let mut acc = Accumulator1170::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.529_f32 + y.sin();
        let b = y * 3.159_f32 - x.cos();
        let mut acc = Accumulator1170::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.851_f32 + y.sin();
        let b = y * 7.83_f32 - x.cos();
        let mut acc = Accumulator1170::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.013_f32 + y.sin();
        let b = y * 0.598_f32 - x.cos();
        let mut acc = Accumulator1170::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.818_f32 + y.sin();
        let b = y * 9.335_f32 - x.cos();
        let mut acc = Accumulator1170::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.503_f32 + y.sin();
        let b = y * 7.966_f32 - x.cos();
        let mut acc = Accumulator1170::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.187_f32 + y.sin();
        let b = y * 8.891_f32 - x.cos();
        let mut acc = Accumulator1170::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.889_f32 + y.sin();
        let b = y * 8.431_f32 - x.cos();
        let mut acc = Accumulator1170::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.957_f32 + y.sin();
        let b = y * 3.984_f32 - x.cos();
        let mut acc = Accumulator1170::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1170(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m1170-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_1170() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1170(total as u64) % 997) as f32;
        total
    }
}

pub mod m1171 {
    use super::*;

    pub struct Accumulator1171<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1171<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.066_f32 + y.sin();
        let b = y * 2.066_f32 - x.cos();
        let mut acc = Accumulator1171::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.817_f32 + y.sin();
        let b = y * 3.029_f32 - x.cos();
        let mut acc = Accumulator1171::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.355_f32 + y.sin();
        let b = y * 2.92_f32 - x.cos();
        let mut acc = Accumulator1171::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.662_f32 + y.sin();
        let b = y * 6.572_f32 - x.cos();
        let mut acc = Accumulator1171::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.796_f32 + y.sin();
        let b = y * 1.702_f32 - x.cos();
        let mut acc = Accumulator1171::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.254_f32 + y.sin();
        let b = y * 7.608_f32 - x.cos();
        let mut acc = Accumulator1171::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.607_f32 + y.sin();
        let b = y * 7.667_f32 - x.cos();
        let mut acc = Accumulator1171::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.498_f32 + y.sin();
        let b = y * 7.46_f32 - x.cos();
        let mut acc = Accumulator1171::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.083_f32 + y.sin();
        let b = y * 2.303_f32 - x.cos();
        let mut acc = Accumulator1171::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.07_f32 + y.sin();
        let b = y * 8.828_f32 - x.cos();
        let mut acc = Accumulator1171::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.643_f32 + y.sin();
        let b = y * 3.708_f32 - x.cos();
        let mut acc = Accumulator1171::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.442_f32 + y.sin();
        let b = y * 2.939_f32 - x.cos();
        let mut acc = Accumulator1171::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.767_f32 + y.sin();
        let b = y * 2.504_f32 - x.cos();
        let mut acc = Accumulator1171::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.244_f32 + y.sin();
        let b = y * 2.71_f32 - x.cos();
        let mut acc = Accumulator1171::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.302_f32 + y.sin();
        let b = y * 0.542_f32 - x.cos();
        let mut acc = Accumulator1171::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.71_f32 + y.sin();
        let b = y * 1.544_f32 - x.cos();
        let mut acc = Accumulator1171::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.06_f32 + y.sin();
        let b = y * 8.919_f32 - x.cos();
        let mut acc = Accumulator1171::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.084_f32 + y.sin();
        let b = y * 0.254_f32 - x.cos();
        let mut acc = Accumulator1171::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.714_f32 + y.sin();
        let b = y * 9.729_f32 - x.cos();
        let mut acc = Accumulator1171::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.561_f32 + y.sin();
        let b = y * 5.56_f32 - x.cos();
        let mut acc = Accumulator1171::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1171(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_1171() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1171(total as u64) % 997) as f32;
        total
    }
}

pub mod m1172 {
    use super::*;

    pub struct Accumulator1172<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1172<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.243_f32 + y.sin();
        let b = y * 6.982_f32 - x.cos();
        let mut acc = Accumulator1172::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.792_f32 + y.sin();
        let b = y * 5.216_f32 - x.cos();
        let mut acc = Accumulator1172::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.635_f32 + y.sin();
        let b = y * 7.398_f32 - x.cos();
        let mut acc = Accumulator1172::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.053_f32 + y.sin();
        let b = y * 2.445_f32 - x.cos();
        let mut acc = Accumulator1172::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.111_f32 + y.sin();
        let b = y * 7.186_f32 - x.cos();
        let mut acc = Accumulator1172::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.364_f32 + y.sin();
        let b = y * 5.365_f32 - x.cos();
        let mut acc = Accumulator1172::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.002_f32 + y.sin();
        let b = y * 3.28_f32 - x.cos();
        let mut acc = Accumulator1172::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.127_f32 + y.sin();
        let b = y * 9.323_f32 - x.cos();
        let mut acc = Accumulator1172::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.162_f32 + y.sin();
        let b = y * 2.337_f32 - x.cos();
        let mut acc = Accumulator1172::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.746_f32 + y.sin();
        let b = y * 3.02_f32 - x.cos();
        let mut acc = Accumulator1172::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.042_f32 + y.sin();
        let b = y * 3.342_f32 - x.cos();
        let mut acc = Accumulator1172::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.826_f32 + y.sin();
        let b = y * 9.255_f32 - x.cos();
        let mut acc = Accumulator1172::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.579_f32 + y.sin();
        let b = y * 6.492_f32 - x.cos();
        let mut acc = Accumulator1172::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.331_f32 + y.sin();
        let b = y * 5.563_f32 - x.cos();
        let mut acc = Accumulator1172::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.933_f32 + y.sin();
        let b = y * 0.119_f32 - x.cos();
        let mut acc = Accumulator1172::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.812_f32 + y.sin();
        let b = y * 0.537_f32 - x.cos();
        let mut acc = Accumulator1172::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.772_f32 + y.sin();
        let b = y * 4.873_f32 - x.cos();
        let mut acc = Accumulator1172::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.621_f32 + y.sin();
        let b = y * 2.074_f32 - x.cos();
        let mut acc = Accumulator1172::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.263_f32 + y.sin();
        let b = y * 8.959_f32 - x.cos();
        let mut acc = Accumulator1172::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.535_f32 + y.sin();
        let b = y * 9.776_f32 - x.cos();
        let mut acc = Accumulator1172::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1172(seed: u64) -> u64 {
        let re = Regex::new(r"m1172-(\d+)").unwrap();
        let hay = format!("m1172-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_1172() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1172(total as u64) % 997) as f32;
        total
    }
}

pub mod m1173 {
    use super::*;

    pub struct Accumulator1173<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1173<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.764_f32 + y.sin();
        let b = y * 9.451_f32 - x.cos();
        let mut acc = Accumulator1173::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.873_f32 + y.sin();
        let b = y * 4.063_f32 - x.cos();
        let mut acc = Accumulator1173::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.768_f32 + y.sin();
        let b = y * 9.028_f32 - x.cos();
        let mut acc = Accumulator1173::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.062_f32 + y.sin();
        let b = y * 7.509_f32 - x.cos();
        let mut acc = Accumulator1173::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.575_f32 + y.sin();
        let b = y * 0.42_f32 - x.cos();
        let mut acc = Accumulator1173::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.494_f32 + y.sin();
        let b = y * 1.718_f32 - x.cos();
        let mut acc = Accumulator1173::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.115_f32 + y.sin();
        let b = y * 3.441_f32 - x.cos();
        let mut acc = Accumulator1173::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.051_f32 + y.sin();
        let b = y * 2.701_f32 - x.cos();
        let mut acc = Accumulator1173::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.094_f32 + y.sin();
        let b = y * 7.037_f32 - x.cos();
        let mut acc = Accumulator1173::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.099_f32 + y.sin();
        let b = y * 4.45_f32 - x.cos();
        let mut acc = Accumulator1173::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.86_f32 + y.sin();
        let b = y * 8.458_f32 - x.cos();
        let mut acc = Accumulator1173::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.437_f32 + y.sin();
        let b = y * 4.437_f32 - x.cos();
        let mut acc = Accumulator1173::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.486_f32 + y.sin();
        let b = y * 0.701_f32 - x.cos();
        let mut acc = Accumulator1173::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.122_f32 + y.sin();
        let b = y * 4.674_f32 - x.cos();
        let mut acc = Accumulator1173::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.767_f32 + y.sin();
        let b = y * 8.621_f32 - x.cos();
        let mut acc = Accumulator1173::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.762_f32 + y.sin();
        let b = y * 3.81_f32 - x.cos();
        let mut acc = Accumulator1173::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.706_f32 + y.sin();
        let b = y * 9.174_f32 - x.cos();
        let mut acc = Accumulator1173::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.334_f32 + y.sin();
        let b = y * 6.033_f32 - x.cos();
        let mut acc = Accumulator1173::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.491_f32 + y.sin();
        let b = y * 6.661_f32 - x.cos();
        let mut acc = Accumulator1173::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.846_f32 + y.sin();
        let b = y * 2.483_f32 - x.cos();
        let mut acc = Accumulator1173::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1173(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_1173() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1173(total as u64) % 997) as f32;
        total
    }
}

pub mod m1174 {
    use super::*;

    pub struct Accumulator1174<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1174<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.481_f32 + y.sin();
        let b = y * 4.892_f32 - x.cos();
        let mut acc = Accumulator1174::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.813_f32 + y.sin();
        let b = y * 5.586_f32 - x.cos();
        let mut acc = Accumulator1174::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.716_f32 + y.sin();
        let b = y * 0.439_f32 - x.cos();
        let mut acc = Accumulator1174::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.585_f32 + y.sin();
        let b = y * 4.18_f32 - x.cos();
        let mut acc = Accumulator1174::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.44_f32 + y.sin();
        let b = y * 0.801_f32 - x.cos();
        let mut acc = Accumulator1174::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.691_f32 + y.sin();
        let b = y * 9.35_f32 - x.cos();
        let mut acc = Accumulator1174::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.542_f32 + y.sin();
        let b = y * 2.516_f32 - x.cos();
        let mut acc = Accumulator1174::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.586_f32 + y.sin();
        let b = y * 2.457_f32 - x.cos();
        let mut acc = Accumulator1174::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.987_f32 + y.sin();
        let b = y * 8.474_f32 - x.cos();
        let mut acc = Accumulator1174::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.477_f32 + y.sin();
        let b = y * 1.175_f32 - x.cos();
        let mut acc = Accumulator1174::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.77_f32 + y.sin();
        let b = y * 2.02_f32 - x.cos();
        let mut acc = Accumulator1174::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.69_f32 + y.sin();
        let b = y * 1.55_f32 - x.cos();
        let mut acc = Accumulator1174::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.326_f32 + y.sin();
        let b = y * 7.612_f32 - x.cos();
        let mut acc = Accumulator1174::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.335_f32 + y.sin();
        let b = y * 1.427_f32 - x.cos();
        let mut acc = Accumulator1174::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.039_f32 + y.sin();
        let b = y * 6.713_f32 - x.cos();
        let mut acc = Accumulator1174::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.634_f32 + y.sin();
        let b = y * 5.698_f32 - x.cos();
        let mut acc = Accumulator1174::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.294_f32 + y.sin();
        let b = y * 4.424_f32 - x.cos();
        let mut acc = Accumulator1174::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.67_f32 + y.sin();
        let b = y * 7.771_f32 - x.cos();
        let mut acc = Accumulator1174::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.843_f32 + y.sin();
        let b = y * 5.323_f32 - x.cos();
        let mut acc = Accumulator1174::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.208_f32 + y.sin();
        let b = y * 7.024_f32 - x.cos();
        let mut acc = Accumulator1174::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1174(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(1174u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_1174() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1174(total as u64) % 997) as f32;
        total
    }
}

pub mod m1175 {
    use super::*;

    pub struct Accumulator1175<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1175<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.419_f32 + y.sin();
        let b = y * 6.456_f32 - x.cos();
        let mut acc = Accumulator1175::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.063_f32 + y.sin();
        let b = y * 0.505_f32 - x.cos();
        let mut acc = Accumulator1175::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.26_f32 + y.sin();
        let b = y * 9.487_f32 - x.cos();
        let mut acc = Accumulator1175::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.406_f32 + y.sin();
        let b = y * 6.737_f32 - x.cos();
        let mut acc = Accumulator1175::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.227_f32 + y.sin();
        let b = y * 6.991_f32 - x.cos();
        let mut acc = Accumulator1175::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.889_f32 + y.sin();
        let b = y * 1.466_f32 - x.cos();
        let mut acc = Accumulator1175::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.002_f32 + y.sin();
        let b = y * 3.265_f32 - x.cos();
        let mut acc = Accumulator1175::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.844_f32 + y.sin();
        let b = y * 5.979_f32 - x.cos();
        let mut acc = Accumulator1175::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.031_f32 + y.sin();
        let b = y * 3.71_f32 - x.cos();
        let mut acc = Accumulator1175::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.531_f32 + y.sin();
        let b = y * 2.987_f32 - x.cos();
        let mut acc = Accumulator1175::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.416_f32 + y.sin();
        let b = y * 7.538_f32 - x.cos();
        let mut acc = Accumulator1175::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.37_f32 + y.sin();
        let b = y * 7.811_f32 - x.cos();
        let mut acc = Accumulator1175::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.642_f32 + y.sin();
        let b = y * 0.122_f32 - x.cos();
        let mut acc = Accumulator1175::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.506_f32 + y.sin();
        let b = y * 7.737_f32 - x.cos();
        let mut acc = Accumulator1175::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.841_f32 + y.sin();
        let b = y * 3.625_f32 - x.cos();
        let mut acc = Accumulator1175::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.549_f32 + y.sin();
        let b = y * 3.069_f32 - x.cos();
        let mut acc = Accumulator1175::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.157_f32 + y.sin();
        let b = y * 2.655_f32 - x.cos();
        let mut acc = Accumulator1175::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.664_f32 + y.sin();
        let b = y * 7.027_f32 - x.cos();
        let mut acc = Accumulator1175::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.126_f32 + y.sin();
        let b = y * 8.288_f32 - x.cos();
        let mut acc = Accumulator1175::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.767_f32 + y.sin();
        let b = y * 4.842_f32 - x.cos();
        let mut acc = Accumulator1175::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1175(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_1175() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1175(total as u64) % 997) as f32;
        total
    }
}

pub mod m1176 {
    use super::*;

    pub struct Accumulator1176<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1176<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.63_f32 + y.sin();
        let b = y * 3.058_f32 - x.cos();
        let mut acc = Accumulator1176::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.958_f32 + y.sin();
        let b = y * 2.114_f32 - x.cos();
        let mut acc = Accumulator1176::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.109_f32 + y.sin();
        let b = y * 4.92_f32 - x.cos();
        let mut acc = Accumulator1176::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.287_f32 + y.sin();
        let b = y * 2.075_f32 - x.cos();
        let mut acc = Accumulator1176::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.718_f32 + y.sin();
        let b = y * 4.622_f32 - x.cos();
        let mut acc = Accumulator1176::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.257_f32 + y.sin();
        let b = y * 4.72_f32 - x.cos();
        let mut acc = Accumulator1176::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.646_f32 + y.sin();
        let b = y * 5.336_f32 - x.cos();
        let mut acc = Accumulator1176::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.474_f32 + y.sin();
        let b = y * 6.203_f32 - x.cos();
        let mut acc = Accumulator1176::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.256_f32 + y.sin();
        let b = y * 3.885_f32 - x.cos();
        let mut acc = Accumulator1176::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.059_f32 + y.sin();
        let b = y * 4.881_f32 - x.cos();
        let mut acc = Accumulator1176::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.245_f32 + y.sin();
        let b = y * 9.249_f32 - x.cos();
        let mut acc = Accumulator1176::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.896_f32 + y.sin();
        let b = y * 1.687_f32 - x.cos();
        let mut acc = Accumulator1176::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.805_f32 + y.sin();
        let b = y * 5.328_f32 - x.cos();
        let mut acc = Accumulator1176::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.14_f32 + y.sin();
        let b = y * 7.234_f32 - x.cos();
        let mut acc = Accumulator1176::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.609_f32 + y.sin();
        let b = y * 2.304_f32 - x.cos();
        let mut acc = Accumulator1176::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.473_f32 + y.sin();
        let b = y * 5.357_f32 - x.cos();
        let mut acc = Accumulator1176::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.18_f32 + y.sin();
        let b = y * 6.091_f32 - x.cos();
        let mut acc = Accumulator1176::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.617_f32 + y.sin();
        let b = y * 0.832_f32 - x.cos();
        let mut acc = Accumulator1176::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.235_f32 + y.sin();
        let b = y * 6.179_f32 - x.cos();
        let mut acc = Accumulator1176::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.131_f32 + y.sin();
        let b = y * 0.263_f32 - x.cos();
        let mut acc = Accumulator1176::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1176(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_1176() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1176(total as u64) % 997) as f32;
        total
    }
}

pub mod m1177 {
    use super::*;

    pub struct Accumulator1177<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1177<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.089_f32 + y.sin();
        let b = y * 4.859_f32 - x.cos();
        let mut acc = Accumulator1177::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.403_f32 + y.sin();
        let b = y * 5.418_f32 - x.cos();
        let mut acc = Accumulator1177::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.103_f32 + y.sin();
        let b = y * 4.771_f32 - x.cos();
        let mut acc = Accumulator1177::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.802_f32 + y.sin();
        let b = y * 7.631_f32 - x.cos();
        let mut acc = Accumulator1177::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.43_f32 + y.sin();
        let b = y * 6.384_f32 - x.cos();
        let mut acc = Accumulator1177::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.943_f32 + y.sin();
        let b = y * 6.107_f32 - x.cos();
        let mut acc = Accumulator1177::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.299_f32 + y.sin();
        let b = y * 5.386_f32 - x.cos();
        let mut acc = Accumulator1177::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.961_f32 + y.sin();
        let b = y * 5.011_f32 - x.cos();
        let mut acc = Accumulator1177::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.845_f32 + y.sin();
        let b = y * 1.363_f32 - x.cos();
        let mut acc = Accumulator1177::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.45_f32 + y.sin();
        let b = y * 5.163_f32 - x.cos();
        let mut acc = Accumulator1177::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.975_f32 + y.sin();
        let b = y * 6.85_f32 - x.cos();
        let mut acc = Accumulator1177::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.147_f32 + y.sin();
        let b = y * 3.545_f32 - x.cos();
        let mut acc = Accumulator1177::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.219_f32 + y.sin();
        let b = y * 0.775_f32 - x.cos();
        let mut acc = Accumulator1177::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.573_f32 + y.sin();
        let b = y * 1.786_f32 - x.cos();
        let mut acc = Accumulator1177::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.662_f32 + y.sin();
        let b = y * 6.568_f32 - x.cos();
        let mut acc = Accumulator1177::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.731_f32 + y.sin();
        let b = y * 8.777_f32 - x.cos();
        let mut acc = Accumulator1177::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.489_f32 + y.sin();
        let b = y * 2.566_f32 - x.cos();
        let mut acc = Accumulator1177::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.195_f32 + y.sin();
        let b = y * 5.819_f32 - x.cos();
        let mut acc = Accumulator1177::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.81_f32 + y.sin();
        let b = y * 0.425_f32 - x.cos();
        let mut acc = Accumulator1177::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.103_f32 + y.sin();
        let b = y * 5.185_f32 - x.cos();
        let mut acc = Accumulator1177::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1177(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m1177-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_1177() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1177(total as u64) % 997) as f32;
        total
    }
}

pub mod m1178 {
    use super::*;

    pub struct Accumulator1178<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1178<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.158_f32 + y.sin();
        let b = y * 7.358_f32 - x.cos();
        let mut acc = Accumulator1178::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.72_f32 + y.sin();
        let b = y * 8.192_f32 - x.cos();
        let mut acc = Accumulator1178::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.817_f32 + y.sin();
        let b = y * 5.77_f32 - x.cos();
        let mut acc = Accumulator1178::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.61_f32 + y.sin();
        let b = y * 8.212_f32 - x.cos();
        let mut acc = Accumulator1178::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.062_f32 + y.sin();
        let b = y * 9.316_f32 - x.cos();
        let mut acc = Accumulator1178::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.76_f32 + y.sin();
        let b = y * 3.301_f32 - x.cos();
        let mut acc = Accumulator1178::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.095_f32 + y.sin();
        let b = y * 6.632_f32 - x.cos();
        let mut acc = Accumulator1178::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.011_f32 + y.sin();
        let b = y * 6.991_f32 - x.cos();
        let mut acc = Accumulator1178::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.622_f32 + y.sin();
        let b = y * 5.424_f32 - x.cos();
        let mut acc = Accumulator1178::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.031_f32 + y.sin();
        let b = y * 8.235_f32 - x.cos();
        let mut acc = Accumulator1178::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.871_f32 + y.sin();
        let b = y * 6.469_f32 - x.cos();
        let mut acc = Accumulator1178::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.475_f32 + y.sin();
        let b = y * 0.728_f32 - x.cos();
        let mut acc = Accumulator1178::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.232_f32 + y.sin();
        let b = y * 6.995_f32 - x.cos();
        let mut acc = Accumulator1178::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.813_f32 + y.sin();
        let b = y * 1.556_f32 - x.cos();
        let mut acc = Accumulator1178::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.766_f32 + y.sin();
        let b = y * 1.851_f32 - x.cos();
        let mut acc = Accumulator1178::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.984_f32 + y.sin();
        let b = y * 6.79_f32 - x.cos();
        let mut acc = Accumulator1178::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.945_f32 + y.sin();
        let b = y * 5.344_f32 - x.cos();
        let mut acc = Accumulator1178::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.227_f32 + y.sin();
        let b = y * 5.553_f32 - x.cos();
        let mut acc = Accumulator1178::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.942_f32 + y.sin();
        let b = y * 3.712_f32 - x.cos();
        let mut acc = Accumulator1178::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.661_f32 + y.sin();
        let b = y * 6.611_f32 - x.cos();
        let mut acc = Accumulator1178::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1178(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_1178() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1178(total as u64) % 997) as f32;
        total
    }
}

pub mod m1179 {
    use super::*;

    pub struct Accumulator1179<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1179<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.966_f32 + y.sin();
        let b = y * 3.613_f32 - x.cos();
        let mut acc = Accumulator1179::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.848_f32 + y.sin();
        let b = y * 4.132_f32 - x.cos();
        let mut acc = Accumulator1179::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.587_f32 + y.sin();
        let b = y * 5.28_f32 - x.cos();
        let mut acc = Accumulator1179::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.852_f32 + y.sin();
        let b = y * 7.445_f32 - x.cos();
        let mut acc = Accumulator1179::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.229_f32 + y.sin();
        let b = y * 8.233_f32 - x.cos();
        let mut acc = Accumulator1179::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.803_f32 + y.sin();
        let b = y * 6.886_f32 - x.cos();
        let mut acc = Accumulator1179::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.765_f32 + y.sin();
        let b = y * 7.039_f32 - x.cos();
        let mut acc = Accumulator1179::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.864_f32 + y.sin();
        let b = y * 1.435_f32 - x.cos();
        let mut acc = Accumulator1179::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.818_f32 + y.sin();
        let b = y * 8.769_f32 - x.cos();
        let mut acc = Accumulator1179::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.244_f32 + y.sin();
        let b = y * 2.584_f32 - x.cos();
        let mut acc = Accumulator1179::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.748_f32 + y.sin();
        let b = y * 1.567_f32 - x.cos();
        let mut acc = Accumulator1179::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.242_f32 + y.sin();
        let b = y * 2.25_f32 - x.cos();
        let mut acc = Accumulator1179::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.329_f32 + y.sin();
        let b = y * 2.741_f32 - x.cos();
        let mut acc = Accumulator1179::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.5_f32 + y.sin();
        let b = y * 6.793_f32 - x.cos();
        let mut acc = Accumulator1179::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.001_f32 + y.sin();
        let b = y * 6.211_f32 - x.cos();
        let mut acc = Accumulator1179::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.598_f32 + y.sin();
        let b = y * 5.503_f32 - x.cos();
        let mut acc = Accumulator1179::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.476_f32 + y.sin();
        let b = y * 1.921_f32 - x.cos();
        let mut acc = Accumulator1179::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.624_f32 + y.sin();
        let b = y * 1.28_f32 - x.cos();
        let mut acc = Accumulator1179::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.327_f32 + y.sin();
        let b = y * 1.087_f32 - x.cos();
        let mut acc = Accumulator1179::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.116_f32 + y.sin();
        let b = y * 9.334_f32 - x.cos();
        let mut acc = Accumulator1179::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1179(seed: u64) -> u64 {
        let re = Regex::new(r"m1179-(\d+)").unwrap();
        let hay = format!("m1179-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_1179() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1179(total as u64) % 997) as f32;
        total
    }
}

pub mod m1180 {
    use super::*;

    pub struct Accumulator1180<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1180<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.059_f32 + y.sin();
        let b = y * 5.476_f32 - x.cos();
        let mut acc = Accumulator1180::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.453_f32 + y.sin();
        let b = y * 9.499_f32 - x.cos();
        let mut acc = Accumulator1180::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.763_f32 + y.sin();
        let b = y * 5.239_f32 - x.cos();
        let mut acc = Accumulator1180::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.688_f32 + y.sin();
        let b = y * 9.022_f32 - x.cos();
        let mut acc = Accumulator1180::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.896_f32 + y.sin();
        let b = y * 5.015_f32 - x.cos();
        let mut acc = Accumulator1180::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.546_f32 + y.sin();
        let b = y * 3.989_f32 - x.cos();
        let mut acc = Accumulator1180::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.646_f32 + y.sin();
        let b = y * 9.718_f32 - x.cos();
        let mut acc = Accumulator1180::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.27_f32 + y.sin();
        let b = y * 5.633_f32 - x.cos();
        let mut acc = Accumulator1180::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.512_f32 + y.sin();
        let b = y * 2.395_f32 - x.cos();
        let mut acc = Accumulator1180::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.989_f32 + y.sin();
        let b = y * 8.998_f32 - x.cos();
        let mut acc = Accumulator1180::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.697_f32 + y.sin();
        let b = y * 9.886_f32 - x.cos();
        let mut acc = Accumulator1180::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.752_f32 + y.sin();
        let b = y * 1.518_f32 - x.cos();
        let mut acc = Accumulator1180::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.334_f32 + y.sin();
        let b = y * 7.943_f32 - x.cos();
        let mut acc = Accumulator1180::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.073_f32 + y.sin();
        let b = y * 6.2_f32 - x.cos();
        let mut acc = Accumulator1180::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.384_f32 + y.sin();
        let b = y * 7.02_f32 - x.cos();
        let mut acc = Accumulator1180::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.026_f32 + y.sin();
        let b = y * 2.689_f32 - x.cos();
        let mut acc = Accumulator1180::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.506_f32 + y.sin();
        let b = y * 3.682_f32 - x.cos();
        let mut acc = Accumulator1180::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.073_f32 + y.sin();
        let b = y * 7.847_f32 - x.cos();
        let mut acc = Accumulator1180::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.936_f32 + y.sin();
        let b = y * 2.366_f32 - x.cos();
        let mut acc = Accumulator1180::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.453_f32 + y.sin();
        let b = y * 7.667_f32 - x.cos();
        let mut acc = Accumulator1180::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1180(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_1180() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1180(total as u64) % 997) as f32;
        total
    }
}

pub mod m1181 {
    use super::*;

    pub struct Accumulator1181<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1181<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.152_f32 + y.sin();
        let b = y * 8.665_f32 - x.cos();
        let mut acc = Accumulator1181::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.509_f32 + y.sin();
        let b = y * 1.466_f32 - x.cos();
        let mut acc = Accumulator1181::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.766_f32 + y.sin();
        let b = y * 7.755_f32 - x.cos();
        let mut acc = Accumulator1181::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.841_f32 + y.sin();
        let b = y * 3.124_f32 - x.cos();
        let mut acc = Accumulator1181::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.478_f32 + y.sin();
        let b = y * 8.82_f32 - x.cos();
        let mut acc = Accumulator1181::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.288_f32 + y.sin();
        let b = y * 5.35_f32 - x.cos();
        let mut acc = Accumulator1181::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.604_f32 + y.sin();
        let b = y * 5.672_f32 - x.cos();
        let mut acc = Accumulator1181::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.465_f32 + y.sin();
        let b = y * 8.74_f32 - x.cos();
        let mut acc = Accumulator1181::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.419_f32 + y.sin();
        let b = y * 2.308_f32 - x.cos();
        let mut acc = Accumulator1181::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.606_f32 + y.sin();
        let b = y * 0.659_f32 - x.cos();
        let mut acc = Accumulator1181::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.439_f32 + y.sin();
        let b = y * 7.834_f32 - x.cos();
        let mut acc = Accumulator1181::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.423_f32 + y.sin();
        let b = y * 1.369_f32 - x.cos();
        let mut acc = Accumulator1181::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.028_f32 + y.sin();
        let b = y * 2.698_f32 - x.cos();
        let mut acc = Accumulator1181::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.456_f32 + y.sin();
        let b = y * 0.774_f32 - x.cos();
        let mut acc = Accumulator1181::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.345_f32 + y.sin();
        let b = y * 1.187_f32 - x.cos();
        let mut acc = Accumulator1181::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.267_f32 + y.sin();
        let b = y * 5.66_f32 - x.cos();
        let mut acc = Accumulator1181::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.029_f32 + y.sin();
        let b = y * 4.632_f32 - x.cos();
        let mut acc = Accumulator1181::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.614_f32 + y.sin();
        let b = y * 6.629_f32 - x.cos();
        let mut acc = Accumulator1181::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.808_f32 + y.sin();
        let b = y * 9.25_f32 - x.cos();
        let mut acc = Accumulator1181::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.174_f32 + y.sin();
        let b = y * 2.213_f32 - x.cos();
        let mut acc = Accumulator1181::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1181(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(1181u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_1181() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1181(total as u64) % 997) as f32;
        total
    }
}

pub mod m1182 {
    use super::*;

    pub struct Accumulator1182<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1182<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.853_f32 + y.sin();
        let b = y * 0.619_f32 - x.cos();
        let mut acc = Accumulator1182::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.143_f32 + y.sin();
        let b = y * 3.948_f32 - x.cos();
        let mut acc = Accumulator1182::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.791_f32 + y.sin();
        let b = y * 6.858_f32 - x.cos();
        let mut acc = Accumulator1182::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.395_f32 + y.sin();
        let b = y * 3.996_f32 - x.cos();
        let mut acc = Accumulator1182::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.216_f32 + y.sin();
        let b = y * 1.903_f32 - x.cos();
        let mut acc = Accumulator1182::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.615_f32 + y.sin();
        let b = y * 7.314_f32 - x.cos();
        let mut acc = Accumulator1182::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.339_f32 + y.sin();
        let b = y * 0.673_f32 - x.cos();
        let mut acc = Accumulator1182::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.673_f32 + y.sin();
        let b = y * 8.313_f32 - x.cos();
        let mut acc = Accumulator1182::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.927_f32 + y.sin();
        let b = y * 8.177_f32 - x.cos();
        let mut acc = Accumulator1182::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.505_f32 + y.sin();
        let b = y * 3.403_f32 - x.cos();
        let mut acc = Accumulator1182::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.381_f32 + y.sin();
        let b = y * 2.329_f32 - x.cos();
        let mut acc = Accumulator1182::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.07_f32 + y.sin();
        let b = y * 0.903_f32 - x.cos();
        let mut acc = Accumulator1182::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.345_f32 + y.sin();
        let b = y * 6.607_f32 - x.cos();
        let mut acc = Accumulator1182::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.759_f32 + y.sin();
        let b = y * 1.939_f32 - x.cos();
        let mut acc = Accumulator1182::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.239_f32 + y.sin();
        let b = y * 3.149_f32 - x.cos();
        let mut acc = Accumulator1182::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.389_f32 + y.sin();
        let b = y * 8.916_f32 - x.cos();
        let mut acc = Accumulator1182::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.423_f32 + y.sin();
        let b = y * 9.43_f32 - x.cos();
        let mut acc = Accumulator1182::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.983_f32 + y.sin();
        let b = y * 8.616_f32 - x.cos();
        let mut acc = Accumulator1182::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.322_f32 + y.sin();
        let b = y * 8.459_f32 - x.cos();
        let mut acc = Accumulator1182::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.297_f32 + y.sin();
        let b = y * 9.404_f32 - x.cos();
        let mut acc = Accumulator1182::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1182(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_1182() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1182(total as u64) % 997) as f32;
        total
    }
}

pub mod m1183 {
    use super::*;

    pub struct Accumulator1183<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1183<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.29_f32 + y.sin();
        let b = y * 8.631_f32 - x.cos();
        let mut acc = Accumulator1183::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.744_f32 + y.sin();
        let b = y * 1.343_f32 - x.cos();
        let mut acc = Accumulator1183::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.953_f32 + y.sin();
        let b = y * 7.245_f32 - x.cos();
        let mut acc = Accumulator1183::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.025_f32 + y.sin();
        let b = y * 2.965_f32 - x.cos();
        let mut acc = Accumulator1183::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.408_f32 + y.sin();
        let b = y * 3.091_f32 - x.cos();
        let mut acc = Accumulator1183::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.649_f32 + y.sin();
        let b = y * 6.711_f32 - x.cos();
        let mut acc = Accumulator1183::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.698_f32 + y.sin();
        let b = y * 1.535_f32 - x.cos();
        let mut acc = Accumulator1183::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.491_f32 + y.sin();
        let b = y * 6.271_f32 - x.cos();
        let mut acc = Accumulator1183::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.163_f32 + y.sin();
        let b = y * 1.688_f32 - x.cos();
        let mut acc = Accumulator1183::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.443_f32 + y.sin();
        let b = y * 7.028_f32 - x.cos();
        let mut acc = Accumulator1183::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.71_f32 + y.sin();
        let b = y * 5.937_f32 - x.cos();
        let mut acc = Accumulator1183::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.36_f32 + y.sin();
        let b = y * 3.601_f32 - x.cos();
        let mut acc = Accumulator1183::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.8_f32 + y.sin();
        let b = y * 5.681_f32 - x.cos();
        let mut acc = Accumulator1183::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.344_f32 + y.sin();
        let b = y * 2.109_f32 - x.cos();
        let mut acc = Accumulator1183::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.602_f32 + y.sin();
        let b = y * 1.62_f32 - x.cos();
        let mut acc = Accumulator1183::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.553_f32 + y.sin();
        let b = y * 4.694_f32 - x.cos();
        let mut acc = Accumulator1183::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.258_f32 + y.sin();
        let b = y * 0.27_f32 - x.cos();
        let mut acc = Accumulator1183::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.382_f32 + y.sin();
        let b = y * 8.476_f32 - x.cos();
        let mut acc = Accumulator1183::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.155_f32 + y.sin();
        let b = y * 5.732_f32 - x.cos();
        let mut acc = Accumulator1183::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.154_f32 + y.sin();
        let b = y * 5.122_f32 - x.cos();
        let mut acc = Accumulator1183::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1183(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_1183() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1183(total as u64) % 997) as f32;
        total
    }
}

pub mod m1184 {
    use super::*;

    pub struct Accumulator1184<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1184<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.934_f32 + y.sin();
        let b = y * 2.757_f32 - x.cos();
        let mut acc = Accumulator1184::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.673_f32 + y.sin();
        let b = y * 5.853_f32 - x.cos();
        let mut acc = Accumulator1184::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.538_f32 + y.sin();
        let b = y * 1.912_f32 - x.cos();
        let mut acc = Accumulator1184::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.42_f32 + y.sin();
        let b = y * 7.601_f32 - x.cos();
        let mut acc = Accumulator1184::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.292_f32 + y.sin();
        let b = y * 5.116_f32 - x.cos();
        let mut acc = Accumulator1184::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.466_f32 + y.sin();
        let b = y * 2.524_f32 - x.cos();
        let mut acc = Accumulator1184::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.842_f32 + y.sin();
        let b = y * 9.269_f32 - x.cos();
        let mut acc = Accumulator1184::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.053_f32 + y.sin();
        let b = y * 4.492_f32 - x.cos();
        let mut acc = Accumulator1184::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.002_f32 + y.sin();
        let b = y * 1.062_f32 - x.cos();
        let mut acc = Accumulator1184::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.877_f32 + y.sin();
        let b = y * 3.276_f32 - x.cos();
        let mut acc = Accumulator1184::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.636_f32 + y.sin();
        let b = y * 4.378_f32 - x.cos();
        let mut acc = Accumulator1184::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.884_f32 + y.sin();
        let b = y * 1.368_f32 - x.cos();
        let mut acc = Accumulator1184::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.863_f32 + y.sin();
        let b = y * 0.442_f32 - x.cos();
        let mut acc = Accumulator1184::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.263_f32 + y.sin();
        let b = y * 3.685_f32 - x.cos();
        let mut acc = Accumulator1184::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.125_f32 + y.sin();
        let b = y * 7.961_f32 - x.cos();
        let mut acc = Accumulator1184::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.459_f32 + y.sin();
        let b = y * 6.504_f32 - x.cos();
        let mut acc = Accumulator1184::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.393_f32 + y.sin();
        let b = y * 8.244_f32 - x.cos();
        let mut acc = Accumulator1184::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.598_f32 + y.sin();
        let b = y * 9.184_f32 - x.cos();
        let mut acc = Accumulator1184::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.069_f32 + y.sin();
        let b = y * 2.031_f32 - x.cos();
        let mut acc = Accumulator1184::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.956_f32 + y.sin();
        let b = y * 4.318_f32 - x.cos();
        let mut acc = Accumulator1184::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1184(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m1184-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_1184() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1184(total as u64) % 997) as f32;
        total
    }
}

pub mod m1185 {
    use super::*;

    pub struct Accumulator1185<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1185<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.528_f32 + y.sin();
        let b = y * 6.08_f32 - x.cos();
        let mut acc = Accumulator1185::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.916_f32 + y.sin();
        let b = y * 0.324_f32 - x.cos();
        let mut acc = Accumulator1185::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.345_f32 + y.sin();
        let b = y * 6.838_f32 - x.cos();
        let mut acc = Accumulator1185::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.971_f32 + y.sin();
        let b = y * 4.826_f32 - x.cos();
        let mut acc = Accumulator1185::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.889_f32 + y.sin();
        let b = y * 7.435_f32 - x.cos();
        let mut acc = Accumulator1185::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.816_f32 + y.sin();
        let b = y * 4.264_f32 - x.cos();
        let mut acc = Accumulator1185::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.801_f32 + y.sin();
        let b = y * 8.821_f32 - x.cos();
        let mut acc = Accumulator1185::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.594_f32 + y.sin();
        let b = y * 1.494_f32 - x.cos();
        let mut acc = Accumulator1185::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.205_f32 + y.sin();
        let b = y * 8.957_f32 - x.cos();
        let mut acc = Accumulator1185::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.629_f32 + y.sin();
        let b = y * 0.63_f32 - x.cos();
        let mut acc = Accumulator1185::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.912_f32 + y.sin();
        let b = y * 5.989_f32 - x.cos();
        let mut acc = Accumulator1185::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.077_f32 + y.sin();
        let b = y * 7.637_f32 - x.cos();
        let mut acc = Accumulator1185::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.676_f32 + y.sin();
        let b = y * 9.815_f32 - x.cos();
        let mut acc = Accumulator1185::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.76_f32 + y.sin();
        let b = y * 0.994_f32 - x.cos();
        let mut acc = Accumulator1185::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.51_f32 + y.sin();
        let b = y * 5.812_f32 - x.cos();
        let mut acc = Accumulator1185::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.757_f32 + y.sin();
        let b = y * 0.918_f32 - x.cos();
        let mut acc = Accumulator1185::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.449_f32 + y.sin();
        let b = y * 6.291_f32 - x.cos();
        let mut acc = Accumulator1185::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.804_f32 + y.sin();
        let b = y * 9.669_f32 - x.cos();
        let mut acc = Accumulator1185::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.074_f32 + y.sin();
        let b = y * 0.43_f32 - x.cos();
        let mut acc = Accumulator1185::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.54_f32 + y.sin();
        let b = y * 0.105_f32 - x.cos();
        let mut acc = Accumulator1185::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1185(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_1185() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1185(total as u64) % 997) as f32;
        total
    }
}

pub mod m1186 {
    use super::*;

    pub struct Accumulator1186<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1186<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.004_f32 + y.sin();
        let b = y * 2.861_f32 - x.cos();
        let mut acc = Accumulator1186::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.373_f32 + y.sin();
        let b = y * 1.759_f32 - x.cos();
        let mut acc = Accumulator1186::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.079_f32 + y.sin();
        let b = y * 9.398_f32 - x.cos();
        let mut acc = Accumulator1186::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.247_f32 + y.sin();
        let b = y * 8.8_f32 - x.cos();
        let mut acc = Accumulator1186::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.329_f32 + y.sin();
        let b = y * 8.156_f32 - x.cos();
        let mut acc = Accumulator1186::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.73_f32 + y.sin();
        let b = y * 9.818_f32 - x.cos();
        let mut acc = Accumulator1186::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.127_f32 + y.sin();
        let b = y * 4.07_f32 - x.cos();
        let mut acc = Accumulator1186::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.304_f32 + y.sin();
        let b = y * 2.648_f32 - x.cos();
        let mut acc = Accumulator1186::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.033_f32 + y.sin();
        let b = y * 2.834_f32 - x.cos();
        let mut acc = Accumulator1186::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.96_f32 + y.sin();
        let b = y * 0.765_f32 - x.cos();
        let mut acc = Accumulator1186::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.081_f32 + y.sin();
        let b = y * 3.843_f32 - x.cos();
        let mut acc = Accumulator1186::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.616_f32 + y.sin();
        let b = y * 1.51_f32 - x.cos();
        let mut acc = Accumulator1186::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.954_f32 + y.sin();
        let b = y * 7.984_f32 - x.cos();
        let mut acc = Accumulator1186::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.716_f32 + y.sin();
        let b = y * 7.243_f32 - x.cos();
        let mut acc = Accumulator1186::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.662_f32 + y.sin();
        let b = y * 7.049_f32 - x.cos();
        let mut acc = Accumulator1186::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.183_f32 + y.sin();
        let b = y * 3.725_f32 - x.cos();
        let mut acc = Accumulator1186::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.997_f32 + y.sin();
        let b = y * 4.054_f32 - x.cos();
        let mut acc = Accumulator1186::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.238_f32 + y.sin();
        let b = y * 5.588_f32 - x.cos();
        let mut acc = Accumulator1186::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.407_f32 + y.sin();
        let b = y * 4.648_f32 - x.cos();
        let mut acc = Accumulator1186::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.468_f32 + y.sin();
        let b = y * 5.567_f32 - x.cos();
        let mut acc = Accumulator1186::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1186(seed: u64) -> u64 {
        let re = Regex::new(r"m1186-(\d+)").unwrap();
        let hay = format!("m1186-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_1186() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1186(total as u64) % 997) as f32;
        total
    }
}

pub mod m1187 {
    use super::*;

    pub struct Accumulator1187<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1187<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.613_f32 + y.sin();
        let b = y * 3.101_f32 - x.cos();
        let mut acc = Accumulator1187::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.89_f32 + y.sin();
        let b = y * 6.956_f32 - x.cos();
        let mut acc = Accumulator1187::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.711_f32 + y.sin();
        let b = y * 7.757_f32 - x.cos();
        let mut acc = Accumulator1187::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.988_f32 + y.sin();
        let b = y * 1.671_f32 - x.cos();
        let mut acc = Accumulator1187::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.962_f32 + y.sin();
        let b = y * 3.591_f32 - x.cos();
        let mut acc = Accumulator1187::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.38_f32 + y.sin();
        let b = y * 0.623_f32 - x.cos();
        let mut acc = Accumulator1187::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.989_f32 + y.sin();
        let b = y * 4.387_f32 - x.cos();
        let mut acc = Accumulator1187::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.916_f32 + y.sin();
        let b = y * 6.271_f32 - x.cos();
        let mut acc = Accumulator1187::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.875_f32 + y.sin();
        let b = y * 9.687_f32 - x.cos();
        let mut acc = Accumulator1187::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.385_f32 + y.sin();
        let b = y * 7.638_f32 - x.cos();
        let mut acc = Accumulator1187::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.996_f32 + y.sin();
        let b = y * 0.425_f32 - x.cos();
        let mut acc = Accumulator1187::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.217_f32 + y.sin();
        let b = y * 5.278_f32 - x.cos();
        let mut acc = Accumulator1187::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.117_f32 + y.sin();
        let b = y * 3.717_f32 - x.cos();
        let mut acc = Accumulator1187::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.911_f32 + y.sin();
        let b = y * 9.83_f32 - x.cos();
        let mut acc = Accumulator1187::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.777_f32 + y.sin();
        let b = y * 6.812_f32 - x.cos();
        let mut acc = Accumulator1187::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.586_f32 + y.sin();
        let b = y * 1.655_f32 - x.cos();
        let mut acc = Accumulator1187::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.048_f32 + y.sin();
        let b = y * 3.25_f32 - x.cos();
        let mut acc = Accumulator1187::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.297_f32 + y.sin();
        let b = y * 4.442_f32 - x.cos();
        let mut acc = Accumulator1187::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.187_f32 + y.sin();
        let b = y * 9.118_f32 - x.cos();
        let mut acc = Accumulator1187::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.565_f32 + y.sin();
        let b = y * 7.297_f32 - x.cos();
        let mut acc = Accumulator1187::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1187(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_1187() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1187(total as u64) % 997) as f32;
        total
    }
}

pub mod m1188 {
    use super::*;

    pub struct Accumulator1188<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1188<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.489_f32 + y.sin();
        let b = y * 7.646_f32 - x.cos();
        let mut acc = Accumulator1188::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.693_f32 + y.sin();
        let b = y * 6.926_f32 - x.cos();
        let mut acc = Accumulator1188::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.785_f32 + y.sin();
        let b = y * 4.03_f32 - x.cos();
        let mut acc = Accumulator1188::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.115_f32 + y.sin();
        let b = y * 9.596_f32 - x.cos();
        let mut acc = Accumulator1188::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.293_f32 + y.sin();
        let b = y * 8.022_f32 - x.cos();
        let mut acc = Accumulator1188::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.138_f32 + y.sin();
        let b = y * 3.626_f32 - x.cos();
        let mut acc = Accumulator1188::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.838_f32 + y.sin();
        let b = y * 3.247_f32 - x.cos();
        let mut acc = Accumulator1188::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.624_f32 + y.sin();
        let b = y * 1.738_f32 - x.cos();
        let mut acc = Accumulator1188::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.179_f32 + y.sin();
        let b = y * 0.479_f32 - x.cos();
        let mut acc = Accumulator1188::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.096_f32 + y.sin();
        let b = y * 8.744_f32 - x.cos();
        let mut acc = Accumulator1188::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.396_f32 + y.sin();
        let b = y * 6.961_f32 - x.cos();
        let mut acc = Accumulator1188::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.296_f32 + y.sin();
        let b = y * 9.711_f32 - x.cos();
        let mut acc = Accumulator1188::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.298_f32 + y.sin();
        let b = y * 2.652_f32 - x.cos();
        let mut acc = Accumulator1188::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.272_f32 + y.sin();
        let b = y * 0.169_f32 - x.cos();
        let mut acc = Accumulator1188::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.709_f32 + y.sin();
        let b = y * 7.725_f32 - x.cos();
        let mut acc = Accumulator1188::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.357_f32 + y.sin();
        let b = y * 1.197_f32 - x.cos();
        let mut acc = Accumulator1188::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.789_f32 + y.sin();
        let b = y * 2.467_f32 - x.cos();
        let mut acc = Accumulator1188::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.597_f32 + y.sin();
        let b = y * 7.046_f32 - x.cos();
        let mut acc = Accumulator1188::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.873_f32 + y.sin();
        let b = y * 3.417_f32 - x.cos();
        let mut acc = Accumulator1188::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.986_f32 + y.sin();
        let b = y * 3.488_f32 - x.cos();
        let mut acc = Accumulator1188::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1188(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(1188u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_1188() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1188(total as u64) % 997) as f32;
        total
    }
}

pub mod m1189 {
    use super::*;

    pub struct Accumulator1189<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1189<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.503_f32 + y.sin();
        let b = y * 4.532_f32 - x.cos();
        let mut acc = Accumulator1189::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.087_f32 + y.sin();
        let b = y * 1.597_f32 - x.cos();
        let mut acc = Accumulator1189::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.97_f32 + y.sin();
        let b = y * 8.357_f32 - x.cos();
        let mut acc = Accumulator1189::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.817_f32 + y.sin();
        let b = y * 5.939_f32 - x.cos();
        let mut acc = Accumulator1189::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.965_f32 + y.sin();
        let b = y * 6.977_f32 - x.cos();
        let mut acc = Accumulator1189::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.544_f32 + y.sin();
        let b = y * 8.008_f32 - x.cos();
        let mut acc = Accumulator1189::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.904_f32 + y.sin();
        let b = y * 2.092_f32 - x.cos();
        let mut acc = Accumulator1189::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.145_f32 + y.sin();
        let b = y * 5.113_f32 - x.cos();
        let mut acc = Accumulator1189::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.502_f32 + y.sin();
        let b = y * 7.422_f32 - x.cos();
        let mut acc = Accumulator1189::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.765_f32 + y.sin();
        let b = y * 9.258_f32 - x.cos();
        let mut acc = Accumulator1189::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.266_f32 + y.sin();
        let b = y * 3.972_f32 - x.cos();
        let mut acc = Accumulator1189::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.558_f32 + y.sin();
        let b = y * 5.405_f32 - x.cos();
        let mut acc = Accumulator1189::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.755_f32 + y.sin();
        let b = y * 5.766_f32 - x.cos();
        let mut acc = Accumulator1189::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.199_f32 + y.sin();
        let b = y * 9.474_f32 - x.cos();
        let mut acc = Accumulator1189::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.65_f32 + y.sin();
        let b = y * 9.572_f32 - x.cos();
        let mut acc = Accumulator1189::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.119_f32 + y.sin();
        let b = y * 9.407_f32 - x.cos();
        let mut acc = Accumulator1189::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.118_f32 + y.sin();
        let b = y * 5.885_f32 - x.cos();
        let mut acc = Accumulator1189::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.608_f32 + y.sin();
        let b = y * 7.956_f32 - x.cos();
        let mut acc = Accumulator1189::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.869_f32 + y.sin();
        let b = y * 1.899_f32 - x.cos();
        let mut acc = Accumulator1189::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.272_f32 + y.sin();
        let b = y * 7.405_f32 - x.cos();
        let mut acc = Accumulator1189::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1189(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_1189() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1189(total as u64) % 997) as f32;
        total
    }
}

pub mod m1190 {
    use super::*;

    pub struct Accumulator1190<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1190<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.396_f32 + y.sin();
        let b = y * 0.122_f32 - x.cos();
        let mut acc = Accumulator1190::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.395_f32 + y.sin();
        let b = y * 4.029_f32 - x.cos();
        let mut acc = Accumulator1190::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.457_f32 + y.sin();
        let b = y * 0.661_f32 - x.cos();
        let mut acc = Accumulator1190::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.618_f32 + y.sin();
        let b = y * 9.236_f32 - x.cos();
        let mut acc = Accumulator1190::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.659_f32 + y.sin();
        let b = y * 3.426_f32 - x.cos();
        let mut acc = Accumulator1190::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.032_f32 + y.sin();
        let b = y * 4.516_f32 - x.cos();
        let mut acc = Accumulator1190::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.621_f32 + y.sin();
        let b = y * 7.932_f32 - x.cos();
        let mut acc = Accumulator1190::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.619_f32 + y.sin();
        let b = y * 9.527_f32 - x.cos();
        let mut acc = Accumulator1190::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.779_f32 + y.sin();
        let b = y * 3.745_f32 - x.cos();
        let mut acc = Accumulator1190::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.263_f32 + y.sin();
        let b = y * 8.239_f32 - x.cos();
        let mut acc = Accumulator1190::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.266_f32 + y.sin();
        let b = y * 8.61_f32 - x.cos();
        let mut acc = Accumulator1190::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.863_f32 + y.sin();
        let b = y * 8.535_f32 - x.cos();
        let mut acc = Accumulator1190::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.611_f32 + y.sin();
        let b = y * 3.393_f32 - x.cos();
        let mut acc = Accumulator1190::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.611_f32 + y.sin();
        let b = y * 4.673_f32 - x.cos();
        let mut acc = Accumulator1190::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.353_f32 + y.sin();
        let b = y * 6.323_f32 - x.cos();
        let mut acc = Accumulator1190::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.695_f32 + y.sin();
        let b = y * 8.931_f32 - x.cos();
        let mut acc = Accumulator1190::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.204_f32 + y.sin();
        let b = y * 1.931_f32 - x.cos();
        let mut acc = Accumulator1190::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.987_f32 + y.sin();
        let b = y * 1.744_f32 - x.cos();
        let mut acc = Accumulator1190::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.165_f32 + y.sin();
        let b = y * 9.532_f32 - x.cos();
        let mut acc = Accumulator1190::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.857_f32 + y.sin();
        let b = y * 5.981_f32 - x.cos();
        let mut acc = Accumulator1190::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1190(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_1190() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1190(total as u64) % 997) as f32;
        total
    }
}

pub mod m1191 {
    use super::*;

    pub struct Accumulator1191<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1191<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.921_f32 + y.sin();
        let b = y * 2.793_f32 - x.cos();
        let mut acc = Accumulator1191::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.963_f32 + y.sin();
        let b = y * 7.887_f32 - x.cos();
        let mut acc = Accumulator1191::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.023_f32 + y.sin();
        let b = y * 3.655_f32 - x.cos();
        let mut acc = Accumulator1191::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.893_f32 + y.sin();
        let b = y * 1.309_f32 - x.cos();
        let mut acc = Accumulator1191::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.38_f32 + y.sin();
        let b = y * 7.355_f32 - x.cos();
        let mut acc = Accumulator1191::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.503_f32 + y.sin();
        let b = y * 3.367_f32 - x.cos();
        let mut acc = Accumulator1191::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.879_f32 + y.sin();
        let b = y * 9.666_f32 - x.cos();
        let mut acc = Accumulator1191::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.429_f32 + y.sin();
        let b = y * 0.609_f32 - x.cos();
        let mut acc = Accumulator1191::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.812_f32 + y.sin();
        let b = y * 4.988_f32 - x.cos();
        let mut acc = Accumulator1191::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.159_f32 + y.sin();
        let b = y * 7.705_f32 - x.cos();
        let mut acc = Accumulator1191::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.597_f32 + y.sin();
        let b = y * 3.889_f32 - x.cos();
        let mut acc = Accumulator1191::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.738_f32 + y.sin();
        let b = y * 5.353_f32 - x.cos();
        let mut acc = Accumulator1191::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.868_f32 + y.sin();
        let b = y * 6.253_f32 - x.cos();
        let mut acc = Accumulator1191::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.965_f32 + y.sin();
        let b = y * 5.379_f32 - x.cos();
        let mut acc = Accumulator1191::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.417_f32 + y.sin();
        let b = y * 5.105_f32 - x.cos();
        let mut acc = Accumulator1191::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.273_f32 + y.sin();
        let b = y * 1.987_f32 - x.cos();
        let mut acc = Accumulator1191::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.719_f32 + y.sin();
        let b = y * 0.543_f32 - x.cos();
        let mut acc = Accumulator1191::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.109_f32 + y.sin();
        let b = y * 3.515_f32 - x.cos();
        let mut acc = Accumulator1191::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.48_f32 + y.sin();
        let b = y * 8.224_f32 - x.cos();
        let mut acc = Accumulator1191::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.622_f32 + y.sin();
        let b = y * 3.016_f32 - x.cos();
        let mut acc = Accumulator1191::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1191(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m1191-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_1191() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1191(total as u64) % 997) as f32;
        total
    }
}

pub mod m1192 {
    use super::*;

    pub struct Accumulator1192<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1192<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.851_f32 + y.sin();
        let b = y * 9.138_f32 - x.cos();
        let mut acc = Accumulator1192::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.117_f32 + y.sin();
        let b = y * 0.948_f32 - x.cos();
        let mut acc = Accumulator1192::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.038_f32 + y.sin();
        let b = y * 5.927_f32 - x.cos();
        let mut acc = Accumulator1192::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.952_f32 + y.sin();
        let b = y * 4.192_f32 - x.cos();
        let mut acc = Accumulator1192::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.205_f32 + y.sin();
        let b = y * 6.097_f32 - x.cos();
        let mut acc = Accumulator1192::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.635_f32 + y.sin();
        let b = y * 4.12_f32 - x.cos();
        let mut acc = Accumulator1192::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.694_f32 + y.sin();
        let b = y * 6.478_f32 - x.cos();
        let mut acc = Accumulator1192::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.788_f32 + y.sin();
        let b = y * 4.582_f32 - x.cos();
        let mut acc = Accumulator1192::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.075_f32 + y.sin();
        let b = y * 6.816_f32 - x.cos();
        let mut acc = Accumulator1192::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.582_f32 + y.sin();
        let b = y * 6.23_f32 - x.cos();
        let mut acc = Accumulator1192::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.375_f32 + y.sin();
        let b = y * 1.223_f32 - x.cos();
        let mut acc = Accumulator1192::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.848_f32 + y.sin();
        let b = y * 4.873_f32 - x.cos();
        let mut acc = Accumulator1192::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.084_f32 + y.sin();
        let b = y * 2.798_f32 - x.cos();
        let mut acc = Accumulator1192::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.688_f32 + y.sin();
        let b = y * 6.815_f32 - x.cos();
        let mut acc = Accumulator1192::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.15_f32 + y.sin();
        let b = y * 1.189_f32 - x.cos();
        let mut acc = Accumulator1192::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.238_f32 + y.sin();
        let b = y * 9.039_f32 - x.cos();
        let mut acc = Accumulator1192::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.337_f32 + y.sin();
        let b = y * 1.582_f32 - x.cos();
        let mut acc = Accumulator1192::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.393_f32 + y.sin();
        let b = y * 3.828_f32 - x.cos();
        let mut acc = Accumulator1192::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.032_f32 + y.sin();
        let b = y * 3.708_f32 - x.cos();
        let mut acc = Accumulator1192::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.837_f32 + y.sin();
        let b = y * 0.581_f32 - x.cos();
        let mut acc = Accumulator1192::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1192(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_1192() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1192(total as u64) % 997) as f32;
        total
    }
}

pub mod m1193 {
    use super::*;

    pub struct Accumulator1193<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1193<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.999_f32 + y.sin();
        let b = y * 3.025_f32 - x.cos();
        let mut acc = Accumulator1193::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.63_f32 + y.sin();
        let b = y * 4.325_f32 - x.cos();
        let mut acc = Accumulator1193::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.0_f32 + y.sin();
        let b = y * 4.889_f32 - x.cos();
        let mut acc = Accumulator1193::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.336_f32 + y.sin();
        let b = y * 3.043_f32 - x.cos();
        let mut acc = Accumulator1193::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.253_f32 + y.sin();
        let b = y * 4.373_f32 - x.cos();
        let mut acc = Accumulator1193::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.629_f32 + y.sin();
        let b = y * 8.155_f32 - x.cos();
        let mut acc = Accumulator1193::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.906_f32 + y.sin();
        let b = y * 6.002_f32 - x.cos();
        let mut acc = Accumulator1193::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.013_f32 + y.sin();
        let b = y * 9.89_f32 - x.cos();
        let mut acc = Accumulator1193::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.667_f32 + y.sin();
        let b = y * 8.352_f32 - x.cos();
        let mut acc = Accumulator1193::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.833_f32 + y.sin();
        let b = y * 8.279_f32 - x.cos();
        let mut acc = Accumulator1193::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.278_f32 + y.sin();
        let b = y * 8.838_f32 - x.cos();
        let mut acc = Accumulator1193::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.782_f32 + y.sin();
        let b = y * 2.097_f32 - x.cos();
        let mut acc = Accumulator1193::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.949_f32 + y.sin();
        let b = y * 2.665_f32 - x.cos();
        let mut acc = Accumulator1193::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.26_f32 + y.sin();
        let b = y * 8.886_f32 - x.cos();
        let mut acc = Accumulator1193::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.184_f32 + y.sin();
        let b = y * 8.817_f32 - x.cos();
        let mut acc = Accumulator1193::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.354_f32 + y.sin();
        let b = y * 7.104_f32 - x.cos();
        let mut acc = Accumulator1193::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.127_f32 + y.sin();
        let b = y * 6.991_f32 - x.cos();
        let mut acc = Accumulator1193::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.225_f32 + y.sin();
        let b = y * 1.032_f32 - x.cos();
        let mut acc = Accumulator1193::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.337_f32 + y.sin();
        let b = y * 7.507_f32 - x.cos();
        let mut acc = Accumulator1193::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.886_f32 + y.sin();
        let b = y * 3.93_f32 - x.cos();
        let mut acc = Accumulator1193::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1193(seed: u64) -> u64 {
        let re = Regex::new(r"m1193-(\d+)").unwrap();
        let hay = format!("m1193-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_1193() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1193(total as u64) % 997) as f32;
        total
    }
}

pub mod m1194 {
    use super::*;

    pub struct Accumulator1194<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1194<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.855_f32 + y.sin();
        let b = y * 7.441_f32 - x.cos();
        let mut acc = Accumulator1194::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.818_f32 + y.sin();
        let b = y * 8.251_f32 - x.cos();
        let mut acc = Accumulator1194::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.927_f32 + y.sin();
        let b = y * 9.138_f32 - x.cos();
        let mut acc = Accumulator1194::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.228_f32 + y.sin();
        let b = y * 2.016_f32 - x.cos();
        let mut acc = Accumulator1194::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.953_f32 + y.sin();
        let b = y * 0.577_f32 - x.cos();
        let mut acc = Accumulator1194::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.237_f32 + y.sin();
        let b = y * 8.853_f32 - x.cos();
        let mut acc = Accumulator1194::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.227_f32 + y.sin();
        let b = y * 0.897_f32 - x.cos();
        let mut acc = Accumulator1194::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.046_f32 + y.sin();
        let b = y * 2.568_f32 - x.cos();
        let mut acc = Accumulator1194::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.799_f32 + y.sin();
        let b = y * 2.861_f32 - x.cos();
        let mut acc = Accumulator1194::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.536_f32 + y.sin();
        let b = y * 9.387_f32 - x.cos();
        let mut acc = Accumulator1194::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.927_f32 + y.sin();
        let b = y * 2.219_f32 - x.cos();
        let mut acc = Accumulator1194::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.175_f32 + y.sin();
        let b = y * 7.417_f32 - x.cos();
        let mut acc = Accumulator1194::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.938_f32 + y.sin();
        let b = y * 0.794_f32 - x.cos();
        let mut acc = Accumulator1194::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.184_f32 + y.sin();
        let b = y * 8.963_f32 - x.cos();
        let mut acc = Accumulator1194::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.662_f32 + y.sin();
        let b = y * 8.54_f32 - x.cos();
        let mut acc = Accumulator1194::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.719_f32 + y.sin();
        let b = y * 5.138_f32 - x.cos();
        let mut acc = Accumulator1194::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.444_f32 + y.sin();
        let b = y * 6.066_f32 - x.cos();
        let mut acc = Accumulator1194::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.552_f32 + y.sin();
        let b = y * 6.397_f32 - x.cos();
        let mut acc = Accumulator1194::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.771_f32 + y.sin();
        let b = y * 7.806_f32 - x.cos();
        let mut acc = Accumulator1194::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.104_f32 + y.sin();
        let b = y * 4.582_f32 - x.cos();
        let mut acc = Accumulator1194::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1194(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_1194() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1194(total as u64) % 997) as f32;
        total
    }
}

pub mod m1195 {
    use super::*;

    pub struct Accumulator1195<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1195<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.685_f32 + y.sin();
        let b = y * 9.89_f32 - x.cos();
        let mut acc = Accumulator1195::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.365_f32 + y.sin();
        let b = y * 4.139_f32 - x.cos();
        let mut acc = Accumulator1195::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.221_f32 + y.sin();
        let b = y * 4.672_f32 - x.cos();
        let mut acc = Accumulator1195::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.507_f32 + y.sin();
        let b = y * 0.364_f32 - x.cos();
        let mut acc = Accumulator1195::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.915_f32 + y.sin();
        let b = y * 9.244_f32 - x.cos();
        let mut acc = Accumulator1195::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.918_f32 + y.sin();
        let b = y * 7.106_f32 - x.cos();
        let mut acc = Accumulator1195::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.134_f32 + y.sin();
        let b = y * 0.615_f32 - x.cos();
        let mut acc = Accumulator1195::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.76_f32 + y.sin();
        let b = y * 9.511_f32 - x.cos();
        let mut acc = Accumulator1195::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.811_f32 + y.sin();
        let b = y * 2.074_f32 - x.cos();
        let mut acc = Accumulator1195::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.432_f32 + y.sin();
        let b = y * 6.729_f32 - x.cos();
        let mut acc = Accumulator1195::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.124_f32 + y.sin();
        let b = y * 3.854_f32 - x.cos();
        let mut acc = Accumulator1195::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.474_f32 + y.sin();
        let b = y * 9.503_f32 - x.cos();
        let mut acc = Accumulator1195::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.422_f32 + y.sin();
        let b = y * 0.642_f32 - x.cos();
        let mut acc = Accumulator1195::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.286_f32 + y.sin();
        let b = y * 3.377_f32 - x.cos();
        let mut acc = Accumulator1195::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.447_f32 + y.sin();
        let b = y * 0.549_f32 - x.cos();
        let mut acc = Accumulator1195::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.57_f32 + y.sin();
        let b = y * 6.355_f32 - x.cos();
        let mut acc = Accumulator1195::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.511_f32 + y.sin();
        let b = y * 7.896_f32 - x.cos();
        let mut acc = Accumulator1195::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.212_f32 + y.sin();
        let b = y * 3.387_f32 - x.cos();
        let mut acc = Accumulator1195::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.16_f32 + y.sin();
        let b = y * 2.152_f32 - x.cos();
        let mut acc = Accumulator1195::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.788_f32 + y.sin();
        let b = y * 2.899_f32 - x.cos();
        let mut acc = Accumulator1195::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1195(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(1195u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_1195() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1195(total as u64) % 997) as f32;
        total
    }
}

pub mod m1196 {
    use super::*;

    pub struct Accumulator1196<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1196<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.912_f32 + y.sin();
        let b = y * 7.221_f32 - x.cos();
        let mut acc = Accumulator1196::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.92_f32 + y.sin();
        let b = y * 5.327_f32 - x.cos();
        let mut acc = Accumulator1196::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.064_f32 + y.sin();
        let b = y * 8.899_f32 - x.cos();
        let mut acc = Accumulator1196::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.688_f32 + y.sin();
        let b = y * 6.811_f32 - x.cos();
        let mut acc = Accumulator1196::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.497_f32 + y.sin();
        let b = y * 6.132_f32 - x.cos();
        let mut acc = Accumulator1196::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.907_f32 + y.sin();
        let b = y * 5.146_f32 - x.cos();
        let mut acc = Accumulator1196::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.073_f32 + y.sin();
        let b = y * 8.034_f32 - x.cos();
        let mut acc = Accumulator1196::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.176_f32 + y.sin();
        let b = y * 1.146_f32 - x.cos();
        let mut acc = Accumulator1196::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.372_f32 + y.sin();
        let b = y * 2.881_f32 - x.cos();
        let mut acc = Accumulator1196::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.084_f32 + y.sin();
        let b = y * 1.442_f32 - x.cos();
        let mut acc = Accumulator1196::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.532_f32 + y.sin();
        let b = y * 2.591_f32 - x.cos();
        let mut acc = Accumulator1196::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.065_f32 + y.sin();
        let b = y * 7.299_f32 - x.cos();
        let mut acc = Accumulator1196::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.949_f32 + y.sin();
        let b = y * 3.469_f32 - x.cos();
        let mut acc = Accumulator1196::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.538_f32 + y.sin();
        let b = y * 1.517_f32 - x.cos();
        let mut acc = Accumulator1196::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.328_f32 + y.sin();
        let b = y * 6.573_f32 - x.cos();
        let mut acc = Accumulator1196::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.643_f32 + y.sin();
        let b = y * 5.572_f32 - x.cos();
        let mut acc = Accumulator1196::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.655_f32 + y.sin();
        let b = y * 7.448_f32 - x.cos();
        let mut acc = Accumulator1196::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.242_f32 + y.sin();
        let b = y * 4.766_f32 - x.cos();
        let mut acc = Accumulator1196::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.318_f32 + y.sin();
        let b = y * 6.333_f32 - x.cos();
        let mut acc = Accumulator1196::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.611_f32 + y.sin();
        let b = y * 9.856_f32 - x.cos();
        let mut acc = Accumulator1196::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1196(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_1196() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1196(total as u64) % 997) as f32;
        total
    }
}

pub mod m1197 {
    use super::*;

    pub struct Accumulator1197<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1197<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.197_f32 + y.sin();
        let b = y * 6.448_f32 - x.cos();
        let mut acc = Accumulator1197::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.498_f32 + y.sin();
        let b = y * 2.018_f32 - x.cos();
        let mut acc = Accumulator1197::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.553_f32 + y.sin();
        let b = y * 7.708_f32 - x.cos();
        let mut acc = Accumulator1197::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.875_f32 + y.sin();
        let b = y * 7.01_f32 - x.cos();
        let mut acc = Accumulator1197::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.486_f32 + y.sin();
        let b = y * 0.391_f32 - x.cos();
        let mut acc = Accumulator1197::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.946_f32 + y.sin();
        let b = y * 9.416_f32 - x.cos();
        let mut acc = Accumulator1197::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.351_f32 + y.sin();
        let b = y * 7.72_f32 - x.cos();
        let mut acc = Accumulator1197::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.674_f32 + y.sin();
        let b = y * 2.861_f32 - x.cos();
        let mut acc = Accumulator1197::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.011_f32 + y.sin();
        let b = y * 0.923_f32 - x.cos();
        let mut acc = Accumulator1197::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.94_f32 + y.sin();
        let b = y * 4.417_f32 - x.cos();
        let mut acc = Accumulator1197::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.186_f32 + y.sin();
        let b = y * 6.989_f32 - x.cos();
        let mut acc = Accumulator1197::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.366_f32 + y.sin();
        let b = y * 1.468_f32 - x.cos();
        let mut acc = Accumulator1197::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.493_f32 + y.sin();
        let b = y * 2.585_f32 - x.cos();
        let mut acc = Accumulator1197::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.996_f32 + y.sin();
        let b = y * 2.876_f32 - x.cos();
        let mut acc = Accumulator1197::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.422_f32 + y.sin();
        let b = y * 5.976_f32 - x.cos();
        let mut acc = Accumulator1197::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.17_f32 + y.sin();
        let b = y * 8.76_f32 - x.cos();
        let mut acc = Accumulator1197::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.544_f32 + y.sin();
        let b = y * 0.613_f32 - x.cos();
        let mut acc = Accumulator1197::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.773_f32 + y.sin();
        let b = y * 9.412_f32 - x.cos();
        let mut acc = Accumulator1197::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.124_f32 + y.sin();
        let b = y * 7.728_f32 - x.cos();
        let mut acc = Accumulator1197::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.821_f32 + y.sin();
        let b = y * 9.722_f32 - x.cos();
        let mut acc = Accumulator1197::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1197(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_1197() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1197(total as u64) % 997) as f32;
        total
    }
}

pub mod m1198 {
    use super::*;

    pub struct Accumulator1198<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1198<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.976_f32 + y.sin();
        let b = y * 6.352_f32 - x.cos();
        let mut acc = Accumulator1198::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.886_f32 + y.sin();
        let b = y * 8.955_f32 - x.cos();
        let mut acc = Accumulator1198::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.002_f32 + y.sin();
        let b = y * 1.097_f32 - x.cos();
        let mut acc = Accumulator1198::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.891_f32 + y.sin();
        let b = y * 6.2_f32 - x.cos();
        let mut acc = Accumulator1198::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.42_f32 + y.sin();
        let b = y * 9.014_f32 - x.cos();
        let mut acc = Accumulator1198::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.397_f32 + y.sin();
        let b = y * 6.308_f32 - x.cos();
        let mut acc = Accumulator1198::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.695_f32 + y.sin();
        let b = y * 5.227_f32 - x.cos();
        let mut acc = Accumulator1198::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.981_f32 + y.sin();
        let b = y * 6.825_f32 - x.cos();
        let mut acc = Accumulator1198::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.974_f32 + y.sin();
        let b = y * 6.246_f32 - x.cos();
        let mut acc = Accumulator1198::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.88_f32 + y.sin();
        let b = y * 9.705_f32 - x.cos();
        let mut acc = Accumulator1198::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.196_f32 + y.sin();
        let b = y * 0.812_f32 - x.cos();
        let mut acc = Accumulator1198::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.971_f32 + y.sin();
        let b = y * 1.636_f32 - x.cos();
        let mut acc = Accumulator1198::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.216_f32 + y.sin();
        let b = y * 3.236_f32 - x.cos();
        let mut acc = Accumulator1198::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.977_f32 + y.sin();
        let b = y * 0.77_f32 - x.cos();
        let mut acc = Accumulator1198::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.725_f32 + y.sin();
        let b = y * 1.378_f32 - x.cos();
        let mut acc = Accumulator1198::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.001_f32 + y.sin();
        let b = y * 7.568_f32 - x.cos();
        let mut acc = Accumulator1198::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.355_f32 + y.sin();
        let b = y * 6.877_f32 - x.cos();
        let mut acc = Accumulator1198::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.115_f32 + y.sin();
        let b = y * 1.25_f32 - x.cos();
        let mut acc = Accumulator1198::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.668_f32 + y.sin();
        let b = y * 7.071_f32 - x.cos();
        let mut acc = Accumulator1198::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.242_f32 + y.sin();
        let b = y * 3.956_f32 - x.cos();
        let mut acc = Accumulator1198::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1198(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m1198-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_1198() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1198(total as u64) % 997) as f32;
        total
    }
}

pub mod m1199 {
    use super::*;

    pub struct Accumulator1199<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1199<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.823_f32 + y.sin();
        let b = y * 0.496_f32 - x.cos();
        let mut acc = Accumulator1199::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.235_f32 + y.sin();
        let b = y * 0.536_f32 - x.cos();
        let mut acc = Accumulator1199::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.841_f32 + y.sin();
        let b = y * 4.552_f32 - x.cos();
        let mut acc = Accumulator1199::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.453_f32 + y.sin();
        let b = y * 7.328_f32 - x.cos();
        let mut acc = Accumulator1199::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.403_f32 + y.sin();
        let b = y * 6.546_f32 - x.cos();
        let mut acc = Accumulator1199::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.484_f32 + y.sin();
        let b = y * 2.697_f32 - x.cos();
        let mut acc = Accumulator1199::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.412_f32 + y.sin();
        let b = y * 6.579_f32 - x.cos();
        let mut acc = Accumulator1199::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.848_f32 + y.sin();
        let b = y * 2.657_f32 - x.cos();
        let mut acc = Accumulator1199::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.622_f32 + y.sin();
        let b = y * 3.804_f32 - x.cos();
        let mut acc = Accumulator1199::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.907_f32 + y.sin();
        let b = y * 4.75_f32 - x.cos();
        let mut acc = Accumulator1199::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.997_f32 + y.sin();
        let b = y * 2.573_f32 - x.cos();
        let mut acc = Accumulator1199::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.71_f32 + y.sin();
        let b = y * 6.982_f32 - x.cos();
        let mut acc = Accumulator1199::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.294_f32 + y.sin();
        let b = y * 7.897_f32 - x.cos();
        let mut acc = Accumulator1199::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.671_f32 + y.sin();
        let b = y * 8.145_f32 - x.cos();
        let mut acc = Accumulator1199::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.638_f32 + y.sin();
        let b = y * 5.887_f32 - x.cos();
        let mut acc = Accumulator1199::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.592_f32 + y.sin();
        let b = y * 2.891_f32 - x.cos();
        let mut acc = Accumulator1199::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.596_f32 + y.sin();
        let b = y * 6.919_f32 - x.cos();
        let mut acc = Accumulator1199::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.015_f32 + y.sin();
        let b = y * 4.279_f32 - x.cos();
        let mut acc = Accumulator1199::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.077_f32 + y.sin();
        let b = y * 2.803_f32 - x.cos();
        let mut acc = Accumulator1199::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.588_f32 + y.sin();
        let b = y * 5.353_f32 - x.cos();
        let mut acc = Accumulator1199::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1199(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_1199() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1199(total as u64) % 997) as f32;
        total
    }
}

pub fn touch_bulk_11() -> f32 {
    let mut total = 0.0_f32;
    total += m1100::run_all_1100();
    total += m1101::run_all_1101();
    total += m1102::run_all_1102();
    total += m1103::run_all_1103();
    total += m1104::run_all_1104();
    total += m1105::run_all_1105();
    total += m1106::run_all_1106();
    total += m1107::run_all_1107();
    total += m1108::run_all_1108();
    total += m1109::run_all_1109();
    total += m1110::run_all_1110();
    total += m1111::run_all_1111();
    total += m1112::run_all_1112();
    total += m1113::run_all_1113();
    total += m1114::run_all_1114();
    total += m1115::run_all_1115();
    total += m1116::run_all_1116();
    total += m1117::run_all_1117();
    total += m1118::run_all_1118();
    total += m1119::run_all_1119();
    total += m1120::run_all_1120();
    total += m1121::run_all_1121();
    total += m1122::run_all_1122();
    total += m1123::run_all_1123();
    total += m1124::run_all_1124();
    total += m1125::run_all_1125();
    total += m1126::run_all_1126();
    total += m1127::run_all_1127();
    total += m1128::run_all_1128();
    total += m1129::run_all_1129();
    total += m1130::run_all_1130();
    total += m1131::run_all_1131();
    total += m1132::run_all_1132();
    total += m1133::run_all_1133();
    total += m1134::run_all_1134();
    total += m1135::run_all_1135();
    total += m1136::run_all_1136();
    total += m1137::run_all_1137();
    total += m1138::run_all_1138();
    total += m1139::run_all_1139();
    total += m1140::run_all_1140();
    total += m1141::run_all_1141();
    total += m1142::run_all_1142();
    total += m1143::run_all_1143();
    total += m1144::run_all_1144();
    total += m1145::run_all_1145();
    total += m1146::run_all_1146();
    total += m1147::run_all_1147();
    total += m1148::run_all_1148();
    total += m1149::run_all_1149();
    total += m1150::run_all_1150();
    total += m1151::run_all_1151();
    total += m1152::run_all_1152();
    total += m1153::run_all_1153();
    total += m1154::run_all_1154();
    total += m1155::run_all_1155();
    total += m1156::run_all_1156();
    total += m1157::run_all_1157();
    total += m1158::run_all_1158();
    total += m1159::run_all_1159();
    total += m1160::run_all_1160();
    total += m1161::run_all_1161();
    total += m1162::run_all_1162();
    total += m1163::run_all_1163();
    total += m1164::run_all_1164();
    total += m1165::run_all_1165();
    total += m1166::run_all_1166();
    total += m1167::run_all_1167();
    total += m1168::run_all_1168();
    total += m1169::run_all_1169();
    total += m1170::run_all_1170();
    total += m1171::run_all_1171();
    total += m1172::run_all_1172();
    total += m1173::run_all_1173();
    total += m1174::run_all_1174();
    total += m1175::run_all_1175();
    total += m1176::run_all_1176();
    total += m1177::run_all_1177();
    total += m1178::run_all_1178();
    total += m1179::run_all_1179();
    total += m1180::run_all_1180();
    total += m1181::run_all_1181();
    total += m1182::run_all_1182();
    total += m1183::run_all_1183();
    total += m1184::run_all_1184();
    total += m1185::run_all_1185();
    total += m1186::run_all_1186();
    total += m1187::run_all_1187();
    total += m1188::run_all_1188();
    total += m1189::run_all_1189();
    total += m1190::run_all_1190();
    total += m1191::run_all_1191();
    total += m1192::run_all_1192();
    total += m1193::run_all_1193();
    total += m1194::run_all_1194();
    total += m1195::run_all_1195();
    total += m1196::run_all_1196();
    total += m1197::run_all_1197();
    total += m1198::run_all_1198();
    total += m1199::run_all_1199();
    total
}
