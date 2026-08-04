//! Auto-generated bulk module (file 1) - exists purely to make `game_rs`
//! heavier for a hot-patch speed experiment. Real arithmetic, real generics
//! (so monomorphization isn't free), and real use of a rotating set of
//! external crates so none of them are dead code. `touch_bulk_1()` is
//! called once from `game::init` so nothing here gets stripped as unreachable.

use rand::{Rng, SeedableRng};
use sha2::{Digest, Sha256};
use noise::{NoiseFn, Perlin};
use regex::Regex;
use itertools::Itertools;
use uuid::Uuid;
use chrono::Utc;


pub mod m100 {
    use super::*;

    pub struct Accumulator100<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator100<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.897_f32 + y.sin();
        let b = y * 1.646_f32 - x.cos();
        let mut acc = Accumulator100::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.695_f32 + y.sin();
        let b = y * 6.712_f32 - x.cos();
        let mut acc = Accumulator100::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.349_f32 + y.sin();
        let b = y * 8.446_f32 - x.cos();
        let mut acc = Accumulator100::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.287_f32 + y.sin();
        let b = y * 1.985_f32 - x.cos();
        let mut acc = Accumulator100::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.091_f32 + y.sin();
        let b = y * 1.191_f32 - x.cos();
        let mut acc = Accumulator100::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.407_f32 + y.sin();
        let b = y * 3.995_f32 - x.cos();
        let mut acc = Accumulator100::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.786_f32 + y.sin();
        let b = y * 9.771_f32 - x.cos();
        let mut acc = Accumulator100::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.923_f32 + y.sin();
        let b = y * 3.817_f32 - x.cos();
        let mut acc = Accumulator100::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.31_f32 + y.sin();
        let b = y * 1.218_f32 - x.cos();
        let mut acc = Accumulator100::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.363_f32 + y.sin();
        let b = y * 6.76_f32 - x.cos();
        let mut acc = Accumulator100::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.122_f32 + y.sin();
        let b = y * 2.127_f32 - x.cos();
        let mut acc = Accumulator100::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.709_f32 + y.sin();
        let b = y * 6.088_f32 - x.cos();
        let mut acc = Accumulator100::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.064_f32 + y.sin();
        let b = y * 7.04_f32 - x.cos();
        let mut acc = Accumulator100::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.387_f32 + y.sin();
        let b = y * 5.843_f32 - x.cos();
        let mut acc = Accumulator100::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.744_f32 + y.sin();
        let b = y * 1.289_f32 - x.cos();
        let mut acc = Accumulator100::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.288_f32 + y.sin();
        let b = y * 6.415_f32 - x.cos();
        let mut acc = Accumulator100::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.982_f32 + y.sin();
        let b = y * 4.61_f32 - x.cos();
        let mut acc = Accumulator100::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.186_f32 + y.sin();
        let b = y * 3.151_f32 - x.cos();
        let mut acc = Accumulator100::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.068_f32 + y.sin();
        let b = y * 0.251_f32 - x.cos();
        let mut acc = Accumulator100::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.237_f32 + y.sin();
        let b = y * 9.767_f32 - x.cos();
        let mut acc = Accumulator100::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_100(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_100() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_100(total as u64) % 997) as f32;
        total
    }
}

pub mod m101 {
    use super::*;

    pub struct Accumulator101<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator101<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.508_f32 + y.sin();
        let b = y * 4.239_f32 - x.cos();
        let mut acc = Accumulator101::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.855_f32 + y.sin();
        let b = y * 3.797_f32 - x.cos();
        let mut acc = Accumulator101::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.193_f32 + y.sin();
        let b = y * 5.491_f32 - x.cos();
        let mut acc = Accumulator101::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.738_f32 + y.sin();
        let b = y * 7.706_f32 - x.cos();
        let mut acc = Accumulator101::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.684_f32 + y.sin();
        let b = y * 0.233_f32 - x.cos();
        let mut acc = Accumulator101::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.827_f32 + y.sin();
        let b = y * 2.333_f32 - x.cos();
        let mut acc = Accumulator101::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.817_f32 + y.sin();
        let b = y * 5.972_f32 - x.cos();
        let mut acc = Accumulator101::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.474_f32 + y.sin();
        let b = y * 3.895_f32 - x.cos();
        let mut acc = Accumulator101::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.693_f32 + y.sin();
        let b = y * 7.536_f32 - x.cos();
        let mut acc = Accumulator101::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.416_f32 + y.sin();
        let b = y * 9.831_f32 - x.cos();
        let mut acc = Accumulator101::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.145_f32 + y.sin();
        let b = y * 8.638_f32 - x.cos();
        let mut acc = Accumulator101::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.121_f32 + y.sin();
        let b = y * 1.016_f32 - x.cos();
        let mut acc = Accumulator101::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.901_f32 + y.sin();
        let b = y * 0.147_f32 - x.cos();
        let mut acc = Accumulator101::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.076_f32 + y.sin();
        let b = y * 2.935_f32 - x.cos();
        let mut acc = Accumulator101::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.509_f32 + y.sin();
        let b = y * 3.711_f32 - x.cos();
        let mut acc = Accumulator101::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.758_f32 + y.sin();
        let b = y * 5.104_f32 - x.cos();
        let mut acc = Accumulator101::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.217_f32 + y.sin();
        let b = y * 2.944_f32 - x.cos();
        let mut acc = Accumulator101::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.153_f32 + y.sin();
        let b = y * 8.701_f32 - x.cos();
        let mut acc = Accumulator101::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.409_f32 + y.sin();
        let b = y * 5.999_f32 - x.cos();
        let mut acc = Accumulator101::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.621_f32 + y.sin();
        let b = y * 2.883_f32 - x.cos();
        let mut acc = Accumulator101::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_101(seed: u64) -> u64 {
        let re = Regex::new(r"m101-(\d+)").unwrap();
        let hay = format!("m101-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_101() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_101(total as u64) % 997) as f32;
        total
    }
}

pub mod m102 {
    use super::*;

    pub struct Accumulator102<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator102<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.158_f32 + y.sin();
        let b = y * 7.123_f32 - x.cos();
        let mut acc = Accumulator102::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.687_f32 + y.sin();
        let b = y * 9.746_f32 - x.cos();
        let mut acc = Accumulator102::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.31_f32 + y.sin();
        let b = y * 3.319_f32 - x.cos();
        let mut acc = Accumulator102::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.418_f32 + y.sin();
        let b = y * 4.191_f32 - x.cos();
        let mut acc = Accumulator102::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.278_f32 + y.sin();
        let b = y * 0.136_f32 - x.cos();
        let mut acc = Accumulator102::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.038_f32 + y.sin();
        let b = y * 5.309_f32 - x.cos();
        let mut acc = Accumulator102::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.078_f32 + y.sin();
        let b = y * 2.381_f32 - x.cos();
        let mut acc = Accumulator102::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.581_f32 + y.sin();
        let b = y * 8.253_f32 - x.cos();
        let mut acc = Accumulator102::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.088_f32 + y.sin();
        let b = y * 0.671_f32 - x.cos();
        let mut acc = Accumulator102::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.253_f32 + y.sin();
        let b = y * 9.514_f32 - x.cos();
        let mut acc = Accumulator102::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.035_f32 + y.sin();
        let b = y * 0.513_f32 - x.cos();
        let mut acc = Accumulator102::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.729_f32 + y.sin();
        let b = y * 2.036_f32 - x.cos();
        let mut acc = Accumulator102::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.761_f32 + y.sin();
        let b = y * 1.81_f32 - x.cos();
        let mut acc = Accumulator102::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.635_f32 + y.sin();
        let b = y * 6.417_f32 - x.cos();
        let mut acc = Accumulator102::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.981_f32 + y.sin();
        let b = y * 7.65_f32 - x.cos();
        let mut acc = Accumulator102::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.093_f32 + y.sin();
        let b = y * 3.177_f32 - x.cos();
        let mut acc = Accumulator102::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.335_f32 + y.sin();
        let b = y * 7.122_f32 - x.cos();
        let mut acc = Accumulator102::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.143_f32 + y.sin();
        let b = y * 1.421_f32 - x.cos();
        let mut acc = Accumulator102::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.05_f32 + y.sin();
        let b = y * 2.071_f32 - x.cos();
        let mut acc = Accumulator102::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.818_f32 + y.sin();
        let b = y * 9.315_f32 - x.cos();
        let mut acc = Accumulator102::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_102(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_102() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_102(total as u64) % 997) as f32;
        total
    }
}

pub mod m103 {
    use super::*;

    pub struct Accumulator103<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator103<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.684_f32 + y.sin();
        let b = y * 7.381_f32 - x.cos();
        let mut acc = Accumulator103::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.654_f32 + y.sin();
        let b = y * 7.481_f32 - x.cos();
        let mut acc = Accumulator103::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.937_f32 + y.sin();
        let b = y * 5.488_f32 - x.cos();
        let mut acc = Accumulator103::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.157_f32 + y.sin();
        let b = y * 6.498_f32 - x.cos();
        let mut acc = Accumulator103::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.892_f32 + y.sin();
        let b = y * 6.222_f32 - x.cos();
        let mut acc = Accumulator103::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.252_f32 + y.sin();
        let b = y * 9.626_f32 - x.cos();
        let mut acc = Accumulator103::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.546_f32 + y.sin();
        let b = y * 8.003_f32 - x.cos();
        let mut acc = Accumulator103::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.837_f32 + y.sin();
        let b = y * 8.213_f32 - x.cos();
        let mut acc = Accumulator103::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.329_f32 + y.sin();
        let b = y * 6.162_f32 - x.cos();
        let mut acc = Accumulator103::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.412_f32 + y.sin();
        let b = y * 8.236_f32 - x.cos();
        let mut acc = Accumulator103::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.548_f32 + y.sin();
        let b = y * 2.439_f32 - x.cos();
        let mut acc = Accumulator103::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.09_f32 + y.sin();
        let b = y * 8.906_f32 - x.cos();
        let mut acc = Accumulator103::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.012_f32 + y.sin();
        let b = y * 1.391_f32 - x.cos();
        let mut acc = Accumulator103::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.748_f32 + y.sin();
        let b = y * 0.807_f32 - x.cos();
        let mut acc = Accumulator103::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.151_f32 + y.sin();
        let b = y * 0.596_f32 - x.cos();
        let mut acc = Accumulator103::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.967_f32 + y.sin();
        let b = y * 2.832_f32 - x.cos();
        let mut acc = Accumulator103::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.973_f32 + y.sin();
        let b = y * 8.627_f32 - x.cos();
        let mut acc = Accumulator103::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.009_f32 + y.sin();
        let b = y * 9.613_f32 - x.cos();
        let mut acc = Accumulator103::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.18_f32 + y.sin();
        let b = y * 8.045_f32 - x.cos();
        let mut acc = Accumulator103::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.906_f32 + y.sin();
        let b = y * 3.117_f32 - x.cos();
        let mut acc = Accumulator103::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_103(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(103u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_103() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_103(total as u64) % 997) as f32;
        total
    }
}

pub mod m104 {
    use super::*;

    pub struct Accumulator104<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator104<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.511_f32 + y.sin();
        let b = y * 6.101_f32 - x.cos();
        let mut acc = Accumulator104::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.842_f32 + y.sin();
        let b = y * 5.42_f32 - x.cos();
        let mut acc = Accumulator104::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.803_f32 + y.sin();
        let b = y * 4.568_f32 - x.cos();
        let mut acc = Accumulator104::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.306_f32 + y.sin();
        let b = y * 2.26_f32 - x.cos();
        let mut acc = Accumulator104::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.4_f32 + y.sin();
        let b = y * 8.246_f32 - x.cos();
        let mut acc = Accumulator104::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.405_f32 + y.sin();
        let b = y * 3.095_f32 - x.cos();
        let mut acc = Accumulator104::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.296_f32 + y.sin();
        let b = y * 2.229_f32 - x.cos();
        let mut acc = Accumulator104::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.623_f32 + y.sin();
        let b = y * 3.51_f32 - x.cos();
        let mut acc = Accumulator104::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.36_f32 + y.sin();
        let b = y * 7.485_f32 - x.cos();
        let mut acc = Accumulator104::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.91_f32 + y.sin();
        let b = y * 7.037_f32 - x.cos();
        let mut acc = Accumulator104::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.155_f32 + y.sin();
        let b = y * 3.505_f32 - x.cos();
        let mut acc = Accumulator104::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.04_f32 + y.sin();
        let b = y * 9.247_f32 - x.cos();
        let mut acc = Accumulator104::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.242_f32 + y.sin();
        let b = y * 1.61_f32 - x.cos();
        let mut acc = Accumulator104::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.724_f32 + y.sin();
        let b = y * 5.067_f32 - x.cos();
        let mut acc = Accumulator104::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.506_f32 + y.sin();
        let b = y * 2.538_f32 - x.cos();
        let mut acc = Accumulator104::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.062_f32 + y.sin();
        let b = y * 1.025_f32 - x.cos();
        let mut acc = Accumulator104::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.516_f32 + y.sin();
        let b = y * 1.288_f32 - x.cos();
        let mut acc = Accumulator104::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.793_f32 + y.sin();
        let b = y * 5.771_f32 - x.cos();
        let mut acc = Accumulator104::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.194_f32 + y.sin();
        let b = y * 8.013_f32 - x.cos();
        let mut acc = Accumulator104::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.588_f32 + y.sin();
        let b = y * 8.241_f32 - x.cos();
        let mut acc = Accumulator104::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_104(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_104() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_104(total as u64) % 997) as f32;
        total
    }
}

pub mod m105 {
    use super::*;

    pub struct Accumulator105<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator105<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.515_f32 + y.sin();
        let b = y * 5.62_f32 - x.cos();
        let mut acc = Accumulator105::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.967_f32 + y.sin();
        let b = y * 1.765_f32 - x.cos();
        let mut acc = Accumulator105::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.555_f32 + y.sin();
        let b = y * 7.806_f32 - x.cos();
        let mut acc = Accumulator105::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.367_f32 + y.sin();
        let b = y * 4.677_f32 - x.cos();
        let mut acc = Accumulator105::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.092_f32 + y.sin();
        let b = y * 4.381_f32 - x.cos();
        let mut acc = Accumulator105::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.63_f32 + y.sin();
        let b = y * 1.789_f32 - x.cos();
        let mut acc = Accumulator105::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.214_f32 + y.sin();
        let b = y * 6.415_f32 - x.cos();
        let mut acc = Accumulator105::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.902_f32 + y.sin();
        let b = y * 1.559_f32 - x.cos();
        let mut acc = Accumulator105::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.175_f32 + y.sin();
        let b = y * 8.34_f32 - x.cos();
        let mut acc = Accumulator105::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.755_f32 + y.sin();
        let b = y * 5.37_f32 - x.cos();
        let mut acc = Accumulator105::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.429_f32 + y.sin();
        let b = y * 4.864_f32 - x.cos();
        let mut acc = Accumulator105::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.264_f32 + y.sin();
        let b = y * 9.214_f32 - x.cos();
        let mut acc = Accumulator105::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.454_f32 + y.sin();
        let b = y * 8.817_f32 - x.cos();
        let mut acc = Accumulator105::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.685_f32 + y.sin();
        let b = y * 5.371_f32 - x.cos();
        let mut acc = Accumulator105::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.43_f32 + y.sin();
        let b = y * 9.762_f32 - x.cos();
        let mut acc = Accumulator105::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.224_f32 + y.sin();
        let b = y * 2.337_f32 - x.cos();
        let mut acc = Accumulator105::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.785_f32 + y.sin();
        let b = y * 3.498_f32 - x.cos();
        let mut acc = Accumulator105::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.803_f32 + y.sin();
        let b = y * 0.739_f32 - x.cos();
        let mut acc = Accumulator105::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.645_f32 + y.sin();
        let b = y * 3.109_f32 - x.cos();
        let mut acc = Accumulator105::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.497_f32 + y.sin();
        let b = y * 7.284_f32 - x.cos();
        let mut acc = Accumulator105::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_105(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_105() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_105(total as u64) % 997) as f32;
        total
    }
}

pub mod m106 {
    use super::*;

    pub struct Accumulator106<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator106<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.687_f32 + y.sin();
        let b = y * 5.711_f32 - x.cos();
        let mut acc = Accumulator106::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.035_f32 + y.sin();
        let b = y * 2.117_f32 - x.cos();
        let mut acc = Accumulator106::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.576_f32 + y.sin();
        let b = y * 0.946_f32 - x.cos();
        let mut acc = Accumulator106::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.887_f32 + y.sin();
        let b = y * 0.102_f32 - x.cos();
        let mut acc = Accumulator106::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.287_f32 + y.sin();
        let b = y * 2.478_f32 - x.cos();
        let mut acc = Accumulator106::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.153_f32 + y.sin();
        let b = y * 7.493_f32 - x.cos();
        let mut acc = Accumulator106::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.222_f32 + y.sin();
        let b = y * 7.447_f32 - x.cos();
        let mut acc = Accumulator106::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.932_f32 + y.sin();
        let b = y * 7.747_f32 - x.cos();
        let mut acc = Accumulator106::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.148_f32 + y.sin();
        let b = y * 0.459_f32 - x.cos();
        let mut acc = Accumulator106::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.905_f32 + y.sin();
        let b = y * 6.483_f32 - x.cos();
        let mut acc = Accumulator106::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.946_f32 + y.sin();
        let b = y * 3.162_f32 - x.cos();
        let mut acc = Accumulator106::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.163_f32 + y.sin();
        let b = y * 8.019_f32 - x.cos();
        let mut acc = Accumulator106::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.419_f32 + y.sin();
        let b = y * 4.383_f32 - x.cos();
        let mut acc = Accumulator106::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.242_f32 + y.sin();
        let b = y * 4.765_f32 - x.cos();
        let mut acc = Accumulator106::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.214_f32 + y.sin();
        let b = y * 8.976_f32 - x.cos();
        let mut acc = Accumulator106::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.169_f32 + y.sin();
        let b = y * 0.581_f32 - x.cos();
        let mut acc = Accumulator106::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.621_f32 + y.sin();
        let b = y * 5.733_f32 - x.cos();
        let mut acc = Accumulator106::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.854_f32 + y.sin();
        let b = y * 5.084_f32 - x.cos();
        let mut acc = Accumulator106::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.883_f32 + y.sin();
        let b = y * 3.716_f32 - x.cos();
        let mut acc = Accumulator106::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.687_f32 + y.sin();
        let b = y * 0.23_f32 - x.cos();
        let mut acc = Accumulator106::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_106(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m106-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_106() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_106(total as u64) % 997) as f32;
        total
    }
}

pub mod m107 {
    use super::*;

    pub struct Accumulator107<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator107<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.885_f32 + y.sin();
        let b = y * 1.647_f32 - x.cos();
        let mut acc = Accumulator107::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.706_f32 + y.sin();
        let b = y * 1.384_f32 - x.cos();
        let mut acc = Accumulator107::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.354_f32 + y.sin();
        let b = y * 6.328_f32 - x.cos();
        let mut acc = Accumulator107::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.499_f32 + y.sin();
        let b = y * 3.952_f32 - x.cos();
        let mut acc = Accumulator107::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.546_f32 + y.sin();
        let b = y * 0.315_f32 - x.cos();
        let mut acc = Accumulator107::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.158_f32 + y.sin();
        let b = y * 5.195_f32 - x.cos();
        let mut acc = Accumulator107::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.4_f32 + y.sin();
        let b = y * 1.356_f32 - x.cos();
        let mut acc = Accumulator107::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.874_f32 + y.sin();
        let b = y * 0.522_f32 - x.cos();
        let mut acc = Accumulator107::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.096_f32 + y.sin();
        let b = y * 5.449_f32 - x.cos();
        let mut acc = Accumulator107::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.233_f32 + y.sin();
        let b = y * 7.265_f32 - x.cos();
        let mut acc = Accumulator107::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.857_f32 + y.sin();
        let b = y * 3.03_f32 - x.cos();
        let mut acc = Accumulator107::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.083_f32 + y.sin();
        let b = y * 7.007_f32 - x.cos();
        let mut acc = Accumulator107::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.971_f32 + y.sin();
        let b = y * 5.013_f32 - x.cos();
        let mut acc = Accumulator107::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.872_f32 + y.sin();
        let b = y * 1.254_f32 - x.cos();
        let mut acc = Accumulator107::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.041_f32 + y.sin();
        let b = y * 9.579_f32 - x.cos();
        let mut acc = Accumulator107::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.146_f32 + y.sin();
        let b = y * 9.17_f32 - x.cos();
        let mut acc = Accumulator107::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.594_f32 + y.sin();
        let b = y * 3.253_f32 - x.cos();
        let mut acc = Accumulator107::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.078_f32 + y.sin();
        let b = y * 3.811_f32 - x.cos();
        let mut acc = Accumulator107::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.502_f32 + y.sin();
        let b = y * 2.832_f32 - x.cos();
        let mut acc = Accumulator107::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.846_f32 + y.sin();
        let b = y * 6.486_f32 - x.cos();
        let mut acc = Accumulator107::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_107(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_107() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_107(total as u64) % 997) as f32;
        total
    }
}

pub mod m108 {
    use super::*;

    pub struct Accumulator108<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator108<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.665_f32 + y.sin();
        let b = y * 2.09_f32 - x.cos();
        let mut acc = Accumulator108::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.92_f32 + y.sin();
        let b = y * 6.074_f32 - x.cos();
        let mut acc = Accumulator108::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.596_f32 + y.sin();
        let b = y * 5.189_f32 - x.cos();
        let mut acc = Accumulator108::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.557_f32 + y.sin();
        let b = y * 2.773_f32 - x.cos();
        let mut acc = Accumulator108::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.303_f32 + y.sin();
        let b = y * 0.263_f32 - x.cos();
        let mut acc = Accumulator108::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.432_f32 + y.sin();
        let b = y * 4.455_f32 - x.cos();
        let mut acc = Accumulator108::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.027_f32 + y.sin();
        let b = y * 4.41_f32 - x.cos();
        let mut acc = Accumulator108::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.103_f32 + y.sin();
        let b = y * 7.942_f32 - x.cos();
        let mut acc = Accumulator108::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.055_f32 + y.sin();
        let b = y * 7.115_f32 - x.cos();
        let mut acc = Accumulator108::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.044_f32 + y.sin();
        let b = y * 3.668_f32 - x.cos();
        let mut acc = Accumulator108::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.969_f32 + y.sin();
        let b = y * 0.366_f32 - x.cos();
        let mut acc = Accumulator108::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.28_f32 + y.sin();
        let b = y * 0.899_f32 - x.cos();
        let mut acc = Accumulator108::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.517_f32 + y.sin();
        let b = y * 6.023_f32 - x.cos();
        let mut acc = Accumulator108::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.442_f32 + y.sin();
        let b = y * 8.147_f32 - x.cos();
        let mut acc = Accumulator108::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.423_f32 + y.sin();
        let b = y * 5.843_f32 - x.cos();
        let mut acc = Accumulator108::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.328_f32 + y.sin();
        let b = y * 0.56_f32 - x.cos();
        let mut acc = Accumulator108::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.669_f32 + y.sin();
        let b = y * 5.423_f32 - x.cos();
        let mut acc = Accumulator108::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.101_f32 + y.sin();
        let b = y * 5.957_f32 - x.cos();
        let mut acc = Accumulator108::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.084_f32 + y.sin();
        let b = y * 0.727_f32 - x.cos();
        let mut acc = Accumulator108::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.206_f32 + y.sin();
        let b = y * 8.316_f32 - x.cos();
        let mut acc = Accumulator108::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_108(seed: u64) -> u64 {
        let re = Regex::new(r"m108-(\d+)").unwrap();
        let hay = format!("m108-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_108() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_108(total as u64) % 997) as f32;
        total
    }
}

pub mod m109 {
    use super::*;

    pub struct Accumulator109<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator109<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.741_f32 + y.sin();
        let b = y * 2.936_f32 - x.cos();
        let mut acc = Accumulator109::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.384_f32 + y.sin();
        let b = y * 2.86_f32 - x.cos();
        let mut acc = Accumulator109::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.288_f32 + y.sin();
        let b = y * 5.494_f32 - x.cos();
        let mut acc = Accumulator109::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.396_f32 + y.sin();
        let b = y * 0.938_f32 - x.cos();
        let mut acc = Accumulator109::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.958_f32 + y.sin();
        let b = y * 2.343_f32 - x.cos();
        let mut acc = Accumulator109::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.203_f32 + y.sin();
        let b = y * 8.259_f32 - x.cos();
        let mut acc = Accumulator109::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.151_f32 + y.sin();
        let b = y * 3.764_f32 - x.cos();
        let mut acc = Accumulator109::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.055_f32 + y.sin();
        let b = y * 5.128_f32 - x.cos();
        let mut acc = Accumulator109::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.714_f32 + y.sin();
        let b = y * 8.847_f32 - x.cos();
        let mut acc = Accumulator109::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.344_f32 + y.sin();
        let b = y * 2.62_f32 - x.cos();
        let mut acc = Accumulator109::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.956_f32 + y.sin();
        let b = y * 1.422_f32 - x.cos();
        let mut acc = Accumulator109::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.447_f32 + y.sin();
        let b = y * 8.035_f32 - x.cos();
        let mut acc = Accumulator109::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.249_f32 + y.sin();
        let b = y * 2.622_f32 - x.cos();
        let mut acc = Accumulator109::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.819_f32 + y.sin();
        let b = y * 2.639_f32 - x.cos();
        let mut acc = Accumulator109::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.21_f32 + y.sin();
        let b = y * 4.012_f32 - x.cos();
        let mut acc = Accumulator109::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.093_f32 + y.sin();
        let b = y * 1.041_f32 - x.cos();
        let mut acc = Accumulator109::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.95_f32 + y.sin();
        let b = y * 1.908_f32 - x.cos();
        let mut acc = Accumulator109::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.465_f32 + y.sin();
        let b = y * 6.836_f32 - x.cos();
        let mut acc = Accumulator109::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.881_f32 + y.sin();
        let b = y * 9.489_f32 - x.cos();
        let mut acc = Accumulator109::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.932_f32 + y.sin();
        let b = y * 4.263_f32 - x.cos();
        let mut acc = Accumulator109::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_109(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_109() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_109(total as u64) % 997) as f32;
        total
    }
}

pub mod m110 {
    use super::*;

    pub struct Accumulator110<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator110<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.632_f32 + y.sin();
        let b = y * 5.605_f32 - x.cos();
        let mut acc = Accumulator110::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.901_f32 + y.sin();
        let b = y * 6.552_f32 - x.cos();
        let mut acc = Accumulator110::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.151_f32 + y.sin();
        let b = y * 1.942_f32 - x.cos();
        let mut acc = Accumulator110::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.556_f32 + y.sin();
        let b = y * 7.719_f32 - x.cos();
        let mut acc = Accumulator110::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.663_f32 + y.sin();
        let b = y * 8.323_f32 - x.cos();
        let mut acc = Accumulator110::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.794_f32 + y.sin();
        let b = y * 0.846_f32 - x.cos();
        let mut acc = Accumulator110::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.298_f32 + y.sin();
        let b = y * 8.267_f32 - x.cos();
        let mut acc = Accumulator110::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.843_f32 + y.sin();
        let b = y * 4.6_f32 - x.cos();
        let mut acc = Accumulator110::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.162_f32 + y.sin();
        let b = y * 3.711_f32 - x.cos();
        let mut acc = Accumulator110::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.378_f32 + y.sin();
        let b = y * 6.661_f32 - x.cos();
        let mut acc = Accumulator110::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.968_f32 + y.sin();
        let b = y * 8.843_f32 - x.cos();
        let mut acc = Accumulator110::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.178_f32 + y.sin();
        let b = y * 3.398_f32 - x.cos();
        let mut acc = Accumulator110::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.241_f32 + y.sin();
        let b = y * 1.507_f32 - x.cos();
        let mut acc = Accumulator110::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.292_f32 + y.sin();
        let b = y * 8.409_f32 - x.cos();
        let mut acc = Accumulator110::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.609_f32 + y.sin();
        let b = y * 2.928_f32 - x.cos();
        let mut acc = Accumulator110::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.721_f32 + y.sin();
        let b = y * 3.969_f32 - x.cos();
        let mut acc = Accumulator110::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.402_f32 + y.sin();
        let b = y * 2.255_f32 - x.cos();
        let mut acc = Accumulator110::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.224_f32 + y.sin();
        let b = y * 8.543_f32 - x.cos();
        let mut acc = Accumulator110::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.806_f32 + y.sin();
        let b = y * 1.02_f32 - x.cos();
        let mut acc = Accumulator110::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.051_f32 + y.sin();
        let b = y * 2.457_f32 - x.cos();
        let mut acc = Accumulator110::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_110(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(110u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_110() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_110(total as u64) % 997) as f32;
        total
    }
}

pub mod m111 {
    use super::*;

    pub struct Accumulator111<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator111<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.076_f32 + y.sin();
        let b = y * 7.056_f32 - x.cos();
        let mut acc = Accumulator111::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.634_f32 + y.sin();
        let b = y * 0.985_f32 - x.cos();
        let mut acc = Accumulator111::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.91_f32 + y.sin();
        let b = y * 9.607_f32 - x.cos();
        let mut acc = Accumulator111::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.724_f32 + y.sin();
        let b = y * 0.412_f32 - x.cos();
        let mut acc = Accumulator111::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.593_f32 + y.sin();
        let b = y * 5.891_f32 - x.cos();
        let mut acc = Accumulator111::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.068_f32 + y.sin();
        let b = y * 8.034_f32 - x.cos();
        let mut acc = Accumulator111::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.855_f32 + y.sin();
        let b = y * 1.535_f32 - x.cos();
        let mut acc = Accumulator111::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.613_f32 + y.sin();
        let b = y * 5.39_f32 - x.cos();
        let mut acc = Accumulator111::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.488_f32 + y.sin();
        let b = y * 1.675_f32 - x.cos();
        let mut acc = Accumulator111::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.041_f32 + y.sin();
        let b = y * 5.717_f32 - x.cos();
        let mut acc = Accumulator111::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.257_f32 + y.sin();
        let b = y * 5.972_f32 - x.cos();
        let mut acc = Accumulator111::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.062_f32 + y.sin();
        let b = y * 5.654_f32 - x.cos();
        let mut acc = Accumulator111::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.565_f32 + y.sin();
        let b = y * 5.009_f32 - x.cos();
        let mut acc = Accumulator111::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.101_f32 + y.sin();
        let b = y * 3.366_f32 - x.cos();
        let mut acc = Accumulator111::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.388_f32 + y.sin();
        let b = y * 9.343_f32 - x.cos();
        let mut acc = Accumulator111::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.593_f32 + y.sin();
        let b = y * 9.286_f32 - x.cos();
        let mut acc = Accumulator111::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.465_f32 + y.sin();
        let b = y * 4.626_f32 - x.cos();
        let mut acc = Accumulator111::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.214_f32 + y.sin();
        let b = y * 1.176_f32 - x.cos();
        let mut acc = Accumulator111::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.818_f32 + y.sin();
        let b = y * 1.211_f32 - x.cos();
        let mut acc = Accumulator111::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.8_f32 + y.sin();
        let b = y * 8.847_f32 - x.cos();
        let mut acc = Accumulator111::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_111(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_111() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_111(total as u64) % 997) as f32;
        total
    }
}

pub mod m112 {
    use super::*;

