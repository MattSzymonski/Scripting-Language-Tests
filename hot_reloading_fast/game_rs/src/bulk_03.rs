//! Auto-generated bulk module (file 3) - exists purely to make `game_rs`
//! heavier for a hot-patch speed experiment. Real arithmetic, real generics
//! (so monomorphization isn't free), and real use of a rotating set of
//! external crates so none of them are dead code. `touch_bulk_3()` is
//! called once from `game::init` so nothing here gets stripped as unreachable.

use rand::{Rng, SeedableRng};
use sha2::{Digest, Sha256};
use noise::{NoiseFn, Perlin};
use regex::Regex;
use itertools::Itertools;
use uuid::Uuid;
use chrono::Utc;


pub mod m300 {
    use super::*;

    pub struct Accumulator300<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator300<T> {
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
        let b = y * 1.459_f32 - x.cos();
        let mut acc = Accumulator300::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.199_f32 + y.sin();
        let b = y * 7.562_f32 - x.cos();
        let mut acc = Accumulator300::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.206_f32 + y.sin();
        let b = y * 3.235_f32 - x.cos();
        let mut acc = Accumulator300::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.027_f32 + y.sin();
        let b = y * 1.427_f32 - x.cos();
        let mut acc = Accumulator300::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.323_f32 + y.sin();
        let b = y * 9.485_f32 - x.cos();
        let mut acc = Accumulator300::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.112_f32 + y.sin();
        let b = y * 8.73_f32 - x.cos();
        let mut acc = Accumulator300::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.809_f32 + y.sin();
        let b = y * 3.985_f32 - x.cos();
        let mut acc = Accumulator300::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.224_f32 + y.sin();
        let b = y * 7.91_f32 - x.cos();
        let mut acc = Accumulator300::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.284_f32 + y.sin();
        let b = y * 9.043_f32 - x.cos();
        let mut acc = Accumulator300::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.983_f32 + y.sin();
        let b = y * 9.563_f32 - x.cos();
        let mut acc = Accumulator300::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.0_f32 + y.sin();
        let b = y * 5.557_f32 - x.cos();
        let mut acc = Accumulator300::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.494_f32 + y.sin();
        let b = y * 9.552_f32 - x.cos();
        let mut acc = Accumulator300::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.641_f32 + y.sin();
        let b = y * 4.435_f32 - x.cos();
        let mut acc = Accumulator300::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.801_f32 + y.sin();
        let b = y * 7.587_f32 - x.cos();
        let mut acc = Accumulator300::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.484_f32 + y.sin();
        let b = y * 8.623_f32 - x.cos();
        let mut acc = Accumulator300::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.847_f32 + y.sin();
        let b = y * 2.436_f32 - x.cos();
        let mut acc = Accumulator300::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.191_f32 + y.sin();
        let b = y * 4.354_f32 - x.cos();
        let mut acc = Accumulator300::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.351_f32 + y.sin();
        let b = y * 3.7_f32 - x.cos();
        let mut acc = Accumulator300::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.333_f32 + y.sin();
        let b = y * 4.712_f32 - x.cos();
        let mut acc = Accumulator300::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.023_f32 + y.sin();
        let b = y * 1.96_f32 - x.cos();
        let mut acc = Accumulator300::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_300(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_300() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_300(total as u64) % 997) as f32;
        total
    }
}

pub mod m301 {
    use super::*;

    pub struct Accumulator301<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator301<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.666_f32 + y.sin();
        let b = y * 8.145_f32 - x.cos();
        let mut acc = Accumulator301::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.046_f32 + y.sin();
        let b = y * 1.287_f32 - x.cos();
        let mut acc = Accumulator301::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.667_f32 + y.sin();
        let b = y * 2.791_f32 - x.cos();
        let mut acc = Accumulator301::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.664_f32 + y.sin();
        let b = y * 5.336_f32 - x.cos();
        let mut acc = Accumulator301::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.993_f32 + y.sin();
        let b = y * 4.947_f32 - x.cos();
        let mut acc = Accumulator301::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.925_f32 + y.sin();
        let b = y * 7.332_f32 - x.cos();
        let mut acc = Accumulator301::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.379_f32 + y.sin();
        let b = y * 7.873_f32 - x.cos();
        let mut acc = Accumulator301::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.77_f32 + y.sin();
        let b = y * 5.975_f32 - x.cos();
        let mut acc = Accumulator301::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.814_f32 + y.sin();
        let b = y * 2.338_f32 - x.cos();
        let mut acc = Accumulator301::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.589_f32 + y.sin();
        let b = y * 8.261_f32 - x.cos();
        let mut acc = Accumulator301::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.328_f32 + y.sin();
        let b = y * 8.747_f32 - x.cos();
        let mut acc = Accumulator301::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.81_f32 + y.sin();
        let b = y * 2.889_f32 - x.cos();
        let mut acc = Accumulator301::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.369_f32 + y.sin();
        let b = y * 6.231_f32 - x.cos();
        let mut acc = Accumulator301::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.249_f32 + y.sin();
        let b = y * 2.993_f32 - x.cos();
        let mut acc = Accumulator301::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.018_f32 + y.sin();
        let b = y * 5.129_f32 - x.cos();
        let mut acc = Accumulator301::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.518_f32 + y.sin();
        let b = y * 3.534_f32 - x.cos();
        let mut acc = Accumulator301::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.394_f32 + y.sin();
        let b = y * 1.041_f32 - x.cos();
        let mut acc = Accumulator301::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.07_f32 + y.sin();
        let b = y * 7.427_f32 - x.cos();
        let mut acc = Accumulator301::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.829_f32 + y.sin();
        let b = y * 3.518_f32 - x.cos();
        let mut acc = Accumulator301::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.675_f32 + y.sin();
        let b = y * 6.777_f32 - x.cos();
        let mut acc = Accumulator301::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_301(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_301() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_301(total as u64) % 997) as f32;
        total
    }
}

pub mod m302 {
    use super::*;

    pub struct Accumulator302<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator302<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.603_f32 + y.sin();
        let b = y * 8.221_f32 - x.cos();
        let mut acc = Accumulator302::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.552_f32 + y.sin();
        let b = y * 6.5_f32 - x.cos();
        let mut acc = Accumulator302::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.677_f32 + y.sin();
        let b = y * 5.327_f32 - x.cos();
        let mut acc = Accumulator302::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.53_f32 + y.sin();
        let b = y * 5.261_f32 - x.cos();
        let mut acc = Accumulator302::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.785_f32 + y.sin();
        let b = y * 3.643_f32 - x.cos();
        let mut acc = Accumulator302::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.219_f32 + y.sin();
        let b = y * 1.989_f32 - x.cos();
        let mut acc = Accumulator302::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.119_f32 + y.sin();
        let b = y * 8.702_f32 - x.cos();
        let mut acc = Accumulator302::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.795_f32 + y.sin();
        let b = y * 5.3_f32 - x.cos();
        let mut acc = Accumulator302::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.744_f32 + y.sin();
        let b = y * 2.359_f32 - x.cos();
        let mut acc = Accumulator302::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.885_f32 + y.sin();
        let b = y * 3.926_f32 - x.cos();
        let mut acc = Accumulator302::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.108_f32 + y.sin();
        let b = y * 9.634_f32 - x.cos();
        let mut acc = Accumulator302::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.541_f32 + y.sin();
        let b = y * 2.823_f32 - x.cos();
        let mut acc = Accumulator302::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.021_f32 + y.sin();
        let b = y * 3.232_f32 - x.cos();
        let mut acc = Accumulator302::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.702_f32 + y.sin();
        let b = y * 6.443_f32 - x.cos();
        let mut acc = Accumulator302::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.563_f32 + y.sin();
        let b = y * 5.892_f32 - x.cos();
        let mut acc = Accumulator302::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.936_f32 + y.sin();
        let b = y * 9.43_f32 - x.cos();
        let mut acc = Accumulator302::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.774_f32 + y.sin();
        let b = y * 9.866_f32 - x.cos();
        let mut acc = Accumulator302::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.18_f32 + y.sin();
        let b = y * 3.378_f32 - x.cos();
        let mut acc = Accumulator302::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.474_f32 + y.sin();
        let b = y * 4.608_f32 - x.cos();
        let mut acc = Accumulator302::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.867_f32 + y.sin();
        let b = y * 4.018_f32 - x.cos();
        let mut acc = Accumulator302::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_302(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m302-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_302() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_302(total as u64) % 997) as f32;
        total
    }
}

pub mod m303 {
    use super::*;

    pub struct Accumulator303<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator303<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.949_f32 + y.sin();
        let b = y * 0.438_f32 - x.cos();
        let mut acc = Accumulator303::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.182_f32 + y.sin();
        let b = y * 9.713_f32 - x.cos();
        let mut acc = Accumulator303::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.803_f32 + y.sin();
        let b = y * 0.638_f32 - x.cos();
        let mut acc = Accumulator303::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.351_f32 + y.sin();
        let b = y * 9.19_f32 - x.cos();
        let mut acc = Accumulator303::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.324_f32 + y.sin();
        let b = y * 1.244_f32 - x.cos();
        let mut acc = Accumulator303::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.949_f32 + y.sin();
        let b = y * 6.768_f32 - x.cos();
        let mut acc = Accumulator303::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.911_f32 + y.sin();
        let b = y * 6.435_f32 - x.cos();
        let mut acc = Accumulator303::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.636_f32 + y.sin();
        let b = y * 3.391_f32 - x.cos();
        let mut acc = Accumulator303::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.782_f32 + y.sin();
        let b = y * 3.171_f32 - x.cos();
        let mut acc = Accumulator303::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.311_f32 + y.sin();
        let b = y * 7.626_f32 - x.cos();
        let mut acc = Accumulator303::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.883_f32 + y.sin();
        let b = y * 5.02_f32 - x.cos();
        let mut acc = Accumulator303::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.712_f32 + y.sin();
        let b = y * 8.349_f32 - x.cos();
        let mut acc = Accumulator303::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.535_f32 + y.sin();
        let b = y * 9.176_f32 - x.cos();
        let mut acc = Accumulator303::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.786_f32 + y.sin();
        let b = y * 6.858_f32 - x.cos();
        let mut acc = Accumulator303::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.114_f32 + y.sin();
        let b = y * 2.685_f32 - x.cos();
        let mut acc = Accumulator303::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.16_f32 + y.sin();
        let b = y * 9.684_f32 - x.cos();
        let mut acc = Accumulator303::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.857_f32 + y.sin();
        let b = y * 0.721_f32 - x.cos();
        let mut acc = Accumulator303::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.425_f32 + y.sin();
        let b = y * 4.964_f32 - x.cos();
        let mut acc = Accumulator303::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.032_f32 + y.sin();
        let b = y * 9.062_f32 - x.cos();
        let mut acc = Accumulator303::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.312_f32 + y.sin();
        let b = y * 8.39_f32 - x.cos();
        let mut acc = Accumulator303::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_303(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_303() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_303(total as u64) % 997) as f32;
        total
    }
}

pub mod m304 {
    use super::*;

    pub struct Accumulator304<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator304<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.276_f32 + y.sin();
        let b = y * 6.824_f32 - x.cos();
        let mut acc = Accumulator304::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.736_f32 + y.sin();
        let b = y * 2.409_f32 - x.cos();
        let mut acc = Accumulator304::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.235_f32 + y.sin();
        let b = y * 7.388_f32 - x.cos();
        let mut acc = Accumulator304::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.176_f32 + y.sin();
        let b = y * 7.29_f32 - x.cos();
        let mut acc = Accumulator304::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.243_f32 + y.sin();
        let b = y * 8.729_f32 - x.cos();
        let mut acc = Accumulator304::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.247_f32 + y.sin();
        let b = y * 6.478_f32 - x.cos();
        let mut acc = Accumulator304::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.357_f32 + y.sin();
        let b = y * 8.187_f32 - x.cos();
        let mut acc = Accumulator304::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.519_f32 + y.sin();
        let b = y * 6.598_f32 - x.cos();
        let mut acc = Accumulator304::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.737_f32 + y.sin();
        let b = y * 4.601_f32 - x.cos();
        let mut acc = Accumulator304::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.424_f32 + y.sin();
        let b = y * 1.316_f32 - x.cos();
        let mut acc = Accumulator304::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.972_f32 + y.sin();
        let b = y * 1.104_f32 - x.cos();
        let mut acc = Accumulator304::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.466_f32 + y.sin();
        let b = y * 4.072_f32 - x.cos();
        let mut acc = Accumulator304::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.864_f32 + y.sin();
        let b = y * 9.157_f32 - x.cos();
        let mut acc = Accumulator304::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.341_f32 + y.sin();
        let b = y * 3.128_f32 - x.cos();
        let mut acc = Accumulator304::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.032_f32 + y.sin();
        let b = y * 8.038_f32 - x.cos();
        let mut acc = Accumulator304::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.09_f32 + y.sin();
        let b = y * 8.58_f32 - x.cos();
        let mut acc = Accumulator304::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.59_f32 + y.sin();
        let b = y * 0.526_f32 - x.cos();
        let mut acc = Accumulator304::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.64_f32 + y.sin();
        let b = y * 9.653_f32 - x.cos();
        let mut acc = Accumulator304::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.694_f32 + y.sin();
        let b = y * 5.15_f32 - x.cos();
        let mut acc = Accumulator304::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.584_f32 + y.sin();
        let b = y * 8.128_f32 - x.cos();
        let mut acc = Accumulator304::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_304(seed: u64) -> u64 {
        let re = Regex::new(r"m304-(\d+)").unwrap();
        let hay = format!("m304-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_304() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_304(total as u64) % 997) as f32;
        total
    }
}

pub mod m305 {
    use super::*;

    pub struct Accumulator305<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator305<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.956_f32 + y.sin();
        let b = y * 9.003_f32 - x.cos();
        let mut acc = Accumulator305::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.448_f32 + y.sin();
        let b = y * 4.033_f32 - x.cos();
        let mut acc = Accumulator305::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.522_f32 + y.sin();
        let b = y * 3.078_f32 - x.cos();
        let mut acc = Accumulator305::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.043_f32 + y.sin();
        let b = y * 7.078_f32 - x.cos();
        let mut acc = Accumulator305::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.203_f32 + y.sin();
        let b = y * 1.962_f32 - x.cos();
        let mut acc = Accumulator305::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.823_f32 + y.sin();
        let b = y * 1.729_f32 - x.cos();
        let mut acc = Accumulator305::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.229_f32 + y.sin();
        let b = y * 1.065_f32 - x.cos();
        let mut acc = Accumulator305::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.252_f32 + y.sin();
        let b = y * 2.925_f32 - x.cos();
        let mut acc = Accumulator305::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.84_f32 + y.sin();
        let b = y * 1.705_f32 - x.cos();
        let mut acc = Accumulator305::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.863_f32 + y.sin();
        let b = y * 0.874_f32 - x.cos();
        let mut acc = Accumulator305::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.498_f32 + y.sin();
        let b = y * 4.966_f32 - x.cos();
        let mut acc = Accumulator305::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.671_f32 + y.sin();
        let b = y * 3.157_f32 - x.cos();
        let mut acc = Accumulator305::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.638_f32 + y.sin();
        let b = y * 7.22_f32 - x.cos();
        let mut acc = Accumulator305::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.331_f32 + y.sin();
        let b = y * 9.11_f32 - x.cos();
        let mut acc = Accumulator305::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.577_f32 + y.sin();
        let b = y * 2.653_f32 - x.cos();
        let mut acc = Accumulator305::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.792_f32 + y.sin();
        let b = y * 7.589_f32 - x.cos();
        let mut acc = Accumulator305::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.315_f32 + y.sin();
        let b = y * 4.457_f32 - x.cos();
        let mut acc = Accumulator305::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.734_f32 + y.sin();
        let b = y * 0.577_f32 - x.cos();
        let mut acc = Accumulator305::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.398_f32 + y.sin();
        let b = y * 1.34_f32 - x.cos();
        let mut acc = Accumulator305::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.447_f32 + y.sin();
        let b = y * 5.5_f32 - x.cos();
        let mut acc = Accumulator305::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_305(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_305() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_305(total as u64) % 997) as f32;
        total
    }
}

pub mod m306 {
    use super::*;

    pub struct Accumulator306<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator306<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.129_f32 + y.sin();
        let b = y * 8.044_f32 - x.cos();
        let mut acc = Accumulator306::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.715_f32 + y.sin();
        let b = y * 1.785_f32 - x.cos();
        let mut acc = Accumulator306::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.775_f32 + y.sin();
        let b = y * 8.801_f32 - x.cos();
        let mut acc = Accumulator306::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.142_f32 + y.sin();
        let b = y * 5.553_f32 - x.cos();
        let mut acc = Accumulator306::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.89_f32 + y.sin();
        let b = y * 7.942_f32 - x.cos();
        let mut acc = Accumulator306::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.343_f32 + y.sin();
        let b = y * 2.952_f32 - x.cos();
        let mut acc = Accumulator306::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.436_f32 + y.sin();
        let b = y * 9.144_f32 - x.cos();
        let mut acc = Accumulator306::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.761_f32 + y.sin();
        let b = y * 8.634_f32 - x.cos();
        let mut acc = Accumulator306::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.104_f32 + y.sin();
        let b = y * 0.263_f32 - x.cos();
        let mut acc = Accumulator306::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.458_f32 + y.sin();
        let b = y * 7.333_f32 - x.cos();
        let mut acc = Accumulator306::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.535_f32 + y.sin();
        let b = y * 8.95_f32 - x.cos();
        let mut acc = Accumulator306::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.047_f32 + y.sin();
        let b = y * 4.641_f32 - x.cos();
        let mut acc = Accumulator306::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.447_f32 + y.sin();
        let b = y * 4.52_f32 - x.cos();
        let mut acc = Accumulator306::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.248_f32 + y.sin();
        let b = y * 3.501_f32 - x.cos();
        let mut acc = Accumulator306::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.97_f32 + y.sin();
        let b = y * 0.218_f32 - x.cos();
        let mut acc = Accumulator306::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.161_f32 + y.sin();
        let b = y * 2.553_f32 - x.cos();
        let mut acc = Accumulator306::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.97_f32 + y.sin();
        let b = y * 4.304_f32 - x.cos();
        let mut acc = Accumulator306::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.71_f32 + y.sin();
        let b = y * 6.737_f32 - x.cos();
        let mut acc = Accumulator306::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.269_f32 + y.sin();
        let b = y * 7.528_f32 - x.cos();
        let mut acc = Accumulator306::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.468_f32 + y.sin();
        let b = y * 8.475_f32 - x.cos();
        let mut acc = Accumulator306::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_306(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(306u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_306() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_306(total as u64) % 997) as f32;
        total
    }
}

pub mod m307 {
    use super::*;

    pub struct Accumulator307<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator307<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.198_f32 + y.sin();
        let b = y * 4.647_f32 - x.cos();
        let mut acc = Accumulator307::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.445_f32 + y.sin();
        let b = y * 8.105_f32 - x.cos();
        let mut acc = Accumulator307::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.188_f32 + y.sin();
        let b = y * 2.589_f32 - x.cos();
        let mut acc = Accumulator307::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.627_f32 + y.sin();
        let b = y * 0.297_f32 - x.cos();
        let mut acc = Accumulator307::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.561_f32 + y.sin();
        let b = y * 5.727_f32 - x.cos();
        let mut acc = Accumulator307::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.568_f32 + y.sin();
        let b = y * 8.608_f32 - x.cos();
        let mut acc = Accumulator307::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.064_f32 + y.sin();
        let b = y * 8.605_f32 - x.cos();
        let mut acc = Accumulator307::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.529_f32 + y.sin();
        let b = y * 4.559_f32 - x.cos();
        let mut acc = Accumulator307::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.154_f32 + y.sin();
        let b = y * 4.709_f32 - x.cos();
        let mut acc = Accumulator307::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.63_f32 + y.sin();
        let b = y * 0.398_f32 - x.cos();
        let mut acc = Accumulator307::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.303_f32 + y.sin();
        let b = y * 4.141_f32 - x.cos();
        let mut acc = Accumulator307::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.364_f32 + y.sin();
        let b = y * 5.578_f32 - x.cos();
        let mut acc = Accumulator307::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.883_f32 + y.sin();
        let b = y * 6.675_f32 - x.cos();
        let mut acc = Accumulator307::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.07_f32 + y.sin();
        let b = y * 7.601_f32 - x.cos();
        let mut acc = Accumulator307::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.162_f32 + y.sin();
        let b = y * 7.585_f32 - x.cos();
        let mut acc = Accumulator307::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.428_f32 + y.sin();
        let b = y * 5.519_f32 - x.cos();
        let mut acc = Accumulator307::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.423_f32 + y.sin();
        let b = y * 4.569_f32 - x.cos();
        let mut acc = Accumulator307::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.019_f32 + y.sin();
        let b = y * 8.292_f32 - x.cos();
        let mut acc = Accumulator307::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.599_f32 + y.sin();
        let b = y * 4.185_f32 - x.cos();
        let mut acc = Accumulator307::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.445_f32 + y.sin();
        let b = y * 4.613_f32 - x.cos();
        let mut acc = Accumulator307::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_307(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_307() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_307(total as u64) % 997) as f32;
        total
    }
}

pub mod m308 {
    use super::*;

    pub struct Accumulator308<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator308<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.819_f32 + y.sin();
        let b = y * 9.741_f32 - x.cos();
        let mut acc = Accumulator308::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.122_f32 + y.sin();
        let b = y * 0.59_f32 - x.cos();
        let mut acc = Accumulator308::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.401_f32 + y.sin();
        let b = y * 1.908_f32 - x.cos();
        let mut acc = Accumulator308::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.087_f32 + y.sin();
        let b = y * 2.241_f32 - x.cos();
        let mut acc = Accumulator308::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.237_f32 + y.sin();
        let b = y * 5.905_f32 - x.cos();
        let mut acc = Accumulator308::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.422_f32 + y.sin();
        let b = y * 3.846_f32 - x.cos();
        let mut acc = Accumulator308::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.718_f32 + y.sin();
        let b = y * 1.371_f32 - x.cos();
        let mut acc = Accumulator308::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.412_f32 + y.sin();
        let b = y * 7.416_f32 - x.cos();
        let mut acc = Accumulator308::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.504_f32 + y.sin();
        let b = y * 3.985_f32 - x.cos();
        let mut acc = Accumulator308::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.346_f32 + y.sin();
        let b = y * 2.281_f32 - x.cos();
        let mut acc = Accumulator308::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.784_f32 + y.sin();
        let b = y * 7.724_f32 - x.cos();
        let mut acc = Accumulator308::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.452_f32 + y.sin();
        let b = y * 6.166_f32 - x.cos();
        let mut acc = Accumulator308::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.223_f32 + y.sin();
        let b = y * 9.06_f32 - x.cos();
        let mut acc = Accumulator308::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.799_f32 + y.sin();
        let b = y * 9.144_f32 - x.cos();
        let mut acc = Accumulator308::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.446_f32 + y.sin();
        let b = y * 8.766_f32 - x.cos();
        let mut acc = Accumulator308::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.886_f32 + y.sin();
        let b = y * 7.373_f32 - x.cos();
        let mut acc = Accumulator308::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.941_f32 + y.sin();
        let b = y * 4.597_f32 - x.cos();
        let mut acc = Accumulator308::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.637_f32 + y.sin();
        let b = y * 6.909_f32 - x.cos();
        let mut acc = Accumulator308::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.256_f32 + y.sin();
        let b = y * 7.403_f32 - x.cos();
        let mut acc = Accumulator308::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.175_f32 + y.sin();
        let b = y * 8.837_f32 - x.cos();
        let mut acc = Accumulator308::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_308(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_308() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_308(total as u64) % 997) as f32;
        total
    }
}

pub mod m309 {
    use super::*;

    pub struct Accumulator309<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator309<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.887_f32 + y.sin();
        let b = y * 2.159_f32 - x.cos();
        let mut acc = Accumulator309::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.177_f32 + y.sin();
        let b = y * 4.081_f32 - x.cos();
        let mut acc = Accumulator309::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.063_f32 + y.sin();
        let b = y * 1.286_f32 - x.cos();
        let mut acc = Accumulator309::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.624_f32 + y.sin();
        let b = y * 3.992_f32 - x.cos();
        let mut acc = Accumulator309::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.768_f32 + y.sin();
        let b = y * 3.53_f32 - x.cos();
        let mut acc = Accumulator309::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.702_f32 + y.sin();
        let b = y * 8.128_f32 - x.cos();
        let mut acc = Accumulator309::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.668_f32 + y.sin();
        let b = y * 5.796_f32 - x.cos();
        let mut acc = Accumulator309::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.161_f32 + y.sin();
        let b = y * 2.225_f32 - x.cos();
        let mut acc = Accumulator309::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.57_f32 + y.sin();
        let b = y * 1.476_f32 - x.cos();
        let mut acc = Accumulator309::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.133_f32 + y.sin();
        let b = y * 4.204_f32 - x.cos();
        let mut acc = Accumulator309::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.862_f32 + y.sin();
        let b = y * 5.098_f32 - x.cos();
        let mut acc = Accumulator309::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.209_f32 + y.sin();
        let b = y * 8.888_f32 - x.cos();
        let mut acc = Accumulator309::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.993_f32 + y.sin();
        let b = y * 0.843_f32 - x.cos();
        let mut acc = Accumulator309::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.514_f32 + y.sin();
        let b = y * 4.201_f32 - x.cos();
        let mut acc = Accumulator309::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.982_f32 + y.sin();
        let b = y * 3.184_f32 - x.cos();
        let mut acc = Accumulator309::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.652_f32 + y.sin();
        let b = y * 2.368_f32 - x.cos();
        let mut acc = Accumulator309::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.444_f32 + y.sin();
        let b = y * 5.355_f32 - x.cos();
        let mut acc = Accumulator309::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.882_f32 + y.sin();
        let b = y * 1.877_f32 - x.cos();
        let mut acc = Accumulator309::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.18_f32 + y.sin();
        let b = y * 1.447_f32 - x.cos();
        let mut acc = Accumulator309::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.185_f32 + y.sin();
        let b = y * 3.01_f32 - x.cos();
        let mut acc = Accumulator309::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_309(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m309-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_309() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_309(total as u64) % 997) as f32;
        total
    }
}

pub mod m310 {
    use super::*;

    pub struct Accumulator310<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator310<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.859_f32 + y.sin();
        let b = y * 8.64_f32 - x.cos();
        let mut acc = Accumulator310::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.698_f32 + y.sin();
        let b = y * 9.782_f32 - x.cos();
        let mut acc = Accumulator310::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.81_f32 + y.sin();
        let b = y * 8.104_f32 - x.cos();
        let mut acc = Accumulator310::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.68_f32 + y.sin();
        let b = y * 7.678_f32 - x.cos();
        let mut acc = Accumulator310::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.336_f32 + y.sin();
        let b = y * 2.836_f32 - x.cos();
        let mut acc = Accumulator310::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.622_f32 + y.sin();
        let b = y * 1.309_f32 - x.cos();
        let mut acc = Accumulator310::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.175_f32 + y.sin();
        let b = y * 0.263_f32 - x.cos();
        let mut acc = Accumulator310::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.116_f32 + y.sin();
        let b = y * 3.602_f32 - x.cos();
        let mut acc = Accumulator310::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.901_f32 + y.sin();
        let b = y * 8.074_f32 - x.cos();
        let mut acc = Accumulator310::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.865_f32 + y.sin();
        let b = y * 6.74_f32 - x.cos();
        let mut acc = Accumulator310::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.88_f32 + y.sin();
        let b = y * 6.806_f32 - x.cos();
        let mut acc = Accumulator310::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.64_f32 + y.sin();
        let b = y * 2.28_f32 - x.cos();
        let mut acc = Accumulator310::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.499_f32 + y.sin();
        let b = y * 1.825_f32 - x.cos();
        let mut acc = Accumulator310::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.297_f32 + y.sin();
        let b = y * 9.729_f32 - x.cos();
        let mut acc = Accumulator310::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.276_f32 + y.sin();
        let b = y * 2.244_f32 - x.cos();
        let mut acc = Accumulator310::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.43_f32 + y.sin();
        let b = y * 4.317_f32 - x.cos();
        let mut acc = Accumulator310::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.378_f32 + y.sin();
        let b = y * 3.507_f32 - x.cos();
        let mut acc = Accumulator310::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.308_f32 + y.sin();
        let b = y * 5.115_f32 - x.cos();
        let mut acc = Accumulator310::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.08_f32 + y.sin();
        let b = y * 3.879_f32 - x.cos();
        let mut acc = Accumulator310::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.604_f32 + y.sin();
        let b = y * 1.965_f32 - x.cos();
        let mut acc = Accumulator310::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_310(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_310() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_310(total as u64) % 997) as f32;
        total
    }
}

pub mod m311 {
    use super::*;

    pub struct Accumulator311<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator311<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.906_f32 + y.sin();
        let b = y * 7.729_f32 - x.cos();
        let mut acc = Accumulator311::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.792_f32 + y.sin();
        let b = y * 2.02_f32 - x.cos();
        let mut acc = Accumulator311::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.075_f32 + y.sin();
        let b = y * 6.058_f32 - x.cos();
        let mut acc = Accumulator311::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.406_f32 + y.sin();
        let b = y * 5.375_f32 - x.cos();
        let mut acc = Accumulator311::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.789_f32 + y.sin();
        let b = y * 4.158_f32 - x.cos();
        let mut acc = Accumulator311::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.671_f32 + y.sin();
        let b = y * 5.174_f32 - x.cos();
        let mut acc = Accumulator311::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.399_f32 + y.sin();
        let b = y * 2.385_f32 - x.cos();
        let mut acc = Accumulator311::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.616_f32 + y.sin();
        let b = y * 6.428_f32 - x.cos();
        let mut acc = Accumulator311::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.802_f32 + y.sin();
        let b = y * 7.211_f32 - x.cos();
        let mut acc = Accumulator311::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.218_f32 + y.sin();
        let b = y * 2.521_f32 - x.cos();
        let mut acc = Accumulator311::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.006_f32 + y.sin();
        let b = y * 6.277_f32 - x.cos();
        let mut acc = Accumulator311::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.243_f32 + y.sin();
        let b = y * 8.485_f32 - x.cos();
        let mut acc = Accumulator311::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.526_f32 + y.sin();
        let b = y * 3.169_f32 - x.cos();
        let mut acc = Accumulator311::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.984_f32 + y.sin();
        let b = y * 6.043_f32 - x.cos();
        let mut acc = Accumulator311::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.098_f32 + y.sin();
        let b = y * 1.753_f32 - x.cos();
        let mut acc = Accumulator311::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.097_f32 + y.sin();
        let b = y * 7.286_f32 - x.cos();
        let mut acc = Accumulator311::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.113_f32 + y.sin();
        let b = y * 1.918_f32 - x.cos();
        let mut acc = Accumulator311::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.225_f32 + y.sin();
        let b = y * 2.314_f32 - x.cos();
        let mut acc = Accumulator311::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.259_f32 + y.sin();
        let b = y * 2.059_f32 - x.cos();
        let mut acc = Accumulator311::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.922_f32 + y.sin();
        let b = y * 5.256_f32 - x.cos();
        let mut acc = Accumulator311::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_311(seed: u64) -> u64 {
        let re = Regex::new(r"m311-(\d+)").unwrap();
        let hay = format!("m311-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_311() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_311(total as u64) % 997) as f32;
        total
    }
}

pub mod m312 {
    use super::*;