    pub struct Accumulator112<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator112<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.153_f32 + y.sin();
        let b = y * 2.149_f32 - x.cos();
        let mut acc = Accumulator112::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.377_f32 + y.sin();
        let b = y * 9.404_f32 - x.cos();
        let mut acc = Accumulator112::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.467_f32 + y.sin();
        let b = y * 7.873_f32 - x.cos();
        let mut acc = Accumulator112::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.308_f32 + y.sin();
        let b = y * 8.073_f32 - x.cos();
        let mut acc = Accumulator112::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.81_f32 + y.sin();
        let b = y * 0.845_f32 - x.cos();
        let mut acc = Accumulator112::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.403_f32 + y.sin();
        let b = y * 8.76_f32 - x.cos();
        let mut acc = Accumulator112::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.138_f32 + y.sin();
        let b = y * 9.452_f32 - x.cos();
        let mut acc = Accumulator112::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.373_f32 + y.sin();
        let b = y * 1.455_f32 - x.cos();
        let mut acc = Accumulator112::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.846_f32 + y.sin();
        let b = y * 2.43_f32 - x.cos();
        let mut acc = Accumulator112::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.278_f32 + y.sin();
        let b = y * 4.174_f32 - x.cos();
        let mut acc = Accumulator112::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.427_f32 + y.sin();
        let b = y * 5.053_f32 - x.cos();
        let mut acc = Accumulator112::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.466_f32 + y.sin();
        let b = y * 7.821_f32 - x.cos();
        let mut acc = Accumulator112::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.085_f32 + y.sin();
        let b = y * 3.389_f32 - x.cos();
        let mut acc = Accumulator112::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.363_f32 + y.sin();
        let b = y * 8.354_f32 - x.cos();
        let mut acc = Accumulator112::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.515_f32 + y.sin();
        let b = y * 4.901_f32 - x.cos();
        let mut acc = Accumulator112::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.568_f32 + y.sin();
        let b = y * 8.302_f32 - x.cos();
        let mut acc = Accumulator112::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.809_f32 + y.sin();
        let b = y * 3.193_f32 - x.cos();
        let mut acc = Accumulator112::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.255_f32 + y.sin();
        let b = y * 4.112_f32 - x.cos();
        let mut acc = Accumulator112::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.997_f32 + y.sin();
        let b = y * 5.265_f32 - x.cos();
        let mut acc = Accumulator112::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.921_f32 + y.sin();
        let b = y * 4.063_f32 - x.cos();
        let mut acc = Accumulator112::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_112(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_112() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_112(total as u64) % 997) as f32;
        total
    }
}

pub mod m113 {
    use super::*;

    pub struct Accumulator113<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator113<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.981_f32 + y.sin();
        let b = y * 4.336_f32 - x.cos();
        let mut acc = Accumulator113::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.962_f32 + y.sin();
        let b = y * 1.235_f32 - x.cos();
        let mut acc = Accumulator113::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.702_f32 + y.sin();
        let b = y * 7.615_f32 - x.cos();
        let mut acc = Accumulator113::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.549_f32 + y.sin();
        let b = y * 8.709_f32 - x.cos();
        let mut acc = Accumulator113::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.943_f32 + y.sin();
        let b = y * 6.65_f32 - x.cos();
        let mut acc = Accumulator113::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.196_f32 + y.sin();
        let b = y * 8.872_f32 - x.cos();
        let mut acc = Accumulator113::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.543_f32 + y.sin();
        let b = y * 4.692_f32 - x.cos();
        let mut acc = Accumulator113::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.0_f32 + y.sin();
        let b = y * 7.56_f32 - x.cos();
        let mut acc = Accumulator113::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.615_f32 + y.sin();
        let b = y * 3.108_f32 - x.cos();
        let mut acc = Accumulator113::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.198_f32 + y.sin();
        let b = y * 5.654_f32 - x.cos();
        let mut acc = Accumulator113::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.93_f32 + y.sin();
        let b = y * 6.567_f32 - x.cos();
        let mut acc = Accumulator113::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.608_f32 + y.sin();
        let b = y * 5.512_f32 - x.cos();
        let mut acc = Accumulator113::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.787_f32 + y.sin();
        let b = y * 1.533_f32 - x.cos();
        let mut acc = Accumulator113::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.785_f32 + y.sin();
        let b = y * 6.316_f32 - x.cos();
        let mut acc = Accumulator113::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.542_f32 + y.sin();
        let b = y * 2.937_f32 - x.cos();
        let mut acc = Accumulator113::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.236_f32 + y.sin();
        let b = y * 3.24_f32 - x.cos();
        let mut acc = Accumulator113::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.685_f32 + y.sin();
        let b = y * 7.845_f32 - x.cos();
        let mut acc = Accumulator113::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.022_f32 + y.sin();
        let b = y * 1.714_f32 - x.cos();
        let mut acc = Accumulator113::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.433_f32 + y.sin();
        let b = y * 3.836_f32 - x.cos();
        let mut acc = Accumulator113::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.407_f32 + y.sin();
        let b = y * 7.136_f32 - x.cos();
        let mut acc = Accumulator113::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_113(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m113-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_113() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_113(total as u64) % 997) as f32;
        total
    }
}

pub mod m114 {
    use super::*;

    pub struct Accumulator114<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator114<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.643_f32 + y.sin();
        let b = y * 6.123_f32 - x.cos();
        let mut acc = Accumulator114::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.454_f32 + y.sin();
        let b = y * 7.736_f32 - x.cos();
        let mut acc = Accumulator114::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.523_f32 + y.sin();
        let b = y * 5.157_f32 - x.cos();
        let mut acc = Accumulator114::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.427_f32 + y.sin();
        let b = y * 1.605_f32 - x.cos();
        let mut acc = Accumulator114::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.815_f32 + y.sin();
        let b = y * 5.574_f32 - x.cos();
        let mut acc = Accumulator114::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.268_f32 + y.sin();
        let b = y * 0.282_f32 - x.cos();
        let mut acc = Accumulator114::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.902_f32 + y.sin();
        let b = y * 0.614_f32 - x.cos();
        let mut acc = Accumulator114::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.699_f32 + y.sin();
        let b = y * 8.34_f32 - x.cos();
        let mut acc = Accumulator114::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.287_f32 + y.sin();
        let b = y * 6.779_f32 - x.cos();
        let mut acc = Accumulator114::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.73_f32 + y.sin();
        let b = y * 3.329_f32 - x.cos();
        let mut acc = Accumulator114::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.821_f32 + y.sin();
        let b = y * 9.22_f32 - x.cos();
        let mut acc = Accumulator114::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.967_f32 + y.sin();
        let b = y * 5.788_f32 - x.cos();
        let mut acc = Accumulator114::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.462_f32 + y.sin();
        let b = y * 4.604_f32 - x.cos();
        let mut acc = Accumulator114::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.503_f32 + y.sin();
        let b = y * 8.14_f32 - x.cos();
        let mut acc = Accumulator114::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.224_f32 + y.sin();
        let b = y * 9.308_f32 - x.cos();
        let mut acc = Accumulator114::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.345_f32 + y.sin();
        let b = y * 0.11_f32 - x.cos();
        let mut acc = Accumulator114::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.056_f32 + y.sin();
        let b = y * 7.611_f32 - x.cos();
        let mut acc = Accumulator114::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.311_f32 + y.sin();
        let b = y * 8.259_f32 - x.cos();
        let mut acc = Accumulator114::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.683_f32 + y.sin();
        let b = y * 1.368_f32 - x.cos();
        let mut acc = Accumulator114::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.173_f32 + y.sin();
        let b = y * 6.798_f32 - x.cos();
        let mut acc = Accumulator114::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_114(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_114() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_114(total as u64) % 997) as f32;
        total
    }
}

pub mod m115 {
    use super::*;

    pub struct Accumulator115<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator115<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.524_f32 + y.sin();
        let b = y * 6.729_f32 - x.cos();
        let mut acc = Accumulator115::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.801_f32 + y.sin();
        let b = y * 2.566_f32 - x.cos();
        let mut acc = Accumulator115::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.898_f32 + y.sin();
        let b = y * 6.124_f32 - x.cos();
        let mut acc = Accumulator115::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.814_f32 + y.sin();
        let b = y * 2.474_f32 - x.cos();
        let mut acc = Accumulator115::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.299_f32 + y.sin();
        let b = y * 8.656_f32 - x.cos();
        let mut acc = Accumulator115::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.728_f32 + y.sin();
        let b = y * 7.61_f32 - x.cos();
        let mut acc = Accumulator115::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.266_f32 + y.sin();
        let b = y * 7.359_f32 - x.cos();
        let mut acc = Accumulator115::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.674_f32 + y.sin();
        let b = y * 8.09_f32 - x.cos();
        let mut acc = Accumulator115::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.439_f32 + y.sin();
        let b = y * 1.01_f32 - x.cos();
        let mut acc = Accumulator115::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.723_f32 + y.sin();
        let b = y * 1.686_f32 - x.cos();
        let mut acc = Accumulator115::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.241_f32 + y.sin();
        let b = y * 0.166_f32 - x.cos();
        let mut acc = Accumulator115::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.032_f32 + y.sin();
        let b = y * 1.332_f32 - x.cos();
        let mut acc = Accumulator115::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.603_f32 + y.sin();
        let b = y * 8.661_f32 - x.cos();
        let mut acc = Accumulator115::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.234_f32 + y.sin();
        let b = y * 1.244_f32 - x.cos();
        let mut acc = Accumulator115::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.438_f32 + y.sin();
        let b = y * 2.472_f32 - x.cos();
        let mut acc = Accumulator115::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.1_f32 + y.sin();
        let b = y * 7.787_f32 - x.cos();
        let mut acc = Accumulator115::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.649_f32 + y.sin();
        let b = y * 8.436_f32 - x.cos();
        let mut acc = Accumulator115::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.757_f32 + y.sin();
        let b = y * 4.202_f32 - x.cos();
        let mut acc = Accumulator115::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.344_f32 + y.sin();
        let b = y * 2.199_f32 - x.cos();
        let mut acc = Accumulator115::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.455_f32 + y.sin();
        let b = y * 1.939_f32 - x.cos();
        let mut acc = Accumulator115::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_115(seed: u64) -> u64 {
        let re = Regex::new(r"m115-(\d+)").unwrap();
        let hay = format!("m115-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_115() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_115(total as u64) % 997) as f32;
        total
    }
}

pub mod m116 {
    use super::*;

    pub struct Accumulator116<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator116<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.895_f32 + y.sin();
        let b = y * 3.076_f32 - x.cos();
        let mut acc = Accumulator116::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.132_f32 + y.sin();
        let b = y * 9.306_f32 - x.cos();
        let mut acc = Accumulator116::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.714_f32 + y.sin();
        let b = y * 4.451_f32 - x.cos();
        let mut acc = Accumulator116::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.613_f32 + y.sin();
        let b = y * 2.635_f32 - x.cos();
        let mut acc = Accumulator116::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.37_f32 + y.sin();
        let b = y * 6.969_f32 - x.cos();
        let mut acc = Accumulator116::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.634_f32 + y.sin();
        let b = y * 4.338_f32 - x.cos();
        let mut acc = Accumulator116::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.816_f32 + y.sin();
        let b = y * 3.847_f32 - x.cos();
        let mut acc = Accumulator116::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.786_f32 + y.sin();
        let b = y * 1.31_f32 - x.cos();
        let mut acc = Accumulator116::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.189_f32 + y.sin();
        let b = y * 0.926_f32 - x.cos();
        let mut acc = Accumulator116::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.569_f32 + y.sin();
        let b = y * 3.116_f32 - x.cos();
        let mut acc = Accumulator116::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.357_f32 + y.sin();
        let b = y * 3.28_f32 - x.cos();
        let mut acc = Accumulator116::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.92_f32 + y.sin();
        let b = y * 8.636_f32 - x.cos();
        let mut acc = Accumulator116::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.785_f32 + y.sin();
        let b = y * 9.551_f32 - x.cos();
        let mut acc = Accumulator116::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.798_f32 + y.sin();
        let b = y * 2.589_f32 - x.cos();
        let mut acc = Accumulator116::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.65_f32 + y.sin();
        let b = y * 4.151_f32 - x.cos();
        let mut acc = Accumulator116::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.791_f32 + y.sin();
        let b = y * 5.355_f32 - x.cos();
        let mut acc = Accumulator116::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.767_f32 + y.sin();
        let b = y * 8.221_f32 - x.cos();
        let mut acc = Accumulator116::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.285_f32 + y.sin();
        let b = y * 0.298_f32 - x.cos();
        let mut acc = Accumulator116::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.239_f32 + y.sin();
        let b = y * 6.94_f32 - x.cos();
        let mut acc = Accumulator116::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.263_f32 + y.sin();
        let b = y * 0.972_f32 - x.cos();
        let mut acc = Accumulator116::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_116(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_116() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_116(total as u64) % 997) as f32;
        total
    }
}

pub mod m117 {
    use super::*;

    pub struct Accumulator117<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator117<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.14_f32 + y.sin();
        let b = y * 2.087_f32 - x.cos();
        let mut acc = Accumulator117::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.75_f32 + y.sin();
        let b = y * 8.825_f32 - x.cos();
        let mut acc = Accumulator117::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.306_f32 + y.sin();
        let b = y * 3.683_f32 - x.cos();
        let mut acc = Accumulator117::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.313_f32 + y.sin();
        let b = y * 3.123_f32 - x.cos();
        let mut acc = Accumulator117::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.816_f32 + y.sin();
        let b = y * 4.483_f32 - x.cos();
        let mut acc = Accumulator117::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.579_f32 + y.sin();
        let b = y * 5.69_f32 - x.cos();
        let mut acc = Accumulator117::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.783_f32 + y.sin();
        let b = y * 9.304_f32 - x.cos();
        let mut acc = Accumulator117::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.071_f32 + y.sin();
        let b = y * 9.807_f32 - x.cos();
        let mut acc = Accumulator117::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.55_f32 + y.sin();
        let b = y * 3.191_f32 - x.cos();
        let mut acc = Accumulator117::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.267_f32 + y.sin();
        let b = y * 0.464_f32 - x.cos();
        let mut acc = Accumulator117::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.618_f32 + y.sin();
        let b = y * 2.129_f32 - x.cos();
        let mut acc = Accumulator117::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.069_f32 + y.sin();
        let b = y * 0.652_f32 - x.cos();
        let mut acc = Accumulator117::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.768_f32 + y.sin();
        let b = y * 8.814_f32 - x.cos();
        let mut acc = Accumulator117::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.984_f32 + y.sin();
        let b = y * 3.027_f32 - x.cos();
        let mut acc = Accumulator117::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.112_f32 + y.sin();
        let b = y * 0.852_f32 - x.cos();
        let mut acc = Accumulator117::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.749_f32 + y.sin();
        let b = y * 6.642_f32 - x.cos();
        let mut acc = Accumulator117::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.006_f32 + y.sin();
        let b = y * 1.598_f32 - x.cos();
        let mut acc = Accumulator117::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.136_f32 + y.sin();
        let b = y * 3.059_f32 - x.cos();
        let mut acc = Accumulator117::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.488_f32 + y.sin();
        let b = y * 3.282_f32 - x.cos();
        let mut acc = Accumulator117::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.299_f32 + y.sin();
        let b = y * 0.53_f32 - x.cos();
        let mut acc = Accumulator117::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_117(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(117u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_117() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_117(total as u64) % 997) as f32;
        total
    }
}

pub mod m118 {
    use super::*;

    pub struct Accumulator118<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator118<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.284_f32 + y.sin();
        let b = y * 8.54_f32 - x.cos();
        let mut acc = Accumulator118::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.23_f32 + y.sin();
        let b = y * 2.836_f32 - x.cos();
        let mut acc = Accumulator118::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.749_f32 + y.sin();
        let b = y * 2.717_f32 - x.cos();
        let mut acc = Accumulator118::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.878_f32 + y.sin();
        let b = y * 8.093_f32 - x.cos();
        let mut acc = Accumulator118::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.369_f32 + y.sin();
        let b = y * 8.929_f32 - x.cos();
        let mut acc = Accumulator118::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.939_f32 + y.sin();
        let b = y * 6.51_f32 - x.cos();
        let mut acc = Accumulator118::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.573_f32 + y.sin();
        let b = y * 0.738_f32 - x.cos();
        let mut acc = Accumulator118::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.643_f32 + y.sin();
        let b = y * 6.687_f32 - x.cos();
        let mut acc = Accumulator118::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.414_f32 + y.sin();
        let b = y * 8.029_f32 - x.cos();
        let mut acc = Accumulator118::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.378_f32 + y.sin();
        let b = y * 2.36_f32 - x.cos();
        let mut acc = Accumulator118::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.226_f32 + y.sin();
        let b = y * 4.457_f32 - x.cos();
        let mut acc = Accumulator118::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.769_f32 + y.sin();
        let b = y * 7.144_f32 - x.cos();
        let mut acc = Accumulator118::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.992_f32 + y.sin();
        let b = y * 5.713_f32 - x.cos();
        let mut acc = Accumulator118::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.027_f32 + y.sin();
        let b = y * 7.186_f32 - x.cos();
        let mut acc = Accumulator118::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.598_f32 + y.sin();
        let b = y * 4.781_f32 - x.cos();
        let mut acc = Accumulator118::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.091_f32 + y.sin();
        let b = y * 9.759_f32 - x.cos();
        let mut acc = Accumulator118::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.913_f32 + y.sin();
        let b = y * 8.581_f32 - x.cos();
        let mut acc = Accumulator118::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.697_f32 + y.sin();
        let b = y * 0.392_f32 - x.cos();
        let mut acc = Accumulator118::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.887_f32 + y.sin();
        let b = y * 6.278_f32 - x.cos();
        let mut acc = Accumulator118::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.187_f32 + y.sin();
        let b = y * 2.141_f32 - x.cos();
        let mut acc = Accumulator118::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_118(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_118() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_118(total as u64) % 997) as f32;
        total
    }
}

pub mod m119 {
    use super::*;

    pub struct Accumulator119<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator119<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.493_f32 + y.sin();
        let b = y * 7.505_f32 - x.cos();
        let mut acc = Accumulator119::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.73_f32 + y.sin();
        let b = y * 4.702_f32 - x.cos();
        let mut acc = Accumulator119::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.297_f32 + y.sin();
        let b = y * 7.719_f32 - x.cos();
        let mut acc = Accumulator119::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.638_f32 + y.sin();
        let b = y * 6.623_f32 - x.cos();
        let mut acc = Accumulator119::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.337_f32 + y.sin();
        let b = y * 4.444_f32 - x.cos();
        let mut acc = Accumulator119::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.217_f32 + y.sin();
        let b = y * 8.391_f32 - x.cos();
        let mut acc = Accumulator119::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.344_f32 + y.sin();
        let b = y * 5.959_f32 - x.cos();
        let mut acc = Accumulator119::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.997_f32 + y.sin();
        let b = y * 9.781_f32 - x.cos();
        let mut acc = Accumulator119::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.461_f32 + y.sin();
        let b = y * 1.04_f32 - x.cos();
        let mut acc = Accumulator119::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.071_f32 + y.sin();
        let b = y * 1.993_f32 - x.cos();
        let mut acc = Accumulator119::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.795_f32 + y.sin();
        let b = y * 7.27_f32 - x.cos();
        let mut acc = Accumulator119::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.76_f32 + y.sin();
        let b = y * 5.872_f32 - x.cos();
        let mut acc = Accumulator119::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.073_f32 + y.sin();
        let b = y * 7.549_f32 - x.cos();
        let mut acc = Accumulator119::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.746_f32 + y.sin();
        let b = y * 8.764_f32 - x.cos();
        let mut acc = Accumulator119::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.596_f32 + y.sin();
        let b = y * 6.751_f32 - x.cos();
        let mut acc = Accumulator119::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.467_f32 + y.sin();
        let b = y * 6.552_f32 - x.cos();
        let mut acc = Accumulator119::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.229_f32 + y.sin();
        let b = y * 3.335_f32 - x.cos();
        let mut acc = Accumulator119::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.739_f32 + y.sin();
        let b = y * 7.916_f32 - x.cos();
        let mut acc = Accumulator119::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.902_f32 + y.sin();
        let b = y * 0.334_f32 - x.cos();
        let mut acc = Accumulator119::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.128_f32 + y.sin();
        let b = y * 0.233_f32 - x.cos();
        let mut acc = Accumulator119::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_119(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_119() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_119(total as u64) % 997) as f32;
        total
    }
}

pub mod m120 {
    use super::*;

    pub struct Accumulator120<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator120<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.446_f32 + y.sin();
        let b = y * 3.68_f32 - x.cos();
        let mut acc = Accumulator120::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.699_f32 + y.sin();
        let b = y * 1.272_f32 - x.cos();
        let mut acc = Accumulator120::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.287_f32 + y.sin();
        let b = y * 0.95_f32 - x.cos();
        let mut acc = Accumulator120::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.988_f32 + y.sin();
        let b = y * 3.688_f32 - x.cos();
        let mut acc = Accumulator120::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.686_f32 + y.sin();
        let b = y * 7.196_f32 - x.cos();
        let mut acc = Accumulator120::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.223_f32 + y.sin();
        let b = y * 6.206_f32 - x.cos();
        let mut acc = Accumulator120::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.605_f32 + y.sin();
        let b = y * 4.76_f32 - x.cos();
        let mut acc = Accumulator120::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.039_f32 + y.sin();
        let b = y * 2.494_f32 - x.cos();
        let mut acc = Accumulator120::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.062_f32 + y.sin();
        let b = y * 1.067_f32 - x.cos();
        let mut acc = Accumulator120::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.793_f32 + y.sin();
        let b = y * 0.429_f32 - x.cos();
        let mut acc = Accumulator120::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.23_f32 + y.sin();
        let b = y * 1.381_f32 - x.cos();
        let mut acc = Accumulator120::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.275_f32 + y.sin();
        let b = y * 3.535_f32 - x.cos();
        let mut acc = Accumulator120::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.76_f32 + y.sin();
        let b = y * 0.667_f32 - x.cos();
        let mut acc = Accumulator120::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.239_f32 + y.sin();
        let b = y * 4.267_f32 - x.cos();
        let mut acc = Accumulator120::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.939_f32 + y.sin();
        let b = y * 7.068_f32 - x.cos();
        let mut acc = Accumulator120::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.938_f32 + y.sin();
        let b = y * 3.523_f32 - x.cos();
        let mut acc = Accumulator120::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.559_f32 + y.sin();
        let b = y * 5.277_f32 - x.cos();
        let mut acc = Accumulator120::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.446_f32 + y.sin();
        let b = y * 6.287_f32 - x.cos();
        let mut acc = Accumulator120::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.205_f32 + y.sin();
        let b = y * 8.538_f32 - x.cos();
        let mut acc = Accumulator120::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.073_f32 + y.sin();
        let b = y * 5.81_f32 - x.cos();
        let mut acc = Accumulator120::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_120(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m120-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_120() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_120(total as u64) % 997) as f32;
        total
    }
}

pub mod m121 {
    use super::*;

    pub struct Accumulator121<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator121<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.708_f32 + y.sin();
        let b = y * 9.235_f32 - x.cos();
        let mut acc = Accumulator121::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.796_f32 + y.sin();
        let b = y * 2.131_f32 - x.cos();
        let mut acc = Accumulator121::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.296_f32 + y.sin();
        let b = y * 4.916_f32 - x.cos();
        let mut acc = Accumulator121::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.063_f32 + y.sin();
        let b = y * 5.449_f32 - x.cos();
        let mut acc = Accumulator121::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.854_f32 + y.sin();
        let b = y * 4.612_f32 - x.cos();
        let mut acc = Accumulator121::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.297_f32 + y.sin();
        let b = y * 7.593_f32 - x.cos();
        let mut acc = Accumulator121::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.492_f32 + y.sin();
        let b = y * 1.103_f32 - x.cos();
        let mut acc = Accumulator121::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.188_f32 + y.sin();
        let b = y * 3.211_f32 - x.cos();
        let mut acc = Accumulator121::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.866_f32 + y.sin();
        let b = y * 1.122_f32 - x.cos();
        let mut acc = Accumulator121::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.525_f32 + y.sin();
        let b = y * 9.248_f32 - x.cos();
        let mut acc = Accumulator121::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.994_f32 + y.sin();
        let b = y * 4.636_f32 - x.cos();
        let mut acc = Accumulator121::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.061_f32 + y.sin();
        let b = y * 3.051_f32 - x.cos();
        let mut acc = Accumulator121::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.241_f32 + y.sin();
        let b = y * 1.938_f32 - x.cos();
        let mut acc = Accumulator121::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.876_f32 + y.sin();
        let b = y * 8.52_f32 - x.cos();
        let mut acc = Accumulator121::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.8_f32 + y.sin();
        let b = y * 1.944_f32 - x.cos();
        let mut acc = Accumulator121::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.378_f32 + y.sin();
        let b = y * 9.476_f32 - x.cos();
        let mut acc = Accumulator121::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.28_f32 + y.sin();
        let b = y * 8.39_f32 - x.cos();
        let mut acc = Accumulator121::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.038_f32 + y.sin();
        let b = y * 0.255_f32 - x.cos();
        let mut acc = Accumulator121::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.398_f32 + y.sin();
        let b = y * 5.931_f32 - x.cos();
        let mut acc = Accumulator121::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.797_f32 + y.sin();
        let b = y * 0.386_f32 - x.cos();
        let mut acc = Accumulator121::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_121(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_121() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_121(total as u64) % 997) as f32;
        total
    }
}

pub mod m122 {
    use super::*;

    pub struct Accumulator122<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator122<T> {
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
        let b = y * 0.503_f32 - x.cos();
        let mut acc = Accumulator122::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.621_f32 + y.sin();
        let b = y * 2.131_f32 - x.cos();
        let mut acc = Accumulator122::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.483_f32 + y.sin();
        let b = y * 0.373_f32 - x.cos();
        let mut acc = Accumulator122::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.615_f32 + y.sin();
        let b = y * 5.814_f32 - x.cos();
        let mut acc = Accumulator122::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.87_f32 + y.sin();
        let b = y * 3.646_f32 - x.cos();
        let mut acc = Accumulator122::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.361_f32 + y.sin();
        let b = y * 6.934_f32 - x.cos();
        let mut acc = Accumulator122::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.878_f32 + y.sin();
        let b = y * 1.123_f32 - x.cos();
        let mut acc = Accumulator122::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.344_f32 + y.sin();
        let b = y * 7.271_f32 - x.cos();
        let mut acc = Accumulator122::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.071_f32 + y.sin();
        let b = y * 1.433_f32 - x.cos();
        let mut acc = Accumulator122::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.815_f32 + y.sin();
        let b = y * 7.72_f32 - x.cos();
        let mut acc = Accumulator122::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.96_f32 + y.sin();
        let b = y * 1.388_f32 - x.cos();
        let mut acc = Accumulator122::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.824_f32 + y.sin();
        let b = y * 0.933_f32 - x.cos();
        let mut acc = Accumulator122::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.606_f32 + y.sin();
        let b = y * 6.445_f32 - x.cos();
        let mut acc = Accumulator122::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.06_f32 + y.sin();
        let b = y * 8.439_f32 - x.cos();
        let mut acc = Accumulator122::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.124_f32 + y.sin();
        let b = y * 9.791_f32 - x.cos();
        let mut acc = Accumulator122::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.447_f32 + y.sin();
        let b = y * 1.004_f32 - x.cos();
        let mut acc = Accumulator122::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.169_f32 + y.sin();
        let b = y * 1.315_f32 - x.cos();
        let mut acc = Accumulator122::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.877_f32 + y.sin();
        let b = y * 1.007_f32 - x.cos();
        let mut acc = Accumulator122::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.847_f32 + y.sin();
        let b = y * 8.712_f32 - x.cos();
        let mut acc = Accumulator122::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.682_f32 + y.sin();
        let b = y * 2.167_f32 - x.cos();
        let mut acc = Accumulator122::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_122(seed: u64) -> u64 {
        let re = Regex::new(r"m122-(\d+)").unwrap();
        let hay = format!("m122-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_122() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_122(total as u64) % 997) as f32;
        total
    }
}

pub mod m123 {
    use super::*;

    pub struct Accumulator123<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator123<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.056_f32 + y.sin();
        let b = y * 7.245_f32 - x.cos();
        let mut acc = Accumulator123::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.504_f32 + y.sin();
        let b = y * 6.777_f32 - x.cos();
        let mut acc = Accumulator123::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.7_f32 + y.sin();
        let b = y * 1.744_f32 - x.cos();
        let mut acc = Accumulator123::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.545_f32 + y.sin();
        let b = y * 2.148_f32 - x.cos();
        let mut acc = Accumulator123::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.684_f32 + y.sin();
        let b = y * 1.259_f32 - x.cos();
        let mut acc = Accumulator123::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.421_f32 + y.sin();
        let b = y * 1.25_f32 - x.cos();
        let mut acc = Accumulator123::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.405_f32 + y.sin();
        let b = y * 6.451_f32 - x.cos();
        let mut acc = Accumulator123::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.771_f32 + y.sin();
        let b = y * 1.554_f32 - x.cos();
        let mut acc = Accumulator123::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.453_f32 + y.sin();
        let b = y * 6.529_f32 - x.cos();
        let mut acc = Accumulator123::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.171_f32 + y.sin();
        let b = y * 5.76_f32 - x.cos();
        let mut acc = Accumulator123::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.711_f32 + y.sin();
        let b = y * 1.364_f32 - x.cos();
        let mut acc = Accumulator123::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.257_f32 + y.sin();
        let b = y * 0.756_f32 - x.cos();
        let mut acc = Accumulator123::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.598_f32 + y.sin();
        let b = y * 3.645_f32 - x.cos();
        let mut acc = Accumulator123::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.545_f32 + y.sin();
        let b = y * 2.66_f32 - x.cos();
        let mut acc = Accumulator123::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.624_f32 + y.sin();
        let b = y * 0.985_f32 - x.cos();
        let mut acc = Accumulator123::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.269_f32 + y.sin();
        let b = y * 1.218_f32 - x.cos();
        let mut acc = Accumulator123::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.499_f32 + y.sin();
        let b = y * 5.9_f32 - x.cos();
        let mut acc = Accumulator123::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.462_f32 + y.sin();
        let b = y * 5.639_f32 - x.cos();
        let mut acc = Accumulator123::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.663_f32 + y.sin();
        let b = y * 9.302_f32 - x.cos();
        let mut acc = Accumulator123::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.628_f32 + y.sin();
        let b = y * 6.701_f32 - x.cos();
        let mut acc = Accumulator123::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_123(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_123() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_123(total as u64) % 997) as f32;
        total
    }
}

pub mod m124 {
    use super::*;

    pub struct Accumulator124<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator124<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.004_f32 + y.sin();
        let b = y * 2.405_f32 - x.cos();
        let mut acc = Accumulator124::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.462_f32 + y.sin();
        let b = y * 2.597_f32 - x.cos();
        let mut acc = Accumulator124::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.674_f32 + y.sin();
        let b = y * 8.951_f32 - x.cos();
        let mut acc = Accumulator124::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.897_f32 + y.sin();
        let b = y * 2.389_f32 - x.cos();
        let mut acc = Accumulator124::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.221_f32 + y.sin();
        let b = y * 5.919_f32 - x.cos();
        let mut acc = Accumulator124::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.109_f32 + y.sin();
        let b = y * 4.6_f32 - x.cos();
        let mut acc = Accumulator124::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.588_f32 + y.sin();
        let b = y * 4.261_f32 - x.cos();
        let mut acc = Accumulator124::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.772_f32 + y.sin();
        let b = y * 8.028_f32 - x.cos();
        let mut acc = Accumulator124::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.784_f32 + y.sin();
        let b = y * 7.824_f32 - x.cos();
        let mut acc = Accumulator124::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.378_f32 + y.sin();
        let b = y * 6.92_f32 - x.cos();
        let mut acc = Accumulator124::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.225_f32 + y.sin();
        let b = y * 2.546_f32 - x.cos();
        let mut acc = Accumulator124::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.257_f32 + y.sin();
        let b = y * 1.293_f32 - x.cos();
        let mut acc = Accumulator124::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.407_f32 + y.sin();
        let b = y * 2.23_f32 - x.cos();
        let mut acc = Accumulator124::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.985_f32 + y.sin();
        let b = y * 6.097_f32 - x.cos();
        let mut acc = Accumulator124::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.235_f32 + y.sin();
        let b = y * 5.175_f32 - x.cos();
        let mut acc = Accumulator124::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.428_f32 + y.sin();
        let b = y * 3.309_f32 - x.cos();
        let mut acc = Accumulator124::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.285_f32 + y.sin();
        let b = y * 2.554_f32 - x.cos();
        let mut acc = Accumulator124::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.817_f32 + y.sin();
        let b = y * 5.002_f32 - x.cos();
        let mut acc = Accumulator124::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.49_f32 + y.sin();
        let b = y * 3.256_f32 - x.cos();
        let mut acc = Accumulator124::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.777_f32 + y.sin();
        let b = y * 3.1_f32 - x.cos();
        let mut acc = Accumulator124::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_124(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(124u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_124() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_124(total as u64) % 997) as f32;
        total
    }
}

pub mod m125 {
    use super::*;

    pub struct Accumulator125<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator125<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.559_f32 + y.sin();
        let b = y * 9.707_f32 - x.cos();
        let mut acc = Accumulator125::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.596_f32 + y.sin();
        let b = y * 5.842_f32 - x.cos();
        let mut acc = Accumulator125::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.477_f32 + y.sin();
        let b = y * 7.213_f32 - x.cos();
        let mut acc = Accumulator125::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.668_f32 + y.sin();
        let b = y * 4.399_f32 - x.cos();
        let mut acc = Accumulator125::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.721_f32 + y.sin();
        let b = y * 1.985_f32 - x.cos();
        let mut acc = Accumulator125::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.689_f32 + y.sin();
        let b = y * 7.207_f32 - x.cos();
        let mut acc = Accumulator125::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.972_f32 + y.sin();
        let b = y * 7.545_f32 - x.cos();
        let mut acc = Accumulator125::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.458_f32 + y.sin();
        let b = y * 9.711_f32 - x.cos();
        let mut acc = Accumulator125::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.687_f32 + y.sin();
        let b = y * 8.951_f32 - x.cos();
        let mut acc = Accumulator125::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.198_f32 + y.sin();
        let b = y * 6.417_f32 - x.cos();
        let mut acc = Accumulator125::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.683_f32 + y.sin();
        let b = y * 5.248_f32 - x.cos();
        let mut acc = Accumulator125::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.127_f32 + y.sin();
        let b = y * 9.714_f32 - x.cos();
        let mut acc = Accumulator125::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.36_f32 + y.sin();
        let b = y * 9.052_f32 - x.cos();
        let mut acc = Accumulator125::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.125_f32 + y.sin();
        let b = y * 5.126_f32 - x.cos();
        let mut acc = Accumulator125::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.127_f32 + y.sin();
        let b = y * 1.835_f32 - x.cos();
        let mut acc = Accumulator125::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.751_f32 + y.sin();
        let b = y * 7.645_f32 - x.cos();
        let mut acc = Accumulator125::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.123_f32 + y.sin();
        let b = y * 7.066_f32 - x.cos();
        let mut acc = Accumulator125::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.927_f32 + y.sin();
        let b = y * 9.527_f32 - x.cos();
        let mut acc = Accumulator125::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.174_f32 + y.sin();
        let b = y * 8.164_f32 - x.cos();
        let mut acc = Accumulator125::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.994_f32 + y.sin();
        let b = y * 5.963_f32 - x.cos();
        let mut acc = Accumulator125::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_125(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_125() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_125(total as u64) % 997) as f32;
        total
    }
}

pub mod m126 {
    use super::*;

    pub struct Accumulator126<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator126<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.183_f32 + y.sin();
        let b = y * 7.815_f32 - x.cos();
        let mut acc = Accumulator126::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.849_f32 + y.sin();
        let b = y * 7.828_f32 - x.cos();
        let mut acc = Accumulator126::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.402_f32 + y.sin();
        let b = y * 5.645_f32 - x.cos();
        let mut acc = Accumulator126::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.212_f32 + y.sin();
        let b = y * 0.536_f32 - x.cos();
        let mut acc = Accumulator126::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.65_f32 + y.sin();
        let b = y * 8.584_f32 - x.cos();
        let mut acc = Accumulator126::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.156_f32 + y.sin();
        let b = y * 5.84_f32 - x.cos();
        let mut acc = Accumulator126::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.988_f32 + y.sin();
        let b = y * 4.89_f32 - x.cos();
        let mut acc = Accumulator126::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.483_f32 + y.sin();
        let b = y * 3.444_f32 - x.cos();
        let mut acc = Accumulator126::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.416_f32 + y.sin();
        let b = y * 3.092_f32 - x.cos();
        let mut acc = Accumulator126::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.997_f32 + y.sin();
        let b = y * 7.742_f32 - x.cos();
        let mut acc = Accumulator126::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.011_f32 + y.sin();
        let b = y * 1.088_f32 - x.cos();
        let mut acc = Accumulator126::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.429_f32 + y.sin();
        let b = y * 8.452_f32 - x.cos();
        let mut acc = Accumulator126::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.401_f32 + y.sin();
        let b = y * 6.782_f32 - x.cos();
        let mut acc = Accumulator126::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.786_f32 + y.sin();
        let b = y * 6.079_f32 - x.cos();
        let mut acc = Accumulator126::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.751_f32 + y.sin();
        let b = y * 0.525_f32 - x.cos();
        let mut acc = Accumulator126::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.842_f32 + y.sin();
        let b = y * 5.729_f32 - x.cos();
        let mut acc = Accumulator126::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.373_f32 + y.sin();
        let b = y * 9.221_f32 - x.cos();
        let mut acc = Accumulator126::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.229_f32 + y.sin();
        let b = y * 8.583_f32 - x.cos();
        let mut acc = Accumulator126::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.158_f32 + y.sin();
        let b = y * 2.993_f32 - x.cos();
        let mut acc = Accumulator126::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.667_f32 + y.sin();
        let b = y * 1.186_f32 - x.cos();
        let mut acc = Accumulator126::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_126(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_126() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_126(total as u64) % 997) as f32;
        total
    }
}

pub mod m127 {
    use super::*;

    pub struct Accumulator127<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator127<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.401_f32 + y.sin();
        let b = y * 3.704_f32 - x.cos();
        let mut acc = Accumulator127::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.768_f32 + y.sin();
        let b = y * 5.525_f32 - x.cos();
        let mut acc = Accumulator127::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.21_f32 + y.sin();
        let b = y * 5.786_f32 - x.cos();
        let mut acc = Accumulator127::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.511_f32 + y.sin();
        let b = y * 8.242_f32 - x.cos();
        let mut acc = Accumulator127::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.472_f32 + y.sin();
        let b = y * 8.299_f32 - x.cos();
        let mut acc = Accumulator127::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.002_f32 + y.sin();
        let b = y * 7.763_f32 - x.cos();
        let mut acc = Accumulator127::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.083_f32 + y.sin();
        let b = y * 8.132_f32 - x.cos();
        let mut acc = Accumulator127::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.562_f32 + y.sin();
        let b = y * 5.064_f32 - x.cos();
        let mut acc = Accumulator127::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.882_f32 + y.sin();
        let b = y * 4.104_f32 - x.cos();
        let mut acc = Accumulator127::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.417_f32 + y.sin();
        let b = y * 2.993_f32 - x.cos();
        let mut acc = Accumulator127::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.411_f32 + y.sin();
        let b = y * 1.714_f32 - x.cos();
        let mut acc = Accumulator127::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.868_f32 + y.sin();
        let b = y * 3.07_f32 - x.cos();
        let mut acc = Accumulator127::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.504_f32 + y.sin();
        let b = y * 5.511_f32 - x.cos();
        let mut acc = Accumulator127::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.724_f32 + y.sin();
        let b = y * 7.829_f32 - x.cos();
        let mut acc = Accumulator127::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.332_f32 + y.sin();
        let b = y * 9.247_f32 - x.cos();
        let mut acc = Accumulator127::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.124_f32 + y.sin();
        let b = y * 7.6_f32 - x.cos();
        let mut acc = Accumulator127::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.53_f32 + y.sin();
        let b = y * 6.251_f32 - x.cos();
        let mut acc = Accumulator127::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.661_f32 + y.sin();
        let b = y * 6.073_f32 - x.cos();
        let mut acc = Accumulator127::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.676_f32 + y.sin();
        let b = y * 3.256_f32 - x.cos();
        let mut acc = Accumulator127::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.285_f32 + y.sin();
        let b = y * 1.059_f32 - x.cos();
        let mut acc = Accumulator127::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_127(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m127-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_127() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_127(total as u64) % 997) as f32;
        total
    }
}

pub mod m128 {
    use super::*;

    pub struct Accumulator128<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator128<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.017_f32 + y.sin();
        let b = y * 7.897_f32 - x.cos();
        let mut acc = Accumulator128::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.21_f32 + y.sin();
        let b = y * 1.525_f32 - x.cos();
        let mut acc = Accumulator128::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.893_f32 + y.sin();
        let b = y * 3.741_f32 - x.cos();
        let mut acc = Accumulator128::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.484_f32 + y.sin();
        let b = y * 9.155_f32 - x.cos();
        let mut acc = Accumulator128::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.614_f32 + y.sin();
        let b = y * 8.058_f32 - x.cos();
        let mut acc = Accumulator128::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.08_f32 + y.sin();
        let b = y * 7.048_f32 - x.cos();
        let mut acc = Accumulator128::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.407_f32 + y.sin();
        let b = y * 8.751_f32 - x.cos();
        let mut acc = Accumulator128::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.07_f32 + y.sin();
        let b = y * 5.3_f32 - x.cos();
        let mut acc = Accumulator128::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.542_f32 + y.sin();
        let b = y * 7.549_f32 - x.cos();
        let mut acc = Accumulator128::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.889_f32 + y.sin();
        let b = y * 4.504_f32 - x.cos();
        let mut acc = Accumulator128::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.773_f32 + y.sin();
        let b = y * 7.949_f32 - x.cos();
        let mut acc = Accumulator128::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.258_f32 + y.sin();
        let b = y * 6.899_f32 - x.cos();
        let mut acc = Accumulator128::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.549_f32 + y.sin();
        let b = y * 0.484_f32 - x.cos();
        let mut acc = Accumulator128::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.873_f32 + y.sin();
        let b = y * 9.086_f32 - x.cos();
        let mut acc = Accumulator128::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.121_f32 + y.sin();
        let b = y * 5.562_f32 - x.cos();
        let mut acc = Accumulator128::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.407_f32 + y.sin();
        let b = y * 2.542_f32 - x.cos();
        let mut acc = Accumulator128::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.46_f32 + y.sin();
        let b = y * 6.691_f32 - x.cos();
        let mut acc = Accumulator128::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.207_f32 + y.sin();
        let b = y * 8.23_f32 - x.cos();
        let mut acc = Accumulator128::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.221_f32 + y.sin();
        let b = y * 0.607_f32 - x.cos();
        let mut acc = Accumulator128::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.188_f32 + y.sin();
        let b = y * 7.49_f32 - x.cos();
        let mut acc = Accumulator128::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_128(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_128() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_128(total as u64) % 997) as f32;
        total
    }
}

pub mod m129 {
    use super::*;

    pub struct Accumulator129<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator129<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.799_f32 + y.sin();
        let b = y * 8.725_f32 - x.cos();
        let mut acc = Accumulator129::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.631_f32 + y.sin();
        let b = y * 5.269_f32 - x.cos();
        let mut acc = Accumulator129::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.746_f32 + y.sin();
        let b = y * 9.591_f32 - x.cos();
        let mut acc = Accumulator129::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.146_f32 + y.sin();
        let b = y * 8.299_f32 - x.cos();
        let mut acc = Accumulator129::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.768_f32 + y.sin();
        let b = y * 3.081_f32 - x.cos();
        let mut acc = Accumulator129::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.779_f32 + y.sin();
        let b = y * 9.157_f32 - x.cos();
        let mut acc = Accumulator129::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.666_f32 + y.sin();
        let b = y * 8.739_f32 - x.cos();
        let mut acc = Accumulator129::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.21_f32 + y.sin();
        let b = y * 1.366_f32 - x.cos();
        let mut acc = Accumulator129::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.988_f32 + y.sin();
        let b = y * 5.286_f32 - x.cos();
        let mut acc = Accumulator129::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.171_f32 + y.sin();
        let b = y * 8.893_f32 - x.cos();
        let mut acc = Accumulator129::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.013_f32 + y.sin();
        let b = y * 5.509_f32 - x.cos();
        let mut acc = Accumulator129::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.558_f32 + y.sin();
        let b = y * 3.409_f32 - x.cos();
        let mut acc = Accumulator129::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.216_f32 + y.sin();
        let b = y * 3.999_f32 - x.cos();
        let mut acc = Accumulator129::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.843_f32 + y.sin();
        let b = y * 6.457_f32 - x.cos();
        let mut acc = Accumulator129::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.073_f32 + y.sin();
        let b = y * 4.696_f32 - x.cos();
        let mut acc = Accumulator129::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.686_f32 + y.sin();
        let b = y * 3.255_f32 - x.cos();
        let mut acc = Accumulator129::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.518_f32 + y.sin();
        let b = y * 6.446_f32 - x.cos();
        let mut acc = Accumulator129::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.007_f32 + y.sin();
        let b = y * 8.315_f32 - x.cos();
        let mut acc = Accumulator129::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.231_f32 + y.sin();
        let b = y * 5.785_f32 - x.cos();
        let mut acc = Accumulator129::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.007_f32 + y.sin();
        let b = y * 3.499_f32 - x.cos();
        let mut acc = Accumulator129::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_129(seed: u64) -> u64 {
        let re = Regex::new(r"m129-(\d+)").unwrap();
        let hay = format!("m129-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_129() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_129(total as u64) % 997) as f32;
        total
    }
}

pub mod m130 {
    use super::*;

    pub struct Accumulator130<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator130<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.23_f32 + y.sin();
        let b = y * 4.323_f32 - x.cos();
        let mut acc = Accumulator130::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.312_f32 + y.sin();
        let b = y * 3.487_f32 - x.cos();
        let mut acc = Accumulator130::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.414_f32 + y.sin();
        let b = y * 9.111_f32 - x.cos();
        let mut acc = Accumulator130::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.278_f32 + y.sin();
        let b = y * 9.496_f32 - x.cos();
        let mut acc = Accumulator130::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.455_f32 + y.sin();
        let b = y * 3.445_f32 - x.cos();
        let mut acc = Accumulator130::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.782_f32 + y.sin();
        let b = y * 0.654_f32 - x.cos();
        let mut acc = Accumulator130::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.202_f32 + y.sin();
        let b = y * 9.111_f32 - x.cos();
        let mut acc = Accumulator130::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.918_f32 + y.sin();
        let b = y * 7.23_f32 - x.cos();
        let mut acc = Accumulator130::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.725_f32 + y.sin();
        let b = y * 6.285_f32 - x.cos();
        let mut acc = Accumulator130::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.717_f32 + y.sin();
        let b = y * 4.646_f32 - x.cos();
        let mut acc = Accumulator130::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.032_f32 + y.sin();
        let b = y * 1.056_f32 - x.cos();
        let mut acc = Accumulator130::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.262_f32 + y.sin();
        let b = y * 6.336_f32 - x.cos();
        let mut acc = Accumulator130::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.714_f32 + y.sin();
        let b = y * 8.465_f32 - x.cos();
        let mut acc = Accumulator130::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.605_f32 + y.sin();
        let b = y * 2.364_f32 - x.cos();
        let mut acc = Accumulator130::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.329_f32 + y.sin();
        let b = y * 9.749_f32 - x.cos();
        let mut acc = Accumulator130::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.322_f32 + y.sin();
        let b = y * 5.803_f32 - x.cos();
        let mut acc = Accumulator130::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.896_f32 + y.sin();
        let b = y * 7.465_f32 - x.cos();
        let mut acc = Accumulator130::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.337_f32 + y.sin();
        let b = y * 7.132_f32 - x.cos();
        let mut acc = Accumulator130::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.493_f32 + y.sin();
        let b = y * 7.849_f32 - x.cos();
        let mut acc = Accumulator130::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.877_f32 + y.sin();
        let b = y * 8.191_f32 - x.cos();
        let mut acc = Accumulator130::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_130(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_130() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_130(total as u64) % 997) as f32;
        total
    }
}

pub mod m131 {
    use super::*;

    pub struct Accumulator131<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator131<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.625_f32 + y.sin();
        let b = y * 9.734_f32 - x.cos();
        let mut acc = Accumulator131::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.742_f32 + y.sin();
        let b = y * 9.279_f32 - x.cos();
        let mut acc = Accumulator131::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.121_f32 + y.sin();
        let b = y * 2.214_f32 - x.cos();
        let mut acc = Accumulator131::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.342_f32 + y.sin();
        let b = y * 9.478_f32 - x.cos();
        let mut acc = Accumulator131::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.342_f32 + y.sin();
        let b = y * 0.835_f32 - x.cos();
        let mut acc = Accumulator131::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.685_f32 + y.sin();
        let b = y * 5.594_f32 - x.cos();
        let mut acc = Accumulator131::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.814_f32 + y.sin();
        let b = y * 5.547_f32 - x.cos();
        let mut acc = Accumulator131::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.38_f32 + y.sin();
        let b = y * 8.695_f32 - x.cos();
        let mut acc = Accumulator131::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.022_f32 + y.sin();
        let b = y * 7.523_f32 - x.cos();
        let mut acc = Accumulator131::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.181_f32 + y.sin();
        let b = y * 9.353_f32 - x.cos();
        let mut acc = Accumulator131::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.26_f32 + y.sin();
        let b = y * 0.584_f32 - x.cos();
        let mut acc = Accumulator131::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.419_f32 + y.sin();
        let b = y * 7.042_f32 - x.cos();
        let mut acc = Accumulator131::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.594_f32 + y.sin();
        let b = y * 5.924_f32 - x.cos();
        let mut acc = Accumulator131::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.801_f32 + y.sin();
        let b = y * 6.926_f32 - x.cos();
        let mut acc = Accumulator131::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.061_f32 + y.sin();
        let b = y * 9.489_f32 - x.cos();
        let mut acc = Accumulator131::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.878_f32 + y.sin();
        let b = y * 7.937_f32 - x.cos();
        let mut acc = Accumulator131::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.504_f32 + y.sin();
        let b = y * 2.414_f32 - x.cos();
        let mut acc = Accumulator131::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.179_f32 + y.sin();
        let b = y * 5.319_f32 - x.cos();
        let mut acc = Accumulator131::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.268_f32 + y.sin();
        let b = y * 3.433_f32 - x.cos();
        let mut acc = Accumulator131::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.748_f32 + y.sin();
        let b = y * 7.747_f32 - x.cos();
        let mut acc = Accumulator131::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_131(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(131u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_131() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_131(total as u64) % 997) as f32;
        total
    }
}

pub mod m132 {
    use super::*;

    pub struct Accumulator132<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator132<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.107_f32 + y.sin();
        let b = y * 5.577_f32 - x.cos();
        let mut acc = Accumulator132::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.932_f32 + y.sin();
        let b = y * 9.699_f32 - x.cos();
        let mut acc = Accumulator132::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.363_f32 + y.sin();
        let b = y * 5.734_f32 - x.cos();
        let mut acc = Accumulator132::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.92_f32 + y.sin();
        let b = y * 8.204_f32 - x.cos();
        let mut acc = Accumulator132::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.914_f32 + y.sin();
        let b = y * 4.133_f32 - x.cos();
        let mut acc = Accumulator132::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.0_f32 + y.sin();
        let b = y * 1.987_f32 - x.cos();
        let mut acc = Accumulator132::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.005_f32 + y.sin();
        let b = y * 7.089_f32 - x.cos();
        let mut acc = Accumulator132::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.354_f32 + y.sin();
        let b = y * 4.44_f32 - x.cos();
        let mut acc = Accumulator132::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.544_f32 + y.sin();
        let b = y * 8.74_f32 - x.cos();
        let mut acc = Accumulator132::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.103_f32 + y.sin();
        let b = y * 9.748_f32 - x.cos();
        let mut acc = Accumulator132::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.969_f32 + y.sin();
        let b = y * 3.672_f32 - x.cos();
        let mut acc = Accumulator132::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.691_f32 + y.sin();
        let b = y * 2.682_f32 - x.cos();
        let mut acc = Accumulator132::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.862_f32 + y.sin();
        let b = y * 9.71_f32 - x.cos();
        let mut acc = Accumulator132::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.234_f32 + y.sin();
        let b = y * 6.366_f32 - x.cos();
        let mut acc = Accumulator132::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.155_f32 + y.sin();
        let b = y * 2.285_f32 - x.cos();
        let mut acc = Accumulator132::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.83_f32 + y.sin();
        let b = y * 1.208_f32 - x.cos();
        let mut acc = Accumulator132::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.195_f32 + y.sin();
        let b = y * 4.757_f32 - x.cos();
        let mut acc = Accumulator132::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.893_f32 + y.sin();
        let b = y * 4.716_f32 - x.cos();
        let mut acc = Accumulator132::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.275_f32 + y.sin();
        let b = y * 7.904_f32 - x.cos();
        let mut acc = Accumulator132::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.346_f32 + y.sin();
        let b = y * 6.759_f32 - x.cos();
        let mut acc = Accumulator132::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_132(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_132() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_132(total as u64) % 997) as f32;
        total
    }
}

pub mod m133 {
    use super::*;

    pub struct Accumulator133<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator133<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.539_f32 + y.sin();
        let b = y * 9.098_f32 - x.cos();
        let mut acc = Accumulator133::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.818_f32 + y.sin();
        let b = y * 2.456_f32 - x.cos();
        let mut acc = Accumulator133::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.458_f32 + y.sin();
        let b = y * 0.923_f32 - x.cos();
        let mut acc = Accumulator133::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.531_f32 + y.sin();
        let b = y * 4.732_f32 - x.cos();
        let mut acc = Accumulator133::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.799_f32 + y.sin();
        let b = y * 1.441_f32 - x.cos();
        let mut acc = Accumulator133::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.962_f32 + y.sin();
        let b = y * 9.211_f32 - x.cos();
        let mut acc = Accumulator133::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.488_f32 + y.sin();
        let b = y * 8.885_f32 - x.cos();
        let mut acc = Accumulator133::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.666_f32 + y.sin();
        let b = y * 3.75_f32 - x.cos();
        let mut acc = Accumulator133::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.279_f32 + y.sin();
        let b = y * 7.337_f32 - x.cos();
        let mut acc = Accumulator133::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.468_f32 + y.sin();
        let b = y * 8.111_f32 - x.cos();
        let mut acc = Accumulator133::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.035_f32 + y.sin();
        let b = y * 4.947_f32 - x.cos();
        let mut acc = Accumulator133::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.491_f32 + y.sin();
        let b = y * 8.593_f32 - x.cos();
        let mut acc = Accumulator133::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.332_f32 + y.sin();
        let b = y * 8.275_f32 - x.cos();
        let mut acc = Accumulator133::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.739_f32 + y.sin();
        let b = y * 3.101_f32 - x.cos();
        let mut acc = Accumulator133::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.866_f32 + y.sin();
        let b = y * 4.631_f32 - x.cos();
        let mut acc = Accumulator133::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.693_f32 + y.sin();
        let b = y * 4.051_f32 - x.cos();
        let mut acc = Accumulator133::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.459_f32 + y.sin();
        let b = y * 4.33_f32 - x.cos();
        let mut acc = Accumulator133::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.745_f32 + y.sin();
        let b = y * 9.308_f32 - x.cos();
        let mut acc = Accumulator133::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.082_f32 + y.sin();
        let b = y * 8.49_f32 - x.cos();
        let mut acc = Accumulator133::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.288_f32 + y.sin();
        let b = y * 2.153_f32 - x.cos();
        let mut acc = Accumulator133::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_133(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_133() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_133(total as u64) % 997) as f32;
        total
    }
}

pub mod m134 {
    use super::*;

    pub struct Accumulator134<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator134<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.176_f32 + y.sin();
        let b = y * 5.41_f32 - x.cos();
        let mut acc = Accumulator134::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.078_f32 + y.sin();
        let b = y * 8.946_f32 - x.cos();
        let mut acc = Accumulator134::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.467_f32 + y.sin();
        let b = y * 7.394_f32 - x.cos();
        let mut acc = Accumulator134::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.668_f32 + y.sin();
        let b = y * 6.627_f32 - x.cos();
        let mut acc = Accumulator134::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.074_f32 + y.sin();
        let b = y * 8.026_f32 - x.cos();
        let mut acc = Accumulator134::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.34_f32 + y.sin();
        let b = y * 9.523_f32 - x.cos();
        let mut acc = Accumulator134::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.289_f32 + y.sin();
        let b = y * 7.412_f32 - x.cos();
        let mut acc = Accumulator134::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.807_f32 + y.sin();
        let b = y * 9.107_f32 - x.cos();
        let mut acc = Accumulator134::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.31_f32 + y.sin();
        let b = y * 6.962_f32 - x.cos();
        let mut acc = Accumulator134::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.861_f32 + y.sin();
        let b = y * 1.645_f32 - x.cos();
        let mut acc = Accumulator134::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.099_f32 + y.sin();
        let b = y * 4.651_f32 - x.cos();
        let mut acc = Accumulator134::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.718_f32 + y.sin();
        let b = y * 9.124_f32 - x.cos();
        let mut acc = Accumulator134::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.689_f32 + y.sin();
        let b = y * 2.917_f32 - x.cos();
        let mut acc = Accumulator134::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.277_f32 + y.sin();
        let b = y * 7.655_f32 - x.cos();
        let mut acc = Accumulator134::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.949_f32 + y.sin();
        let b = y * 8.085_f32 - x.cos();
        let mut acc = Accumulator134::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.238_f32 + y.sin();
        let b = y * 8.497_f32 - x.cos();
        let mut acc = Accumulator134::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.8_f32 + y.sin();
        let b = y * 8.305_f32 - x.cos();
        let mut acc = Accumulator134::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.859_f32 + y.sin();
        let b = y * 1.284_f32 - x.cos();
        let mut acc = Accumulator134::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.917_f32 + y.sin();
        let b = y * 8.7_f32 - x.cos();
        let mut acc = Accumulator134::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.226_f32 + y.sin();
        let b = y * 5.158_f32 - x.cos();
        let mut acc = Accumulator134::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_134(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m134-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_134() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_134(total as u64) % 997) as f32;
        total
    }
}

pub mod m135 {
    use super::*;

    pub struct Accumulator135<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator135<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.978_f32 + y.sin();
        let b = y * 2.272_f32 - x.cos();
        let mut acc = Accumulator135::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.284_f32 + y.sin();
        let b = y * 0.119_f32 - x.cos();
        let mut acc = Accumulator135::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.167_f32 + y.sin();
        let b = y * 7.657_f32 - x.cos();
        let mut acc = Accumulator135::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.794_f32 + y.sin();
        let b = y * 9.772_f32 - x.cos();
        let mut acc = Accumulator135::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.185_f32 + y.sin();
        let b = y * 9.541_f32 - x.cos();
        let mut acc = Accumulator135::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.827_f32 + y.sin();
        let b = y * 2.382_f32 - x.cos();
        let mut acc = Accumulator135::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.281_f32 + y.sin();
        let b = y * 3.447_f32 - x.cos();
        let mut acc = Accumulator135::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.489_f32 + y.sin();
        let b = y * 6.62_f32 - x.cos();
        let mut acc = Accumulator135::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.875_f32 + y.sin();
        let b = y * 4.962_f32 - x.cos();
        let mut acc = Accumulator135::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.08_f32 + y.sin();
        let b = y * 0.169_f32 - x.cos();
        let mut acc = Accumulator135::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.398_f32 + y.sin();
        let b = y * 4.152_f32 - x.cos();
        let mut acc = Accumulator135::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.514_f32 + y.sin();
        let b = y * 9.276_f32 - x.cos();
        let mut acc = Accumulator135::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.131_f32 + y.sin();
        let b = y * 8.413_f32 - x.cos();
        let mut acc = Accumulator135::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.9_f32 + y.sin();
        let b = y * 2.35_f32 - x.cos();
        let mut acc = Accumulator135::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.872_f32 + y.sin();
        let b = y * 0.994_f32 - x.cos();
        let mut acc = Accumulator135::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.151_f32 + y.sin();
        let b = y * 1.309_f32 - x.cos();
        let mut acc = Accumulator135::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.566_f32 + y.sin();
        let b = y * 0.958_f32 - x.cos();
        let mut acc = Accumulator135::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.135_f32 + y.sin();
        let b = y * 2.118_f32 - x.cos();
        let mut acc = Accumulator135::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.327_f32 + y.sin();
        let b = y * 8.779_f32 - x.cos();
        let mut acc = Accumulator135::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.329_f32 + y.sin();
        let b = y * 5.633_f32 - x.cos();
        let mut acc = Accumulator135::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_135(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_135() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_135(total as u64) % 997) as f32;
        total
    }
}

pub mod m136 {
    use super::*;

    pub struct Accumulator136<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator136<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.443_f32 + y.sin();
        let b = y * 3.485_f32 - x.cos();
        let mut acc = Accumulator136::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.575_f32 + y.sin();
        let b = y * 6.297_f32 - x.cos();
        let mut acc = Accumulator136::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.497_f32 + y.sin();
        let b = y * 9.532_f32 - x.cos();
        let mut acc = Accumulator136::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.679_f32 + y.sin();
        let b = y * 3.062_f32 - x.cos();
        let mut acc = Accumulator136::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.329_f32 + y.sin();
        let b = y * 3.055_f32 - x.cos();
        let mut acc = Accumulator136::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.613_f32 + y.sin();
        let b = y * 5.783_f32 - x.cos();
        let mut acc = Accumulator136::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.691_f32 + y.sin();
        let b = y * 0.106_f32 - x.cos();
        let mut acc = Accumulator136::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.866_f32 + y.sin();
        let b = y * 1.342_f32 - x.cos();
        let mut acc = Accumulator136::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.827_f32 + y.sin();
        let b = y * 7.298_f32 - x.cos();
        let mut acc = Accumulator136::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.384_f32 + y.sin();
        let b = y * 0.296_f32 - x.cos();
        let mut acc = Accumulator136::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.671_f32 + y.sin();
        let b = y * 5.414_f32 - x.cos();
        let mut acc = Accumulator136::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.845_f32 + y.sin();
        let b = y * 2.111_f32 - x.cos();
        let mut acc = Accumulator136::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.081_f32 + y.sin();
        let b = y * 6.819_f32 - x.cos();
        let mut acc = Accumulator136::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.421_f32 + y.sin();
        let b = y * 2.68_f32 - x.cos();
        let mut acc = Accumulator136::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.276_f32 + y.sin();
        let b = y * 0.288_f32 - x.cos();
        let mut acc = Accumulator136::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.335_f32 + y.sin();
        let b = y * 4.93_f32 - x.cos();
        let mut acc = Accumulator136::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.474_f32 + y.sin();
        let b = y * 0.886_f32 - x.cos();
        let mut acc = Accumulator136::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.581_f32 + y.sin();
        let b = y * 2.112_f32 - x.cos();
        let mut acc = Accumulator136::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.836_f32 + y.sin();
        let b = y * 1.297_f32 - x.cos();
        let mut acc = Accumulator136::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.544_f32 + y.sin();
        let b = y * 9.786_f32 - x.cos();
        let mut acc = Accumulator136::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_136(seed: u64) -> u64 {
        let re = Regex::new(r"m136-(\d+)").unwrap();
        let hay = format!("m136-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_136() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_136(total as u64) % 997) as f32;
        total
    }
}

pub mod m137 {
    use super::*;

    pub struct Accumulator137<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator137<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.23_f32 + y.sin();
        let b = y * 6.159_f32 - x.cos();
        let mut acc = Accumulator137::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.96_f32 + y.sin();
        let b = y * 6.003_f32 - x.cos();
        let mut acc = Accumulator137::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.299_f32 + y.sin();
        let b = y * 6.547_f32 - x.cos();
        let mut acc = Accumulator137::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.308_f32 + y.sin();
        let b = y * 8.575_f32 - x.cos();
        let mut acc = Accumulator137::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.845_f32 + y.sin();
        let b = y * 7.64_f32 - x.cos();
        let mut acc = Accumulator137::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.454_f32 + y.sin();
        let b = y * 1.238_f32 - x.cos();
        let mut acc = Accumulator137::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.528_f32 + y.sin();
        let b = y * 8.127_f32 - x.cos();
        let mut acc = Accumulator137::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.827_f32 + y.sin();
        let b = y * 2.033_f32 - x.cos();
        let mut acc = Accumulator137::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.727_f32 + y.sin();
        let b = y * 6.699_f32 - x.cos();
        let mut acc = Accumulator137::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.58_f32 + y.sin();
        let b = y * 4.127_f32 - x.cos();
        let mut acc = Accumulator137::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.907_f32 + y.sin();
        let b = y * 1.906_f32 - x.cos();
        let mut acc = Accumulator137::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.566_f32 + y.sin();
        let b = y * 1.571_f32 - x.cos();
        let mut acc = Accumulator137::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.344_f32 + y.sin();
        let b = y * 5.242_f32 - x.cos();
        let mut acc = Accumulator137::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.474_f32 + y.sin();
        let b = y * 2.156_f32 - x.cos();
        let mut acc = Accumulator137::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.437_f32 + y.sin();
        let b = y * 2.127_f32 - x.cos();
        let mut acc = Accumulator137::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.54_f32 + y.sin();
        let b = y * 0.855_f32 - x.cos();
        let mut acc = Accumulator137::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.904_f32 + y.sin();
        let b = y * 3.229_f32 - x.cos();
        let mut acc = Accumulator137::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.96_f32 + y.sin();
        let b = y * 2.247_f32 - x.cos();
        let mut acc = Accumulator137::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.206_f32 + y.sin();
        let b = y * 0.275_f32 - x.cos();
        let mut acc = Accumulator137::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.021_f32 + y.sin();
        let b = y * 5.647_f32 - x.cos();
        let mut acc = Accumulator137::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_137(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_137() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_137(total as u64) % 997) as f32;
        total
    }
}