    pub struct Accumulator312<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator312<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.447_f32 + y.sin();
        let b = y * 7.213_f32 - x.cos();
        let mut acc = Accumulator312::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.042_f32 + y.sin();
        let b = y * 1.846_f32 - x.cos();
        let mut acc = Accumulator312::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.242_f32 + y.sin();
        let b = y * 7.824_f32 - x.cos();
        let mut acc = Accumulator312::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.547_f32 + y.sin();
        let b = y * 4.312_f32 - x.cos();
        let mut acc = Accumulator312::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.944_f32 + y.sin();
        let b = y * 2.49_f32 - x.cos();
        let mut acc = Accumulator312::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.663_f32 + y.sin();
        let b = y * 1.041_f32 - x.cos();
        let mut acc = Accumulator312::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.642_f32 + y.sin();
        let b = y * 7.868_f32 - x.cos();
        let mut acc = Accumulator312::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.343_f32 + y.sin();
        let b = y * 3.908_f32 - x.cos();
        let mut acc = Accumulator312::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.551_f32 + y.sin();
        let b = y * 2.925_f32 - x.cos();
        let mut acc = Accumulator312::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.897_f32 + y.sin();
        let b = y * 7.656_f32 - x.cos();
        let mut acc = Accumulator312::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.842_f32 + y.sin();
        let b = y * 8.583_f32 - x.cos();
        let mut acc = Accumulator312::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.573_f32 + y.sin();
        let b = y * 6.612_f32 - x.cos();
        let mut acc = Accumulator312::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.808_f32 + y.sin();
        let b = y * 6.74_f32 - x.cos();
        let mut acc = Accumulator312::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.917_f32 + y.sin();
        let b = y * 3.054_f32 - x.cos();
        let mut acc = Accumulator312::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.359_f32 + y.sin();
        let b = y * 9.533_f32 - x.cos();
        let mut acc = Accumulator312::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.675_f32 + y.sin();
        let b = y * 9.573_f32 - x.cos();
        let mut acc = Accumulator312::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.041_f32 + y.sin();
        let b = y * 1.303_f32 - x.cos();
        let mut acc = Accumulator312::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.757_f32 + y.sin();
        let b = y * 2.688_f32 - x.cos();
        let mut acc = Accumulator312::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.525_f32 + y.sin();
        let b = y * 8.437_f32 - x.cos();
        let mut acc = Accumulator312::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.938_f32 + y.sin();
        let b = y * 5.036_f32 - x.cos();
        let mut acc = Accumulator312::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_312(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_312() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_312(total as u64) % 997) as f32;
        total
    }
}

pub mod m313 {
    use super::*;

    pub struct Accumulator313<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator313<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.598_f32 + y.sin();
        let b = y * 3.491_f32 - x.cos();
        let mut acc = Accumulator313::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.87_f32 + y.sin();
        let b = y * 1.833_f32 - x.cos();
        let mut acc = Accumulator313::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.008_f32 + y.sin();
        let b = y * 7.372_f32 - x.cos();
        let mut acc = Accumulator313::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.027_f32 + y.sin();
        let b = y * 8.166_f32 - x.cos();
        let mut acc = Accumulator313::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.156_f32 + y.sin();
        let b = y * 6.533_f32 - x.cos();
        let mut acc = Accumulator313::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.919_f32 + y.sin();
        let b = y * 9.309_f32 - x.cos();
        let mut acc = Accumulator313::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.988_f32 + y.sin();
        let b = y * 1.717_f32 - x.cos();
        let mut acc = Accumulator313::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.992_f32 + y.sin();
        let b = y * 2.704_f32 - x.cos();
        let mut acc = Accumulator313::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.434_f32 + y.sin();
        let b = y * 5.497_f32 - x.cos();
        let mut acc = Accumulator313::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.338_f32 + y.sin();
        let b = y * 6.669_f32 - x.cos();
        let mut acc = Accumulator313::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.723_f32 + y.sin();
        let b = y * 4.881_f32 - x.cos();
        let mut acc = Accumulator313::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.064_f32 + y.sin();
        let b = y * 1.026_f32 - x.cos();
        let mut acc = Accumulator313::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.566_f32 + y.sin();
        let b = y * 2.673_f32 - x.cos();
        let mut acc = Accumulator313::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.53_f32 + y.sin();
        let b = y * 3.223_f32 - x.cos();
        let mut acc = Accumulator313::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.909_f32 + y.sin();
        let b = y * 1.569_f32 - x.cos();
        let mut acc = Accumulator313::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.236_f32 + y.sin();
        let b = y * 2.574_f32 - x.cos();
        let mut acc = Accumulator313::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.892_f32 + y.sin();
        let b = y * 2.214_f32 - x.cos();
        let mut acc = Accumulator313::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.695_f32 + y.sin();
        let b = y * 6.806_f32 - x.cos();
        let mut acc = Accumulator313::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.8_f32 + y.sin();
        let b = y * 3.715_f32 - x.cos();
        let mut acc = Accumulator313::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.986_f32 + y.sin();
        let b = y * 7.217_f32 - x.cos();
        let mut acc = Accumulator313::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_313(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(313u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_313() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_313(total as u64) % 997) as f32;
        total
    }
}

pub mod m314 {
    use super::*;

    pub struct Accumulator314<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator314<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.857_f32 + y.sin();
        let b = y * 5.022_f32 - x.cos();
        let mut acc = Accumulator314::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.1_f32 + y.sin();
        let b = y * 7.601_f32 - x.cos();
        let mut acc = Accumulator314::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.585_f32 + y.sin();
        let b = y * 6.702_f32 - x.cos();
        let mut acc = Accumulator314::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.617_f32 + y.sin();
        let b = y * 9.122_f32 - x.cos();
        let mut acc = Accumulator314::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.169_f32 + y.sin();
        let b = y * 0.855_f32 - x.cos();
        let mut acc = Accumulator314::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.889_f32 + y.sin();
        let b = y * 5.409_f32 - x.cos();
        let mut acc = Accumulator314::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.824_f32 + y.sin();
        let b = y * 0.534_f32 - x.cos();
        let mut acc = Accumulator314::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.139_f32 + y.sin();
        let b = y * 6.064_f32 - x.cos();
        let mut acc = Accumulator314::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.169_f32 + y.sin();
        let b = y * 1.005_f32 - x.cos();
        let mut acc = Accumulator314::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.859_f32 + y.sin();
        let b = y * 1.646_f32 - x.cos();
        let mut acc = Accumulator314::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.831_f32 + y.sin();
        let b = y * 1.864_f32 - x.cos();
        let mut acc = Accumulator314::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.675_f32 + y.sin();
        let b = y * 3.315_f32 - x.cos();
        let mut acc = Accumulator314::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.372_f32 + y.sin();
        let b = y * 3.531_f32 - x.cos();
        let mut acc = Accumulator314::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.451_f32 + y.sin();
        let b = y * 1.901_f32 - x.cos();
        let mut acc = Accumulator314::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.826_f32 + y.sin();
        let b = y * 3.246_f32 - x.cos();
        let mut acc = Accumulator314::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.262_f32 + y.sin();
        let b = y * 7.095_f32 - x.cos();
        let mut acc = Accumulator314::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.714_f32 + y.sin();
        let b = y * 9.586_f32 - x.cos();
        let mut acc = Accumulator314::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.238_f32 + y.sin();
        let b = y * 2.267_f32 - x.cos();
        let mut acc = Accumulator314::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.174_f32 + y.sin();
        let b = y * 6.181_f32 - x.cos();
        let mut acc = Accumulator314::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.008_f32 + y.sin();
        let b = y * 3.655_f32 - x.cos();
        let mut acc = Accumulator314::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_314(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_314() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_314(total as u64) % 997) as f32;
        total
    }
}

pub mod m315 {
    use super::*;

    pub struct Accumulator315<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator315<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.073_f32 + y.sin();
        let b = y * 6.585_f32 - x.cos();
        let mut acc = Accumulator315::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.799_f32 + y.sin();
        let b = y * 5.475_f32 - x.cos();
        let mut acc = Accumulator315::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.025_f32 + y.sin();
        let b = y * 3.414_f32 - x.cos();
        let mut acc = Accumulator315::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.982_f32 + y.sin();
        let b = y * 7.426_f32 - x.cos();
        let mut acc = Accumulator315::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.52_f32 + y.sin();
        let b = y * 8.921_f32 - x.cos();
        let mut acc = Accumulator315::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.634_f32 + y.sin();
        let b = y * 7.418_f32 - x.cos();
        let mut acc = Accumulator315::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.494_f32 + y.sin();
        let b = y * 1.271_f32 - x.cos();
        let mut acc = Accumulator315::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.773_f32 + y.sin();
        let b = y * 6.93_f32 - x.cos();
        let mut acc = Accumulator315::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.898_f32 + y.sin();
        let b = y * 2.77_f32 - x.cos();
        let mut acc = Accumulator315::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.176_f32 + y.sin();
        let b = y * 8.604_f32 - x.cos();
        let mut acc = Accumulator315::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.986_f32 + y.sin();
        let b = y * 8.595_f32 - x.cos();
        let mut acc = Accumulator315::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.379_f32 + y.sin();
        let b = y * 2.04_f32 - x.cos();
        let mut acc = Accumulator315::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.515_f32 + y.sin();
        let b = y * 7.657_f32 - x.cos();
        let mut acc = Accumulator315::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.684_f32 + y.sin();
        let b = y * 2.692_f32 - x.cos();
        let mut acc = Accumulator315::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.564_f32 + y.sin();
        let b = y * 4.148_f32 - x.cos();
        let mut acc = Accumulator315::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.76_f32 + y.sin();
        let b = y * 3.092_f32 - x.cos();
        let mut acc = Accumulator315::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.65_f32 + y.sin();
        let b = y * 9.89_f32 - x.cos();
        let mut acc = Accumulator315::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.108_f32 + y.sin();
        let b = y * 0.203_f32 - x.cos();
        let mut acc = Accumulator315::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.354_f32 + y.sin();
        let b = y * 3.915_f32 - x.cos();
        let mut acc = Accumulator315::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.73_f32 + y.sin();
        let b = y * 0.791_f32 - x.cos();
        let mut acc = Accumulator315::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_315(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_315() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_315(total as u64) % 997) as f32;
        total
    }
}

pub mod m316 {
    use super::*;

    pub struct Accumulator316<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator316<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.036_f32 + y.sin();
        let b = y * 1.939_f32 - x.cos();
        let mut acc = Accumulator316::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.099_f32 + y.sin();
        let b = y * 4.989_f32 - x.cos();
        let mut acc = Accumulator316::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.746_f32 + y.sin();
        let b = y * 5.5_f32 - x.cos();
        let mut acc = Accumulator316::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.16_f32 + y.sin();
        let b = y * 4.238_f32 - x.cos();
        let mut acc = Accumulator316::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.777_f32 + y.sin();
        let b = y * 2.381_f32 - x.cos();
        let mut acc = Accumulator316::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.561_f32 + y.sin();
        let b = y * 0.339_f32 - x.cos();
        let mut acc = Accumulator316::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.204_f32 + y.sin();
        let b = y * 7.769_f32 - x.cos();
        let mut acc = Accumulator316::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.012_f32 + y.sin();
        let b = y * 7.999_f32 - x.cos();
        let mut acc = Accumulator316::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.379_f32 + y.sin();
        let b = y * 2.955_f32 - x.cos();
        let mut acc = Accumulator316::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.072_f32 + y.sin();
        let b = y * 8.086_f32 - x.cos();
        let mut acc = Accumulator316::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.334_f32 + y.sin();
        let b = y * 0.206_f32 - x.cos();
        let mut acc = Accumulator316::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.363_f32 + y.sin();
        let b = y * 6.972_f32 - x.cos();
        let mut acc = Accumulator316::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.88_f32 + y.sin();
        let b = y * 5.305_f32 - x.cos();
        let mut acc = Accumulator316::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.152_f32 + y.sin();
        let b = y * 1.876_f32 - x.cos();
        let mut acc = Accumulator316::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.632_f32 + y.sin();
        let b = y * 9.655_f32 - x.cos();
        let mut acc = Accumulator316::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.588_f32 + y.sin();
        let b = y * 6.551_f32 - x.cos();
        let mut acc = Accumulator316::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.033_f32 + y.sin();
        let b = y * 7.053_f32 - x.cos();
        let mut acc = Accumulator316::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.283_f32 + y.sin();
        let b = y * 2.576_f32 - x.cos();
        let mut acc = Accumulator316::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.973_f32 + y.sin();
        let b = y * 3.678_f32 - x.cos();
        let mut acc = Accumulator316::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.929_f32 + y.sin();
        let b = y * 2.837_f32 - x.cos();
        let mut acc = Accumulator316::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_316(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m316-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_316() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_316(total as u64) % 997) as f32;
        total
    }
}

pub mod m317 {
    use super::*;

    pub struct Accumulator317<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator317<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.584_f32 + y.sin();
        let b = y * 2.93_f32 - x.cos();
        let mut acc = Accumulator317::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.712_f32 + y.sin();
        let b = y * 7.767_f32 - x.cos();
        let mut acc = Accumulator317::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.008_f32 + y.sin();
        let b = y * 8.624_f32 - x.cos();
        let mut acc = Accumulator317::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.132_f32 + y.sin();
        let b = y * 3.634_f32 - x.cos();
        let mut acc = Accumulator317::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.342_f32 + y.sin();
        let b = y * 2.246_f32 - x.cos();
        let mut acc = Accumulator317::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.388_f32 + y.sin();
        let b = y * 6.599_f32 - x.cos();
        let mut acc = Accumulator317::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.907_f32 + y.sin();
        let b = y * 6.122_f32 - x.cos();
        let mut acc = Accumulator317::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.996_f32 + y.sin();
        let b = y * 1.903_f32 - x.cos();
        let mut acc = Accumulator317::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.865_f32 + y.sin();
        let b = y * 5.188_f32 - x.cos();
        let mut acc = Accumulator317::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.731_f32 + y.sin();
        let b = y * 8.204_f32 - x.cos();
        let mut acc = Accumulator317::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.271_f32 + y.sin();
        let b = y * 8.803_f32 - x.cos();
        let mut acc = Accumulator317::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.024_f32 + y.sin();
        let b = y * 2.249_f32 - x.cos();
        let mut acc = Accumulator317::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.688_f32 + y.sin();
        let b = y * 0.531_f32 - x.cos();
        let mut acc = Accumulator317::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.169_f32 + y.sin();
        let b = y * 7.099_f32 - x.cos();
        let mut acc = Accumulator317::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.122_f32 + y.sin();
        let b = y * 2.596_f32 - x.cos();
        let mut acc = Accumulator317::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.722_f32 + y.sin();
        let b = y * 4.536_f32 - x.cos();
        let mut acc = Accumulator317::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.785_f32 + y.sin();
        let b = y * 4.205_f32 - x.cos();
        let mut acc = Accumulator317::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.832_f32 + y.sin();
        let b = y * 8.013_f32 - x.cos();
        let mut acc = Accumulator317::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.318_f32 + y.sin();
        let b = y * 0.517_f32 - x.cos();
        let mut acc = Accumulator317::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.155_f32 + y.sin();
        let b = y * 8.55_f32 - x.cos();
        let mut acc = Accumulator317::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_317(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_317() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_317(total as u64) % 997) as f32;
        total
    }
}

pub mod m318 {
    use super::*;

    pub struct Accumulator318<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator318<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.71_f32 + y.sin();
        let b = y * 6.795_f32 - x.cos();
        let mut acc = Accumulator318::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.843_f32 + y.sin();
        let b = y * 5.472_f32 - x.cos();
        let mut acc = Accumulator318::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.516_f32 + y.sin();
        let b = y * 4.587_f32 - x.cos();
        let mut acc = Accumulator318::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.527_f32 + y.sin();
        let b = y * 6.106_f32 - x.cos();
        let mut acc = Accumulator318::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.792_f32 + y.sin();
        let b = y * 2.034_f32 - x.cos();
        let mut acc = Accumulator318::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.88_f32 + y.sin();
        let b = y * 4.36_f32 - x.cos();
        let mut acc = Accumulator318::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.947_f32 + y.sin();
        let b = y * 9.692_f32 - x.cos();
        let mut acc = Accumulator318::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.116_f32 + y.sin();
        let b = y * 5.299_f32 - x.cos();
        let mut acc = Accumulator318::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.331_f32 + y.sin();
        let b = y * 2.837_f32 - x.cos();
        let mut acc = Accumulator318::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.214_f32 + y.sin();
        let b = y * 9.235_f32 - x.cos();
        let mut acc = Accumulator318::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.067_f32 + y.sin();
        let b = y * 9.184_f32 - x.cos();
        let mut acc = Accumulator318::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.229_f32 + y.sin();
        let b = y * 7.888_f32 - x.cos();
        let mut acc = Accumulator318::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.028_f32 + y.sin();
        let b = y * 1.125_f32 - x.cos();
        let mut acc = Accumulator318::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.921_f32 + y.sin();
        let b = y * 1.593_f32 - x.cos();
        let mut acc = Accumulator318::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.163_f32 + y.sin();
        let b = y * 7.679_f32 - x.cos();
        let mut acc = Accumulator318::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.365_f32 + y.sin();
        let b = y * 2.176_f32 - x.cos();
        let mut acc = Accumulator318::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.126_f32 + y.sin();
        let b = y * 5.805_f32 - x.cos();
        let mut acc = Accumulator318::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.847_f32 + y.sin();
        let b = y * 2.526_f32 - x.cos();
        let mut acc = Accumulator318::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.306_f32 + y.sin();
        let b = y * 8.773_f32 - x.cos();
        let mut acc = Accumulator318::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.593_f32 + y.sin();
        let b = y * 1.57_f32 - x.cos();
        let mut acc = Accumulator318::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_318(seed: u64) -> u64 {
        let re = Regex::new(r"m318-(\d+)").unwrap();
        let hay = format!("m318-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_318() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_318(total as u64) % 997) as f32;
        total
    }
}

pub mod m319 {
    use super::*;

    pub struct Accumulator319<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator319<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.732_f32 + y.sin();
        let b = y * 2.791_f32 - x.cos();
        let mut acc = Accumulator319::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.073_f32 + y.sin();
        let b = y * 9.818_f32 - x.cos();
        let mut acc = Accumulator319::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.246_f32 + y.sin();
        let b = y * 7.63_f32 - x.cos();
        let mut acc = Accumulator319::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.761_f32 + y.sin();
        let b = y * 4.473_f32 - x.cos();
        let mut acc = Accumulator319::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.687_f32 + y.sin();
        let b = y * 6.979_f32 - x.cos();
        let mut acc = Accumulator319::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.081_f32 + y.sin();
        let b = y * 7.783_f32 - x.cos();
        let mut acc = Accumulator319::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.862_f32 + y.sin();
        let b = y * 3.119_f32 - x.cos();
        let mut acc = Accumulator319::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.764_f32 + y.sin();
        let b = y * 6.934_f32 - x.cos();
        let mut acc = Accumulator319::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.574_f32 + y.sin();
        let b = y * 8.497_f32 - x.cos();
        let mut acc = Accumulator319::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.289_f32 + y.sin();
        let b = y * 9.877_f32 - x.cos();
        let mut acc = Accumulator319::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.401_f32 + y.sin();
        let b = y * 4.531_f32 - x.cos();
        let mut acc = Accumulator319::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.058_f32 + y.sin();
        let b = y * 4.413_f32 - x.cos();
        let mut acc = Accumulator319::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.576_f32 + y.sin();
        let b = y * 7.824_f32 - x.cos();
        let mut acc = Accumulator319::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.045_f32 + y.sin();
        let b = y * 7.809_f32 - x.cos();
        let mut acc = Accumulator319::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.069_f32 + y.sin();
        let b = y * 9.622_f32 - x.cos();
        let mut acc = Accumulator319::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.023_f32 + y.sin();
        let b = y * 4.121_f32 - x.cos();
        let mut acc = Accumulator319::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.188_f32 + y.sin();
        let b = y * 2.323_f32 - x.cos();
        let mut acc = Accumulator319::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.232_f32 + y.sin();
        let b = y * 7.038_f32 - x.cos();
        let mut acc = Accumulator319::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.378_f32 + y.sin();
        let b = y * 0.907_f32 - x.cos();
        let mut acc = Accumulator319::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.625_f32 + y.sin();
        let b = y * 6.803_f32 - x.cos();
        let mut acc = Accumulator319::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_319(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_319() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_319(total as u64) % 997) as f32;
        total
    }
}

pub mod m320 {
    use super::*;

    pub struct Accumulator320<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator320<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.303_f32 + y.sin();
        let b = y * 6.465_f32 - x.cos();
        let mut acc = Accumulator320::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.694_f32 + y.sin();
        let b = y * 5.337_f32 - x.cos();
        let mut acc = Accumulator320::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.014_f32 + y.sin();
        let b = y * 5.587_f32 - x.cos();
        let mut acc = Accumulator320::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.104_f32 + y.sin();
        let b = y * 7.026_f32 - x.cos();
        let mut acc = Accumulator320::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.916_f32 + y.sin();
        let b = y * 5.001_f32 - x.cos();
        let mut acc = Accumulator320::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.875_f32 + y.sin();
        let b = y * 0.637_f32 - x.cos();
        let mut acc = Accumulator320::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.547_f32 + y.sin();
        let b = y * 0.324_f32 - x.cos();
        let mut acc = Accumulator320::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.314_f32 + y.sin();
        let b = y * 7.161_f32 - x.cos();
        let mut acc = Accumulator320::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.343_f32 + y.sin();
        let b = y * 7.61_f32 - x.cos();
        let mut acc = Accumulator320::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.972_f32 + y.sin();
        let b = y * 3.41_f32 - x.cos();
        let mut acc = Accumulator320::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.157_f32 + y.sin();
        let b = y * 3.053_f32 - x.cos();
        let mut acc = Accumulator320::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.521_f32 + y.sin();
        let b = y * 6.846_f32 - x.cos();
        let mut acc = Accumulator320::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.081_f32 + y.sin();
        let b = y * 3.671_f32 - x.cos();
        let mut acc = Accumulator320::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.534_f32 + y.sin();
        let b = y * 4.241_f32 - x.cos();
        let mut acc = Accumulator320::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.346_f32 + y.sin();
        let b = y * 5.233_f32 - x.cos();
        let mut acc = Accumulator320::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.986_f32 + y.sin();
        let b = y * 8.077_f32 - x.cos();
        let mut acc = Accumulator320::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.897_f32 + y.sin();
        let b = y * 5.475_f32 - x.cos();
        let mut acc = Accumulator320::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.457_f32 + y.sin();
        let b = y * 8.02_f32 - x.cos();
        let mut acc = Accumulator320::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.708_f32 + y.sin();
        let b = y * 4.99_f32 - x.cos();
        let mut acc = Accumulator320::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.474_f32 + y.sin();
        let b = y * 9.433_f32 - x.cos();
        let mut acc = Accumulator320::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_320(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(320u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_320() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_320(total as u64) % 997) as f32;
        total
    }
}

pub mod m321 {
    use super::*;

    pub struct Accumulator321<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator321<T> {
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
        let b = y * 3.879_f32 - x.cos();
        let mut acc = Accumulator321::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.489_f32 + y.sin();
        let b = y * 7.151_f32 - x.cos();
        let mut acc = Accumulator321::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.579_f32 + y.sin();
        let b = y * 9.101_f32 - x.cos();
        let mut acc = Accumulator321::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.329_f32 + y.sin();
        let b = y * 6.578_f32 - x.cos();
        let mut acc = Accumulator321::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.198_f32 + y.sin();
        let b = y * 3.706_f32 - x.cos();
        let mut acc = Accumulator321::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.415_f32 + y.sin();
        let b = y * 3.472_f32 - x.cos();
        let mut acc = Accumulator321::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.966_f32 + y.sin();
        let b = y * 4.308_f32 - x.cos();
        let mut acc = Accumulator321::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.17_f32 + y.sin();
        let b = y * 4.578_f32 - x.cos();
        let mut acc = Accumulator321::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.342_f32 + y.sin();
        let b = y * 6.31_f32 - x.cos();
        let mut acc = Accumulator321::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.69_f32 + y.sin();
        let b = y * 0.392_f32 - x.cos();
        let mut acc = Accumulator321::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.838_f32 + y.sin();
        let b = y * 0.359_f32 - x.cos();
        let mut acc = Accumulator321::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.11_f32 + y.sin();
        let b = y * 3.074_f32 - x.cos();
        let mut acc = Accumulator321::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.859_f32 + y.sin();
        let b = y * 5.752_f32 - x.cos();
        let mut acc = Accumulator321::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.618_f32 + y.sin();
        let b = y * 6.238_f32 - x.cos();
        let mut acc = Accumulator321::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.154_f32 + y.sin();
        let b = y * 1.856_f32 - x.cos();
        let mut acc = Accumulator321::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.419_f32 + y.sin();
        let b = y * 2.495_f32 - x.cos();
        let mut acc = Accumulator321::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.455_f32 + y.sin();
        let b = y * 4.103_f32 - x.cos();
        let mut acc = Accumulator321::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.116_f32 + y.sin();
        let b = y * 9.629_f32 - x.cos();
        let mut acc = Accumulator321::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.139_f32 + y.sin();
        let b = y * 6.145_f32 - x.cos();
        let mut acc = Accumulator321::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.592_f32 + y.sin();
        let b = y * 9.808_f32 - x.cos();
        let mut acc = Accumulator321::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_321(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_321() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_321(total as u64) % 997) as f32;
        total
    }
}

pub mod m322 {
    use super::*;

    pub struct Accumulator322<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator322<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.448_f32 + y.sin();
        let b = y * 6.543_f32 - x.cos();
        let mut acc = Accumulator322::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.366_f32 + y.sin();
        let b = y * 4.942_f32 - x.cos();
        let mut acc = Accumulator322::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.268_f32 + y.sin();
        let b = y * 9.515_f32 - x.cos();
        let mut acc = Accumulator322::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.814_f32 + y.sin();
        let b = y * 9.152_f32 - x.cos();
        let mut acc = Accumulator322::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.11_f32 + y.sin();
        let b = y * 5.796_f32 - x.cos();
        let mut acc = Accumulator322::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.182_f32 + y.sin();
        let b = y * 2.843_f32 - x.cos();
        let mut acc = Accumulator322::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.709_f32 + y.sin();
        let b = y * 8.6_f32 - x.cos();
        let mut acc = Accumulator322::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.687_f32 + y.sin();
        let b = y * 8.755_f32 - x.cos();
        let mut acc = Accumulator322::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.819_f32 + y.sin();
        let b = y * 6.48_f32 - x.cos();
        let mut acc = Accumulator322::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.738_f32 + y.sin();
        let b = y * 1.29_f32 - x.cos();
        let mut acc = Accumulator322::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.871_f32 + y.sin();
        let b = y * 9.887_f32 - x.cos();
        let mut acc = Accumulator322::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.044_f32 + y.sin();
        let b = y * 8.671_f32 - x.cos();
        let mut acc = Accumulator322::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.835_f32 + y.sin();
        let b = y * 2.416_f32 - x.cos();
        let mut acc = Accumulator322::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.949_f32 + y.sin();
        let b = y * 0.588_f32 - x.cos();
        let mut acc = Accumulator322::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.642_f32 + y.sin();
        let b = y * 6.481_f32 - x.cos();
        let mut acc = Accumulator322::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.935_f32 + y.sin();
        let b = y * 1.132_f32 - x.cos();
        let mut acc = Accumulator322::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.854_f32 + y.sin();
        let b = y * 9.274_f32 - x.cos();
        let mut acc = Accumulator322::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.583_f32 + y.sin();
        let b = y * 7.933_f32 - x.cos();
        let mut acc = Accumulator322::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.563_f32 + y.sin();
        let b = y * 0.487_f32 - x.cos();
        let mut acc = Accumulator322::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.245_f32 + y.sin();
        let b = y * 3.921_f32 - x.cos();
        let mut acc = Accumulator322::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_322(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_322() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_322(total as u64) % 997) as f32;
        total
    }
}

pub mod m323 {
    use super::*;

    pub struct Accumulator323<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator323<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.702_f32 + y.sin();
        let b = y * 0.853_f32 - x.cos();
        let mut acc = Accumulator323::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.962_f32 + y.sin();
        let b = y * 0.54_f32 - x.cos();
        let mut acc = Accumulator323::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.348_f32 + y.sin();
        let b = y * 8.397_f32 - x.cos();
        let mut acc = Accumulator323::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.696_f32 + y.sin();
        let b = y * 1.266_f32 - x.cos();
        let mut acc = Accumulator323::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.799_f32 + y.sin();
        let b = y * 2.961_f32 - x.cos();
        let mut acc = Accumulator323::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.378_f32 + y.sin();
        let b = y * 1.186_f32 - x.cos();
        let mut acc = Accumulator323::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.127_f32 + y.sin();
        let b = y * 5.416_f32 - x.cos();
        let mut acc = Accumulator323::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.901_f32 + y.sin();
        let b = y * 2.587_f32 - x.cos();
        let mut acc = Accumulator323::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.517_f32 + y.sin();
        let b = y * 9.532_f32 - x.cos();
        let mut acc = Accumulator323::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.159_f32 + y.sin();
        let b = y * 2.592_f32 - x.cos();
        let mut acc = Accumulator323::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.362_f32 + y.sin();
        let b = y * 0.424_f32 - x.cos();
        let mut acc = Accumulator323::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.706_f32 + y.sin();
        let b = y * 2.269_f32 - x.cos();
        let mut acc = Accumulator323::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.242_f32 + y.sin();
        let b = y * 7.953_f32 - x.cos();
        let mut acc = Accumulator323::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.704_f32 + y.sin();
        let b = y * 4.565_f32 - x.cos();
        let mut acc = Accumulator323::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.29_f32 + y.sin();
        let b = y * 5.414_f32 - x.cos();
        let mut acc = Accumulator323::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.109_f32 + y.sin();
        let b = y * 6.369_f32 - x.cos();
        let mut acc = Accumulator323::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.672_f32 + y.sin();
        let b = y * 1.161_f32 - x.cos();
        let mut acc = Accumulator323::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.3_f32 + y.sin();
        let b = y * 0.914_f32 - x.cos();
        let mut acc = Accumulator323::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.819_f32 + y.sin();
        let b = y * 5.526_f32 - x.cos();
        let mut acc = Accumulator323::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.795_f32 + y.sin();
        let b = y * 1.935_f32 - x.cos();
        let mut acc = Accumulator323::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_323(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m323-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_323() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_323(total as u64) % 997) as f32;
        total
    }
}

pub mod m324 {
    use super::*;

    pub struct Accumulator324<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator324<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.803_f32 + y.sin();
        let b = y * 8.726_f32 - x.cos();
        let mut acc = Accumulator324::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.406_f32 + y.sin();
        let b = y * 7.674_f32 - x.cos();
        let mut acc = Accumulator324::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.002_f32 + y.sin();
        let b = y * 9.516_f32 - x.cos();
        let mut acc = Accumulator324::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.183_f32 + y.sin();
        let b = y * 3.678_f32 - x.cos();
        let mut acc = Accumulator324::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.299_f32 + y.sin();
        let b = y * 3.624_f32 - x.cos();
        let mut acc = Accumulator324::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.927_f32 + y.sin();
        let b = y * 6.039_f32 - x.cos();
        let mut acc = Accumulator324::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.889_f32 + y.sin();
        let b = y * 2.12_f32 - x.cos();
        let mut acc = Accumulator324::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.85_f32 + y.sin();
        let b = y * 6.147_f32 - x.cos();
        let mut acc = Accumulator324::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.306_f32 + y.sin();
        let b = y * 1.591_f32 - x.cos();
        let mut acc = Accumulator324::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.447_f32 + y.sin();
        let b = y * 7.748_f32 - x.cos();
        let mut acc = Accumulator324::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.189_f32 + y.sin();
        let b = y * 3.453_f32 - x.cos();
        let mut acc = Accumulator324::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.806_f32 + y.sin();
        let b = y * 0.993_f32 - x.cos();
        let mut acc = Accumulator324::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.659_f32 + y.sin();
        let b = y * 2.757_f32 - x.cos();
        let mut acc = Accumulator324::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.579_f32 + y.sin();
        let b = y * 6.015_f32 - x.cos();
        let mut acc = Accumulator324::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.686_f32 + y.sin();
        let b = y * 3.415_f32 - x.cos();
        let mut acc = Accumulator324::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.571_f32 + y.sin();
        let b = y * 1.535_f32 - x.cos();
        let mut acc = Accumulator324::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.802_f32 + y.sin();
        let b = y * 3.31_f32 - x.cos();
        let mut acc = Accumulator324::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.683_f32 + y.sin();
        let b = y * 0.408_f32 - x.cos();
        let mut acc = Accumulator324::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.795_f32 + y.sin();
        let b = y * 0.528_f32 - x.cos();
        let mut acc = Accumulator324::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.914_f32 + y.sin();
        let b = y * 3.647_f32 - x.cos();
        let mut acc = Accumulator324::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_324(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_324() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_324(total as u64) % 997) as f32;
        total
    }
}

pub mod m325 {
    use super::*;