pub mod m138 {
    use super::*;

    pub struct Accumulator138<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator138<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.498_f32 + y.sin();
        let b = y * 1.129_f32 - x.cos();
        let mut acc = Accumulator138::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.455_f32 + y.sin();
        let b = y * 5.875_f32 - x.cos();
        let mut acc = Accumulator138::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.544_f32 + y.sin();
        let b = y * 3.767_f32 - x.cos();
        let mut acc = Accumulator138::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.977_f32 + y.sin();
        let b = y * 4.247_f32 - x.cos();
        let mut acc = Accumulator138::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.875_f32 + y.sin();
        let b = y * 6.893_f32 - x.cos();
        let mut acc = Accumulator138::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.642_f32 + y.sin();
        let b = y * 5.379_f32 - x.cos();
        let mut acc = Accumulator138::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.95_f32 + y.sin();
        let b = y * 9.178_f32 - x.cos();
        let mut acc = Accumulator138::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.612_f32 + y.sin();
        let b = y * 7.548_f32 - x.cos();
        let mut acc = Accumulator138::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.712_f32 + y.sin();
        let b = y * 4.003_f32 - x.cos();
        let mut acc = Accumulator138::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.321_f32 + y.sin();
        let b = y * 4.7_f32 - x.cos();
        let mut acc = Accumulator138::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.055_f32 + y.sin();
        let b = y * 4.895_f32 - x.cos();
        let mut acc = Accumulator138::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.147_f32 + y.sin();
        let b = y * 2.73_f32 - x.cos();
        let mut acc = Accumulator138::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.799_f32 + y.sin();
        let b = y * 9.889_f32 - x.cos();
        let mut acc = Accumulator138::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.686_f32 + y.sin();
        let b = y * 6.658_f32 - x.cos();
        let mut acc = Accumulator138::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.922_f32 + y.sin();
        let b = y * 0.522_f32 - x.cos();
        let mut acc = Accumulator138::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.603_f32 + y.sin();
        let b = y * 0.354_f32 - x.cos();
        let mut acc = Accumulator138::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.705_f32 + y.sin();
        let b = y * 8.274_f32 - x.cos();
        let mut acc = Accumulator138::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.131_f32 + y.sin();
        let b = y * 4.843_f32 - x.cos();
        let mut acc = Accumulator138::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.741_f32 + y.sin();
        let b = y * 4.406_f32 - x.cos();
        let mut acc = Accumulator138::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.785_f32 + y.sin();
        let b = y * 2.815_f32 - x.cos();
        let mut acc = Accumulator138::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_138(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(138u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_138() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_138(total as u64) % 997) as f32;
        total
    }
}

pub mod m139 {
    use super::*;

    pub struct Accumulator139<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator139<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.907_f32 + y.sin();
        let b = y * 1.409_f32 - x.cos();
        let mut acc = Accumulator139::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.235_f32 + y.sin();
        let b = y * 0.943_f32 - x.cos();
        let mut acc = Accumulator139::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.857_f32 + y.sin();
        let b = y * 3.547_f32 - x.cos();
        let mut acc = Accumulator139::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.243_f32 + y.sin();
        let b = y * 4.139_f32 - x.cos();
        let mut acc = Accumulator139::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.9_f32 + y.sin();
        let b = y * 0.151_f32 - x.cos();
        let mut acc = Accumulator139::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.568_f32 + y.sin();
        let b = y * 8.974_f32 - x.cos();
        let mut acc = Accumulator139::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.014_f32 + y.sin();
        let b = y * 3.877_f32 - x.cos();
        let mut acc = Accumulator139::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.404_f32 + y.sin();
        let b = y * 5.741_f32 - x.cos();
        let mut acc = Accumulator139::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.391_f32 + y.sin();
        let b = y * 5.928_f32 - x.cos();
        let mut acc = Accumulator139::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.377_f32 + y.sin();
        let b = y * 0.453_f32 - x.cos();
        let mut acc = Accumulator139::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.337_f32 + y.sin();
        let b = y * 5.805_f32 - x.cos();
        let mut acc = Accumulator139::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.921_f32 + y.sin();
        let b = y * 3.122_f32 - x.cos();
        let mut acc = Accumulator139::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.401_f32 + y.sin();
        let b = y * 2.88_f32 - x.cos();
        let mut acc = Accumulator139::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.715_f32 + y.sin();
        let b = y * 2.801_f32 - x.cos();
        let mut acc = Accumulator139::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.282_f32 + y.sin();
        let b = y * 2.755_f32 - x.cos();
        let mut acc = Accumulator139::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.379_f32 + y.sin();
        let b = y * 5.467_f32 - x.cos();
        let mut acc = Accumulator139::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.697_f32 + y.sin();
        let b = y * 8.585_f32 - x.cos();
        let mut acc = Accumulator139::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.054_f32 + y.sin();
        let b = y * 6.862_f32 - x.cos();
        let mut acc = Accumulator139::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.873_f32 + y.sin();
        let b = y * 1.249_f32 - x.cos();
        let mut acc = Accumulator139::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.599_f32 + y.sin();
        let b = y * 8.079_f32 - x.cos();
        let mut acc = Accumulator139::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_139(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_139() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_139(total as u64) % 997) as f32;
        total
    }
}

pub mod m140 {
    use super::*;

    pub struct Accumulator140<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator140<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.869_f32 + y.sin();
        let b = y * 8.692_f32 - x.cos();
        let mut acc = Accumulator140::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.627_f32 + y.sin();
        let b = y * 1.577_f32 - x.cos();
        let mut acc = Accumulator140::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.339_f32 + y.sin();
        let b = y * 8.237_f32 - x.cos();
        let mut acc = Accumulator140::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.671_f32 + y.sin();
        let b = y * 5.723_f32 - x.cos();
        let mut acc = Accumulator140::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.578_f32 + y.sin();
        let b = y * 4.807_f32 - x.cos();
        let mut acc = Accumulator140::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.16_f32 + y.sin();
        let b = y * 3.996_f32 - x.cos();
        let mut acc = Accumulator140::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.134_f32 + y.sin();
        let b = y * 7.191_f32 - x.cos();
        let mut acc = Accumulator140::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.413_f32 + y.sin();
        let b = y * 2.886_f32 - x.cos();
        let mut acc = Accumulator140::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.447_f32 + y.sin();
        let b = y * 5.927_f32 - x.cos();
        let mut acc = Accumulator140::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.552_f32 + y.sin();
        let b = y * 6.338_f32 - x.cos();
        let mut acc = Accumulator140::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.416_f32 + y.sin();
        let b = y * 6.139_f32 - x.cos();
        let mut acc = Accumulator140::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.447_f32 + y.sin();
        let b = y * 1.41_f32 - x.cos();
        let mut acc = Accumulator140::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.481_f32 + y.sin();
        let b = y * 7.137_f32 - x.cos();
        let mut acc = Accumulator140::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.478_f32 + y.sin();
        let b = y * 2.896_f32 - x.cos();
        let mut acc = Accumulator140::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.14_f32 + y.sin();
        let b = y * 9.752_f32 - x.cos();
        let mut acc = Accumulator140::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.358_f32 + y.sin();
        let b = y * 2.593_f32 - x.cos();
        let mut acc = Accumulator140::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.615_f32 + y.sin();
        let b = y * 7.389_f32 - x.cos();
        let mut acc = Accumulator140::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.458_f32 + y.sin();
        let b = y * 5.06_f32 - x.cos();
        let mut acc = Accumulator140::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.81_f32 + y.sin();
        let b = y * 4.518_f32 - x.cos();
        let mut acc = Accumulator140::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.177_f32 + y.sin();
        let b = y * 3.274_f32 - x.cos();
        let mut acc = Accumulator140::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_140(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_140() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_140(total as u64) % 997) as f32;
        total
    }
}

pub mod m141 {
    use super::*;

    pub struct Accumulator141<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator141<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.8_f32 + y.sin();
        let b = y * 1.923_f32 - x.cos();
        let mut acc = Accumulator141::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.454_f32 + y.sin();
        let b = y * 4.05_f32 - x.cos();
        let mut acc = Accumulator141::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.835_f32 + y.sin();
        let b = y * 0.739_f32 - x.cos();
        let mut acc = Accumulator141::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.251_f32 + y.sin();
        let b = y * 9.169_f32 - x.cos();
        let mut acc = Accumulator141::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.355_f32 + y.sin();
        let b = y * 1.124_f32 - x.cos();
        let mut acc = Accumulator141::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.567_f32 + y.sin();
        let b = y * 0.945_f32 - x.cos();
        let mut acc = Accumulator141::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.67_f32 + y.sin();
        let b = y * 9.556_f32 - x.cos();
        let mut acc = Accumulator141::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.577_f32 + y.sin();
        let b = y * 7.124_f32 - x.cos();
        let mut acc = Accumulator141::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.135_f32 + y.sin();
        let b = y * 1.325_f32 - x.cos();
        let mut acc = Accumulator141::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.06_f32 + y.sin();
        let b = y * 5.783_f32 - x.cos();
        let mut acc = Accumulator141::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.295_f32 + y.sin();
        let b = y * 1.019_f32 - x.cos();
        let mut acc = Accumulator141::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.875_f32 + y.sin();
        let b = y * 1.279_f32 - x.cos();
        let mut acc = Accumulator141::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.407_f32 + y.sin();
        let b = y * 2.407_f32 - x.cos();
        let mut acc = Accumulator141::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.338_f32 + y.sin();
        let b = y * 9.591_f32 - x.cos();
        let mut acc = Accumulator141::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.572_f32 + y.sin();
        let b = y * 1.319_f32 - x.cos();
        let mut acc = Accumulator141::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.473_f32 + y.sin();
        let b = y * 2.485_f32 - x.cos();
        let mut acc = Accumulator141::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.26_f32 + y.sin();
        let b = y * 2.216_f32 - x.cos();
        let mut acc = Accumulator141::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.147_f32 + y.sin();
        let b = y * 1.414_f32 - x.cos();
        let mut acc = Accumulator141::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.201_f32 + y.sin();
        let b = y * 0.196_f32 - x.cos();
        let mut acc = Accumulator141::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.669_f32 + y.sin();
        let b = y * 2.689_f32 - x.cos();
        let mut acc = Accumulator141::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_141(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m141-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_141() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_141(total as u64) % 997) as f32;
        total
    }
}

pub mod m142 {
    use super::*;

    pub struct Accumulator142<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator142<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.019_f32 + y.sin();
        let b = y * 3.396_f32 - x.cos();
        let mut acc = Accumulator142::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.61_f32 + y.sin();
        let b = y * 6.063_f32 - x.cos();
        let mut acc = Accumulator142::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.878_f32 + y.sin();
        let b = y * 2.016_f32 - x.cos();
        let mut acc = Accumulator142::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.656_f32 + y.sin();
        let b = y * 5.169_f32 - x.cos();
        let mut acc = Accumulator142::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.089_f32 + y.sin();
        let b = y * 1.849_f32 - x.cos();
        let mut acc = Accumulator142::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.879_f32 + y.sin();
        let b = y * 1.147_f32 - x.cos();
        let mut acc = Accumulator142::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.275_f32 + y.sin();
        let b = y * 3.811_f32 - x.cos();
        let mut acc = Accumulator142::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.597_f32 + y.sin();
        let b = y * 9.545_f32 - x.cos();
        let mut acc = Accumulator142::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.022_f32 + y.sin();
        let b = y * 0.785_f32 - x.cos();
        let mut acc = Accumulator142::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.483_f32 + y.sin();
        let b = y * 3.711_f32 - x.cos();
        let mut acc = Accumulator142::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.643_f32 + y.sin();
        let b = y * 3.833_f32 - x.cos();
        let mut acc = Accumulator142::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.358_f32 + y.sin();
        let b = y * 1.875_f32 - x.cos();
        let mut acc = Accumulator142::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.339_f32 + y.sin();
        let b = y * 5.81_f32 - x.cos();
        let mut acc = Accumulator142::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.087_f32 + y.sin();
        let b = y * 0.298_f32 - x.cos();
        let mut acc = Accumulator142::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.658_f32 + y.sin();
        let b = y * 5.331_f32 - x.cos();
        let mut acc = Accumulator142::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.143_f32 + y.sin();
        let b = y * 4.679_f32 - x.cos();
        let mut acc = Accumulator142::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.008_f32 + y.sin();
        let b = y * 1.221_f32 - x.cos();
        let mut acc = Accumulator142::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.098_f32 + y.sin();
        let b = y * 5.319_f32 - x.cos();
        let mut acc = Accumulator142::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.874_f32 + y.sin();
        let b = y * 8.07_f32 - x.cos();
        let mut acc = Accumulator142::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.584_f32 + y.sin();
        let b = y * 2.586_f32 - x.cos();
        let mut acc = Accumulator142::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_142(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_142() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_142(total as u64) % 997) as f32;
        total
    }
}

pub mod m143 {
    use super::*;

    pub struct Accumulator143<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator143<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.94_f32 + y.sin();
        let b = y * 9.294_f32 - x.cos();
        let mut acc = Accumulator143::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.132_f32 + y.sin();
        let b = y * 5.849_f32 - x.cos();
        let mut acc = Accumulator143::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.343_f32 + y.sin();
        let b = y * 8.492_f32 - x.cos();
        let mut acc = Accumulator143::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.039_f32 + y.sin();
        let b = y * 6.147_f32 - x.cos();
        let mut acc = Accumulator143::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.793_f32 + y.sin();
        let b = y * 1.67_f32 - x.cos();
        let mut acc = Accumulator143::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.35_f32 + y.sin();
        let b = y * 0.974_f32 - x.cos();
        let mut acc = Accumulator143::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.836_f32 + y.sin();
        let b = y * 1.112_f32 - x.cos();
        let mut acc = Accumulator143::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.158_f32 + y.sin();
        let b = y * 8.511_f32 - x.cos();
        let mut acc = Accumulator143::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.372_f32 + y.sin();
        let b = y * 5.713_f32 - x.cos();
        let mut acc = Accumulator143::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.533_f32 + y.sin();
        let b = y * 9.1_f32 - x.cos();
        let mut acc = Accumulator143::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.274_f32 + y.sin();
        let b = y * 3.548_f32 - x.cos();
        let mut acc = Accumulator143::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.65_f32 + y.sin();
        let b = y * 9.281_f32 - x.cos();
        let mut acc = Accumulator143::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.953_f32 + y.sin();
        let b = y * 3.253_f32 - x.cos();
        let mut acc = Accumulator143::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.946_f32 + y.sin();
        let b = y * 0.291_f32 - x.cos();
        let mut acc = Accumulator143::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.762_f32 + y.sin();
        let b = y * 9.186_f32 - x.cos();
        let mut acc = Accumulator143::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.955_f32 + y.sin();
        let b = y * 0.4_f32 - x.cos();
        let mut acc = Accumulator143::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.834_f32 + y.sin();
        let b = y * 1.098_f32 - x.cos();
        let mut acc = Accumulator143::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.004_f32 + y.sin();
        let b = y * 0.451_f32 - x.cos();
        let mut acc = Accumulator143::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.201_f32 + y.sin();
        let b = y * 0.276_f32 - x.cos();
        let mut acc = Accumulator143::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.533_f32 + y.sin();
        let b = y * 1.32_f32 - x.cos();
        let mut acc = Accumulator143::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_143(seed: u64) -> u64 {
        let re = Regex::new(r"m143-(\d+)").unwrap();
        let hay = format!("m143-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_143() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_143(total as u64) % 997) as f32;
        total
    }
}

pub mod m144 {
    use super::*;

    pub struct Accumulator144<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator144<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.44_f32 + y.sin();
        let b = y * 1.712_f32 - x.cos();
        let mut acc = Accumulator144::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.33_f32 + y.sin();
        let b = y * 9.894_f32 - x.cos();
        let mut acc = Accumulator144::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.532_f32 + y.sin();
        let b = y * 9.643_f32 - x.cos();
        let mut acc = Accumulator144::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.642_f32 + y.sin();
        let b = y * 8.041_f32 - x.cos();
        let mut acc = Accumulator144::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.709_f32 + y.sin();
        let b = y * 1.888_f32 - x.cos();
        let mut acc = Accumulator144::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.005_f32 + y.sin();
        let b = y * 0.173_f32 - x.cos();
        let mut acc = Accumulator144::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.271_f32 + y.sin();
        let b = y * 8.393_f32 - x.cos();
        let mut acc = Accumulator144::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.416_f32 + y.sin();
        let b = y * 9.317_f32 - x.cos();
        let mut acc = Accumulator144::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.744_f32 + y.sin();
        let b = y * 1.881_f32 - x.cos();
        let mut acc = Accumulator144::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.053_f32 + y.sin();
        let b = y * 8.911_f32 - x.cos();
        let mut acc = Accumulator144::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.553_f32 + y.sin();
        let b = y * 6.535_f32 - x.cos();
        let mut acc = Accumulator144::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.217_f32 + y.sin();
        let b = y * 1.496_f32 - x.cos();
        let mut acc = Accumulator144::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.84_f32 + y.sin();
        let b = y * 8.615_f32 - x.cos();
        let mut acc = Accumulator144::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.811_f32 + y.sin();
        let b = y * 3.629_f32 - x.cos();
        let mut acc = Accumulator144::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.248_f32 + y.sin();
        let b = y * 8.982_f32 - x.cos();
        let mut acc = Accumulator144::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.258_f32 + y.sin();
        let b = y * 1.502_f32 - x.cos();
        let mut acc = Accumulator144::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.299_f32 + y.sin();
        let b = y * 6.59_f32 - x.cos();
        let mut acc = Accumulator144::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.276_f32 + y.sin();
        let b = y * 5.625_f32 - x.cos();
        let mut acc = Accumulator144::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.11_f32 + y.sin();
        let b = y * 5.384_f32 - x.cos();
        let mut acc = Accumulator144::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.647_f32 + y.sin();
        let b = y * 4.843_f32 - x.cos();
        let mut acc = Accumulator144::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_144(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_144() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_144(total as u64) % 997) as f32;
        total
    }
}

pub mod m145 {
    use super::*;

    pub struct Accumulator145<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator145<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.813_f32 + y.sin();
        let b = y * 7.573_f32 - x.cos();
        let mut acc = Accumulator145::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.621_f32 + y.sin();
        let b = y * 2.718_f32 - x.cos();
        let mut acc = Accumulator145::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.095_f32 + y.sin();
        let b = y * 0.941_f32 - x.cos();
        let mut acc = Accumulator145::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.56_f32 + y.sin();
        let b = y * 9.879_f32 - x.cos();
        let mut acc = Accumulator145::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.571_f32 + y.sin();
        let b = y * 6.993_f32 - x.cos();
        let mut acc = Accumulator145::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.299_f32 + y.sin();
        let b = y * 7.95_f32 - x.cos();
        let mut acc = Accumulator145::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.214_f32 + y.sin();
        let b = y * 1.893_f32 - x.cos();
        let mut acc = Accumulator145::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.715_f32 + y.sin();
        let b = y * 6.598_f32 - x.cos();
        let mut acc = Accumulator145::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.446_f32 + y.sin();
        let b = y * 5.668_f32 - x.cos();
        let mut acc = Accumulator145::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.283_f32 + y.sin();
        let b = y * 4.659_f32 - x.cos();
        let mut acc = Accumulator145::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.863_f32 + y.sin();
        let b = y * 5.512_f32 - x.cos();
        let mut acc = Accumulator145::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.722_f32 + y.sin();
        let b = y * 3.85_f32 - x.cos();
        let mut acc = Accumulator145::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.858_f32 + y.sin();
        let b = y * 8.997_f32 - x.cos();
        let mut acc = Accumulator145::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.722_f32 + y.sin();
        let b = y * 4.892_f32 - x.cos();
        let mut acc = Accumulator145::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.873_f32 + y.sin();
        let b = y * 7.986_f32 - x.cos();
        let mut acc = Accumulator145::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.652_f32 + y.sin();
        let b = y * 9.745_f32 - x.cos();
        let mut acc = Accumulator145::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.721_f32 + y.sin();
        let b = y * 8.276_f32 - x.cos();
        let mut acc = Accumulator145::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.274_f32 + y.sin();
        let b = y * 8.384_f32 - x.cos();
        let mut acc = Accumulator145::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.594_f32 + y.sin();
        let b = y * 4.125_f32 - x.cos();
        let mut acc = Accumulator145::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.539_f32 + y.sin();
        let b = y * 7.584_f32 - x.cos();
        let mut acc = Accumulator145::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_145(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(145u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_145() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_145(total as u64) % 997) as f32;
        total
    }
}

pub mod m146 {
    use super::*;

    pub struct Accumulator146<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator146<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.819_f32 + y.sin();
        let b = y * 4.901_f32 - x.cos();
        let mut acc = Accumulator146::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.103_f32 + y.sin();
        let b = y * 4.503_f32 - x.cos();
        let mut acc = Accumulator146::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.607_f32 + y.sin();
        let b = y * 6.826_f32 - x.cos();
        let mut acc = Accumulator146::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.704_f32 + y.sin();
        let b = y * 6.674_f32 - x.cos();
        let mut acc = Accumulator146::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.968_f32 + y.sin();
        let b = y * 0.698_f32 - x.cos();
        let mut acc = Accumulator146::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.287_f32 + y.sin();
        let b = y * 8.939_f32 - x.cos();
        let mut acc = Accumulator146::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.81_f32 + y.sin();
        let b = y * 5.142_f32 - x.cos();
        let mut acc = Accumulator146::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.069_f32 + y.sin();
        let b = y * 2.948_f32 - x.cos();
        let mut acc = Accumulator146::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.016_f32 + y.sin();
        let b = y * 9.478_f32 - x.cos();
        let mut acc = Accumulator146::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.666_f32 + y.sin();
        let b = y * 2.807_f32 - x.cos();
        let mut acc = Accumulator146::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.157_f32 + y.sin();
        let b = y * 5.99_f32 - x.cos();
        let mut acc = Accumulator146::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.775_f32 + y.sin();
        let b = y * 9.42_f32 - x.cos();
        let mut acc = Accumulator146::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.191_f32 + y.sin();
        let b = y * 9.767_f32 - x.cos();
        let mut acc = Accumulator146::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.063_f32 + y.sin();
        let b = y * 5.264_f32 - x.cos();
        let mut acc = Accumulator146::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.873_f32 + y.sin();
        let b = y * 3.3_f32 - x.cos();
        let mut acc = Accumulator146::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.811_f32 + y.sin();
        let b = y * 5.556_f32 - x.cos();
        let mut acc = Accumulator146::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.519_f32 + y.sin();
        let b = y * 0.828_f32 - x.cos();
        let mut acc = Accumulator146::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.947_f32 + y.sin();
        let b = y * 7.158_f32 - x.cos();
        let mut acc = Accumulator146::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.733_f32 + y.sin();
        let b = y * 9.639_f32 - x.cos();
        let mut acc = Accumulator146::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.752_f32 + y.sin();
        let b = y * 2.135_f32 - x.cos();
        let mut acc = Accumulator146::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_146(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_146() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_146(total as u64) % 997) as f32;
        total
    }
}

pub mod m147 {
    use super::*;

    pub struct Accumulator147<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator147<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.599_f32 + y.sin();
        let b = y * 7.34_f32 - x.cos();
        let mut acc = Accumulator147::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.38_f32 + y.sin();
        let b = y * 8.588_f32 - x.cos();
        let mut acc = Accumulator147::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.195_f32 + y.sin();
        let b = y * 7.848_f32 - x.cos();
        let mut acc = Accumulator147::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.124_f32 + y.sin();
        let b = y * 5.466_f32 - x.cos();
        let mut acc = Accumulator147::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.847_f32 + y.sin();
        let b = y * 5.566_f32 - x.cos();
        let mut acc = Accumulator147::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.776_f32 + y.sin();
        let b = y * 4.818_f32 - x.cos();
        let mut acc = Accumulator147::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.758_f32 + y.sin();
        let b = y * 2.949_f32 - x.cos();
        let mut acc = Accumulator147::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.68_f32 + y.sin();
        let b = y * 0.704_f32 - x.cos();
        let mut acc = Accumulator147::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.241_f32 + y.sin();
        let b = y * 5.769_f32 - x.cos();
        let mut acc = Accumulator147::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.615_f32 + y.sin();
        let b = y * 3.905_f32 - x.cos();
        let mut acc = Accumulator147::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.323_f32 + y.sin();
        let b = y * 6.279_f32 - x.cos();
        let mut acc = Accumulator147::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.461_f32 + y.sin();
        let b = y * 8.126_f32 - x.cos();
        let mut acc = Accumulator147::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.818_f32 + y.sin();
        let b = y * 5.249_f32 - x.cos();
        let mut acc = Accumulator147::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.896_f32 + y.sin();
        let b = y * 2.117_f32 - x.cos();
        let mut acc = Accumulator147::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.066_f32 + y.sin();
        let b = y * 7.927_f32 - x.cos();
        let mut acc = Accumulator147::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.522_f32 + y.sin();
        let b = y * 4.097_f32 - x.cos();
        let mut acc = Accumulator147::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.5_f32 + y.sin();
        let b = y * 7.005_f32 - x.cos();
        let mut acc = Accumulator147::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.489_f32 + y.sin();
        let b = y * 8.476_f32 - x.cos();
        let mut acc = Accumulator147::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.231_f32 + y.sin();
        let b = y * 7.503_f32 - x.cos();
        let mut acc = Accumulator147::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.083_f32 + y.sin();
        let b = y * 3.303_f32 - x.cos();
        let mut acc = Accumulator147::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_147(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_147() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_147(total as u64) % 997) as f32;
        total
    }
}

pub mod m148 {
    use super::*;

    pub struct Accumulator148<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator148<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.465_f32 + y.sin();
        let b = y * 0.7_f32 - x.cos();
        let mut acc = Accumulator148::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.966_f32 + y.sin();
        let b = y * 1.399_f32 - x.cos();
        let mut acc = Accumulator148::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.145_f32 + y.sin();
        let b = y * 1.995_f32 - x.cos();
        let mut acc = Accumulator148::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.487_f32 + y.sin();
        let b = y * 6.731_f32 - x.cos();
        let mut acc = Accumulator148::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.879_f32 + y.sin();
        let b = y * 9.294_f32 - x.cos();
        let mut acc = Accumulator148::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.205_f32 + y.sin();
        let b = y * 5.358_f32 - x.cos();
        let mut acc = Accumulator148::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.853_f32 + y.sin();
        let b = y * 6.662_f32 - x.cos();
        let mut acc = Accumulator148::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.329_f32 + y.sin();
        let b = y * 3.337_f32 - x.cos();
        let mut acc = Accumulator148::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.574_f32 + y.sin();
        let b = y * 5.559_f32 - x.cos();
        let mut acc = Accumulator148::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.136_f32 + y.sin();
        let b = y * 9.646_f32 - x.cos();
        let mut acc = Accumulator148::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.942_f32 + y.sin();
        let b = y * 1.2_f32 - x.cos();
        let mut acc = Accumulator148::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.523_f32 + y.sin();
        let b = y * 6.107_f32 - x.cos();
        let mut acc = Accumulator148::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.487_f32 + y.sin();
        let b = y * 2.391_f32 - x.cos();
        let mut acc = Accumulator148::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.937_f32 + y.sin();
        let b = y * 8.277_f32 - x.cos();
        let mut acc = Accumulator148::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.515_f32 + y.sin();
        let b = y * 6.785_f32 - x.cos();
        let mut acc = Accumulator148::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.507_f32 + y.sin();
        let b = y * 0.922_f32 - x.cos();
        let mut acc = Accumulator148::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.721_f32 + y.sin();
        let b = y * 4.301_f32 - x.cos();
        let mut acc = Accumulator148::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.602_f32 + y.sin();
        let b = y * 5.43_f32 - x.cos();
        let mut acc = Accumulator148::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.998_f32 + y.sin();
        let b = y * 3.672_f32 - x.cos();
        let mut acc = Accumulator148::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.246_f32 + y.sin();
        let b = y * 0.827_f32 - x.cos();
        let mut acc = Accumulator148::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_148(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m148-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_148() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_148(total as u64) % 997) as f32;
        total
    }
}

pub mod m149 {
    use super::*;

    pub struct Accumulator149<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator149<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.685_f32 + y.sin();
        let b = y * 8.569_f32 - x.cos();
        let mut acc = Accumulator149::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.148_f32 + y.sin();
        let b = y * 9.831_f32 - x.cos();
        let mut acc = Accumulator149::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.304_f32 + y.sin();
        let b = y * 8.557_f32 - x.cos();
        let mut acc = Accumulator149::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.833_f32 + y.sin();
        let b = y * 5.092_f32 - x.cos();
        let mut acc = Accumulator149::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.532_f32 + y.sin();
        let b = y * 8.667_f32 - x.cos();
        let mut acc = Accumulator149::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.314_f32 + y.sin();
        let b = y * 0.34_f32 - x.cos();
        let mut acc = Accumulator149::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.981_f32 + y.sin();
        let b = y * 3.773_f32 - x.cos();
        let mut acc = Accumulator149::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.135_f32 + y.sin();
        let b = y * 6.172_f32 - x.cos();
        let mut acc = Accumulator149::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.451_f32 + y.sin();
        let b = y * 5.795_f32 - x.cos();
        let mut acc = Accumulator149::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.471_f32 + y.sin();
        let b = y * 2.793_f32 - x.cos();
        let mut acc = Accumulator149::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.854_f32 + y.sin();
        let b = y * 4.301_f32 - x.cos();
        let mut acc = Accumulator149::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.686_f32 + y.sin();
        let b = y * 2.981_f32 - x.cos();
        let mut acc = Accumulator149::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.487_f32 + y.sin();
        let b = y * 0.509_f32 - x.cos();
        let mut acc = Accumulator149::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.148_f32 + y.sin();
        let b = y * 6.447_f32 - x.cos();
        let mut acc = Accumulator149::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.646_f32 + y.sin();
        let b = y * 0.656_f32 - x.cos();
        let mut acc = Accumulator149::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.87_f32 + y.sin();
        let b = y * 5.225_f32 - x.cos();
        let mut acc = Accumulator149::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.091_f32 + y.sin();
        let b = y * 1.917_f32 - x.cos();
        let mut acc = Accumulator149::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.0_f32 + y.sin();
        let b = y * 0.576_f32 - x.cos();
        let mut acc = Accumulator149::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.313_f32 + y.sin();
        let b = y * 0.414_f32 - x.cos();
        let mut acc = Accumulator149::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.709_f32 + y.sin();
        let b = y * 5.419_f32 - x.cos();
        let mut acc = Accumulator149::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_149(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_149() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_149(total as u64) % 997) as f32;
        total
    }
}

pub mod m150 {
    use super::*;

    pub struct Accumulator150<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator150<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.205_f32 + y.sin();
        let b = y * 4.735_f32 - x.cos();
        let mut acc = Accumulator150::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.996_f32 + y.sin();
        let b = y * 0.466_f32 - x.cos();
        let mut acc = Accumulator150::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.053_f32 + y.sin();
        let b = y * 0.525_f32 - x.cos();
        let mut acc = Accumulator150::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.78_f32 + y.sin();
        let b = y * 3.502_f32 - x.cos();
        let mut acc = Accumulator150::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.311_f32 + y.sin();
        let b = y * 7.673_f32 - x.cos();
        let mut acc = Accumulator150::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.341_f32 + y.sin();
        let b = y * 3.809_f32 - x.cos();
        let mut acc = Accumulator150::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.629_f32 + y.sin();
        let b = y * 6.639_f32 - x.cos();
        let mut acc = Accumulator150::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.425_f32 + y.sin();
        let b = y * 7.307_f32 - x.cos();
        let mut acc = Accumulator150::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.794_f32 + y.sin();
        let b = y * 5.051_f32 - x.cos();
        let mut acc = Accumulator150::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.314_f32 + y.sin();
        let b = y * 1.709_f32 - x.cos();
        let mut acc = Accumulator150::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.573_f32 + y.sin();
        let b = y * 1.204_f32 - x.cos();
        let mut acc = Accumulator150::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.569_f32 + y.sin();
        let b = y * 5.217_f32 - x.cos();
        let mut acc = Accumulator150::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.115_f32 + y.sin();
        let b = y * 7.841_f32 - x.cos();
        let mut acc = Accumulator150::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.197_f32 + y.sin();
        let b = y * 9.375_f32 - x.cos();
        let mut acc = Accumulator150::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.224_f32 + y.sin();
        let b = y * 4.08_f32 - x.cos();
        let mut acc = Accumulator150::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.846_f32 + y.sin();
        let b = y * 4.646_f32 - x.cos();
        let mut acc = Accumulator150::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.976_f32 + y.sin();
        let b = y * 1.802_f32 - x.cos();
        let mut acc = Accumulator150::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.82_f32 + y.sin();
        let b = y * 0.911_f32 - x.cos();
        let mut acc = Accumulator150::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.415_f32 + y.sin();
        let b = y * 2.995_f32 - x.cos();
        let mut acc = Accumulator150::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.529_f32 + y.sin();
        let b = y * 3.717_f32 - x.cos();
        let mut acc = Accumulator150::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_150(seed: u64) -> u64 {
        let re = Regex::new(r"m150-(\d+)").unwrap();
        let hay = format!("m150-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_150() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_150(total as u64) % 997) as f32;
        total
    }
}

pub mod m151 {
    use super::*;