    pub struct Accumulator325<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator325<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.3_f32 + y.sin();
        let b = y * 1.664_f32 - x.cos();
        let mut acc = Accumulator325::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.566_f32 + y.sin();
        let b = y * 6.23_f32 - x.cos();
        let mut acc = Accumulator325::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.166_f32 + y.sin();
        let b = y * 0.158_f32 - x.cos();
        let mut acc = Accumulator325::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.132_f32 + y.sin();
        let b = y * 8.788_f32 - x.cos();
        let mut acc = Accumulator325::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.918_f32 + y.sin();
        let b = y * 8.587_f32 - x.cos();
        let mut acc = Accumulator325::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.15_f32 + y.sin();
        let b = y * 2.054_f32 - x.cos();
        let mut acc = Accumulator325::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.14_f32 + y.sin();
        let b = y * 2.358_f32 - x.cos();
        let mut acc = Accumulator325::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.315_f32 + y.sin();
        let b = y * 0.507_f32 - x.cos();
        let mut acc = Accumulator325::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.694_f32 + y.sin();
        let b = y * 6.171_f32 - x.cos();
        let mut acc = Accumulator325::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.256_f32 + y.sin();
        let b = y * 0.146_f32 - x.cos();
        let mut acc = Accumulator325::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.424_f32 + y.sin();
        let b = y * 9.666_f32 - x.cos();
        let mut acc = Accumulator325::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.679_f32 + y.sin();
        let b = y * 6.976_f32 - x.cos();
        let mut acc = Accumulator325::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.779_f32 + y.sin();
        let b = y * 2.461_f32 - x.cos();
        let mut acc = Accumulator325::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.775_f32 + y.sin();
        let b = y * 6.256_f32 - x.cos();
        let mut acc = Accumulator325::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.21_f32 + y.sin();
        let b = y * 9.632_f32 - x.cos();
        let mut acc = Accumulator325::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.394_f32 + y.sin();
        let b = y * 6.95_f32 - x.cos();
        let mut acc = Accumulator325::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.754_f32 + y.sin();
        let b = y * 3.885_f32 - x.cos();
        let mut acc = Accumulator325::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.219_f32 + y.sin();
        let b = y * 9.068_f32 - x.cos();
        let mut acc = Accumulator325::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.968_f32 + y.sin();
        let b = y * 9.277_f32 - x.cos();
        let mut acc = Accumulator325::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.661_f32 + y.sin();
        let b = y * 9.655_f32 - x.cos();
        let mut acc = Accumulator325::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_325(seed: u64) -> u64 {
        let re = Regex::new(r"m325-(\d+)").unwrap();
        let hay = format!("m325-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_325() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_325(total as u64) % 997) as f32;
        total
    }
}

pub mod m326 {
    use super::*;

    pub struct Accumulator326<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator326<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.103_f32 + y.sin();
        let b = y * 2.142_f32 - x.cos();
        let mut acc = Accumulator326::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.858_f32 + y.sin();
        let b = y * 4.399_f32 - x.cos();
        let mut acc = Accumulator326::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.419_f32 + y.sin();
        let b = y * 0.794_f32 - x.cos();
        let mut acc = Accumulator326::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.113_f32 + y.sin();
        let b = y * 7.587_f32 - x.cos();
        let mut acc = Accumulator326::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.105_f32 + y.sin();
        let b = y * 5.319_f32 - x.cos();
        let mut acc = Accumulator326::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.347_f32 + y.sin();
        let b = y * 2.849_f32 - x.cos();
        let mut acc = Accumulator326::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.572_f32 + y.sin();
        let b = y * 5.277_f32 - x.cos();
        let mut acc = Accumulator326::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.758_f32 + y.sin();
        let b = y * 4.87_f32 - x.cos();
        let mut acc = Accumulator326::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.779_f32 + y.sin();
        let b = y * 5.859_f32 - x.cos();
        let mut acc = Accumulator326::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.756_f32 + y.sin();
        let b = y * 0.83_f32 - x.cos();
        let mut acc = Accumulator326::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.835_f32 + y.sin();
        let b = y * 1.803_f32 - x.cos();
        let mut acc = Accumulator326::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.376_f32 + y.sin();
        let b = y * 7.588_f32 - x.cos();
        let mut acc = Accumulator326::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.307_f32 + y.sin();
        let b = y * 1.108_f32 - x.cos();
        let mut acc = Accumulator326::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.738_f32 + y.sin();
        let b = y * 6.198_f32 - x.cos();
        let mut acc = Accumulator326::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.789_f32 + y.sin();
        let b = y * 8.332_f32 - x.cos();
        let mut acc = Accumulator326::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.553_f32 + y.sin();
        let b = y * 6.335_f32 - x.cos();
        let mut acc = Accumulator326::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.841_f32 + y.sin();
        let b = y * 8.558_f32 - x.cos();
        let mut acc = Accumulator326::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.139_f32 + y.sin();
        let b = y * 8.692_f32 - x.cos();
        let mut acc = Accumulator326::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.917_f32 + y.sin();
        let b = y * 2.163_f32 - x.cos();
        let mut acc = Accumulator326::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.212_f32 + y.sin();
        let b = y * 5.698_f32 - x.cos();
        let mut acc = Accumulator326::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_326(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_326() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_326(total as u64) % 997) as f32;
        total
    }
}

pub mod m327 {
    use super::*;

    pub struct Accumulator327<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator327<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.428_f32 + y.sin();
        let b = y * 7.344_f32 - x.cos();
        let mut acc = Accumulator327::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.846_f32 + y.sin();
        let b = y * 5.041_f32 - x.cos();
        let mut acc = Accumulator327::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.7_f32 + y.sin();
        let b = y * 9.656_f32 - x.cos();
        let mut acc = Accumulator327::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.328_f32 + y.sin();
        let b = y * 5.462_f32 - x.cos();
        let mut acc = Accumulator327::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.285_f32 + y.sin();
        let b = y * 3.115_f32 - x.cos();
        let mut acc = Accumulator327::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.365_f32 + y.sin();
        let b = y * 8.594_f32 - x.cos();
        let mut acc = Accumulator327::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.225_f32 + y.sin();
        let b = y * 6.827_f32 - x.cos();
        let mut acc = Accumulator327::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.46_f32 + y.sin();
        let b = y * 5.18_f32 - x.cos();
        let mut acc = Accumulator327::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.355_f32 + y.sin();
        let b = y * 6.7_f32 - x.cos();
        let mut acc = Accumulator327::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.322_f32 + y.sin();
        let b = y * 6.375_f32 - x.cos();
        let mut acc = Accumulator327::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.623_f32 + y.sin();
        let b = y * 8.041_f32 - x.cos();
        let mut acc = Accumulator327::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.56_f32 + y.sin();
        let b = y * 3.936_f32 - x.cos();
        let mut acc = Accumulator327::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.638_f32 + y.sin();
        let b = y * 4.582_f32 - x.cos();
        let mut acc = Accumulator327::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.197_f32 + y.sin();
        let b = y * 6.351_f32 - x.cos();
        let mut acc = Accumulator327::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.028_f32 + y.sin();
        let b = y * 5.688_f32 - x.cos();
        let mut acc = Accumulator327::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.764_f32 + y.sin();
        let b = y * 8.543_f32 - x.cos();
        let mut acc = Accumulator327::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.686_f32 + y.sin();
        let b = y * 0.96_f32 - x.cos();
        let mut acc = Accumulator327::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.612_f32 + y.sin();
        let b = y * 8.279_f32 - x.cos();
        let mut acc = Accumulator327::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.766_f32 + y.sin();
        let b = y * 1.437_f32 - x.cos();
        let mut acc = Accumulator327::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.512_f32 + y.sin();
        let b = y * 3.873_f32 - x.cos();
        let mut acc = Accumulator327::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_327(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(327u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_327() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_327(total as u64) % 997) as f32;
        total
    }
}

pub mod m328 {
    use super::*;

    pub struct Accumulator328<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator328<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.659_f32 + y.sin();
        let b = y * 1.716_f32 - x.cos();
        let mut acc = Accumulator328::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.812_f32 + y.sin();
        let b = y * 9.43_f32 - x.cos();
        let mut acc = Accumulator328::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.283_f32 + y.sin();
        let b = y * 7.425_f32 - x.cos();
        let mut acc = Accumulator328::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.831_f32 + y.sin();
        let b = y * 4.611_f32 - x.cos();
        let mut acc = Accumulator328::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.325_f32 + y.sin();
        let b = y * 9.863_f32 - x.cos();
        let mut acc = Accumulator328::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.011_f32 + y.sin();
        let b = y * 8.036_f32 - x.cos();
        let mut acc = Accumulator328::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.549_f32 + y.sin();
        let b = y * 4.114_f32 - x.cos();
        let mut acc = Accumulator328::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.327_f32 + y.sin();
        let b = y * 5.565_f32 - x.cos();
        let mut acc = Accumulator328::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.205_f32 + y.sin();
        let b = y * 8.502_f32 - x.cos();
        let mut acc = Accumulator328::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.549_f32 + y.sin();
        let b = y * 8.065_f32 - x.cos();
        let mut acc = Accumulator328::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.461_f32 + y.sin();
        let b = y * 4.022_f32 - x.cos();
        let mut acc = Accumulator328::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.282_f32 + y.sin();
        let b = y * 6.13_f32 - x.cos();
        let mut acc = Accumulator328::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.135_f32 + y.sin();
        let b = y * 3.954_f32 - x.cos();
        let mut acc = Accumulator328::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.745_f32 + y.sin();
        let b = y * 0.532_f32 - x.cos();
        let mut acc = Accumulator328::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.026_f32 + y.sin();
        let b = y * 8.198_f32 - x.cos();
        let mut acc = Accumulator328::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.709_f32 + y.sin();
        let b = y * 6.867_f32 - x.cos();
        let mut acc = Accumulator328::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.09_f32 + y.sin();
        let b = y * 2.387_f32 - x.cos();
        let mut acc = Accumulator328::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.951_f32 + y.sin();
        let b = y * 5.901_f32 - x.cos();
        let mut acc = Accumulator328::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.836_f32 + y.sin();
        let b = y * 0.385_f32 - x.cos();
        let mut acc = Accumulator328::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.781_f32 + y.sin();
        let b = y * 2.014_f32 - x.cos();
        let mut acc = Accumulator328::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_328(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_328() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_328(total as u64) % 997) as f32;
        total
    }
}

pub mod m329 {
    use super::*;

    pub struct Accumulator329<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator329<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.614_f32 + y.sin();
        let b = y * 5.598_f32 - x.cos();
        let mut acc = Accumulator329::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.408_f32 + y.sin();
        let b = y * 6.11_f32 - x.cos();
        let mut acc = Accumulator329::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.867_f32 + y.sin();
        let b = y * 8.899_f32 - x.cos();
        let mut acc = Accumulator329::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.975_f32 + y.sin();
        let b = y * 1.382_f32 - x.cos();
        let mut acc = Accumulator329::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.396_f32 + y.sin();
        let b = y * 5.325_f32 - x.cos();
        let mut acc = Accumulator329::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.924_f32 + y.sin();
        let b = y * 5.391_f32 - x.cos();
        let mut acc = Accumulator329::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.869_f32 + y.sin();
        let b = y * 5.178_f32 - x.cos();
        let mut acc = Accumulator329::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.009_f32 + y.sin();
        let b = y * 9.145_f32 - x.cos();
        let mut acc = Accumulator329::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.27_f32 + y.sin();
        let b = y * 1.111_f32 - x.cos();
        let mut acc = Accumulator329::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.71_f32 + y.sin();
        let b = y * 6.228_f32 - x.cos();
        let mut acc = Accumulator329::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.234_f32 + y.sin();
        let b = y * 7.043_f32 - x.cos();
        let mut acc = Accumulator329::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.577_f32 + y.sin();
        let b = y * 6.364_f32 - x.cos();
        let mut acc = Accumulator329::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.238_f32 + y.sin();
        let b = y * 8.778_f32 - x.cos();
        let mut acc = Accumulator329::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.605_f32 + y.sin();
        let b = y * 9.683_f32 - x.cos();
        let mut acc = Accumulator329::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.733_f32 + y.sin();
        let b = y * 3.684_f32 - x.cos();
        let mut acc = Accumulator329::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.701_f32 + y.sin();
        let b = y * 9.865_f32 - x.cos();
        let mut acc = Accumulator329::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.1_f32 + y.sin();
        let b = y * 5.141_f32 - x.cos();
        let mut acc = Accumulator329::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.696_f32 + y.sin();
        let b = y * 8.01_f32 - x.cos();
        let mut acc = Accumulator329::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.447_f32 + y.sin();
        let b = y * 2.392_f32 - x.cos();
        let mut acc = Accumulator329::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.674_f32 + y.sin();
        let b = y * 6.667_f32 - x.cos();
        let mut acc = Accumulator329::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_329(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_329() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_329(total as u64) % 997) as f32;
        total
    }
}

pub mod m330 {
    use super::*;

    pub struct Accumulator330<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator330<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.983_f32 + y.sin();
        let b = y * 7.295_f32 - x.cos();
        let mut acc = Accumulator330::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.508_f32 + y.sin();
        let b = y * 6.985_f32 - x.cos();
        let mut acc = Accumulator330::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.012_f32 + y.sin();
        let b = y * 1.901_f32 - x.cos();
        let mut acc = Accumulator330::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.632_f32 + y.sin();
        let b = y * 7.74_f32 - x.cos();
        let mut acc = Accumulator330::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.937_f32 + y.sin();
        let b = y * 3.558_f32 - x.cos();
        let mut acc = Accumulator330::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.424_f32 + y.sin();
        let b = y * 7.667_f32 - x.cos();
        let mut acc = Accumulator330::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.295_f32 + y.sin();
        let b = y * 5.635_f32 - x.cos();
        let mut acc = Accumulator330::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.72_f32 + y.sin();
        let b = y * 8.859_f32 - x.cos();
        let mut acc = Accumulator330::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.612_f32 + y.sin();
        let b = y * 2.11_f32 - x.cos();
        let mut acc = Accumulator330::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.327_f32 + y.sin();
        let b = y * 2.083_f32 - x.cos();
        let mut acc = Accumulator330::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.103_f32 + y.sin();
        let b = y * 5.734_f32 - x.cos();
        let mut acc = Accumulator330::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.93_f32 + y.sin();
        let b = y * 0.432_f32 - x.cos();
        let mut acc = Accumulator330::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.008_f32 + y.sin();
        let b = y * 4.211_f32 - x.cos();
        let mut acc = Accumulator330::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.332_f32 + y.sin();
        let b = y * 5.126_f32 - x.cos();
        let mut acc = Accumulator330::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.711_f32 + y.sin();
        let b = y * 4.914_f32 - x.cos();
        let mut acc = Accumulator330::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.32_f32 + y.sin();
        let b = y * 8.992_f32 - x.cos();
        let mut acc = Accumulator330::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.948_f32 + y.sin();
        let b = y * 4.661_f32 - x.cos();
        let mut acc = Accumulator330::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.746_f32 + y.sin();
        let b = y * 4.672_f32 - x.cos();
        let mut acc = Accumulator330::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.911_f32 + y.sin();
        let b = y * 5.288_f32 - x.cos();
        let mut acc = Accumulator330::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.293_f32 + y.sin();
        let b = y * 9.118_f32 - x.cos();
        let mut acc = Accumulator330::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_330(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m330-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_330() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_330(total as u64) % 997) as f32;
        total
    }
}

pub mod m331 {
    use super::*;

    pub struct Accumulator331<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator331<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.581_f32 + y.sin();
        let b = y * 5.189_f32 - x.cos();
        let mut acc = Accumulator331::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.594_f32 + y.sin();
        let b = y * 7.809_f32 - x.cos();
        let mut acc = Accumulator331::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.609_f32 + y.sin();
        let b = y * 9.288_f32 - x.cos();
        let mut acc = Accumulator331::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.556_f32 + y.sin();
        let b = y * 1.741_f32 - x.cos();
        let mut acc = Accumulator331::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.927_f32 + y.sin();
        let b = y * 6.381_f32 - x.cos();
        let mut acc = Accumulator331::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.062_f32 + y.sin();
        let b = y * 2.925_f32 - x.cos();
        let mut acc = Accumulator331::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.912_f32 + y.sin();
        let b = y * 1.337_f32 - x.cos();
        let mut acc = Accumulator331::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.358_f32 + y.sin();
        let b = y * 6.977_f32 - x.cos();
        let mut acc = Accumulator331::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.622_f32 + y.sin();
        let b = y * 9.833_f32 - x.cos();
        let mut acc = Accumulator331::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.008_f32 + y.sin();
        let b = y * 2.987_f32 - x.cos();
        let mut acc = Accumulator331::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.591_f32 + y.sin();
        let b = y * 5.952_f32 - x.cos();
        let mut acc = Accumulator331::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.68_f32 + y.sin();
        let b = y * 7.318_f32 - x.cos();
        let mut acc = Accumulator331::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.454_f32 + y.sin();
        let b = y * 1.399_f32 - x.cos();
        let mut acc = Accumulator331::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.157_f32 + y.sin();
        let b = y * 9.317_f32 - x.cos();
        let mut acc = Accumulator331::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.032_f32 + y.sin();
        let b = y * 7.111_f32 - x.cos();
        let mut acc = Accumulator331::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.455_f32 + y.sin();
        let b = y * 9.589_f32 - x.cos();
        let mut acc = Accumulator331::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.391_f32 + y.sin();
        let b = y * 5.931_f32 - x.cos();
        let mut acc = Accumulator331::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.782_f32 + y.sin();
        let b = y * 0.161_f32 - x.cos();
        let mut acc = Accumulator331::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.187_f32 + y.sin();
        let b = y * 6.704_f32 - x.cos();
        let mut acc = Accumulator331::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.499_f32 + y.sin();
        let b = y * 3.808_f32 - x.cos();
        let mut acc = Accumulator331::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_331(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_331() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_331(total as u64) % 997) as f32;
        total
    }
}

pub mod m332 {
    use super::*;

    pub struct Accumulator332<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator332<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.263_f32 + y.sin();
        let b = y * 8.661_f32 - x.cos();
        let mut acc = Accumulator332::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.696_f32 + y.sin();
        let b = y * 4.741_f32 - x.cos();
        let mut acc = Accumulator332::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.207_f32 + y.sin();
        let b = y * 6.097_f32 - x.cos();
        let mut acc = Accumulator332::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.917_f32 + y.sin();
        let b = y * 4.342_f32 - x.cos();
        let mut acc = Accumulator332::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.752_f32 + y.sin();
        let b = y * 8.614_f32 - x.cos();
        let mut acc = Accumulator332::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.354_f32 + y.sin();
        let b = y * 6.174_f32 - x.cos();
        let mut acc = Accumulator332::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.984_f32 + y.sin();
        let b = y * 1.507_f32 - x.cos();
        let mut acc = Accumulator332::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.932_f32 + y.sin();
        let b = y * 4.011_f32 - x.cos();
        let mut acc = Accumulator332::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.036_f32 + y.sin();
        let b = y * 8.256_f32 - x.cos();
        let mut acc = Accumulator332::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.827_f32 + y.sin();
        let b = y * 4.497_f32 - x.cos();
        let mut acc = Accumulator332::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.341_f32 + y.sin();
        let b = y * 9.834_f32 - x.cos();
        let mut acc = Accumulator332::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.648_f32 + y.sin();
        let b = y * 2.264_f32 - x.cos();
        let mut acc = Accumulator332::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.686_f32 + y.sin();
        let b = y * 1.657_f32 - x.cos();
        let mut acc = Accumulator332::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.852_f32 + y.sin();
        let b = y * 4.463_f32 - x.cos();
        let mut acc = Accumulator332::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.812_f32 + y.sin();
        let b = y * 8.391_f32 - x.cos();
        let mut acc = Accumulator332::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.481_f32 + y.sin();
        let b = y * 2.554_f32 - x.cos();
        let mut acc = Accumulator332::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.976_f32 + y.sin();
        let b = y * 0.577_f32 - x.cos();
        let mut acc = Accumulator332::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.291_f32 + y.sin();
        let b = y * 2.585_f32 - x.cos();
        let mut acc = Accumulator332::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.778_f32 + y.sin();
        let b = y * 5.589_f32 - x.cos();
        let mut acc = Accumulator332::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.608_f32 + y.sin();
        let b = y * 3.176_f32 - x.cos();
        let mut acc = Accumulator332::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_332(seed: u64) -> u64 {
        let re = Regex::new(r"m332-(\d+)").unwrap();
        let hay = format!("m332-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_332() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_332(total as u64) % 997) as f32;
        total
    }
}

pub mod m333 {
    use super::*;

    pub struct Accumulator333<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator333<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.84_f32 + y.sin();
        let b = y * 4.676_f32 - x.cos();
        let mut acc = Accumulator333::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.021_f32 + y.sin();
        let b = y * 4.379_f32 - x.cos();
        let mut acc = Accumulator333::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.518_f32 + y.sin();
        let b = y * 9.27_f32 - x.cos();
        let mut acc = Accumulator333::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.66_f32 + y.sin();
        let b = y * 9.376_f32 - x.cos();
        let mut acc = Accumulator333::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.659_f32 + y.sin();
        let b = y * 9.313_f32 - x.cos();
        let mut acc = Accumulator333::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.1_f32 + y.sin();
        let b = y * 4.601_f32 - x.cos();
        let mut acc = Accumulator333::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.89_f32 + y.sin();
        let b = y * 7.787_f32 - x.cos();
        let mut acc = Accumulator333::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.364_f32 + y.sin();
        let b = y * 1.839_f32 - x.cos();
        let mut acc = Accumulator333::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.563_f32 + y.sin();
        let b = y * 8.656_f32 - x.cos();
        let mut acc = Accumulator333::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.452_f32 + y.sin();
        let b = y * 9.864_f32 - x.cos();
        let mut acc = Accumulator333::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.062_f32 + y.sin();
        let b = y * 8.535_f32 - x.cos();
        let mut acc = Accumulator333::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.986_f32 + y.sin();
        let b = y * 1.23_f32 - x.cos();
        let mut acc = Accumulator333::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.493_f32 + y.sin();
        let b = y * 9.002_f32 - x.cos();
        let mut acc = Accumulator333::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.638_f32 + y.sin();
        let b = y * 9.12_f32 - x.cos();
        let mut acc = Accumulator333::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.321_f32 + y.sin();
        let b = y * 6.068_f32 - x.cos();
        let mut acc = Accumulator333::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.092_f32 + y.sin();
        let b = y * 1.985_f32 - x.cos();
        let mut acc = Accumulator333::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.437_f32 + y.sin();
        let b = y * 3.619_f32 - x.cos();
        let mut acc = Accumulator333::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.172_f32 + y.sin();
        let b = y * 8.297_f32 - x.cos();
        let mut acc = Accumulator333::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.99_f32 + y.sin();
        let b = y * 3.086_f32 - x.cos();
        let mut acc = Accumulator333::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.756_f32 + y.sin();
        let b = y * 1.652_f32 - x.cos();
        let mut acc = Accumulator333::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_333(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_333() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_333(total as u64) % 997) as f32;
        total
    }
}

pub mod m334 {
    use super::*;

    pub struct Accumulator334<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator334<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.34_f32 + y.sin();
        let b = y * 7.058_f32 - x.cos();
        let mut acc = Accumulator334::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.911_f32 + y.sin();
        let b = y * 7.512_f32 - x.cos();
        let mut acc = Accumulator334::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.465_f32 + y.sin();
        let b = y * 9.752_f32 - x.cos();
        let mut acc = Accumulator334::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.978_f32 + y.sin();
        let b = y * 0.596_f32 - x.cos();
        let mut acc = Accumulator334::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.303_f32 + y.sin();
        let b = y * 3.341_f32 - x.cos();
        let mut acc = Accumulator334::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.08_f32 + y.sin();
        let b = y * 6.358_f32 - x.cos();
        let mut acc = Accumulator334::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.687_f32 + y.sin();
        let b = y * 8.583_f32 - x.cos();
        let mut acc = Accumulator334::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.509_f32 + y.sin();
        let b = y * 2.85_f32 - x.cos();
        let mut acc = Accumulator334::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.742_f32 + y.sin();
        let b = y * 8.291_f32 - x.cos();
        let mut acc = Accumulator334::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.241_f32 + y.sin();
        let b = y * 9.134_f32 - x.cos();
        let mut acc = Accumulator334::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.543_f32 + y.sin();
        let b = y * 9.635_f32 - x.cos();
        let mut acc = Accumulator334::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.156_f32 + y.sin();
        let b = y * 3.325_f32 - x.cos();
        let mut acc = Accumulator334::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.067_f32 + y.sin();
        let b = y * 7.632_f32 - x.cos();
        let mut acc = Accumulator334::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.302_f32 + y.sin();
        let b = y * 9.83_f32 - x.cos();
        let mut acc = Accumulator334::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.71_f32 + y.sin();
        let b = y * 3.852_f32 - x.cos();
        let mut acc = Accumulator334::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.425_f32 + y.sin();
        let b = y * 7.683_f32 - x.cos();
        let mut acc = Accumulator334::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.732_f32 + y.sin();
        let b = y * 6.081_f32 - x.cos();
        let mut acc = Accumulator334::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.564_f32 + y.sin();
        let b = y * 5.641_f32 - x.cos();
        let mut acc = Accumulator334::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.709_f32 + y.sin();
        let b = y * 4.751_f32 - x.cos();
        let mut acc = Accumulator334::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.761_f32 + y.sin();
        let b = y * 3.739_f32 - x.cos();
        let mut acc = Accumulator334::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_334(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(334u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_334() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_334(total as u64) % 997) as f32;
        total
    }
}

pub mod m335 {
    use super::*;

    pub struct Accumulator335<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator335<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.62_f32 + y.sin();
        let b = y * 3.528_f32 - x.cos();
        let mut acc = Accumulator335::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.15_f32 + y.sin();
        let b = y * 2.321_f32 - x.cos();
        let mut acc = Accumulator335::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.289_f32 + y.sin();
        let b = y * 0.157_f32 - x.cos();
        let mut acc = Accumulator335::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.875_f32 + y.sin();
        let b = y * 6.669_f32 - x.cos();
        let mut acc = Accumulator335::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.813_f32 + y.sin();
        let b = y * 0.868_f32 - x.cos();
        let mut acc = Accumulator335::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.796_f32 + y.sin();
        let b = y * 8.889_f32 - x.cos();
        let mut acc = Accumulator335::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.407_f32 + y.sin();
        let b = y * 7.003_f32 - x.cos();
        let mut acc = Accumulator335::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.542_f32 + y.sin();
        let b = y * 1.879_f32 - x.cos();
        let mut acc = Accumulator335::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.38_f32 + y.sin();
        let b = y * 1.438_f32 - x.cos();
        let mut acc = Accumulator335::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.674_f32 + y.sin();
        let b = y * 0.56_f32 - x.cos();
        let mut acc = Accumulator335::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.984_f32 + y.sin();
        let b = y * 6.012_f32 - x.cos();
        let mut acc = Accumulator335::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.217_f32 + y.sin();
        let b = y * 1.271_f32 - x.cos();
        let mut acc = Accumulator335::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.195_f32 + y.sin();
        let b = y * 2.988_f32 - x.cos();
        let mut acc = Accumulator335::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.843_f32 + y.sin();
        let b = y * 8.855_f32 - x.cos();
        let mut acc = Accumulator335::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.172_f32 + y.sin();
        let b = y * 9.272_f32 - x.cos();
        let mut acc = Accumulator335::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.096_f32 + y.sin();
        let b = y * 8.775_f32 - x.cos();
        let mut acc = Accumulator335::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.492_f32 + y.sin();
        let b = y * 1.287_f32 - x.cos();
        let mut acc = Accumulator335::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.641_f32 + y.sin();
        let b = y * 5.564_f32 - x.cos();
        let mut acc = Accumulator335::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.215_f32 + y.sin();
        let b = y * 7.737_f32 - x.cos();
        let mut acc = Accumulator335::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.716_f32 + y.sin();
        let b = y * 2.269_f32 - x.cos();
        let mut acc = Accumulator335::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_335(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_335() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_335(total as u64) % 997) as f32;
        total
    }
}

pub mod m336 {
    use super::*;

    pub struct Accumulator336<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator336<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.164_f32 + y.sin();
        let b = y * 4.903_f32 - x.cos();
        let mut acc = Accumulator336::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.515_f32 + y.sin();
        let b = y * 2.949_f32 - x.cos();
        let mut acc = Accumulator336::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.573_f32 + y.sin();
        let b = y * 4.714_f32 - x.cos();
        let mut acc = Accumulator336::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.731_f32 + y.sin();
        let b = y * 4.532_f32 - x.cos();
        let mut acc = Accumulator336::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.283_f32 + y.sin();
        let b = y * 5.725_f32 - x.cos();
        let mut acc = Accumulator336::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.865_f32 + y.sin();
        let b = y * 7.633_f32 - x.cos();
        let mut acc = Accumulator336::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.876_f32 + y.sin();
        let b = y * 3.021_f32 - x.cos();
        let mut acc = Accumulator336::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.835_f32 + y.sin();
        let b = y * 7.746_f32 - x.cos();
        let mut acc = Accumulator336::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.09_f32 + y.sin();
        let b = y * 4.247_f32 - x.cos();
        let mut acc = Accumulator336::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.58_f32 + y.sin();
        let b = y * 2.946_f32 - x.cos();
        let mut acc = Accumulator336::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.466_f32 + y.sin();
        let b = y * 5.458_f32 - x.cos();
        let mut acc = Accumulator336::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.259_f32 + y.sin();
        let b = y * 2.476_f32 - x.cos();
        let mut acc = Accumulator336::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.395_f32 + y.sin();
        let b = y * 7.806_f32 - x.cos();
        let mut acc = Accumulator336::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.62_f32 + y.sin();
        let b = y * 0.109_f32 - x.cos();
        let mut acc = Accumulator336::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.155_f32 + y.sin();
        let b = y * 1.421_f32 - x.cos();
        let mut acc = Accumulator336::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.342_f32 + y.sin();
        let b = y * 9.616_f32 - x.cos();
        let mut acc = Accumulator336::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.374_f32 + y.sin();
        let b = y * 5.12_f32 - x.cos();
        let mut acc = Accumulator336::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.174_f32 + y.sin();
        let b = y * 4.123_f32 - x.cos();
        let mut acc = Accumulator336::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.613_f32 + y.sin();
        let b = y * 2.623_f32 - x.cos();
        let mut acc = Accumulator336::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.336_f32 + y.sin();
        let b = y * 1.364_f32 - x.cos();
        let mut acc = Accumulator336::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_336(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_336() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_336(total as u64) % 997) as f32;
        total
    }
}

pub mod m337 {
    use super::*;

    pub struct Accumulator337<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator337<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.155_f32 + y.sin();
        let b = y * 0.302_f32 - x.cos();
        let mut acc = Accumulator337::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.562_f32 + y.sin();
        let b = y * 2.917_f32 - x.cos();
        let mut acc = Accumulator337::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.533_f32 + y.sin();
        let b = y * 9.647_f32 - x.cos();
        let mut acc = Accumulator337::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.235_f32 + y.sin();
        let b = y * 9.193_f32 - x.cos();
        let mut acc = Accumulator337::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.568_f32 + y.sin();
        let b = y * 4.231_f32 - x.cos();
        let mut acc = Accumulator337::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.615_f32 + y.sin();
        let b = y * 8.654_f32 - x.cos();
        let mut acc = Accumulator337::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.818_f32 + y.sin();
        let b = y * 4.508_f32 - x.cos();
        let mut acc = Accumulator337::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.735_f32 + y.sin();
        let b = y * 7.691_f32 - x.cos();
        let mut acc = Accumulator337::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.83_f32 + y.sin();
        let b = y * 7.582_f32 - x.cos();
        let mut acc = Accumulator337::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.5_f32 + y.sin();
        let b = y * 3.392_f32 - x.cos();
        let mut acc = Accumulator337::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.195_f32 + y.sin();
        let b = y * 7.412_f32 - x.cos();
        let mut acc = Accumulator337::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.248_f32 + y.sin();
        let b = y * 0.294_f32 - x.cos();
        let mut acc = Accumulator337::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.255_f32 + y.sin();
        let b = y * 6.973_f32 - x.cos();
        let mut acc = Accumulator337::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.268_f32 + y.sin();
        let b = y * 0.678_f32 - x.cos();
        let mut acc = Accumulator337::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.787_f32 + y.sin();
        let b = y * 0.47_f32 - x.cos();
        let mut acc = Accumulator337::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.585_f32 + y.sin();
        let b = y * 5.498_f32 - x.cos();
        let mut acc = Accumulator337::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.589_f32 + y.sin();
        let b = y * 3.811_f32 - x.cos();
        let mut acc = Accumulator337::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.785_f32 + y.sin();
        let b = y * 9.047_f32 - x.cos();
        let mut acc = Accumulator337::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.447_f32 + y.sin();
        let b = y * 1.458_f32 - x.cos();
        let mut acc = Accumulator337::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.206_f32 + y.sin();
        let b = y * 8.599_f32 - x.cos();
        let mut acc = Accumulator337::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_337(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m337-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_337() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_337(total as u64) % 997) as f32;
        total
    }
}