    pub struct Accumulator151<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator151<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.187_f32 + y.sin();
        let b = y * 7.584_f32 - x.cos();
        let mut acc = Accumulator151::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.436_f32 + y.sin();
        let b = y * 4.936_f32 - x.cos();
        let mut acc = Accumulator151::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.594_f32 + y.sin();
        let b = y * 9.409_f32 - x.cos();
        let mut acc = Accumulator151::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.303_f32 + y.sin();
        let b = y * 0.252_f32 - x.cos();
        let mut acc = Accumulator151::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.33_f32 + y.sin();
        let b = y * 7.961_f32 - x.cos();
        let mut acc = Accumulator151::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.228_f32 + y.sin();
        let b = y * 2.958_f32 - x.cos();
        let mut acc = Accumulator151::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.515_f32 + y.sin();
        let b = y * 5.388_f32 - x.cos();
        let mut acc = Accumulator151::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.412_f32 + y.sin();
        let b = y * 0.231_f32 - x.cos();
        let mut acc = Accumulator151::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.031_f32 + y.sin();
        let b = y * 5.296_f32 - x.cos();
        let mut acc = Accumulator151::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.964_f32 + y.sin();
        let b = y * 8.745_f32 - x.cos();
        let mut acc = Accumulator151::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.005_f32 + y.sin();
        let b = y * 3.766_f32 - x.cos();
        let mut acc = Accumulator151::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.172_f32 + y.sin();
        let b = y * 7.839_f32 - x.cos();
        let mut acc = Accumulator151::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.52_f32 + y.sin();
        let b = y * 9.396_f32 - x.cos();
        let mut acc = Accumulator151::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.185_f32 + y.sin();
        let b = y * 4.186_f32 - x.cos();
        let mut acc = Accumulator151::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.136_f32 + y.sin();
        let b = y * 6.862_f32 - x.cos();
        let mut acc = Accumulator151::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.74_f32 + y.sin();
        let b = y * 4.44_f32 - x.cos();
        let mut acc = Accumulator151::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.559_f32 + y.sin();
        let b = y * 5.421_f32 - x.cos();
        let mut acc = Accumulator151::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.75_f32 + y.sin();
        let b = y * 6.081_f32 - x.cos();
        let mut acc = Accumulator151::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.385_f32 + y.sin();
        let b = y * 8.725_f32 - x.cos();
        let mut acc = Accumulator151::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.897_f32 + y.sin();
        let b = y * 7.442_f32 - x.cos();
        let mut acc = Accumulator151::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_151(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_151() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_151(total as u64) % 997) as f32;
        total
    }
}

pub mod m152 {
    use super::*;

    pub struct Accumulator152<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator152<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.803_f32 + y.sin();
        let b = y * 8.511_f32 - x.cos();
        let mut acc = Accumulator152::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.127_f32 + y.sin();
        let b = y * 6.085_f32 - x.cos();
        let mut acc = Accumulator152::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.469_f32 + y.sin();
        let b = y * 4.166_f32 - x.cos();
        let mut acc = Accumulator152::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.941_f32 + y.sin();
        let b = y * 4.212_f32 - x.cos();
        let mut acc = Accumulator152::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.695_f32 + y.sin();
        let b = y * 4.364_f32 - x.cos();
        let mut acc = Accumulator152::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.645_f32 + y.sin();
        let b = y * 6.589_f32 - x.cos();
        let mut acc = Accumulator152::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.658_f32 + y.sin();
        let b = y * 2.196_f32 - x.cos();
        let mut acc = Accumulator152::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.923_f32 + y.sin();
        let b = y * 5.449_f32 - x.cos();
        let mut acc = Accumulator152::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.126_f32 + y.sin();
        let b = y * 0.727_f32 - x.cos();
        let mut acc = Accumulator152::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.59_f32 + y.sin();
        let b = y * 9.174_f32 - x.cos();
        let mut acc = Accumulator152::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.169_f32 + y.sin();
        let b = y * 0.511_f32 - x.cos();
        let mut acc = Accumulator152::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.094_f32 + y.sin();
        let b = y * 3.275_f32 - x.cos();
        let mut acc = Accumulator152::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.103_f32 + y.sin();
        let b = y * 8.965_f32 - x.cos();
        let mut acc = Accumulator152::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.575_f32 + y.sin();
        let b = y * 8.284_f32 - x.cos();
        let mut acc = Accumulator152::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.205_f32 + y.sin();
        let b = y * 5.196_f32 - x.cos();
        let mut acc = Accumulator152::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.969_f32 + y.sin();
        let b = y * 7.982_f32 - x.cos();
        let mut acc = Accumulator152::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.133_f32 + y.sin();
        let b = y * 7.398_f32 - x.cos();
        let mut acc = Accumulator152::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.876_f32 + y.sin();
        let b = y * 7.719_f32 - x.cos();
        let mut acc = Accumulator152::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.893_f32 + y.sin();
        let b = y * 1.918_f32 - x.cos();
        let mut acc = Accumulator152::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.388_f32 + y.sin();
        let b = y * 1.577_f32 - x.cos();
        let mut acc = Accumulator152::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_152(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(152u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_152() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_152(total as u64) % 997) as f32;
        total
    }
}

pub mod m153 {
    use super::*;

    pub struct Accumulator153<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator153<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.186_f32 + y.sin();
        let b = y * 2.872_f32 - x.cos();
        let mut acc = Accumulator153::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.591_f32 + y.sin();
        let b = y * 5.108_f32 - x.cos();
        let mut acc = Accumulator153::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.307_f32 + y.sin();
        let b = y * 5.275_f32 - x.cos();
        let mut acc = Accumulator153::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.163_f32 + y.sin();
        let b = y * 3.776_f32 - x.cos();
        let mut acc = Accumulator153::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.837_f32 + y.sin();
        let b = y * 7.042_f32 - x.cos();
        let mut acc = Accumulator153::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.75_f32 + y.sin();
        let b = y * 8.316_f32 - x.cos();
        let mut acc = Accumulator153::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.21_f32 + y.sin();
        let b = y * 7.246_f32 - x.cos();
        let mut acc = Accumulator153::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.755_f32 + y.sin();
        let b = y * 3.694_f32 - x.cos();
        let mut acc = Accumulator153::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.467_f32 + y.sin();
        let b = y * 9.462_f32 - x.cos();
        let mut acc = Accumulator153::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.82_f32 + y.sin();
        let b = y * 6.928_f32 - x.cos();
        let mut acc = Accumulator153::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.682_f32 + y.sin();
        let b = y * 7.471_f32 - x.cos();
        let mut acc = Accumulator153::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.929_f32 + y.sin();
        let b = y * 2.184_f32 - x.cos();
        let mut acc = Accumulator153::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.927_f32 + y.sin();
        let b = y * 4.717_f32 - x.cos();
        let mut acc = Accumulator153::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.447_f32 + y.sin();
        let b = y * 1.192_f32 - x.cos();
        let mut acc = Accumulator153::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.367_f32 + y.sin();
        let b = y * 1.777_f32 - x.cos();
        let mut acc = Accumulator153::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.869_f32 + y.sin();
        let b = y * 5.637_f32 - x.cos();
        let mut acc = Accumulator153::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.64_f32 + y.sin();
        let b = y * 2.226_f32 - x.cos();
        let mut acc = Accumulator153::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.257_f32 + y.sin();
        let b = y * 2.021_f32 - x.cos();
        let mut acc = Accumulator153::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.337_f32 + y.sin();
        let b = y * 0.765_f32 - x.cos();
        let mut acc = Accumulator153::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.717_f32 + y.sin();
        let b = y * 8.207_f32 - x.cos();
        let mut acc = Accumulator153::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_153(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_153() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_153(total as u64) % 997) as f32;
        total
    }
}

pub mod m154 {
    use super::*;

    pub struct Accumulator154<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator154<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.629_f32 + y.sin();
        let b = y * 2.905_f32 - x.cos();
        let mut acc = Accumulator154::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.295_f32 + y.sin();
        let b = y * 2.437_f32 - x.cos();
        let mut acc = Accumulator154::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.32_f32 + y.sin();
        let b = y * 5.907_f32 - x.cos();
        let mut acc = Accumulator154::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.667_f32 + y.sin();
        let b = y * 2.304_f32 - x.cos();
        let mut acc = Accumulator154::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.232_f32 + y.sin();
        let b = y * 1.564_f32 - x.cos();
        let mut acc = Accumulator154::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.715_f32 + y.sin();
        let b = y * 0.907_f32 - x.cos();
        let mut acc = Accumulator154::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.774_f32 + y.sin();
        let b = y * 4.773_f32 - x.cos();
        let mut acc = Accumulator154::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.683_f32 + y.sin();
        let b = y * 9.85_f32 - x.cos();
        let mut acc = Accumulator154::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.644_f32 + y.sin();
        let b = y * 5.875_f32 - x.cos();
        let mut acc = Accumulator154::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.884_f32 + y.sin();
        let b = y * 1.683_f32 - x.cos();
        let mut acc = Accumulator154::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.869_f32 + y.sin();
        let b = y * 3.663_f32 - x.cos();
        let mut acc = Accumulator154::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.307_f32 + y.sin();
        let b = y * 2.735_f32 - x.cos();
        let mut acc = Accumulator154::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.481_f32 + y.sin();
        let b = y * 9.613_f32 - x.cos();
        let mut acc = Accumulator154::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.283_f32 + y.sin();
        let b = y * 1.203_f32 - x.cos();
        let mut acc = Accumulator154::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.516_f32 + y.sin();
        let b = y * 4.707_f32 - x.cos();
        let mut acc = Accumulator154::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.08_f32 + y.sin();
        let b = y * 5.547_f32 - x.cos();
        let mut acc = Accumulator154::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.812_f32 + y.sin();
        let b = y * 8.124_f32 - x.cos();
        let mut acc = Accumulator154::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.083_f32 + y.sin();
        let b = y * 7.072_f32 - x.cos();
        let mut acc = Accumulator154::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.743_f32 + y.sin();
        let b = y * 0.789_f32 - x.cos();
        let mut acc = Accumulator154::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.533_f32 + y.sin();
        let b = y * 7.958_f32 - x.cos();
        let mut acc = Accumulator154::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_154(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_154() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_154(total as u64) % 997) as f32;
        total
    }
}

pub mod m155 {
    use super::*;

    pub struct Accumulator155<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator155<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.011_f32 + y.sin();
        let b = y * 3.966_f32 - x.cos();
        let mut acc = Accumulator155::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.718_f32 + y.sin();
        let b = y * 7.302_f32 - x.cos();
        let mut acc = Accumulator155::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.585_f32 + y.sin();
        let b = y * 6.156_f32 - x.cos();
        let mut acc = Accumulator155::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.077_f32 + y.sin();
        let b = y * 2.214_f32 - x.cos();
        let mut acc = Accumulator155::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.956_f32 + y.sin();
        let b = y * 1.651_f32 - x.cos();
        let mut acc = Accumulator155::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.934_f32 + y.sin();
        let b = y * 6.848_f32 - x.cos();
        let mut acc = Accumulator155::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.354_f32 + y.sin();
        let b = y * 1.871_f32 - x.cos();
        let mut acc = Accumulator155::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.172_f32 + y.sin();
        let b = y * 4.448_f32 - x.cos();
        let mut acc = Accumulator155::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.116_f32 + y.sin();
        let b = y * 4.681_f32 - x.cos();
        let mut acc = Accumulator155::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.898_f32 + y.sin();
        let b = y * 4.555_f32 - x.cos();
        let mut acc = Accumulator155::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.421_f32 + y.sin();
        let b = y * 7.928_f32 - x.cos();
        let mut acc = Accumulator155::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.608_f32 + y.sin();
        let b = y * 2.62_f32 - x.cos();
        let mut acc = Accumulator155::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.743_f32 + y.sin();
        let b = y * 4.514_f32 - x.cos();
        let mut acc = Accumulator155::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.489_f32 + y.sin();
        let b = y * 0.205_f32 - x.cos();
        let mut acc = Accumulator155::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.147_f32 + y.sin();
        let b = y * 2.693_f32 - x.cos();
        let mut acc = Accumulator155::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.865_f32 + y.sin();
        let b = y * 8.02_f32 - x.cos();
        let mut acc = Accumulator155::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.484_f32 + y.sin();
        let b = y * 6.748_f32 - x.cos();
        let mut acc = Accumulator155::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.485_f32 + y.sin();
        let b = y * 7.071_f32 - x.cos();
        let mut acc = Accumulator155::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.942_f32 + y.sin();
        let b = y * 7.218_f32 - x.cos();
        let mut acc = Accumulator155::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.815_f32 + y.sin();
        let b = y * 5.793_f32 - x.cos();
        let mut acc = Accumulator155::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_155(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m155-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_155() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_155(total as u64) % 997) as f32;
        total
    }
}

pub mod m156 {
    use super::*;

    pub struct Accumulator156<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator156<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.489_f32 + y.sin();
        let b = y * 2.334_f32 - x.cos();
        let mut acc = Accumulator156::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.988_f32 + y.sin();
        let b = y * 4.398_f32 - x.cos();
        let mut acc = Accumulator156::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.42_f32 + y.sin();
        let b = y * 2.459_f32 - x.cos();
        let mut acc = Accumulator156::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.885_f32 + y.sin();
        let b = y * 5.557_f32 - x.cos();
        let mut acc = Accumulator156::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.508_f32 + y.sin();
        let b = y * 1.412_f32 - x.cos();
        let mut acc = Accumulator156::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.362_f32 + y.sin();
        let b = y * 7.443_f32 - x.cos();
        let mut acc = Accumulator156::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.47_f32 + y.sin();
        let b = y * 5.742_f32 - x.cos();
        let mut acc = Accumulator156::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.148_f32 + y.sin();
        let b = y * 3.894_f32 - x.cos();
        let mut acc = Accumulator156::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.645_f32 + y.sin();
        let b = y * 9.462_f32 - x.cos();
        let mut acc = Accumulator156::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.537_f32 + y.sin();
        let b = y * 8.461_f32 - x.cos();
        let mut acc = Accumulator156::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.83_f32 + y.sin();
        let b = y * 7.542_f32 - x.cos();
        let mut acc = Accumulator156::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.884_f32 + y.sin();
        let b = y * 9.805_f32 - x.cos();
        let mut acc = Accumulator156::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.872_f32 + y.sin();
        let b = y * 9.521_f32 - x.cos();
        let mut acc = Accumulator156::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.926_f32 + y.sin();
        let b = y * 3.717_f32 - x.cos();
        let mut acc = Accumulator156::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.711_f32 + y.sin();
        let b = y * 6.695_f32 - x.cos();
        let mut acc = Accumulator156::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.139_f32 + y.sin();
        let b = y * 8.726_f32 - x.cos();
        let mut acc = Accumulator156::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.058_f32 + y.sin();
        let b = y * 3.99_f32 - x.cos();
        let mut acc = Accumulator156::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.387_f32 + y.sin();
        let b = y * 8.055_f32 - x.cos();
        let mut acc = Accumulator156::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.825_f32 + y.sin();
        let b = y * 2.865_f32 - x.cos();
        let mut acc = Accumulator156::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.911_f32 + y.sin();
        let b = y * 6.338_f32 - x.cos();
        let mut acc = Accumulator156::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_156(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_156() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_156(total as u64) % 997) as f32;
        total
    }
}

pub mod m157 {
    use super::*;

    pub struct Accumulator157<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator157<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.424_f32 + y.sin();
        let b = y * 9.111_f32 - x.cos();
        let mut acc = Accumulator157::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.003_f32 + y.sin();
        let b = y * 0.261_f32 - x.cos();
        let mut acc = Accumulator157::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.688_f32 + y.sin();
        let b = y * 9.827_f32 - x.cos();
        let mut acc = Accumulator157::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.999_f32 + y.sin();
        let b = y * 8.371_f32 - x.cos();
        let mut acc = Accumulator157::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.024_f32 + y.sin();
        let b = y * 0.25_f32 - x.cos();
        let mut acc = Accumulator157::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.336_f32 + y.sin();
        let b = y * 5.631_f32 - x.cos();
        let mut acc = Accumulator157::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.165_f32 + y.sin();
        let b = y * 5.084_f32 - x.cos();
        let mut acc = Accumulator157::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.445_f32 + y.sin();
        let b = y * 6.95_f32 - x.cos();
        let mut acc = Accumulator157::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.518_f32 + y.sin();
        let b = y * 7.983_f32 - x.cos();
        let mut acc = Accumulator157::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.479_f32 + y.sin();
        let b = y * 3.038_f32 - x.cos();
        let mut acc = Accumulator157::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.898_f32 + y.sin();
        let b = y * 0.556_f32 - x.cos();
        let mut acc = Accumulator157::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.269_f32 + y.sin();
        let b = y * 9.07_f32 - x.cos();
        let mut acc = Accumulator157::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.988_f32 + y.sin();
        let b = y * 5.833_f32 - x.cos();
        let mut acc = Accumulator157::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.434_f32 + y.sin();
        let b = y * 1.531_f32 - x.cos();
        let mut acc = Accumulator157::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.996_f32 + y.sin();
        let b = y * 4.849_f32 - x.cos();
        let mut acc = Accumulator157::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.77_f32 + y.sin();
        let b = y * 2.177_f32 - x.cos();
        let mut acc = Accumulator157::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.496_f32 + y.sin();
        let b = y * 8.119_f32 - x.cos();
        let mut acc = Accumulator157::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.697_f32 + y.sin();
        let b = y * 8.784_f32 - x.cos();
        let mut acc = Accumulator157::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.137_f32 + y.sin();
        let b = y * 5.942_f32 - x.cos();
        let mut acc = Accumulator157::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.503_f32 + y.sin();
        let b = y * 5.324_f32 - x.cos();
        let mut acc = Accumulator157::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_157(seed: u64) -> u64 {
        let re = Regex::new(r"m157-(\d+)").unwrap();
        let hay = format!("m157-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_157() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_157(total as u64) % 997) as f32;
        total
    }
}

pub mod m158 {
    use super::*;

    pub struct Accumulator158<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator158<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.226_f32 + y.sin();
        let b = y * 7.992_f32 - x.cos();
        let mut acc = Accumulator158::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.148_f32 + y.sin();
        let b = y * 5.204_f32 - x.cos();
        let mut acc = Accumulator158::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.574_f32 + y.sin();
        let b = y * 5.278_f32 - x.cos();
        let mut acc = Accumulator158::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.817_f32 + y.sin();
        let b = y * 5.778_f32 - x.cos();
        let mut acc = Accumulator158::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.969_f32 + y.sin();
        let b = y * 9.474_f32 - x.cos();
        let mut acc = Accumulator158::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.735_f32 + y.sin();
        let b = y * 5.154_f32 - x.cos();
        let mut acc = Accumulator158::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.764_f32 + y.sin();
        let b = y * 1.272_f32 - x.cos();
        let mut acc = Accumulator158::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.121_f32 + y.sin();
        let b = y * 2.906_f32 - x.cos();
        let mut acc = Accumulator158::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.913_f32 + y.sin();
        let b = y * 2.986_f32 - x.cos();
        let mut acc = Accumulator158::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.294_f32 + y.sin();
        let b = y * 3.738_f32 - x.cos();
        let mut acc = Accumulator158::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.005_f32 + y.sin();
        let b = y * 3.205_f32 - x.cos();
        let mut acc = Accumulator158::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.966_f32 + y.sin();
        let b = y * 8.248_f32 - x.cos();
        let mut acc = Accumulator158::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.699_f32 + y.sin();
        let b = y * 7.976_f32 - x.cos();
        let mut acc = Accumulator158::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.905_f32 + y.sin();
        let b = y * 4.743_f32 - x.cos();
        let mut acc = Accumulator158::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.172_f32 + y.sin();
        let b = y * 3.533_f32 - x.cos();
        let mut acc = Accumulator158::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.556_f32 + y.sin();
        let b = y * 4.16_f32 - x.cos();
        let mut acc = Accumulator158::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.082_f32 + y.sin();
        let b = y * 7.61_f32 - x.cos();
        let mut acc = Accumulator158::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.913_f32 + y.sin();
        let b = y * 1.472_f32 - x.cos();
        let mut acc = Accumulator158::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.066_f32 + y.sin();
        let b = y * 3.434_f32 - x.cos();
        let mut acc = Accumulator158::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.62_f32 + y.sin();
        let b = y * 4.932_f32 - x.cos();
        let mut acc = Accumulator158::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_158(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_158() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_158(total as u64) % 997) as f32;
        total
    }
}

pub mod m159 {
    use super::*;

    pub struct Accumulator159<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator159<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.5_f32 + y.sin();
        let b = y * 4.128_f32 - x.cos();
        let mut acc = Accumulator159::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.219_f32 + y.sin();
        let b = y * 0.171_f32 - x.cos();
        let mut acc = Accumulator159::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.334_f32 + y.sin();
        let b = y * 8.316_f32 - x.cos();
        let mut acc = Accumulator159::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.12_f32 + y.sin();
        let b = y * 7.492_f32 - x.cos();
        let mut acc = Accumulator159::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.549_f32 + y.sin();
        let b = y * 4.07_f32 - x.cos();
        let mut acc = Accumulator159::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.572_f32 + y.sin();
        let b = y * 6.502_f32 - x.cos();
        let mut acc = Accumulator159::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.351_f32 + y.sin();
        let b = y * 0.895_f32 - x.cos();
        let mut acc = Accumulator159::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.307_f32 + y.sin();
        let b = y * 7.316_f32 - x.cos();
        let mut acc = Accumulator159::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.899_f32 + y.sin();
        let b = y * 0.862_f32 - x.cos();
        let mut acc = Accumulator159::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.422_f32 + y.sin();
        let b = y * 8.062_f32 - x.cos();
        let mut acc = Accumulator159::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.645_f32 + y.sin();
        let b = y * 2.546_f32 - x.cos();
        let mut acc = Accumulator159::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.485_f32 + y.sin();
        let b = y * 5.504_f32 - x.cos();
        let mut acc = Accumulator159::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.18_f32 + y.sin();
        let b = y * 8.45_f32 - x.cos();
        let mut acc = Accumulator159::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.117_f32 + y.sin();
        let b = y * 7.838_f32 - x.cos();
        let mut acc = Accumulator159::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.598_f32 + y.sin();
        let b = y * 0.757_f32 - x.cos();
        let mut acc = Accumulator159::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.477_f32 + y.sin();
        let b = y * 3.836_f32 - x.cos();
        let mut acc = Accumulator159::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.066_f32 + y.sin();
        let b = y * 7.831_f32 - x.cos();
        let mut acc = Accumulator159::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.912_f32 + y.sin();
        let b = y * 8.809_f32 - x.cos();
        let mut acc = Accumulator159::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.513_f32 + y.sin();
        let b = y * 1.07_f32 - x.cos();
        let mut acc = Accumulator159::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.336_f32 + y.sin();
        let b = y * 7.317_f32 - x.cos();
        let mut acc = Accumulator159::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_159(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(159u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_159() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_159(total as u64) % 997) as f32;
        total
    }
}

pub mod m160 {
    use super::*;

    pub struct Accumulator160<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator160<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.792_f32 + y.sin();
        let b = y * 6.851_f32 - x.cos();
        let mut acc = Accumulator160::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.991_f32 + y.sin();
        let b = y * 1.618_f32 - x.cos();
        let mut acc = Accumulator160::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.694_f32 + y.sin();
        let b = y * 5.897_f32 - x.cos();
        let mut acc = Accumulator160::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.62_f32 + y.sin();
        let b = y * 7.593_f32 - x.cos();
        let mut acc = Accumulator160::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.189_f32 + y.sin();
        let b = y * 0.335_f32 - x.cos();
        let mut acc = Accumulator160::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.52_f32 + y.sin();
        let b = y * 6.373_f32 - x.cos();
        let mut acc = Accumulator160::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.54_f32 + y.sin();
        let b = y * 1.705_f32 - x.cos();
        let mut acc = Accumulator160::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.641_f32 + y.sin();
        let b = y * 5.187_f32 - x.cos();
        let mut acc = Accumulator160::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.189_f32 + y.sin();
        let b = y * 9.708_f32 - x.cos();
        let mut acc = Accumulator160::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.154_f32 + y.sin();
        let b = y * 7.635_f32 - x.cos();
        let mut acc = Accumulator160::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.544_f32 + y.sin();
        let b = y * 1.944_f32 - x.cos();
        let mut acc = Accumulator160::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.156_f32 + y.sin();
        let b = y * 3.385_f32 - x.cos();
        let mut acc = Accumulator160::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.388_f32 + y.sin();
        let b = y * 8.51_f32 - x.cos();
        let mut acc = Accumulator160::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.212_f32 + y.sin();
        let b = y * 6.751_f32 - x.cos();
        let mut acc = Accumulator160::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.387_f32 + y.sin();
        let b = y * 1.33_f32 - x.cos();
        let mut acc = Accumulator160::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.222_f32 + y.sin();
        let b = y * 1.065_f32 - x.cos();
        let mut acc = Accumulator160::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.534_f32 + y.sin();
        let b = y * 9.087_f32 - x.cos();
        let mut acc = Accumulator160::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.071_f32 + y.sin();
        let b = y * 6.501_f32 - x.cos();
        let mut acc = Accumulator160::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.37_f32 + y.sin();
        let b = y * 0.398_f32 - x.cos();
        let mut acc = Accumulator160::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.066_f32 + y.sin();
        let b = y * 8.823_f32 - x.cos();
        let mut acc = Accumulator160::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_160(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_160() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_160(total as u64) % 997) as f32;
        total
    }
}

pub mod m161 {
    use super::*;

    pub struct Accumulator161<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator161<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.531_f32 + y.sin();
        let b = y * 5.513_f32 - x.cos();
        let mut acc = Accumulator161::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.479_f32 + y.sin();
        let b = y * 1.412_f32 - x.cos();
        let mut acc = Accumulator161::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.791_f32 + y.sin();
        let b = y * 5.955_f32 - x.cos();
        let mut acc = Accumulator161::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.235_f32 + y.sin();
        let b = y * 2.7_f32 - x.cos();
        let mut acc = Accumulator161::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.648_f32 + y.sin();
        let b = y * 5.97_f32 - x.cos();
        let mut acc = Accumulator161::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.895_f32 + y.sin();
        let b = y * 3.082_f32 - x.cos();
        let mut acc = Accumulator161::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.014_f32 + y.sin();
        let b = y * 0.975_f32 - x.cos();
        let mut acc = Accumulator161::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.23_f32 + y.sin();
        let b = y * 6.226_f32 - x.cos();
        let mut acc = Accumulator161::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.952_f32 + y.sin();
        let b = y * 2.863_f32 - x.cos();
        let mut acc = Accumulator161::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.646_f32 + y.sin();
        let b = y * 8.6_f32 - x.cos();
        let mut acc = Accumulator161::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.904_f32 + y.sin();
        let b = y * 5.901_f32 - x.cos();
        let mut acc = Accumulator161::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.33_f32 + y.sin();
        let b = y * 7.499_f32 - x.cos();
        let mut acc = Accumulator161::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.457_f32 + y.sin();
        let b = y * 2.147_f32 - x.cos();
        let mut acc = Accumulator161::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.835_f32 + y.sin();
        let b = y * 8.658_f32 - x.cos();
        let mut acc = Accumulator161::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.029_f32 + y.sin();
        let b = y * 9.281_f32 - x.cos();
        let mut acc = Accumulator161::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.877_f32 + y.sin();
        let b = y * 3.839_f32 - x.cos();
        let mut acc = Accumulator161::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.544_f32 + y.sin();
        let b = y * 0.101_f32 - x.cos();
        let mut acc = Accumulator161::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.697_f32 + y.sin();
        let b = y * 1.234_f32 - x.cos();
        let mut acc = Accumulator161::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.222_f32 + y.sin();
        let b = y * 3.037_f32 - x.cos();
        let mut acc = Accumulator161::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.273_f32 + y.sin();
        let b = y * 2.449_f32 - x.cos();
        let mut acc = Accumulator161::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_161(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_161() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_161(total as u64) % 997) as f32;
        total
    }
}

pub mod m162 {
    use super::*;

    pub struct Accumulator162<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator162<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.183_f32 + y.sin();
        let b = y * 6.731_f32 - x.cos();
        let mut acc = Accumulator162::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.501_f32 + y.sin();
        let b = y * 5.776_f32 - x.cos();
        let mut acc = Accumulator162::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.75_f32 + y.sin();
        let b = y * 9.896_f32 - x.cos();
        let mut acc = Accumulator162::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.007_f32 + y.sin();
        let b = y * 0.868_f32 - x.cos();
        let mut acc = Accumulator162::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.502_f32 + y.sin();
        let b = y * 9.513_f32 - x.cos();
        let mut acc = Accumulator162::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.198_f32 + y.sin();
        let b = y * 5.388_f32 - x.cos();
        let mut acc = Accumulator162::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.246_f32 + y.sin();
        let b = y * 2.569_f32 - x.cos();
        let mut acc = Accumulator162::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.021_f32 + y.sin();
        let b = y * 2.614_f32 - x.cos();
        let mut acc = Accumulator162::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.034_f32 + y.sin();
        let b = y * 5.513_f32 - x.cos();
        let mut acc = Accumulator162::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.629_f32 + y.sin();
        let b = y * 0.983_f32 - x.cos();
        let mut acc = Accumulator162::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.385_f32 + y.sin();
        let b = y * 1.134_f32 - x.cos();
        let mut acc = Accumulator162::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.74_f32 + y.sin();
        let b = y * 2.842_f32 - x.cos();
        let mut acc = Accumulator162::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.317_f32 + y.sin();
        let b = y * 3.836_f32 - x.cos();
        let mut acc = Accumulator162::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.261_f32 + y.sin();
        let b = y * 2.49_f32 - x.cos();
        let mut acc = Accumulator162::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.256_f32 + y.sin();
        let b = y * 1.987_f32 - x.cos();
        let mut acc = Accumulator162::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.13_f32 + y.sin();
        let b = y * 9.205_f32 - x.cos();
        let mut acc = Accumulator162::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.304_f32 + y.sin();
        let b = y * 8.688_f32 - x.cos();
        let mut acc = Accumulator162::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.076_f32 + y.sin();
        let b = y * 3.535_f32 - x.cos();
        let mut acc = Accumulator162::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.616_f32 + y.sin();
        let b = y * 0.759_f32 - x.cos();
        let mut acc = Accumulator162::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.489_f32 + y.sin();
        let b = y * 8.487_f32 - x.cos();
        let mut acc = Accumulator162::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_162(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m162-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_162() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_162(total as u64) % 997) as f32;
        total
    }
}

pub mod m163 {
    use super::*;