pub mod m338 {
    use super::*;

    pub struct Accumulator338<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator338<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.138_f32 + y.sin();
        let b = y * 7.392_f32 - x.cos();
        let mut acc = Accumulator338::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.858_f32 + y.sin();
        let b = y * 9.626_f32 - x.cos();
        let mut acc = Accumulator338::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.764_f32 + y.sin();
        let b = y * 8.707_f32 - x.cos();
        let mut acc = Accumulator338::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.732_f32 + y.sin();
        let b = y * 2.937_f32 - x.cos();
        let mut acc = Accumulator338::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.583_f32 + y.sin();
        let b = y * 9.181_f32 - x.cos();
        let mut acc = Accumulator338::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.184_f32 + y.sin();
        let b = y * 9.893_f32 - x.cos();
        let mut acc = Accumulator338::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.379_f32 + y.sin();
        let b = y * 0.382_f32 - x.cos();
        let mut acc = Accumulator338::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.486_f32 + y.sin();
        let b = y * 1.147_f32 - x.cos();
        let mut acc = Accumulator338::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.659_f32 + y.sin();
        let b = y * 6.13_f32 - x.cos();
        let mut acc = Accumulator338::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.413_f32 + y.sin();
        let b = y * 0.518_f32 - x.cos();
        let mut acc = Accumulator338::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.442_f32 + y.sin();
        let b = y * 0.581_f32 - x.cos();
        let mut acc = Accumulator338::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.33_f32 + y.sin();
        let b = y * 2.474_f32 - x.cos();
        let mut acc = Accumulator338::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.366_f32 + y.sin();
        let b = y * 5.501_f32 - x.cos();
        let mut acc = Accumulator338::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.317_f32 + y.sin();
        let b = y * 9.436_f32 - x.cos();
        let mut acc = Accumulator338::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.303_f32 + y.sin();
        let b = y * 1.733_f32 - x.cos();
        let mut acc = Accumulator338::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.088_f32 + y.sin();
        let b = y * 8.456_f32 - x.cos();
        let mut acc = Accumulator338::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.368_f32 + y.sin();
        let b = y * 4.165_f32 - x.cos();
        let mut acc = Accumulator338::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.32_f32 + y.sin();
        let b = y * 8.865_f32 - x.cos();
        let mut acc = Accumulator338::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.276_f32 + y.sin();
        let b = y * 0.921_f32 - x.cos();
        let mut acc = Accumulator338::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.022_f32 + y.sin();
        let b = y * 8.874_f32 - x.cos();
        let mut acc = Accumulator338::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_338(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_338() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_338(total as u64) % 997) as f32;
        total
    }
}

pub mod m339 {
    use super::*;

    pub struct Accumulator339<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator339<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.52_f32 + y.sin();
        let b = y * 9.328_f32 - x.cos();
        let mut acc = Accumulator339::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.002_f32 + y.sin();
        let b = y * 2.867_f32 - x.cos();
        let mut acc = Accumulator339::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.916_f32 + y.sin();
        let b = y * 5.488_f32 - x.cos();
        let mut acc = Accumulator339::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.532_f32 + y.sin();
        let b = y * 5.177_f32 - x.cos();
        let mut acc = Accumulator339::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.228_f32 + y.sin();
        let b = y * 1.127_f32 - x.cos();
        let mut acc = Accumulator339::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.922_f32 + y.sin();
        let b = y * 8.012_f32 - x.cos();
        let mut acc = Accumulator339::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.062_f32 + y.sin();
        let b = y * 0.814_f32 - x.cos();
        let mut acc = Accumulator339::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.766_f32 + y.sin();
        let b = y * 3.741_f32 - x.cos();
        let mut acc = Accumulator339::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.722_f32 + y.sin();
        let b = y * 8.246_f32 - x.cos();
        let mut acc = Accumulator339::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.525_f32 + y.sin();
        let b = y * 4.424_f32 - x.cos();
        let mut acc = Accumulator339::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.448_f32 + y.sin();
        let b = y * 5.194_f32 - x.cos();
        let mut acc = Accumulator339::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.438_f32 + y.sin();
        let b = y * 8.038_f32 - x.cos();
        let mut acc = Accumulator339::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.243_f32 + y.sin();
        let b = y * 7.886_f32 - x.cos();
        let mut acc = Accumulator339::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.183_f32 + y.sin();
        let b = y * 5.343_f32 - x.cos();
        let mut acc = Accumulator339::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.617_f32 + y.sin();
        let b = y * 7.526_f32 - x.cos();
        let mut acc = Accumulator339::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.505_f32 + y.sin();
        let b = y * 9.617_f32 - x.cos();
        let mut acc = Accumulator339::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.335_f32 + y.sin();
        let b = y * 8.862_f32 - x.cos();
        let mut acc = Accumulator339::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.811_f32 + y.sin();
        let b = y * 5.345_f32 - x.cos();
        let mut acc = Accumulator339::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.568_f32 + y.sin();
        let b = y * 0.119_f32 - x.cos();
        let mut acc = Accumulator339::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.065_f32 + y.sin();
        let b = y * 2.342_f32 - x.cos();
        let mut acc = Accumulator339::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_339(seed: u64) -> u64 {
        let re = Regex::new(r"m339-(\d+)").unwrap();
        let hay = format!("m339-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_339() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_339(total as u64) % 997) as f32;
        total
    }
}

pub mod m340 {
    use super::*;

    pub struct Accumulator340<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator340<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.821_f32 + y.sin();
        let b = y * 0.824_f32 - x.cos();
        let mut acc = Accumulator340::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.31_f32 + y.sin();
        let b = y * 9.813_f32 - x.cos();
        let mut acc = Accumulator340::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.925_f32 + y.sin();
        let b = y * 0.521_f32 - x.cos();
        let mut acc = Accumulator340::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.468_f32 + y.sin();
        let b = y * 7.398_f32 - x.cos();
        let mut acc = Accumulator340::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.934_f32 + y.sin();
        let b = y * 2.297_f32 - x.cos();
        let mut acc = Accumulator340::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.411_f32 + y.sin();
        let b = y * 3.619_f32 - x.cos();
        let mut acc = Accumulator340::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.371_f32 + y.sin();
        let b = y * 4.639_f32 - x.cos();
        let mut acc = Accumulator340::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.582_f32 + y.sin();
        let b = y * 6.793_f32 - x.cos();
        let mut acc = Accumulator340::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.71_f32 + y.sin();
        let b = y * 9.172_f32 - x.cos();
        let mut acc = Accumulator340::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.121_f32 + y.sin();
        let b = y * 3.252_f32 - x.cos();
        let mut acc = Accumulator340::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.222_f32 + y.sin();
        let b = y * 0.242_f32 - x.cos();
        let mut acc = Accumulator340::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.437_f32 + y.sin();
        let b = y * 3.208_f32 - x.cos();
        let mut acc = Accumulator340::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.374_f32 + y.sin();
        let b = y * 6.708_f32 - x.cos();
        let mut acc = Accumulator340::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.917_f32 + y.sin();
        let b = y * 4.507_f32 - x.cos();
        let mut acc = Accumulator340::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.511_f32 + y.sin();
        let b = y * 1.552_f32 - x.cos();
        let mut acc = Accumulator340::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.409_f32 + y.sin();
        let b = y * 3.456_f32 - x.cos();
        let mut acc = Accumulator340::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.255_f32 + y.sin();
        let b = y * 0.476_f32 - x.cos();
        let mut acc = Accumulator340::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.518_f32 + y.sin();
        let b = y * 6.955_f32 - x.cos();
        let mut acc = Accumulator340::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.677_f32 + y.sin();
        let b = y * 3.89_f32 - x.cos();
        let mut acc = Accumulator340::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.483_f32 + y.sin();
        let b = y * 8.299_f32 - x.cos();
        let mut acc = Accumulator340::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_340(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_340() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_340(total as u64) % 997) as f32;
        total
    }
}

pub mod m341 {
    use super::*;

    pub struct Accumulator341<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator341<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.022_f32 + y.sin();
        let b = y * 3.344_f32 - x.cos();
        let mut acc = Accumulator341::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.835_f32 + y.sin();
        let b = y * 8.855_f32 - x.cos();
        let mut acc = Accumulator341::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.412_f32 + y.sin();
        let b = y * 0.445_f32 - x.cos();
        let mut acc = Accumulator341::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.492_f32 + y.sin();
        let b = y * 1.415_f32 - x.cos();
        let mut acc = Accumulator341::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.96_f32 + y.sin();
        let b = y * 7.721_f32 - x.cos();
        let mut acc = Accumulator341::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.931_f32 + y.sin();
        let b = y * 1.633_f32 - x.cos();
        let mut acc = Accumulator341::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.127_f32 + y.sin();
        let b = y * 8.856_f32 - x.cos();
        let mut acc = Accumulator341::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.7_f32 + y.sin();
        let b = y * 1.359_f32 - x.cos();
        let mut acc = Accumulator341::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.496_f32 + y.sin();
        let b = y * 2.995_f32 - x.cos();
        let mut acc = Accumulator341::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.775_f32 + y.sin();
        let b = y * 7.764_f32 - x.cos();
        let mut acc = Accumulator341::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.983_f32 + y.sin();
        let b = y * 8.64_f32 - x.cos();
        let mut acc = Accumulator341::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.606_f32 + y.sin();
        let b = y * 9.522_f32 - x.cos();
        let mut acc = Accumulator341::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.932_f32 + y.sin();
        let b = y * 6.808_f32 - x.cos();
        let mut acc = Accumulator341::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.809_f32 + y.sin();
        let b = y * 1.816_f32 - x.cos();
        let mut acc = Accumulator341::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.356_f32 + y.sin();
        let b = y * 8.963_f32 - x.cos();
        let mut acc = Accumulator341::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.009_f32 + y.sin();
        let b = y * 3.868_f32 - x.cos();
        let mut acc = Accumulator341::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.654_f32 + y.sin();
        let b = y * 6.555_f32 - x.cos();
        let mut acc = Accumulator341::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.829_f32 + y.sin();
        let b = y * 9.798_f32 - x.cos();
        let mut acc = Accumulator341::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.905_f32 + y.sin();
        let b = y * 9.771_f32 - x.cos();
        let mut acc = Accumulator341::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.033_f32 + y.sin();
        let b = y * 6.602_f32 - x.cos();
        let mut acc = Accumulator341::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_341(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(341u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_341() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_341(total as u64) % 997) as f32;
        total
    }
}

pub mod m342 {
    use super::*;

    pub struct Accumulator342<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator342<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.261_f32 + y.sin();
        let b = y * 6.268_f32 - x.cos();
        let mut acc = Accumulator342::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.121_f32 + y.sin();
        let b = y * 0.898_f32 - x.cos();
        let mut acc = Accumulator342::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.364_f32 + y.sin();
        let b = y * 8.669_f32 - x.cos();
        let mut acc = Accumulator342::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.607_f32 + y.sin();
        let b = y * 0.394_f32 - x.cos();
        let mut acc = Accumulator342::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.386_f32 + y.sin();
        let b = y * 8.286_f32 - x.cos();
        let mut acc = Accumulator342::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.794_f32 + y.sin();
        let b = y * 5.399_f32 - x.cos();
        let mut acc = Accumulator342::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.09_f32 + y.sin();
        let b = y * 6.413_f32 - x.cos();
        let mut acc = Accumulator342::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.882_f32 + y.sin();
        let b = y * 8.135_f32 - x.cos();
        let mut acc = Accumulator342::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.175_f32 + y.sin();
        let b = y * 7.846_f32 - x.cos();
        let mut acc = Accumulator342::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.119_f32 + y.sin();
        let b = y * 4.97_f32 - x.cos();
        let mut acc = Accumulator342::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.618_f32 + y.sin();
        let b = y * 4.932_f32 - x.cos();
        let mut acc = Accumulator342::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.09_f32 + y.sin();
        let b = y * 7.454_f32 - x.cos();
        let mut acc = Accumulator342::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.702_f32 + y.sin();
        let b = y * 7.51_f32 - x.cos();
        let mut acc = Accumulator342::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.067_f32 + y.sin();
        let b = y * 5.933_f32 - x.cos();
        let mut acc = Accumulator342::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.658_f32 + y.sin();
        let b = y * 4.873_f32 - x.cos();
        let mut acc = Accumulator342::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.013_f32 + y.sin();
        let b = y * 9.583_f32 - x.cos();
        let mut acc = Accumulator342::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.558_f32 + y.sin();
        let b = y * 4.527_f32 - x.cos();
        let mut acc = Accumulator342::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.363_f32 + y.sin();
        let b = y * 5.345_f32 - x.cos();
        let mut acc = Accumulator342::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.195_f32 + y.sin();
        let b = y * 0.419_f32 - x.cos();
        let mut acc = Accumulator342::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.691_f32 + y.sin();
        let b = y * 6.565_f32 - x.cos();
        let mut acc = Accumulator342::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_342(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_342() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_342(total as u64) % 997) as f32;
        total
    }
}

pub mod m343 {
    use super::*;

    pub struct Accumulator343<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator343<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.139_f32 + y.sin();
        let b = y * 5.295_f32 - x.cos();
        let mut acc = Accumulator343::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.641_f32 + y.sin();
        let b = y * 7.882_f32 - x.cos();
        let mut acc = Accumulator343::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.565_f32 + y.sin();
        let b = y * 0.896_f32 - x.cos();
        let mut acc = Accumulator343::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.554_f32 + y.sin();
        let b = y * 0.397_f32 - x.cos();
        let mut acc = Accumulator343::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.67_f32 + y.sin();
        let b = y * 8.465_f32 - x.cos();
        let mut acc = Accumulator343::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.714_f32 + y.sin();
        let b = y * 2.714_f32 - x.cos();
        let mut acc = Accumulator343::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.158_f32 + y.sin();
        let b = y * 5.965_f32 - x.cos();
        let mut acc = Accumulator343::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.021_f32 + y.sin();
        let b = y * 7.875_f32 - x.cos();
        let mut acc = Accumulator343::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.28_f32 + y.sin();
        let b = y * 8.558_f32 - x.cos();
        let mut acc = Accumulator343::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.074_f32 + y.sin();
        let b = y * 9.595_f32 - x.cos();
        let mut acc = Accumulator343::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.616_f32 + y.sin();
        let b = y * 8.568_f32 - x.cos();
        let mut acc = Accumulator343::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.532_f32 + y.sin();
        let b = y * 0.27_f32 - x.cos();
        let mut acc = Accumulator343::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.358_f32 + y.sin();
        let b = y * 9.813_f32 - x.cos();
        let mut acc = Accumulator343::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.384_f32 + y.sin();
        let b = y * 9.57_f32 - x.cos();
        let mut acc = Accumulator343::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.919_f32 + y.sin();
        let b = y * 7.862_f32 - x.cos();
        let mut acc = Accumulator343::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.928_f32 + y.sin();
        let b = y * 6.108_f32 - x.cos();
        let mut acc = Accumulator343::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.057_f32 + y.sin();
        let b = y * 7.883_f32 - x.cos();
        let mut acc = Accumulator343::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.318_f32 + y.sin();
        let b = y * 9.283_f32 - x.cos();
        let mut acc = Accumulator343::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.858_f32 + y.sin();
        let b = y * 1.825_f32 - x.cos();
        let mut acc = Accumulator343::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.721_f32 + y.sin();
        let b = y * 3.222_f32 - x.cos();
        let mut acc = Accumulator343::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_343(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_343() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_343(total as u64) % 997) as f32;
        total
    }
}

pub mod m344 {
    use super::*;

    pub struct Accumulator344<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator344<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.897_f32 + y.sin();
        let b = y * 9.224_f32 - x.cos();
        let mut acc = Accumulator344::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.691_f32 + y.sin();
        let b = y * 3.292_f32 - x.cos();
        let mut acc = Accumulator344::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.103_f32 + y.sin();
        let b = y * 8.198_f32 - x.cos();
        let mut acc = Accumulator344::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.43_f32 + y.sin();
        let b = y * 2.782_f32 - x.cos();
        let mut acc = Accumulator344::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.463_f32 + y.sin();
        let b = y * 8.869_f32 - x.cos();
        let mut acc = Accumulator344::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.656_f32 + y.sin();
        let b = y * 4.787_f32 - x.cos();
        let mut acc = Accumulator344::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.007_f32 + y.sin();
        let b = y * 1.516_f32 - x.cos();
        let mut acc = Accumulator344::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.882_f32 + y.sin();
        let b = y * 8.641_f32 - x.cos();
        let mut acc = Accumulator344::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.494_f32 + y.sin();
        let b = y * 9.809_f32 - x.cos();
        let mut acc = Accumulator344::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.015_f32 + y.sin();
        let b = y * 6.927_f32 - x.cos();
        let mut acc = Accumulator344::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.597_f32 + y.sin();
        let b = y * 3.45_f32 - x.cos();
        let mut acc = Accumulator344::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.811_f32 + y.sin();
        let b = y * 8.852_f32 - x.cos();
        let mut acc = Accumulator344::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.829_f32 + y.sin();
        let b = y * 8.64_f32 - x.cos();
        let mut acc = Accumulator344::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.491_f32 + y.sin();
        let b = y * 9.864_f32 - x.cos();
        let mut acc = Accumulator344::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.997_f32 + y.sin();
        let b = y * 0.765_f32 - x.cos();
        let mut acc = Accumulator344::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.651_f32 + y.sin();
        let b = y * 2.403_f32 - x.cos();
        let mut acc = Accumulator344::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.004_f32 + y.sin();
        let b = y * 9.099_f32 - x.cos();
        let mut acc = Accumulator344::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.941_f32 + y.sin();
        let b = y * 0.492_f32 - x.cos();
        let mut acc = Accumulator344::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.901_f32 + y.sin();
        let b = y * 3.636_f32 - x.cos();
        let mut acc = Accumulator344::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.108_f32 + y.sin();
        let b = y * 7.311_f32 - x.cos();
        let mut acc = Accumulator344::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_344(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m344-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_344() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_344(total as u64) % 997) as f32;
        total
    }
}

pub mod m345 {
    use super::*;

    pub struct Accumulator345<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator345<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.51_f32 + y.sin();
        let b = y * 0.335_f32 - x.cos();
        let mut acc = Accumulator345::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.555_f32 + y.sin();
        let b = y * 4.744_f32 - x.cos();
        let mut acc = Accumulator345::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.011_f32 + y.sin();
        let b = y * 5.738_f32 - x.cos();
        let mut acc = Accumulator345::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.681_f32 + y.sin();
        let b = y * 5.56_f32 - x.cos();
        let mut acc = Accumulator345::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.832_f32 + y.sin();
        let b = y * 8.686_f32 - x.cos();
        let mut acc = Accumulator345::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.394_f32 + y.sin();
        let b = y * 8.429_f32 - x.cos();
        let mut acc = Accumulator345::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.142_f32 + y.sin();
        let b = y * 5.864_f32 - x.cos();
        let mut acc = Accumulator345::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.02_f32 + y.sin();
        let b = y * 4.062_f32 - x.cos();
        let mut acc = Accumulator345::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.501_f32 + y.sin();
        let b = y * 9.581_f32 - x.cos();
        let mut acc = Accumulator345::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.693_f32 + y.sin();
        let b = y * 8.224_f32 - x.cos();
        let mut acc = Accumulator345::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.661_f32 + y.sin();
        let b = y * 8.314_f32 - x.cos();
        let mut acc = Accumulator345::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.7_f32 + y.sin();
        let b = y * 8.299_f32 - x.cos();
        let mut acc = Accumulator345::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.737_f32 + y.sin();
        let b = y * 0.898_f32 - x.cos();
        let mut acc = Accumulator345::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.935_f32 + y.sin();
        let b = y * 1.673_f32 - x.cos();
        let mut acc = Accumulator345::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.385_f32 + y.sin();
        let b = y * 7.854_f32 - x.cos();
        let mut acc = Accumulator345::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.487_f32 + y.sin();
        let b = y * 4.599_f32 - x.cos();
        let mut acc = Accumulator345::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.846_f32 + y.sin();
        let b = y * 7.604_f32 - x.cos();
        let mut acc = Accumulator345::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.493_f32 + y.sin();
        let b = y * 8.688_f32 - x.cos();
        let mut acc = Accumulator345::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.173_f32 + y.sin();
        let b = y * 5.615_f32 - x.cos();
        let mut acc = Accumulator345::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.981_f32 + y.sin();
        let b = y * 8.535_f32 - x.cos();
        let mut acc = Accumulator345::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_345(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_345() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_345(total as u64) % 997) as f32;
        total
    }
}

pub mod m346 {
    use super::*;

    pub struct Accumulator346<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator346<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.825_f32 + y.sin();
        let b = y * 2.926_f32 - x.cos();
        let mut acc = Accumulator346::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.825_f32 + y.sin();
        let b = y * 3.581_f32 - x.cos();
        let mut acc = Accumulator346::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.324_f32 + y.sin();
        let b = y * 6.473_f32 - x.cos();
        let mut acc = Accumulator346::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.857_f32 + y.sin();
        let b = y * 4.299_f32 - x.cos();
        let mut acc = Accumulator346::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.452_f32 + y.sin();
        let b = y * 2.826_f32 - x.cos();
        let mut acc = Accumulator346::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.737_f32 + y.sin();
        let b = y * 1.585_f32 - x.cos();
        let mut acc = Accumulator346::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.088_f32 + y.sin();
        let b = y * 0.192_f32 - x.cos();
        let mut acc = Accumulator346::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.115_f32 + y.sin();
        let b = y * 6.298_f32 - x.cos();
        let mut acc = Accumulator346::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.661_f32 + y.sin();
        let b = y * 1.241_f32 - x.cos();
        let mut acc = Accumulator346::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.497_f32 + y.sin();
        let b = y * 4.777_f32 - x.cos();
        let mut acc = Accumulator346::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.532_f32 + y.sin();
        let b = y * 7.618_f32 - x.cos();
        let mut acc = Accumulator346::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.465_f32 + y.sin();
        let b = y * 2.911_f32 - x.cos();
        let mut acc = Accumulator346::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.947_f32 + y.sin();
        let b = y * 2.455_f32 - x.cos();
        let mut acc = Accumulator346::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.269_f32 + y.sin();
        let b = y * 6.231_f32 - x.cos();
        let mut acc = Accumulator346::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.608_f32 + y.sin();
        let b = y * 5.963_f32 - x.cos();
        let mut acc = Accumulator346::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.824_f32 + y.sin();
        let b = y * 4.879_f32 - x.cos();
        let mut acc = Accumulator346::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.41_f32 + y.sin();
        let b = y * 9.084_f32 - x.cos();
        let mut acc = Accumulator346::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.221_f32 + y.sin();
        let b = y * 3.87_f32 - x.cos();
        let mut acc = Accumulator346::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.721_f32 + y.sin();
        let b = y * 3.955_f32 - x.cos();
        let mut acc = Accumulator346::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.171_f32 + y.sin();
        let b = y * 5.19_f32 - x.cos();
        let mut acc = Accumulator346::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_346(seed: u64) -> u64 {
        let re = Regex::new(r"m346-(\d+)").unwrap();
        let hay = format!("m346-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_346() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_346(total as u64) % 997) as f32;
        total
    }
}

pub mod m347 {
    use super::*;

    pub struct Accumulator347<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator347<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.396_f32 + y.sin();
        let b = y * 3.597_f32 - x.cos();
        let mut acc = Accumulator347::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.429_f32 + y.sin();
        let b = y * 0.825_f32 - x.cos();
        let mut acc = Accumulator347::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.905_f32 + y.sin();
        let b = y * 5.03_f32 - x.cos();
        let mut acc = Accumulator347::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.996_f32 + y.sin();
        let b = y * 6.362_f32 - x.cos();
        let mut acc = Accumulator347::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.525_f32 + y.sin();
        let b = y * 0.758_f32 - x.cos();
        let mut acc = Accumulator347::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.563_f32 + y.sin();
        let b = y * 8.462_f32 - x.cos();
        let mut acc = Accumulator347::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.777_f32 + y.sin();
        let b = y * 5.317_f32 - x.cos();
        let mut acc = Accumulator347::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.441_f32 + y.sin();
        let b = y * 1.55_f32 - x.cos();
        let mut acc = Accumulator347::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.948_f32 + y.sin();
        let b = y * 5.213_f32 - x.cos();
        let mut acc = Accumulator347::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.9_f32 + y.sin();
        let b = y * 6.803_f32 - x.cos();
        let mut acc = Accumulator347::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.863_f32 + y.sin();
        let b = y * 9.539_f32 - x.cos();
        let mut acc = Accumulator347::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.606_f32 + y.sin();
        let b = y * 2.935_f32 - x.cos();
        let mut acc = Accumulator347::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.73_f32 + y.sin();
        let b = y * 2.433_f32 - x.cos();
        let mut acc = Accumulator347::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.717_f32 + y.sin();
        let b = y * 0.344_f32 - x.cos();
        let mut acc = Accumulator347::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.867_f32 + y.sin();
        let b = y * 6.753_f32 - x.cos();
        let mut acc = Accumulator347::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.868_f32 + y.sin();
        let b = y * 8.65_f32 - x.cos();
        let mut acc = Accumulator347::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.312_f32 + y.sin();
        let b = y * 8.096_f32 - x.cos();
        let mut acc = Accumulator347::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.18_f32 + y.sin();
        let b = y * 5.9_f32 - x.cos();
        let mut acc = Accumulator347::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.366_f32 + y.sin();
        let b = y * 2.305_f32 - x.cos();
        let mut acc = Accumulator347::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.007_f32 + y.sin();
        let b = y * 2.385_f32 - x.cos();
        let mut acc = Accumulator347::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_347(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_347() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_347(total as u64) % 997) as f32;
        total
    }
}

pub mod m348 {
    use super::*;

    pub struct Accumulator348<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator348<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.257_f32 + y.sin();
        let b = y * 7.182_f32 - x.cos();
        let mut acc = Accumulator348::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.345_f32 + y.sin();
        let b = y * 8.693_f32 - x.cos();
        let mut acc = Accumulator348::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.106_f32 + y.sin();
        let b = y * 6.817_f32 - x.cos();
        let mut acc = Accumulator348::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.772_f32 + y.sin();
        let b = y * 6.084_f32 - x.cos();
        let mut acc = Accumulator348::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.827_f32 + y.sin();
        let b = y * 1.628_f32 - x.cos();
        let mut acc = Accumulator348::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.518_f32 + y.sin();
        let b = y * 0.817_f32 - x.cos();
        let mut acc = Accumulator348::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.625_f32 + y.sin();
        let b = y * 3.232_f32 - x.cos();
        let mut acc = Accumulator348::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.72_f32 + y.sin();
        let b = y * 5.529_f32 - x.cos();
        let mut acc = Accumulator348::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.862_f32 + y.sin();
        let b = y * 3.559_f32 - x.cos();
        let mut acc = Accumulator348::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.571_f32 + y.sin();
        let b = y * 1.106_f32 - x.cos();
        let mut acc = Accumulator348::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.609_f32 + y.sin();
        let b = y * 7.106_f32 - x.cos();
        let mut acc = Accumulator348::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.887_f32 + y.sin();
        let b = y * 4.599_f32 - x.cos();
        let mut acc = Accumulator348::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.213_f32 + y.sin();
        let b = y * 9.51_f32 - x.cos();
        let mut acc = Accumulator348::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.159_f32 + y.sin();
        let b = y * 3.283_f32 - x.cos();
        let mut acc = Accumulator348::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.584_f32 + y.sin();
        let b = y * 5.641_f32 - x.cos();
        let mut acc = Accumulator348::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.011_f32 + y.sin();
        let b = y * 6.793_f32 - x.cos();
        let mut acc = Accumulator348::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.654_f32 + y.sin();
        let b = y * 1.729_f32 - x.cos();
        let mut acc = Accumulator348::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.153_f32 + y.sin();
        let b = y * 7.544_f32 - x.cos();
        let mut acc = Accumulator348::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.656_f32 + y.sin();
        let b = y * 9.283_f32 - x.cos();
        let mut acc = Accumulator348::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.255_f32 + y.sin();
        let b = y * 8.351_f32 - x.cos();
        let mut acc = Accumulator348::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_348(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(348u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_348() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_348(total as u64) % 997) as f32;
        total
    }
}

pub mod m349 {
    use super::*;

    pub struct Accumulator349<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator349<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.226_f32 + y.sin();
        let b = y * 9.744_f32 - x.cos();
        let mut acc = Accumulator349::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.055_f32 + y.sin();
        let b = y * 8.486_f32 - x.cos();
        let mut acc = Accumulator349::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.229_f32 + y.sin();
        let b = y * 5.862_f32 - x.cos();
        let mut acc = Accumulator349::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.639_f32 + y.sin();
        let b = y * 2.96_f32 - x.cos();
        let mut acc = Accumulator349::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.183_f32 + y.sin();
        let b = y * 3.44_f32 - x.cos();
        let mut acc = Accumulator349::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.807_f32 + y.sin();
        let b = y * 1.707_f32 - x.cos();
        let mut acc = Accumulator349::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.409_f32 + y.sin();
        let b = y * 5.802_f32 - x.cos();
        let mut acc = Accumulator349::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.389_f32 + y.sin();
        let b = y * 2.687_f32 - x.cos();
        let mut acc = Accumulator349::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.558_f32 + y.sin();
        let b = y * 3.283_f32 - x.cos();
        let mut acc = Accumulator349::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.171_f32 + y.sin();
        let b = y * 8.546_f32 - x.cos();
        let mut acc = Accumulator349::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.8_f32 + y.sin();
        let b = y * 3.984_f32 - x.cos();
        let mut acc = Accumulator349::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.685_f32 + y.sin();
        let b = y * 6.767_f32 - x.cos();
        let mut acc = Accumulator349::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.404_f32 + y.sin();
        let b = y * 0.92_f32 - x.cos();
        let mut acc = Accumulator349::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.583_f32 + y.sin();
        let b = y * 7.417_f32 - x.cos();
        let mut acc = Accumulator349::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.596_f32 + y.sin();
        let b = y * 6.07_f32 - x.cos();
        let mut acc = Accumulator349::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.283_f32 + y.sin();
        let b = y * 6.288_f32 - x.cos();
        let mut acc = Accumulator349::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.921_f32 + y.sin();
        let b = y * 2.72_f32 - x.cos();
        let mut acc = Accumulator349::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.448_f32 + y.sin();
        let b = y * 7.143_f32 - x.cos();
        let mut acc = Accumulator349::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.075_f32 + y.sin();
        let b = y * 3.241_f32 - x.cos();
        let mut acc = Accumulator349::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.51_f32 + y.sin();
        let b = y * 6.073_f32 - x.cos();
        let mut acc = Accumulator349::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_349(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_349() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_349(total as u64) % 997) as f32;
        total
    }
}

pub mod m350 {
    use super::*;

    pub struct Accumulator350<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator350<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.62_f32 + y.sin();
        let b = y * 2.642_f32 - x.cos();
        let mut acc = Accumulator350::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.297_f32 + y.sin();
        let b = y * 3.957_f32 - x.cos();
        let mut acc = Accumulator350::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.411_f32 + y.sin();
        let b = y * 6.489_f32 - x.cos();
        let mut acc = Accumulator350::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.203_f32 + y.sin();
        let b = y * 9.358_f32 - x.cos();
        let mut acc = Accumulator350::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.36_f32 + y.sin();
        let b = y * 0.471_f32 - x.cos();
        let mut acc = Accumulator350::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.904_f32 + y.sin();
        let b = y * 1.859_f32 - x.cos();
        let mut acc = Accumulator350::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.938_f32 + y.sin();
        let b = y * 3.884_f32 - x.cos();
        let mut acc = Accumulator350::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.134_f32 + y.sin();
        let b = y * 9.378_f32 - x.cos();
        let mut acc = Accumulator350::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.209_f32 + y.sin();
        let b = y * 2.761_f32 - x.cos();
        let mut acc = Accumulator350::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.121_f32 + y.sin();
        let b = y * 7.478_f32 - x.cos();
        let mut acc = Accumulator350::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.965_f32 + y.sin();
        let b = y * 1.505_f32 - x.cos();
        let mut acc = Accumulator350::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.451_f32 + y.sin();
        let b = y * 3.085_f32 - x.cos();
        let mut acc = Accumulator350::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.744_f32 + y.sin();
        let b = y * 8.476_f32 - x.cos();
        let mut acc = Accumulator350::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.448_f32 + y.sin();
        let b = y * 7.047_f32 - x.cos();
        let mut acc = Accumulator350::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.37_f32 + y.sin();
        let b = y * 1.828_f32 - x.cos();
        let mut acc = Accumulator350::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.149_f32 + y.sin();
        let b = y * 3.749_f32 - x.cos();
        let mut acc = Accumulator350::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.032_f32 + y.sin();
        let b = y * 1.923_f32 - x.cos();
        let mut acc = Accumulator350::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.437_f32 + y.sin();
        let b = y * 0.125_f32 - x.cos();
        let mut acc = Accumulator350::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.441_f32 + y.sin();
        let b = y * 7.848_f32 - x.cos();
        let mut acc = Accumulator350::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.996_f32 + y.sin();
        let b = y * 6.8_f32 - x.cos();
        let mut acc = Accumulator350::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_350(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_350() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_350(total as u64) % 997) as f32;
        total
    }
}