    pub struct Accumulator163<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator163<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.949_f32 + y.sin();
        let b = y * 6.477_f32 - x.cos();
        let mut acc = Accumulator163::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.71_f32 + y.sin();
        let b = y * 1.563_f32 - x.cos();
        let mut acc = Accumulator163::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.217_f32 + y.sin();
        let b = y * 3.985_f32 - x.cos();
        let mut acc = Accumulator163::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.836_f32 + y.sin();
        let b = y * 1.855_f32 - x.cos();
        let mut acc = Accumulator163::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.519_f32 + y.sin();
        let b = y * 2.22_f32 - x.cos();
        let mut acc = Accumulator163::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.133_f32 + y.sin();
        let b = y * 9.516_f32 - x.cos();
        let mut acc = Accumulator163::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.928_f32 + y.sin();
        let b = y * 9.379_f32 - x.cos();
        let mut acc = Accumulator163::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.208_f32 + y.sin();
        let b = y * 2.397_f32 - x.cos();
        let mut acc = Accumulator163::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.841_f32 + y.sin();
        let b = y * 1.801_f32 - x.cos();
        let mut acc = Accumulator163::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.649_f32 + y.sin();
        let b = y * 8.679_f32 - x.cos();
        let mut acc = Accumulator163::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.055_f32 + y.sin();
        let b = y * 8.231_f32 - x.cos();
        let mut acc = Accumulator163::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.588_f32 + y.sin();
        let b = y * 4.928_f32 - x.cos();
        let mut acc = Accumulator163::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.629_f32 + y.sin();
        let b = y * 3.155_f32 - x.cos();
        let mut acc = Accumulator163::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.032_f32 + y.sin();
        let b = y * 9.424_f32 - x.cos();
        let mut acc = Accumulator163::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.217_f32 + y.sin();
        let b = y * 5.415_f32 - x.cos();
        let mut acc = Accumulator163::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.601_f32 + y.sin();
        let b = y * 9.482_f32 - x.cos();
        let mut acc = Accumulator163::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.256_f32 + y.sin();
        let b = y * 5.623_f32 - x.cos();
        let mut acc = Accumulator163::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.421_f32 + y.sin();
        let b = y * 7.768_f32 - x.cos();
        let mut acc = Accumulator163::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.568_f32 + y.sin();
        let b = y * 9.134_f32 - x.cos();
        let mut acc = Accumulator163::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.429_f32 + y.sin();
        let b = y * 0.457_f32 - x.cos();
        let mut acc = Accumulator163::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_163(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_163() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_163(total as u64) % 997) as f32;
        total
    }
}

pub mod m164 {
    use super::*;

    pub struct Accumulator164<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator164<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.384_f32 + y.sin();
        let b = y * 4.599_f32 - x.cos();
        let mut acc = Accumulator164::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.99_f32 + y.sin();
        let b = y * 9.184_f32 - x.cos();
        let mut acc = Accumulator164::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.104_f32 + y.sin();
        let b = y * 8.464_f32 - x.cos();
        let mut acc = Accumulator164::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.688_f32 + y.sin();
        let b = y * 6.049_f32 - x.cos();
        let mut acc = Accumulator164::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.892_f32 + y.sin();
        let b = y * 2.397_f32 - x.cos();
        let mut acc = Accumulator164::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.169_f32 + y.sin();
        let b = y * 0.233_f32 - x.cos();
        let mut acc = Accumulator164::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.94_f32 + y.sin();
        let b = y * 1.127_f32 - x.cos();
        let mut acc = Accumulator164::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.363_f32 + y.sin();
        let b = y * 6.948_f32 - x.cos();
        let mut acc = Accumulator164::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.229_f32 + y.sin();
        let b = y * 3.387_f32 - x.cos();
        let mut acc = Accumulator164::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.88_f32 + y.sin();
        let b = y * 1.876_f32 - x.cos();
        let mut acc = Accumulator164::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.886_f32 + y.sin();
        let b = y * 4.921_f32 - x.cos();
        let mut acc = Accumulator164::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.669_f32 + y.sin();
        let b = y * 8.867_f32 - x.cos();
        let mut acc = Accumulator164::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.303_f32 + y.sin();
        let b = y * 0.227_f32 - x.cos();
        let mut acc = Accumulator164::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.239_f32 + y.sin();
        let b = y * 6.338_f32 - x.cos();
        let mut acc = Accumulator164::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.222_f32 + y.sin();
        let b = y * 4.078_f32 - x.cos();
        let mut acc = Accumulator164::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.806_f32 + y.sin();
        let b = y * 6.929_f32 - x.cos();
        let mut acc = Accumulator164::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.187_f32 + y.sin();
        let b = y * 1.744_f32 - x.cos();
        let mut acc = Accumulator164::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.373_f32 + y.sin();
        let b = y * 2.777_f32 - x.cos();
        let mut acc = Accumulator164::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.871_f32 + y.sin();
        let b = y * 1.459_f32 - x.cos();
        let mut acc = Accumulator164::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.754_f32 + y.sin();
        let b = y * 3.199_f32 - x.cos();
        let mut acc = Accumulator164::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_164(seed: u64) -> u64 {
        let re = Regex::new(r"m164-(\d+)").unwrap();
        let hay = format!("m164-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_164() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_164(total as u64) % 997) as f32;
        total
    }
}

pub mod m165 {
    use super::*;

    pub struct Accumulator165<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator165<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.205_f32 + y.sin();
        let b = y * 0.285_f32 - x.cos();
        let mut acc = Accumulator165::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.962_f32 + y.sin();
        let b = y * 6.753_f32 - x.cos();
        let mut acc = Accumulator165::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.102_f32 + y.sin();
        let b = y * 6.704_f32 - x.cos();
        let mut acc = Accumulator165::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.999_f32 + y.sin();
        let b = y * 0.596_f32 - x.cos();
        let mut acc = Accumulator165::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.229_f32 + y.sin();
        let b = y * 1.878_f32 - x.cos();
        let mut acc = Accumulator165::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.482_f32 + y.sin();
        let b = y * 7.304_f32 - x.cos();
        let mut acc = Accumulator165::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.815_f32 + y.sin();
        let b = y * 1.021_f32 - x.cos();
        let mut acc = Accumulator165::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.747_f32 + y.sin();
        let b = y * 9.176_f32 - x.cos();
        let mut acc = Accumulator165::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.019_f32 + y.sin();
        let b = y * 0.341_f32 - x.cos();
        let mut acc = Accumulator165::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.685_f32 + y.sin();
        let b = y * 2.125_f32 - x.cos();
        let mut acc = Accumulator165::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.68_f32 + y.sin();
        let b = y * 3.071_f32 - x.cos();
        let mut acc = Accumulator165::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.931_f32 + y.sin();
        let b = y * 5.18_f32 - x.cos();
        let mut acc = Accumulator165::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.29_f32 + y.sin();
        let b = y * 3.184_f32 - x.cos();
        let mut acc = Accumulator165::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.376_f32 + y.sin();
        let b = y * 0.146_f32 - x.cos();
        let mut acc = Accumulator165::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.14_f32 + y.sin();
        let b = y * 6.155_f32 - x.cos();
        let mut acc = Accumulator165::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.329_f32 + y.sin();
        let b = y * 8.321_f32 - x.cos();
        let mut acc = Accumulator165::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.571_f32 + y.sin();
        let b = y * 2.013_f32 - x.cos();
        let mut acc = Accumulator165::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.937_f32 + y.sin();
        let b = y * 9.887_f32 - x.cos();
        let mut acc = Accumulator165::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.487_f32 + y.sin();
        let b = y * 2.79_f32 - x.cos();
        let mut acc = Accumulator165::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.724_f32 + y.sin();
        let b = y * 9.149_f32 - x.cos();
        let mut acc = Accumulator165::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_165(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_165() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_165(total as u64) % 997) as f32;
        total
    }
}

pub mod m166 {
    use super::*;

    pub struct Accumulator166<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator166<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.583_f32 + y.sin();
        let b = y * 9.399_f32 - x.cos();
        let mut acc = Accumulator166::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.433_f32 + y.sin();
        let b = y * 0.692_f32 - x.cos();
        let mut acc = Accumulator166::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.28_f32 + y.sin();
        let b = y * 0.872_f32 - x.cos();
        let mut acc = Accumulator166::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.941_f32 + y.sin();
        let b = y * 4.699_f32 - x.cos();
        let mut acc = Accumulator166::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.579_f32 + y.sin();
        let b = y * 2.697_f32 - x.cos();
        let mut acc = Accumulator166::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.437_f32 + y.sin();
        let b = y * 3.607_f32 - x.cos();
        let mut acc = Accumulator166::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.432_f32 + y.sin();
        let b = y * 8.106_f32 - x.cos();
        let mut acc = Accumulator166::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.595_f32 + y.sin();
        let b = y * 7.063_f32 - x.cos();
        let mut acc = Accumulator166::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.603_f32 + y.sin();
        let b = y * 7.73_f32 - x.cos();
        let mut acc = Accumulator166::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.02_f32 + y.sin();
        let b = y * 7.071_f32 - x.cos();
        let mut acc = Accumulator166::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.073_f32 + y.sin();
        let b = y * 1.806_f32 - x.cos();
        let mut acc = Accumulator166::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.708_f32 + y.sin();
        let b = y * 3.332_f32 - x.cos();
        let mut acc = Accumulator166::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.147_f32 + y.sin();
        let b = y * 3.246_f32 - x.cos();
        let mut acc = Accumulator166::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.369_f32 + y.sin();
        let b = y * 1.475_f32 - x.cos();
        let mut acc = Accumulator166::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.357_f32 + y.sin();
        let b = y * 2.788_f32 - x.cos();
        let mut acc = Accumulator166::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.541_f32 + y.sin();
        let b = y * 1.581_f32 - x.cos();
        let mut acc = Accumulator166::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.984_f32 + y.sin();
        let b = y * 0.689_f32 - x.cos();
        let mut acc = Accumulator166::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.574_f32 + y.sin();
        let b = y * 9.056_f32 - x.cos();
        let mut acc = Accumulator166::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.986_f32 + y.sin();
        let b = y * 8.506_f32 - x.cos();
        let mut acc = Accumulator166::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.48_f32 + y.sin();
        let b = y * 9.353_f32 - x.cos();
        let mut acc = Accumulator166::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_166(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(166u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_166() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_166(total as u64) % 997) as f32;
        total
    }
}

pub mod m167 {
    use super::*;

    pub struct Accumulator167<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator167<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.534_f32 + y.sin();
        let b = y * 5.226_f32 - x.cos();
        let mut acc = Accumulator167::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.591_f32 + y.sin();
        let b = y * 4.748_f32 - x.cos();
        let mut acc = Accumulator167::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.454_f32 + y.sin();
        let b = y * 9.532_f32 - x.cos();
        let mut acc = Accumulator167::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.02_f32 + y.sin();
        let b = y * 5.528_f32 - x.cos();
        let mut acc = Accumulator167::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.376_f32 + y.sin();
        let b = y * 8.337_f32 - x.cos();
        let mut acc = Accumulator167::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.81_f32 + y.sin();
        let b = y * 1.506_f32 - x.cos();
        let mut acc = Accumulator167::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.241_f32 + y.sin();
        let b = y * 8.159_f32 - x.cos();
        let mut acc = Accumulator167::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.726_f32 + y.sin();
        let b = y * 4.171_f32 - x.cos();
        let mut acc = Accumulator167::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.502_f32 + y.sin();
        let b = y * 3.93_f32 - x.cos();
        let mut acc = Accumulator167::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.273_f32 + y.sin();
        let b = y * 6.306_f32 - x.cos();
        let mut acc = Accumulator167::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.384_f32 + y.sin();
        let b = y * 5.668_f32 - x.cos();
        let mut acc = Accumulator167::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.531_f32 + y.sin();
        let b = y * 5.705_f32 - x.cos();
        let mut acc = Accumulator167::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.313_f32 + y.sin();
        let b = y * 3.84_f32 - x.cos();
        let mut acc = Accumulator167::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.85_f32 + y.sin();
        let b = y * 0.248_f32 - x.cos();
        let mut acc = Accumulator167::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.256_f32 + y.sin();
        let b = y * 4.843_f32 - x.cos();
        let mut acc = Accumulator167::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.472_f32 + y.sin();
        let b = y * 3.965_f32 - x.cos();
        let mut acc = Accumulator167::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.493_f32 + y.sin();
        let b = y * 1.614_f32 - x.cos();
        let mut acc = Accumulator167::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.03_f32 + y.sin();
        let b = y * 7.259_f32 - x.cos();
        let mut acc = Accumulator167::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.314_f32 + y.sin();
        let b = y * 2.717_f32 - x.cos();
        let mut acc = Accumulator167::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.701_f32 + y.sin();
        let b = y * 0.986_f32 - x.cos();
        let mut acc = Accumulator167::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_167(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_167() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_167(total as u64) % 997) as f32;
        total
    }
}

pub mod m168 {
    use super::*;

    pub struct Accumulator168<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator168<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.853_f32 + y.sin();
        let b = y * 4.599_f32 - x.cos();
        let mut acc = Accumulator168::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.778_f32 + y.sin();
        let b = y * 6.843_f32 - x.cos();
        let mut acc = Accumulator168::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.552_f32 + y.sin();
        let b = y * 2.332_f32 - x.cos();
        let mut acc = Accumulator168::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.753_f32 + y.sin();
        let b = y * 5.635_f32 - x.cos();
        let mut acc = Accumulator168::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.228_f32 + y.sin();
        let b = y * 1.481_f32 - x.cos();
        let mut acc = Accumulator168::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.203_f32 + y.sin();
        let b = y * 0.919_f32 - x.cos();
        let mut acc = Accumulator168::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.027_f32 + y.sin();
        let b = y * 5.369_f32 - x.cos();
        let mut acc = Accumulator168::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.283_f32 + y.sin();
        let b = y * 3.546_f32 - x.cos();
        let mut acc = Accumulator168::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.354_f32 + y.sin();
        let b = y * 6.471_f32 - x.cos();
        let mut acc = Accumulator168::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.961_f32 + y.sin();
        let b = y * 1.712_f32 - x.cos();
        let mut acc = Accumulator168::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.556_f32 + y.sin();
        let b = y * 4.977_f32 - x.cos();
        let mut acc = Accumulator168::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.185_f32 + y.sin();
        let b = y * 4.634_f32 - x.cos();
        let mut acc = Accumulator168::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.246_f32 + y.sin();
        let b = y * 0.634_f32 - x.cos();
        let mut acc = Accumulator168::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.931_f32 + y.sin();
        let b = y * 6.179_f32 - x.cos();
        let mut acc = Accumulator168::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.976_f32 + y.sin();
        let b = y * 5.765_f32 - x.cos();
        let mut acc = Accumulator168::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.875_f32 + y.sin();
        let b = y * 1.644_f32 - x.cos();
        let mut acc = Accumulator168::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.365_f32 + y.sin();
        let b = y * 2.304_f32 - x.cos();
        let mut acc = Accumulator168::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.031_f32 + y.sin();
        let b = y * 8.332_f32 - x.cos();
        let mut acc = Accumulator168::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.709_f32 + y.sin();
        let b = y * 8.778_f32 - x.cos();
        let mut acc = Accumulator168::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.742_f32 + y.sin();
        let b = y * 0.287_f32 - x.cos();
        let mut acc = Accumulator168::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_168(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_168() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_168(total as u64) % 997) as f32;
        total
    }
}

pub mod m169 {
    use super::*;

    pub struct Accumulator169<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator169<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.137_f32 + y.sin();
        let b = y * 1.412_f32 - x.cos();
        let mut acc = Accumulator169::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.369_f32 + y.sin();
        let b = y * 4.638_f32 - x.cos();
        let mut acc = Accumulator169::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.577_f32 + y.sin();
        let b = y * 9.68_f32 - x.cos();
        let mut acc = Accumulator169::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.954_f32 + y.sin();
        let b = y * 9.278_f32 - x.cos();
        let mut acc = Accumulator169::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.083_f32 + y.sin();
        let b = y * 8.759_f32 - x.cos();
        let mut acc = Accumulator169::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.473_f32 + y.sin();
        let b = y * 1.728_f32 - x.cos();
        let mut acc = Accumulator169::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.508_f32 + y.sin();
        let b = y * 8.56_f32 - x.cos();
        let mut acc = Accumulator169::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.161_f32 + y.sin();
        let b = y * 0.324_f32 - x.cos();
        let mut acc = Accumulator169::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.042_f32 + y.sin();
        let b = y * 9.659_f32 - x.cos();
        let mut acc = Accumulator169::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.128_f32 + y.sin();
        let b = y * 7.283_f32 - x.cos();
        let mut acc = Accumulator169::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.181_f32 + y.sin();
        let b = y * 3.687_f32 - x.cos();
        let mut acc = Accumulator169::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.28_f32 + y.sin();
        let b = y * 8.025_f32 - x.cos();
        let mut acc = Accumulator169::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.677_f32 + y.sin();
        let b = y * 2.266_f32 - x.cos();
        let mut acc = Accumulator169::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.29_f32 + y.sin();
        let b = y * 4.435_f32 - x.cos();
        let mut acc = Accumulator169::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.01_f32 + y.sin();
        let b = y * 1.575_f32 - x.cos();
        let mut acc = Accumulator169::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.264_f32 + y.sin();
        let b = y * 1.031_f32 - x.cos();
        let mut acc = Accumulator169::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.236_f32 + y.sin();
        let b = y * 7.961_f32 - x.cos();
        let mut acc = Accumulator169::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.5_f32 + y.sin();
        let b = y * 7.006_f32 - x.cos();
        let mut acc = Accumulator169::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.108_f32 + y.sin();
        let b = y * 9.443_f32 - x.cos();
        let mut acc = Accumulator169::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.496_f32 + y.sin();
        let b = y * 1.868_f32 - x.cos();
        let mut acc = Accumulator169::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_169(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m169-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_169() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_169(total as u64) % 997) as f32;
        total
    }
}

pub mod m170 {
    use super::*;

    pub struct Accumulator170<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator170<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.263_f32 + y.sin();
        let b = y * 3.582_f32 - x.cos();
        let mut acc = Accumulator170::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.624_f32 + y.sin();
        let b = y * 0.465_f32 - x.cos();
        let mut acc = Accumulator170::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.233_f32 + y.sin();
        let b = y * 0.254_f32 - x.cos();
        let mut acc = Accumulator170::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.85_f32 + y.sin();
        let b = y * 0.186_f32 - x.cos();
        let mut acc = Accumulator170::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.425_f32 + y.sin();
        let b = y * 7.961_f32 - x.cos();
        let mut acc = Accumulator170::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.247_f32 + y.sin();
        let b = y * 7.56_f32 - x.cos();
        let mut acc = Accumulator170::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.494_f32 + y.sin();
        let b = y * 1.672_f32 - x.cos();
        let mut acc = Accumulator170::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.945_f32 + y.sin();
        let b = y * 4.76_f32 - x.cos();
        let mut acc = Accumulator170::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.757_f32 + y.sin();
        let b = y * 8.864_f32 - x.cos();
        let mut acc = Accumulator170::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.381_f32 + y.sin();
        let b = y * 7.708_f32 - x.cos();
        let mut acc = Accumulator170::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.773_f32 + y.sin();
        let b = y * 8.191_f32 - x.cos();
        let mut acc = Accumulator170::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.912_f32 + y.sin();
        let b = y * 9.314_f32 - x.cos();
        let mut acc = Accumulator170::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.836_f32 + y.sin();
        let b = y * 3.543_f32 - x.cos();
        let mut acc = Accumulator170::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.852_f32 + y.sin();
        let b = y * 5.691_f32 - x.cos();
        let mut acc = Accumulator170::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.445_f32 + y.sin();
        let b = y * 9.199_f32 - x.cos();
        let mut acc = Accumulator170::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.175_f32 + y.sin();
        let b = y * 7.886_f32 - x.cos();
        let mut acc = Accumulator170::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.877_f32 + y.sin();
        let b = y * 7.185_f32 - x.cos();
        let mut acc = Accumulator170::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.532_f32 + y.sin();
        let b = y * 4.497_f32 - x.cos();
        let mut acc = Accumulator170::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.256_f32 + y.sin();
        let b = y * 8.476_f32 - x.cos();
        let mut acc = Accumulator170::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.169_f32 + y.sin();
        let b = y * 2.232_f32 - x.cos();
        let mut acc = Accumulator170::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_170(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_170() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_170(total as u64) % 997) as f32;
        total
    }
}

pub mod m171 {
    use super::*;

    pub struct Accumulator171<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator171<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.631_f32 + y.sin();
        let b = y * 9.653_f32 - x.cos();
        let mut acc = Accumulator171::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.986_f32 + y.sin();
        let b = y * 4.303_f32 - x.cos();
        let mut acc = Accumulator171::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.244_f32 + y.sin();
        let b = y * 9.668_f32 - x.cos();
        let mut acc = Accumulator171::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.435_f32 + y.sin();
        let b = y * 0.506_f32 - x.cos();
        let mut acc = Accumulator171::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.954_f32 + y.sin();
        let b = y * 5.618_f32 - x.cos();
        let mut acc = Accumulator171::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.302_f32 + y.sin();
        let b = y * 5.968_f32 - x.cos();
        let mut acc = Accumulator171::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.722_f32 + y.sin();
        let b = y * 7.768_f32 - x.cos();
        let mut acc = Accumulator171::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.839_f32 + y.sin();
        let b = y * 7.391_f32 - x.cos();
        let mut acc = Accumulator171::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.847_f32 + y.sin();
        let b = y * 0.147_f32 - x.cos();
        let mut acc = Accumulator171::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.244_f32 + y.sin();
        let b = y * 6.432_f32 - x.cos();
        let mut acc = Accumulator171::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.882_f32 + y.sin();
        let b = y * 9.775_f32 - x.cos();
        let mut acc = Accumulator171::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.479_f32 + y.sin();
        let b = y * 9.31_f32 - x.cos();
        let mut acc = Accumulator171::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.655_f32 + y.sin();
        let b = y * 4.215_f32 - x.cos();
        let mut acc = Accumulator171::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.133_f32 + y.sin();
        let b = y * 1.607_f32 - x.cos();
        let mut acc = Accumulator171::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.409_f32 + y.sin();
        let b = y * 0.694_f32 - x.cos();
        let mut acc = Accumulator171::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.848_f32 + y.sin();
        let b = y * 5.583_f32 - x.cos();
        let mut acc = Accumulator171::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.545_f32 + y.sin();
        let b = y * 9.316_f32 - x.cos();
        let mut acc = Accumulator171::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.666_f32 + y.sin();
        let b = y * 3.843_f32 - x.cos();
        let mut acc = Accumulator171::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.82_f32 + y.sin();
        let b = y * 7.588_f32 - x.cos();
        let mut acc = Accumulator171::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.45_f32 + y.sin();
        let b = y * 6.393_f32 - x.cos();
        let mut acc = Accumulator171::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_171(seed: u64) -> u64 {
        let re = Regex::new(r"m171-(\d+)").unwrap();
        let hay = format!("m171-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_171() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_171(total as u64) % 997) as f32;
        total
    }
}

pub mod m172 {
    use super::*;

    pub struct Accumulator172<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator172<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.274_f32 + y.sin();
        let b = y * 6.533_f32 - x.cos();
        let mut acc = Accumulator172::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.522_f32 + y.sin();
        let b = y * 6.998_f32 - x.cos();
        let mut acc = Accumulator172::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.91_f32 + y.sin();
        let b = y * 5.409_f32 - x.cos();
        let mut acc = Accumulator172::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.163_f32 + y.sin();
        let b = y * 5.997_f32 - x.cos();
        let mut acc = Accumulator172::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.337_f32 + y.sin();
        let b = y * 1.963_f32 - x.cos();
        let mut acc = Accumulator172::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.78_f32 + y.sin();
        let b = y * 0.41_f32 - x.cos();
        let mut acc = Accumulator172::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.505_f32 + y.sin();
        let b = y * 5.784_f32 - x.cos();
        let mut acc = Accumulator172::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.631_f32 + y.sin();
        let b = y * 6.713_f32 - x.cos();
        let mut acc = Accumulator172::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.186_f32 + y.sin();
        let b = y * 9.592_f32 - x.cos();
        let mut acc = Accumulator172::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.721_f32 + y.sin();
        let b = y * 3.553_f32 - x.cos();
        let mut acc = Accumulator172::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.875_f32 + y.sin();
        let b = y * 5.945_f32 - x.cos();
        let mut acc = Accumulator172::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.184_f32 + y.sin();
        let b = y * 1.661_f32 - x.cos();
        let mut acc = Accumulator172::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.826_f32 + y.sin();
        let b = y * 8.369_f32 - x.cos();
        let mut acc = Accumulator172::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.375_f32 + y.sin();
        let b = y * 3.144_f32 - x.cos();
        let mut acc = Accumulator172::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.145_f32 + y.sin();
        let b = y * 7.886_f32 - x.cos();
        let mut acc = Accumulator172::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.724_f32 + y.sin();
        let b = y * 5.284_f32 - x.cos();
        let mut acc = Accumulator172::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.785_f32 + y.sin();
        let b = y * 2.932_f32 - x.cos();
        let mut acc = Accumulator172::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.837_f32 + y.sin();
        let b = y * 3.202_f32 - x.cos();
        let mut acc = Accumulator172::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.343_f32 + y.sin();
        let b = y * 2.954_f32 - x.cos();
        let mut acc = Accumulator172::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.781_f32 + y.sin();
        let b = y * 7.193_f32 - x.cos();
        let mut acc = Accumulator172::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_172(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_172() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_172(total as u64) % 997) as f32;
        total
    }
}

pub mod m173 {
    use super::*;

    pub struct Accumulator173<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator173<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.388_f32 + y.sin();
        let b = y * 3.238_f32 - x.cos();
        let mut acc = Accumulator173::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.742_f32 + y.sin();
        let b = y * 9.172_f32 - x.cos();
        let mut acc = Accumulator173::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.064_f32 + y.sin();
        let b = y * 4.044_f32 - x.cos();
        let mut acc = Accumulator173::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.646_f32 + y.sin();
        let b = y * 0.96_f32 - x.cos();
        let mut acc = Accumulator173::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.701_f32 + y.sin();
        let b = y * 0.439_f32 - x.cos();
        let mut acc = Accumulator173::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.615_f32 + y.sin();
        let b = y * 4.543_f32 - x.cos();
        let mut acc = Accumulator173::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.107_f32 + y.sin();
        let b = y * 6.528_f32 - x.cos();
        let mut acc = Accumulator173::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.807_f32 + y.sin();
        let b = y * 9.201_f32 - x.cos();
        let mut acc = Accumulator173::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.84_f32 + y.sin();
        let b = y * 5.62_f32 - x.cos();
        let mut acc = Accumulator173::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.547_f32 + y.sin();
        let b = y * 3.477_f32 - x.cos();
        let mut acc = Accumulator173::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.626_f32 + y.sin();
        let b = y * 8.934_f32 - x.cos();
        let mut acc = Accumulator173::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.577_f32 + y.sin();
        let b = y * 7.838_f32 - x.cos();
        let mut acc = Accumulator173::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.463_f32 + y.sin();
        let b = y * 2.262_f32 - x.cos();
        let mut acc = Accumulator173::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.766_f32 + y.sin();
        let b = y * 7.678_f32 - x.cos();
        let mut acc = Accumulator173::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.205_f32 + y.sin();
        let b = y * 6.403_f32 - x.cos();
        let mut acc = Accumulator173::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.435_f32 + y.sin();
        let b = y * 6.742_f32 - x.cos();
        let mut acc = Accumulator173::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.837_f32 + y.sin();
        let b = y * 5.59_f32 - x.cos();
        let mut acc = Accumulator173::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.473_f32 + y.sin();
        let b = y * 8.394_f32 - x.cos();
        let mut acc = Accumulator173::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.96_f32 + y.sin();
        let b = y * 4.581_f32 - x.cos();
        let mut acc = Accumulator173::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.232_f32 + y.sin();
        let b = y * 6.227_f32 - x.cos();
        let mut acc = Accumulator173::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_173(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(173u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_173() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_173(total as u64) % 997) as f32;
        total
    }
}

pub mod m174 {
    use super::*;

    pub struct Accumulator174<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator174<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.995_f32 + y.sin();
        let b = y * 3.965_f32 - x.cos();
        let mut acc = Accumulator174::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.596_f32 + y.sin();
        let b = y * 6.915_f32 - x.cos();
        let mut acc = Accumulator174::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.876_f32 + y.sin();
        let b = y * 0.61_f32 - x.cos();
        let mut acc = Accumulator174::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.319_f32 + y.sin();
        let b = y * 4.002_f32 - x.cos();
        let mut acc = Accumulator174::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.989_f32 + y.sin();
        let b = y * 8.853_f32 - x.cos();
        let mut acc = Accumulator174::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.559_f32 + y.sin();
        let b = y * 5.958_f32 - x.cos();
        let mut acc = Accumulator174::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.607_f32 + y.sin();
        let b = y * 9.411_f32 - x.cos();
        let mut acc = Accumulator174::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.936_f32 + y.sin();
        let b = y * 3.641_f32 - x.cos();
        let mut acc = Accumulator174::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.392_f32 + y.sin();
        let b = y * 8.708_f32 - x.cos();
        let mut acc = Accumulator174::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.252_f32 + y.sin();
        let b = y * 1.071_f32 - x.cos();
        let mut acc = Accumulator174::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.462_f32 + y.sin();
        let b = y * 7.466_f32 - x.cos();
        let mut acc = Accumulator174::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.132_f32 + y.sin();
        let b = y * 9.726_f32 - x.cos();
        let mut acc = Accumulator174::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.249_f32 + y.sin();
        let b = y * 5.358_f32 - x.cos();
        let mut acc = Accumulator174::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.962_f32 + y.sin();
        let b = y * 8.645_f32 - x.cos();
        let mut acc = Accumulator174::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.096_f32 + y.sin();
        let b = y * 6.538_f32 - x.cos();
        let mut acc = Accumulator174::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.048_f32 + y.sin();
        let b = y * 5.426_f32 - x.cos();
        let mut acc = Accumulator174::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.446_f32 + y.sin();
        let b = y * 2.355_f32 - x.cos();
        let mut acc = Accumulator174::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.491_f32 + y.sin();
        let b = y * 1.658_f32 - x.cos();
        let mut acc = Accumulator174::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.553_f32 + y.sin();
        let b = y * 7.356_f32 - x.cos();
        let mut acc = Accumulator174::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.2_f32 + y.sin();
        let b = y * 7.118_f32 - x.cos();
        let mut acc = Accumulator174::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_174(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_174() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_174(total as u64) % 997) as f32;
        total
    }
}

pub mod m175 {
    use super::*;

    pub struct Accumulator175<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator175<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.648_f32 + y.sin();
        let b = y * 3.365_f32 - x.cos();
        let mut acc = Accumulator175::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.826_f32 + y.sin();
        let b = y * 3.848_f32 - x.cos();
        let mut acc = Accumulator175::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.377_f32 + y.sin();
        let b = y * 1.436_f32 - x.cos();
        let mut acc = Accumulator175::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.667_f32 + y.sin();
        let b = y * 3.193_f32 - x.cos();
        let mut acc = Accumulator175::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.653_f32 + y.sin();
        let b = y * 0.22_f32 - x.cos();
        let mut acc = Accumulator175::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.5_f32 + y.sin();
        let b = y * 3.681_f32 - x.cos();
        let mut acc = Accumulator175::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.146_f32 + y.sin();
        let b = y * 8.396_f32 - x.cos();
        let mut acc = Accumulator175::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.889_f32 + y.sin();
        let b = y * 8.008_f32 - x.cos();
        let mut acc = Accumulator175::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.303_f32 + y.sin();
        let b = y * 8.684_f32 - x.cos();
        let mut acc = Accumulator175::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.205_f32 + y.sin();
        let b = y * 9.161_f32 - x.cos();
        let mut acc = Accumulator175::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.475_f32 + y.sin();
        let b = y * 8.3_f32 - x.cos();
        let mut acc = Accumulator175::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.833_f32 + y.sin();
        let b = y * 8.833_f32 - x.cos();
        let mut acc = Accumulator175::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.981_f32 + y.sin();
        let b = y * 5.526_f32 - x.cos();
        let mut acc = Accumulator175::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.891_f32 + y.sin();
        let b = y * 8.421_f32 - x.cos();
        let mut acc = Accumulator175::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.425_f32 + y.sin();
        let b = y * 4.895_f32 - x.cos();
        let mut acc = Accumulator175::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.822_f32 + y.sin();
        let b = y * 2.601_f32 - x.cos();
        let mut acc = Accumulator175::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.602_f32 + y.sin();
        let b = y * 5.135_f32 - x.cos();
        let mut acc = Accumulator175::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.129_f32 + y.sin();
        let b = y * 3.554_f32 - x.cos();
        let mut acc = Accumulator175::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.872_f32 + y.sin();
        let b = y * 4.352_f32 - x.cos();
        let mut acc = Accumulator175::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.389_f32 + y.sin();
        let b = y * 2.879_f32 - x.cos();
        let mut acc = Accumulator175::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_175(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_175() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_175(total as u64) % 997) as f32;
        total
    }
}

pub mod m176 {
    use super::*;

    pub struct Accumulator176<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator176<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.223_f32 + y.sin();
        let b = y * 9.634_f32 - x.cos();
        let mut acc = Accumulator176::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.942_f32 + y.sin();
        let b = y * 6.269_f32 - x.cos();
        let mut acc = Accumulator176::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.836_f32 + y.sin();
        let b = y * 7.823_f32 - x.cos();
        let mut acc = Accumulator176::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.628_f32 + y.sin();
        let b = y * 3.666_f32 - x.cos();
        let mut acc = Accumulator176::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.257_f32 + y.sin();
        let b = y * 3.255_f32 - x.cos();
        let mut acc = Accumulator176::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.349_f32 + y.sin();
        let b = y * 9.219_f32 - x.cos();
        let mut acc = Accumulator176::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.542_f32 + y.sin();
        let b = y * 1.242_f32 - x.cos();
        let mut acc = Accumulator176::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.52_f32 + y.sin();
        let b = y * 0.328_f32 - x.cos();
        let mut acc = Accumulator176::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.993_f32 + y.sin();
        let b = y * 1.82_f32 - x.cos();
        let mut acc = Accumulator176::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.004_f32 + y.sin();
        let b = y * 3.098_f32 - x.cos();
        let mut acc = Accumulator176::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.184_f32 + y.sin();
        let b = y * 3.836_f32 - x.cos();
        let mut acc = Accumulator176::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.856_f32 + y.sin();
        let b = y * 7.293_f32 - x.cos();
        let mut acc = Accumulator176::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.304_f32 + y.sin();
        let b = y * 9.057_f32 - x.cos();
        let mut acc = Accumulator176::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.551_f32 + y.sin();
        let b = y * 3.493_f32 - x.cos();
        let mut acc = Accumulator176::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.281_f32 + y.sin();
        let b = y * 0.329_f32 - x.cos();
        let mut acc = Accumulator176::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.039_f32 + y.sin();
        let b = y * 0.482_f32 - x.cos();
        let mut acc = Accumulator176::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.971_f32 + y.sin();
        let b = y * 8.059_f32 - x.cos();
        let mut acc = Accumulator176::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.157_f32 + y.sin();
        let b = y * 2.825_f32 - x.cos();
        let mut acc = Accumulator176::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.371_f32 + y.sin();
        let b = y * 6.631_f32 - x.cos();
        let mut acc = Accumulator176::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.182_f32 + y.sin();
        let b = y * 8.55_f32 - x.cos();
        let mut acc = Accumulator176::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_176(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m176-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_176() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_176(total as u64) % 997) as f32;
        total
    }
}

pub mod m177 {
    use super::*;

    pub struct Accumulator177<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator177<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.099_f32 + y.sin();
        let b = y * 9.284_f32 - x.cos();
        let mut acc = Accumulator177::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.097_f32 + y.sin();
        let b = y * 5.928_f32 - x.cos();
        let mut acc = Accumulator177::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.313_f32 + y.sin();
        let b = y * 3.078_f32 - x.cos();
        let mut acc = Accumulator177::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.785_f32 + y.sin();
        let b = y * 9.018_f32 - x.cos();
        let mut acc = Accumulator177::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.76_f32 + y.sin();
        let b = y * 7.445_f32 - x.cos();
        let mut acc = Accumulator177::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.553_f32 + y.sin();
        let b = y * 9.137_f32 - x.cos();
        let mut acc = Accumulator177::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.477_f32 + y.sin();
        let b = y * 1.246_f32 - x.cos();
        let mut acc = Accumulator177::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.607_f32 + y.sin();
        let b = y * 1.658_f32 - x.cos();
        let mut acc = Accumulator177::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.163_f32 + y.sin();
        let b = y * 8.339_f32 - x.cos();
        let mut acc = Accumulator177::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.858_f32 + y.sin();
        let b = y * 5.055_f32 - x.cos();
        let mut acc = Accumulator177::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.759_f32 + y.sin();
        let b = y * 9.174_f32 - x.cos();
        let mut acc = Accumulator177::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.788_f32 + y.sin();
        let b = y * 5.635_f32 - x.cos();
        let mut acc = Accumulator177::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.298_f32 + y.sin();
        let b = y * 7.134_f32 - x.cos();
        let mut acc = Accumulator177::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.025_f32 + y.sin();
        let b = y * 7.484_f32 - x.cos();
        let mut acc = Accumulator177::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.193_f32 + y.sin();
        let b = y * 1.47_f32 - x.cos();
        let mut acc = Accumulator177::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.537_f32 + y.sin();
        let b = y * 9.725_f32 - x.cos();
        let mut acc = Accumulator177::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.737_f32 + y.sin();
        let b = y * 8.18_f32 - x.cos();
        let mut acc = Accumulator177::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.922_f32 + y.sin();
        let b = y * 0.92_f32 - x.cos();
        let mut acc = Accumulator177::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.109_f32 + y.sin();
        let b = y * 1.366_f32 - x.cos();
        let mut acc = Accumulator177::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.732_f32 + y.sin();
        let b = y * 4.571_f32 - x.cos();
        let mut acc = Accumulator177::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_177(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_177() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_177(total as u64) % 997) as f32;
        total
    }
}

pub mod m178 {
    use super::*;

    pub struct Accumulator178<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator178<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.483_f32 + y.sin();
        let b = y * 5.432_f32 - x.cos();
        let mut acc = Accumulator178::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.221_f32 + y.sin();
        let b = y * 2.188_f32 - x.cos();
        let mut acc = Accumulator178::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.15_f32 + y.sin();
        let b = y * 6.286_f32 - x.cos();
        let mut acc = Accumulator178::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.019_f32 + y.sin();
        let b = y * 7.572_f32 - x.cos();
        let mut acc = Accumulator178::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.482_f32 + y.sin();
        let b = y * 6.134_f32 - x.cos();
        let mut acc = Accumulator178::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.174_f32 + y.sin();
        let b = y * 3.984_f32 - x.cos();
        let mut acc = Accumulator178::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.763_f32 + y.sin();
        let b = y * 8.038_f32 - x.cos();
        let mut acc = Accumulator178::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.716_f32 + y.sin();
        let b = y * 0.62_f32 - x.cos();
        let mut acc = Accumulator178::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.248_f32 + y.sin();
        let b = y * 8.941_f32 - x.cos();
        let mut acc = Accumulator178::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.216_f32 + y.sin();
        let b = y * 6.171_f32 - x.cos();
        let mut acc = Accumulator178::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.09_f32 + y.sin();
        let b = y * 4.047_f32 - x.cos();
        let mut acc = Accumulator178::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.613_f32 + y.sin();
        let b = y * 0.896_f32 - x.cos();
        let mut acc = Accumulator178::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.545_f32 + y.sin();
        let b = y * 2.031_f32 - x.cos();
        let mut acc = Accumulator178::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.492_f32 + y.sin();
        let b = y * 2.642_f32 - x.cos();
        let mut acc = Accumulator178::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.206_f32 + y.sin();
        let b = y * 7.476_f32 - x.cos();
        let mut acc = Accumulator178::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.008_f32 + y.sin();
        let b = y * 3.252_f32 - x.cos();
        let mut acc = Accumulator178::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.66_f32 + y.sin();
        let b = y * 0.525_f32 - x.cos();
        let mut acc = Accumulator178::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.972_f32 + y.sin();
        let b = y * 4.88_f32 - x.cos();
        let mut acc = Accumulator178::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.912_f32 + y.sin();
        let b = y * 0.632_f32 - x.cos();
        let mut acc = Accumulator178::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.538_f32 + y.sin();
        let b = y * 2.803_f32 - x.cos();
        let mut acc = Accumulator178::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_178(seed: u64) -> u64 {
        let re = Regex::new(r"m178-(\d+)").unwrap();
        let hay = format!("m178-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_178() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_178(total as u64) % 997) as f32;
        total
    }
}

pub mod m179 {
    use super::*;

    pub struct Accumulator179<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator179<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.54_f32 + y.sin();
        let b = y * 5.942_f32 - x.cos();
        let mut acc = Accumulator179::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.232_f32 + y.sin();
        let b = y * 0.817_f32 - x.cos();
        let mut acc = Accumulator179::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.331_f32 + y.sin();
        let b = y * 7.766_f32 - x.cos();
        let mut acc = Accumulator179::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.021_f32 + y.sin();
        let b = y * 9.104_f32 - x.cos();
        let mut acc = Accumulator179::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.81_f32 + y.sin();
        let b = y * 3.519_f32 - x.cos();
        let mut acc = Accumulator179::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.109_f32 + y.sin();
        let b = y * 7.025_f32 - x.cos();
        let mut acc = Accumulator179::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.873_f32 + y.sin();
        let b = y * 4.621_f32 - x.cos();
        let mut acc = Accumulator179::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.129_f32 + y.sin();
        let b = y * 9.748_f32 - x.cos();
        let mut acc = Accumulator179::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.275_f32 + y.sin();
        let b = y * 4.226_f32 - x.cos();
        let mut acc = Accumulator179::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.102_f32 + y.sin();
        let b = y * 4.175_f32 - x.cos();
        let mut acc = Accumulator179::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.575_f32 + y.sin();
        let b = y * 9.015_f32 - x.cos();
        let mut acc = Accumulator179::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.624_f32 + y.sin();
        let b = y * 7.408_f32 - x.cos();
        let mut acc = Accumulator179::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.496_f32 + y.sin();
        let b = y * 6.858_f32 - x.cos();
        let mut acc = Accumulator179::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.308_f32 + y.sin();
        let b = y * 7.633_f32 - x.cos();
        let mut acc = Accumulator179::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.469_f32 + y.sin();
        let b = y * 5.807_f32 - x.cos();
        let mut acc = Accumulator179::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.767_f32 + y.sin();
        let b = y * 7.763_f32 - x.cos();
        let mut acc = Accumulator179::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.641_f32 + y.sin();
        let b = y * 3.331_f32 - x.cos();
        let mut acc = Accumulator179::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.99_f32 + y.sin();
        let b = y * 4.18_f32 - x.cos();
        let mut acc = Accumulator179::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.224_f32 + y.sin();
        let b = y * 5.204_f32 - x.cos();
        let mut acc = Accumulator179::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.858_f32 + y.sin();
        let b = y * 1.828_f32 - x.cos();
        let mut acc = Accumulator179::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_179(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_179() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_179(total as u64) % 997) as f32;
        total
    }
}

pub mod m180 {
    use super::*;

    pub struct Accumulator180<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator180<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.566_f32 + y.sin();
        let b = y * 5.684_f32 - x.cos();
        let mut acc = Accumulator180::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.86_f32 + y.sin();
        let b = y * 6.387_f32 - x.cos();
        let mut acc = Accumulator180::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.449_f32 + y.sin();
        let b = y * 6.797_f32 - x.cos();
        let mut acc = Accumulator180::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.741_f32 + y.sin();
        let b = y * 9.322_f32 - x.cos();
        let mut acc = Accumulator180::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.53_f32 + y.sin();
        let b = y * 3.615_f32 - x.cos();
        let mut acc = Accumulator180::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.941_f32 + y.sin();
        let b = y * 3.8_f32 - x.cos();
        let mut acc = Accumulator180::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.847_f32 + y.sin();
        let b = y * 8.857_f32 - x.cos();
        let mut acc = Accumulator180::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.556_f32 + y.sin();
        let b = y * 9.565_f32 - x.cos();
        let mut acc = Accumulator180::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.554_f32 + y.sin();
        let b = y * 4.875_f32 - x.cos();
        let mut acc = Accumulator180::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.824_f32 + y.sin();
        let b = y * 3.317_f32 - x.cos();
        let mut acc = Accumulator180::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.372_f32 + y.sin();
        let b = y * 3.133_f32 - x.cos();
        let mut acc = Accumulator180::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.833_f32 + y.sin();
        let b = y * 6.67_f32 - x.cos();
        let mut acc = Accumulator180::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.198_f32 + y.sin();
        let b = y * 6.704_f32 - x.cos();
        let mut acc = Accumulator180::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.077_f32 + y.sin();
        let b = y * 7.114_f32 - x.cos();
        let mut acc = Accumulator180::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.068_f32 + y.sin();
        let b = y * 8.208_f32 - x.cos();
        let mut acc = Accumulator180::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.952_f32 + y.sin();
        let b = y * 0.284_f32 - x.cos();
        let mut acc = Accumulator180::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.995_f32 + y.sin();
        let b = y * 8.078_f32 - x.cos();
        let mut acc = Accumulator180::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.888_f32 + y.sin();
        let b = y * 8.114_f32 - x.cos();
        let mut acc = Accumulator180::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.3_f32 + y.sin();
        let b = y * 5.978_f32 - x.cos();
        let mut acc = Accumulator180::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.576_f32 + y.sin();
        let b = y * 5.733_f32 - x.cos();
        let mut acc = Accumulator180::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_180(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(180u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_180() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_180(total as u64) % 997) as f32;
        total
    }
}

pub mod m181 {
    use super::*;

    pub struct Accumulator181<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator181<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.734_f32 + y.sin();
        let b = y * 7.45_f32 - x.cos();
        let mut acc = Accumulator181::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.68_f32 + y.sin();
        let b = y * 1.793_f32 - x.cos();
        let mut acc = Accumulator181::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.044_f32 + y.sin();
        let b = y * 2.892_f32 - x.cos();
        let mut acc = Accumulator181::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.388_f32 + y.sin();
        let b = y * 7.079_f32 - x.cos();
        let mut acc = Accumulator181::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.678_f32 + y.sin();
        let b = y * 7.037_f32 - x.cos();
        let mut acc = Accumulator181::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.648_f32 + y.sin();
        let b = y * 3.52_f32 - x.cos();
        let mut acc = Accumulator181::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.566_f32 + y.sin();
        let b = y * 8.421_f32 - x.cos();
        let mut acc = Accumulator181::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.584_f32 + y.sin();
        let b = y * 3.671_f32 - x.cos();
        let mut acc = Accumulator181::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.008_f32 + y.sin();
        let b = y * 1.628_f32 - x.cos();
        let mut acc = Accumulator181::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.982_f32 + y.sin();
        let b = y * 1.031_f32 - x.cos();
        let mut acc = Accumulator181::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.468_f32 + y.sin();
        let b = y * 2.004_f32 - x.cos();
        let mut acc = Accumulator181::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.077_f32 + y.sin();
        let b = y * 7.409_f32 - x.cos();
        let mut acc = Accumulator181::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.189_f32 + y.sin();
        let b = y * 0.826_f32 - x.cos();
        let mut acc = Accumulator181::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.543_f32 + y.sin();
        let b = y * 3.524_f32 - x.cos();
        let mut acc = Accumulator181::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.274_f32 + y.sin();
        let b = y * 6.863_f32 - x.cos();
        let mut acc = Accumulator181::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.435_f32 + y.sin();
        let b = y * 3.079_f32 - x.cos();
        let mut acc = Accumulator181::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.463_f32 + y.sin();
        let b = y * 6.874_f32 - x.cos();
        let mut acc = Accumulator181::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.673_f32 + y.sin();
        let b = y * 7.818_f32 - x.cos();
        let mut acc = Accumulator181::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.192_f32 + y.sin();
        let b = y * 7.303_f32 - x.cos();
        let mut acc = Accumulator181::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.895_f32 + y.sin();
        let b = y * 3.014_f32 - x.cos();
        let mut acc = Accumulator181::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_181(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_181() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_181(total as u64) % 997) as f32;
        total
    }
}

pub mod m182 {
    use super::*;

    pub struct Accumulator182<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator182<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.221_f32 + y.sin();
        let b = y * 5.245_f32 - x.cos();
        let mut acc = Accumulator182::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.081_f32 + y.sin();
        let b = y * 9.046_f32 - x.cos();
        let mut acc = Accumulator182::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.77_f32 + y.sin();
        let b = y * 3.462_f32 - x.cos();
        let mut acc = Accumulator182::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.613_f32 + y.sin();
        let b = y * 7.136_f32 - x.cos();
        let mut acc = Accumulator182::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.444_f32 + y.sin();
        let b = y * 8.513_f32 - x.cos();
        let mut acc = Accumulator182::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.335_f32 + y.sin();
        let b = y * 4.066_f32 - x.cos();
        let mut acc = Accumulator182::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.741_f32 + y.sin();
        let b = y * 8.008_f32 - x.cos();
        let mut acc = Accumulator182::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.051_f32 + y.sin();
        let b = y * 5.589_f32 - x.cos();
        let mut acc = Accumulator182::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.029_f32 + y.sin();
        let b = y * 5.329_f32 - x.cos();
        let mut acc = Accumulator182::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.49_f32 + y.sin();
        let b = y * 1.605_f32 - x.cos();
        let mut acc = Accumulator182::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.787_f32 + y.sin();
        let b = y * 2.757_f32 - x.cos();
        let mut acc = Accumulator182::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.415_f32 + y.sin();
        let b = y * 8.402_f32 - x.cos();
        let mut acc = Accumulator182::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.778_f32 + y.sin();
        let b = y * 6.638_f32 - x.cos();
        let mut acc = Accumulator182::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.567_f32 + y.sin();
        let b = y * 0.225_f32 - x.cos();
        let mut acc = Accumulator182::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.142_f32 + y.sin();
        let b = y * 2.286_f32 - x.cos();
        let mut acc = Accumulator182::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.138_f32 + y.sin();
        let b = y * 7.376_f32 - x.cos();
        let mut acc = Accumulator182::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.091_f32 + y.sin();
        let b = y * 5.727_f32 - x.cos();
        let mut acc = Accumulator182::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.105_f32 + y.sin();
        let b = y * 5.404_f32 - x.cos();
        let mut acc = Accumulator182::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.46_f32 + y.sin();
        let b = y * 8.793_f32 - x.cos();
        let mut acc = Accumulator182::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.802_f32 + y.sin();
        let b = y * 0.776_f32 - x.cos();
        let mut acc = Accumulator182::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_182(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_182() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_182(total as u64) % 997) as f32;
        total
    }
}

pub mod m183 {
    use super::*;

    pub struct Accumulator183<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator183<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.249_f32 + y.sin();
        let b = y * 2.873_f32 - x.cos();
        let mut acc = Accumulator183::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.929_f32 + y.sin();
        let b = y * 6.361_f32 - x.cos();
        let mut acc = Accumulator183::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.21_f32 + y.sin();
        let b = y * 1.351_f32 - x.cos();
        let mut acc = Accumulator183::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.205_f32 + y.sin();
        let b = y * 4.188_f32 - x.cos();
        let mut acc = Accumulator183::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.671_f32 + y.sin();
        let b = y * 5.295_f32 - x.cos();
        let mut acc = Accumulator183::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.746_f32 + y.sin();
        let b = y * 4.117_f32 - x.cos();
        let mut acc = Accumulator183::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.473_f32 + y.sin();
        let b = y * 9.162_f32 - x.cos();
        let mut acc = Accumulator183::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.62_f32 + y.sin();
        let b = y * 4.443_f32 - x.cos();
        let mut acc = Accumulator183::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.672_f32 + y.sin();
        let b = y * 3.682_f32 - x.cos();
        let mut acc = Accumulator183::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.699_f32 + y.sin();
        let b = y * 6.764_f32 - x.cos();
        let mut acc = Accumulator183::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.306_f32 + y.sin();
        let b = y * 4.465_f32 - x.cos();
        let mut acc = Accumulator183::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.087_f32 + y.sin();
        let b = y * 1.717_f32 - x.cos();
        let mut acc = Accumulator183::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.443_f32 + y.sin();
        let b = y * 5.5_f32 - x.cos();
        let mut acc = Accumulator183::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.282_f32 + y.sin();
        let b = y * 2.282_f32 - x.cos();
        let mut acc = Accumulator183::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.16_f32 + y.sin();
        let b = y * 1.471_f32 - x.cos();
        let mut acc = Accumulator183::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.374_f32 + y.sin();
        let b = y * 6.454_f32 - x.cos();
        let mut acc = Accumulator183::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.355_f32 + y.sin();
        let b = y * 2.385_f32 - x.cos();
        let mut acc = Accumulator183::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.24_f32 + y.sin();
        let b = y * 9.82_f32 - x.cos();
        let mut acc = Accumulator183::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.294_f32 + y.sin();
        let b = y * 7.687_f32 - x.cos();
        let mut acc = Accumulator183::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.87_f32 + y.sin();
        let b = y * 9.463_f32 - x.cos();
        let mut acc = Accumulator183::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_183(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m183-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_183() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_183(total as u64) % 997) as f32;
        total
    }
}

pub mod m184 {
    use super::*;

    pub struct Accumulator184<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator184<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.679_f32 + y.sin();
        let b = y * 4.361_f32 - x.cos();
        let mut acc = Accumulator184::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.027_f32 + y.sin();
        let b = y * 3.712_f32 - x.cos();
        let mut acc = Accumulator184::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.242_f32 + y.sin();
        let b = y * 5.928_f32 - x.cos();
        let mut acc = Accumulator184::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.252_f32 + y.sin();
        let b = y * 1.407_f32 - x.cos();
        let mut acc = Accumulator184::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.022_f32 + y.sin();
        let b = y * 2.556_f32 - x.cos();
        let mut acc = Accumulator184::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.15_f32 + y.sin();
        let b = y * 3.006_f32 - x.cos();
        let mut acc = Accumulator184::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.895_f32 + y.sin();
        let b = y * 4.124_f32 - x.cos();
        let mut acc = Accumulator184::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.351_f32 + y.sin();
        let b = y * 4.462_f32 - x.cos();
        let mut acc = Accumulator184::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.691_f32 + y.sin();
        let b = y * 4.426_f32 - x.cos();
        let mut acc = Accumulator184::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.192_f32 + y.sin();
        let b = y * 7.123_f32 - x.cos();
        let mut acc = Accumulator184::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.878_f32 + y.sin();
        let b = y * 0.564_f32 - x.cos();
        let mut acc = Accumulator184::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.99_f32 + y.sin();
        let b = y * 0.68_f32 - x.cos();
        let mut acc = Accumulator184::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.279_f32 + y.sin();
        let b = y * 6.428_f32 - x.cos();
        let mut acc = Accumulator184::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.435_f32 + y.sin();
        let b = y * 4.329_f32 - x.cos();
        let mut acc = Accumulator184::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.387_f32 + y.sin();
        let b = y * 4.367_f32 - x.cos();
        let mut acc = Accumulator184::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.19_f32 + y.sin();
        let b = y * 0.397_f32 - x.cos();
        let mut acc = Accumulator184::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.799_f32 + y.sin();
        let b = y * 7.399_f32 - x.cos();
        let mut acc = Accumulator184::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.903_f32 + y.sin();
        let b = y * 0.345_f32 - x.cos();
        let mut acc = Accumulator184::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.627_f32 + y.sin();
        let b = y * 4.643_f32 - x.cos();
        let mut acc = Accumulator184::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.165_f32 + y.sin();
        let b = y * 1.149_f32 - x.cos();
        let mut acc = Accumulator184::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_184(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_184() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_184(total as u64) % 997) as f32;
        total
    }
}

pub mod m185 {
    use super::*;

    pub struct Accumulator185<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator185<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.329_f32 + y.sin();
        let b = y * 3.548_f32 - x.cos();
        let mut acc = Accumulator185::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.676_f32 + y.sin();
        let b = y * 5.957_f32 - x.cos();
        let mut acc = Accumulator185::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.918_f32 + y.sin();
        let b = y * 4.682_f32 - x.cos();
        let mut acc = Accumulator185::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.202_f32 + y.sin();
        let b = y * 4.069_f32 - x.cos();
        let mut acc = Accumulator185::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.385_f32 + y.sin();
        let b = y * 5.053_f32 - x.cos();
        let mut acc = Accumulator185::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.115_f32 + y.sin();
        let b = y * 4.3_f32 - x.cos();
        let mut acc = Accumulator185::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.266_f32 + y.sin();
        let b = y * 7.0_f32 - x.cos();
        let mut acc = Accumulator185::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.732_f32 + y.sin();
        let b = y * 0.516_f32 - x.cos();
        let mut acc = Accumulator185::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.184_f32 + y.sin();
        let b = y * 1.197_f32 - x.cos();
        let mut acc = Accumulator185::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.43_f32 + y.sin();
        let b = y * 8.652_f32 - x.cos();
        let mut acc = Accumulator185::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.524_f32 + y.sin();
        let b = y * 5.423_f32 - x.cos();
        let mut acc = Accumulator185::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.962_f32 + y.sin();
        let b = y * 6.943_f32 - x.cos();
        let mut acc = Accumulator185::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.193_f32 + y.sin();
        let b = y * 7.887_f32 - x.cos();
        let mut acc = Accumulator185::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.676_f32 + y.sin();
        let b = y * 7.108_f32 - x.cos();
        let mut acc = Accumulator185::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.273_f32 + y.sin();
        let b = y * 3.646_f32 - x.cos();
        let mut acc = Accumulator185::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.797_f32 + y.sin();
        let b = y * 1.521_f32 - x.cos();
        let mut acc = Accumulator185::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.759_f32 + y.sin();
        let b = y * 4.398_f32 - x.cos();
        let mut acc = Accumulator185::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.411_f32 + y.sin();
        let b = y * 2.062_f32 - x.cos();
        let mut acc = Accumulator185::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.911_f32 + y.sin();
        let b = y * 7.559_f32 - x.cos();
        let mut acc = Accumulator185::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.416_f32 + y.sin();
        let b = y * 0.562_f32 - x.cos();
        let mut acc = Accumulator185::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_185(seed: u64) -> u64 {
        let re = Regex::new(r"m185-(\d+)").unwrap();
        let hay = format!("m185-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_185() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_185(total as u64) % 997) as f32;
        total
    }
}

pub mod m186 {
    use super::*;

    pub struct Accumulator186<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator186<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.285_f32 + y.sin();
        let b = y * 1.472_f32 - x.cos();
        let mut acc = Accumulator186::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.462_f32 + y.sin();
        let b = y * 9.237_f32 - x.cos();
        let mut acc = Accumulator186::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.522_f32 + y.sin();
        let b = y * 8.208_f32 - x.cos();
        let mut acc = Accumulator186::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.488_f32 + y.sin();
        let b = y * 5.637_f32 - x.cos();
        let mut acc = Accumulator186::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.995_f32 + y.sin();
        let b = y * 6.624_f32 - x.cos();
        let mut acc = Accumulator186::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.799_f32 + y.sin();
        let b = y * 1.174_f32 - x.cos();
        let mut acc = Accumulator186::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.108_f32 + y.sin();
        let b = y * 3.789_f32 - x.cos();
        let mut acc = Accumulator186::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.786_f32 + y.sin();
        let b = y * 9.694_f32 - x.cos();
        let mut acc = Accumulator186::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.883_f32 + y.sin();
        let b = y * 9.758_f32 - x.cos();
        let mut acc = Accumulator186::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.156_f32 + y.sin();
        let b = y * 6.445_f32 - x.cos();
        let mut acc = Accumulator186::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.903_f32 + y.sin();
        let b = y * 3.897_f32 - x.cos();
        let mut acc = Accumulator186::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.55_f32 + y.sin();
        let b = y * 9.609_f32 - x.cos();
        let mut acc = Accumulator186::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.139_f32 + y.sin();
        let b = y * 4.217_f32 - x.cos();
        let mut acc = Accumulator186::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.393_f32 + y.sin();
        let b = y * 7.204_f32 - x.cos();
        let mut acc = Accumulator186::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.077_f32 + y.sin();
        let b = y * 3.611_f32 - x.cos();
        let mut acc = Accumulator186::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.258_f32 + y.sin();
        let b = y * 6.852_f32 - x.cos();
        let mut acc = Accumulator186::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.163_f32 + y.sin();
        let b = y * 1.659_f32 - x.cos();
        let mut acc = Accumulator186::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.643_f32 + y.sin();
        let b = y * 5.232_f32 - x.cos();
        let mut acc = Accumulator186::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.033_f32 + y.sin();
        let b = y * 5.271_f32 - x.cos();
        let mut acc = Accumulator186::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.791_f32 + y.sin();
        let b = y * 2.66_f32 - x.cos();
        let mut acc = Accumulator186::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_186(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_186() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_186(total as u64) % 997) as f32;
        total
    }
}

pub mod m187 {
    use super::*;

    pub struct Accumulator187<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator187<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.444_f32 + y.sin();
        let b = y * 9.175_f32 - x.cos();
        let mut acc = Accumulator187::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.462_f32 + y.sin();
        let b = y * 3.199_f32 - x.cos();
        let mut acc = Accumulator187::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.84_f32 + y.sin();
        let b = y * 4.688_f32 - x.cos();
        let mut acc = Accumulator187::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.765_f32 + y.sin();
        let b = y * 6.027_f32 - x.cos();
        let mut acc = Accumulator187::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.345_f32 + y.sin();
        let b = y * 8.057_f32 - x.cos();
        let mut acc = Accumulator187::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.172_f32 + y.sin();
        let b = y * 7.55_f32 - x.cos();
        let mut acc = Accumulator187::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.009_f32 + y.sin();
        let b = y * 4.497_f32 - x.cos();
        let mut acc = Accumulator187::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.104_f32 + y.sin();
        let b = y * 5.388_f32 - x.cos();
        let mut acc = Accumulator187::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.435_f32 + y.sin();
        let b = y * 1.408_f32 - x.cos();
        let mut acc = Accumulator187::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.23_f32 + y.sin();
        let b = y * 8.245_f32 - x.cos();
        let mut acc = Accumulator187::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.837_f32 + y.sin();
        let b = y * 7.35_f32 - x.cos();
        let mut acc = Accumulator187::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.932_f32 + y.sin();
        let b = y * 3.922_f32 - x.cos();
        let mut acc = Accumulator187::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.193_f32 + y.sin();
        let b = y * 4.48_f32 - x.cos();
        let mut acc = Accumulator187::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.241_f32 + y.sin();
        let b = y * 7.637_f32 - x.cos();
        let mut acc = Accumulator187::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.491_f32 + y.sin();
        let b = y * 2.052_f32 - x.cos();
        let mut acc = Accumulator187::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.752_f32 + y.sin();
        let b = y * 1.331_f32 - x.cos();
        let mut acc = Accumulator187::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.341_f32 + y.sin();
        let b = y * 7.269_f32 - x.cos();
        let mut acc = Accumulator187::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.079_f32 + y.sin();
        let b = y * 9.576_f32 - x.cos();
        let mut acc = Accumulator187::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.099_f32 + y.sin();
        let b = y * 3.468_f32 - x.cos();
        let mut acc = Accumulator187::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.671_f32 + y.sin();
        let b = y * 2.226_f32 - x.cos();
        let mut acc = Accumulator187::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_187(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(187u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_187() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_187(total as u64) % 997) as f32;
        total
    }
}

pub mod m188 {
    use super::*;

    pub struct Accumulator188<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator188<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.157_f32 + y.sin();
        let b = y * 9.813_f32 - x.cos();
        let mut acc = Accumulator188::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.657_f32 + y.sin();
        let b = y * 3.601_f32 - x.cos();
        let mut acc = Accumulator188::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.941_f32 + y.sin();
        let b = y * 5.792_f32 - x.cos();
        let mut acc = Accumulator188::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.563_f32 + y.sin();
        let b = y * 9.424_f32 - x.cos();
        let mut acc = Accumulator188::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.533_f32 + y.sin();
        let b = y * 5.224_f32 - x.cos();
        let mut acc = Accumulator188::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.642_f32 + y.sin();
        let b = y * 9.207_f32 - x.cos();
        let mut acc = Accumulator188::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.566_f32 + y.sin();
        let b = y * 2.394_f32 - x.cos();
        let mut acc = Accumulator188::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.095_f32 + y.sin();
        let b = y * 2.865_f32 - x.cos();
        let mut acc = Accumulator188::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.287_f32 + y.sin();
        let b = y * 7.516_f32 - x.cos();
        let mut acc = Accumulator188::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.334_f32 + y.sin();
        let b = y * 5.231_f32 - x.cos();
        let mut acc = Accumulator188::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.14_f32 + y.sin();
        let b = y * 8.168_f32 - x.cos();
        let mut acc = Accumulator188::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.363_f32 + y.sin();
        let b = y * 1.845_f32 - x.cos();
        let mut acc = Accumulator188::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.823_f32 + y.sin();
        let b = y * 8.333_f32 - x.cos();
        let mut acc = Accumulator188::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.05_f32 + y.sin();
        let b = y * 7.576_f32 - x.cos();
        let mut acc = Accumulator188::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.468_f32 + y.sin();
        let b = y * 6.849_f32 - x.cos();
        let mut acc = Accumulator188::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.529_f32 + y.sin();
        let b = y * 6.072_f32 - x.cos();
        let mut acc = Accumulator188::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.046_f32 + y.sin();
        let b = y * 7.637_f32 - x.cos();
        let mut acc = Accumulator188::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.459_f32 + y.sin();
        let b = y * 9.624_f32 - x.cos();
        let mut acc = Accumulator188::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.654_f32 + y.sin();
        let b = y * 5.015_f32 - x.cos();
        let mut acc = Accumulator188::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.203_f32 + y.sin();
        let b = y * 7.277_f32 - x.cos();
        let mut acc = Accumulator188::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_188(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_188() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_188(total as u64) % 997) as f32;
        total
    }
}