pub mod m351 {
    use super::*;

    pub struct Accumulator351<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator351<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.115_f32 + y.sin();
        let b = y * 7.326_f32 - x.cos();
        let mut acc = Accumulator351::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.606_f32 + y.sin();
        let b = y * 1.365_f32 - x.cos();
        let mut acc = Accumulator351::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.681_f32 + y.sin();
        let b = y * 8.93_f32 - x.cos();
        let mut acc = Accumulator351::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.302_f32 + y.sin();
        let b = y * 3.773_f32 - x.cos();
        let mut acc = Accumulator351::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.405_f32 + y.sin();
        let b = y * 0.805_f32 - x.cos();
        let mut acc = Accumulator351::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.417_f32 + y.sin();
        let b = y * 6.143_f32 - x.cos();
        let mut acc = Accumulator351::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.78_f32 + y.sin();
        let b = y * 2.099_f32 - x.cos();
        let mut acc = Accumulator351::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.253_f32 + y.sin();
        let b = y * 9.778_f32 - x.cos();
        let mut acc = Accumulator351::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.8_f32 + y.sin();
        let b = y * 5.152_f32 - x.cos();
        let mut acc = Accumulator351::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.064_f32 + y.sin();
        let b = y * 9.865_f32 - x.cos();
        let mut acc = Accumulator351::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.851_f32 + y.sin();
        let b = y * 5.54_f32 - x.cos();
        let mut acc = Accumulator351::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.12_f32 + y.sin();
        let b = y * 1.765_f32 - x.cos();
        let mut acc = Accumulator351::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.63_f32 + y.sin();
        let b = y * 6.806_f32 - x.cos();
        let mut acc = Accumulator351::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.303_f32 + y.sin();
        let b = y * 8.028_f32 - x.cos();
        let mut acc = Accumulator351::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.353_f32 + y.sin();
        let b = y * 6.797_f32 - x.cos();
        let mut acc = Accumulator351::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.302_f32 + y.sin();
        let b = y * 8.677_f32 - x.cos();
        let mut acc = Accumulator351::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.368_f32 + y.sin();
        let b = y * 5.227_f32 - x.cos();
        let mut acc = Accumulator351::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.247_f32 + y.sin();
        let b = y * 4.812_f32 - x.cos();
        let mut acc = Accumulator351::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.826_f32 + y.sin();
        let b = y * 1.489_f32 - x.cos();
        let mut acc = Accumulator351::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.873_f32 + y.sin();
        let b = y * 6.002_f32 - x.cos();
        let mut acc = Accumulator351::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_351(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m351-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_351() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_351(total as u64) % 997) as f32;
        total
    }
}

pub mod m352 {
    use super::*;

    pub struct Accumulator352<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator352<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.953_f32 + y.sin();
        let b = y * 5.689_f32 - x.cos();
        let mut acc = Accumulator352::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.518_f32 + y.sin();
        let b = y * 0.195_f32 - x.cos();
        let mut acc = Accumulator352::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.975_f32 + y.sin();
        let b = y * 0.657_f32 - x.cos();
        let mut acc = Accumulator352::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.76_f32 + y.sin();
        let b = y * 4.639_f32 - x.cos();
        let mut acc = Accumulator352::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.739_f32 + y.sin();
        let b = y * 7.029_f32 - x.cos();
        let mut acc = Accumulator352::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.385_f32 + y.sin();
        let b = y * 7.057_f32 - x.cos();
        let mut acc = Accumulator352::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.268_f32 + y.sin();
        let b = y * 2.219_f32 - x.cos();
        let mut acc = Accumulator352::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.42_f32 + y.sin();
        let b = y * 7.599_f32 - x.cos();
        let mut acc = Accumulator352::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.444_f32 + y.sin();
        let b = y * 9.296_f32 - x.cos();
        let mut acc = Accumulator352::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.639_f32 + y.sin();
        let b = y * 0.212_f32 - x.cos();
        let mut acc = Accumulator352::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.85_f32 + y.sin();
        let b = y * 8.511_f32 - x.cos();
        let mut acc = Accumulator352::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.345_f32 + y.sin();
        let b = y * 1.426_f32 - x.cos();
        let mut acc = Accumulator352::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.537_f32 + y.sin();
        let b = y * 0.177_f32 - x.cos();
        let mut acc = Accumulator352::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.92_f32 + y.sin();
        let b = y * 0.679_f32 - x.cos();
        let mut acc = Accumulator352::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.347_f32 + y.sin();
        let b = y * 4.969_f32 - x.cos();
        let mut acc = Accumulator352::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.837_f32 + y.sin();
        let b = y * 3.495_f32 - x.cos();
        let mut acc = Accumulator352::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.48_f32 + y.sin();
        let b = y * 9.564_f32 - x.cos();
        let mut acc = Accumulator352::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.159_f32 + y.sin();
        let b = y * 7.512_f32 - x.cos();
        let mut acc = Accumulator352::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.779_f32 + y.sin();
        let b = y * 0.471_f32 - x.cos();
        let mut acc = Accumulator352::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.406_f32 + y.sin();
        let b = y * 2.841_f32 - x.cos();
        let mut acc = Accumulator352::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_352(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_352() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_352(total as u64) % 997) as f32;
        total
    }
}

pub mod m353 {
    use super::*;

    pub struct Accumulator353<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator353<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.276_f32 + y.sin();
        let b = y * 0.486_f32 - x.cos();
        let mut acc = Accumulator353::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.373_f32 + y.sin();
        let b = y * 5.86_f32 - x.cos();
        let mut acc = Accumulator353::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.618_f32 + y.sin();
        let b = y * 9.502_f32 - x.cos();
        let mut acc = Accumulator353::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.398_f32 + y.sin();
        let b = y * 2.262_f32 - x.cos();
        let mut acc = Accumulator353::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.327_f32 + y.sin();
        let b = y * 3.182_f32 - x.cos();
        let mut acc = Accumulator353::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.82_f32 + y.sin();
        let b = y * 2.538_f32 - x.cos();
        let mut acc = Accumulator353::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.022_f32 + y.sin();
        let b = y * 3.408_f32 - x.cos();
        let mut acc = Accumulator353::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.071_f32 + y.sin();
        let b = y * 0.813_f32 - x.cos();
        let mut acc = Accumulator353::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.559_f32 + y.sin();
        let b = y * 5.866_f32 - x.cos();
        let mut acc = Accumulator353::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.177_f32 + y.sin();
        let b = y * 6.486_f32 - x.cos();
        let mut acc = Accumulator353::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.39_f32 + y.sin();
        let b = y * 5.702_f32 - x.cos();
        let mut acc = Accumulator353::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.812_f32 + y.sin();
        let b = y * 6.949_f32 - x.cos();
        let mut acc = Accumulator353::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.346_f32 + y.sin();
        let b = y * 9.64_f32 - x.cos();
        let mut acc = Accumulator353::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.51_f32 + y.sin();
        let b = y * 3.272_f32 - x.cos();
        let mut acc = Accumulator353::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.854_f32 + y.sin();
        let b = y * 2.43_f32 - x.cos();
        let mut acc = Accumulator353::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.424_f32 + y.sin();
        let b = y * 3.694_f32 - x.cos();
        let mut acc = Accumulator353::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.122_f32 + y.sin();
        let b = y * 5.125_f32 - x.cos();
        let mut acc = Accumulator353::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.453_f32 + y.sin();
        let b = y * 6.313_f32 - x.cos();
        let mut acc = Accumulator353::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.301_f32 + y.sin();
        let b = y * 1.524_f32 - x.cos();
        let mut acc = Accumulator353::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.605_f32 + y.sin();
        let b = y * 6.786_f32 - x.cos();
        let mut acc = Accumulator353::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_353(seed: u64) -> u64 {
        let re = Regex::new(r"m353-(\d+)").unwrap();
        let hay = format!("m353-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_353() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_353(total as u64) % 997) as f32;
        total
    }
}

pub mod m354 {
    use super::*;

    pub struct Accumulator354<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator354<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.797_f32 + y.sin();
        let b = y * 0.48_f32 - x.cos();
        let mut acc = Accumulator354::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.532_f32 + y.sin();
        let b = y * 2.01_f32 - x.cos();
        let mut acc = Accumulator354::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.358_f32 + y.sin();
        let b = y * 7.783_f32 - x.cos();
        let mut acc = Accumulator354::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.326_f32 + y.sin();
        let b = y * 9.733_f32 - x.cos();
        let mut acc = Accumulator354::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.789_f32 + y.sin();
        let b = y * 1.654_f32 - x.cos();
        let mut acc = Accumulator354::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.032_f32 + y.sin();
        let b = y * 2.427_f32 - x.cos();
        let mut acc = Accumulator354::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.779_f32 + y.sin();
        let b = y * 1.069_f32 - x.cos();
        let mut acc = Accumulator354::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.325_f32 + y.sin();
        let b = y * 3.759_f32 - x.cos();
        let mut acc = Accumulator354::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.255_f32 + y.sin();
        let b = y * 6.691_f32 - x.cos();
        let mut acc = Accumulator354::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.357_f32 + y.sin();
        let b = y * 7.011_f32 - x.cos();
        let mut acc = Accumulator354::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.012_f32 + y.sin();
        let b = y * 9.723_f32 - x.cos();
        let mut acc = Accumulator354::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.723_f32 + y.sin();
        let b = y * 8.706_f32 - x.cos();
        let mut acc = Accumulator354::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.365_f32 + y.sin();
        let b = y * 3.722_f32 - x.cos();
        let mut acc = Accumulator354::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.037_f32 + y.sin();
        let b = y * 3.184_f32 - x.cos();
        let mut acc = Accumulator354::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.062_f32 + y.sin();
        let b = y * 1.297_f32 - x.cos();
        let mut acc = Accumulator354::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.234_f32 + y.sin();
        let b = y * 5.23_f32 - x.cos();
        let mut acc = Accumulator354::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.629_f32 + y.sin();
        let b = y * 5.023_f32 - x.cos();
        let mut acc = Accumulator354::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.798_f32 + y.sin();
        let b = y * 1.298_f32 - x.cos();
        let mut acc = Accumulator354::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.857_f32 + y.sin();
        let b = y * 1.336_f32 - x.cos();
        let mut acc = Accumulator354::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.908_f32 + y.sin();
        let b = y * 0.463_f32 - x.cos();
        let mut acc = Accumulator354::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_354(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_354() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_354(total as u64) % 997) as f32;
        total
    }
}

pub mod m355 {
    use super::*;

    pub struct Accumulator355<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator355<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.477_f32 + y.sin();
        let b = y * 5.224_f32 - x.cos();
        let mut acc = Accumulator355::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.244_f32 + y.sin();
        let b = y * 0.594_f32 - x.cos();
        let mut acc = Accumulator355::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.347_f32 + y.sin();
        let b = y * 5.063_f32 - x.cos();
        let mut acc = Accumulator355::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.05_f32 + y.sin();
        let b = y * 3.854_f32 - x.cos();
        let mut acc = Accumulator355::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.697_f32 + y.sin();
        let b = y * 6.348_f32 - x.cos();
        let mut acc = Accumulator355::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.269_f32 + y.sin();
        let b = y * 4.992_f32 - x.cos();
        let mut acc = Accumulator355::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.855_f32 + y.sin();
        let b = y * 2.71_f32 - x.cos();
        let mut acc = Accumulator355::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.856_f32 + y.sin();
        let b = y * 4.069_f32 - x.cos();
        let mut acc = Accumulator355::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.499_f32 + y.sin();
        let b = y * 5.304_f32 - x.cos();
        let mut acc = Accumulator355::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.803_f32 + y.sin();
        let b = y * 2.326_f32 - x.cos();
        let mut acc = Accumulator355::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.595_f32 + y.sin();
        let b = y * 9.794_f32 - x.cos();
        let mut acc = Accumulator355::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.758_f32 + y.sin();
        let b = y * 2.084_f32 - x.cos();
        let mut acc = Accumulator355::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.681_f32 + y.sin();
        let b = y * 2.081_f32 - x.cos();
        let mut acc = Accumulator355::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.409_f32 + y.sin();
        let b = y * 0.21_f32 - x.cos();
        let mut acc = Accumulator355::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.947_f32 + y.sin();
        let b = y * 7.445_f32 - x.cos();
        let mut acc = Accumulator355::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.182_f32 + y.sin();
        let b = y * 9.347_f32 - x.cos();
        let mut acc = Accumulator355::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.1_f32 + y.sin();
        let b = y * 2.854_f32 - x.cos();
        let mut acc = Accumulator355::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.011_f32 + y.sin();
        let b = y * 6.654_f32 - x.cos();
        let mut acc = Accumulator355::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.453_f32 + y.sin();
        let b = y * 2.28_f32 - x.cos();
        let mut acc = Accumulator355::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.533_f32 + y.sin();
        let b = y * 8.244_f32 - x.cos();
        let mut acc = Accumulator355::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_355(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(355u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_355() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_355(total as u64) % 997) as f32;
        total
    }
}

pub mod m356 {
    use super::*;

    pub struct Accumulator356<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator356<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.168_f32 + y.sin();
        let b = y * 5.471_f32 - x.cos();
        let mut acc = Accumulator356::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.709_f32 + y.sin();
        let b = y * 2.354_f32 - x.cos();
        let mut acc = Accumulator356::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.032_f32 + y.sin();
        let b = y * 4.318_f32 - x.cos();
        let mut acc = Accumulator356::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.46_f32 + y.sin();
        let b = y * 9.043_f32 - x.cos();
        let mut acc = Accumulator356::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.998_f32 + y.sin();
        let b = y * 1.871_f32 - x.cos();
        let mut acc = Accumulator356::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.866_f32 + y.sin();
        let b = y * 8.734_f32 - x.cos();
        let mut acc = Accumulator356::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.241_f32 + y.sin();
        let b = y * 3.797_f32 - x.cos();
        let mut acc = Accumulator356::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.264_f32 + y.sin();
        let b = y * 8.306_f32 - x.cos();
        let mut acc = Accumulator356::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.249_f32 + y.sin();
        let b = y * 4.095_f32 - x.cos();
        let mut acc = Accumulator356::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.92_f32 + y.sin();
        let b = y * 8.356_f32 - x.cos();
        let mut acc = Accumulator356::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.045_f32 + y.sin();
        let b = y * 4.994_f32 - x.cos();
        let mut acc = Accumulator356::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.091_f32 + y.sin();
        let b = y * 6.765_f32 - x.cos();
        let mut acc = Accumulator356::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.535_f32 + y.sin();
        let b = y * 7.528_f32 - x.cos();
        let mut acc = Accumulator356::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.187_f32 + y.sin();
        let b = y * 6.936_f32 - x.cos();
        let mut acc = Accumulator356::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.706_f32 + y.sin();
        let b = y * 2.206_f32 - x.cos();
        let mut acc = Accumulator356::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.477_f32 + y.sin();
        let b = y * 3.894_f32 - x.cos();
        let mut acc = Accumulator356::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.573_f32 + y.sin();
        let b = y * 4.041_f32 - x.cos();
        let mut acc = Accumulator356::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.558_f32 + y.sin();
        let b = y * 5.238_f32 - x.cos();
        let mut acc = Accumulator356::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.707_f32 + y.sin();
        let b = y * 4.552_f32 - x.cos();
        let mut acc = Accumulator356::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.336_f32 + y.sin();
        let b = y * 0.703_f32 - x.cos();
        let mut acc = Accumulator356::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_356(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_356() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_356(total as u64) % 997) as f32;
        total
    }
}

pub mod m357 {
    use super::*;

    pub struct Accumulator357<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator357<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.105_f32 + y.sin();
        let b = y * 6.751_f32 - x.cos();
        let mut acc = Accumulator357::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.155_f32 + y.sin();
        let b = y * 4.709_f32 - x.cos();
        let mut acc = Accumulator357::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.515_f32 + y.sin();
        let b = y * 7.645_f32 - x.cos();
        let mut acc = Accumulator357::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.91_f32 + y.sin();
        let b = y * 4.802_f32 - x.cos();
        let mut acc = Accumulator357::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.347_f32 + y.sin();
        let b = y * 7.711_f32 - x.cos();
        let mut acc = Accumulator357::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.056_f32 + y.sin();
        let b = y * 0.518_f32 - x.cos();
        let mut acc = Accumulator357::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.684_f32 + y.sin();
        let b = y * 3.734_f32 - x.cos();
        let mut acc = Accumulator357::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.795_f32 + y.sin();
        let b = y * 5.638_f32 - x.cos();
        let mut acc = Accumulator357::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.338_f32 + y.sin();
        let b = y * 4.996_f32 - x.cos();
        let mut acc = Accumulator357::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.37_f32 + y.sin();
        let b = y * 4.541_f32 - x.cos();
        let mut acc = Accumulator357::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.837_f32 + y.sin();
        let b = y * 3.023_f32 - x.cos();
        let mut acc = Accumulator357::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.111_f32 + y.sin();
        let b = y * 1.58_f32 - x.cos();
        let mut acc = Accumulator357::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.324_f32 + y.sin();
        let b = y * 6.242_f32 - x.cos();
        let mut acc = Accumulator357::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.872_f32 + y.sin();
        let b = y * 3.115_f32 - x.cos();
        let mut acc = Accumulator357::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.814_f32 + y.sin();
        let b = y * 2.481_f32 - x.cos();
        let mut acc = Accumulator357::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.88_f32 + y.sin();
        let b = y * 7.472_f32 - x.cos();
        let mut acc = Accumulator357::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.095_f32 + y.sin();
        let b = y * 8.685_f32 - x.cos();
        let mut acc = Accumulator357::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.77_f32 + y.sin();
        let b = y * 4.34_f32 - x.cos();
        let mut acc = Accumulator357::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.791_f32 + y.sin();
        let b = y * 4.553_f32 - x.cos();
        let mut acc = Accumulator357::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.478_f32 + y.sin();
        let b = y * 7.589_f32 - x.cos();
        let mut acc = Accumulator357::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_357(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_357() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_357(total as u64) % 997) as f32;
        total
    }
}

pub mod m358 {
    use super::*;

    pub struct Accumulator358<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator358<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.518_f32 + y.sin();
        let b = y * 7.305_f32 - x.cos();
        let mut acc = Accumulator358::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.601_f32 + y.sin();
        let b = y * 0.212_f32 - x.cos();
        let mut acc = Accumulator358::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.259_f32 + y.sin();
        let b = y * 7.954_f32 - x.cos();
        let mut acc = Accumulator358::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.526_f32 + y.sin();
        let b = y * 0.993_f32 - x.cos();
        let mut acc = Accumulator358::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.945_f32 + y.sin();
        let b = y * 3.365_f32 - x.cos();
        let mut acc = Accumulator358::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.27_f32 + y.sin();
        let b = y * 6.5_f32 - x.cos();
        let mut acc = Accumulator358::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.563_f32 + y.sin();
        let b = y * 8.995_f32 - x.cos();
        let mut acc = Accumulator358::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.276_f32 + y.sin();
        let b = y * 9.212_f32 - x.cos();
        let mut acc = Accumulator358::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.618_f32 + y.sin();
        let b = y * 1.84_f32 - x.cos();
        let mut acc = Accumulator358::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.761_f32 + y.sin();
        let b = y * 8.898_f32 - x.cos();
        let mut acc = Accumulator358::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.523_f32 + y.sin();
        let b = y * 5.155_f32 - x.cos();
        let mut acc = Accumulator358::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.123_f32 + y.sin();
        let b = y * 1.741_f32 - x.cos();
        let mut acc = Accumulator358::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.195_f32 + y.sin();
        let b = y * 2.086_f32 - x.cos();
        let mut acc = Accumulator358::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.698_f32 + y.sin();
        let b = y * 2.96_f32 - x.cos();
        let mut acc = Accumulator358::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.722_f32 + y.sin();
        let b = y * 2.336_f32 - x.cos();
        let mut acc = Accumulator358::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.901_f32 + y.sin();
        let b = y * 8.846_f32 - x.cos();
        let mut acc = Accumulator358::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.267_f32 + y.sin();
        let b = y * 8.704_f32 - x.cos();
        let mut acc = Accumulator358::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.098_f32 + y.sin();
        let b = y * 3.192_f32 - x.cos();
        let mut acc = Accumulator358::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.601_f32 + y.sin();
        let b = y * 9.214_f32 - x.cos();
        let mut acc = Accumulator358::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.186_f32 + y.sin();
        let b = y * 6.095_f32 - x.cos();
        let mut acc = Accumulator358::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_358(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m358-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_358() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_358(total as u64) % 997) as f32;
        total
    }
}

pub mod m359 {
    use super::*;

    pub struct Accumulator359<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator359<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.856_f32 + y.sin();
        let b = y * 3.379_f32 - x.cos();
        let mut acc = Accumulator359::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.669_f32 + y.sin();
        let b = y * 7.934_f32 - x.cos();
        let mut acc = Accumulator359::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.54_f32 + y.sin();
        let b = y * 3.31_f32 - x.cos();
        let mut acc = Accumulator359::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.362_f32 + y.sin();
        let b = y * 9.592_f32 - x.cos();
        let mut acc = Accumulator359::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.277_f32 + y.sin();
        let b = y * 3.391_f32 - x.cos();
        let mut acc = Accumulator359::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.788_f32 + y.sin();
        let b = y * 4.333_f32 - x.cos();
        let mut acc = Accumulator359::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.656_f32 + y.sin();
        let b = y * 6.995_f32 - x.cos();
        let mut acc = Accumulator359::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.195_f32 + y.sin();
        let b = y * 4.117_f32 - x.cos();
        let mut acc = Accumulator359::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.831_f32 + y.sin();
        let b = y * 9.112_f32 - x.cos();
        let mut acc = Accumulator359::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.995_f32 + y.sin();
        let b = y * 5.148_f32 - x.cos();
        let mut acc = Accumulator359::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.925_f32 + y.sin();
        let b = y * 1.236_f32 - x.cos();
        let mut acc = Accumulator359::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.127_f32 + y.sin();
        let b = y * 9.007_f32 - x.cos();
        let mut acc = Accumulator359::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.635_f32 + y.sin();
        let b = y * 2.441_f32 - x.cos();
        let mut acc = Accumulator359::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.46_f32 + y.sin();
        let b = y * 7.384_f32 - x.cos();
        let mut acc = Accumulator359::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.164_f32 + y.sin();
        let b = y * 5.917_f32 - x.cos();
        let mut acc = Accumulator359::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.61_f32 + y.sin();
        let b = y * 9.82_f32 - x.cos();
        let mut acc = Accumulator359::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.532_f32 + y.sin();
        let b = y * 8.743_f32 - x.cos();
        let mut acc = Accumulator359::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.289_f32 + y.sin();
        let b = y * 4.083_f32 - x.cos();
        let mut acc = Accumulator359::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.966_f32 + y.sin();
        let b = y * 0.989_f32 - x.cos();
        let mut acc = Accumulator359::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.415_f32 + y.sin();
        let b = y * 6.498_f32 - x.cos();
        let mut acc = Accumulator359::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_359(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_359() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_359(total as u64) % 997) as f32;
        total
    }
}

pub mod m360 {
    use super::*;

    pub struct Accumulator360<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator360<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.575_f32 + y.sin();
        let b = y * 5.817_f32 - x.cos();
        let mut acc = Accumulator360::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.186_f32 + y.sin();
        let b = y * 6.466_f32 - x.cos();
        let mut acc = Accumulator360::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.53_f32 + y.sin();
        let b = y * 0.944_f32 - x.cos();
        let mut acc = Accumulator360::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.333_f32 + y.sin();
        let b = y * 8.291_f32 - x.cos();
        let mut acc = Accumulator360::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.814_f32 + y.sin();
        let b = y * 3.676_f32 - x.cos();
        let mut acc = Accumulator360::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.252_f32 + y.sin();
        let b = y * 2.568_f32 - x.cos();
        let mut acc = Accumulator360::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.091_f32 + y.sin();
        let b = y * 4.447_f32 - x.cos();
        let mut acc = Accumulator360::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.848_f32 + y.sin();
        let b = y * 9.539_f32 - x.cos();
        let mut acc = Accumulator360::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.699_f32 + y.sin();
        let b = y * 4.386_f32 - x.cos();
        let mut acc = Accumulator360::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.754_f32 + y.sin();
        let b = y * 2.09_f32 - x.cos();
        let mut acc = Accumulator360::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.494_f32 + y.sin();
        let b = y * 5.628_f32 - x.cos();
        let mut acc = Accumulator360::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.883_f32 + y.sin();
        let b = y * 1.66_f32 - x.cos();
        let mut acc = Accumulator360::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.552_f32 + y.sin();
        let b = y * 3.963_f32 - x.cos();
        let mut acc = Accumulator360::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.107_f32 + y.sin();
        let b = y * 9.898_f32 - x.cos();
        let mut acc = Accumulator360::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.146_f32 + y.sin();
        let b = y * 0.69_f32 - x.cos();
        let mut acc = Accumulator360::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.081_f32 + y.sin();
        let b = y * 3.178_f32 - x.cos();
        let mut acc = Accumulator360::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.896_f32 + y.sin();
        let b = y * 3.38_f32 - x.cos();
        let mut acc = Accumulator360::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.366_f32 + y.sin();
        let b = y * 9.752_f32 - x.cos();
        let mut acc = Accumulator360::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.378_f32 + y.sin();
        let b = y * 2.073_f32 - x.cos();
        let mut acc = Accumulator360::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.399_f32 + y.sin();
        let b = y * 4.669_f32 - x.cos();
        let mut acc = Accumulator360::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_360(seed: u64) -> u64 {
        let re = Regex::new(r"m360-(\d+)").unwrap();
        let hay = format!("m360-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_360() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_360(total as u64) % 997) as f32;
        total
    }
}

pub mod m361 {
    use super::*;

    pub struct Accumulator361<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator361<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.043_f32 + y.sin();
        let b = y * 8.584_f32 - x.cos();
        let mut acc = Accumulator361::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.632_f32 + y.sin();
        let b = y * 3.455_f32 - x.cos();
        let mut acc = Accumulator361::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.414_f32 + y.sin();
        let b = y * 5.857_f32 - x.cos();
        let mut acc = Accumulator361::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.69_f32 + y.sin();
        let b = y * 5.891_f32 - x.cos();
        let mut acc = Accumulator361::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.62_f32 + y.sin();
        let b = y * 6.081_f32 - x.cos();
        let mut acc = Accumulator361::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.843_f32 + y.sin();
        let b = y * 0.463_f32 - x.cos();
        let mut acc = Accumulator361::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.753_f32 + y.sin();
        let b = y * 2.231_f32 - x.cos();
        let mut acc = Accumulator361::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.795_f32 + y.sin();
        let b = y * 0.946_f32 - x.cos();
        let mut acc = Accumulator361::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.198_f32 + y.sin();
        let b = y * 8.918_f32 - x.cos();
        let mut acc = Accumulator361::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.121_f32 + y.sin();
        let b = y * 3.169_f32 - x.cos();
        let mut acc = Accumulator361::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.241_f32 + y.sin();
        let b = y * 5.509_f32 - x.cos();
        let mut acc = Accumulator361::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.454_f32 + y.sin();
        let b = y * 1.997_f32 - x.cos();
        let mut acc = Accumulator361::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.375_f32 + y.sin();
        let b = y * 2.48_f32 - x.cos();
        let mut acc = Accumulator361::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.444_f32 + y.sin();
        let b = y * 1.483_f32 - x.cos();
        let mut acc = Accumulator361::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.564_f32 + y.sin();
        let b = y * 1.829_f32 - x.cos();
        let mut acc = Accumulator361::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.489_f32 + y.sin();
        let b = y * 0.56_f32 - x.cos();
        let mut acc = Accumulator361::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.839_f32 + y.sin();
        let b = y * 1.65_f32 - x.cos();
        let mut acc = Accumulator361::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.501_f32 + y.sin();
        let b = y * 3.103_f32 - x.cos();
        let mut acc = Accumulator361::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.423_f32 + y.sin();
        let b = y * 2.901_f32 - x.cos();
        let mut acc = Accumulator361::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.899_f32 + y.sin();
        let b = y * 3.728_f32 - x.cos();
        let mut acc = Accumulator361::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_361(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_361() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_361(total as u64) % 997) as f32;
        total
    }
}

pub mod m362 {
    use super::*;

    pub struct Accumulator362<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator362<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.07_f32 + y.sin();
        let b = y * 9.329_f32 - x.cos();
        let mut acc = Accumulator362::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.625_f32 + y.sin();
        let b = y * 7.415_f32 - x.cos();
        let mut acc = Accumulator362::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.996_f32 + y.sin();
        let b = y * 4.903_f32 - x.cos();
        let mut acc = Accumulator362::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.59_f32 + y.sin();
        let b = y * 6.402_f32 - x.cos();
        let mut acc = Accumulator362::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.557_f32 + y.sin();
        let b = y * 6.148_f32 - x.cos();
        let mut acc = Accumulator362::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.426_f32 + y.sin();
        let b = y * 2.523_f32 - x.cos();
        let mut acc = Accumulator362::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.753_f32 + y.sin();
        let b = y * 1.169_f32 - x.cos();
        let mut acc = Accumulator362::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.535_f32 + y.sin();
        let b = y * 3.191_f32 - x.cos();
        let mut acc = Accumulator362::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.359_f32 + y.sin();
        let b = y * 1.507_f32 - x.cos();
        let mut acc = Accumulator362::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.857_f32 + y.sin();
        let b = y * 9.03_f32 - x.cos();
        let mut acc = Accumulator362::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.563_f32 + y.sin();
        let b = y * 5.097_f32 - x.cos();
        let mut acc = Accumulator362::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.466_f32 + y.sin();
        let b = y * 9.201_f32 - x.cos();
        let mut acc = Accumulator362::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.378_f32 + y.sin();
        let b = y * 9.759_f32 - x.cos();
        let mut acc = Accumulator362::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.635_f32 + y.sin();
        let b = y * 1.528_f32 - x.cos();
        let mut acc = Accumulator362::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.586_f32 + y.sin();
        let b = y * 4.228_f32 - x.cos();
        let mut acc = Accumulator362::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.557_f32 + y.sin();
        let b = y * 9.458_f32 - x.cos();
        let mut acc = Accumulator362::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.846_f32 + y.sin();
        let b = y * 2.078_f32 - x.cos();
        let mut acc = Accumulator362::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.284_f32 + y.sin();
        let b = y * 1.033_f32 - x.cos();
        let mut acc = Accumulator362::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.967_f32 + y.sin();
        let b = y * 5.031_f32 - x.cos();
        let mut acc = Accumulator362::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.663_f32 + y.sin();
        let b = y * 0.773_f32 - x.cos();
        let mut acc = Accumulator362::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_362(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(362u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_362() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_362(total as u64) % 997) as f32;
        total
    }
}

pub mod m363 {
    use super::*;

    pub struct Accumulator363<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator363<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.343_f32 + y.sin();
        let b = y * 6.439_f32 - x.cos();
        let mut acc = Accumulator363::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.343_f32 + y.sin();
        let b = y * 2.222_f32 - x.cos();
        let mut acc = Accumulator363::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.239_f32 + y.sin();
        let b = y * 3.743_f32 - x.cos();
        let mut acc = Accumulator363::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.909_f32 + y.sin();
        let b = y * 1.529_f32 - x.cos();
        let mut acc = Accumulator363::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.1_f32 + y.sin();
        let b = y * 0.971_f32 - x.cos();
        let mut acc = Accumulator363::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.569_f32 + y.sin();
        let b = y * 2.447_f32 - x.cos();
        let mut acc = Accumulator363::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.155_f32 + y.sin();
        let b = y * 7.661_f32 - x.cos();
        let mut acc = Accumulator363::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.405_f32 + y.sin();
        let b = y * 8.846_f32 - x.cos();
        let mut acc = Accumulator363::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.398_f32 + y.sin();
        let b = y * 5.218_f32 - x.cos();
        let mut acc = Accumulator363::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.832_f32 + y.sin();
        let b = y * 3.004_f32 - x.cos();
        let mut acc = Accumulator363::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.194_f32 + y.sin();
        let b = y * 4.012_f32 - x.cos();
        let mut acc = Accumulator363::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.085_f32 + y.sin();
        let b = y * 7.758_f32 - x.cos();
        let mut acc = Accumulator363::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.698_f32 + y.sin();
        let b = y * 8.487_f32 - x.cos();
        let mut acc = Accumulator363::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.367_f32 + y.sin();
        let b = y * 9.559_f32 - x.cos();
        let mut acc = Accumulator363::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.14_f32 + y.sin();
        let b = y * 3.444_f32 - x.cos();
        let mut acc = Accumulator363::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.231_f32 + y.sin();
        let b = y * 8.759_f32 - x.cos();
        let mut acc = Accumulator363::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.623_f32 + y.sin();
        let b = y * 5.02_f32 - x.cos();
        let mut acc = Accumulator363::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.968_f32 + y.sin();
        let b = y * 8.25_f32 - x.cos();
        let mut acc = Accumulator363::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.953_f32 + y.sin();
        let b = y * 7.684_f32 - x.cos();
        let mut acc = Accumulator363::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.919_f32 + y.sin();
        let b = y * 6.787_f32 - x.cos();
        let mut acc = Accumulator363::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_363(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_363() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_363(total as u64) % 997) as f32;
        total
    }
}

pub mod m364 {
    use super::*;

    pub struct Accumulator364<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator364<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.634_f32 + y.sin();
        let b = y * 5.704_f32 - x.cos();
        let mut acc = Accumulator364::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.608_f32 + y.sin();
        let b = y * 6.25_f32 - x.cos();
        let mut acc = Accumulator364::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.558_f32 + y.sin();
        let b = y * 2.265_f32 - x.cos();
        let mut acc = Accumulator364::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.932_f32 + y.sin();
        let b = y * 9.102_f32 - x.cos();
        let mut acc = Accumulator364::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.578_f32 + y.sin();
        let b = y * 7.921_f32 - x.cos();
        let mut acc = Accumulator364::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.332_f32 + y.sin();
        let b = y * 0.749_f32 - x.cos();
        let mut acc = Accumulator364::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.285_f32 + y.sin();
        let b = y * 1.938_f32 - x.cos();
        let mut acc = Accumulator364::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.566_f32 + y.sin();
        let b = y * 0.923_f32 - x.cos();
        let mut acc = Accumulator364::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.083_f32 + y.sin();
        let b = y * 0.681_f32 - x.cos();
        let mut acc = Accumulator364::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.707_f32 + y.sin();
        let b = y * 8.309_f32 - x.cos();
        let mut acc = Accumulator364::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.204_f32 + y.sin();
        let b = y * 1.065_f32 - x.cos();
        let mut acc = Accumulator364::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.663_f32 + y.sin();
        let b = y * 1.126_f32 - x.cos();
        let mut acc = Accumulator364::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.934_f32 + y.sin();
        let b = y * 1.127_f32 - x.cos();
        let mut acc = Accumulator364::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.703_f32 + y.sin();
        let b = y * 7.38_f32 - x.cos();
        let mut acc = Accumulator364::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.912_f32 + y.sin();
        let b = y * 9.394_f32 - x.cos();
        let mut acc = Accumulator364::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.861_f32 + y.sin();
        let b = y * 4.497_f32 - x.cos();
        let mut acc = Accumulator364::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.789_f32 + y.sin();
        let b = y * 5.267_f32 - x.cos();
        let mut acc = Accumulator364::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.118_f32 + y.sin();
        let b = y * 1.76_f32 - x.cos();
        let mut acc = Accumulator364::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.57_f32 + y.sin();
        let b = y * 4.259_f32 - x.cos();
        let mut acc = Accumulator364::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.816_f32 + y.sin();
        let b = y * 0.934_f32 - x.cos();
        let mut acc = Accumulator364::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_364(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_364() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_364(total as u64) % 997) as f32;
        total
    }
}

pub mod m365 {
    use super::*;

    pub struct Accumulator365<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator365<T> {
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
        let b = y * 9.488_f32 - x.cos();
        let mut acc = Accumulator365::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.785_f32 + y.sin();
        let b = y * 2.329_f32 - x.cos();
        let mut acc = Accumulator365::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.691_f32 + y.sin();
        let b = y * 1.437_f32 - x.cos();
        let mut acc = Accumulator365::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.414_f32 + y.sin();
        let b = y * 5.025_f32 - x.cos();
        let mut acc = Accumulator365::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.807_f32 + y.sin();
        let b = y * 2.843_f32 - x.cos();
        let mut acc = Accumulator365::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.175_f32 + y.sin();
        let b = y * 1.817_f32 - x.cos();
        let mut acc = Accumulator365::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.245_f32 + y.sin();
        let b = y * 4.274_f32 - x.cos();
        let mut acc = Accumulator365::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.98_f32 + y.sin();
        let b = y * 7.303_f32 - x.cos();
        let mut acc = Accumulator365::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.966_f32 + y.sin();
        let b = y * 9.077_f32 - x.cos();
        let mut acc = Accumulator365::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.315_f32 + y.sin();
        let b = y * 9.806_f32 - x.cos();
        let mut acc = Accumulator365::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.955_f32 + y.sin();
        let b = y * 9.136_f32 - x.cos();
        let mut acc = Accumulator365::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.948_f32 + y.sin();
        let b = y * 2.242_f32 - x.cos();
        let mut acc = Accumulator365::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.417_f32 + y.sin();
        let b = y * 4.768_f32 - x.cos();
        let mut acc = Accumulator365::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.6_f32 + y.sin();
        let b = y * 0.241_f32 - x.cos();
        let mut acc = Accumulator365::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.607_f32 + y.sin();
        let b = y * 8.542_f32 - x.cos();
        let mut acc = Accumulator365::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.206_f32 + y.sin();
        let b = y * 3.271_f32 - x.cos();
        let mut acc = Accumulator365::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.77_f32 + y.sin();
        let b = y * 6.125_f32 - x.cos();
        let mut acc = Accumulator365::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.4_f32 + y.sin();
        let b = y * 3.679_f32 - x.cos();
        let mut acc = Accumulator365::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.772_f32 + y.sin();
        let b = y * 2.954_f32 - x.cos();
        let mut acc = Accumulator365::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.813_f32 + y.sin();
        let b = y * 8.015_f32 - x.cos();
        let mut acc = Accumulator365::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_365(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m365-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_365() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_365(total as u64) % 997) as f32;
        total
    }
}

pub mod m366 {
    use super::*;

    pub struct Accumulator366<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator366<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.52_f32 + y.sin();
        let b = y * 0.482_f32 - x.cos();
        let mut acc = Accumulator366::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.524_f32 + y.sin();
        let b = y * 4.413_f32 - x.cos();
        let mut acc = Accumulator366::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.546_f32 + y.sin();
        let b = y * 3.217_f32 - x.cos();
        let mut acc = Accumulator366::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.663_f32 + y.sin();
        let b = y * 3.391_f32 - x.cos();
        let mut acc = Accumulator366::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.698_f32 + y.sin();
        let b = y * 3.238_f32 - x.cos();
        let mut acc = Accumulator366::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.691_f32 + y.sin();
        let b = y * 9.436_f32 - x.cos();
        let mut acc = Accumulator366::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.154_f32 + y.sin();
        let b = y * 4.88_f32 - x.cos();
        let mut acc = Accumulator366::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.361_f32 + y.sin();
        let b = y * 8.381_f32 - x.cos();
        let mut acc = Accumulator366::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.721_f32 + y.sin();
        let b = y * 4.174_f32 - x.cos();
        let mut acc = Accumulator366::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.455_f32 + y.sin();
        let b = y * 6.561_f32 - x.cos();
        let mut acc = Accumulator366::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.738_f32 + y.sin();
        let b = y * 3.202_f32 - x.cos();
        let mut acc = Accumulator366::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.052_f32 + y.sin();
        let b = y * 0.123_f32 - x.cos();
        let mut acc = Accumulator366::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.771_f32 + y.sin();
        let b = y * 8.802_f32 - x.cos();
        let mut acc = Accumulator366::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.561_f32 + y.sin();
        let b = y * 5.228_f32 - x.cos();
        let mut acc = Accumulator366::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.895_f32 + y.sin();
        let b = y * 8.648_f32 - x.cos();
        let mut acc = Accumulator366::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.654_f32 + y.sin();
        let b = y * 4.332_f32 - x.cos();
        let mut acc = Accumulator366::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.051_f32 + y.sin();
        let b = y * 2.6_f32 - x.cos();
        let mut acc = Accumulator366::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.104_f32 + y.sin();
        let b = y * 2.041_f32 - x.cos();
        let mut acc = Accumulator366::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.91_f32 + y.sin();
        let b = y * 2.861_f32 - x.cos();
        let mut acc = Accumulator366::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.697_f32 + y.sin();
        let b = y * 2.578_f32 - x.cos();
        let mut acc = Accumulator366::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_366(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_366() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_366(total as u64) % 997) as f32;
        total
    }
}

pub mod m367 {
    use super::*;

    pub struct Accumulator367<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator367<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.214_f32 + y.sin();
        let b = y * 3.412_f32 - x.cos();
        let mut acc = Accumulator367::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.289_f32 + y.sin();
        let b = y * 9.537_f32 - x.cos();
        let mut acc = Accumulator367::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.26_f32 + y.sin();
        let b = y * 1.303_f32 - x.cos();
        let mut acc = Accumulator367::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.846_f32 + y.sin();
        let b = y * 7.561_f32 - x.cos();
        let mut acc = Accumulator367::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.595_f32 + y.sin();
        let b = y * 6.687_f32 - x.cos();
        let mut acc = Accumulator367::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.307_f32 + y.sin();
        let b = y * 8.088_f32 - x.cos();
        let mut acc = Accumulator367::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.267_f32 + y.sin();
        let b = y * 4.494_f32 - x.cos();
        let mut acc = Accumulator367::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.454_f32 + y.sin();
        let b = y * 1.25_f32 - x.cos();
        let mut acc = Accumulator367::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.207_f32 + y.sin();
        let b = y * 7.715_f32 - x.cos();
        let mut acc = Accumulator367::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.3_f32 + y.sin();
        let b = y * 8.196_f32 - x.cos();
        let mut acc = Accumulator367::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.401_f32 + y.sin();
        let b = y * 4.234_f32 - x.cos();
        let mut acc = Accumulator367::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.874_f32 + y.sin();
        let b = y * 9.701_f32 - x.cos();
        let mut acc = Accumulator367::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.088_f32 + y.sin();
        let b = y * 8.308_f32 - x.cos();
        let mut acc = Accumulator367::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.626_f32 + y.sin();
        let b = y * 6.865_f32 - x.cos();
        let mut acc = Accumulator367::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.407_f32 + y.sin();
        let b = y * 2.315_f32 - x.cos();
        let mut acc = Accumulator367::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.034_f32 + y.sin();
        let b = y * 6.649_f32 - x.cos();
        let mut acc = Accumulator367::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.221_f32 + y.sin();
        let b = y * 7.572_f32 - x.cos();
        let mut acc = Accumulator367::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.649_f32 + y.sin();
        let b = y * 7.765_f32 - x.cos();
        let mut acc = Accumulator367::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.812_f32 + y.sin();
        let b = y * 2.278_f32 - x.cos();
        let mut acc = Accumulator367::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.076_f32 + y.sin();
        let b = y * 1.435_f32 - x.cos();
        let mut acc = Accumulator367::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_367(seed: u64) -> u64 {
        let re = Regex::new(r"m367-(\d+)").unwrap();
        let hay = format!("m367-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_367() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_367(total as u64) % 997) as f32;
        total
    }
}

pub mod m368 {
    use super::*;

    pub struct Accumulator368<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator368<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.884_f32 + y.sin();
        let b = y * 7.309_f32 - x.cos();
        let mut acc = Accumulator368::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.372_f32 + y.sin();
        let b = y * 7.793_f32 - x.cos();
        let mut acc = Accumulator368::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.266_f32 + y.sin();
        let b = y * 8.575_f32 - x.cos();
        let mut acc = Accumulator368::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.806_f32 + y.sin();
        let b = y * 3.681_f32 - x.cos();
        let mut acc = Accumulator368::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.51_f32 + y.sin();
        let b = y * 7.444_f32 - x.cos();
        let mut acc = Accumulator368::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.422_f32 + y.sin();
        let b = y * 3.156_f32 - x.cos();
        let mut acc = Accumulator368::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.944_f32 + y.sin();
        let b = y * 7.633_f32 - x.cos();
        let mut acc = Accumulator368::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.953_f32 + y.sin();
        let b = y * 3.219_f32 - x.cos();
        let mut acc = Accumulator368::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.934_f32 + y.sin();
        let b = y * 2.982_f32 - x.cos();
        let mut acc = Accumulator368::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.459_f32 + y.sin();
        let b = y * 7.5_f32 - x.cos();
        let mut acc = Accumulator368::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.919_f32 + y.sin();
        let b = y * 9.812_f32 - x.cos();
        let mut acc = Accumulator368::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.533_f32 + y.sin();
        let b = y * 5.11_f32 - x.cos();
        let mut acc = Accumulator368::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.621_f32 + y.sin();
        let b = y * 0.219_f32 - x.cos();
        let mut acc = Accumulator368::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.716_f32 + y.sin();
        let b = y * 5.588_f32 - x.cos();
        let mut acc = Accumulator368::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.081_f32 + y.sin();
        let b = y * 5.963_f32 - x.cos();
        let mut acc = Accumulator368::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.868_f32 + y.sin();
        let b = y * 3.284_f32 - x.cos();
        let mut acc = Accumulator368::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.26_f32 + y.sin();
        let b = y * 6.137_f32 - x.cos();
        let mut acc = Accumulator368::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.405_f32 + y.sin();
        let b = y * 1.767_f32 - x.cos();
        let mut acc = Accumulator368::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.617_f32 + y.sin();
        let b = y * 5.833_f32 - x.cos();
        let mut acc = Accumulator368::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.239_f32 + y.sin();
        let b = y * 6.264_f32 - x.cos();
        let mut acc = Accumulator368::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_368(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_368() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_368(total as u64) % 997) as f32;
        total
    }
}

pub mod m369 {
    use super::*;

    pub struct Accumulator369<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator369<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.001_f32 + y.sin();
        let b = y * 3.642_f32 - x.cos();
        let mut acc = Accumulator369::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.767_f32 + y.sin();
        let b = y * 1.815_f32 - x.cos();
        let mut acc = Accumulator369::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.171_f32 + y.sin();
        let b = y * 6.348_f32 - x.cos();
        let mut acc = Accumulator369::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.855_f32 + y.sin();
        let b = y * 3.308_f32 - x.cos();
        let mut acc = Accumulator369::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.005_f32 + y.sin();
        let b = y * 3.269_f32 - x.cos();
        let mut acc = Accumulator369::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.059_f32 + y.sin();
        let b = y * 0.5_f32 - x.cos();
        let mut acc = Accumulator369::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.997_f32 + y.sin();
        let b = y * 0.115_f32 - x.cos();
        let mut acc = Accumulator369::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.681_f32 + y.sin();
        let b = y * 6.661_f32 - x.cos();
        let mut acc = Accumulator369::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.926_f32 + y.sin();
        let b = y * 8.381_f32 - x.cos();
        let mut acc = Accumulator369::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.246_f32 + y.sin();
        let b = y * 2.889_f32 - x.cos();
        let mut acc = Accumulator369::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.624_f32 + y.sin();
        let b = y * 7.724_f32 - x.cos();
        let mut acc = Accumulator369::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.778_f32 + y.sin();
        let b = y * 9.734_f32 - x.cos();
        let mut acc = Accumulator369::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.151_f32 + y.sin();
        let b = y * 7.288_f32 - x.cos();
        let mut acc = Accumulator369::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.602_f32 + y.sin();
        let b = y * 8.415_f32 - x.cos();
        let mut acc = Accumulator369::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.154_f32 + y.sin();
        let b = y * 1.572_f32 - x.cos();
        let mut acc = Accumulator369::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.773_f32 + y.sin();
        let b = y * 1.79_f32 - x.cos();
        let mut acc = Accumulator369::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.01_f32 + y.sin();
        let b = y * 5.98_f32 - x.cos();
        let mut acc = Accumulator369::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.464_f32 + y.sin();
        let b = y * 8.712_f32 - x.cos();
        let mut acc = Accumulator369::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.482_f32 + y.sin();
        let b = y * 2.113_f32 - x.cos();
        let mut acc = Accumulator369::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.391_f32 + y.sin();
        let b = y * 8.505_f32 - x.cos();
        let mut acc = Accumulator369::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_369(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(369u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_369() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_369(total as u64) % 997) as f32;
        total
    }
}

pub mod m370 {
    use super::*;

    pub struct Accumulator370<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator370<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.38_f32 + y.sin();
        let b = y * 9.863_f32 - x.cos();
        let mut acc = Accumulator370::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.595_f32 + y.sin();
        let b = y * 2.238_f32 - x.cos();
        let mut acc = Accumulator370::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.395_f32 + y.sin();
        let b = y * 9.532_f32 - x.cos();
        let mut acc = Accumulator370::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.416_f32 + y.sin();
        let b = y * 0.519_f32 - x.cos();
        let mut acc = Accumulator370::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.241_f32 + y.sin();
        let b = y * 4.419_f32 - x.cos();
        let mut acc = Accumulator370::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.729_f32 + y.sin();
        let b = y * 1.527_f32 - x.cos();
        let mut acc = Accumulator370::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.162_f32 + y.sin();
        let b = y * 6.378_f32 - x.cos();
        let mut acc = Accumulator370::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.002_f32 + y.sin();
        let b = y * 6.578_f32 - x.cos();
        let mut acc = Accumulator370::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.336_f32 + y.sin();
        let b = y * 5.136_f32 - x.cos();
        let mut acc = Accumulator370::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.355_f32 + y.sin();
        let b = y * 7.396_f32 - x.cos();
        let mut acc = Accumulator370::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.833_f32 + y.sin();
        let b = y * 0.856_f32 - x.cos();
        let mut acc = Accumulator370::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.295_f32 + y.sin();
        let b = y * 7.382_f32 - x.cos();
        let mut acc = Accumulator370::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.329_f32 + y.sin();
        let b = y * 2.084_f32 - x.cos();
        let mut acc = Accumulator370::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.312_f32 + y.sin();
        let b = y * 6.885_f32 - x.cos();
        let mut acc = Accumulator370::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.233_f32 + y.sin();
        let b = y * 9.417_f32 - x.cos();
        let mut acc = Accumulator370::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.4_f32 + y.sin();
        let b = y * 6.057_f32 - x.cos();
        let mut acc = Accumulator370::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.809_f32 + y.sin();
        let b = y * 1.691_f32 - x.cos();
        let mut acc = Accumulator370::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.482_f32 + y.sin();
        let b = y * 6.776_f32 - x.cos();
        let mut acc = Accumulator370::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.509_f32 + y.sin();
        let b = y * 2.336_f32 - x.cos();
        let mut acc = Accumulator370::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.464_f32 + y.sin();
        let b = y * 5.611_f32 - x.cos();
        let mut acc = Accumulator370::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_370(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_370() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_370(total as u64) % 997) as f32;
        total
    }
}

pub mod m371 {
    use super::*;

    pub struct Accumulator371<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator371<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.072_f32 + y.sin();
        let b = y * 5.881_f32 - x.cos();
        let mut acc = Accumulator371::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.613_f32 + y.sin();
        let b = y * 5.319_f32 - x.cos();
        let mut acc = Accumulator371::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.961_f32 + y.sin();
        let b = y * 8.14_f32 - x.cos();
        let mut acc = Accumulator371::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.113_f32 + y.sin();
        let b = y * 2.748_f32 - x.cos();
        let mut acc = Accumulator371::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.784_f32 + y.sin();
        let b = y * 0.665_f32 - x.cos();
        let mut acc = Accumulator371::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.558_f32 + y.sin();
        let b = y * 8.169_f32 - x.cos();
        let mut acc = Accumulator371::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.784_f32 + y.sin();
        let b = y * 2.19_f32 - x.cos();
        let mut acc = Accumulator371::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.549_f32 + y.sin();
        let b = y * 6.577_f32 - x.cos();
        let mut acc = Accumulator371::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.711_f32 + y.sin();
        let b = y * 2.282_f32 - x.cos();
        let mut acc = Accumulator371::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.524_f32 + y.sin();
        let b = y * 9.344_f32 - x.cos();
        let mut acc = Accumulator371::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.75_f32 + y.sin();
        let b = y * 6.913_f32 - x.cos();
        let mut acc = Accumulator371::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.99_f32 + y.sin();
        let b = y * 8.802_f32 - x.cos();
        let mut acc = Accumulator371::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.214_f32 + y.sin();
        let b = y * 7.461_f32 - x.cos();
        let mut acc = Accumulator371::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.959_f32 + y.sin();
        let b = y * 7.0_f32 - x.cos();
        let mut acc = Accumulator371::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.435_f32 + y.sin();
        let b = y * 2.497_f32 - x.cos();
        let mut acc = Accumulator371::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.749_f32 + y.sin();
        let b = y * 2.275_f32 - x.cos();
        let mut acc = Accumulator371::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.149_f32 + y.sin();
        let b = y * 8.048_f32 - x.cos();
        let mut acc = Accumulator371::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.246_f32 + y.sin();
        let b = y * 6.661_f32 - x.cos();
        let mut acc = Accumulator371::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.736_f32 + y.sin();
        let b = y * 9.861_f32 - x.cos();
        let mut acc = Accumulator371::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.14_f32 + y.sin();
        let b = y * 4.708_f32 - x.cos();
        let mut acc = Accumulator371::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_371(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_371() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_371(total as u64) % 997) as f32;
        total
    }
}

pub mod m372 {
    use super::*;

    pub struct Accumulator372<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator372<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.643_f32 + y.sin();
        let b = y * 1.754_f32 - x.cos();
        let mut acc = Accumulator372::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.223_f32 + y.sin();
        let b = y * 4.563_f32 - x.cos();
        let mut acc = Accumulator372::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.976_f32 + y.sin();
        let b = y * 1.58_f32 - x.cos();
        let mut acc = Accumulator372::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.211_f32 + y.sin();
        let b = y * 4.278_f32 - x.cos();
        let mut acc = Accumulator372::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.168_f32 + y.sin();
        let b = y * 4.197_f32 - x.cos();
        let mut acc = Accumulator372::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.241_f32 + y.sin();
        let b = y * 5.578_f32 - x.cos();
        let mut acc = Accumulator372::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.482_f32 + y.sin();
        let b = y * 6.219_f32 - x.cos();
        let mut acc = Accumulator372::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.494_f32 + y.sin();
        let b = y * 5.126_f32 - x.cos();
        let mut acc = Accumulator372::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.385_f32 + y.sin();
        let b = y * 8.623_f32 - x.cos();
        let mut acc = Accumulator372::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.54_f32 + y.sin();
        let b = y * 6.443_f32 - x.cos();
        let mut acc = Accumulator372::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.352_f32 + y.sin();
        let b = y * 2.759_f32 - x.cos();
        let mut acc = Accumulator372::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.738_f32 + y.sin();
        let b = y * 0.309_f32 - x.cos();
        let mut acc = Accumulator372::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.828_f32 + y.sin();
        let b = y * 9.829_f32 - x.cos();
        let mut acc = Accumulator372::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.143_f32 + y.sin();
        let b = y * 6.778_f32 - x.cos();
        let mut acc = Accumulator372::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.018_f32 + y.sin();
        let b = y * 9.408_f32 - x.cos();
        let mut acc = Accumulator372::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.698_f32 + y.sin();
        let b = y * 5.218_f32 - x.cos();
        let mut acc = Accumulator372::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.358_f32 + y.sin();
        let b = y * 9.154_f32 - x.cos();
        let mut acc = Accumulator372::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.225_f32 + y.sin();
        let b = y * 6.983_f32 - x.cos();
        let mut acc = Accumulator372::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.092_f32 + y.sin();
        let b = y * 4.783_f32 - x.cos();
        let mut acc = Accumulator372::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.683_f32 + y.sin();
        let b = y * 1.055_f32 - x.cos();
        let mut acc = Accumulator372::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_372(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m372-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_372() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_372(total as u64) % 997) as f32;
        total
    }
}

pub mod m373 {
    use super::*;

    pub struct Accumulator373<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator373<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.517_f32 + y.sin();
        let b = y * 2.725_f32 - x.cos();
        let mut acc = Accumulator373::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.674_f32 + y.sin();
        let b = y * 4.556_f32 - x.cos();
        let mut acc = Accumulator373::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.028_f32 + y.sin();
        let b = y * 2.659_f32 - x.cos();
        let mut acc = Accumulator373::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.725_f32 + y.sin();
        let b = y * 4.208_f32 - x.cos();
        let mut acc = Accumulator373::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.146_f32 + y.sin();
        let b = y * 4.751_f32 - x.cos();
        let mut acc = Accumulator373::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.021_f32 + y.sin();
        let b = y * 0.743_f32 - x.cos();
        let mut acc = Accumulator373::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.221_f32 + y.sin();
        let b = y * 5.603_f32 - x.cos();
        let mut acc = Accumulator373::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.911_f32 + y.sin();
        let b = y * 1.924_f32 - x.cos();
        let mut acc = Accumulator373::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.248_f32 + y.sin();
        let b = y * 7.144_f32 - x.cos();
        let mut acc = Accumulator373::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.531_f32 + y.sin();
        let b = y * 8.791_f32 - x.cos();
        let mut acc = Accumulator373::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.404_f32 + y.sin();
        let b = y * 4.964_f32 - x.cos();
        let mut acc = Accumulator373::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.277_f32 + y.sin();
        let b = y * 3.013_f32 - x.cos();
        let mut acc = Accumulator373::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.443_f32 + y.sin();
        let b = y * 5.583_f32 - x.cos();
        let mut acc = Accumulator373::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.347_f32 + y.sin();
        let b = y * 0.623_f32 - x.cos();
        let mut acc = Accumulator373::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.967_f32 + y.sin();
        let b = y * 8.377_f32 - x.cos();
        let mut acc = Accumulator373::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.646_f32 + y.sin();
        let b = y * 3.091_f32 - x.cos();
        let mut acc = Accumulator373::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.029_f32 + y.sin();
        let b = y * 2.482_f32 - x.cos();
        let mut acc = Accumulator373::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.616_f32 + y.sin();
        let b = y * 4.95_f32 - x.cos();
        let mut acc = Accumulator373::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.056_f32 + y.sin();
        let b = y * 2.136_f32 - x.cos();
        let mut acc = Accumulator373::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.822_f32 + y.sin();
        let b = y * 6.553_f32 - x.cos();
        let mut acc = Accumulator373::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_373(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_373() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_373(total as u64) % 997) as f32;
        total
    }
}

pub mod m374 {
    use super::*;

    pub struct Accumulator374<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator374<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.362_f32 + y.sin();
        let b = y * 4.833_f32 - x.cos();
        let mut acc = Accumulator374::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.917_f32 + y.sin();
        let b = y * 6.123_f32 - x.cos();
        let mut acc = Accumulator374::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.002_f32 + y.sin();
        let b = y * 7.559_f32 - x.cos();
        let mut acc = Accumulator374::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.428_f32 + y.sin();
        let b = y * 8.285_f32 - x.cos();
        let mut acc = Accumulator374::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.608_f32 + y.sin();
        let b = y * 1.525_f32 - x.cos();
        let mut acc = Accumulator374::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.29_f32 + y.sin();
        let b = y * 3.476_f32 - x.cos();
        let mut acc = Accumulator374::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.611_f32 + y.sin();
        let b = y * 7.207_f32 - x.cos();
        let mut acc = Accumulator374::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.128_f32 + y.sin();
        let b = y * 4.03_f32 - x.cos();
        let mut acc = Accumulator374::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.838_f32 + y.sin();
        let b = y * 7.9_f32 - x.cos();
        let mut acc = Accumulator374::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.809_f32 + y.sin();
        let b = y * 4.086_f32 - x.cos();
        let mut acc = Accumulator374::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.114_f32 + y.sin();
        let b = y * 0.447_f32 - x.cos();
        let mut acc = Accumulator374::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.293_f32 + y.sin();
        let b = y * 8.812_f32 - x.cos();
        let mut acc = Accumulator374::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.503_f32 + y.sin();
        let b = y * 6.83_f32 - x.cos();
        let mut acc = Accumulator374::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.833_f32 + y.sin();
        let b = y * 1.194_f32 - x.cos();
        let mut acc = Accumulator374::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.231_f32 + y.sin();
        let b = y * 1.511_f32 - x.cos();
        let mut acc = Accumulator374::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.778_f32 + y.sin();
        let b = y * 9.689_f32 - x.cos();
        let mut acc = Accumulator374::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.718_f32 + y.sin();
        let b = y * 6.491_f32 - x.cos();
        let mut acc = Accumulator374::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.129_f32 + y.sin();
        let b = y * 6.287_f32 - x.cos();
        let mut acc = Accumulator374::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.337_f32 + y.sin();
        let b = y * 5.789_f32 - x.cos();
        let mut acc = Accumulator374::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.37_f32 + y.sin();
        let b = y * 0.561_f32 - x.cos();
        let mut acc = Accumulator374::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_374(seed: u64) -> u64 {
        let re = Regex::new(r"m374-(\d+)").unwrap();
        let hay = format!("m374-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_374() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_374(total as u64) % 997) as f32;
        total
    }
}

pub mod m375 {
    use super::*;

    pub struct Accumulator375<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator375<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.025_f32 + y.sin();
        let b = y * 8.749_f32 - x.cos();
        let mut acc = Accumulator375::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.919_f32 + y.sin();
        let b = y * 6.121_f32 - x.cos();
        let mut acc = Accumulator375::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.664_f32 + y.sin();
        let b = y * 5.77_f32 - x.cos();
        let mut acc = Accumulator375::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.931_f32 + y.sin();
        let b = y * 3.015_f32 - x.cos();
        let mut acc = Accumulator375::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.536_f32 + y.sin();
        let b = y * 3.197_f32 - x.cos();
        let mut acc = Accumulator375::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.344_f32 + y.sin();
        let b = y * 9.73_f32 - x.cos();
        let mut acc = Accumulator375::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.252_f32 + y.sin();
        let b = y * 5.625_f32 - x.cos();
        let mut acc = Accumulator375::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.009_f32 + y.sin();
        let b = y * 8.39_f32 - x.cos();
        let mut acc = Accumulator375::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.104_f32 + y.sin();
        let b = y * 6.262_f32 - x.cos();
        let mut acc = Accumulator375::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.993_f32 + y.sin();
        let b = y * 4.335_f32 - x.cos();
        let mut acc = Accumulator375::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.973_f32 + y.sin();
        let b = y * 2.874_f32 - x.cos();
        let mut acc = Accumulator375::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.201_f32 + y.sin();
        let b = y * 5.279_f32 - x.cos();
        let mut acc = Accumulator375::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.984_f32 + y.sin();
        let b = y * 0.7_f32 - x.cos();
        let mut acc = Accumulator375::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.527_f32 + y.sin();
        let b = y * 1.866_f32 - x.cos();
        let mut acc = Accumulator375::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.947_f32 + y.sin();
        let b = y * 4.149_f32 - x.cos();
        let mut acc = Accumulator375::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.949_f32 + y.sin();
        let b = y * 5.107_f32 - x.cos();
        let mut acc = Accumulator375::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.487_f32 + y.sin();
        let b = y * 1.98_f32 - x.cos();
        let mut acc = Accumulator375::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.499_f32 + y.sin();
        let b = y * 7.196_f32 - x.cos();
        let mut acc = Accumulator375::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.649_f32 + y.sin();
        let b = y * 7.963_f32 - x.cos();
        let mut acc = Accumulator375::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.082_f32 + y.sin();
        let b = y * 0.723_f32 - x.cos();
        let mut acc = Accumulator375::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_375(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_375() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_375(total as u64) % 997) as f32;
        total
    }
}