pub mod m189 {
    use super::*;

    pub struct Accumulator189<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator189<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.553_f32 + y.sin();
        let b = y * 1.118_f32 - x.cos();
        let mut acc = Accumulator189::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.762_f32 + y.sin();
        let b = y * 2.238_f32 - x.cos();
        let mut acc = Accumulator189::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.354_f32 + y.sin();
        let b = y * 9.182_f32 - x.cos();
        let mut acc = Accumulator189::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.476_f32 + y.sin();
        let b = y * 1.973_f32 - x.cos();
        let mut acc = Accumulator189::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.869_f32 + y.sin();
        let b = y * 4.432_f32 - x.cos();
        let mut acc = Accumulator189::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.425_f32 + y.sin();
        let b = y * 7.275_f32 - x.cos();
        let mut acc = Accumulator189::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.324_f32 + y.sin();
        let b = y * 2.188_f32 - x.cos();
        let mut acc = Accumulator189::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.057_f32 + y.sin();
        let b = y * 4.023_f32 - x.cos();
        let mut acc = Accumulator189::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.187_f32 + y.sin();
        let b = y * 0.254_f32 - x.cos();
        let mut acc = Accumulator189::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.692_f32 + y.sin();
        let b = y * 2.993_f32 - x.cos();
        let mut acc = Accumulator189::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.727_f32 + y.sin();
        let b = y * 5.403_f32 - x.cos();
        let mut acc = Accumulator189::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.793_f32 + y.sin();
        let b = y * 7.97_f32 - x.cos();
        let mut acc = Accumulator189::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.18_f32 + y.sin();
        let b = y * 8.116_f32 - x.cos();
        let mut acc = Accumulator189::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.989_f32 + y.sin();
        let b = y * 2.313_f32 - x.cos();
        let mut acc = Accumulator189::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.475_f32 + y.sin();
        let b = y * 5.447_f32 - x.cos();
        let mut acc = Accumulator189::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.321_f32 + y.sin();
        let b = y * 0.366_f32 - x.cos();
        let mut acc = Accumulator189::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.058_f32 + y.sin();
        let b = y * 4.189_f32 - x.cos();
        let mut acc = Accumulator189::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.351_f32 + y.sin();
        let b = y * 1.767_f32 - x.cos();
        let mut acc = Accumulator189::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.942_f32 + y.sin();
        let b = y * 7.559_f32 - x.cos();
        let mut acc = Accumulator189::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.135_f32 + y.sin();
        let b = y * 0.672_f32 - x.cos();
        let mut acc = Accumulator189::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_189(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_189() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_189(total as u64) % 997) as f32;
        total
    }
}

pub mod m190 {
    use super::*;

    pub struct Accumulator190<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator190<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.466_f32 + y.sin();
        let b = y * 6.924_f32 - x.cos();
        let mut acc = Accumulator190::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.388_f32 + y.sin();
        let b = y * 0.52_f32 - x.cos();
        let mut acc = Accumulator190::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.137_f32 + y.sin();
        let b = y * 1.274_f32 - x.cos();
        let mut acc = Accumulator190::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.343_f32 + y.sin();
        let b = y * 1.595_f32 - x.cos();
        let mut acc = Accumulator190::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.488_f32 + y.sin();
        let b = y * 2.511_f32 - x.cos();
        let mut acc = Accumulator190::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.68_f32 + y.sin();
        let b = y * 1.332_f32 - x.cos();
        let mut acc = Accumulator190::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.164_f32 + y.sin();
        let b = y * 6.984_f32 - x.cos();
        let mut acc = Accumulator190::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.234_f32 + y.sin();
        let b = y * 4.381_f32 - x.cos();
        let mut acc = Accumulator190::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.761_f32 + y.sin();
        let b = y * 6.357_f32 - x.cos();
        let mut acc = Accumulator190::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.893_f32 + y.sin();
        let b = y * 8.928_f32 - x.cos();
        let mut acc = Accumulator190::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.122_f32 + y.sin();
        let b = y * 1.478_f32 - x.cos();
        let mut acc = Accumulator190::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.854_f32 + y.sin();
        let b = y * 7.534_f32 - x.cos();
        let mut acc = Accumulator190::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.499_f32 + y.sin();
        let b = y * 0.301_f32 - x.cos();
        let mut acc = Accumulator190::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.236_f32 + y.sin();
        let b = y * 1.119_f32 - x.cos();
        let mut acc = Accumulator190::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.638_f32 + y.sin();
        let b = y * 6.303_f32 - x.cos();
        let mut acc = Accumulator190::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.646_f32 + y.sin();
        let b = y * 4.704_f32 - x.cos();
        let mut acc = Accumulator190::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.217_f32 + y.sin();
        let b = y * 1.376_f32 - x.cos();
        let mut acc = Accumulator190::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.991_f32 + y.sin();
        let b = y * 6.883_f32 - x.cos();
        let mut acc = Accumulator190::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.051_f32 + y.sin();
        let b = y * 5.581_f32 - x.cos();
        let mut acc = Accumulator190::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.619_f32 + y.sin();
        let b = y * 6.452_f32 - x.cos();
        let mut acc = Accumulator190::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_190(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m190-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_190() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_190(total as u64) % 997) as f32;
        total
    }
}

pub mod m191 {
    use super::*;

    pub struct Accumulator191<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator191<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.084_f32 + y.sin();
        let b = y * 3.547_f32 - x.cos();
        let mut acc = Accumulator191::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.523_f32 + y.sin();
        let b = y * 9.494_f32 - x.cos();
        let mut acc = Accumulator191::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.492_f32 + y.sin();
        let b = y * 6.773_f32 - x.cos();
        let mut acc = Accumulator191::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.411_f32 + y.sin();
        let b = y * 0.526_f32 - x.cos();
        let mut acc = Accumulator191::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.822_f32 + y.sin();
        let b = y * 8.422_f32 - x.cos();
        let mut acc = Accumulator191::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.453_f32 + y.sin();
        let b = y * 5.913_f32 - x.cos();
        let mut acc = Accumulator191::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.956_f32 + y.sin();
        let b = y * 3.261_f32 - x.cos();
        let mut acc = Accumulator191::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.231_f32 + y.sin();
        let b = y * 8.655_f32 - x.cos();
        let mut acc = Accumulator191::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.895_f32 + y.sin();
        let b = y * 8.211_f32 - x.cos();
        let mut acc = Accumulator191::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.086_f32 + y.sin();
        let b = y * 7.754_f32 - x.cos();
        let mut acc = Accumulator191::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.809_f32 + y.sin();
        let b = y * 7.091_f32 - x.cos();
        let mut acc = Accumulator191::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.859_f32 + y.sin();
        let b = y * 5.178_f32 - x.cos();
        let mut acc = Accumulator191::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.746_f32 + y.sin();
        let b = y * 0.283_f32 - x.cos();
        let mut acc = Accumulator191::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.151_f32 + y.sin();
        let b = y * 4.161_f32 - x.cos();
        let mut acc = Accumulator191::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.042_f32 + y.sin();
        let b = y * 4.33_f32 - x.cos();
        let mut acc = Accumulator191::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.974_f32 + y.sin();
        let b = y * 8.447_f32 - x.cos();
        let mut acc = Accumulator191::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.307_f32 + y.sin();
        let b = y * 3.21_f32 - x.cos();
        let mut acc = Accumulator191::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.668_f32 + y.sin();
        let b = y * 2.292_f32 - x.cos();
        let mut acc = Accumulator191::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.654_f32 + y.sin();
        let b = y * 1.555_f32 - x.cos();
        let mut acc = Accumulator191::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.482_f32 + y.sin();
        let b = y * 3.817_f32 - x.cos();
        let mut acc = Accumulator191::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_191(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_191() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_191(total as u64) % 997) as f32;
        total
    }
}

pub mod m192 {
    use super::*;

    pub struct Accumulator192<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator192<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.149_f32 + y.sin();
        let b = y * 2.073_f32 - x.cos();
        let mut acc = Accumulator192::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.478_f32 + y.sin();
        let b = y * 7.811_f32 - x.cos();
        let mut acc = Accumulator192::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.983_f32 + y.sin();
        let b = y * 3.977_f32 - x.cos();
        let mut acc = Accumulator192::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.896_f32 + y.sin();
        let b = y * 6.882_f32 - x.cos();
        let mut acc = Accumulator192::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.623_f32 + y.sin();
        let b = y * 9.066_f32 - x.cos();
        let mut acc = Accumulator192::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.604_f32 + y.sin();
        let b = y * 3.046_f32 - x.cos();
        let mut acc = Accumulator192::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.435_f32 + y.sin();
        let b = y * 9.022_f32 - x.cos();
        let mut acc = Accumulator192::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.678_f32 + y.sin();
        let b = y * 1.253_f32 - x.cos();
        let mut acc = Accumulator192::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.283_f32 + y.sin();
        let b = y * 7.285_f32 - x.cos();
        let mut acc = Accumulator192::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.524_f32 + y.sin();
        let b = y * 0.113_f32 - x.cos();
        let mut acc = Accumulator192::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.392_f32 + y.sin();
        let b = y * 2.814_f32 - x.cos();
        let mut acc = Accumulator192::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.957_f32 + y.sin();
        let b = y * 2.286_f32 - x.cos();
        let mut acc = Accumulator192::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.755_f32 + y.sin();
        let b = y * 0.775_f32 - x.cos();
        let mut acc = Accumulator192::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.99_f32 + y.sin();
        let b = y * 8.972_f32 - x.cos();
        let mut acc = Accumulator192::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.028_f32 + y.sin();
        let b = y * 6.733_f32 - x.cos();
        let mut acc = Accumulator192::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.173_f32 + y.sin();
        let b = y * 9.054_f32 - x.cos();
        let mut acc = Accumulator192::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.549_f32 + y.sin();
        let b = y * 5.918_f32 - x.cos();
        let mut acc = Accumulator192::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.789_f32 + y.sin();
        let b = y * 7.662_f32 - x.cos();
        let mut acc = Accumulator192::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.805_f32 + y.sin();
        let b = y * 4.608_f32 - x.cos();
        let mut acc = Accumulator192::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.458_f32 + y.sin();
        let b = y * 9.833_f32 - x.cos();
        let mut acc = Accumulator192::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_192(seed: u64) -> u64 {
        let re = Regex::new(r"m192-(\d+)").unwrap();
        let hay = format!("m192-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_192() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_192(total as u64) % 997) as f32;
        total
    }
}

pub mod m193 {
    use super::*;

    pub struct Accumulator193<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator193<T> {
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
        let b = y * 1.74_f32 - x.cos();
        let mut acc = Accumulator193::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.913_f32 + y.sin();
        let b = y * 2.813_f32 - x.cos();
        let mut acc = Accumulator193::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.884_f32 + y.sin();
        let b = y * 7.969_f32 - x.cos();
        let mut acc = Accumulator193::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.345_f32 + y.sin();
        let b = y * 3.236_f32 - x.cos();
        let mut acc = Accumulator193::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.256_f32 + y.sin();
        let b = y * 8.216_f32 - x.cos();
        let mut acc = Accumulator193::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.532_f32 + y.sin();
        let b = y * 2.846_f32 - x.cos();
        let mut acc = Accumulator193::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.205_f32 + y.sin();
        let b = y * 6.723_f32 - x.cos();
        let mut acc = Accumulator193::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.132_f32 + y.sin();
        let b = y * 6.75_f32 - x.cos();
        let mut acc = Accumulator193::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.037_f32 + y.sin();
        let b = y * 3.419_f32 - x.cos();
        let mut acc = Accumulator193::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.68_f32 + y.sin();
        let b = y * 1.981_f32 - x.cos();
        let mut acc = Accumulator193::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.037_f32 + y.sin();
        let b = y * 9.228_f32 - x.cos();
        let mut acc = Accumulator193::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.419_f32 + y.sin();
        let b = y * 7.984_f32 - x.cos();
        let mut acc = Accumulator193::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.506_f32 + y.sin();
        let b = y * 0.391_f32 - x.cos();
        let mut acc = Accumulator193::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.21_f32 + y.sin();
        let b = y * 1.985_f32 - x.cos();
        let mut acc = Accumulator193::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.165_f32 + y.sin();
        let b = y * 5.391_f32 - x.cos();
        let mut acc = Accumulator193::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.917_f32 + y.sin();
        let b = y * 4.581_f32 - x.cos();
        let mut acc = Accumulator193::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.075_f32 + y.sin();
        let b = y * 9.233_f32 - x.cos();
        let mut acc = Accumulator193::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.704_f32 + y.sin();
        let b = y * 5.426_f32 - x.cos();
        let mut acc = Accumulator193::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.518_f32 + y.sin();
        let b = y * 0.924_f32 - x.cos();
        let mut acc = Accumulator193::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.319_f32 + y.sin();
        let b = y * 4.52_f32 - x.cos();
        let mut acc = Accumulator193::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_193(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_193() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_193(total as u64) % 997) as f32;
        total
    }
}

pub mod m194 {
    use super::*;

    pub struct Accumulator194<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator194<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.971_f32 + y.sin();
        let b = y * 8.573_f32 - x.cos();
        let mut acc = Accumulator194::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.138_f32 + y.sin();
        let b = y * 3.901_f32 - x.cos();
        let mut acc = Accumulator194::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.039_f32 + y.sin();
        let b = y * 5.183_f32 - x.cos();
        let mut acc = Accumulator194::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.236_f32 + y.sin();
        let b = y * 2.719_f32 - x.cos();
        let mut acc = Accumulator194::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.052_f32 + y.sin();
        let b = y * 9.434_f32 - x.cos();
        let mut acc = Accumulator194::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.173_f32 + y.sin();
        let b = y * 7.959_f32 - x.cos();
        let mut acc = Accumulator194::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.258_f32 + y.sin();
        let b = y * 6.766_f32 - x.cos();
        let mut acc = Accumulator194::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.97_f32 + y.sin();
        let b = y * 0.789_f32 - x.cos();
        let mut acc = Accumulator194::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.546_f32 + y.sin();
        let b = y * 2.379_f32 - x.cos();
        let mut acc = Accumulator194::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.803_f32 + y.sin();
        let b = y * 0.491_f32 - x.cos();
        let mut acc = Accumulator194::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.865_f32 + y.sin();
        let b = y * 3.016_f32 - x.cos();
        let mut acc = Accumulator194::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.317_f32 + y.sin();
        let b = y * 7.68_f32 - x.cos();
        let mut acc = Accumulator194::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.258_f32 + y.sin();
        let b = y * 5.482_f32 - x.cos();
        let mut acc = Accumulator194::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.214_f32 + y.sin();
        let b = y * 4.817_f32 - x.cos();
        let mut acc = Accumulator194::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.222_f32 + y.sin();
        let b = y * 5.121_f32 - x.cos();
        let mut acc = Accumulator194::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.038_f32 + y.sin();
        let b = y * 6.917_f32 - x.cos();
        let mut acc = Accumulator194::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.975_f32 + y.sin();
        let b = y * 2.888_f32 - x.cos();
        let mut acc = Accumulator194::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.446_f32 + y.sin();
        let b = y * 0.95_f32 - x.cos();
        let mut acc = Accumulator194::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.466_f32 + y.sin();
        let b = y * 5.025_f32 - x.cos();
        let mut acc = Accumulator194::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.212_f32 + y.sin();
        let b = y * 4.316_f32 - x.cos();
        let mut acc = Accumulator194::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_194(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(194u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_194() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_194(total as u64) % 997) as f32;
        total
    }
}

pub mod m195 {
    use super::*;

    pub struct Accumulator195<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator195<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.761_f32 + y.sin();
        let b = y * 1.284_f32 - x.cos();
        let mut acc = Accumulator195::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.7_f32 + y.sin();
        let b = y * 1.461_f32 - x.cos();
        let mut acc = Accumulator195::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.732_f32 + y.sin();
        let b = y * 2.296_f32 - x.cos();
        let mut acc = Accumulator195::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.611_f32 + y.sin();
        let b = y * 4.489_f32 - x.cos();
        let mut acc = Accumulator195::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.564_f32 + y.sin();
        let b = y * 9.222_f32 - x.cos();
        let mut acc = Accumulator195::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.432_f32 + y.sin();
        let b = y * 3.086_f32 - x.cos();
        let mut acc = Accumulator195::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.767_f32 + y.sin();
        let b = y * 7.768_f32 - x.cos();
        let mut acc = Accumulator195::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.223_f32 + y.sin();
        let b = y * 0.928_f32 - x.cos();
        let mut acc = Accumulator195::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.309_f32 + y.sin();
        let b = y * 5.277_f32 - x.cos();
        let mut acc = Accumulator195::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.179_f32 + y.sin();
        let b = y * 8.863_f32 - x.cos();
        let mut acc = Accumulator195::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.54_f32 + y.sin();
        let b = y * 3.477_f32 - x.cos();
        let mut acc = Accumulator195::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.151_f32 + y.sin();
        let b = y * 5.56_f32 - x.cos();
        let mut acc = Accumulator195::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.651_f32 + y.sin();
        let b = y * 2.23_f32 - x.cos();
        let mut acc = Accumulator195::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.877_f32 + y.sin();
        let b = y * 8.018_f32 - x.cos();
        let mut acc = Accumulator195::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.114_f32 + y.sin();
        let b = y * 0.123_f32 - x.cos();
        let mut acc = Accumulator195::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.342_f32 + y.sin();
        let b = y * 5.851_f32 - x.cos();
        let mut acc = Accumulator195::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.31_f32 + y.sin();
        let b = y * 6.75_f32 - x.cos();
        let mut acc = Accumulator195::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.238_f32 + y.sin();
        let b = y * 0.991_f32 - x.cos();
        let mut acc = Accumulator195::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.605_f32 + y.sin();
        let b = y * 9.206_f32 - x.cos();
        let mut acc = Accumulator195::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.79_f32 + y.sin();
        let b = y * 4.798_f32 - x.cos();
        let mut acc = Accumulator195::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_195(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_195() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_195(total as u64) % 997) as f32;
        total
    }
}

pub mod m196 {
    use super::*;

    pub struct Accumulator196<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator196<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.827_f32 + y.sin();
        let b = y * 4.924_f32 - x.cos();
        let mut acc = Accumulator196::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.896_f32 + y.sin();
        let b = y * 2.399_f32 - x.cos();
        let mut acc = Accumulator196::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.618_f32 + y.sin();
        let b = y * 6.481_f32 - x.cos();
        let mut acc = Accumulator196::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.53_f32 + y.sin();
        let b = y * 3.082_f32 - x.cos();
        let mut acc = Accumulator196::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.423_f32 + y.sin();
        let b = y * 9.19_f32 - x.cos();
        let mut acc = Accumulator196::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.337_f32 + y.sin();
        let b = y * 2.609_f32 - x.cos();
        let mut acc = Accumulator196::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.604_f32 + y.sin();
        let b = y * 0.962_f32 - x.cos();
        let mut acc = Accumulator196::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.823_f32 + y.sin();
        let b = y * 7.614_f32 - x.cos();
        let mut acc = Accumulator196::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.662_f32 + y.sin();
        let b = y * 1.77_f32 - x.cos();
        let mut acc = Accumulator196::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.716_f32 + y.sin();
        let b = y * 4.135_f32 - x.cos();
        let mut acc = Accumulator196::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.8_f32 + y.sin();
        let b = y * 9.354_f32 - x.cos();
        let mut acc = Accumulator196::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.626_f32 + y.sin();
        let b = y * 5.069_f32 - x.cos();
        let mut acc = Accumulator196::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.02_f32 + y.sin();
        let b = y * 4.468_f32 - x.cos();
        let mut acc = Accumulator196::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.416_f32 + y.sin();
        let b = y * 5.255_f32 - x.cos();
        let mut acc = Accumulator196::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.593_f32 + y.sin();
        let b = y * 3.949_f32 - x.cos();
        let mut acc = Accumulator196::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.354_f32 + y.sin();
        let b = y * 2.296_f32 - x.cos();
        let mut acc = Accumulator196::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.655_f32 + y.sin();
        let b = y * 4.023_f32 - x.cos();
        let mut acc = Accumulator196::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.919_f32 + y.sin();
        let b = y * 0.331_f32 - x.cos();
        let mut acc = Accumulator196::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.912_f32 + y.sin();
        let b = y * 5.31_f32 - x.cos();
        let mut acc = Accumulator196::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.949_f32 + y.sin();
        let b = y * 2.906_f32 - x.cos();
        let mut acc = Accumulator196::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_196(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_196() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_196(total as u64) % 997) as f32;
        total
    }
}

pub mod m197 {
    use super::*;

    pub struct Accumulator197<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator197<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.832_f32 + y.sin();
        let b = y * 3.465_f32 - x.cos();
        let mut acc = Accumulator197::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.134_f32 + y.sin();
        let b = y * 5.305_f32 - x.cos();
        let mut acc = Accumulator197::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.903_f32 + y.sin();
        let b = y * 2.287_f32 - x.cos();
        let mut acc = Accumulator197::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.242_f32 + y.sin();
        let b = y * 7.143_f32 - x.cos();
        let mut acc = Accumulator197::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.661_f32 + y.sin();
        let b = y * 6.618_f32 - x.cos();
        let mut acc = Accumulator197::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.857_f32 + y.sin();
        let b = y * 2.399_f32 - x.cos();
        let mut acc = Accumulator197::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.178_f32 + y.sin();
        let b = y * 2.228_f32 - x.cos();
        let mut acc = Accumulator197::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.66_f32 + y.sin();
        let b = y * 3.377_f32 - x.cos();
        let mut acc = Accumulator197::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.029_f32 + y.sin();
        let b = y * 4.707_f32 - x.cos();
        let mut acc = Accumulator197::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.204_f32 + y.sin();
        let b = y * 1.294_f32 - x.cos();
        let mut acc = Accumulator197::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.967_f32 + y.sin();
        let b = y * 7.944_f32 - x.cos();
        let mut acc = Accumulator197::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.852_f32 + y.sin();
        let b = y * 5.074_f32 - x.cos();
        let mut acc = Accumulator197::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.903_f32 + y.sin();
        let b = y * 9.387_f32 - x.cos();
        let mut acc = Accumulator197::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.603_f32 + y.sin();
        let b = y * 9.843_f32 - x.cos();
        let mut acc = Accumulator197::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.896_f32 + y.sin();
        let b = y * 6.572_f32 - x.cos();
        let mut acc = Accumulator197::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.315_f32 + y.sin();
        let b = y * 2.819_f32 - x.cos();
        let mut acc = Accumulator197::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.104_f32 + y.sin();
        let b = y * 9.225_f32 - x.cos();
        let mut acc = Accumulator197::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.22_f32 + y.sin();
        let b = y * 7.707_f32 - x.cos();
        let mut acc = Accumulator197::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.146_f32 + y.sin();
        let b = y * 9.002_f32 - x.cos();
        let mut acc = Accumulator197::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.14_f32 + y.sin();
        let b = y * 5.038_f32 - x.cos();
        let mut acc = Accumulator197::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_197(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m197-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_197() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_197(total as u64) % 997) as f32;
        total
    }
}

pub mod m198 {
    use super::*;

    pub struct Accumulator198<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator198<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.344_f32 + y.sin();
        let b = y * 8.368_f32 - x.cos();
        let mut acc = Accumulator198::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.2_f32 + y.sin();
        let b = y * 1.351_f32 - x.cos();
        let mut acc = Accumulator198::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.588_f32 + y.sin();
        let b = y * 1.401_f32 - x.cos();
        let mut acc = Accumulator198::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.145_f32 + y.sin();
        let b = y * 1.703_f32 - x.cos();
        let mut acc = Accumulator198::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.671_f32 + y.sin();
        let b = y * 7.763_f32 - x.cos();
        let mut acc = Accumulator198::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.734_f32 + y.sin();
        let b = y * 2.012_f32 - x.cos();
        let mut acc = Accumulator198::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.506_f32 + y.sin();
        let b = y * 1.19_f32 - x.cos();
        let mut acc = Accumulator198::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.336_f32 + y.sin();
        let b = y * 4.828_f32 - x.cos();
        let mut acc = Accumulator198::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.856_f32 + y.sin();
        let b = y * 5.084_f32 - x.cos();
        let mut acc = Accumulator198::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.367_f32 + y.sin();
        let b = y * 6.633_f32 - x.cos();
        let mut acc = Accumulator198::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.807_f32 + y.sin();
        let b = y * 1.912_f32 - x.cos();
        let mut acc = Accumulator198::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.268_f32 + y.sin();
        let b = y * 0.47_f32 - x.cos();
        let mut acc = Accumulator198::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.671_f32 + y.sin();
        let b = y * 4.362_f32 - x.cos();
        let mut acc = Accumulator198::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.21_f32 + y.sin();
        let b = y * 6.878_f32 - x.cos();
        let mut acc = Accumulator198::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.571_f32 + y.sin();
        let b = y * 0.491_f32 - x.cos();
        let mut acc = Accumulator198::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.265_f32 + y.sin();
        let b = y * 5.705_f32 - x.cos();
        let mut acc = Accumulator198::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.065_f32 + y.sin();
        let b = y * 5.848_f32 - x.cos();
        let mut acc = Accumulator198::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.45_f32 + y.sin();
        let b = y * 3.33_f32 - x.cos();
        let mut acc = Accumulator198::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.79_f32 + y.sin();
        let b = y * 2.775_f32 - x.cos();
        let mut acc = Accumulator198::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.379_f32 + y.sin();
        let b = y * 3.026_f32 - x.cos();
        let mut acc = Accumulator198::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_198(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_198() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_198(total as u64) % 997) as f32;
        total
    }
}

pub mod m199 {
    use super::*;

    pub struct Accumulator199<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator199<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.033_f32 + y.sin();
        let b = y * 7.904_f32 - x.cos();
        let mut acc = Accumulator199::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.023_f32 + y.sin();
        let b = y * 7.606_f32 - x.cos();
        let mut acc = Accumulator199::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.862_f32 + y.sin();
        let b = y * 4.554_f32 - x.cos();
        let mut acc = Accumulator199::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.646_f32 + y.sin();
        let b = y * 5.887_f32 - x.cos();
        let mut acc = Accumulator199::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.549_f32 + y.sin();
        let b = y * 1.442_f32 - x.cos();
        let mut acc = Accumulator199::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.929_f32 + y.sin();
        let b = y * 8.116_f32 - x.cos();
        let mut acc = Accumulator199::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.825_f32 + y.sin();
        let b = y * 5.047_f32 - x.cos();
        let mut acc = Accumulator199::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.783_f32 + y.sin();
        let b = y * 7.684_f32 - x.cos();
        let mut acc = Accumulator199::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.344_f32 + y.sin();
        let b = y * 7.427_f32 - x.cos();
        let mut acc = Accumulator199::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.277_f32 + y.sin();
        let b = y * 8.887_f32 - x.cos();
        let mut acc = Accumulator199::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.292_f32 + y.sin();
        let b = y * 3.964_f32 - x.cos();
        let mut acc = Accumulator199::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.284_f32 + y.sin();
        let b = y * 9.262_f32 - x.cos();
        let mut acc = Accumulator199::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.817_f32 + y.sin();
        let b = y * 7.385_f32 - x.cos();
        let mut acc = Accumulator199::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.779_f32 + y.sin();
        let b = y * 1.683_f32 - x.cos();
        let mut acc = Accumulator199::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.71_f32 + y.sin();
        let b = y * 5.828_f32 - x.cos();
        let mut acc = Accumulator199::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.689_f32 + y.sin();
        let b = y * 2.134_f32 - x.cos();
        let mut acc = Accumulator199::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.22_f32 + y.sin();
        let b = y * 3.445_f32 - x.cos();
        let mut acc = Accumulator199::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.17_f32 + y.sin();
        let b = y * 1.1_f32 - x.cos();
        let mut acc = Accumulator199::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.5_f32 + y.sin();
        let b = y * 9.889_f32 - x.cos();
        let mut acc = Accumulator199::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.713_f32 + y.sin();
        let b = y * 2.424_f32 - x.cos();
        let mut acc = Accumulator199::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_199(seed: u64) -> u64 {
        let re = Regex::new(r"m199-(\d+)").unwrap();
        let hay = format!("m199-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_199() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_199(total as u64) % 997) as f32;
        total
    }
}

pub fn touch_bulk_1() -> f32 {
    let mut total = 0.0_f32;
    total += m100::run_all_100();
    total += m101::run_all_101();
    total += m102::run_all_102();
    total += m103::run_all_103();
    total += m104::run_all_104();
    total += m105::run_all_105();
    total += m106::run_all_106();
    total += m107::run_all_107();
    total += m108::run_all_108();
    total += m109::run_all_109();
    total += m110::run_all_110();
    total += m111::run_all_111();
    total += m112::run_all_112();
    total += m113::run_all_113();
    total += m114::run_all_114();
    total += m115::run_all_115();
    total += m116::run_all_116();
    total += m117::run_all_117();
    total += m118::run_all_118();
    total += m119::run_all_119();
    total += m120::run_all_120();
    total += m121::run_all_121();
    total += m122::run_all_122();
    total += m123::run_all_123();
    total += m124::run_all_124();
    total += m125::run_all_125();
    total += m126::run_all_126();
    total += m127::run_all_127();
    total += m128::run_all_128();
    total += m129::run_all_129();
    total += m130::run_all_130();
    total += m131::run_all_131();
    total += m132::run_all_132();
    total += m133::run_all_133();
    total += m134::run_all_134();
    total += m135::run_all_135();
    total += m136::run_all_136();
    total += m137::run_all_137();
    total += m138::run_all_138();
    total += m139::run_all_139();
    total += m140::run_all_140();
    total += m141::run_all_141();
    total += m142::run_all_142();
    total += m143::run_all_143();
    total += m144::run_all_144();
    total += m145::run_all_145();
    total += m146::run_all_146();
    total += m147::run_all_147();
    total += m148::run_all_148();
    total += m149::run_all_149();
    total += m150::run_all_150();
    total += m151::run_all_151();
    total += m152::run_all_152();
    total += m153::run_all_153();
    total += m154::run_all_154();
    total += m155::run_all_155();
    total += m156::run_all_156();
    total += m157::run_all_157();
    total += m158::run_all_158();
    total += m159::run_all_159();
    total += m160::run_all_160();
    total += m161::run_all_161();
    total += m162::run_all_162();
    total += m163::run_all_163();
    total += m164::run_all_164();
    total += m165::run_all_165();
    total += m166::run_all_166();
    total += m167::run_all_167();
    total += m168::run_all_168();
    total += m169::run_all_169();
    total += m170::run_all_170();
    total += m171::run_all_171();
    total += m172::run_all_172();
    total += m173::run_all_173();
    total += m174::run_all_174();
    total += m175::run_all_175();
    total += m176::run_all_176();
    total += m177::run_all_177();
    total += m178::run_all_178();
    total += m179::run_all_179();
    total += m180::run_all_180();
    total += m181::run_all_181();
    total += m182::run_all_182();
    total += m183::run_all_183();
    total += m184::run_all_184();
    total += m185::run_all_185();
    total += m186::run_all_186();
    total += m187::run_all_187();
    total += m188::run_all_188();
    total += m189::run_all_189();
    total += m190::run_all_190();
    total += m191::run_all_191();
    total += m192::run_all_192();
    total += m193::run_all_193();
    total += m194::run_all_194();
    total += m195::run_all_195();
    total += m196::run_all_196();
    total += m197::run_all_197();
    total += m198::run_all_198();
    total += m199::run_all_199();
    total
}