pub mod m376 {
    use super::*;

    pub struct Accumulator376<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator376<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.717_f32 + y.sin();
        let b = y * 4.634_f32 - x.cos();
        let mut acc = Accumulator376::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.276_f32 + y.sin();
        let b = y * 6.625_f32 - x.cos();
        let mut acc = Accumulator376::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.442_f32 + y.sin();
        let b = y * 1.403_f32 - x.cos();
        let mut acc = Accumulator376::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.906_f32 + y.sin();
        let b = y * 3.495_f32 - x.cos();
        let mut acc = Accumulator376::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.656_f32 + y.sin();
        let b = y * 1.967_f32 - x.cos();
        let mut acc = Accumulator376::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.771_f32 + y.sin();
        let b = y * 0.953_f32 - x.cos();
        let mut acc = Accumulator376::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.731_f32 + y.sin();
        let b = y * 1.927_f32 - x.cos();
        let mut acc = Accumulator376::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.204_f32 + y.sin();
        let b = y * 2.932_f32 - x.cos();
        let mut acc = Accumulator376::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.861_f32 + y.sin();
        let b = y * 9.43_f32 - x.cos();
        let mut acc = Accumulator376::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.488_f32 + y.sin();
        let b = y * 9.237_f32 - x.cos();
        let mut acc = Accumulator376::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.836_f32 + y.sin();
        let b = y * 6.751_f32 - x.cos();
        let mut acc = Accumulator376::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.308_f32 + y.sin();
        let b = y * 1.42_f32 - x.cos();
        let mut acc = Accumulator376::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.616_f32 + y.sin();
        let b = y * 6.827_f32 - x.cos();
        let mut acc = Accumulator376::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.201_f32 + y.sin();
        let b = y * 4.451_f32 - x.cos();
        let mut acc = Accumulator376::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.539_f32 + y.sin();
        let b = y * 8.893_f32 - x.cos();
        let mut acc = Accumulator376::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.634_f32 + y.sin();
        let b = y * 0.883_f32 - x.cos();
        let mut acc = Accumulator376::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.189_f32 + y.sin();
        let b = y * 1.328_f32 - x.cos();
        let mut acc = Accumulator376::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.076_f32 + y.sin();
        let b = y * 1.245_f32 - x.cos();
        let mut acc = Accumulator376::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.458_f32 + y.sin();
        let b = y * 3.372_f32 - x.cos();
        let mut acc = Accumulator376::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.727_f32 + y.sin();
        let b = y * 7.164_f32 - x.cos();
        let mut acc = Accumulator376::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_376(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(376u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_376() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_376(total as u64) % 997) as f32;
        total
    }
}

pub mod m377 {
    use super::*;

    pub struct Accumulator377<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator377<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.488_f32 + y.sin();
        let b = y * 6.897_f32 - x.cos();
        let mut acc = Accumulator377::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.572_f32 + y.sin();
        let b = y * 5.823_f32 - x.cos();
        let mut acc = Accumulator377::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.573_f32 + y.sin();
        let b = y * 1.66_f32 - x.cos();
        let mut acc = Accumulator377::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.742_f32 + y.sin();
        let b = y * 3.695_f32 - x.cos();
        let mut acc = Accumulator377::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.538_f32 + y.sin();
        let b = y * 7.573_f32 - x.cos();
        let mut acc = Accumulator377::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.861_f32 + y.sin();
        let b = y * 1.108_f32 - x.cos();
        let mut acc = Accumulator377::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.159_f32 + y.sin();
        let b = y * 0.792_f32 - x.cos();
        let mut acc = Accumulator377::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.994_f32 + y.sin();
        let b = y * 1.578_f32 - x.cos();
        let mut acc = Accumulator377::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.465_f32 + y.sin();
        let b = y * 3.175_f32 - x.cos();
        let mut acc = Accumulator377::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.332_f32 + y.sin();
        let b = y * 2.592_f32 - x.cos();
        let mut acc = Accumulator377::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.488_f32 + y.sin();
        let b = y * 3.83_f32 - x.cos();
        let mut acc = Accumulator377::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.149_f32 + y.sin();
        let b = y * 5.043_f32 - x.cos();
        let mut acc = Accumulator377::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.74_f32 + y.sin();
        let b = y * 3.102_f32 - x.cos();
        let mut acc = Accumulator377::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.603_f32 + y.sin();
        let b = y * 4.269_f32 - x.cos();
        let mut acc = Accumulator377::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.643_f32 + y.sin();
        let b = y * 7.72_f32 - x.cos();
        let mut acc = Accumulator377::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.32_f32 + y.sin();
        let b = y * 6.467_f32 - x.cos();
        let mut acc = Accumulator377::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.144_f32 + y.sin();
        let b = y * 4.959_f32 - x.cos();
        let mut acc = Accumulator377::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.777_f32 + y.sin();
        let b = y * 3.823_f32 - x.cos();
        let mut acc = Accumulator377::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.849_f32 + y.sin();
        let b = y * 9.4_f32 - x.cos();
        let mut acc = Accumulator377::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.249_f32 + y.sin();
        let b = y * 4.602_f32 - x.cos();
        let mut acc = Accumulator377::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_377(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_377() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_377(total as u64) % 997) as f32;
        total
    }
}

pub mod m378 {
    use super::*;

    pub struct Accumulator378<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator378<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.236_f32 + y.sin();
        let b = y * 2.474_f32 - x.cos();
        let mut acc = Accumulator378::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.419_f32 + y.sin();
        let b = y * 5.68_f32 - x.cos();
        let mut acc = Accumulator378::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.502_f32 + y.sin();
        let b = y * 4.218_f32 - x.cos();
        let mut acc = Accumulator378::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.137_f32 + y.sin();
        let b = y * 0.9_f32 - x.cos();
        let mut acc = Accumulator378::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.947_f32 + y.sin();
        let b = y * 5.315_f32 - x.cos();
        let mut acc = Accumulator378::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.489_f32 + y.sin();
        let b = y * 2.129_f32 - x.cos();
        let mut acc = Accumulator378::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.569_f32 + y.sin();
        let b = y * 5.345_f32 - x.cos();
        let mut acc = Accumulator378::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.97_f32 + y.sin();
        let b = y * 2.708_f32 - x.cos();
        let mut acc = Accumulator378::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.243_f32 + y.sin();
        let b = y * 0.275_f32 - x.cos();
        let mut acc = Accumulator378::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.369_f32 + y.sin();
        let b = y * 7.847_f32 - x.cos();
        let mut acc = Accumulator378::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.475_f32 + y.sin();
        let b = y * 7.667_f32 - x.cos();
        let mut acc = Accumulator378::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.664_f32 + y.sin();
        let b = y * 2.226_f32 - x.cos();
        let mut acc = Accumulator378::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.317_f32 + y.sin();
        let b = y * 2.8_f32 - x.cos();
        let mut acc = Accumulator378::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.56_f32 + y.sin();
        let b = y * 1.463_f32 - x.cos();
        let mut acc = Accumulator378::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.583_f32 + y.sin();
        let b = y * 0.148_f32 - x.cos();
        let mut acc = Accumulator378::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.154_f32 + y.sin();
        let b = y * 5.867_f32 - x.cos();
        let mut acc = Accumulator378::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.569_f32 + y.sin();
        let b = y * 4.518_f32 - x.cos();
        let mut acc = Accumulator378::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.533_f32 + y.sin();
        let b = y * 2.43_f32 - x.cos();
        let mut acc = Accumulator378::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.377_f32 + y.sin();
        let b = y * 6.73_f32 - x.cos();
        let mut acc = Accumulator378::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.66_f32 + y.sin();
        let b = y * 1.021_f32 - x.cos();
        let mut acc = Accumulator378::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_378(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_378() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_378(total as u64) % 997) as f32;
        total
    }
}

pub mod m379 {
    use super::*;

    pub struct Accumulator379<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator379<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.601_f32 + y.sin();
        let b = y * 8.916_f32 - x.cos();
        let mut acc = Accumulator379::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.18_f32 + y.sin();
        let b = y * 5.308_f32 - x.cos();
        let mut acc = Accumulator379::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.886_f32 + y.sin();
        let b = y * 2.837_f32 - x.cos();
        let mut acc = Accumulator379::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.254_f32 + y.sin();
        let b = y * 3.97_f32 - x.cos();
        let mut acc = Accumulator379::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.355_f32 + y.sin();
        let b = y * 9.744_f32 - x.cos();
        let mut acc = Accumulator379::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.707_f32 + y.sin();
        let b = y * 1.058_f32 - x.cos();
        let mut acc = Accumulator379::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.679_f32 + y.sin();
        let b = y * 5.134_f32 - x.cos();
        let mut acc = Accumulator379::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.708_f32 + y.sin();
        let b = y * 7.38_f32 - x.cos();
        let mut acc = Accumulator379::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.123_f32 + y.sin();
        let b = y * 5.421_f32 - x.cos();
        let mut acc = Accumulator379::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.054_f32 + y.sin();
        let b = y * 3.645_f32 - x.cos();
        let mut acc = Accumulator379::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.521_f32 + y.sin();
        let b = y * 8.447_f32 - x.cos();
        let mut acc = Accumulator379::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.824_f32 + y.sin();
        let b = y * 2.031_f32 - x.cos();
        let mut acc = Accumulator379::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.286_f32 + y.sin();
        let b = y * 8.78_f32 - x.cos();
        let mut acc = Accumulator379::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.158_f32 + y.sin();
        let b = y * 4.502_f32 - x.cos();
        let mut acc = Accumulator379::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.831_f32 + y.sin();
        let b = y * 6.638_f32 - x.cos();
        let mut acc = Accumulator379::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.636_f32 + y.sin();
        let b = y * 0.15_f32 - x.cos();
        let mut acc = Accumulator379::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.851_f32 + y.sin();
        let b = y * 8.797_f32 - x.cos();
        let mut acc = Accumulator379::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.798_f32 + y.sin();
        let b = y * 6.763_f32 - x.cos();
        let mut acc = Accumulator379::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.406_f32 + y.sin();
        let b = y * 7.741_f32 - x.cos();
        let mut acc = Accumulator379::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.653_f32 + y.sin();
        let b = y * 0.321_f32 - x.cos();
        let mut acc = Accumulator379::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_379(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m379-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_379() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_379(total as u64) % 997) as f32;
        total
    }
}

pub mod m380 {
    use super::*;

    pub struct Accumulator380<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator380<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.411_f32 + y.sin();
        let b = y * 3.035_f32 - x.cos();
        let mut acc = Accumulator380::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.6_f32 + y.sin();
        let b = y * 7.488_f32 - x.cos();
        let mut acc = Accumulator380::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.349_f32 + y.sin();
        let b = y * 1.739_f32 - x.cos();
        let mut acc = Accumulator380::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.395_f32 + y.sin();
        let b = y * 4.457_f32 - x.cos();
        let mut acc = Accumulator380::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.296_f32 + y.sin();
        let b = y * 7.98_f32 - x.cos();
        let mut acc = Accumulator380::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.659_f32 + y.sin();
        let b = y * 1.519_f32 - x.cos();
        let mut acc = Accumulator380::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.54_f32 + y.sin();
        let b = y * 5.854_f32 - x.cos();
        let mut acc = Accumulator380::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.644_f32 + y.sin();
        let b = y * 2.285_f32 - x.cos();
        let mut acc = Accumulator380::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.677_f32 + y.sin();
        let b = y * 2.943_f32 - x.cos();
        let mut acc = Accumulator380::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.775_f32 + y.sin();
        let b = y * 8.084_f32 - x.cos();
        let mut acc = Accumulator380::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.491_f32 + y.sin();
        let b = y * 9.707_f32 - x.cos();
        let mut acc = Accumulator380::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.922_f32 + y.sin();
        let b = y * 9.877_f32 - x.cos();
        let mut acc = Accumulator380::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.696_f32 + y.sin();
        let b = y * 5.259_f32 - x.cos();
        let mut acc = Accumulator380::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.104_f32 + y.sin();
        let b = y * 2.851_f32 - x.cos();
        let mut acc = Accumulator380::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.467_f32 + y.sin();
        let b = y * 8.001_f32 - x.cos();
        let mut acc = Accumulator380::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.589_f32 + y.sin();
        let b = y * 5.43_f32 - x.cos();
        let mut acc = Accumulator380::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.172_f32 + y.sin();
        let b = y * 5.778_f32 - x.cos();
        let mut acc = Accumulator380::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.15_f32 + y.sin();
        let b = y * 8.38_f32 - x.cos();
        let mut acc = Accumulator380::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.769_f32 + y.sin();
        let b = y * 9.785_f32 - x.cos();
        let mut acc = Accumulator380::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.753_f32 + y.sin();
        let b = y * 4.738_f32 - x.cos();
        let mut acc = Accumulator380::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_380(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_380() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_380(total as u64) % 997) as f32;
        total
    }
}

pub mod m381 {
    use super::*;

    pub struct Accumulator381<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator381<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.533_f32 + y.sin();
        let b = y * 9.118_f32 - x.cos();
        let mut acc = Accumulator381::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.979_f32 + y.sin();
        let b = y * 1.792_f32 - x.cos();
        let mut acc = Accumulator381::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.977_f32 + y.sin();
        let b = y * 4.066_f32 - x.cos();
        let mut acc = Accumulator381::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.537_f32 + y.sin();
        let b = y * 2.61_f32 - x.cos();
        let mut acc = Accumulator381::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.077_f32 + y.sin();
        let b = y * 7.691_f32 - x.cos();
        let mut acc = Accumulator381::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.361_f32 + y.sin();
        let b = y * 9.095_f32 - x.cos();
        let mut acc = Accumulator381::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.571_f32 + y.sin();
        let b = y * 2.089_f32 - x.cos();
        let mut acc = Accumulator381::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.902_f32 + y.sin();
        let b = y * 4.942_f32 - x.cos();
        let mut acc = Accumulator381::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.854_f32 + y.sin();
        let b = y * 7.34_f32 - x.cos();
        let mut acc = Accumulator381::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.281_f32 + y.sin();
        let b = y * 0.737_f32 - x.cos();
        let mut acc = Accumulator381::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.471_f32 + y.sin();
        let b = y * 7.339_f32 - x.cos();
        let mut acc = Accumulator381::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.512_f32 + y.sin();
        let b = y * 6.342_f32 - x.cos();
        let mut acc = Accumulator381::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.584_f32 + y.sin();
        let b = y * 5.525_f32 - x.cos();
        let mut acc = Accumulator381::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.287_f32 + y.sin();
        let b = y * 4.706_f32 - x.cos();
        let mut acc = Accumulator381::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.184_f32 + y.sin();
        let b = y * 9.754_f32 - x.cos();
        let mut acc = Accumulator381::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.124_f32 + y.sin();
        let b = y * 2.42_f32 - x.cos();
        let mut acc = Accumulator381::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.915_f32 + y.sin();
        let b = y * 9.702_f32 - x.cos();
        let mut acc = Accumulator381::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.569_f32 + y.sin();
        let b = y * 0.504_f32 - x.cos();
        let mut acc = Accumulator381::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.305_f32 + y.sin();
        let b = y * 1.798_f32 - x.cos();
        let mut acc = Accumulator381::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.209_f32 + y.sin();
        let b = y * 6.46_f32 - x.cos();
        let mut acc = Accumulator381::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_381(seed: u64) -> u64 {
        let re = Regex::new(r"m381-(\d+)").unwrap();
        let hay = format!("m381-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_381() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_381(total as u64) % 997) as f32;
        total
    }
}

pub mod m382 {
    use super::*;

    pub struct Accumulator382<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator382<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.368_f32 + y.sin();
        let b = y * 2.358_f32 - x.cos();
        let mut acc = Accumulator382::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.443_f32 + y.sin();
        let b = y * 0.405_f32 - x.cos();
        let mut acc = Accumulator382::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.582_f32 + y.sin();
        let b = y * 0.364_f32 - x.cos();
        let mut acc = Accumulator382::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.389_f32 + y.sin();
        let b = y * 2.232_f32 - x.cos();
        let mut acc = Accumulator382::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.59_f32 + y.sin();
        let b = y * 3.482_f32 - x.cos();
        let mut acc = Accumulator382::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.339_f32 + y.sin();
        let b = y * 9.796_f32 - x.cos();
        let mut acc = Accumulator382::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.287_f32 + y.sin();
        let b = y * 5.226_f32 - x.cos();
        let mut acc = Accumulator382::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.715_f32 + y.sin();
        let b = y * 4.488_f32 - x.cos();
        let mut acc = Accumulator382::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.042_f32 + y.sin();
        let b = y * 9.771_f32 - x.cos();
        let mut acc = Accumulator382::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.708_f32 + y.sin();
        let b = y * 8.172_f32 - x.cos();
        let mut acc = Accumulator382::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.351_f32 + y.sin();
        let b = y * 7.392_f32 - x.cos();
        let mut acc = Accumulator382::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.577_f32 + y.sin();
        let b = y * 6.742_f32 - x.cos();
        let mut acc = Accumulator382::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.457_f32 + y.sin();
        let b = y * 9.635_f32 - x.cos();
        let mut acc = Accumulator382::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.502_f32 + y.sin();
        let b = y * 7.278_f32 - x.cos();
        let mut acc = Accumulator382::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.408_f32 + y.sin();
        let b = y * 7.983_f32 - x.cos();
        let mut acc = Accumulator382::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.685_f32 + y.sin();
        let b = y * 8.794_f32 - x.cos();
        let mut acc = Accumulator382::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.995_f32 + y.sin();
        let b = y * 8.915_f32 - x.cos();
        let mut acc = Accumulator382::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.853_f32 + y.sin();
        let b = y * 0.667_f32 - x.cos();
        let mut acc = Accumulator382::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.408_f32 + y.sin();
        let b = y * 8.687_f32 - x.cos();
        let mut acc = Accumulator382::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.61_f32 + y.sin();
        let b = y * 6.375_f32 - x.cos();
        let mut acc = Accumulator382::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_382(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_382() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_382(total as u64) % 997) as f32;
        total
    }
}

pub mod m383 {
    use super::*;

    pub struct Accumulator383<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator383<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.246_f32 + y.sin();
        let b = y * 7.361_f32 - x.cos();
        let mut acc = Accumulator383::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.292_f32 + y.sin();
        let b = y * 5.431_f32 - x.cos();
        let mut acc = Accumulator383::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.727_f32 + y.sin();
        let b = y * 1.429_f32 - x.cos();
        let mut acc = Accumulator383::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.779_f32 + y.sin();
        let b = y * 6.781_f32 - x.cos();
        let mut acc = Accumulator383::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.63_f32 + y.sin();
        let b = y * 2.967_f32 - x.cos();
        let mut acc = Accumulator383::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.027_f32 + y.sin();
        let b = y * 4.048_f32 - x.cos();
        let mut acc = Accumulator383::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.024_f32 + y.sin();
        let b = y * 0.324_f32 - x.cos();
        let mut acc = Accumulator383::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.848_f32 + y.sin();
        let b = y * 8.531_f32 - x.cos();
        let mut acc = Accumulator383::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.361_f32 + y.sin();
        let b = y * 7.105_f32 - x.cos();
        let mut acc = Accumulator383::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.363_f32 + y.sin();
        let b = y * 7.982_f32 - x.cos();
        let mut acc = Accumulator383::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.469_f32 + y.sin();
        let b = y * 4.539_f32 - x.cos();
        let mut acc = Accumulator383::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.817_f32 + y.sin();
        let b = y * 3.191_f32 - x.cos();
        let mut acc = Accumulator383::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.065_f32 + y.sin();
        let b = y * 9.179_f32 - x.cos();
        let mut acc = Accumulator383::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.585_f32 + y.sin();
        let b = y * 1.304_f32 - x.cos();
        let mut acc = Accumulator383::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.387_f32 + y.sin();
        let b = y * 7.983_f32 - x.cos();
        let mut acc = Accumulator383::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.68_f32 + y.sin();
        let b = y * 7.267_f32 - x.cos();
        let mut acc = Accumulator383::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.419_f32 + y.sin();
        let b = y * 9.133_f32 - x.cos();
        let mut acc = Accumulator383::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.265_f32 + y.sin();
        let b = y * 8.95_f32 - x.cos();
        let mut acc = Accumulator383::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.328_f32 + y.sin();
        let b = y * 8.856_f32 - x.cos();
        let mut acc = Accumulator383::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.637_f32 + y.sin();
        let b = y * 7.502_f32 - x.cos();
        let mut acc = Accumulator383::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_383(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(383u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_383() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_383(total as u64) % 997) as f32;
        total
    }
}

pub mod m384 {
    use super::*;

    pub struct Accumulator384<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator384<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.586_f32 + y.sin();
        let b = y * 3.699_f32 - x.cos();
        let mut acc = Accumulator384::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.092_f32 + y.sin();
        let b = y * 1.932_f32 - x.cos();
        let mut acc = Accumulator384::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.724_f32 + y.sin();
        let b = y * 6.703_f32 - x.cos();
        let mut acc = Accumulator384::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.385_f32 + y.sin();
        let b = y * 1.644_f32 - x.cos();
        let mut acc = Accumulator384::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.203_f32 + y.sin();
        let b = y * 8.849_f32 - x.cos();
        let mut acc = Accumulator384::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.301_f32 + y.sin();
        let b = y * 2.922_f32 - x.cos();
        let mut acc = Accumulator384::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.769_f32 + y.sin();
        let b = y * 1.285_f32 - x.cos();
        let mut acc = Accumulator384::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.062_f32 + y.sin();
        let b = y * 1.955_f32 - x.cos();
        let mut acc = Accumulator384::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.374_f32 + y.sin();
        let b = y * 3.539_f32 - x.cos();
        let mut acc = Accumulator384::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.179_f32 + y.sin();
        let b = y * 9.222_f32 - x.cos();
        let mut acc = Accumulator384::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.547_f32 + y.sin();
        let b = y * 3.071_f32 - x.cos();
        let mut acc = Accumulator384::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.032_f32 + y.sin();
        let b = y * 7.023_f32 - x.cos();
        let mut acc = Accumulator384::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.621_f32 + y.sin();
        let b = y * 3.125_f32 - x.cos();
        let mut acc = Accumulator384::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.134_f32 + y.sin();
        let b = y * 6.001_f32 - x.cos();
        let mut acc = Accumulator384::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.336_f32 + y.sin();
        let b = y * 5.125_f32 - x.cos();
        let mut acc = Accumulator384::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.335_f32 + y.sin();
        let b = y * 3.609_f32 - x.cos();
        let mut acc = Accumulator384::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.759_f32 + y.sin();
        let b = y * 5.077_f32 - x.cos();
        let mut acc = Accumulator384::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.043_f32 + y.sin();
        let b = y * 2.177_f32 - x.cos();
        let mut acc = Accumulator384::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.087_f32 + y.sin();
        let b = y * 5.75_f32 - x.cos();
        let mut acc = Accumulator384::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.802_f32 + y.sin();
        let b = y * 9.681_f32 - x.cos();
        let mut acc = Accumulator384::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_384(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_384() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_384(total as u64) % 997) as f32;
        total
    }
}

pub mod m385 {
    use super::*;

    pub struct Accumulator385<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator385<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.18_f32 + y.sin();
        let b = y * 6.403_f32 - x.cos();
        let mut acc = Accumulator385::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.509_f32 + y.sin();
        let b = y * 6.312_f32 - x.cos();
        let mut acc = Accumulator385::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.284_f32 + y.sin();
        let b = y * 1.74_f32 - x.cos();
        let mut acc = Accumulator385::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.279_f32 + y.sin();
        let b = y * 8.297_f32 - x.cos();
        let mut acc = Accumulator385::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.822_f32 + y.sin();
        let b = y * 2.009_f32 - x.cos();
        let mut acc = Accumulator385::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.434_f32 + y.sin();
        let b = y * 7.969_f32 - x.cos();
        let mut acc = Accumulator385::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.328_f32 + y.sin();
        let b = y * 9.706_f32 - x.cos();
        let mut acc = Accumulator385::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.713_f32 + y.sin();
        let b = y * 7.161_f32 - x.cos();
        let mut acc = Accumulator385::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.741_f32 + y.sin();
        let b = y * 5.922_f32 - x.cos();
        let mut acc = Accumulator385::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.561_f32 + y.sin();
        let b = y * 4.671_f32 - x.cos();
        let mut acc = Accumulator385::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.581_f32 + y.sin();
        let b = y * 0.119_f32 - x.cos();
        let mut acc = Accumulator385::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.164_f32 + y.sin();
        let b = y * 9.566_f32 - x.cos();
        let mut acc = Accumulator385::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.967_f32 + y.sin();
        let b = y * 7.301_f32 - x.cos();
        let mut acc = Accumulator385::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.011_f32 + y.sin();
        let b = y * 5.522_f32 - x.cos();
        let mut acc = Accumulator385::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.615_f32 + y.sin();
        let b = y * 5.241_f32 - x.cos();
        let mut acc = Accumulator385::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.197_f32 + y.sin();
        let b = y * 9.082_f32 - x.cos();
        let mut acc = Accumulator385::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.721_f32 + y.sin();
        let b = y * 9.039_f32 - x.cos();
        let mut acc = Accumulator385::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.495_f32 + y.sin();
        let b = y * 3.932_f32 - x.cos();
        let mut acc = Accumulator385::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.714_f32 + y.sin();
        let b = y * 6.485_f32 - x.cos();
        let mut acc = Accumulator385::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.512_f32 + y.sin();
        let b = y * 8.786_f32 - x.cos();
        let mut acc = Accumulator385::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_385(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_385() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_385(total as u64) % 997) as f32;
        total
    }
}

pub mod m386 {
    use super::*;

    pub struct Accumulator386<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator386<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.318_f32 + y.sin();
        let b = y * 2.513_f32 - x.cos();
        let mut acc = Accumulator386::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.182_f32 + y.sin();
        let b = y * 8.741_f32 - x.cos();
        let mut acc = Accumulator386::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.086_f32 + y.sin();
        let b = y * 2.734_f32 - x.cos();
        let mut acc = Accumulator386::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.36_f32 + y.sin();
        let b = y * 8.343_f32 - x.cos();
        let mut acc = Accumulator386::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.923_f32 + y.sin();
        let b = y * 9.75_f32 - x.cos();
        let mut acc = Accumulator386::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.822_f32 + y.sin();
        let b = y * 4.961_f32 - x.cos();
        let mut acc = Accumulator386::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.51_f32 + y.sin();
        let b = y * 5.386_f32 - x.cos();
        let mut acc = Accumulator386::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.735_f32 + y.sin();
        let b = y * 0.976_f32 - x.cos();
        let mut acc = Accumulator386::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.101_f32 + y.sin();
        let b = y * 5.143_f32 - x.cos();
        let mut acc = Accumulator386::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.088_f32 + y.sin();
        let b = y * 9.307_f32 - x.cos();
        let mut acc = Accumulator386::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.113_f32 + y.sin();
        let b = y * 5.761_f32 - x.cos();
        let mut acc = Accumulator386::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.364_f32 + y.sin();
        let b = y * 3.191_f32 - x.cos();
        let mut acc = Accumulator386::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.372_f32 + y.sin();
        let b = y * 7.008_f32 - x.cos();
        let mut acc = Accumulator386::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.293_f32 + y.sin();
        let b = y * 3.44_f32 - x.cos();
        let mut acc = Accumulator386::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.246_f32 + y.sin();
        let b = y * 5.192_f32 - x.cos();
        let mut acc = Accumulator386::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.891_f32 + y.sin();
        let b = y * 6.94_f32 - x.cos();
        let mut acc = Accumulator386::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.578_f32 + y.sin();
        let b = y * 0.518_f32 - x.cos();
        let mut acc = Accumulator386::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.669_f32 + y.sin();
        let b = y * 8.251_f32 - x.cos();
        let mut acc = Accumulator386::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.237_f32 + y.sin();
        let b = y * 7.293_f32 - x.cos();
        let mut acc = Accumulator386::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.258_f32 + y.sin();
        let b = y * 8.352_f32 - x.cos();
        let mut acc = Accumulator386::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_386(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m386-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_386() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_386(total as u64) % 997) as f32;
        total
    }
}

pub mod m387 {
    use super::*;

    pub struct Accumulator387<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator387<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.109_f32 + y.sin();
        let b = y * 5.77_f32 - x.cos();
        let mut acc = Accumulator387::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.685_f32 + y.sin();
        let b = y * 8.45_f32 - x.cos();
        let mut acc = Accumulator387::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.656_f32 + y.sin();
        let b = y * 0.898_f32 - x.cos();
        let mut acc = Accumulator387::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.686_f32 + y.sin();
        let b = y * 5.927_f32 - x.cos();
        let mut acc = Accumulator387::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.721_f32 + y.sin();
        let b = y * 3.594_f32 - x.cos();
        let mut acc = Accumulator387::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.729_f32 + y.sin();
        let b = y * 5.115_f32 - x.cos();
        let mut acc = Accumulator387::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.276_f32 + y.sin();
        let b = y * 7.433_f32 - x.cos();
        let mut acc = Accumulator387::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.521_f32 + y.sin();
        let b = y * 9.465_f32 - x.cos();
        let mut acc = Accumulator387::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.745_f32 + y.sin();
        let b = y * 0.938_f32 - x.cos();
        let mut acc = Accumulator387::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.865_f32 + y.sin();
        let b = y * 6.538_f32 - x.cos();
        let mut acc = Accumulator387::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.041_f32 + y.sin();
        let b = y * 2.483_f32 - x.cos();
        let mut acc = Accumulator387::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.567_f32 + y.sin();
        let b = y * 7.829_f32 - x.cos();
        let mut acc = Accumulator387::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.44_f32 + y.sin();
        let b = y * 1.221_f32 - x.cos();
        let mut acc = Accumulator387::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.084_f32 + y.sin();
        let b = y * 9.489_f32 - x.cos();
        let mut acc = Accumulator387::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.986_f32 + y.sin();
        let b = y * 1.067_f32 - x.cos();
        let mut acc = Accumulator387::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.034_f32 + y.sin();
        let b = y * 7.376_f32 - x.cos();
        let mut acc = Accumulator387::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.717_f32 + y.sin();
        let b = y * 7.38_f32 - x.cos();
        let mut acc = Accumulator387::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.373_f32 + y.sin();
        let b = y * 7.187_f32 - x.cos();
        let mut acc = Accumulator387::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.101_f32 + y.sin();
        let b = y * 8.567_f32 - x.cos();
        let mut acc = Accumulator387::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.6_f32 + y.sin();
        let b = y * 4.314_f32 - x.cos();
        let mut acc = Accumulator387::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_387(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_387() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_387(total as u64) % 997) as f32;
        total
    }
}

pub mod m388 {
    use super::*;

    pub struct Accumulator388<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator388<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.862_f32 + y.sin();
        let b = y * 5.944_f32 - x.cos();
        let mut acc = Accumulator388::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.47_f32 + y.sin();
        let b = y * 0.544_f32 - x.cos();
        let mut acc = Accumulator388::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.775_f32 + y.sin();
        let b = y * 2.194_f32 - x.cos();
        let mut acc = Accumulator388::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.834_f32 + y.sin();
        let b = y * 8.198_f32 - x.cos();
        let mut acc = Accumulator388::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.86_f32 + y.sin();
        let b = y * 0.661_f32 - x.cos();
        let mut acc = Accumulator388::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.004_f32 + y.sin();
        let b = y * 3.866_f32 - x.cos();
        let mut acc = Accumulator388::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.285_f32 + y.sin();
        let b = y * 9.289_f32 - x.cos();
        let mut acc = Accumulator388::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.437_f32 + y.sin();
        let b = y * 4.681_f32 - x.cos();
        let mut acc = Accumulator388::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.69_f32 + y.sin();
        let b = y * 3.058_f32 - x.cos();
        let mut acc = Accumulator388::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.737_f32 + y.sin();
        let b = y * 6.898_f32 - x.cos();
        let mut acc = Accumulator388::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.006_f32 + y.sin();
        let b = y * 5.563_f32 - x.cos();
        let mut acc = Accumulator388::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.067_f32 + y.sin();
        let b = y * 6.591_f32 - x.cos();
        let mut acc = Accumulator388::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.601_f32 + y.sin();
        let b = y * 3.541_f32 - x.cos();
        let mut acc = Accumulator388::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.24_f32 + y.sin();
        let b = y * 6.0_f32 - x.cos();
        let mut acc = Accumulator388::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.678_f32 + y.sin();
        let b = y * 7.621_f32 - x.cos();
        let mut acc = Accumulator388::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.212_f32 + y.sin();
        let b = y * 9.767_f32 - x.cos();
        let mut acc = Accumulator388::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.244_f32 + y.sin();
        let b = y * 2.855_f32 - x.cos();
        let mut acc = Accumulator388::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.842_f32 + y.sin();
        let b = y * 3.566_f32 - x.cos();
        let mut acc = Accumulator388::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.312_f32 + y.sin();
        let b = y * 6.188_f32 - x.cos();
        let mut acc = Accumulator388::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.325_f32 + y.sin();
        let b = y * 4.58_f32 - x.cos();
        let mut acc = Accumulator388::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_388(seed: u64) -> u64 {
        let re = Regex::new(r"m388-(\d+)").unwrap();
        let hay = format!("m388-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_388() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_388(total as u64) % 997) as f32;
        total
    }
}

pub mod m389 {
    use super::*;

    pub struct Accumulator389<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator389<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.716_f32 + y.sin();
        let b = y * 8.669_f32 - x.cos();
        let mut acc = Accumulator389::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.884_f32 + y.sin();
        let b = y * 9.853_f32 - x.cos();
        let mut acc = Accumulator389::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.235_f32 + y.sin();
        let b = y * 9.186_f32 - x.cos();
        let mut acc = Accumulator389::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.578_f32 + y.sin();
        let b = y * 8.953_f32 - x.cos();
        let mut acc = Accumulator389::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.851_f32 + y.sin();
        let b = y * 5.159_f32 - x.cos();
        let mut acc = Accumulator389::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.026_f32 + y.sin();
        let b = y * 4.526_f32 - x.cos();
        let mut acc = Accumulator389::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.31_f32 + y.sin();
        let b = y * 2.808_f32 - x.cos();
        let mut acc = Accumulator389::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.74_f32 + y.sin();
        let b = y * 2.129_f32 - x.cos();
        let mut acc = Accumulator389::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.922_f32 + y.sin();
        let b = y * 6.115_f32 - x.cos();
        let mut acc = Accumulator389::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.861_f32 + y.sin();
        let b = y * 8.966_f32 - x.cos();
        let mut acc = Accumulator389::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.019_f32 + y.sin();
        let b = y * 6.94_f32 - x.cos();
        let mut acc = Accumulator389::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.684_f32 + y.sin();
        let b = y * 0.741_f32 - x.cos();
        let mut acc = Accumulator389::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.029_f32 + y.sin();
        let b = y * 7.62_f32 - x.cos();
        let mut acc = Accumulator389::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.284_f32 + y.sin();
        let b = y * 7.537_f32 - x.cos();
        let mut acc = Accumulator389::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.479_f32 + y.sin();
        let b = y * 5.863_f32 - x.cos();
        let mut acc = Accumulator389::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.0_f32 + y.sin();
        let b = y * 6.094_f32 - x.cos();
        let mut acc = Accumulator389::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.601_f32 + y.sin();
        let b = y * 8.169_f32 - x.cos();
        let mut acc = Accumulator389::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.265_f32 + y.sin();
        let b = y * 8.359_f32 - x.cos();
        let mut acc = Accumulator389::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.766_f32 + y.sin();
        let b = y * 6.066_f32 - x.cos();
        let mut acc = Accumulator389::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.324_f32 + y.sin();
        let b = y * 7.091_f32 - x.cos();
        let mut acc = Accumulator389::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_389(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_389() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_389(total as u64) % 997) as f32;
        total
    }
}

pub mod m390 {
    use super::*;

    pub struct Accumulator390<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator390<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.131_f32 + y.sin();
        let b = y * 0.78_f32 - x.cos();
        let mut acc = Accumulator390::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.131_f32 + y.sin();
        let b = y * 6.456_f32 - x.cos();
        let mut acc = Accumulator390::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.093_f32 + y.sin();
        let b = y * 2.765_f32 - x.cos();
        let mut acc = Accumulator390::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.206_f32 + y.sin();
        let b = y * 6.703_f32 - x.cos();
        let mut acc = Accumulator390::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.883_f32 + y.sin();
        let b = y * 9.83_f32 - x.cos();
        let mut acc = Accumulator390::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.38_f32 + y.sin();
        let b = y * 7.446_f32 - x.cos();
        let mut acc = Accumulator390::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.17_f32 + y.sin();
        let b = y * 5.767_f32 - x.cos();
        let mut acc = Accumulator390::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.104_f32 + y.sin();
        let b = y * 9.684_f32 - x.cos();
        let mut acc = Accumulator390::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.179_f32 + y.sin();
        let b = y * 6.094_f32 - x.cos();
        let mut acc = Accumulator390::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.248_f32 + y.sin();
        let b = y * 9.411_f32 - x.cos();
        let mut acc = Accumulator390::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.107_f32 + y.sin();
        let b = y * 9.408_f32 - x.cos();
        let mut acc = Accumulator390::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.931_f32 + y.sin();
        let b = y * 4.477_f32 - x.cos();
        let mut acc = Accumulator390::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.688_f32 + y.sin();
        let b = y * 2.63_f32 - x.cos();
        let mut acc = Accumulator390::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.185_f32 + y.sin();
        let b = y * 4.581_f32 - x.cos();
        let mut acc = Accumulator390::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.809_f32 + y.sin();
        let b = y * 4.563_f32 - x.cos();
        let mut acc = Accumulator390::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.043_f32 + y.sin();
        let b = y * 9.069_f32 - x.cos();
        let mut acc = Accumulator390::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.322_f32 + y.sin();
        let b = y * 4.399_f32 - x.cos();
        let mut acc = Accumulator390::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.13_f32 + y.sin();
        let b = y * 8.847_f32 - x.cos();
        let mut acc = Accumulator390::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.745_f32 + y.sin();
        let b = y * 9.815_f32 - x.cos();
        let mut acc = Accumulator390::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.136_f32 + y.sin();
        let b = y * 9.472_f32 - x.cos();
        let mut acc = Accumulator390::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_390(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(390u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_390() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_390(total as u64) % 997) as f32;
        total
    }
}

pub mod m391 {
    use super::*;

    pub struct Accumulator391<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator391<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.71_f32 + y.sin();
        let b = y * 2.724_f32 - x.cos();
        let mut acc = Accumulator391::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.812_f32 + y.sin();
        let b = y * 0.184_f32 - x.cos();
        let mut acc = Accumulator391::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.641_f32 + y.sin();
        let b = y * 6.978_f32 - x.cos();
        let mut acc = Accumulator391::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.845_f32 + y.sin();
        let b = y * 2.565_f32 - x.cos();
        let mut acc = Accumulator391::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.349_f32 + y.sin();
        let b = y * 6.498_f32 - x.cos();
        let mut acc = Accumulator391::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.853_f32 + y.sin();
        let b = y * 7.89_f32 - x.cos();
        let mut acc = Accumulator391::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.094_f32 + y.sin();
        let b = y * 6.744_f32 - x.cos();
        let mut acc = Accumulator391::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.266_f32 + y.sin();
        let b = y * 6.534_f32 - x.cos();
        let mut acc = Accumulator391::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.162_f32 + y.sin();
        let b = y * 6.303_f32 - x.cos();
        let mut acc = Accumulator391::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.116_f32 + y.sin();
        let b = y * 8.071_f32 - x.cos();
        let mut acc = Accumulator391::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.99_f32 + y.sin();
        let b = y * 1.828_f32 - x.cos();
        let mut acc = Accumulator391::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.606_f32 + y.sin();
        let b = y * 7.361_f32 - x.cos();
        let mut acc = Accumulator391::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.207_f32 + y.sin();
        let b = y * 8.008_f32 - x.cos();
        let mut acc = Accumulator391::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.584_f32 + y.sin();
        let b = y * 0.887_f32 - x.cos();
        let mut acc = Accumulator391::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.219_f32 + y.sin();
        let b = y * 6.839_f32 - x.cos();
        let mut acc = Accumulator391::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.925_f32 + y.sin();
        let b = y * 8.581_f32 - x.cos();
        let mut acc = Accumulator391::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.831_f32 + y.sin();
        let b = y * 3.049_f32 - x.cos();
        let mut acc = Accumulator391::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.708_f32 + y.sin();
        let b = y * 9.618_f32 - x.cos();
        let mut acc = Accumulator391::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.801_f32 + y.sin();
        let b = y * 3.038_f32 - x.cos();
        let mut acc = Accumulator391::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.784_f32 + y.sin();
        let b = y * 0.703_f32 - x.cos();
        let mut acc = Accumulator391::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_391(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_391() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_391(total as u64) % 997) as f32;
        total
    }
}

pub mod m392 {
    use super::*;

    pub struct Accumulator392<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator392<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.207_f32 + y.sin();
        let b = y * 8.798_f32 - x.cos();
        let mut acc = Accumulator392::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.481_f32 + y.sin();
        let b = y * 8.119_f32 - x.cos();
        let mut acc = Accumulator392::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.136_f32 + y.sin();
        let b = y * 2.09_f32 - x.cos();
        let mut acc = Accumulator392::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.464_f32 + y.sin();
        let b = y * 4.099_f32 - x.cos();
        let mut acc = Accumulator392::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.764_f32 + y.sin();
        let b = y * 9.346_f32 - x.cos();
        let mut acc = Accumulator392::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.767_f32 + y.sin();
        let b = y * 5.408_f32 - x.cos();
        let mut acc = Accumulator392::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.944_f32 + y.sin();
        let b = y * 2.671_f32 - x.cos();
        let mut acc = Accumulator392::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.311_f32 + y.sin();
        let b = y * 3.847_f32 - x.cos();
        let mut acc = Accumulator392::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.516_f32 + y.sin();
        let b = y * 4.748_f32 - x.cos();
        let mut acc = Accumulator392::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.653_f32 + y.sin();
        let b = y * 6.012_f32 - x.cos();
        let mut acc = Accumulator392::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.44_f32 + y.sin();
        let b = y * 2.321_f32 - x.cos();
        let mut acc = Accumulator392::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.761_f32 + y.sin();
        let b = y * 8.537_f32 - x.cos();
        let mut acc = Accumulator392::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.748_f32 + y.sin();
        let b = y * 4.975_f32 - x.cos();
        let mut acc = Accumulator392::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.473_f32 + y.sin();
        let b = y * 4.48_f32 - x.cos();
        let mut acc = Accumulator392::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.397_f32 + y.sin();
        let b = y * 6.104_f32 - x.cos();
        let mut acc = Accumulator392::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.582_f32 + y.sin();
        let b = y * 2.256_f32 - x.cos();
        let mut acc = Accumulator392::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.132_f32 + y.sin();
        let b = y * 9.407_f32 - x.cos();
        let mut acc = Accumulator392::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.382_f32 + y.sin();
        let b = y * 7.008_f32 - x.cos();
        let mut acc = Accumulator392::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.736_f32 + y.sin();
        let b = y * 1.155_f32 - x.cos();
        let mut acc = Accumulator392::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.739_f32 + y.sin();
        let b = y * 0.454_f32 - x.cos();
        let mut acc = Accumulator392::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_392(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_392() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_392(total as u64) % 997) as f32;
        total
    }
}

pub mod m393 {
    use super::*;

    pub struct Accumulator393<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator393<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.528_f32 + y.sin();
        let b = y * 0.897_f32 - x.cos();
        let mut acc = Accumulator393::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.256_f32 + y.sin();
        let b = y * 1.436_f32 - x.cos();
        let mut acc = Accumulator393::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.67_f32 + y.sin();
        let b = y * 5.319_f32 - x.cos();
        let mut acc = Accumulator393::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.87_f32 + y.sin();
        let b = y * 5.523_f32 - x.cos();
        let mut acc = Accumulator393::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.936_f32 + y.sin();
        let b = y * 7.719_f32 - x.cos();
        let mut acc = Accumulator393::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.582_f32 + y.sin();
        let b = y * 6.96_f32 - x.cos();
        let mut acc = Accumulator393::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.821_f32 + y.sin();
        let b = y * 4.301_f32 - x.cos();
        let mut acc = Accumulator393::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.332_f32 + y.sin();
        let b = y * 4.061_f32 - x.cos();
        let mut acc = Accumulator393::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.751_f32 + y.sin();
        let b = y * 4.451_f32 - x.cos();
        let mut acc = Accumulator393::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.305_f32 + y.sin();
        let b = y * 6.065_f32 - x.cos();
        let mut acc = Accumulator393::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.562_f32 + y.sin();
        let b = y * 8.115_f32 - x.cos();
        let mut acc = Accumulator393::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.769_f32 + y.sin();
        let b = y * 5.445_f32 - x.cos();
        let mut acc = Accumulator393::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.702_f32 + y.sin();
        let b = y * 2.66_f32 - x.cos();
        let mut acc = Accumulator393::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.85_f32 + y.sin();
        let b = y * 7.267_f32 - x.cos();
        let mut acc = Accumulator393::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.055_f32 + y.sin();
        let b = y * 0.565_f32 - x.cos();
        let mut acc = Accumulator393::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.743_f32 + y.sin();
        let b = y * 1.576_f32 - x.cos();
        let mut acc = Accumulator393::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.262_f32 + y.sin();
        let b = y * 5.345_f32 - x.cos();
        let mut acc = Accumulator393::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.163_f32 + y.sin();
        let b = y * 0.958_f32 - x.cos();
        let mut acc = Accumulator393::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.243_f32 + y.sin();
        let b = y * 0.361_f32 - x.cos();
        let mut acc = Accumulator393::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.266_f32 + y.sin();
        let b = y * 2.56_f32 - x.cos();
        let mut acc = Accumulator393::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_393(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m393-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_393() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_393(total as u64) % 997) as f32;
        total
    }
}

pub mod m394 {
    use super::*;

    pub struct Accumulator394<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator394<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.358_f32 + y.sin();
        let b = y * 2.22_f32 - x.cos();
        let mut acc = Accumulator394::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.266_f32 + y.sin();
        let b = y * 1.577_f32 - x.cos();
        let mut acc = Accumulator394::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.762_f32 + y.sin();
        let b = y * 7.182_f32 - x.cos();
        let mut acc = Accumulator394::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.14_f32 + y.sin();
        let b = y * 1.28_f32 - x.cos();
        let mut acc = Accumulator394::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.915_f32 + y.sin();
        let b = y * 3.154_f32 - x.cos();
        let mut acc = Accumulator394::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.221_f32 + y.sin();
        let b = y * 7.231_f32 - x.cos();
        let mut acc = Accumulator394::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.529_f32 + y.sin();
        let b = y * 6.554_f32 - x.cos();
        let mut acc = Accumulator394::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.892_f32 + y.sin();
        let b = y * 2.282_f32 - x.cos();
        let mut acc = Accumulator394::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.884_f32 + y.sin();
        let b = y * 6.1_f32 - x.cos();
        let mut acc = Accumulator394::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.493_f32 + y.sin();
        let b = y * 8.048_f32 - x.cos();
        let mut acc = Accumulator394::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.527_f32 + y.sin();
        let b = y * 2.897_f32 - x.cos();
        let mut acc = Accumulator394::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.311_f32 + y.sin();
        let b = y * 0.156_f32 - x.cos();
        let mut acc = Accumulator394::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.46_f32 + y.sin();
        let b = y * 1.373_f32 - x.cos();
        let mut acc = Accumulator394::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.687_f32 + y.sin();
        let b = y * 1.457_f32 - x.cos();
        let mut acc = Accumulator394::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.2_f32 + y.sin();
        let b = y * 6.032_f32 - x.cos();
        let mut acc = Accumulator394::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.889_f32 + y.sin();
        let b = y * 5.167_f32 - x.cos();
        let mut acc = Accumulator394::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.333_f32 + y.sin();
        let b = y * 3.315_f32 - x.cos();
        let mut acc = Accumulator394::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.339_f32 + y.sin();
        let b = y * 9.501_f32 - x.cos();
        let mut acc = Accumulator394::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.265_f32 + y.sin();
        let b = y * 5.581_f32 - x.cos();
        let mut acc = Accumulator394::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.71_f32 + y.sin();
        let b = y * 6.454_f32 - x.cos();
        let mut acc = Accumulator394::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_394(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_394() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_394(total as u64) % 997) as f32;
        total
    }
}

pub mod m395 {
    use super::*;

    pub struct Accumulator395<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator395<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.931_f32 + y.sin();
        let b = y * 1.5_f32 - x.cos();
        let mut acc = Accumulator395::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.81_f32 + y.sin();
        let b = y * 2.574_f32 - x.cos();
        let mut acc = Accumulator395::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.927_f32 + y.sin();
        let b = y * 4.818_f32 - x.cos();
        let mut acc = Accumulator395::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.861_f32 + y.sin();
        let b = y * 9.449_f32 - x.cos();
        let mut acc = Accumulator395::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.889_f32 + y.sin();
        let b = y * 8.242_f32 - x.cos();
        let mut acc = Accumulator395::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.504_f32 + y.sin();
        let b = y * 3.476_f32 - x.cos();
        let mut acc = Accumulator395::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.243_f32 + y.sin();
        let b = y * 5.534_f32 - x.cos();
        let mut acc = Accumulator395::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.606_f32 + y.sin();
        let b = y * 8.511_f32 - x.cos();
        let mut acc = Accumulator395::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.142_f32 + y.sin();
        let b = y * 1.598_f32 - x.cos();
        let mut acc = Accumulator395::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.13_f32 + y.sin();
        let b = y * 5.106_f32 - x.cos();
        let mut acc = Accumulator395::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.519_f32 + y.sin();
        let b = y * 2.211_f32 - x.cos();
        let mut acc = Accumulator395::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.754_f32 + y.sin();
        let b = y * 1.287_f32 - x.cos();
        let mut acc = Accumulator395::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.752_f32 + y.sin();
        let b = y * 4.705_f32 - x.cos();
        let mut acc = Accumulator395::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.778_f32 + y.sin();
        let b = y * 2.376_f32 - x.cos();
        let mut acc = Accumulator395::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.505_f32 + y.sin();
        let b = y * 0.999_f32 - x.cos();
        let mut acc = Accumulator395::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.346_f32 + y.sin();
        let b = y * 8.811_f32 - x.cos();
        let mut acc = Accumulator395::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.188_f32 + y.sin();
        let b = y * 0.248_f32 - x.cos();
        let mut acc = Accumulator395::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.442_f32 + y.sin();
        let b = y * 1.562_f32 - x.cos();
        let mut acc = Accumulator395::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.097_f32 + y.sin();
        let b = y * 8.606_f32 - x.cos();
        let mut acc = Accumulator395::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.277_f32 + y.sin();
        let b = y * 9.659_f32 - x.cos();
        let mut acc = Accumulator395::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_395(seed: u64) -> u64 {
        let re = Regex::new(r"m395-(\d+)").unwrap();
        let hay = format!("m395-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_395() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_395(total as u64) % 997) as f32;
        total
    }
}

pub mod m396 {
    use super::*;

    pub struct Accumulator396<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator396<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.598_f32 + y.sin();
        let b = y * 0.295_f32 - x.cos();
        let mut acc = Accumulator396::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.111_f32 + y.sin();
        let b = y * 3.465_f32 - x.cos();
        let mut acc = Accumulator396::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.952_f32 + y.sin();
        let b = y * 0.953_f32 - x.cos();
        let mut acc = Accumulator396::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.374_f32 + y.sin();
        let b = y * 7.337_f32 - x.cos();
        let mut acc = Accumulator396::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.36_f32 + y.sin();
        let b = y * 8.992_f32 - x.cos();
        let mut acc = Accumulator396::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.483_f32 + y.sin();
        let b = y * 4.019_f32 - x.cos();
        let mut acc = Accumulator396::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.431_f32 + y.sin();
        let b = y * 0.301_f32 - x.cos();
        let mut acc = Accumulator396::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.51_f32 + y.sin();
        let b = y * 7.984_f32 - x.cos();
        let mut acc = Accumulator396::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.429_f32 + y.sin();
        let b = y * 4.695_f32 - x.cos();
        let mut acc = Accumulator396::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.078_f32 + y.sin();
        let b = y * 7.207_f32 - x.cos();
        let mut acc = Accumulator396::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.402_f32 + y.sin();
        let b = y * 8.288_f32 - x.cos();
        let mut acc = Accumulator396::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.211_f32 + y.sin();
        let b = y * 1.301_f32 - x.cos();
        let mut acc = Accumulator396::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.387_f32 + y.sin();
        let b = y * 8.439_f32 - x.cos();
        let mut acc = Accumulator396::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.916_f32 + y.sin();
        let b = y * 3.299_f32 - x.cos();
        let mut acc = Accumulator396::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.092_f32 + y.sin();
        let b = y * 0.519_f32 - x.cos();
        let mut acc = Accumulator396::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.877_f32 + y.sin();
        let b = y * 2.805_f32 - x.cos();
        let mut acc = Accumulator396::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.782_f32 + y.sin();
        let b = y * 3.874_f32 - x.cos();
        let mut acc = Accumulator396::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.778_f32 + y.sin();
        let b = y * 5.15_f32 - x.cos();
        let mut acc = Accumulator396::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.564_f32 + y.sin();
        let b = y * 4.567_f32 - x.cos();
        let mut acc = Accumulator396::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.781_f32 + y.sin();
        let b = y * 2.427_f32 - x.cos();
        let mut acc = Accumulator396::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_396(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_396() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_396(total as u64) % 997) as f32;
        total
    }
}

pub mod m397 {
    use super::*;

    pub struct Accumulator397<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator397<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.383_f32 + y.sin();
        let b = y * 2.984_f32 - x.cos();
        let mut acc = Accumulator397::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.648_f32 + y.sin();
        let b = y * 5.82_f32 - x.cos();
        let mut acc = Accumulator397::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.463_f32 + y.sin();
        let b = y * 2.209_f32 - x.cos();
        let mut acc = Accumulator397::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.734_f32 + y.sin();
        let b = y * 5.594_f32 - x.cos();
        let mut acc = Accumulator397::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.324_f32 + y.sin();
        let b = y * 9.754_f32 - x.cos();
        let mut acc = Accumulator397::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.659_f32 + y.sin();
        let b = y * 1.581_f32 - x.cos();
        let mut acc = Accumulator397::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.336_f32 + y.sin();
        let b = y * 0.132_f32 - x.cos();
        let mut acc = Accumulator397::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.383_f32 + y.sin();
        let b = y * 8.43_f32 - x.cos();
        let mut acc = Accumulator397::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.055_f32 + y.sin();
        let b = y * 8.978_f32 - x.cos();
        let mut acc = Accumulator397::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.646_f32 + y.sin();
        let b = y * 2.61_f32 - x.cos();
        let mut acc = Accumulator397::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.803_f32 + y.sin();
        let b = y * 9.313_f32 - x.cos();
        let mut acc = Accumulator397::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.056_f32 + y.sin();
        let b = y * 2.444_f32 - x.cos();
        let mut acc = Accumulator397::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.707_f32 + y.sin();
        let b = y * 7.71_f32 - x.cos();
        let mut acc = Accumulator397::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.28_f32 + y.sin();
        let b = y * 4.687_f32 - x.cos();
        let mut acc = Accumulator397::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.939_f32 + y.sin();
        let b = y * 0.89_f32 - x.cos();
        let mut acc = Accumulator397::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.744_f32 + y.sin();
        let b = y * 1.675_f32 - x.cos();
        let mut acc = Accumulator397::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.341_f32 + y.sin();
        let b = y * 4.126_f32 - x.cos();
        let mut acc = Accumulator397::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.341_f32 + y.sin();
        let b = y * 7.252_f32 - x.cos();
        let mut acc = Accumulator397::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.334_f32 + y.sin();
        let b = y * 5.476_f32 - x.cos();
        let mut acc = Accumulator397::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.304_f32 + y.sin();
        let b = y * 3.662_f32 - x.cos();
        let mut acc = Accumulator397::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_397(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(397u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_397() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_397(total as u64) % 997) as f32;
        total
    }
}

pub mod m398 {
    use super::*;

    pub struct Accumulator398<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator398<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.884_f32 + y.sin();
        let b = y * 7.93_f32 - x.cos();
        let mut acc = Accumulator398::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.153_f32 + y.sin();
        let b = y * 8.658_f32 - x.cos();
        let mut acc = Accumulator398::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.954_f32 + y.sin();
        let b = y * 9.744_f32 - x.cos();
        let mut acc = Accumulator398::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.532_f32 + y.sin();
        let b = y * 0.735_f32 - x.cos();
        let mut acc = Accumulator398::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.382_f32 + y.sin();
        let b = y * 5.3_f32 - x.cos();
        let mut acc = Accumulator398::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.839_f32 + y.sin();
        let b = y * 9.862_f32 - x.cos();
        let mut acc = Accumulator398::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.979_f32 + y.sin();
        let b = y * 8.649_f32 - x.cos();
        let mut acc = Accumulator398::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.173_f32 + y.sin();
        let b = y * 8.31_f32 - x.cos();
        let mut acc = Accumulator398::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.686_f32 + y.sin();
        let b = y * 6.922_f32 - x.cos();
        let mut acc = Accumulator398::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.187_f32 + y.sin();
        let b = y * 2.851_f32 - x.cos();
        let mut acc = Accumulator398::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.865_f32 + y.sin();
        let b = y * 0.126_f32 - x.cos();
        let mut acc = Accumulator398::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.464_f32 + y.sin();
        let b = y * 3.884_f32 - x.cos();
        let mut acc = Accumulator398::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.476_f32 + y.sin();
        let b = y * 2.04_f32 - x.cos();
        let mut acc = Accumulator398::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.765_f32 + y.sin();
        let b = y * 9.276_f32 - x.cos();
        let mut acc = Accumulator398::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.765_f32 + y.sin();
        let b = y * 4.278_f32 - x.cos();
        let mut acc = Accumulator398::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.741_f32 + y.sin();
        let b = y * 8.434_f32 - x.cos();
        let mut acc = Accumulator398::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.692_f32 + y.sin();
        let b = y * 3.949_f32 - x.cos();
        let mut acc = Accumulator398::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.906_f32 + y.sin();
        let b = y * 9.512_f32 - x.cos();
        let mut acc = Accumulator398::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.322_f32 + y.sin();
        let b = y * 9.075_f32 - x.cos();
        let mut acc = Accumulator398::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.428_f32 + y.sin();
        let b = y * 7.724_f32 - x.cos();
        let mut acc = Accumulator398::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_398(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_398() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_398(total as u64) % 997) as f32;
        total
    }
}

pub mod m399 {
    use super::*;

    pub struct Accumulator399<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator399<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.801_f32 + y.sin();
        let b = y * 9.26_f32 - x.cos();
        let mut acc = Accumulator399::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.859_f32 + y.sin();
        let b = y * 0.544_f32 - x.cos();
        let mut acc = Accumulator399::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.898_f32 + y.sin();
        let b = y * 4.139_f32 - x.cos();
        let mut acc = Accumulator399::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.051_f32 + y.sin();
        let b = y * 7.282_f32 - x.cos();
        let mut acc = Accumulator399::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.788_f32 + y.sin();
        let b = y * 4.153_f32 - x.cos();
        let mut acc = Accumulator399::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.688_f32 + y.sin();
        let b = y * 5.778_f32 - x.cos();
        let mut acc = Accumulator399::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.787_f32 + y.sin();
        let b = y * 8.765_f32 - x.cos();
        let mut acc = Accumulator399::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.906_f32 + y.sin();
        let b = y * 9.287_f32 - x.cos();
        let mut acc = Accumulator399::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.507_f32 + y.sin();
        let b = y * 0.879_f32 - x.cos();
        let mut acc = Accumulator399::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.268_f32 + y.sin();
        let b = y * 6.351_f32 - x.cos();
        let mut acc = Accumulator399::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.277_f32 + y.sin();
        let b = y * 4.713_f32 - x.cos();
        let mut acc = Accumulator399::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.863_f32 + y.sin();
        let b = y * 0.704_f32 - x.cos();
        let mut acc = Accumulator399::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.274_f32 + y.sin();
        let b = y * 7.931_f32 - x.cos();
        let mut acc = Accumulator399::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.512_f32 + y.sin();
        let b = y * 0.263_f32 - x.cos();
        let mut acc = Accumulator399::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.219_f32 + y.sin();
        let b = y * 4.287_f32 - x.cos();
        let mut acc = Accumulator399::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.755_f32 + y.sin();
        let b = y * 4.139_f32 - x.cos();
        let mut acc = Accumulator399::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.013_f32 + y.sin();
        let b = y * 7.529_f32 - x.cos();
        let mut acc = Accumulator399::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.498_f32 + y.sin();
        let b = y * 4.816_f32 - x.cos();
        let mut acc = Accumulator399::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.735_f32 + y.sin();
        let b = y * 4.097_f32 - x.cos();
        let mut acc = Accumulator399::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.998_f32 + y.sin();
        let b = y * 5.476_f32 - x.cos();
        let mut acc = Accumulator399::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_399(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_399() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_399(total as u64) % 997) as f32;
        total
    }
}

pub fn touch_bulk_3() -> f32 {
    let mut total = 0.0_f32;
    total += m300::run_all_300();
    total += m301::run_all_301();
    total += m302::run_all_302();
    total += m303::run_all_303();
    total += m304::run_all_304();
    total += m305::run_all_305();
    total += m306::run_all_306();
    total += m307::run_all_307();
    total += m308::run_all_308();
    total += m309::run_all_309();
    total += m310::run_all_310();
    total += m311::run_all_311();
    total += m312::run_all_312();
    total += m313::run_all_313();
    total += m314::run_all_314();
    total += m315::run_all_315();
    total += m316::run_all_316();
    total += m317::run_all_317();
    total += m318::run_all_318();
    total += m319::run_all_319();
    total += m320::run_all_320();
    total += m321::run_all_321();
    total += m322::run_all_322();
    total += m323::run_all_323();
    total += m324::run_all_324();
    total += m325::run_all_325();
    total += m326::run_all_326();
    total += m327::run_all_327();
    total += m328::run_all_328();
    total += m329::run_all_329();
    total += m330::run_all_330();
    total += m331::run_all_331();
    total += m332::run_all_332();
    total += m333::run_all_333();
    total += m334::run_all_334();
    total += m335::run_all_335();
    total += m336::run_all_336();
    total += m337::run_all_337();
    total += m338::run_all_338();
    total += m339::run_all_339();
    total += m340::run_all_340();
    total += m341::run_all_341();
    total += m342::run_all_342();
    total += m343::run_all_343();
    total += m344::run_all_344();
    total += m345::run_all_345();
    total += m346::run_all_346();
    total += m347::run_all_347();
    total += m348::run_all_348();
    total += m349::run_all_349();
    total += m350::run_all_350();
    total += m351::run_all_351();
    total += m352::run_all_352();
    total += m353::run_all_353();
    total += m354::run_all_354();
    total += m355::run_all_355();
    total += m356::run_all_356();
    total += m357::run_all_357();
    total += m358::run_all_358();
    total += m359::run_all_359();
    total += m360::run_all_360();
    total += m361::run_all_361();
    total += m362::run_all_362();
    total += m363::run_all_363();
    total += m364::run_all_364();
    total += m365::run_all_365();
    total += m366::run_all_366();
    total += m367::run_all_367();
    total += m368::run_all_368();
    total += m369::run_all_369();
    total += m370::run_all_370();
    total += m371::run_all_371();
    total += m372::run_all_372();
    total += m373::run_all_373();
    total += m374::run_all_374();
    total += m375::run_all_375();
    total += m376::run_all_376();
    total += m377::run_all_377();
    total += m378::run_all_378();
    total += m379::run_all_379();
    total += m380::run_all_380();
    total += m381::run_all_381();
    total += m382::run_all_382();
    total += m383::run_all_383();
    total += m384::run_all_384();
    total += m385::run_all_385();
    total += m386::run_all_386();
    total += m387::run_all_387();
    total += m388::run_all_388();
    total += m389::run_all_389();
    total += m390::run_all_390();
    total += m391::run_all_391();
    total += m392::run_all_392();
    total += m393::run_all_393();
    total += m394::run_all_394();
    total += m395::run_all_395();
    total += m396::run_all_396();
    total += m397::run_all_397();
    total += m398::run_all_398();
    total += m399::run_all_399();
    total
}
