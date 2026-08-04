//! Auto-generated bulk module (file 10) - exists purely to make `game_rs`
//! heavier for a hot-patch speed experiment. Real arithmetic, real generics
//! (so monomorphization isn't free), and real use of a rotating set of
//! external crates so none of them are dead code. `touch_bulk_10()` is
//! called once from `game::init` so nothing here gets stripped as unreachable.

use rand::{Rng, SeedableRng};
use sha2::{Digest, Sha256};
use noise::{NoiseFn, Perlin};
use regex::Regex;
use itertools::Itertools;
use uuid::Uuid;
use chrono::Utc;


pub mod m1000 {
    use super::*;

    pub struct Accumulator1000<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1000<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.567_f32 + y.sin();
        let b = y * 7.162_f32 - x.cos();
        let mut acc = Accumulator1000::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.336_f32 + y.sin();
        let b = y * 7.841_f32 - x.cos();
        let mut acc = Accumulator1000::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.619_f32 + y.sin();
        let b = y * 4.234_f32 - x.cos();
        let mut acc = Accumulator1000::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.772_f32 + y.sin();
        let b = y * 6.488_f32 - x.cos();
        let mut acc = Accumulator1000::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.456_f32 + y.sin();
        let b = y * 4.911_f32 - x.cos();
        let mut acc = Accumulator1000::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.691_f32 + y.sin();
        let b = y * 2.803_f32 - x.cos();
        let mut acc = Accumulator1000::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.836_f32 + y.sin();
        let b = y * 4.868_f32 - x.cos();
        let mut acc = Accumulator1000::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.845_f32 + y.sin();
        let b = y * 9.026_f32 - x.cos();
        let mut acc = Accumulator1000::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.256_f32 + y.sin();
        let b = y * 9.18_f32 - x.cos();
        let mut acc = Accumulator1000::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.234_f32 + y.sin();
        let b = y * 0.175_f32 - x.cos();
        let mut acc = Accumulator1000::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.504_f32 + y.sin();
        let b = y * 1.964_f32 - x.cos();
        let mut acc = Accumulator1000::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.142_f32 + y.sin();
        let b = y * 9.086_f32 - x.cos();
        let mut acc = Accumulator1000::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.499_f32 + y.sin();
        let b = y * 3.157_f32 - x.cos();
        let mut acc = Accumulator1000::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.501_f32 + y.sin();
        let b = y * 1.505_f32 - x.cos();
        let mut acc = Accumulator1000::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.199_f32 + y.sin();
        let b = y * 8.866_f32 - x.cos();
        let mut acc = Accumulator1000::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.998_f32 + y.sin();
        let b = y * 2.091_f32 - x.cos();
        let mut acc = Accumulator1000::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.726_f32 + y.sin();
        let b = y * 3.226_f32 - x.cos();
        let mut acc = Accumulator1000::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.61_f32 + y.sin();
        let b = y * 4.615_f32 - x.cos();
        let mut acc = Accumulator1000::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.589_f32 + y.sin();
        let b = y * 2.634_f32 - x.cos();
        let mut acc = Accumulator1000::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.071_f32 + y.sin();
        let b = y * 7.978_f32 - x.cos();
        let mut acc = Accumulator1000::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1000(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_1000() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1000(total as u64) % 997) as f32;
        total
    }
}

pub mod m1001 {
    use super::*;

    pub struct Accumulator1001<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1001<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.61_f32 + y.sin();
        let b = y * 1.572_f32 - x.cos();
        let mut acc = Accumulator1001::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.012_f32 + y.sin();
        let b = y * 0.458_f32 - x.cos();
        let mut acc = Accumulator1001::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.952_f32 + y.sin();
        let b = y * 7.934_f32 - x.cos();
        let mut acc = Accumulator1001::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.772_f32 + y.sin();
        let b = y * 5.305_f32 - x.cos();
        let mut acc = Accumulator1001::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.651_f32 + y.sin();
        let b = y * 3.374_f32 - x.cos();
        let mut acc = Accumulator1001::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.237_f32 + y.sin();
        let b = y * 1.65_f32 - x.cos();
        let mut acc = Accumulator1001::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.713_f32 + y.sin();
        let b = y * 8.928_f32 - x.cos();
        let mut acc = Accumulator1001::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.889_f32 + y.sin();
        let b = y * 9.351_f32 - x.cos();
        let mut acc = Accumulator1001::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.6_f32 + y.sin();
        let b = y * 2.304_f32 - x.cos();
        let mut acc = Accumulator1001::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.055_f32 + y.sin();
        let b = y * 6.166_f32 - x.cos();
        let mut acc = Accumulator1001::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.195_f32 + y.sin();
        let b = y * 1.872_f32 - x.cos();
        let mut acc = Accumulator1001::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.057_f32 + y.sin();
        let b = y * 2.465_f32 - x.cos();
        let mut acc = Accumulator1001::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.272_f32 + y.sin();
        let b = y * 0.782_f32 - x.cos();
        let mut acc = Accumulator1001::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.645_f32 + y.sin();
        let b = y * 2.732_f32 - x.cos();
        let mut acc = Accumulator1001::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.523_f32 + y.sin();
        let b = y * 0.129_f32 - x.cos();
        let mut acc = Accumulator1001::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.394_f32 + y.sin();
        let b = y * 5.828_f32 - x.cos();
        let mut acc = Accumulator1001::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.524_f32 + y.sin();
        let b = y * 8.0_f32 - x.cos();
        let mut acc = Accumulator1001::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.824_f32 + y.sin();
        let b = y * 0.239_f32 - x.cos();
        let mut acc = Accumulator1001::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.728_f32 + y.sin();
        let b = y * 1.802_f32 - x.cos();
        let mut acc = Accumulator1001::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.935_f32 + y.sin();
        let b = y * 5.277_f32 - x.cos();
        let mut acc = Accumulator1001::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1001(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_1001() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1001(total as u64) % 997) as f32;
        total
    }
}

pub mod m1002 {
    use super::*;

    pub struct Accumulator1002<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1002<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.477_f32 + y.sin();
        let b = y * 9.695_f32 - x.cos();
        let mut acc = Accumulator1002::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.999_f32 + y.sin();
        let b = y * 4.258_f32 - x.cos();
        let mut acc = Accumulator1002::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.077_f32 + y.sin();
        let b = y * 2.307_f32 - x.cos();
        let mut acc = Accumulator1002::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.513_f32 + y.sin();
        let b = y * 1.938_f32 - x.cos();
        let mut acc = Accumulator1002::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.694_f32 + y.sin();
        let b = y * 1.357_f32 - x.cos();
        let mut acc = Accumulator1002::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.373_f32 + y.sin();
        let b = y * 6.827_f32 - x.cos();
        let mut acc = Accumulator1002::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.041_f32 + y.sin();
        let b = y * 5.515_f32 - x.cos();
        let mut acc = Accumulator1002::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.588_f32 + y.sin();
        let b = y * 4.302_f32 - x.cos();
        let mut acc = Accumulator1002::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.789_f32 + y.sin();
        let b = y * 6.699_f32 - x.cos();
        let mut acc = Accumulator1002::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.851_f32 + y.sin();
        let b = y * 5.403_f32 - x.cos();
        let mut acc = Accumulator1002::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.436_f32 + y.sin();
        let b = y * 9.645_f32 - x.cos();
        let mut acc = Accumulator1002::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.633_f32 + y.sin();
        let b = y * 5.182_f32 - x.cos();
        let mut acc = Accumulator1002::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.501_f32 + y.sin();
        let b = y * 0.317_f32 - x.cos();
        let mut acc = Accumulator1002::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.931_f32 + y.sin();
        let b = y * 1.236_f32 - x.cos();
        let mut acc = Accumulator1002::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.813_f32 + y.sin();
        let b = y * 1.304_f32 - x.cos();
        let mut acc = Accumulator1002::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.213_f32 + y.sin();
        let b = y * 1.487_f32 - x.cos();
        let mut acc = Accumulator1002::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.931_f32 + y.sin();
        let b = y * 0.56_f32 - x.cos();
        let mut acc = Accumulator1002::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.973_f32 + y.sin();
        let b = y * 3.376_f32 - x.cos();
        let mut acc = Accumulator1002::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.562_f32 + y.sin();
        let b = y * 0.266_f32 - x.cos();
        let mut acc = Accumulator1002::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.832_f32 + y.sin();
        let b = y * 8.997_f32 - x.cos();
        let mut acc = Accumulator1002::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1002(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m1002-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_1002() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1002(total as u64) % 997) as f32;
        total
    }
}

pub mod m1003 {
    use super::*;

    pub struct Accumulator1003<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1003<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.243_f32 + y.sin();
        let b = y * 5.377_f32 - x.cos();
        let mut acc = Accumulator1003::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.398_f32 + y.sin();
        let b = y * 7.649_f32 - x.cos();
        let mut acc = Accumulator1003::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.87_f32 + y.sin();
        let b = y * 1.09_f32 - x.cos();
        let mut acc = Accumulator1003::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.83_f32 + y.sin();
        let b = y * 3.766_f32 - x.cos();
        let mut acc = Accumulator1003::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.196_f32 + y.sin();
        let b = y * 1.083_f32 - x.cos();
        let mut acc = Accumulator1003::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.764_f32 + y.sin();
        let b = y * 2.435_f32 - x.cos();
        let mut acc = Accumulator1003::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.279_f32 + y.sin();
        let b = y * 5.743_f32 - x.cos();
        let mut acc = Accumulator1003::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.93_f32 + y.sin();
        let b = y * 6.014_f32 - x.cos();
        let mut acc = Accumulator1003::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.22_f32 + y.sin();
        let b = y * 4.964_f32 - x.cos();
        let mut acc = Accumulator1003::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.439_f32 + y.sin();
        let b = y * 7.663_f32 - x.cos();
        let mut acc = Accumulator1003::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.258_f32 + y.sin();
        let b = y * 5.648_f32 - x.cos();
        let mut acc = Accumulator1003::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.447_f32 + y.sin();
        let b = y * 7.824_f32 - x.cos();
        let mut acc = Accumulator1003::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.309_f32 + y.sin();
        let b = y * 7.752_f32 - x.cos();
        let mut acc = Accumulator1003::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.166_f32 + y.sin();
        let b = y * 5.268_f32 - x.cos();
        let mut acc = Accumulator1003::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.775_f32 + y.sin();
        let b = y * 1.339_f32 - x.cos();
        let mut acc = Accumulator1003::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.122_f32 + y.sin();
        let b = y * 1.469_f32 - x.cos();
        let mut acc = Accumulator1003::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.98_f32 + y.sin();
        let b = y * 8.341_f32 - x.cos();
        let mut acc = Accumulator1003::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.862_f32 + y.sin();
        let b = y * 8.546_f32 - x.cos();
        let mut acc = Accumulator1003::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.85_f32 + y.sin();
        let b = y * 7.328_f32 - x.cos();
        let mut acc = Accumulator1003::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.083_f32 + y.sin();
        let b = y * 5.81_f32 - x.cos();
        let mut acc = Accumulator1003::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1003(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_1003() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1003(total as u64) % 997) as f32;
        total
    }
}

pub mod m1004 {
    use super::*;

    pub struct Accumulator1004<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1004<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.714_f32 + y.sin();
        let b = y * 2.538_f32 - x.cos();
        let mut acc = Accumulator1004::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.257_f32 + y.sin();
        let b = y * 1.128_f32 - x.cos();
        let mut acc = Accumulator1004::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.946_f32 + y.sin();
        let b = y * 0.224_f32 - x.cos();
        let mut acc = Accumulator1004::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.53_f32 + y.sin();
        let b = y * 6.827_f32 - x.cos();
        let mut acc = Accumulator1004::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.13_f32 + y.sin();
        let b = y * 1.79_f32 - x.cos();
        let mut acc = Accumulator1004::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.758_f32 + y.sin();
        let b = y * 9.3_f32 - x.cos();
        let mut acc = Accumulator1004::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.665_f32 + y.sin();
        let b = y * 6.62_f32 - x.cos();
        let mut acc = Accumulator1004::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.182_f32 + y.sin();
        let b = y * 7.628_f32 - x.cos();
        let mut acc = Accumulator1004::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.415_f32 + y.sin();
        let b = y * 2.388_f32 - x.cos();
        let mut acc = Accumulator1004::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.767_f32 + y.sin();
        let b = y * 2.56_f32 - x.cos();
        let mut acc = Accumulator1004::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.483_f32 + y.sin();
        let b = y * 5.098_f32 - x.cos();
        let mut acc = Accumulator1004::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.64_f32 + y.sin();
        let b = y * 1.134_f32 - x.cos();
        let mut acc = Accumulator1004::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.011_f32 + y.sin();
        let b = y * 7.957_f32 - x.cos();
        let mut acc = Accumulator1004::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.943_f32 + y.sin();
        let b = y * 3.512_f32 - x.cos();
        let mut acc = Accumulator1004::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.738_f32 + y.sin();
        let b = y * 5.038_f32 - x.cos();
        let mut acc = Accumulator1004::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.427_f32 + y.sin();
        let b = y * 3.009_f32 - x.cos();
        let mut acc = Accumulator1004::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.111_f32 + y.sin();
        let b = y * 2.268_f32 - x.cos();
        let mut acc = Accumulator1004::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.181_f32 + y.sin();
        let b = y * 0.375_f32 - x.cos();
        let mut acc = Accumulator1004::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.087_f32 + y.sin();
        let b = y * 5.622_f32 - x.cos();
        let mut acc = Accumulator1004::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.018_f32 + y.sin();
        let b = y * 7.266_f32 - x.cos();
        let mut acc = Accumulator1004::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1004(seed: u64) -> u64 {
        let re = Regex::new(r"m1004-(\d+)").unwrap();
        let hay = format!("m1004-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_1004() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1004(total as u64) % 997) as f32;
        total
    }
}

pub mod m1005 {
    use super::*;

    pub struct Accumulator1005<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1005<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.189_f32 + y.sin();
        let b = y * 5.597_f32 - x.cos();
        let mut acc = Accumulator1005::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.272_f32 + y.sin();
        let b = y * 2.218_f32 - x.cos();
        let mut acc = Accumulator1005::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.728_f32 + y.sin();
        let b = y * 2.461_f32 - x.cos();
        let mut acc = Accumulator1005::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.682_f32 + y.sin();
        let b = y * 0.806_f32 - x.cos();
        let mut acc = Accumulator1005::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.357_f32 + y.sin();
        let b = y * 7.414_f32 - x.cos();
        let mut acc = Accumulator1005::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.262_f32 + y.sin();
        let b = y * 5.156_f32 - x.cos();
        let mut acc = Accumulator1005::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.228_f32 + y.sin();
        let b = y * 8.558_f32 - x.cos();
        let mut acc = Accumulator1005::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.852_f32 + y.sin();
        let b = y * 8.525_f32 - x.cos();
        let mut acc = Accumulator1005::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.63_f32 + y.sin();
        let b = y * 5.492_f32 - x.cos();
        let mut acc = Accumulator1005::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.925_f32 + y.sin();
        let b = y * 2.526_f32 - x.cos();
        let mut acc = Accumulator1005::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.263_f32 + y.sin();
        let b = y * 7.866_f32 - x.cos();
        let mut acc = Accumulator1005::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.825_f32 + y.sin();
        let b = y * 8.582_f32 - x.cos();
        let mut acc = Accumulator1005::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.365_f32 + y.sin();
        let b = y * 5.167_f32 - x.cos();
        let mut acc = Accumulator1005::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.929_f32 + y.sin();
        let b = y * 7.749_f32 - x.cos();
        let mut acc = Accumulator1005::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.368_f32 + y.sin();
        let b = y * 1.737_f32 - x.cos();
        let mut acc = Accumulator1005::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.671_f32 + y.sin();
        let b = y * 1.253_f32 - x.cos();
        let mut acc = Accumulator1005::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.238_f32 + y.sin();
        let b = y * 5.305_f32 - x.cos();
        let mut acc = Accumulator1005::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.854_f32 + y.sin();
        let b = y * 7.902_f32 - x.cos();
        let mut acc = Accumulator1005::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.519_f32 + y.sin();
        let b = y * 5.895_f32 - x.cos();
        let mut acc = Accumulator1005::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.105_f32 + y.sin();
        let b = y * 8.597_f32 - x.cos();
        let mut acc = Accumulator1005::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1005(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_1005() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1005(total as u64) % 997) as f32;
        total
    }
}

pub mod m1006 {
    use super::*;

    pub struct Accumulator1006<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1006<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.442_f32 + y.sin();
        let b = y * 3.198_f32 - x.cos();
        let mut acc = Accumulator1006::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.238_f32 + y.sin();
        let b = y * 0.68_f32 - x.cos();
        let mut acc = Accumulator1006::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.567_f32 + y.sin();
        let b = y * 6.579_f32 - x.cos();
        let mut acc = Accumulator1006::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.908_f32 + y.sin();
        let b = y * 5.027_f32 - x.cos();
        let mut acc = Accumulator1006::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.163_f32 + y.sin();
        let b = y * 9.695_f32 - x.cos();
        let mut acc = Accumulator1006::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.561_f32 + y.sin();
        let b = y * 2.194_f32 - x.cos();
        let mut acc = Accumulator1006::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.394_f32 + y.sin();
        let b = y * 3.195_f32 - x.cos();
        let mut acc = Accumulator1006::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.734_f32 + y.sin();
        let b = y * 0.599_f32 - x.cos();
        let mut acc = Accumulator1006::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.239_f32 + y.sin();
        let b = y * 1.207_f32 - x.cos();
        let mut acc = Accumulator1006::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.614_f32 + y.sin();
        let b = y * 0.685_f32 - x.cos();
        let mut acc = Accumulator1006::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.553_f32 + y.sin();
        let b = y * 0.34_f32 - x.cos();
        let mut acc = Accumulator1006::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.796_f32 + y.sin();
        let b = y * 9.095_f32 - x.cos();
        let mut acc = Accumulator1006::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.681_f32 + y.sin();
        let b = y * 3.774_f32 - x.cos();
        let mut acc = Accumulator1006::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.549_f32 + y.sin();
        let b = y * 4.194_f32 - x.cos();
        let mut acc = Accumulator1006::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.13_f32 + y.sin();
        let b = y * 8.905_f32 - x.cos();
        let mut acc = Accumulator1006::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.928_f32 + y.sin();
        let b = y * 7.608_f32 - x.cos();
        let mut acc = Accumulator1006::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.391_f32 + y.sin();
        let b = y * 7.884_f32 - x.cos();
        let mut acc = Accumulator1006::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.135_f32 + y.sin();
        let b = y * 1.747_f32 - x.cos();
        let mut acc = Accumulator1006::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.304_f32 + y.sin();
        let b = y * 7.393_f32 - x.cos();
        let mut acc = Accumulator1006::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.644_f32 + y.sin();
        let b = y * 8.402_f32 - x.cos();
        let mut acc = Accumulator1006::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1006(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(1006u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_1006() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1006(total as u64) % 997) as f32;
        total
    }
}

pub mod m1007 {
    use super::*;

    pub struct Accumulator1007<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1007<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.399_f32 + y.sin();
        let b = y * 0.161_f32 - x.cos();
        let mut acc = Accumulator1007::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.018_f32 + y.sin();
        let b = y * 3.108_f32 - x.cos();
        let mut acc = Accumulator1007::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.803_f32 + y.sin();
        let b = y * 6.039_f32 - x.cos();
        let mut acc = Accumulator1007::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.487_f32 + y.sin();
        let b = y * 0.73_f32 - x.cos();
        let mut acc = Accumulator1007::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.675_f32 + y.sin();
        let b = y * 5.346_f32 - x.cos();
        let mut acc = Accumulator1007::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.006_f32 + y.sin();
        let b = y * 1.615_f32 - x.cos();
        let mut acc = Accumulator1007::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.88_f32 + y.sin();
        let b = y * 3.946_f32 - x.cos();
        let mut acc = Accumulator1007::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.35_f32 + y.sin();
        let b = y * 3.845_f32 - x.cos();
        let mut acc = Accumulator1007::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.956_f32 + y.sin();
        let b = y * 9.561_f32 - x.cos();
        let mut acc = Accumulator1007::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.403_f32 + y.sin();
        let b = y * 3.372_f32 - x.cos();
        let mut acc = Accumulator1007::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.143_f32 + y.sin();
        let b = y * 6.096_f32 - x.cos();
        let mut acc = Accumulator1007::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.73_f32 + y.sin();
        let b = y * 6.928_f32 - x.cos();
        let mut acc = Accumulator1007::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.18_f32 + y.sin();
        let b = y * 3.746_f32 - x.cos();
        let mut acc = Accumulator1007::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.73_f32 + y.sin();
        let b = y * 5.051_f32 - x.cos();
        let mut acc = Accumulator1007::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.502_f32 + y.sin();
        let b = y * 2.713_f32 - x.cos();
        let mut acc = Accumulator1007::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.885_f32 + y.sin();
        let b = y * 2.238_f32 - x.cos();
        let mut acc = Accumulator1007::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.381_f32 + y.sin();
        let b = y * 2.182_f32 - x.cos();
        let mut acc = Accumulator1007::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.255_f32 + y.sin();
        let b = y * 5.093_f32 - x.cos();
        let mut acc = Accumulator1007::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.002_f32 + y.sin();
        let b = y * 1.4_f32 - x.cos();
        let mut acc = Accumulator1007::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.869_f32 + y.sin();
        let b = y * 8.196_f32 - x.cos();
        let mut acc = Accumulator1007::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1007(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_1007() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1007(total as u64) % 997) as f32;
        total
    }
}

pub mod m1008 {
    use super::*;

    pub struct Accumulator1008<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1008<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.168_f32 + y.sin();
        let b = y * 4.148_f32 - x.cos();
        let mut acc = Accumulator1008::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.225_f32 + y.sin();
        let b = y * 0.537_f32 - x.cos();
        let mut acc = Accumulator1008::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.464_f32 + y.sin();
        let b = y * 5.016_f32 - x.cos();
        let mut acc = Accumulator1008::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.925_f32 + y.sin();
        let b = y * 2.232_f32 - x.cos();
        let mut acc = Accumulator1008::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.081_f32 + y.sin();
        let b = y * 3.168_f32 - x.cos();
        let mut acc = Accumulator1008::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.63_f32 + y.sin();
        let b = y * 1.501_f32 - x.cos();
        let mut acc = Accumulator1008::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.12_f32 + y.sin();
        let b = y * 4.048_f32 - x.cos();
        let mut acc = Accumulator1008::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.71_f32 + y.sin();
        let b = y * 9.748_f32 - x.cos();
        let mut acc = Accumulator1008::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.131_f32 + y.sin();
        let b = y * 8.363_f32 - x.cos();
        let mut acc = Accumulator1008::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.42_f32 + y.sin();
        let b = y * 4.532_f32 - x.cos();
        let mut acc = Accumulator1008::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.461_f32 + y.sin();
        let b = y * 6.109_f32 - x.cos();
        let mut acc = Accumulator1008::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.604_f32 + y.sin();
        let b = y * 7.452_f32 - x.cos();
        let mut acc = Accumulator1008::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.582_f32 + y.sin();
        let b = y * 8.232_f32 - x.cos();
        let mut acc = Accumulator1008::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.601_f32 + y.sin();
        let b = y * 3.211_f32 - x.cos();
        let mut acc = Accumulator1008::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.652_f32 + y.sin();
        let b = y * 6.416_f32 - x.cos();
        let mut acc = Accumulator1008::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.278_f32 + y.sin();
        let b = y * 4.603_f32 - x.cos();
        let mut acc = Accumulator1008::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.062_f32 + y.sin();
        let b = y * 5.579_f32 - x.cos();
        let mut acc = Accumulator1008::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.997_f32 + y.sin();
        let b = y * 1.884_f32 - x.cos();
        let mut acc = Accumulator1008::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.723_f32 + y.sin();
        let b = y * 5.915_f32 - x.cos();
        let mut acc = Accumulator1008::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.347_f32 + y.sin();
        let b = y * 3.87_f32 - x.cos();
        let mut acc = Accumulator1008::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1008(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_1008() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1008(total as u64) % 997) as f32;
        total
    }
}

pub mod m1009 {
    use super::*;

    pub struct Accumulator1009<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1009<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.493_f32 + y.sin();
        let b = y * 6.961_f32 - x.cos();
        let mut acc = Accumulator1009::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.926_f32 + y.sin();
        let b = y * 0.548_f32 - x.cos();
        let mut acc = Accumulator1009::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.104_f32 + y.sin();
        let b = y * 8.616_f32 - x.cos();
        let mut acc = Accumulator1009::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.651_f32 + y.sin();
        let b = y * 5.919_f32 - x.cos();
        let mut acc = Accumulator1009::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.05_f32 + y.sin();
        let b = y * 3.041_f32 - x.cos();
        let mut acc = Accumulator1009::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.266_f32 + y.sin();
        let b = y * 1.53_f32 - x.cos();
        let mut acc = Accumulator1009::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.531_f32 + y.sin();
        let b = y * 5.777_f32 - x.cos();
        let mut acc = Accumulator1009::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.237_f32 + y.sin();
        let b = y * 5.994_f32 - x.cos();
        let mut acc = Accumulator1009::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.377_f32 + y.sin();
        let b = y * 0.13_f32 - x.cos();
        let mut acc = Accumulator1009::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.101_f32 + y.sin();
        let b = y * 8.491_f32 - x.cos();
        let mut acc = Accumulator1009::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.87_f32 + y.sin();
        let b = y * 9.432_f32 - x.cos();
        let mut acc = Accumulator1009::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.088_f32 + y.sin();
        let b = y * 7.7_f32 - x.cos();
        let mut acc = Accumulator1009::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.527_f32 + y.sin();
        let b = y * 2.843_f32 - x.cos();
        let mut acc = Accumulator1009::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.236_f32 + y.sin();
        let b = y * 4.334_f32 - x.cos();
        let mut acc = Accumulator1009::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.529_f32 + y.sin();
        let b = y * 0.308_f32 - x.cos();
        let mut acc = Accumulator1009::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.513_f32 + y.sin();
        let b = y * 7.862_f32 - x.cos();
        let mut acc = Accumulator1009::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.643_f32 + y.sin();
        let b = y * 2.804_f32 - x.cos();
        let mut acc = Accumulator1009::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.912_f32 + y.sin();
        let b = y * 1.395_f32 - x.cos();
        let mut acc = Accumulator1009::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.582_f32 + y.sin();
        let b = y * 2.488_f32 - x.cos();
        let mut acc = Accumulator1009::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.559_f32 + y.sin();
        let b = y * 0.142_f32 - x.cos();
        let mut acc = Accumulator1009::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1009(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m1009-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_1009() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1009(total as u64) % 997) as f32;
        total
    }
}

pub mod m1010 {
    use super::*;

    pub struct Accumulator1010<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1010<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.904_f32 + y.sin();
        let b = y * 4.853_f32 - x.cos();
        let mut acc = Accumulator1010::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.711_f32 + y.sin();
        let b = y * 5.141_f32 - x.cos();
        let mut acc = Accumulator1010::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.303_f32 + y.sin();
        let b = y * 3.013_f32 - x.cos();
        let mut acc = Accumulator1010::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.764_f32 + y.sin();
        let b = y * 1.358_f32 - x.cos();
        let mut acc = Accumulator1010::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.546_f32 + y.sin();
        let b = y * 9.56_f32 - x.cos();
        let mut acc = Accumulator1010::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.513_f32 + y.sin();
        let b = y * 6.338_f32 - x.cos();
        let mut acc = Accumulator1010::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.029_f32 + y.sin();
        let b = y * 0.75_f32 - x.cos();
        let mut acc = Accumulator1010::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.124_f32 + y.sin();
        let b = y * 0.857_f32 - x.cos();
        let mut acc = Accumulator1010::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.677_f32 + y.sin();
        let b = y * 6.454_f32 - x.cos();
        let mut acc = Accumulator1010::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.733_f32 + y.sin();
        let b = y * 1.905_f32 - x.cos();
        let mut acc = Accumulator1010::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.254_f32 + y.sin();
        let b = y * 6.712_f32 - x.cos();
        let mut acc = Accumulator1010::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.142_f32 + y.sin();
        let b = y * 8.897_f32 - x.cos();
        let mut acc = Accumulator1010::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.588_f32 + y.sin();
        let b = y * 1.543_f32 - x.cos();
        let mut acc = Accumulator1010::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.052_f32 + y.sin();
        let b = y * 7.693_f32 - x.cos();
        let mut acc = Accumulator1010::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.246_f32 + y.sin();
        let b = y * 4.178_f32 - x.cos();
        let mut acc = Accumulator1010::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.121_f32 + y.sin();
        let b = y * 2.188_f32 - x.cos();
        let mut acc = Accumulator1010::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.548_f32 + y.sin();
        let b = y * 3.752_f32 - x.cos();
        let mut acc = Accumulator1010::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.27_f32 + y.sin();
        let b = y * 5.385_f32 - x.cos();
        let mut acc = Accumulator1010::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.088_f32 + y.sin();
        let b = y * 3.149_f32 - x.cos();
        let mut acc = Accumulator1010::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.077_f32 + y.sin();
        let b = y * 5.043_f32 - x.cos();
        let mut acc = Accumulator1010::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1010(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_1010() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1010(total as u64) % 997) as f32;
        total
    }
}

pub mod m1011 {
    use super::*;

    pub struct Accumulator1011<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1011<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.552_f32 + y.sin();
        let b = y * 1.273_f32 - x.cos();
        let mut acc = Accumulator1011::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.315_f32 + y.sin();
        let b = y * 3.319_f32 - x.cos();
        let mut acc = Accumulator1011::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.763_f32 + y.sin();
        let b = y * 3.006_f32 - x.cos();
        let mut acc = Accumulator1011::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.697_f32 + y.sin();
        let b = y * 4.228_f32 - x.cos();
        let mut acc = Accumulator1011::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.164_f32 + y.sin();
        let b = y * 8.292_f32 - x.cos();
        let mut acc = Accumulator1011::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.162_f32 + y.sin();
        let b = y * 3.236_f32 - x.cos();
        let mut acc = Accumulator1011::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.555_f32 + y.sin();
        let b = y * 0.668_f32 - x.cos();
        let mut acc = Accumulator1011::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.282_f32 + y.sin();
        let b = y * 8.419_f32 - x.cos();
        let mut acc = Accumulator1011::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.476_f32 + y.sin();
        let b = y * 0.685_f32 - x.cos();
        let mut acc = Accumulator1011::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.601_f32 + y.sin();
        let b = y * 3.339_f32 - x.cos();
        let mut acc = Accumulator1011::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.365_f32 + y.sin();
        let b = y * 5.953_f32 - x.cos();
        let mut acc = Accumulator1011::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.615_f32 + y.sin();
        let b = y * 8.527_f32 - x.cos();
        let mut acc = Accumulator1011::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.951_f32 + y.sin();
        let b = y * 0.889_f32 - x.cos();
        let mut acc = Accumulator1011::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.267_f32 + y.sin();
        let b = y * 6.109_f32 - x.cos();
        let mut acc = Accumulator1011::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.128_f32 + y.sin();
        let b = y * 5.13_f32 - x.cos();
        let mut acc = Accumulator1011::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.928_f32 + y.sin();
        let b = y * 5.21_f32 - x.cos();
        let mut acc = Accumulator1011::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.472_f32 + y.sin();
        let b = y * 4.35_f32 - x.cos();
        let mut acc = Accumulator1011::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.168_f32 + y.sin();
        let b = y * 3.464_f32 - x.cos();
        let mut acc = Accumulator1011::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.507_f32 + y.sin();
        let b = y * 5.996_f32 - x.cos();
        let mut acc = Accumulator1011::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.189_f32 + y.sin();
        let b = y * 7.101_f32 - x.cos();
        let mut acc = Accumulator1011::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1011(seed: u64) -> u64 {
        let re = Regex::new(r"m1011-(\d+)").unwrap();
        let hay = format!("m1011-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_1011() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1011(total as u64) % 997) as f32;
        total
    }
}

pub mod m1012 {
    use super::*;

    pub struct Accumulator1012<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1012<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.252_f32 + y.sin();
        let b = y * 0.953_f32 - x.cos();
        let mut acc = Accumulator1012::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.437_f32 + y.sin();
        let b = y * 5.116_f32 - x.cos();
        let mut acc = Accumulator1012::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.377_f32 + y.sin();
        let b = y * 7.807_f32 - x.cos();
        let mut acc = Accumulator1012::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.97_f32 + y.sin();
        let b = y * 3.48_f32 - x.cos();
        let mut acc = Accumulator1012::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.6_f32 + y.sin();
        let b = y * 6.375_f32 - x.cos();
        let mut acc = Accumulator1012::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.854_f32 + y.sin();
        let b = y * 9.539_f32 - x.cos();
        let mut acc = Accumulator1012::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.378_f32 + y.sin();
        let b = y * 5.796_f32 - x.cos();
        let mut acc = Accumulator1012::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.488_f32 + y.sin();
        let b = y * 5.438_f32 - x.cos();
        let mut acc = Accumulator1012::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.808_f32 + y.sin();
        let b = y * 7.836_f32 - x.cos();
        let mut acc = Accumulator1012::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.996_f32 + y.sin();
        let b = y * 3.282_f32 - x.cos();
        let mut acc = Accumulator1012::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.079_f32 + y.sin();
        let b = y * 6.825_f32 - x.cos();
        let mut acc = Accumulator1012::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.145_f32 + y.sin();
        let b = y * 4.752_f32 - x.cos();
        let mut acc = Accumulator1012::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.908_f32 + y.sin();
        let b = y * 5.389_f32 - x.cos();
        let mut acc = Accumulator1012::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.809_f32 + y.sin();
        let b = y * 7.809_f32 - x.cos();
        let mut acc = Accumulator1012::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.206_f32 + y.sin();
        let b = y * 1.802_f32 - x.cos();
        let mut acc = Accumulator1012::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.667_f32 + y.sin();
        let b = y * 1.816_f32 - x.cos();
        let mut acc = Accumulator1012::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.562_f32 + y.sin();
        let b = y * 4.556_f32 - x.cos();
        let mut acc = Accumulator1012::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.629_f32 + y.sin();
        let b = y * 7.081_f32 - x.cos();
        let mut acc = Accumulator1012::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.849_f32 + y.sin();
        let b = y * 3.493_f32 - x.cos();
        let mut acc = Accumulator1012::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.63_f32 + y.sin();
        let b = y * 5.557_f32 - x.cos();
        let mut acc = Accumulator1012::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1012(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_1012() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1012(total as u64) % 997) as f32;
        total
    }
}

pub mod m1013 {
    use super::*;

    pub struct Accumulator1013<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1013<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.231_f32 + y.sin();
        let b = y * 6.104_f32 - x.cos();
        let mut acc = Accumulator1013::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.376_f32 + y.sin();
        let b = y * 4.311_f32 - x.cos();
        let mut acc = Accumulator1013::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.066_f32 + y.sin();
        let b = y * 3.851_f32 - x.cos();
        let mut acc = Accumulator1013::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.334_f32 + y.sin();
        let b = y * 0.773_f32 - x.cos();
        let mut acc = Accumulator1013::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.947_f32 + y.sin();
        let b = y * 0.98_f32 - x.cos();
        let mut acc = Accumulator1013::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.068_f32 + y.sin();
        let b = y * 1.323_f32 - x.cos();
        let mut acc = Accumulator1013::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.884_f32 + y.sin();
        let b = y * 3.687_f32 - x.cos();
        let mut acc = Accumulator1013::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.832_f32 + y.sin();
        let b = y * 9.814_f32 - x.cos();
        let mut acc = Accumulator1013::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.419_f32 + y.sin();
        let b = y * 3.807_f32 - x.cos();
        let mut acc = Accumulator1013::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.032_f32 + y.sin();
        let b = y * 0.279_f32 - x.cos();
        let mut acc = Accumulator1013::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.138_f32 + y.sin();
        let b = y * 4.821_f32 - x.cos();
        let mut acc = Accumulator1013::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.455_f32 + y.sin();
        let b = y * 5.875_f32 - x.cos();
        let mut acc = Accumulator1013::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.832_f32 + y.sin();
        let b = y * 8.21_f32 - x.cos();
        let mut acc = Accumulator1013::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.216_f32 + y.sin();
        let b = y * 7.08_f32 - x.cos();
        let mut acc = Accumulator1013::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.133_f32 + y.sin();
        let b = y * 3.376_f32 - x.cos();
        let mut acc = Accumulator1013::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.441_f32 + y.sin();
        let b = y * 6.917_f32 - x.cos();
        let mut acc = Accumulator1013::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.408_f32 + y.sin();
        let b = y * 2.819_f32 - x.cos();
        let mut acc = Accumulator1013::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.715_f32 + y.sin();
        let b = y * 2.631_f32 - x.cos();
        let mut acc = Accumulator1013::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.493_f32 + y.sin();
        let b = y * 4.756_f32 - x.cos();
        let mut acc = Accumulator1013::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.226_f32 + y.sin();
        let b = y * 6.8_f32 - x.cos();
        let mut acc = Accumulator1013::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1013(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(1013u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_1013() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1013(total as u64) % 997) as f32;
        total
    }
}

pub mod m1014 {
    use super::*;

    pub struct Accumulator1014<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1014<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.494_f32 + y.sin();
        let b = y * 3.832_f32 - x.cos();
        let mut acc = Accumulator1014::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.454_f32 + y.sin();
        let b = y * 6.094_f32 - x.cos();
        let mut acc = Accumulator1014::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.131_f32 + y.sin();
        let b = y * 9.516_f32 - x.cos();
        let mut acc = Accumulator1014::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.231_f32 + y.sin();
        let b = y * 7.945_f32 - x.cos();
        let mut acc = Accumulator1014::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.525_f32 + y.sin();
        let b = y * 9.607_f32 - x.cos();
        let mut acc = Accumulator1014::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.053_f32 + y.sin();
        let b = y * 2.491_f32 - x.cos();
        let mut acc = Accumulator1014::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.65_f32 + y.sin();
        let b = y * 8.55_f32 - x.cos();
        let mut acc = Accumulator1014::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.93_f32 + y.sin();
        let b = y * 1.697_f32 - x.cos();
        let mut acc = Accumulator1014::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.176_f32 + y.sin();
        let b = y * 6.548_f32 - x.cos();
        let mut acc = Accumulator1014::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.952_f32 + y.sin();
        let b = y * 1.37_f32 - x.cos();
        let mut acc = Accumulator1014::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.236_f32 + y.sin();
        let b = y * 6.379_f32 - x.cos();
        let mut acc = Accumulator1014::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.84_f32 + y.sin();
        let b = y * 2.601_f32 - x.cos();
        let mut acc = Accumulator1014::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.364_f32 + y.sin();
        let b = y * 6.484_f32 - x.cos();
        let mut acc = Accumulator1014::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.544_f32 + y.sin();
        let b = y * 1.125_f32 - x.cos();
        let mut acc = Accumulator1014::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.515_f32 + y.sin();
        let b = y * 9.499_f32 - x.cos();
        let mut acc = Accumulator1014::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.096_f32 + y.sin();
        let b = y * 4.175_f32 - x.cos();
        let mut acc = Accumulator1014::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.723_f32 + y.sin();
        let b = y * 1.355_f32 - x.cos();
        let mut acc = Accumulator1014::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.823_f32 + y.sin();
        let b = y * 0.698_f32 - x.cos();
        let mut acc = Accumulator1014::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.348_f32 + y.sin();
        let b = y * 3.215_f32 - x.cos();
        let mut acc = Accumulator1014::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.673_f32 + y.sin();
        let b = y * 0.508_f32 - x.cos();
        let mut acc = Accumulator1014::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1014(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_1014() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1014(total as u64) % 997) as f32;
        total
    }
}

pub mod m1015 {
    use super::*;

    pub struct Accumulator1015<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1015<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.389_f32 + y.sin();
        let b = y * 3.996_f32 - x.cos();
        let mut acc = Accumulator1015::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.866_f32 + y.sin();
        let b = y * 6.269_f32 - x.cos();
        let mut acc = Accumulator1015::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.979_f32 + y.sin();
        let b = y * 6.085_f32 - x.cos();
        let mut acc = Accumulator1015::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.464_f32 + y.sin();
        let b = y * 4.571_f32 - x.cos();
        let mut acc = Accumulator1015::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.296_f32 + y.sin();
        let b = y * 4.256_f32 - x.cos();
        let mut acc = Accumulator1015::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.781_f32 + y.sin();
        let b = y * 7.611_f32 - x.cos();
        let mut acc = Accumulator1015::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.12_f32 + y.sin();
        let b = y * 3.865_f32 - x.cos();
        let mut acc = Accumulator1015::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.695_f32 + y.sin();
        let b = y * 9.429_f32 - x.cos();
        let mut acc = Accumulator1015::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.815_f32 + y.sin();
        let b = y * 0.39_f32 - x.cos();
        let mut acc = Accumulator1015::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.358_f32 + y.sin();
        let b = y * 3.571_f32 - x.cos();
        let mut acc = Accumulator1015::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.563_f32 + y.sin();
        let b = y * 6.7_f32 - x.cos();
        let mut acc = Accumulator1015::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.166_f32 + y.sin();
        let b = y * 1.358_f32 - x.cos();
        let mut acc = Accumulator1015::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.359_f32 + y.sin();
        let b = y * 2.981_f32 - x.cos();
        let mut acc = Accumulator1015::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.387_f32 + y.sin();
        let b = y * 8.353_f32 - x.cos();
        let mut acc = Accumulator1015::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.158_f32 + y.sin();
        let b = y * 8.139_f32 - x.cos();
        let mut acc = Accumulator1015::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.807_f32 + y.sin();
        let b = y * 8.194_f32 - x.cos();
        let mut acc = Accumulator1015::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.846_f32 + y.sin();
        let b = y * 2.549_f32 - x.cos();
        let mut acc = Accumulator1015::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.863_f32 + y.sin();
        let b = y * 6.817_f32 - x.cos();
        let mut acc = Accumulator1015::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.41_f32 + y.sin();
        let b = y * 6.8_f32 - x.cos();
        let mut acc = Accumulator1015::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.714_f32 + y.sin();
        let b = y * 8.164_f32 - x.cos();
        let mut acc = Accumulator1015::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1015(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_1015() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1015(total as u64) % 997) as f32;
        total
    }
}

pub mod m1016 {
    use super::*;

    pub struct Accumulator1016<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1016<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.186_f32 + y.sin();
        let b = y * 2.078_f32 - x.cos();
        let mut acc = Accumulator1016::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.625_f32 + y.sin();
        let b = y * 8.659_f32 - x.cos();
        let mut acc = Accumulator1016::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.219_f32 + y.sin();
        let b = y * 6.897_f32 - x.cos();
        let mut acc = Accumulator1016::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.311_f32 + y.sin();
        let b = y * 6.797_f32 - x.cos();
        let mut acc = Accumulator1016::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.936_f32 + y.sin();
        let b = y * 8.49_f32 - x.cos();
        let mut acc = Accumulator1016::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.34_f32 + y.sin();
        let b = y * 4.528_f32 - x.cos();
        let mut acc = Accumulator1016::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.01_f32 + y.sin();
        let b = y * 5.043_f32 - x.cos();
        let mut acc = Accumulator1016::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.404_f32 + y.sin();
        let b = y * 4.443_f32 - x.cos();
        let mut acc = Accumulator1016::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.598_f32 + y.sin();
        let b = y * 0.114_f32 - x.cos();
        let mut acc = Accumulator1016::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.928_f32 + y.sin();
        let b = y * 0.88_f32 - x.cos();
        let mut acc = Accumulator1016::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.056_f32 + y.sin();
        let b = y * 0.587_f32 - x.cos();
        let mut acc = Accumulator1016::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.448_f32 + y.sin();
        let b = y * 1.801_f32 - x.cos();
        let mut acc = Accumulator1016::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.066_f32 + y.sin();
        let b = y * 4.652_f32 - x.cos();
        let mut acc = Accumulator1016::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.724_f32 + y.sin();
        let b = y * 2.413_f32 - x.cos();
        let mut acc = Accumulator1016::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.761_f32 + y.sin();
        let b = y * 1.725_f32 - x.cos();
        let mut acc = Accumulator1016::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.392_f32 + y.sin();
        let b = y * 4.059_f32 - x.cos();
        let mut acc = Accumulator1016::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.718_f32 + y.sin();
        let b = y * 5.556_f32 - x.cos();
        let mut acc = Accumulator1016::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.862_f32 + y.sin();
        let b = y * 1.834_f32 - x.cos();
        let mut acc = Accumulator1016::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.11_f32 + y.sin();
        let b = y * 3.205_f32 - x.cos();
        let mut acc = Accumulator1016::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.015_f32 + y.sin();
        let b = y * 0.258_f32 - x.cos();
        let mut acc = Accumulator1016::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1016(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m1016-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_1016() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1016(total as u64) % 997) as f32;
        total
    }
}

pub mod m1017 {
    use super::*;

    pub struct Accumulator1017<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1017<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.919_f32 + y.sin();
        let b = y * 5.047_f32 - x.cos();
        let mut acc = Accumulator1017::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.884_f32 + y.sin();
        let b = y * 8.714_f32 - x.cos();
        let mut acc = Accumulator1017::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.987_f32 + y.sin();
        let b = y * 3.775_f32 - x.cos();
        let mut acc = Accumulator1017::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.666_f32 + y.sin();
        let b = y * 1.732_f32 - x.cos();
        let mut acc = Accumulator1017::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.414_f32 + y.sin();
        let b = y * 9.55_f32 - x.cos();
        let mut acc = Accumulator1017::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.946_f32 + y.sin();
        let b = y * 4.436_f32 - x.cos();
        let mut acc = Accumulator1017::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.308_f32 + y.sin();
        let b = y * 3.217_f32 - x.cos();
        let mut acc = Accumulator1017::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.768_f32 + y.sin();
        let b = y * 2.679_f32 - x.cos();
        let mut acc = Accumulator1017::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.841_f32 + y.sin();
        let b = y * 2.394_f32 - x.cos();
        let mut acc = Accumulator1017::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.563_f32 + y.sin();
        let b = y * 6.007_f32 - x.cos();
        let mut acc = Accumulator1017::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.058_f32 + y.sin();
        let b = y * 4.052_f32 - x.cos();
        let mut acc = Accumulator1017::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.61_f32 + y.sin();
        let b = y * 8.869_f32 - x.cos();
        let mut acc = Accumulator1017::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.398_f32 + y.sin();
        let b = y * 7.865_f32 - x.cos();
        let mut acc = Accumulator1017::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.51_f32 + y.sin();
        let b = y * 8.622_f32 - x.cos();
        let mut acc = Accumulator1017::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.894_f32 + y.sin();
        let b = y * 5.103_f32 - x.cos();
        let mut acc = Accumulator1017::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.132_f32 + y.sin();
        let b = y * 5.678_f32 - x.cos();
        let mut acc = Accumulator1017::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.27_f32 + y.sin();
        let b = y * 0.572_f32 - x.cos();
        let mut acc = Accumulator1017::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.51_f32 + y.sin();
        let b = y * 3.826_f32 - x.cos();
        let mut acc = Accumulator1017::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.797_f32 + y.sin();
        let b = y * 3.685_f32 - x.cos();
        let mut acc = Accumulator1017::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.278_f32 + y.sin();
        let b = y * 9.25_f32 - x.cos();
        let mut acc = Accumulator1017::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1017(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_1017() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1017(total as u64) % 997) as f32;
        total
    }
}

pub mod m1018 {
    use super::*;

    pub struct Accumulator1018<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1018<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.632_f32 + y.sin();
        let b = y * 4.758_f32 - x.cos();
        let mut acc = Accumulator1018::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.672_f32 + y.sin();
        let b = y * 4.58_f32 - x.cos();
        let mut acc = Accumulator1018::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.233_f32 + y.sin();
        let b = y * 5.634_f32 - x.cos();
        let mut acc = Accumulator1018::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.387_f32 + y.sin();
        let b = y * 8.282_f32 - x.cos();
        let mut acc = Accumulator1018::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.81_f32 + y.sin();
        let b = y * 6.481_f32 - x.cos();
        let mut acc = Accumulator1018::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.952_f32 + y.sin();
        let b = y * 6.81_f32 - x.cos();
        let mut acc = Accumulator1018::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.523_f32 + y.sin();
        let b = y * 7.78_f32 - x.cos();
        let mut acc = Accumulator1018::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.421_f32 + y.sin();
        let b = y * 2.96_f32 - x.cos();
        let mut acc = Accumulator1018::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.516_f32 + y.sin();
        let b = y * 4.992_f32 - x.cos();
        let mut acc = Accumulator1018::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.28_f32 + y.sin();
        let b = y * 0.134_f32 - x.cos();
        let mut acc = Accumulator1018::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.163_f32 + y.sin();
        let b = y * 0.496_f32 - x.cos();
        let mut acc = Accumulator1018::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.846_f32 + y.sin();
        let b = y * 4.607_f32 - x.cos();
        let mut acc = Accumulator1018::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.039_f32 + y.sin();
        let b = y * 7.68_f32 - x.cos();
        let mut acc = Accumulator1018::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.777_f32 + y.sin();
        let b = y * 0.693_f32 - x.cos();
        let mut acc = Accumulator1018::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.23_f32 + y.sin();
        let b = y * 1.856_f32 - x.cos();
        let mut acc = Accumulator1018::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.312_f32 + y.sin();
        let b = y * 2.977_f32 - x.cos();
        let mut acc = Accumulator1018::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.08_f32 + y.sin();
        let b = y * 2.473_f32 - x.cos();
        let mut acc = Accumulator1018::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.397_f32 + y.sin();
        let b = y * 1.546_f32 - x.cos();
        let mut acc = Accumulator1018::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.259_f32 + y.sin();
        let b = y * 9.31_f32 - x.cos();
        let mut acc = Accumulator1018::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.452_f32 + y.sin();
        let b = y * 0.507_f32 - x.cos();
        let mut acc = Accumulator1018::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1018(seed: u64) -> u64 {
        let re = Regex::new(r"m1018-(\d+)").unwrap();
        let hay = format!("m1018-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_1018() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1018(total as u64) % 997) as f32;
        total
    }
}

pub mod m1019 {
    use super::*;

    pub struct Accumulator1019<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1019<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.225_f32 + y.sin();
        let b = y * 5.641_f32 - x.cos();
        let mut acc = Accumulator1019::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.458_f32 + y.sin();
        let b = y * 2.84_f32 - x.cos();
        let mut acc = Accumulator1019::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.225_f32 + y.sin();
        let b = y * 2.26_f32 - x.cos();
        let mut acc = Accumulator1019::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.395_f32 + y.sin();
        let b = y * 0.466_f32 - x.cos();
        let mut acc = Accumulator1019::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.992_f32 + y.sin();
        let b = y * 0.173_f32 - x.cos();
        let mut acc = Accumulator1019::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.388_f32 + y.sin();
        let b = y * 0.738_f32 - x.cos();
        let mut acc = Accumulator1019::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.125_f32 + y.sin();
        let b = y * 8.086_f32 - x.cos();
        let mut acc = Accumulator1019::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.375_f32 + y.sin();
        let b = y * 2.436_f32 - x.cos();
        let mut acc = Accumulator1019::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.936_f32 + y.sin();
        let b = y * 1.037_f32 - x.cos();
        let mut acc = Accumulator1019::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.542_f32 + y.sin();
        let b = y * 3.597_f32 - x.cos();
        let mut acc = Accumulator1019::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.515_f32 + y.sin();
        let b = y * 9.522_f32 - x.cos();
        let mut acc = Accumulator1019::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.105_f32 + y.sin();
        let b = y * 1.394_f32 - x.cos();
        let mut acc = Accumulator1019::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.859_f32 + y.sin();
        let b = y * 9.73_f32 - x.cos();
        let mut acc = Accumulator1019::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.142_f32 + y.sin();
        let b = y * 2.461_f32 - x.cos();
        let mut acc = Accumulator1019::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.632_f32 + y.sin();
        let b = y * 2.118_f32 - x.cos();
        let mut acc = Accumulator1019::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.237_f32 + y.sin();
        let b = y * 8.475_f32 - x.cos();
        let mut acc = Accumulator1019::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.894_f32 + y.sin();
        let b = y * 7.916_f32 - x.cos();
        let mut acc = Accumulator1019::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.445_f32 + y.sin();
        let b = y * 6.177_f32 - x.cos();
        let mut acc = Accumulator1019::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.12_f32 + y.sin();
        let b = y * 8.314_f32 - x.cos();
        let mut acc = Accumulator1019::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.091_f32 + y.sin();
        let b = y * 5.675_f32 - x.cos();
        let mut acc = Accumulator1019::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1019(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_1019() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1019(total as u64) % 997) as f32;
        total
    }
}

pub mod m1020 {
    use super::*;

    pub struct Accumulator1020<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1020<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.172_f32 + y.sin();
        let b = y * 6.408_f32 - x.cos();
        let mut acc = Accumulator1020::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.101_f32 + y.sin();
        let b = y * 8.31_f32 - x.cos();
        let mut acc = Accumulator1020::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.862_f32 + y.sin();
        let b = y * 8.306_f32 - x.cos();
        let mut acc = Accumulator1020::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.68_f32 + y.sin();
        let b = y * 8.006_f32 - x.cos();
        let mut acc = Accumulator1020::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.031_f32 + y.sin();
        let b = y * 8.252_f32 - x.cos();
        let mut acc = Accumulator1020::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.235_f32 + y.sin();
        let b = y * 1.632_f32 - x.cos();
        let mut acc = Accumulator1020::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.144_f32 + y.sin();
        let b = y * 1.43_f32 - x.cos();
        let mut acc = Accumulator1020::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.294_f32 + y.sin();
        let b = y * 1.378_f32 - x.cos();
        let mut acc = Accumulator1020::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.001_f32 + y.sin();
        let b = y * 9.159_f32 - x.cos();
        let mut acc = Accumulator1020::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.626_f32 + y.sin();
        let b = y * 4.701_f32 - x.cos();
        let mut acc = Accumulator1020::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.125_f32 + y.sin();
        let b = y * 4.6_f32 - x.cos();
        let mut acc = Accumulator1020::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.03_f32 + y.sin();
        let b = y * 0.803_f32 - x.cos();
        let mut acc = Accumulator1020::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.874_f32 + y.sin();
        let b = y * 1.644_f32 - x.cos();
        let mut acc = Accumulator1020::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.307_f32 + y.sin();
        let b = y * 1.083_f32 - x.cos();
        let mut acc = Accumulator1020::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.185_f32 + y.sin();
        let b = y * 0.904_f32 - x.cos();
        let mut acc = Accumulator1020::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.753_f32 + y.sin();
        let b = y * 2.769_f32 - x.cos();
        let mut acc = Accumulator1020::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.177_f32 + y.sin();
        let b = y * 2.295_f32 - x.cos();
        let mut acc = Accumulator1020::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.01_f32 + y.sin();
        let b = y * 1.283_f32 - x.cos();
        let mut acc = Accumulator1020::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.895_f32 + y.sin();
        let b = y * 0.399_f32 - x.cos();
        let mut acc = Accumulator1020::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.481_f32 + y.sin();
        let b = y * 4.362_f32 - x.cos();
        let mut acc = Accumulator1020::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1020(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(1020u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_1020() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1020(total as u64) % 997) as f32;
        total
    }
}

pub mod m1021 {
    use super::*;

    pub struct Accumulator1021<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1021<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.324_f32 + y.sin();
        let b = y * 2.711_f32 - x.cos();
        let mut acc = Accumulator1021::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.01_f32 + y.sin();
        let b = y * 7.157_f32 - x.cos();
        let mut acc = Accumulator1021::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.534_f32 + y.sin();
        let b = y * 8.733_f32 - x.cos();
        let mut acc = Accumulator1021::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.759_f32 + y.sin();
        let b = y * 2.777_f32 - x.cos();
        let mut acc = Accumulator1021::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.495_f32 + y.sin();
        let b = y * 4.803_f32 - x.cos();
        let mut acc = Accumulator1021::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.413_f32 + y.sin();
        let b = y * 1.72_f32 - x.cos();
        let mut acc = Accumulator1021::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.869_f32 + y.sin();
        let b = y * 3.537_f32 - x.cos();
        let mut acc = Accumulator1021::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.935_f32 + y.sin();
        let b = y * 1.057_f32 - x.cos();
        let mut acc = Accumulator1021::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.699_f32 + y.sin();
        let b = y * 7.886_f32 - x.cos();
        let mut acc = Accumulator1021::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.454_f32 + y.sin();
        let b = y * 9.783_f32 - x.cos();
        let mut acc = Accumulator1021::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.819_f32 + y.sin();
        let b = y * 1.431_f32 - x.cos();
        let mut acc = Accumulator1021::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.145_f32 + y.sin();
        let b = y * 5.127_f32 - x.cos();
        let mut acc = Accumulator1021::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.593_f32 + y.sin();
        let b = y * 1.152_f32 - x.cos();
        let mut acc = Accumulator1021::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.103_f32 + y.sin();
        let b = y * 0.455_f32 - x.cos();
        let mut acc = Accumulator1021::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.187_f32 + y.sin();
        let b = y * 6.924_f32 - x.cos();
        let mut acc = Accumulator1021::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.445_f32 + y.sin();
        let b = y * 5.386_f32 - x.cos();
        let mut acc = Accumulator1021::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.022_f32 + y.sin();
        let b = y * 2.937_f32 - x.cos();
        let mut acc = Accumulator1021::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.35_f32 + y.sin();
        let b = y * 9.862_f32 - x.cos();
        let mut acc = Accumulator1021::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.14_f32 + y.sin();
        let b = y * 6.699_f32 - x.cos();
        let mut acc = Accumulator1021::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.18_f32 + y.sin();
        let b = y * 1.356_f32 - x.cos();
        let mut acc = Accumulator1021::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1021(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_1021() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1021(total as u64) % 997) as f32;
        total
    }
}

pub mod m1022 {
    use super::*;

    pub struct Accumulator1022<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1022<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.882_f32 + y.sin();
        let b = y * 2.717_f32 - x.cos();
        let mut acc = Accumulator1022::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.533_f32 + y.sin();
        let b = y * 7.672_f32 - x.cos();
        let mut acc = Accumulator1022::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.612_f32 + y.sin();
        let b = y * 5.776_f32 - x.cos();
        let mut acc = Accumulator1022::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.806_f32 + y.sin();
        let b = y * 8.864_f32 - x.cos();
        let mut acc = Accumulator1022::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.27_f32 + y.sin();
        let b = y * 5.22_f32 - x.cos();
        let mut acc = Accumulator1022::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.122_f32 + y.sin();
        let b = y * 7.134_f32 - x.cos();
        let mut acc = Accumulator1022::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.639_f32 + y.sin();
        let b = y * 5.672_f32 - x.cos();
        let mut acc = Accumulator1022::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.878_f32 + y.sin();
        let b = y * 6.153_f32 - x.cos();
        let mut acc = Accumulator1022::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.37_f32 + y.sin();
        let b = y * 4.636_f32 - x.cos();
        let mut acc = Accumulator1022::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.589_f32 + y.sin();
        let b = y * 8.334_f32 - x.cos();
        let mut acc = Accumulator1022::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.608_f32 + y.sin();
        let b = y * 8.336_f32 - x.cos();
        let mut acc = Accumulator1022::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.636_f32 + y.sin();
        let b = y * 2.32_f32 - x.cos();
        let mut acc = Accumulator1022::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.274_f32 + y.sin();
        let b = y * 1.204_f32 - x.cos();
        let mut acc = Accumulator1022::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.928_f32 + y.sin();
        let b = y * 2.084_f32 - x.cos();
        let mut acc = Accumulator1022::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.056_f32 + y.sin();
        let b = y * 9.578_f32 - x.cos();
        let mut acc = Accumulator1022::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.523_f32 + y.sin();
        let b = y * 9.495_f32 - x.cos();
        let mut acc = Accumulator1022::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.44_f32 + y.sin();
        let b = y * 3.791_f32 - x.cos();
        let mut acc = Accumulator1022::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.564_f32 + y.sin();
        let b = y * 9.487_f32 - x.cos();
        let mut acc = Accumulator1022::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.704_f32 + y.sin();
        let b = y * 1.407_f32 - x.cos();
        let mut acc = Accumulator1022::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.468_f32 + y.sin();
        let b = y * 4.364_f32 - x.cos();
        let mut acc = Accumulator1022::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1022(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_1022() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1022(total as u64) % 997) as f32;
        total
    }
}

pub mod m1023 {
    use super::*;

    pub struct Accumulator1023<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1023<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.459_f32 + y.sin();
        let b = y * 7.228_f32 - x.cos();
        let mut acc = Accumulator1023::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.822_f32 + y.sin();
        let b = y * 1.711_f32 - x.cos();
        let mut acc = Accumulator1023::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.451_f32 + y.sin();
        let b = y * 0.472_f32 - x.cos();
        let mut acc = Accumulator1023::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.922_f32 + y.sin();
        let b = y * 8.301_f32 - x.cos();
        let mut acc = Accumulator1023::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.792_f32 + y.sin();
        let b = y * 5.562_f32 - x.cos();
        let mut acc = Accumulator1023::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.68_f32 + y.sin();
        let b = y * 1.751_f32 - x.cos();
        let mut acc = Accumulator1023::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.622_f32 + y.sin();
        let b = y * 4.06_f32 - x.cos();
        let mut acc = Accumulator1023::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.437_f32 + y.sin();
        let b = y * 5.255_f32 - x.cos();
        let mut acc = Accumulator1023::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.488_f32 + y.sin();
        let b = y * 2.068_f32 - x.cos();
        let mut acc = Accumulator1023::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.174_f32 + y.sin();
        let b = y * 0.704_f32 - x.cos();
        let mut acc = Accumulator1023::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.783_f32 + y.sin();
        let b = y * 3.354_f32 - x.cos();
        let mut acc = Accumulator1023::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.261_f32 + y.sin();
        let b = y * 7.196_f32 - x.cos();
        let mut acc = Accumulator1023::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.253_f32 + y.sin();
        let b = y * 5.423_f32 - x.cos();
        let mut acc = Accumulator1023::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.508_f32 + y.sin();
        let b = y * 3.316_f32 - x.cos();
        let mut acc = Accumulator1023::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.594_f32 + y.sin();
        let b = y * 2.488_f32 - x.cos();
        let mut acc = Accumulator1023::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.915_f32 + y.sin();
        let b = y * 2.878_f32 - x.cos();
        let mut acc = Accumulator1023::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.481_f32 + y.sin();
        let b = y * 3.502_f32 - x.cos();
        let mut acc = Accumulator1023::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.669_f32 + y.sin();
        let b = y * 2.773_f32 - x.cos();
        let mut acc = Accumulator1023::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.527_f32 + y.sin();
        let b = y * 3.399_f32 - x.cos();
        let mut acc = Accumulator1023::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.753_f32 + y.sin();
        let b = y * 8.241_f32 - x.cos();
        let mut acc = Accumulator1023::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1023(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m1023-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_1023() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1023(total as u64) % 997) as f32;
        total
    }
}

pub mod m1024 {
    use super::*;

    pub struct Accumulator1024<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1024<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.175_f32 + y.sin();
        let b = y * 9.737_f32 - x.cos();
        let mut acc = Accumulator1024::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.967_f32 + y.sin();
        let b = y * 5.92_f32 - x.cos();
        let mut acc = Accumulator1024::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.399_f32 + y.sin();
        let b = y * 6.932_f32 - x.cos();
        let mut acc = Accumulator1024::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.868_f32 + y.sin();
        let b = y * 6.282_f32 - x.cos();
        let mut acc = Accumulator1024::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.737_f32 + y.sin();
        let b = y * 5.877_f32 - x.cos();
        let mut acc = Accumulator1024::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.984_f32 + y.sin();
        let b = y * 6.636_f32 - x.cos();
        let mut acc = Accumulator1024::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.259_f32 + y.sin();
        let b = y * 5.831_f32 - x.cos();
        let mut acc = Accumulator1024::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.208_f32 + y.sin();
        let b = y * 8.912_f32 - x.cos();
        let mut acc = Accumulator1024::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.409_f32 + y.sin();
        let b = y * 0.525_f32 - x.cos();
        let mut acc = Accumulator1024::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.719_f32 + y.sin();
        let b = y * 0.261_f32 - x.cos();
        let mut acc = Accumulator1024::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.677_f32 + y.sin();
        let b = y * 8.233_f32 - x.cos();
        let mut acc = Accumulator1024::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.986_f32 + y.sin();
        let b = y * 4.604_f32 - x.cos();
        let mut acc = Accumulator1024::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.524_f32 + y.sin();
        let b = y * 1.001_f32 - x.cos();
        let mut acc = Accumulator1024::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.117_f32 + y.sin();
        let b = y * 6.762_f32 - x.cos();
        let mut acc = Accumulator1024::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.332_f32 + y.sin();
        let b = y * 8.899_f32 - x.cos();
        let mut acc = Accumulator1024::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.991_f32 + y.sin();
        let b = y * 0.959_f32 - x.cos();
        let mut acc = Accumulator1024::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.175_f32 + y.sin();
        let b = y * 0.128_f32 - x.cos();
        let mut acc = Accumulator1024::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.603_f32 + y.sin();
        let b = y * 6.991_f32 - x.cos();
        let mut acc = Accumulator1024::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.326_f32 + y.sin();
        let b = y * 8.284_f32 - x.cos();
        let mut acc = Accumulator1024::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.562_f32 + y.sin();
        let b = y * 6.799_f32 - x.cos();
        let mut acc = Accumulator1024::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1024(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_1024() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1024(total as u64) % 997) as f32;
        total
    }
}

pub mod m1025 {
    use super::*;

    pub struct Accumulator1025<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1025<T> {
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
        let b = y * 5.882_f32 - x.cos();
        let mut acc = Accumulator1025::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.796_f32 + y.sin();
        let b = y * 1.813_f32 - x.cos();
        let mut acc = Accumulator1025::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.138_f32 + y.sin();
        let b = y * 0.594_f32 - x.cos();
        let mut acc = Accumulator1025::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.236_f32 + y.sin();
        let b = y * 6.473_f32 - x.cos();
        let mut acc = Accumulator1025::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.495_f32 + y.sin();
        let b = y * 5.788_f32 - x.cos();
        let mut acc = Accumulator1025::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.264_f32 + y.sin();
        let b = y * 2.656_f32 - x.cos();
        let mut acc = Accumulator1025::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.134_f32 + y.sin();
        let b = y * 1.525_f32 - x.cos();
        let mut acc = Accumulator1025::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.756_f32 + y.sin();
        let b = y * 1.987_f32 - x.cos();
        let mut acc = Accumulator1025::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.348_f32 + y.sin();
        let b = y * 6.53_f32 - x.cos();
        let mut acc = Accumulator1025::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.432_f32 + y.sin();
        let b = y * 0.808_f32 - x.cos();
        let mut acc = Accumulator1025::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.097_f32 + y.sin();
        let b = y * 8.354_f32 - x.cos();
        let mut acc = Accumulator1025::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.501_f32 + y.sin();
        let b = y * 4.969_f32 - x.cos();
        let mut acc = Accumulator1025::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.594_f32 + y.sin();
        let b = y * 1.053_f32 - x.cos();
        let mut acc = Accumulator1025::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.863_f32 + y.sin();
        let b = y * 0.988_f32 - x.cos();
        let mut acc = Accumulator1025::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.631_f32 + y.sin();
        let b = y * 1.004_f32 - x.cos();
        let mut acc = Accumulator1025::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.617_f32 + y.sin();
        let b = y * 8.934_f32 - x.cos();
        let mut acc = Accumulator1025::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.741_f32 + y.sin();
        let b = y * 2.889_f32 - x.cos();
        let mut acc = Accumulator1025::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.612_f32 + y.sin();
        let b = y * 5.53_f32 - x.cos();
        let mut acc = Accumulator1025::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.183_f32 + y.sin();
        let b = y * 0.816_f32 - x.cos();
        let mut acc = Accumulator1025::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.774_f32 + y.sin();
        let b = y * 8.834_f32 - x.cos();
        let mut acc = Accumulator1025::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1025(seed: u64) -> u64 {
        let re = Regex::new(r"m1025-(\d+)").unwrap();
        let hay = format!("m1025-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_1025() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1025(total as u64) % 997) as f32;
        total
    }
}

pub mod m1026 {
    use super::*;

    pub struct Accumulator1026<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1026<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.639_f32 + y.sin();
        let b = y * 6.51_f32 - x.cos();
        let mut acc = Accumulator1026::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.69_f32 + y.sin();
        let b = y * 8.719_f32 - x.cos();
        let mut acc = Accumulator1026::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.884_f32 + y.sin();
        let b = y * 4.36_f32 - x.cos();
        let mut acc = Accumulator1026::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.497_f32 + y.sin();
        let b = y * 7.312_f32 - x.cos();
        let mut acc = Accumulator1026::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.973_f32 + y.sin();
        let b = y * 5.312_f32 - x.cos();
        let mut acc = Accumulator1026::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.407_f32 + y.sin();
        let b = y * 7.825_f32 - x.cos();
        let mut acc = Accumulator1026::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.101_f32 + y.sin();
        let b = y * 5.363_f32 - x.cos();
        let mut acc = Accumulator1026::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.311_f32 + y.sin();
        let b = y * 3.611_f32 - x.cos();
        let mut acc = Accumulator1026::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.359_f32 + y.sin();
        let b = y * 4.042_f32 - x.cos();
        let mut acc = Accumulator1026::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.024_f32 + y.sin();
        let b = y * 5.797_f32 - x.cos();
        let mut acc = Accumulator1026::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.438_f32 + y.sin();
        let b = y * 0.826_f32 - x.cos();
        let mut acc = Accumulator1026::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.513_f32 + y.sin();
        let b = y * 8.469_f32 - x.cos();
        let mut acc = Accumulator1026::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.274_f32 + y.sin();
        let b = y * 1.312_f32 - x.cos();
        let mut acc = Accumulator1026::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.493_f32 + y.sin();
        let b = y * 2.771_f32 - x.cos();
        let mut acc = Accumulator1026::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.795_f32 + y.sin();
        let b = y * 3.788_f32 - x.cos();
        let mut acc = Accumulator1026::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.858_f32 + y.sin();
        let b = y * 1.373_f32 - x.cos();
        let mut acc = Accumulator1026::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.54_f32 + y.sin();
        let b = y * 3.836_f32 - x.cos();
        let mut acc = Accumulator1026::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.385_f32 + y.sin();
        let b = y * 5.756_f32 - x.cos();
        let mut acc = Accumulator1026::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.274_f32 + y.sin();
        let b = y * 6.631_f32 - x.cos();
        let mut acc = Accumulator1026::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.854_f32 + y.sin();
        let b = y * 3.959_f32 - x.cos();
        let mut acc = Accumulator1026::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1026(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_1026() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1026(total as u64) % 997) as f32;
        total
    }
}

pub mod m1027 {
    use super::*;

    pub struct Accumulator1027<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1027<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.281_f32 + y.sin();
        let b = y * 1.402_f32 - x.cos();
        let mut acc = Accumulator1027::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.062_f32 + y.sin();
        let b = y * 7.768_f32 - x.cos();
        let mut acc = Accumulator1027::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.609_f32 + y.sin();
        let b = y * 7.775_f32 - x.cos();
        let mut acc = Accumulator1027::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.952_f32 + y.sin();
        let b = y * 6.685_f32 - x.cos();
        let mut acc = Accumulator1027::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.968_f32 + y.sin();
        let b = y * 9.638_f32 - x.cos();
        let mut acc = Accumulator1027::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.682_f32 + y.sin();
        let b = y * 8.028_f32 - x.cos();
        let mut acc = Accumulator1027::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.598_f32 + y.sin();
        let b = y * 6.17_f32 - x.cos();
        let mut acc = Accumulator1027::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.146_f32 + y.sin();
        let b = y * 2.537_f32 - x.cos();
        let mut acc = Accumulator1027::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.651_f32 + y.sin();
        let b = y * 5.316_f32 - x.cos();
        let mut acc = Accumulator1027::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.332_f32 + y.sin();
        let b = y * 3.086_f32 - x.cos();
        let mut acc = Accumulator1027::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.885_f32 + y.sin();
        let b = y * 9.264_f32 - x.cos();
        let mut acc = Accumulator1027::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.539_f32 + y.sin();
        let b = y * 8.348_f32 - x.cos();
        let mut acc = Accumulator1027::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.669_f32 + y.sin();
        let b = y * 1.041_f32 - x.cos();
        let mut acc = Accumulator1027::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.767_f32 + y.sin();
        let b = y * 5.477_f32 - x.cos();
        let mut acc = Accumulator1027::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.73_f32 + y.sin();
        let b = y * 5.329_f32 - x.cos();
        let mut acc = Accumulator1027::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.224_f32 + y.sin();
        let b = y * 4.328_f32 - x.cos();
        let mut acc = Accumulator1027::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.613_f32 + y.sin();
        let b = y * 3.408_f32 - x.cos();
        let mut acc = Accumulator1027::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.862_f32 + y.sin();
        let b = y * 6.394_f32 - x.cos();
        let mut acc = Accumulator1027::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.363_f32 + y.sin();
        let b = y * 6.597_f32 - x.cos();
        let mut acc = Accumulator1027::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.212_f32 + y.sin();
        let b = y * 5.983_f32 - x.cos();
        let mut acc = Accumulator1027::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1027(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(1027u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_1027() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1027(total as u64) % 997) as f32;
        total
    }
}

pub mod m1028 {
    use super::*;

    pub struct Accumulator1028<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1028<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.351_f32 + y.sin();
        let b = y * 4.121_f32 - x.cos();
        let mut acc = Accumulator1028::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.194_f32 + y.sin();
        let b = y * 5.793_f32 - x.cos();
        let mut acc = Accumulator1028::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.735_f32 + y.sin();
        let b = y * 3.582_f32 - x.cos();
        let mut acc = Accumulator1028::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.336_f32 + y.sin();
        let b = y * 4.024_f32 - x.cos();
        let mut acc = Accumulator1028::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.058_f32 + y.sin();
        let b = y * 0.226_f32 - x.cos();
        let mut acc = Accumulator1028::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.699_f32 + y.sin();
        let b = y * 7.848_f32 - x.cos();
        let mut acc = Accumulator1028::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.871_f32 + y.sin();
        let b = y * 5.737_f32 - x.cos();
        let mut acc = Accumulator1028::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.59_f32 + y.sin();
        let b = y * 7.748_f32 - x.cos();
        let mut acc = Accumulator1028::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.034_f32 + y.sin();
        let b = y * 4.557_f32 - x.cos();
        let mut acc = Accumulator1028::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.227_f32 + y.sin();
        let b = y * 5.486_f32 - x.cos();
        let mut acc = Accumulator1028::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.425_f32 + y.sin();
        let b = y * 5.466_f32 - x.cos();
        let mut acc = Accumulator1028::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.219_f32 + y.sin();
        let b = y * 4.771_f32 - x.cos();
        let mut acc = Accumulator1028::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.554_f32 + y.sin();
        let b = y * 0.168_f32 - x.cos();
        let mut acc = Accumulator1028::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.762_f32 + y.sin();
        let b = y * 8.506_f32 - x.cos();
        let mut acc = Accumulator1028::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.606_f32 + y.sin();
        let b = y * 8.949_f32 - x.cos();
        let mut acc = Accumulator1028::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.039_f32 + y.sin();
        let b = y * 9.75_f32 - x.cos();
        let mut acc = Accumulator1028::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.843_f32 + y.sin();
        let b = y * 9.376_f32 - x.cos();
        let mut acc = Accumulator1028::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.808_f32 + y.sin();
        let b = y * 3.167_f32 - x.cos();
        let mut acc = Accumulator1028::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.596_f32 + y.sin();
        let b = y * 2.997_f32 - x.cos();
        let mut acc = Accumulator1028::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.135_f32 + y.sin();
        let b = y * 4.204_f32 - x.cos();
        let mut acc = Accumulator1028::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1028(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_1028() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1028(total as u64) % 997) as f32;
        total
    }
}

pub mod m1029 {
    use super::*;

    pub struct Accumulator1029<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1029<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.173_f32 + y.sin();
        let b = y * 2.787_f32 - x.cos();
        let mut acc = Accumulator1029::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.201_f32 + y.sin();
        let b = y * 7.059_f32 - x.cos();
        let mut acc = Accumulator1029::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.579_f32 + y.sin();
        let b = y * 2.949_f32 - x.cos();
        let mut acc = Accumulator1029::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.7_f32 + y.sin();
        let b = y * 0.102_f32 - x.cos();
        let mut acc = Accumulator1029::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.914_f32 + y.sin();
        let b = y * 3.196_f32 - x.cos();
        let mut acc = Accumulator1029::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.444_f32 + y.sin();
        let b = y * 5.089_f32 - x.cos();
        let mut acc = Accumulator1029::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.336_f32 + y.sin();
        let b = y * 6.964_f32 - x.cos();
        let mut acc = Accumulator1029::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.276_f32 + y.sin();
        let b = y * 4.399_f32 - x.cos();
        let mut acc = Accumulator1029::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.123_f32 + y.sin();
        let b = y * 0.615_f32 - x.cos();
        let mut acc = Accumulator1029::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.523_f32 + y.sin();
        let b = y * 7.933_f32 - x.cos();
        let mut acc = Accumulator1029::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.08_f32 + y.sin();
        let b = y * 5.982_f32 - x.cos();
        let mut acc = Accumulator1029::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.403_f32 + y.sin();
        let b = y * 2.081_f32 - x.cos();
        let mut acc = Accumulator1029::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.43_f32 + y.sin();
        let b = y * 0.633_f32 - x.cos();
        let mut acc = Accumulator1029::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.806_f32 + y.sin();
        let b = y * 7.158_f32 - x.cos();
        let mut acc = Accumulator1029::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.817_f32 + y.sin();
        let b = y * 3.627_f32 - x.cos();
        let mut acc = Accumulator1029::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.295_f32 + y.sin();
        let b = y * 1.636_f32 - x.cos();
        let mut acc = Accumulator1029::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.899_f32 + y.sin();
        let b = y * 8.287_f32 - x.cos();
        let mut acc = Accumulator1029::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.535_f32 + y.sin();
        let b = y * 5.717_f32 - x.cos();
        let mut acc = Accumulator1029::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.191_f32 + y.sin();
        let b = y * 7.095_f32 - x.cos();
        let mut acc = Accumulator1029::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.871_f32 + y.sin();
        let b = y * 5.516_f32 - x.cos();
        let mut acc = Accumulator1029::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1029(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_1029() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1029(total as u64) % 997) as f32;
        total
    }
}

pub mod m1030 {
    use super::*;

    pub struct Accumulator1030<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1030<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.735_f32 + y.sin();
        let b = y * 0.563_f32 - x.cos();
        let mut acc = Accumulator1030::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.717_f32 + y.sin();
        let b = y * 6.908_f32 - x.cos();
        let mut acc = Accumulator1030::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.142_f32 + y.sin();
        let b = y * 0.913_f32 - x.cos();
        let mut acc = Accumulator1030::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.643_f32 + y.sin();
        let b = y * 1.449_f32 - x.cos();
        let mut acc = Accumulator1030::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.408_f32 + y.sin();
        let b = y * 4.256_f32 - x.cos();
        let mut acc = Accumulator1030::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.133_f32 + y.sin();
        let b = y * 8.072_f32 - x.cos();
        let mut acc = Accumulator1030::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.4_f32 + y.sin();
        let b = y * 0.612_f32 - x.cos();
        let mut acc = Accumulator1030::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.808_f32 + y.sin();
        let b = y * 6.26_f32 - x.cos();
        let mut acc = Accumulator1030::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.156_f32 + y.sin();
        let b = y * 4.112_f32 - x.cos();
        let mut acc = Accumulator1030::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.521_f32 + y.sin();
        let b = y * 2.188_f32 - x.cos();
        let mut acc = Accumulator1030::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.316_f32 + y.sin();
        let b = y * 1.637_f32 - x.cos();
        let mut acc = Accumulator1030::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.689_f32 + y.sin();
        let b = y * 6.295_f32 - x.cos();
        let mut acc = Accumulator1030::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.334_f32 + y.sin();
        let b = y * 2.402_f32 - x.cos();
        let mut acc = Accumulator1030::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.264_f32 + y.sin();
        let b = y * 8.357_f32 - x.cos();
        let mut acc = Accumulator1030::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.738_f32 + y.sin();
        let b = y * 3.313_f32 - x.cos();
        let mut acc = Accumulator1030::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.926_f32 + y.sin();
        let b = y * 5.924_f32 - x.cos();
        let mut acc = Accumulator1030::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.7_f32 + y.sin();
        let b = y * 2.432_f32 - x.cos();
        let mut acc = Accumulator1030::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.158_f32 + y.sin();
        let b = y * 4.049_f32 - x.cos();
        let mut acc = Accumulator1030::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.628_f32 + y.sin();
        let b = y * 1.601_f32 - x.cos();
        let mut acc = Accumulator1030::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.527_f32 + y.sin();
        let b = y * 8.642_f32 - x.cos();
        let mut acc = Accumulator1030::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1030(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m1030-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_1030() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1030(total as u64) % 997) as f32;
        total
    }
}

pub mod m1031 {
    use super::*;

    pub struct Accumulator1031<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1031<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.126_f32 + y.sin();
        let b = y * 3.912_f32 - x.cos();
        let mut acc = Accumulator1031::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.082_f32 + y.sin();
        let b = y * 3.614_f32 - x.cos();
        let mut acc = Accumulator1031::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.004_f32 + y.sin();
        let b = y * 8.633_f32 - x.cos();
        let mut acc = Accumulator1031::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.315_f32 + y.sin();
        let b = y * 8.682_f32 - x.cos();
        let mut acc = Accumulator1031::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.191_f32 + y.sin();
        let b = y * 4.342_f32 - x.cos();
        let mut acc = Accumulator1031::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.156_f32 + y.sin();
        let b = y * 8.971_f32 - x.cos();
        let mut acc = Accumulator1031::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.288_f32 + y.sin();
        let b = y * 1.041_f32 - x.cos();
        let mut acc = Accumulator1031::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.63_f32 + y.sin();
        let b = y * 0.307_f32 - x.cos();
        let mut acc = Accumulator1031::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.911_f32 + y.sin();
        let b = y * 3.915_f32 - x.cos();
        let mut acc = Accumulator1031::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.604_f32 + y.sin();
        let b = y * 0.381_f32 - x.cos();
        let mut acc = Accumulator1031::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.629_f32 + y.sin();
        let b = y * 8.229_f32 - x.cos();
        let mut acc = Accumulator1031::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.769_f32 + y.sin();
        let b = y * 0.101_f32 - x.cos();
        let mut acc = Accumulator1031::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.243_f32 + y.sin();
        let b = y * 2.772_f32 - x.cos();
        let mut acc = Accumulator1031::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.433_f32 + y.sin();
        let b = y * 5.474_f32 - x.cos();
        let mut acc = Accumulator1031::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.466_f32 + y.sin();
        let b = y * 7.936_f32 - x.cos();
        let mut acc = Accumulator1031::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.146_f32 + y.sin();
        let b = y * 3.968_f32 - x.cos();
        let mut acc = Accumulator1031::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.589_f32 + y.sin();
        let b = y * 7.51_f32 - x.cos();
        let mut acc = Accumulator1031::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.271_f32 + y.sin();
        let b = y * 0.925_f32 - x.cos();
        let mut acc = Accumulator1031::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.324_f32 + y.sin();
        let b = y * 1.15_f32 - x.cos();
        let mut acc = Accumulator1031::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.211_f32 + y.sin();
        let b = y * 0.608_f32 - x.cos();
        let mut acc = Accumulator1031::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1031(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_1031() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1031(total as u64) % 997) as f32;
        total
    }
}

pub mod m1032 {
    use super::*;

    pub struct Accumulator1032<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1032<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.669_f32 + y.sin();
        let b = y * 2.069_f32 - x.cos();
        let mut acc = Accumulator1032::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.383_f32 + y.sin();
        let b = y * 7.958_f32 - x.cos();
        let mut acc = Accumulator1032::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.958_f32 + y.sin();
        let b = y * 1.42_f32 - x.cos();
        let mut acc = Accumulator1032::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.553_f32 + y.sin();
        let b = y * 0.374_f32 - x.cos();
        let mut acc = Accumulator1032::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.315_f32 + y.sin();
        let b = y * 5.534_f32 - x.cos();
        let mut acc = Accumulator1032::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.467_f32 + y.sin();
        let b = y * 1.416_f32 - x.cos();
        let mut acc = Accumulator1032::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.336_f32 + y.sin();
        let b = y * 5.654_f32 - x.cos();
        let mut acc = Accumulator1032::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.425_f32 + y.sin();
        let b = y * 1.047_f32 - x.cos();
        let mut acc = Accumulator1032::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.108_f32 + y.sin();
        let b = y * 5.07_f32 - x.cos();
        let mut acc = Accumulator1032::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.133_f32 + y.sin();
        let b = y * 1.263_f32 - x.cos();
        let mut acc = Accumulator1032::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.307_f32 + y.sin();
        let b = y * 9.277_f32 - x.cos();
        let mut acc = Accumulator1032::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.899_f32 + y.sin();
        let b = y * 6.237_f32 - x.cos();
        let mut acc = Accumulator1032::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.297_f32 + y.sin();
        let b = y * 5.474_f32 - x.cos();
        let mut acc = Accumulator1032::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.22_f32 + y.sin();
        let b = y * 6.418_f32 - x.cos();
        let mut acc = Accumulator1032::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.764_f32 + y.sin();
        let b = y * 1.204_f32 - x.cos();
        let mut acc = Accumulator1032::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.209_f32 + y.sin();
        let b = y * 2.449_f32 - x.cos();
        let mut acc = Accumulator1032::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.569_f32 + y.sin();
        let b = y * 4.875_f32 - x.cos();
        let mut acc = Accumulator1032::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.908_f32 + y.sin();
        let b = y * 3.971_f32 - x.cos();
        let mut acc = Accumulator1032::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.649_f32 + y.sin();
        let b = y * 7.839_f32 - x.cos();
        let mut acc = Accumulator1032::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.395_f32 + y.sin();
        let b = y * 6.971_f32 - x.cos();
        let mut acc = Accumulator1032::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1032(seed: u64) -> u64 {
        let re = Regex::new(r"m1032-(\d+)").unwrap();
        let hay = format!("m1032-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_1032() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1032(total as u64) % 997) as f32;
        total
    }
}

pub mod m1033 {
    use super::*;

    pub struct Accumulator1033<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1033<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.235_f32 + y.sin();
        let b = y * 8.172_f32 - x.cos();
        let mut acc = Accumulator1033::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.242_f32 + y.sin();
        let b = y * 0.913_f32 - x.cos();
        let mut acc = Accumulator1033::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.336_f32 + y.sin();
        let b = y * 4.652_f32 - x.cos();
        let mut acc = Accumulator1033::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.068_f32 + y.sin();
        let b = y * 8.123_f32 - x.cos();
        let mut acc = Accumulator1033::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.515_f32 + y.sin();
        let b = y * 7.156_f32 - x.cos();
        let mut acc = Accumulator1033::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.195_f32 + y.sin();
        let b = y * 1.213_f32 - x.cos();
        let mut acc = Accumulator1033::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.705_f32 + y.sin();
        let b = y * 4.948_f32 - x.cos();
        let mut acc = Accumulator1033::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.925_f32 + y.sin();
        let b = y * 0.726_f32 - x.cos();
        let mut acc = Accumulator1033::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.141_f32 + y.sin();
        let b = y * 8.736_f32 - x.cos();
        let mut acc = Accumulator1033::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.194_f32 + y.sin();
        let b = y * 1.451_f32 - x.cos();
        let mut acc = Accumulator1033::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.81_f32 + y.sin();
        let b = y * 1.67_f32 - x.cos();
        let mut acc = Accumulator1033::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.198_f32 + y.sin();
        let b = y * 9.842_f32 - x.cos();
        let mut acc = Accumulator1033::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.934_f32 + y.sin();
        let b = y * 7.185_f32 - x.cos();
        let mut acc = Accumulator1033::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.817_f32 + y.sin();
        let b = y * 9.353_f32 - x.cos();
        let mut acc = Accumulator1033::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.884_f32 + y.sin();
        let b = y * 7.442_f32 - x.cos();
        let mut acc = Accumulator1033::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.422_f32 + y.sin();
        let b = y * 1.823_f32 - x.cos();
        let mut acc = Accumulator1033::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.597_f32 + y.sin();
        let b = y * 7.027_f32 - x.cos();
        let mut acc = Accumulator1033::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.48_f32 + y.sin();
        let b = y * 8.942_f32 - x.cos();
        let mut acc = Accumulator1033::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.7_f32 + y.sin();
        let b = y * 4.994_f32 - x.cos();
        let mut acc = Accumulator1033::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.087_f32 + y.sin();
        let b = y * 8.245_f32 - x.cos();
        let mut acc = Accumulator1033::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1033(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_1033() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1033(total as u64) % 997) as f32;
        total
    }
}

pub mod m1034 {
    use super::*;

    pub struct Accumulator1034<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1034<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.579_f32 + y.sin();
        let b = y * 7.308_f32 - x.cos();
        let mut acc = Accumulator1034::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.893_f32 + y.sin();
        let b = y * 7.889_f32 - x.cos();
        let mut acc = Accumulator1034::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.219_f32 + y.sin();
        let b = y * 4.3_f32 - x.cos();
        let mut acc = Accumulator1034::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.379_f32 + y.sin();
        let b = y * 2.023_f32 - x.cos();
        let mut acc = Accumulator1034::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.315_f32 + y.sin();
        let b = y * 3.166_f32 - x.cos();
        let mut acc = Accumulator1034::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.496_f32 + y.sin();
        let b = y * 1.68_f32 - x.cos();
        let mut acc = Accumulator1034::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.918_f32 + y.sin();
        let b = y * 4.934_f32 - x.cos();
        let mut acc = Accumulator1034::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.39_f32 + y.sin();
        let b = y * 1.424_f32 - x.cos();
        let mut acc = Accumulator1034::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.456_f32 + y.sin();
        let b = y * 1.478_f32 - x.cos();
        let mut acc = Accumulator1034::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.314_f32 + y.sin();
        let b = y * 2.994_f32 - x.cos();
        let mut acc = Accumulator1034::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.782_f32 + y.sin();
        let b = y * 3.368_f32 - x.cos();
        let mut acc = Accumulator1034::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.24_f32 + y.sin();
        let b = y * 4.338_f32 - x.cos();
        let mut acc = Accumulator1034::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.857_f32 + y.sin();
        let b = y * 8.17_f32 - x.cos();
        let mut acc = Accumulator1034::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.695_f32 + y.sin();
        let b = y * 5.909_f32 - x.cos();
        let mut acc = Accumulator1034::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.0_f32 + y.sin();
        let b = y * 1.218_f32 - x.cos();
        let mut acc = Accumulator1034::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.516_f32 + y.sin();
        let b = y * 0.734_f32 - x.cos();
        let mut acc = Accumulator1034::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.337_f32 + y.sin();
        let b = y * 8.537_f32 - x.cos();
        let mut acc = Accumulator1034::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.987_f32 + y.sin();
        let b = y * 8.127_f32 - x.cos();
        let mut acc = Accumulator1034::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.059_f32 + y.sin();
        let b = y * 2.195_f32 - x.cos();
        let mut acc = Accumulator1034::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.952_f32 + y.sin();
        let b = y * 0.876_f32 - x.cos();
        let mut acc = Accumulator1034::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1034(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(1034u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_1034() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1034(total as u64) % 997) as f32;
        total
    }
}

pub mod m1035 {
    use super::*;

    pub struct Accumulator1035<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1035<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.944_f32 + y.sin();
        let b = y * 9.859_f32 - x.cos();
        let mut acc = Accumulator1035::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.136_f32 + y.sin();
        let b = y * 3.195_f32 - x.cos();
        let mut acc = Accumulator1035::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.741_f32 + y.sin();
        let b = y * 5.659_f32 - x.cos();
        let mut acc = Accumulator1035::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.146_f32 + y.sin();
        let b = y * 4.953_f32 - x.cos();
        let mut acc = Accumulator1035::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.236_f32 + y.sin();
        let b = y * 0.791_f32 - x.cos();
        let mut acc = Accumulator1035::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.779_f32 + y.sin();
        let b = y * 2.571_f32 - x.cos();
        let mut acc = Accumulator1035::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.397_f32 + y.sin();
        let b = y * 3.828_f32 - x.cos();
        let mut acc = Accumulator1035::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.07_f32 + y.sin();
        let b = y * 9.785_f32 - x.cos();
        let mut acc = Accumulator1035::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.135_f32 + y.sin();
        let b = y * 6.683_f32 - x.cos();
        let mut acc = Accumulator1035::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.868_f32 + y.sin();
        let b = y * 3.974_f32 - x.cos();
        let mut acc = Accumulator1035::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.888_f32 + y.sin();
        let b = y * 0.605_f32 - x.cos();
        let mut acc = Accumulator1035::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.577_f32 + y.sin();
        let b = y * 7.588_f32 - x.cos();
        let mut acc = Accumulator1035::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.192_f32 + y.sin();
        let b = y * 7.389_f32 - x.cos();
        let mut acc = Accumulator1035::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.11_f32 + y.sin();
        let b = y * 0.397_f32 - x.cos();
        let mut acc = Accumulator1035::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.434_f32 + y.sin();
        let b = y * 4.556_f32 - x.cos();
        let mut acc = Accumulator1035::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.798_f32 + y.sin();
        let b = y * 5.326_f32 - x.cos();
        let mut acc = Accumulator1035::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.505_f32 + y.sin();
        let b = y * 5.164_f32 - x.cos();
        let mut acc = Accumulator1035::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.336_f32 + y.sin();
        let b = y * 6.759_f32 - x.cos();
        let mut acc = Accumulator1035::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.841_f32 + y.sin();
        let b = y * 0.361_f32 - x.cos();
        let mut acc = Accumulator1035::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.098_f32 + y.sin();
        let b = y * 2.626_f32 - x.cos();
        let mut acc = Accumulator1035::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1035(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_1035() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1035(total as u64) % 997) as f32;
        total
    }
}

pub mod m1036 {
    use super::*;

    pub struct Accumulator1036<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1036<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.457_f32 + y.sin();
        let b = y * 7.62_f32 - x.cos();
        let mut acc = Accumulator1036::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.944_f32 + y.sin();
        let b = y * 0.36_f32 - x.cos();
        let mut acc = Accumulator1036::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.722_f32 + y.sin();
        let b = y * 3.064_f32 - x.cos();
        let mut acc = Accumulator1036::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.377_f32 + y.sin();
        let b = y * 2.571_f32 - x.cos();
        let mut acc = Accumulator1036::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.794_f32 + y.sin();
        let b = y * 1.311_f32 - x.cos();
        let mut acc = Accumulator1036::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.279_f32 + y.sin();
        let b = y * 1.028_f32 - x.cos();
        let mut acc = Accumulator1036::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.743_f32 + y.sin();
        let b = y * 5.564_f32 - x.cos();
        let mut acc = Accumulator1036::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.452_f32 + y.sin();
        let b = y * 2.677_f32 - x.cos();
        let mut acc = Accumulator1036::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.589_f32 + y.sin();
        let b = y * 2.852_f32 - x.cos();
        let mut acc = Accumulator1036::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.561_f32 + y.sin();
        let b = y * 8.968_f32 - x.cos();
        let mut acc = Accumulator1036::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.18_f32 + y.sin();
        let b = y * 5.021_f32 - x.cos();
        let mut acc = Accumulator1036::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.239_f32 + y.sin();
        let b = y * 9.493_f32 - x.cos();
        let mut acc = Accumulator1036::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.733_f32 + y.sin();
        let b = y * 2.625_f32 - x.cos();
        let mut acc = Accumulator1036::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.137_f32 + y.sin();
        let b = y * 9.419_f32 - x.cos();
        let mut acc = Accumulator1036::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.7_f32 + y.sin();
        let b = y * 7.175_f32 - x.cos();
        let mut acc = Accumulator1036::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.223_f32 + y.sin();
        let b = y * 5.087_f32 - x.cos();
        let mut acc = Accumulator1036::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.579_f32 + y.sin();
        let b = y * 3.434_f32 - x.cos();
        let mut acc = Accumulator1036::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.788_f32 + y.sin();
        let b = y * 2.038_f32 - x.cos();
        let mut acc = Accumulator1036::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.229_f32 + y.sin();
        let b = y * 2.198_f32 - x.cos();
        let mut acc = Accumulator1036::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.531_f32 + y.sin();
        let b = y * 4.891_f32 - x.cos();
        let mut acc = Accumulator1036::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1036(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_1036() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1036(total as u64) % 997) as f32;
        total
    }
}

pub mod m1037 {
    use super::*;

    pub struct Accumulator1037<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1037<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.027_f32 + y.sin();
        let b = y * 1.615_f32 - x.cos();
        let mut acc = Accumulator1037::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.054_f32 + y.sin();
        let b = y * 7.983_f32 - x.cos();
        let mut acc = Accumulator1037::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.111_f32 + y.sin();
        let b = y * 5.367_f32 - x.cos();
        let mut acc = Accumulator1037::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.209_f32 + y.sin();
        let b = y * 9.833_f32 - x.cos();
        let mut acc = Accumulator1037::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.21_f32 + y.sin();
        let b = y * 9.816_f32 - x.cos();
        let mut acc = Accumulator1037::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.023_f32 + y.sin();
        let b = y * 5.577_f32 - x.cos();
        let mut acc = Accumulator1037::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.843_f32 + y.sin();
        let b = y * 8.151_f32 - x.cos();
        let mut acc = Accumulator1037::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.81_f32 + y.sin();
        let b = y * 0.811_f32 - x.cos();
        let mut acc = Accumulator1037::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.771_f32 + y.sin();
        let b = y * 0.776_f32 - x.cos();
        let mut acc = Accumulator1037::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.306_f32 + y.sin();
        let b = y * 9.461_f32 - x.cos();
        let mut acc = Accumulator1037::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.961_f32 + y.sin();
        let b = y * 3.202_f32 - x.cos();
        let mut acc = Accumulator1037::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.365_f32 + y.sin();
        let b = y * 5.521_f32 - x.cos();
        let mut acc = Accumulator1037::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.691_f32 + y.sin();
        let b = y * 9.639_f32 - x.cos();
        let mut acc = Accumulator1037::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.072_f32 + y.sin();
        let b = y * 4.975_f32 - x.cos();
        let mut acc = Accumulator1037::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.125_f32 + y.sin();
        let b = y * 5.84_f32 - x.cos();
        let mut acc = Accumulator1037::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.4_f32 + y.sin();
        let b = y * 6.459_f32 - x.cos();
        let mut acc = Accumulator1037::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.809_f32 + y.sin();
        let b = y * 8.951_f32 - x.cos();
        let mut acc = Accumulator1037::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.105_f32 + y.sin();
        let b = y * 3.697_f32 - x.cos();
        let mut acc = Accumulator1037::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.785_f32 + y.sin();
        let b = y * 9.512_f32 - x.cos();
        let mut acc = Accumulator1037::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.397_f32 + y.sin();
        let b = y * 6.222_f32 - x.cos();
        let mut acc = Accumulator1037::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1037(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m1037-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_1037() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1037(total as u64) % 997) as f32;
        total
    }
}

pub mod m1038 {
    use super::*;

    pub struct Accumulator1038<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1038<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.261_f32 + y.sin();
        let b = y * 4.93_f32 - x.cos();
        let mut acc = Accumulator1038::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.413_f32 + y.sin();
        let b = y * 5.635_f32 - x.cos();
        let mut acc = Accumulator1038::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.487_f32 + y.sin();
        let b = y * 7.041_f32 - x.cos();
        let mut acc = Accumulator1038::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.047_f32 + y.sin();
        let b = y * 5.681_f32 - x.cos();
        let mut acc = Accumulator1038::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.251_f32 + y.sin();
        let b = y * 1.175_f32 - x.cos();
        let mut acc = Accumulator1038::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.44_f32 + y.sin();
        let b = y * 3.32_f32 - x.cos();
        let mut acc = Accumulator1038::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.685_f32 + y.sin();
        let b = y * 2.169_f32 - x.cos();
        let mut acc = Accumulator1038::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.007_f32 + y.sin();
        let b = y * 7.666_f32 - x.cos();
        let mut acc = Accumulator1038::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.702_f32 + y.sin();
        let b = y * 5.805_f32 - x.cos();
        let mut acc = Accumulator1038::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.767_f32 + y.sin();
        let b = y * 4.078_f32 - x.cos();
        let mut acc = Accumulator1038::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.1_f32 + y.sin();
        let b = y * 3.27_f32 - x.cos();
        let mut acc = Accumulator1038::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.697_f32 + y.sin();
        let b = y * 9.568_f32 - x.cos();
        let mut acc = Accumulator1038::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.194_f32 + y.sin();
        let b = y * 7.673_f32 - x.cos();
        let mut acc = Accumulator1038::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.426_f32 + y.sin();
        let b = y * 0.8_f32 - x.cos();
        let mut acc = Accumulator1038::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.159_f32 + y.sin();
        let b = y * 7.851_f32 - x.cos();
        let mut acc = Accumulator1038::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.851_f32 + y.sin();
        let b = y * 9.771_f32 - x.cos();
        let mut acc = Accumulator1038::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.67_f32 + y.sin();
        let b = y * 2.238_f32 - x.cos();
        let mut acc = Accumulator1038::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.204_f32 + y.sin();
        let b = y * 8.594_f32 - x.cos();
        let mut acc = Accumulator1038::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.602_f32 + y.sin();
        let b = y * 6.185_f32 - x.cos();
        let mut acc = Accumulator1038::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.737_f32 + y.sin();
        let b = y * 0.21_f32 - x.cos();
        let mut acc = Accumulator1038::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1038(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_1038() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1038(total as u64) % 997) as f32;
        total
    }
}

pub mod m1039 {
    use super::*;

    pub struct Accumulator1039<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1039<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.154_f32 + y.sin();
        let b = y * 7.365_f32 - x.cos();
        let mut acc = Accumulator1039::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.512_f32 + y.sin();
        let b = y * 1.393_f32 - x.cos();
        let mut acc = Accumulator1039::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.02_f32 + y.sin();
        let b = y * 5.035_f32 - x.cos();
        let mut acc = Accumulator1039::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.567_f32 + y.sin();
        let b = y * 9.171_f32 - x.cos();
        let mut acc = Accumulator1039::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.836_f32 + y.sin();
        let b = y * 5.265_f32 - x.cos();
        let mut acc = Accumulator1039::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.433_f32 + y.sin();
        let b = y * 6.414_f32 - x.cos();
        let mut acc = Accumulator1039::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.104_f32 + y.sin();
        let b = y * 5.719_f32 - x.cos();
        let mut acc = Accumulator1039::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.86_f32 + y.sin();
        let b = y * 2.279_f32 - x.cos();
        let mut acc = Accumulator1039::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.3_f32 + y.sin();
        let b = y * 6.537_f32 - x.cos();
        let mut acc = Accumulator1039::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.836_f32 + y.sin();
        let b = y * 9.474_f32 - x.cos();
        let mut acc = Accumulator1039::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.984_f32 + y.sin();
        let b = y * 1.555_f32 - x.cos();
        let mut acc = Accumulator1039::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.535_f32 + y.sin();
        let b = y * 4.708_f32 - x.cos();
        let mut acc = Accumulator1039::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.475_f32 + y.sin();
        let b = y * 6.395_f32 - x.cos();
        let mut acc = Accumulator1039::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.346_f32 + y.sin();
        let b = y * 9.527_f32 - x.cos();
        let mut acc = Accumulator1039::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.016_f32 + y.sin();
        let b = y * 3.963_f32 - x.cos();
        let mut acc = Accumulator1039::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.149_f32 + y.sin();
        let b = y * 4.678_f32 - x.cos();
        let mut acc = Accumulator1039::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.207_f32 + y.sin();
        let b = y * 5.852_f32 - x.cos();
        let mut acc = Accumulator1039::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.811_f32 + y.sin();
        let b = y * 1.985_f32 - x.cos();
        let mut acc = Accumulator1039::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.225_f32 + y.sin();
        let b = y * 2.636_f32 - x.cos();
        let mut acc = Accumulator1039::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.174_f32 + y.sin();
        let b = y * 3.464_f32 - x.cos();
        let mut acc = Accumulator1039::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1039(seed: u64) -> u64 {
        let re = Regex::new(r"m1039-(\d+)").unwrap();
        let hay = format!("m1039-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_1039() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1039(total as u64) % 997) as f32;
        total
    }
}

pub mod m1040 {
    use super::*;

    pub struct Accumulator1040<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1040<T> {
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
        let b = y * 6.366_f32 - x.cos();
        let mut acc = Accumulator1040::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.549_f32 + y.sin();
        let b = y * 2.696_f32 - x.cos();
        let mut acc = Accumulator1040::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.029_f32 + y.sin();
        let b = y * 1.43_f32 - x.cos();
        let mut acc = Accumulator1040::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.87_f32 + y.sin();
        let b = y * 8.662_f32 - x.cos();
        let mut acc = Accumulator1040::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.877_f32 + y.sin();
        let b = y * 6.245_f32 - x.cos();
        let mut acc = Accumulator1040::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.089_f32 + y.sin();
        let b = y * 2.664_f32 - x.cos();
        let mut acc = Accumulator1040::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.364_f32 + y.sin();
        let b = y * 7.06_f32 - x.cos();
        let mut acc = Accumulator1040::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.809_f32 + y.sin();
        let b = y * 9.461_f32 - x.cos();
        let mut acc = Accumulator1040::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.657_f32 + y.sin();
        let b = y * 4.479_f32 - x.cos();
        let mut acc = Accumulator1040::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.736_f32 + y.sin();
        let b = y * 1.028_f32 - x.cos();
        let mut acc = Accumulator1040::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.006_f32 + y.sin();
        let b = y * 3.367_f32 - x.cos();
        let mut acc = Accumulator1040::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.753_f32 + y.sin();
        let b = y * 2.572_f32 - x.cos();
        let mut acc = Accumulator1040::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.513_f32 + y.sin();
        let b = y * 8.714_f32 - x.cos();
        let mut acc = Accumulator1040::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.516_f32 + y.sin();
        let b = y * 2.945_f32 - x.cos();
        let mut acc = Accumulator1040::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.445_f32 + y.sin();
        let b = y * 3.007_f32 - x.cos();
        let mut acc = Accumulator1040::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.589_f32 + y.sin();
        let b = y * 7.631_f32 - x.cos();
        let mut acc = Accumulator1040::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.751_f32 + y.sin();
        let b = y * 0.313_f32 - x.cos();
        let mut acc = Accumulator1040::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.734_f32 + y.sin();
        let b = y * 5.446_f32 - x.cos();
        let mut acc = Accumulator1040::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.606_f32 + y.sin();
        let b = y * 0.381_f32 - x.cos();
        let mut acc = Accumulator1040::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.004_f32 + y.sin();
        let b = y * 5.718_f32 - x.cos();
        let mut acc = Accumulator1040::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1040(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_1040() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1040(total as u64) % 997) as f32;
        total
    }
}

pub mod m1041 {
    use super::*;

    pub struct Accumulator1041<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1041<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.159_f32 + y.sin();
        let b = y * 9.378_f32 - x.cos();
        let mut acc = Accumulator1041::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.533_f32 + y.sin();
        let b = y * 6.511_f32 - x.cos();
        let mut acc = Accumulator1041::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.4_f32 + y.sin();
        let b = y * 7.078_f32 - x.cos();
        let mut acc = Accumulator1041::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.503_f32 + y.sin();
        let b = y * 2.534_f32 - x.cos();
        let mut acc = Accumulator1041::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.115_f32 + y.sin();
        let b = y * 4.968_f32 - x.cos();
        let mut acc = Accumulator1041::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.86_f32 + y.sin();
        let b = y * 8.289_f32 - x.cos();
        let mut acc = Accumulator1041::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.324_f32 + y.sin();
        let b = y * 8.757_f32 - x.cos();
        let mut acc = Accumulator1041::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.118_f32 + y.sin();
        let b = y * 2.761_f32 - x.cos();
        let mut acc = Accumulator1041::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.766_f32 + y.sin();
        let b = y * 6.532_f32 - x.cos();
        let mut acc = Accumulator1041::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.21_f32 + y.sin();
        let b = y * 5.935_f32 - x.cos();
        let mut acc = Accumulator1041::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.836_f32 + y.sin();
        let b = y * 3.566_f32 - x.cos();
        let mut acc = Accumulator1041::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.246_f32 + y.sin();
        let b = y * 3.855_f32 - x.cos();
        let mut acc = Accumulator1041::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.897_f32 + y.sin();
        let b = y * 2.596_f32 - x.cos();
        let mut acc = Accumulator1041::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.719_f32 + y.sin();
        let b = y * 6.884_f32 - x.cos();
        let mut acc = Accumulator1041::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.998_f32 + y.sin();
        let b = y * 1.825_f32 - x.cos();
        let mut acc = Accumulator1041::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.466_f32 + y.sin();
        let b = y * 2.919_f32 - x.cos();
        let mut acc = Accumulator1041::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.093_f32 + y.sin();
        let b = y * 7.968_f32 - x.cos();
        let mut acc = Accumulator1041::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.107_f32 + y.sin();
        let b = y * 0.968_f32 - x.cos();
        let mut acc = Accumulator1041::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.208_f32 + y.sin();
        let b = y * 7.915_f32 - x.cos();
        let mut acc = Accumulator1041::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.678_f32 + y.sin();
        let b = y * 3.425_f32 - x.cos();
        let mut acc = Accumulator1041::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1041(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(1041u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_1041() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1041(total as u64) % 997) as f32;
        total
    }
}

pub mod m1042 {
    use super::*;

    pub struct Accumulator1042<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1042<T> {
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
        let b = y * 7.074_f32 - x.cos();
        let mut acc = Accumulator1042::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.872_f32 + y.sin();
        let b = y * 0.719_f32 - x.cos();
        let mut acc = Accumulator1042::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.438_f32 + y.sin();
        let b = y * 1.15_f32 - x.cos();
        let mut acc = Accumulator1042::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.65_f32 + y.sin();
        let b = y * 4.139_f32 - x.cos();
        let mut acc = Accumulator1042::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.076_f32 + y.sin();
        let b = y * 6.191_f32 - x.cos();
        let mut acc = Accumulator1042::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.189_f32 + y.sin();
        let b = y * 1.375_f32 - x.cos();
        let mut acc = Accumulator1042::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.279_f32 + y.sin();
        let b = y * 9.251_f32 - x.cos();
        let mut acc = Accumulator1042::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.518_f32 + y.sin();
        let b = y * 3.077_f32 - x.cos();
        let mut acc = Accumulator1042::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.154_f32 + y.sin();
        let b = y * 4.374_f32 - x.cos();
        let mut acc = Accumulator1042::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.777_f32 + y.sin();
        let b = y * 1.19_f32 - x.cos();
        let mut acc = Accumulator1042::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.574_f32 + y.sin();
        let b = y * 9.394_f32 - x.cos();
        let mut acc = Accumulator1042::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.002_f32 + y.sin();
        let b = y * 5.211_f32 - x.cos();
        let mut acc = Accumulator1042::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.327_f32 + y.sin();
        let b = y * 2.505_f32 - x.cos();
        let mut acc = Accumulator1042::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.295_f32 + y.sin();
        let b = y * 2.308_f32 - x.cos();
        let mut acc = Accumulator1042::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.271_f32 + y.sin();
        let b = y * 4.187_f32 - x.cos();
        let mut acc = Accumulator1042::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.708_f32 + y.sin();
        let b = y * 1.946_f32 - x.cos();
        let mut acc = Accumulator1042::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.356_f32 + y.sin();
        let b = y * 2.514_f32 - x.cos();
        let mut acc = Accumulator1042::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.859_f32 + y.sin();
        let b = y * 2.143_f32 - x.cos();
        let mut acc = Accumulator1042::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.201_f32 + y.sin();
        let b = y * 4.364_f32 - x.cos();
        let mut acc = Accumulator1042::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.774_f32 + y.sin();
        let b = y * 2.98_f32 - x.cos();
        let mut acc = Accumulator1042::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1042(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_1042() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1042(total as u64) % 997) as f32;
        total
    }
}

pub mod m1043 {
    use super::*;

    pub struct Accumulator1043<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1043<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.06_f32 + y.sin();
        let b = y * 3.971_f32 - x.cos();
        let mut acc = Accumulator1043::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.416_f32 + y.sin();
        let b = y * 9.492_f32 - x.cos();
        let mut acc = Accumulator1043::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.862_f32 + y.sin();
        let b = y * 4.988_f32 - x.cos();
        let mut acc = Accumulator1043::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.771_f32 + y.sin();
        let b = y * 0.275_f32 - x.cos();
        let mut acc = Accumulator1043::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.566_f32 + y.sin();
        let b = y * 4.213_f32 - x.cos();
        let mut acc = Accumulator1043::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.739_f32 + y.sin();
        let b = y * 2.65_f32 - x.cos();
        let mut acc = Accumulator1043::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.997_f32 + y.sin();
        let b = y * 6.669_f32 - x.cos();
        let mut acc = Accumulator1043::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.295_f32 + y.sin();
        let b = y * 2.828_f32 - x.cos();
        let mut acc = Accumulator1043::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.552_f32 + y.sin();
        let b = y * 5.792_f32 - x.cos();
        let mut acc = Accumulator1043::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.715_f32 + y.sin();
        let b = y * 9.533_f32 - x.cos();
        let mut acc = Accumulator1043::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.02_f32 + y.sin();
        let b = y * 9.406_f32 - x.cos();
        let mut acc = Accumulator1043::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.806_f32 + y.sin();
        let b = y * 0.956_f32 - x.cos();
        let mut acc = Accumulator1043::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.021_f32 + y.sin();
        let b = y * 2.292_f32 - x.cos();
        let mut acc = Accumulator1043::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.947_f32 + y.sin();
        let b = y * 1.214_f32 - x.cos();
        let mut acc = Accumulator1043::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.246_f32 + y.sin();
        let b = y * 4.568_f32 - x.cos();
        let mut acc = Accumulator1043::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.68_f32 + y.sin();
        let b = y * 8.433_f32 - x.cos();
        let mut acc = Accumulator1043::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.469_f32 + y.sin();
        let b = y * 2.205_f32 - x.cos();
        let mut acc = Accumulator1043::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.566_f32 + y.sin();
        let b = y * 8.961_f32 - x.cos();
        let mut acc = Accumulator1043::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.642_f32 + y.sin();
        let b = y * 0.475_f32 - x.cos();
        let mut acc = Accumulator1043::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.228_f32 + y.sin();
        let b = y * 2.118_f32 - x.cos();
        let mut acc = Accumulator1043::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1043(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_1043() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1043(total as u64) % 997) as f32;
        total
    }
}

pub mod m1044 {
    use super::*;

    pub struct Accumulator1044<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1044<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.252_f32 + y.sin();
        let b = y * 2.826_f32 - x.cos();
        let mut acc = Accumulator1044::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.507_f32 + y.sin();
        let b = y * 3.392_f32 - x.cos();
        let mut acc = Accumulator1044::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.505_f32 + y.sin();
        let b = y * 8.558_f32 - x.cos();
        let mut acc = Accumulator1044::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.643_f32 + y.sin();
        let b = y * 4.009_f32 - x.cos();
        let mut acc = Accumulator1044::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.242_f32 + y.sin();
        let b = y * 9.41_f32 - x.cos();
        let mut acc = Accumulator1044::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.802_f32 + y.sin();
        let b = y * 8.396_f32 - x.cos();
        let mut acc = Accumulator1044::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.893_f32 + y.sin();
        let b = y * 3.06_f32 - x.cos();
        let mut acc = Accumulator1044::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.179_f32 + y.sin();
        let b = y * 1.452_f32 - x.cos();
        let mut acc = Accumulator1044::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.158_f32 + y.sin();
        let b = y * 1.334_f32 - x.cos();
        let mut acc = Accumulator1044::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.204_f32 + y.sin();
        let b = y * 2.116_f32 - x.cos();
        let mut acc = Accumulator1044::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.647_f32 + y.sin();
        let b = y * 5.417_f32 - x.cos();
        let mut acc = Accumulator1044::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.996_f32 + y.sin();
        let b = y * 5.68_f32 - x.cos();
        let mut acc = Accumulator1044::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.204_f32 + y.sin();
        let b = y * 6.67_f32 - x.cos();
        let mut acc = Accumulator1044::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.539_f32 + y.sin();
        let b = y * 5.854_f32 - x.cos();
        let mut acc = Accumulator1044::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.986_f32 + y.sin();
        let b = y * 1.148_f32 - x.cos();
        let mut acc = Accumulator1044::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.122_f32 + y.sin();
        let b = y * 3.668_f32 - x.cos();
        let mut acc = Accumulator1044::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.908_f32 + y.sin();
        let b = y * 2.654_f32 - x.cos();
        let mut acc = Accumulator1044::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.887_f32 + y.sin();
        let b = y * 8.402_f32 - x.cos();
        let mut acc = Accumulator1044::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.381_f32 + y.sin();
        let b = y * 9.733_f32 - x.cos();
        let mut acc = Accumulator1044::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.678_f32 + y.sin();
        let b = y * 0.278_f32 - x.cos();
        let mut acc = Accumulator1044::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1044(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m1044-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_1044() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1044(total as u64) % 997) as f32;
        total
    }
}

pub mod m1045 {
    use super::*;

    pub struct Accumulator1045<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1045<T> {
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
        let b = y * 6.004_f32 - x.cos();
        let mut acc = Accumulator1045::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.335_f32 + y.sin();
        let b = y * 4.809_f32 - x.cos();
        let mut acc = Accumulator1045::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.986_f32 + y.sin();
        let b = y * 8.557_f32 - x.cos();
        let mut acc = Accumulator1045::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.547_f32 + y.sin();
        let b = y * 6.854_f32 - x.cos();
        let mut acc = Accumulator1045::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.072_f32 + y.sin();
        let b = y * 5.914_f32 - x.cos();
        let mut acc = Accumulator1045::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.073_f32 + y.sin();
        let b = y * 8.98_f32 - x.cos();
        let mut acc = Accumulator1045::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.116_f32 + y.sin();
        let b = y * 6.427_f32 - x.cos();
        let mut acc = Accumulator1045::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.905_f32 + y.sin();
        let b = y * 5.321_f32 - x.cos();
        let mut acc = Accumulator1045::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.798_f32 + y.sin();
        let b = y * 4.467_f32 - x.cos();
        let mut acc = Accumulator1045::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.629_f32 + y.sin();
        let b = y * 2.332_f32 - x.cos();
        let mut acc = Accumulator1045::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.191_f32 + y.sin();
        let b = y * 4.73_f32 - x.cos();
        let mut acc = Accumulator1045::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.548_f32 + y.sin();
        let b = y * 8.407_f32 - x.cos();
        let mut acc = Accumulator1045::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.027_f32 + y.sin();
        let b = y * 3.701_f32 - x.cos();
        let mut acc = Accumulator1045::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.341_f32 + y.sin();
        let b = y * 3.196_f32 - x.cos();
        let mut acc = Accumulator1045::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.749_f32 + y.sin();
        let b = y * 9.665_f32 - x.cos();
        let mut acc = Accumulator1045::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.557_f32 + y.sin();
        let b = y * 7.087_f32 - x.cos();
        let mut acc = Accumulator1045::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.436_f32 + y.sin();
        let b = y * 5.565_f32 - x.cos();
        let mut acc = Accumulator1045::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.325_f32 + y.sin();
        let b = y * 3.871_f32 - x.cos();
        let mut acc = Accumulator1045::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.482_f32 + y.sin();
        let b = y * 2.515_f32 - x.cos();
        let mut acc = Accumulator1045::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.95_f32 + y.sin();
        let b = y * 6.158_f32 - x.cos();
        let mut acc = Accumulator1045::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1045(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_1045() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1045(total as u64) % 997) as f32;
        total
    }
}

pub mod m1046 {
    use super::*;

    pub struct Accumulator1046<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1046<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.867_f32 + y.sin();
        let b = y * 6.243_f32 - x.cos();
        let mut acc = Accumulator1046::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.1_f32 + y.sin();
        let b = y * 1.498_f32 - x.cos();
        let mut acc = Accumulator1046::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.928_f32 + y.sin();
        let b = y * 5.899_f32 - x.cos();
        let mut acc = Accumulator1046::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.393_f32 + y.sin();
        let b = y * 2.072_f32 - x.cos();
        let mut acc = Accumulator1046::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.682_f32 + y.sin();
        let b = y * 8.391_f32 - x.cos();
        let mut acc = Accumulator1046::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.777_f32 + y.sin();
        let b = y * 5.72_f32 - x.cos();
        let mut acc = Accumulator1046::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.679_f32 + y.sin();
        let b = y * 9.076_f32 - x.cos();
        let mut acc = Accumulator1046::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.389_f32 + y.sin();
        let b = y * 4.859_f32 - x.cos();
        let mut acc = Accumulator1046::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.177_f32 + y.sin();
        let b = y * 3.196_f32 - x.cos();
        let mut acc = Accumulator1046::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.321_f32 + y.sin();
        let b = y * 2.191_f32 - x.cos();
        let mut acc = Accumulator1046::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.427_f32 + y.sin();
        let b = y * 8.379_f32 - x.cos();
        let mut acc = Accumulator1046::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.334_f32 + y.sin();
        let b = y * 3.717_f32 - x.cos();
        let mut acc = Accumulator1046::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.916_f32 + y.sin();
        let b = y * 3.516_f32 - x.cos();
        let mut acc = Accumulator1046::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.473_f32 + y.sin();
        let b = y * 5.044_f32 - x.cos();
        let mut acc = Accumulator1046::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.052_f32 + y.sin();
        let b = y * 1.704_f32 - x.cos();
        let mut acc = Accumulator1046::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.337_f32 + y.sin();
        let b = y * 6.834_f32 - x.cos();
        let mut acc = Accumulator1046::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.992_f32 + y.sin();
        let b = y * 3.691_f32 - x.cos();
        let mut acc = Accumulator1046::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.791_f32 + y.sin();
        let b = y * 6.328_f32 - x.cos();
        let mut acc = Accumulator1046::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.009_f32 + y.sin();
        let b = y * 5.674_f32 - x.cos();
        let mut acc = Accumulator1046::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.673_f32 + y.sin();
        let b = y * 0.874_f32 - x.cos();
        let mut acc = Accumulator1046::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1046(seed: u64) -> u64 {
        let re = Regex::new(r"m1046-(\d+)").unwrap();
        let hay = format!("m1046-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_1046() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1046(total as u64) % 997) as f32;
        total
    }
}

pub mod m1047 {
    use super::*;

    pub struct Accumulator1047<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1047<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.956_f32 + y.sin();
        let b = y * 4.693_f32 - x.cos();
        let mut acc = Accumulator1047::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.988_f32 + y.sin();
        let b = y * 7.338_f32 - x.cos();
        let mut acc = Accumulator1047::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.453_f32 + y.sin();
        let b = y * 7.301_f32 - x.cos();
        let mut acc = Accumulator1047::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.219_f32 + y.sin();
        let b = y * 2.532_f32 - x.cos();
        let mut acc = Accumulator1047::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.777_f32 + y.sin();
        let b = y * 1.553_f32 - x.cos();
        let mut acc = Accumulator1047::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.046_f32 + y.sin();
        let b = y * 9.092_f32 - x.cos();
        let mut acc = Accumulator1047::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.73_f32 + y.sin();
        let b = y * 5.155_f32 - x.cos();
        let mut acc = Accumulator1047::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.189_f32 + y.sin();
        let b = y * 1.254_f32 - x.cos();
        let mut acc = Accumulator1047::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.48_f32 + y.sin();
        let b = y * 9.369_f32 - x.cos();
        let mut acc = Accumulator1047::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.247_f32 + y.sin();
        let b = y * 0.332_f32 - x.cos();
        let mut acc = Accumulator1047::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.616_f32 + y.sin();
        let b = y * 8.684_f32 - x.cos();
        let mut acc = Accumulator1047::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.61_f32 + y.sin();
        let b = y * 5.088_f32 - x.cos();
        let mut acc = Accumulator1047::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.39_f32 + y.sin();
        let b = y * 5.453_f32 - x.cos();
        let mut acc = Accumulator1047::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.284_f32 + y.sin();
        let b = y * 6.262_f32 - x.cos();
        let mut acc = Accumulator1047::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.607_f32 + y.sin();
        let b = y * 6.352_f32 - x.cos();
        let mut acc = Accumulator1047::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.673_f32 + y.sin();
        let b = y * 8.274_f32 - x.cos();
        let mut acc = Accumulator1047::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.568_f32 + y.sin();
        let b = y * 8.458_f32 - x.cos();
        let mut acc = Accumulator1047::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.349_f32 + y.sin();
        let b = y * 1.894_f32 - x.cos();
        let mut acc = Accumulator1047::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.806_f32 + y.sin();
        let b = y * 1.733_f32 - x.cos();
        let mut acc = Accumulator1047::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.748_f32 + y.sin();
        let b = y * 3.711_f32 - x.cos();
        let mut acc = Accumulator1047::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1047(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_1047() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1047(total as u64) % 997) as f32;
        total
    }
}

pub mod m1048 {
    use super::*;

    pub struct Accumulator1048<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1048<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.322_f32 + y.sin();
        let b = y * 0.814_f32 - x.cos();
        let mut acc = Accumulator1048::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.787_f32 + y.sin();
        let b = y * 2.806_f32 - x.cos();
        let mut acc = Accumulator1048::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.579_f32 + y.sin();
        let b = y * 7.007_f32 - x.cos();
        let mut acc = Accumulator1048::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.439_f32 + y.sin();
        let b = y * 9.201_f32 - x.cos();
        let mut acc = Accumulator1048::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.073_f32 + y.sin();
        let b = y * 8.943_f32 - x.cos();
        let mut acc = Accumulator1048::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.229_f32 + y.sin();
        let b = y * 3.831_f32 - x.cos();
        let mut acc = Accumulator1048::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.172_f32 + y.sin();
        let b = y * 2.957_f32 - x.cos();
        let mut acc = Accumulator1048::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.603_f32 + y.sin();
        let b = y * 6.356_f32 - x.cos();
        let mut acc = Accumulator1048::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.093_f32 + y.sin();
        let b = y * 3.317_f32 - x.cos();
        let mut acc = Accumulator1048::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.372_f32 + y.sin();
        let b = y * 0.838_f32 - x.cos();
        let mut acc = Accumulator1048::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.568_f32 + y.sin();
        let b = y * 7.602_f32 - x.cos();
        let mut acc = Accumulator1048::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.834_f32 + y.sin();
        let b = y * 2.787_f32 - x.cos();
        let mut acc = Accumulator1048::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.576_f32 + y.sin();
        let b = y * 0.916_f32 - x.cos();
        let mut acc = Accumulator1048::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.678_f32 + y.sin();
        let b = y * 2.639_f32 - x.cos();
        let mut acc = Accumulator1048::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.362_f32 + y.sin();
        let b = y * 0.11_f32 - x.cos();
        let mut acc = Accumulator1048::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.089_f32 + y.sin();
        let b = y * 7.978_f32 - x.cos();
        let mut acc = Accumulator1048::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.5_f32 + y.sin();
        let b = y * 6.232_f32 - x.cos();
        let mut acc = Accumulator1048::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.988_f32 + y.sin();
        let b = y * 0.422_f32 - x.cos();
        let mut acc = Accumulator1048::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.801_f32 + y.sin();
        let b = y * 5.501_f32 - x.cos();
        let mut acc = Accumulator1048::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.676_f32 + y.sin();
        let b = y * 6.906_f32 - x.cos();
        let mut acc = Accumulator1048::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1048(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(1048u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_1048() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1048(total as u64) % 997) as f32;
        total
    }
}

pub mod m1049 {
    use super::*;

    pub struct Accumulator1049<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1049<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.4_f32 + y.sin();
        let b = y * 4.719_f32 - x.cos();
        let mut acc = Accumulator1049::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.184_f32 + y.sin();
        let b = y * 6.833_f32 - x.cos();
        let mut acc = Accumulator1049::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.765_f32 + y.sin();
        let b = y * 2.357_f32 - x.cos();
        let mut acc = Accumulator1049::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.369_f32 + y.sin();
        let b = y * 2.09_f32 - x.cos();
        let mut acc = Accumulator1049::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.17_f32 + y.sin();
        let b = y * 4.217_f32 - x.cos();
        let mut acc = Accumulator1049::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.243_f32 + y.sin();
        let b = y * 9.128_f32 - x.cos();
        let mut acc = Accumulator1049::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.879_f32 + y.sin();
        let b = y * 5.47_f32 - x.cos();
        let mut acc = Accumulator1049::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.822_f32 + y.sin();
        let b = y * 1.771_f32 - x.cos();
        let mut acc = Accumulator1049::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.089_f32 + y.sin();
        let b = y * 5.01_f32 - x.cos();
        let mut acc = Accumulator1049::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.845_f32 + y.sin();
        let b = y * 1.407_f32 - x.cos();
        let mut acc = Accumulator1049::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.861_f32 + y.sin();
        let b = y * 9.16_f32 - x.cos();
        let mut acc = Accumulator1049::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.164_f32 + y.sin();
        let b = y * 9.442_f32 - x.cos();
        let mut acc = Accumulator1049::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.667_f32 + y.sin();
        let b = y * 7.385_f32 - x.cos();
        let mut acc = Accumulator1049::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.702_f32 + y.sin();
        let b = y * 5.44_f32 - x.cos();
        let mut acc = Accumulator1049::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.137_f32 + y.sin();
        let b = y * 2.833_f32 - x.cos();
        let mut acc = Accumulator1049::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.828_f32 + y.sin();
        let b = y * 3.821_f32 - x.cos();
        let mut acc = Accumulator1049::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.767_f32 + y.sin();
        let b = y * 2.796_f32 - x.cos();
        let mut acc = Accumulator1049::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.9_f32 + y.sin();
        let b = y * 1.768_f32 - x.cos();
        let mut acc = Accumulator1049::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.787_f32 + y.sin();
        let b = y * 5.348_f32 - x.cos();
        let mut acc = Accumulator1049::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.067_f32 + y.sin();
        let b = y * 9.365_f32 - x.cos();
        let mut acc = Accumulator1049::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1049(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_1049() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1049(total as u64) % 997) as f32;
        total
    }
}

pub mod m1050 {
    use super::*;

    pub struct Accumulator1050<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1050<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.419_f32 + y.sin();
        let b = y * 9.264_f32 - x.cos();
        let mut acc = Accumulator1050::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.027_f32 + y.sin();
        let b = y * 5.431_f32 - x.cos();
        let mut acc = Accumulator1050::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.155_f32 + y.sin();
        let b = y * 1.533_f32 - x.cos();
        let mut acc = Accumulator1050::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.085_f32 + y.sin();
        let b = y * 9.788_f32 - x.cos();
        let mut acc = Accumulator1050::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.886_f32 + y.sin();
        let b = y * 5.003_f32 - x.cos();
        let mut acc = Accumulator1050::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.637_f32 + y.sin();
        let b = y * 4.179_f32 - x.cos();
        let mut acc = Accumulator1050::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.99_f32 + y.sin();
        let b = y * 4.539_f32 - x.cos();
        let mut acc = Accumulator1050::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.37_f32 + y.sin();
        let b = y * 4.997_f32 - x.cos();
        let mut acc = Accumulator1050::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.376_f32 + y.sin();
        let b = y * 5.148_f32 - x.cos();
        let mut acc = Accumulator1050::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.144_f32 + y.sin();
        let b = y * 8.737_f32 - x.cos();
        let mut acc = Accumulator1050::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.404_f32 + y.sin();
        let b = y * 7.195_f32 - x.cos();
        let mut acc = Accumulator1050::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.014_f32 + y.sin();
        let b = y * 1.755_f32 - x.cos();
        let mut acc = Accumulator1050::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.012_f32 + y.sin();
        let b = y * 1.163_f32 - x.cos();
        let mut acc = Accumulator1050::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.825_f32 + y.sin();
        let b = y * 7.956_f32 - x.cos();
        let mut acc = Accumulator1050::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.442_f32 + y.sin();
        let b = y * 9.701_f32 - x.cos();
        let mut acc = Accumulator1050::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.251_f32 + y.sin();
        let b = y * 9.783_f32 - x.cos();
        let mut acc = Accumulator1050::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.919_f32 + y.sin();
        let b = y * 6.847_f32 - x.cos();
        let mut acc = Accumulator1050::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.685_f32 + y.sin();
        let b = y * 7.749_f32 - x.cos();
        let mut acc = Accumulator1050::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.152_f32 + y.sin();
        let b = y * 1.026_f32 - x.cos();
        let mut acc = Accumulator1050::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.836_f32 + y.sin();
        let b = y * 9.441_f32 - x.cos();
        let mut acc = Accumulator1050::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1050(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_1050() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1050(total as u64) % 997) as f32;
        total
    }
}

pub mod m1051 {
    use super::*;

    pub struct Accumulator1051<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1051<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.514_f32 + y.sin();
        let b = y * 7.925_f32 - x.cos();
        let mut acc = Accumulator1051::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.342_f32 + y.sin();
        let b = y * 9.79_f32 - x.cos();
        let mut acc = Accumulator1051::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.714_f32 + y.sin();
        let b = y * 5.967_f32 - x.cos();
        let mut acc = Accumulator1051::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.759_f32 + y.sin();
        let b = y * 3.536_f32 - x.cos();
        let mut acc = Accumulator1051::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.568_f32 + y.sin();
        let b = y * 0.829_f32 - x.cos();
        let mut acc = Accumulator1051::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.115_f32 + y.sin();
        let b = y * 4.759_f32 - x.cos();
        let mut acc = Accumulator1051::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.913_f32 + y.sin();
        let b = y * 4.561_f32 - x.cos();
        let mut acc = Accumulator1051::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.352_f32 + y.sin();
        let b = y * 1.064_f32 - x.cos();
        let mut acc = Accumulator1051::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.604_f32 + y.sin();
        let b = y * 3.311_f32 - x.cos();
        let mut acc = Accumulator1051::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.873_f32 + y.sin();
        let b = y * 9.566_f32 - x.cos();
        let mut acc = Accumulator1051::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.449_f32 + y.sin();
        let b = y * 2.908_f32 - x.cos();
        let mut acc = Accumulator1051::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.096_f32 + y.sin();
        let b = y * 4.842_f32 - x.cos();
        let mut acc = Accumulator1051::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.907_f32 + y.sin();
        let b = y * 0.564_f32 - x.cos();
        let mut acc = Accumulator1051::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.597_f32 + y.sin();
        let b = y * 7.417_f32 - x.cos();
        let mut acc = Accumulator1051::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.289_f32 + y.sin();
        let b = y * 1.051_f32 - x.cos();
        let mut acc = Accumulator1051::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.991_f32 + y.sin();
        let b = y * 7.486_f32 - x.cos();
        let mut acc = Accumulator1051::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.417_f32 + y.sin();
        let b = y * 0.298_f32 - x.cos();
        let mut acc = Accumulator1051::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.149_f32 + y.sin();
        let b = y * 5.654_f32 - x.cos();
        let mut acc = Accumulator1051::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.414_f32 + y.sin();
        let b = y * 9.786_f32 - x.cos();
        let mut acc = Accumulator1051::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.788_f32 + y.sin();
        let b = y * 4.059_f32 - x.cos();
        let mut acc = Accumulator1051::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1051(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m1051-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_1051() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1051(total as u64) % 997) as f32;
        total
    }
}

pub mod m1052 {
    use super::*;

    pub struct Accumulator1052<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1052<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.019_f32 + y.sin();
        let b = y * 2.056_f32 - x.cos();
        let mut acc = Accumulator1052::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.821_f32 + y.sin();
        let b = y * 8.818_f32 - x.cos();
        let mut acc = Accumulator1052::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.337_f32 + y.sin();
        let b = y * 8.448_f32 - x.cos();
        let mut acc = Accumulator1052::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.375_f32 + y.sin();
        let b = y * 6.791_f32 - x.cos();
        let mut acc = Accumulator1052::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.597_f32 + y.sin();
        let b = y * 9.453_f32 - x.cos();
        let mut acc = Accumulator1052::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.425_f32 + y.sin();
        let b = y * 5.968_f32 - x.cos();
        let mut acc = Accumulator1052::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.898_f32 + y.sin();
        let b = y * 7.039_f32 - x.cos();
        let mut acc = Accumulator1052::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.649_f32 + y.sin();
        let b = y * 3.772_f32 - x.cos();
        let mut acc = Accumulator1052::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.02_f32 + y.sin();
        let b = y * 4.853_f32 - x.cos();
        let mut acc = Accumulator1052::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.708_f32 + y.sin();
        let b = y * 2.062_f32 - x.cos();
        let mut acc = Accumulator1052::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.254_f32 + y.sin();
        let b = y * 6.544_f32 - x.cos();
        let mut acc = Accumulator1052::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.378_f32 + y.sin();
        let b = y * 1.385_f32 - x.cos();
        let mut acc = Accumulator1052::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.4_f32 + y.sin();
        let b = y * 9.495_f32 - x.cos();
        let mut acc = Accumulator1052::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.148_f32 + y.sin();
        let b = y * 7.683_f32 - x.cos();
        let mut acc = Accumulator1052::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.539_f32 + y.sin();
        let b = y * 1.841_f32 - x.cos();
        let mut acc = Accumulator1052::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.472_f32 + y.sin();
        let b = y * 5.471_f32 - x.cos();
        let mut acc = Accumulator1052::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.346_f32 + y.sin();
        let b = y * 9.566_f32 - x.cos();
        let mut acc = Accumulator1052::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.873_f32 + y.sin();
        let b = y * 8.352_f32 - x.cos();
        let mut acc = Accumulator1052::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.549_f32 + y.sin();
        let b = y * 9.29_f32 - x.cos();
        let mut acc = Accumulator1052::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.773_f32 + y.sin();
        let b = y * 7.711_f32 - x.cos();
        let mut acc = Accumulator1052::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1052(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_1052() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1052(total as u64) % 997) as f32;
        total
    }
}

pub mod m1053 {
    use super::*;

    pub struct Accumulator1053<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1053<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.591_f32 + y.sin();
        let b = y * 3.973_f32 - x.cos();
        let mut acc = Accumulator1053::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.229_f32 + y.sin();
        let b = y * 3.816_f32 - x.cos();
        let mut acc = Accumulator1053::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.179_f32 + y.sin();
        let b = y * 3.445_f32 - x.cos();
        let mut acc = Accumulator1053::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.702_f32 + y.sin();
        let b = y * 9.313_f32 - x.cos();
        let mut acc = Accumulator1053::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.047_f32 + y.sin();
        let b = y * 1.198_f32 - x.cos();
        let mut acc = Accumulator1053::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.287_f32 + y.sin();
        let b = y * 2.931_f32 - x.cos();
        let mut acc = Accumulator1053::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.492_f32 + y.sin();
        let b = y * 3.032_f32 - x.cos();
        let mut acc = Accumulator1053::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.398_f32 + y.sin();
        let b = y * 7.75_f32 - x.cos();
        let mut acc = Accumulator1053::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.269_f32 + y.sin();
        let b = y * 6.523_f32 - x.cos();
        let mut acc = Accumulator1053::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.55_f32 + y.sin();
        let b = y * 2.33_f32 - x.cos();
        let mut acc = Accumulator1053::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.495_f32 + y.sin();
        let b = y * 2.458_f32 - x.cos();
        let mut acc = Accumulator1053::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.766_f32 + y.sin();
        let b = y * 0.435_f32 - x.cos();
        let mut acc = Accumulator1053::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.301_f32 + y.sin();
        let b = y * 5.707_f32 - x.cos();
        let mut acc = Accumulator1053::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.07_f32 + y.sin();
        let b = y * 8.372_f32 - x.cos();
        let mut acc = Accumulator1053::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.922_f32 + y.sin();
        let b = y * 7.803_f32 - x.cos();
        let mut acc = Accumulator1053::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.517_f32 + y.sin();
        let b = y * 7.284_f32 - x.cos();
        let mut acc = Accumulator1053::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.73_f32 + y.sin();
        let b = y * 0.108_f32 - x.cos();
        let mut acc = Accumulator1053::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.636_f32 + y.sin();
        let b = y * 8.372_f32 - x.cos();
        let mut acc = Accumulator1053::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.684_f32 + y.sin();
        let b = y * 8.609_f32 - x.cos();
        let mut acc = Accumulator1053::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.591_f32 + y.sin();
        let b = y * 6.213_f32 - x.cos();
        let mut acc = Accumulator1053::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1053(seed: u64) -> u64 {
        let re = Regex::new(r"m1053-(\d+)").unwrap();
        let hay = format!("m1053-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_1053() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1053(total as u64) % 997) as f32;
        total
    }
}

pub mod m1054 {
    use super::*;

    pub struct Accumulator1054<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1054<T> {
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
        let b = y * 9.024_f32 - x.cos();
        let mut acc = Accumulator1054::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.816_f32 + y.sin();
        let b = y * 8.805_f32 - x.cos();
        let mut acc = Accumulator1054::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.181_f32 + y.sin();
        let b = y * 5.533_f32 - x.cos();
        let mut acc = Accumulator1054::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.856_f32 + y.sin();
        let b = y * 1.892_f32 - x.cos();
        let mut acc = Accumulator1054::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.732_f32 + y.sin();
        let b = y * 0.25_f32 - x.cos();
        let mut acc = Accumulator1054::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.176_f32 + y.sin();
        let b = y * 9.645_f32 - x.cos();
        let mut acc = Accumulator1054::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.556_f32 + y.sin();
        let b = y * 3.323_f32 - x.cos();
        let mut acc = Accumulator1054::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.95_f32 + y.sin();
        let b = y * 9.646_f32 - x.cos();
        let mut acc = Accumulator1054::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.08_f32 + y.sin();
        let b = y * 7.913_f32 - x.cos();
        let mut acc = Accumulator1054::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.545_f32 + y.sin();
        let b = y * 0.845_f32 - x.cos();
        let mut acc = Accumulator1054::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.394_f32 + y.sin();
        let b = y * 1.301_f32 - x.cos();
        let mut acc = Accumulator1054::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.731_f32 + y.sin();
        let b = y * 9.167_f32 - x.cos();
        let mut acc = Accumulator1054::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.209_f32 + y.sin();
        let b = y * 3.811_f32 - x.cos();
        let mut acc = Accumulator1054::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.995_f32 + y.sin();
        let b = y * 5.226_f32 - x.cos();
        let mut acc = Accumulator1054::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.192_f32 + y.sin();
        let b = y * 8.345_f32 - x.cos();
        let mut acc = Accumulator1054::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.607_f32 + y.sin();
        let b = y * 3.777_f32 - x.cos();
        let mut acc = Accumulator1054::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.19_f32 + y.sin();
        let b = y * 0.764_f32 - x.cos();
        let mut acc = Accumulator1054::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.458_f32 + y.sin();
        let b = y * 7.369_f32 - x.cos();
        let mut acc = Accumulator1054::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.302_f32 + y.sin();
        let b = y * 0.126_f32 - x.cos();
        let mut acc = Accumulator1054::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.445_f32 + y.sin();
        let b = y * 6.943_f32 - x.cos();
        let mut acc = Accumulator1054::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1054(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_1054() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1054(total as u64) % 997) as f32;
        total
    }
}

pub mod m1055 {
    use super::*;

    pub struct Accumulator1055<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1055<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.193_f32 + y.sin();
        let b = y * 5.435_f32 - x.cos();
        let mut acc = Accumulator1055::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.6_f32 + y.sin();
        let b = y * 1.293_f32 - x.cos();
        let mut acc = Accumulator1055::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.582_f32 + y.sin();
        let b = y * 0.115_f32 - x.cos();
        let mut acc = Accumulator1055::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.606_f32 + y.sin();
        let b = y * 5.514_f32 - x.cos();
        let mut acc = Accumulator1055::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.772_f32 + y.sin();
        let b = y * 3.554_f32 - x.cos();
        let mut acc = Accumulator1055::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.12_f32 + y.sin();
        let b = y * 8.685_f32 - x.cos();
        let mut acc = Accumulator1055::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.027_f32 + y.sin();
        let b = y * 3.686_f32 - x.cos();
        let mut acc = Accumulator1055::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.514_f32 + y.sin();
        let b = y * 9.863_f32 - x.cos();
        let mut acc = Accumulator1055::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.489_f32 + y.sin();
        let b = y * 3.821_f32 - x.cos();
        let mut acc = Accumulator1055::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.298_f32 + y.sin();
        let b = y * 1.142_f32 - x.cos();
        let mut acc = Accumulator1055::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.052_f32 + y.sin();
        let b = y * 7.102_f32 - x.cos();
        let mut acc = Accumulator1055::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.244_f32 + y.sin();
        let b = y * 8.145_f32 - x.cos();
        let mut acc = Accumulator1055::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.24_f32 + y.sin();
        let b = y * 0.517_f32 - x.cos();
        let mut acc = Accumulator1055::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.809_f32 + y.sin();
        let b = y * 2.458_f32 - x.cos();
        let mut acc = Accumulator1055::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.102_f32 + y.sin();
        let b = y * 1.356_f32 - x.cos();
        let mut acc = Accumulator1055::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.999_f32 + y.sin();
        let b = y * 2.159_f32 - x.cos();
        let mut acc = Accumulator1055::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.053_f32 + y.sin();
        let b = y * 5.392_f32 - x.cos();
        let mut acc = Accumulator1055::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.066_f32 + y.sin();
        let b = y * 6.827_f32 - x.cos();
        let mut acc = Accumulator1055::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.972_f32 + y.sin();
        let b = y * 6.3_f32 - x.cos();
        let mut acc = Accumulator1055::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.094_f32 + y.sin();
        let b = y * 7.946_f32 - x.cos();
        let mut acc = Accumulator1055::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1055(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(1055u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_1055() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1055(total as u64) % 997) as f32;
        total
    }
}

pub mod m1056 {
    use super::*;

    pub struct Accumulator1056<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1056<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.542_f32 + y.sin();
        let b = y * 0.184_f32 - x.cos();
        let mut acc = Accumulator1056::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.835_f32 + y.sin();
        let b = y * 9.153_f32 - x.cos();
        let mut acc = Accumulator1056::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.41_f32 + y.sin();
        let b = y * 5.501_f32 - x.cos();
        let mut acc = Accumulator1056::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.711_f32 + y.sin();
        let b = y * 5.918_f32 - x.cos();
        let mut acc = Accumulator1056::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.726_f32 + y.sin();
        let b = y * 2.182_f32 - x.cos();
        let mut acc = Accumulator1056::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.181_f32 + y.sin();
        let b = y * 6.328_f32 - x.cos();
        let mut acc = Accumulator1056::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.622_f32 + y.sin();
        let b = y * 0.928_f32 - x.cos();
        let mut acc = Accumulator1056::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.484_f32 + y.sin();
        let b = y * 1.164_f32 - x.cos();
        let mut acc = Accumulator1056::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.218_f32 + y.sin();
        let b = y * 7.488_f32 - x.cos();
        let mut acc = Accumulator1056::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.269_f32 + y.sin();
        let b = y * 0.837_f32 - x.cos();
        let mut acc = Accumulator1056::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.461_f32 + y.sin();
        let b = y * 2.227_f32 - x.cos();
        let mut acc = Accumulator1056::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.788_f32 + y.sin();
        let b = y * 8.164_f32 - x.cos();
        let mut acc = Accumulator1056::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.097_f32 + y.sin();
        let b = y * 4.114_f32 - x.cos();
        let mut acc = Accumulator1056::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.402_f32 + y.sin();
        let b = y * 4.65_f32 - x.cos();
        let mut acc = Accumulator1056::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.322_f32 + y.sin();
        let b = y * 4.601_f32 - x.cos();
        let mut acc = Accumulator1056::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.735_f32 + y.sin();
        let b = y * 4.87_f32 - x.cos();
        let mut acc = Accumulator1056::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.516_f32 + y.sin();
        let b = y * 2.47_f32 - x.cos();
        let mut acc = Accumulator1056::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.358_f32 + y.sin();
        let b = y * 8.724_f32 - x.cos();
        let mut acc = Accumulator1056::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.711_f32 + y.sin();
        let b = y * 3.027_f32 - x.cos();
        let mut acc = Accumulator1056::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.409_f32 + y.sin();
        let b = y * 7.005_f32 - x.cos();
        let mut acc = Accumulator1056::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1056(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_1056() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1056(total as u64) % 997) as f32;
        total
    }
}

pub mod m1057 {
    use super::*;

    pub struct Accumulator1057<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1057<T> {
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
        let b = y * 2.793_f32 - x.cos();
        let mut acc = Accumulator1057::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.627_f32 + y.sin();
        let b = y * 5.631_f32 - x.cos();
        let mut acc = Accumulator1057::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.072_f32 + y.sin();
        let b = y * 1.044_f32 - x.cos();
        let mut acc = Accumulator1057::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.353_f32 + y.sin();
        let b = y * 3.797_f32 - x.cos();
        let mut acc = Accumulator1057::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.04_f32 + y.sin();
        let b = y * 9.309_f32 - x.cos();
        let mut acc = Accumulator1057::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.96_f32 + y.sin();
        let b = y * 0.69_f32 - x.cos();
        let mut acc = Accumulator1057::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.57_f32 + y.sin();
        let b = y * 8.684_f32 - x.cos();
        let mut acc = Accumulator1057::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.793_f32 + y.sin();
        let b = y * 8.51_f32 - x.cos();
        let mut acc = Accumulator1057::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.535_f32 + y.sin();
        let b = y * 2.84_f32 - x.cos();
        let mut acc = Accumulator1057::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.1_f32 + y.sin();
        let b = y * 8.235_f32 - x.cos();
        let mut acc = Accumulator1057::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.092_f32 + y.sin();
        let b = y * 1.719_f32 - x.cos();
        let mut acc = Accumulator1057::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.939_f32 + y.sin();
        let b = y * 5.73_f32 - x.cos();
        let mut acc = Accumulator1057::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.225_f32 + y.sin();
        let b = y * 9.512_f32 - x.cos();
        let mut acc = Accumulator1057::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.794_f32 + y.sin();
        let b = y * 0.335_f32 - x.cos();
        let mut acc = Accumulator1057::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.721_f32 + y.sin();
        let b = y * 8.207_f32 - x.cos();
        let mut acc = Accumulator1057::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.408_f32 + y.sin();
        let b = y * 7.832_f32 - x.cos();
        let mut acc = Accumulator1057::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.216_f32 + y.sin();
        let b = y * 1.36_f32 - x.cos();
        let mut acc = Accumulator1057::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.83_f32 + y.sin();
        let b = y * 1.056_f32 - x.cos();
        let mut acc = Accumulator1057::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.243_f32 + y.sin();
        let b = y * 2.816_f32 - x.cos();
        let mut acc = Accumulator1057::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.919_f32 + y.sin();
        let b = y * 1.87_f32 - x.cos();
        let mut acc = Accumulator1057::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1057(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_1057() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1057(total as u64) % 997) as f32;
        total
    }
}

pub mod m1058 {
    use super::*;

    pub struct Accumulator1058<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1058<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.425_f32 + y.sin();
        let b = y * 1.165_f32 - x.cos();
        let mut acc = Accumulator1058::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.442_f32 + y.sin();
        let b = y * 7.229_f32 - x.cos();
        let mut acc = Accumulator1058::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.851_f32 + y.sin();
        let b = y * 0.153_f32 - x.cos();
        let mut acc = Accumulator1058::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.411_f32 + y.sin();
        let b = y * 7.807_f32 - x.cos();
        let mut acc = Accumulator1058::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.499_f32 + y.sin();
        let b = y * 2.692_f32 - x.cos();
        let mut acc = Accumulator1058::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.089_f32 + y.sin();
        let b = y * 4.749_f32 - x.cos();
        let mut acc = Accumulator1058::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.665_f32 + y.sin();
        let b = y * 1.981_f32 - x.cos();
        let mut acc = Accumulator1058::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.741_f32 + y.sin();
        let b = y * 1.747_f32 - x.cos();
        let mut acc = Accumulator1058::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.303_f32 + y.sin();
        let b = y * 0.743_f32 - x.cos();
        let mut acc = Accumulator1058::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.636_f32 + y.sin();
        let b = y * 5.178_f32 - x.cos();
        let mut acc = Accumulator1058::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.036_f32 + y.sin();
        let b = y * 8.821_f32 - x.cos();
        let mut acc = Accumulator1058::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.055_f32 + y.sin();
        let b = y * 9.493_f32 - x.cos();
        let mut acc = Accumulator1058::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.544_f32 + y.sin();
        let b = y * 8.369_f32 - x.cos();
        let mut acc = Accumulator1058::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.803_f32 + y.sin();
        let b = y * 5.496_f32 - x.cos();
        let mut acc = Accumulator1058::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.617_f32 + y.sin();
        let b = y * 7.152_f32 - x.cos();
        let mut acc = Accumulator1058::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.481_f32 + y.sin();
        let b = y * 4.169_f32 - x.cos();
        let mut acc = Accumulator1058::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.465_f32 + y.sin();
        let b = y * 3.846_f32 - x.cos();
        let mut acc = Accumulator1058::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.248_f32 + y.sin();
        let b = y * 3.535_f32 - x.cos();
        let mut acc = Accumulator1058::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.465_f32 + y.sin();
        let b = y * 2.469_f32 - x.cos();
        let mut acc = Accumulator1058::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.28_f32 + y.sin();
        let b = y * 4.64_f32 - x.cos();
        let mut acc = Accumulator1058::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1058(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m1058-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_1058() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1058(total as u64) % 997) as f32;
        total
    }
}

pub mod m1059 {
    use super::*;

    pub struct Accumulator1059<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1059<T> {
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
        let b = y * 1.08_f32 - x.cos();
        let mut acc = Accumulator1059::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.455_f32 + y.sin();
        let b = y * 5.908_f32 - x.cos();
        let mut acc = Accumulator1059::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.912_f32 + y.sin();
        let b = y * 8.88_f32 - x.cos();
        let mut acc = Accumulator1059::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.2_f32 + y.sin();
        let b = y * 5.134_f32 - x.cos();
        let mut acc = Accumulator1059::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.355_f32 + y.sin();
        let b = y * 6.589_f32 - x.cos();
        let mut acc = Accumulator1059::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.497_f32 + y.sin();
        let b = y * 2.173_f32 - x.cos();
        let mut acc = Accumulator1059::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.179_f32 + y.sin();
        let b = y * 4.027_f32 - x.cos();
        let mut acc = Accumulator1059::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.025_f32 + y.sin();
        let b = y * 5.755_f32 - x.cos();
        let mut acc = Accumulator1059::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.901_f32 + y.sin();
        let b = y * 2.824_f32 - x.cos();
        let mut acc = Accumulator1059::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.049_f32 + y.sin();
        let b = y * 1.984_f32 - x.cos();
        let mut acc = Accumulator1059::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.591_f32 + y.sin();
        let b = y * 0.742_f32 - x.cos();
        let mut acc = Accumulator1059::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.66_f32 + y.sin();
        let b = y * 8.978_f32 - x.cos();
        let mut acc = Accumulator1059::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.98_f32 + y.sin();
        let b = y * 3.01_f32 - x.cos();
        let mut acc = Accumulator1059::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.906_f32 + y.sin();
        let b = y * 5.243_f32 - x.cos();
        let mut acc = Accumulator1059::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.398_f32 + y.sin();
        let b = y * 4.064_f32 - x.cos();
        let mut acc = Accumulator1059::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.127_f32 + y.sin();
        let b = y * 6.015_f32 - x.cos();
        let mut acc = Accumulator1059::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.318_f32 + y.sin();
        let b = y * 9.179_f32 - x.cos();
        let mut acc = Accumulator1059::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.606_f32 + y.sin();
        let b = y * 4.763_f32 - x.cos();
        let mut acc = Accumulator1059::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.615_f32 + y.sin();
        let b = y * 3.702_f32 - x.cos();
        let mut acc = Accumulator1059::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.553_f32 + y.sin();
        let b = y * 3.978_f32 - x.cos();
        let mut acc = Accumulator1059::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1059(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_1059() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1059(total as u64) % 997) as f32;
        total
    }
}

pub mod m1060 {
    use super::*;

    pub struct Accumulator1060<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1060<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.653_f32 + y.sin();
        let b = y * 2.596_f32 - x.cos();
        let mut acc = Accumulator1060::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.869_f32 + y.sin();
        let b = y * 3.744_f32 - x.cos();
        let mut acc = Accumulator1060::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.079_f32 + y.sin();
        let b = y * 7.512_f32 - x.cos();
        let mut acc = Accumulator1060::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.874_f32 + y.sin();
        let b = y * 9.033_f32 - x.cos();
        let mut acc = Accumulator1060::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.82_f32 + y.sin();
        let b = y * 4.386_f32 - x.cos();
        let mut acc = Accumulator1060::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.435_f32 + y.sin();
        let b = y * 5.508_f32 - x.cos();
        let mut acc = Accumulator1060::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.374_f32 + y.sin();
        let b = y * 3.457_f32 - x.cos();
        let mut acc = Accumulator1060::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.196_f32 + y.sin();
        let b = y * 6.264_f32 - x.cos();
        let mut acc = Accumulator1060::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.396_f32 + y.sin();
        let b = y * 8.37_f32 - x.cos();
        let mut acc = Accumulator1060::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.465_f32 + y.sin();
        let b = y * 7.371_f32 - x.cos();
        let mut acc = Accumulator1060::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.54_f32 + y.sin();
        let b = y * 3.811_f32 - x.cos();
        let mut acc = Accumulator1060::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.988_f32 + y.sin();
        let b = y * 6.977_f32 - x.cos();
        let mut acc = Accumulator1060::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.649_f32 + y.sin();
        let b = y * 2.419_f32 - x.cos();
        let mut acc = Accumulator1060::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.108_f32 + y.sin();
        let b = y * 9.66_f32 - x.cos();
        let mut acc = Accumulator1060::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.118_f32 + y.sin();
        let b = y * 0.194_f32 - x.cos();
        let mut acc = Accumulator1060::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.325_f32 + y.sin();
        let b = y * 0.571_f32 - x.cos();
        let mut acc = Accumulator1060::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.582_f32 + y.sin();
        let b = y * 9.088_f32 - x.cos();
        let mut acc = Accumulator1060::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.646_f32 + y.sin();
        let b = y * 1.608_f32 - x.cos();
        let mut acc = Accumulator1060::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.814_f32 + y.sin();
        let b = y * 2.069_f32 - x.cos();
        let mut acc = Accumulator1060::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.335_f32 + y.sin();
        let b = y * 0.723_f32 - x.cos();
        let mut acc = Accumulator1060::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1060(seed: u64) -> u64 {
        let re = Regex::new(r"m1060-(\d+)").unwrap();
        let hay = format!("m1060-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_1060() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1060(total as u64) % 997) as f32;
        total
    }
}

pub mod m1061 {
    use super::*;

    pub struct Accumulator1061<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1061<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.15_f32 + y.sin();
        let b = y * 9.316_f32 - x.cos();
        let mut acc = Accumulator1061::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.133_f32 + y.sin();
        let b = y * 6.614_f32 - x.cos();
        let mut acc = Accumulator1061::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.996_f32 + y.sin();
        let b = y * 5.731_f32 - x.cos();
        let mut acc = Accumulator1061::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.427_f32 + y.sin();
        let b = y * 5.771_f32 - x.cos();
        let mut acc = Accumulator1061::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.223_f32 + y.sin();
        let b = y * 4.359_f32 - x.cos();
        let mut acc = Accumulator1061::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.46_f32 + y.sin();
        let b = y * 5.063_f32 - x.cos();
        let mut acc = Accumulator1061::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.297_f32 + y.sin();
        let b = y * 3.333_f32 - x.cos();
        let mut acc = Accumulator1061::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.58_f32 + y.sin();
        let b = y * 9.211_f32 - x.cos();
        let mut acc = Accumulator1061::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.705_f32 + y.sin();
        let b = y * 5.773_f32 - x.cos();
        let mut acc = Accumulator1061::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.999_f32 + y.sin();
        let b = y * 6.005_f32 - x.cos();
        let mut acc = Accumulator1061::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.123_f32 + y.sin();
        let b = y * 9.892_f32 - x.cos();
        let mut acc = Accumulator1061::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.459_f32 + y.sin();
        let b = y * 5.224_f32 - x.cos();
        let mut acc = Accumulator1061::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.562_f32 + y.sin();
        let b = y * 8.509_f32 - x.cos();
        let mut acc = Accumulator1061::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.87_f32 + y.sin();
        let b = y * 0.363_f32 - x.cos();
        let mut acc = Accumulator1061::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.874_f32 + y.sin();
        let b = y * 5.394_f32 - x.cos();
        let mut acc = Accumulator1061::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.188_f32 + y.sin();
        let b = y * 8.268_f32 - x.cos();
        let mut acc = Accumulator1061::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.986_f32 + y.sin();
        let b = y * 2.333_f32 - x.cos();
        let mut acc = Accumulator1061::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.019_f32 + y.sin();
        let b = y * 3.134_f32 - x.cos();
        let mut acc = Accumulator1061::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.487_f32 + y.sin();
        let b = y * 5.871_f32 - x.cos();
        let mut acc = Accumulator1061::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.913_f32 + y.sin();
        let b = y * 5.064_f32 - x.cos();
        let mut acc = Accumulator1061::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1061(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_1061() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1061(total as u64) % 997) as f32;
        total
    }
}

pub mod m1062 {
    use super::*;

    pub struct Accumulator1062<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1062<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.976_f32 + y.sin();
        let b = y * 3.546_f32 - x.cos();
        let mut acc = Accumulator1062::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.622_f32 + y.sin();
        let b = y * 9.515_f32 - x.cos();
        let mut acc = Accumulator1062::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.852_f32 + y.sin();
        let b = y * 4.315_f32 - x.cos();
        let mut acc = Accumulator1062::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.458_f32 + y.sin();
        let b = y * 8.029_f32 - x.cos();
        let mut acc = Accumulator1062::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.666_f32 + y.sin();
        let b = y * 0.152_f32 - x.cos();
        let mut acc = Accumulator1062::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.268_f32 + y.sin();
        let b = y * 3.591_f32 - x.cos();
        let mut acc = Accumulator1062::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.337_f32 + y.sin();
        let b = y * 9.488_f32 - x.cos();
        let mut acc = Accumulator1062::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.45_f32 + y.sin();
        let b = y * 0.549_f32 - x.cos();
        let mut acc = Accumulator1062::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.888_f32 + y.sin();
        let b = y * 1.015_f32 - x.cos();
        let mut acc = Accumulator1062::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.452_f32 + y.sin();
        let b = y * 4.151_f32 - x.cos();
        let mut acc = Accumulator1062::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.071_f32 + y.sin();
        let b = y * 3.401_f32 - x.cos();
        let mut acc = Accumulator1062::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.671_f32 + y.sin();
        let b = y * 4.033_f32 - x.cos();
        let mut acc = Accumulator1062::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.176_f32 + y.sin();
        let b = y * 7.124_f32 - x.cos();
        let mut acc = Accumulator1062::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.827_f32 + y.sin();
        let b = y * 2.917_f32 - x.cos();
        let mut acc = Accumulator1062::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.899_f32 + y.sin();
        let b = y * 8.895_f32 - x.cos();
        let mut acc = Accumulator1062::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.689_f32 + y.sin();
        let b = y * 9.247_f32 - x.cos();
        let mut acc = Accumulator1062::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.069_f32 + y.sin();
        let b = y * 9.389_f32 - x.cos();
        let mut acc = Accumulator1062::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.315_f32 + y.sin();
        let b = y * 7.993_f32 - x.cos();
        let mut acc = Accumulator1062::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.03_f32 + y.sin();
        let b = y * 0.318_f32 - x.cos();
        let mut acc = Accumulator1062::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.157_f32 + y.sin();
        let b = y * 9.784_f32 - x.cos();
        let mut acc = Accumulator1062::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1062(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(1062u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_1062() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1062(total as u64) % 997) as f32;
        total
    }
}

pub mod m1063 {
    use super::*;

    pub struct Accumulator1063<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1063<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.244_f32 + y.sin();
        let b = y * 5.985_f32 - x.cos();
        let mut acc = Accumulator1063::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.94_f32 + y.sin();
        let b = y * 2.822_f32 - x.cos();
        let mut acc = Accumulator1063::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.059_f32 + y.sin();
        let b = y * 1.268_f32 - x.cos();
        let mut acc = Accumulator1063::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.682_f32 + y.sin();
        let b = y * 7.689_f32 - x.cos();
        let mut acc = Accumulator1063::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.44_f32 + y.sin();
        let b = y * 4.661_f32 - x.cos();
        let mut acc = Accumulator1063::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.289_f32 + y.sin();
        let b = y * 1.09_f32 - x.cos();
        let mut acc = Accumulator1063::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.981_f32 + y.sin();
        let b = y * 5.204_f32 - x.cos();
        let mut acc = Accumulator1063::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.794_f32 + y.sin();
        let b = y * 4.486_f32 - x.cos();
        let mut acc = Accumulator1063::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.848_f32 + y.sin();
        let b = y * 2.196_f32 - x.cos();
        let mut acc = Accumulator1063::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.384_f32 + y.sin();
        let b = y * 4.165_f32 - x.cos();
        let mut acc = Accumulator1063::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.011_f32 + y.sin();
        let b = y * 6.78_f32 - x.cos();
        let mut acc = Accumulator1063::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.358_f32 + y.sin();
        let b = y * 2.097_f32 - x.cos();
        let mut acc = Accumulator1063::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.094_f32 + y.sin();
        let b = y * 4.124_f32 - x.cos();
        let mut acc = Accumulator1063::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.405_f32 + y.sin();
        let b = y * 3.084_f32 - x.cos();
        let mut acc = Accumulator1063::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.212_f32 + y.sin();
        let b = y * 2.245_f32 - x.cos();
        let mut acc = Accumulator1063::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.789_f32 + y.sin();
        let b = y * 2.382_f32 - x.cos();
        let mut acc = Accumulator1063::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.702_f32 + y.sin();
        let b = y * 7.61_f32 - x.cos();
        let mut acc = Accumulator1063::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.392_f32 + y.sin();
        let b = y * 2.29_f32 - x.cos();
        let mut acc = Accumulator1063::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.652_f32 + y.sin();
        let b = y * 4.089_f32 - x.cos();
        let mut acc = Accumulator1063::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.731_f32 + y.sin();
        let b = y * 6.957_f32 - x.cos();
        let mut acc = Accumulator1063::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1063(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_1063() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1063(total as u64) % 997) as f32;
        total
    }
}

pub mod m1064 {
    use super::*;

    pub struct Accumulator1064<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1064<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.571_f32 + y.sin();
        let b = y * 2.484_f32 - x.cos();
        let mut acc = Accumulator1064::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.512_f32 + y.sin();
        let b = y * 5.376_f32 - x.cos();
        let mut acc = Accumulator1064::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.101_f32 + y.sin();
        let b = y * 3.405_f32 - x.cos();
        let mut acc = Accumulator1064::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.993_f32 + y.sin();
        let b = y * 5.543_f32 - x.cos();
        let mut acc = Accumulator1064::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.985_f32 + y.sin();
        let b = y * 9.643_f32 - x.cos();
        let mut acc = Accumulator1064::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.605_f32 + y.sin();
        let b = y * 0.68_f32 - x.cos();
        let mut acc = Accumulator1064::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.802_f32 + y.sin();
        let b = y * 6.949_f32 - x.cos();
        let mut acc = Accumulator1064::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.796_f32 + y.sin();
        let b = y * 8.602_f32 - x.cos();
        let mut acc = Accumulator1064::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.546_f32 + y.sin();
        let b = y * 6.858_f32 - x.cos();
        let mut acc = Accumulator1064::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.395_f32 + y.sin();
        let b = y * 6.888_f32 - x.cos();
        let mut acc = Accumulator1064::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.859_f32 + y.sin();
        let b = y * 5.011_f32 - x.cos();
        let mut acc = Accumulator1064::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.407_f32 + y.sin();
        let b = y * 4.184_f32 - x.cos();
        let mut acc = Accumulator1064::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.337_f32 + y.sin();
        let b = y * 0.18_f32 - x.cos();
        let mut acc = Accumulator1064::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.677_f32 + y.sin();
        let b = y * 4.426_f32 - x.cos();
        let mut acc = Accumulator1064::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.186_f32 + y.sin();
        let b = y * 2.627_f32 - x.cos();
        let mut acc = Accumulator1064::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.014_f32 + y.sin();
        let b = y * 5.853_f32 - x.cos();
        let mut acc = Accumulator1064::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.473_f32 + y.sin();
        let b = y * 3.572_f32 - x.cos();
        let mut acc = Accumulator1064::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.988_f32 + y.sin();
        let b = y * 0.436_f32 - x.cos();
        let mut acc = Accumulator1064::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.467_f32 + y.sin();
        let b = y * 3.424_f32 - x.cos();
        let mut acc = Accumulator1064::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.836_f32 + y.sin();
        let b = y * 9.421_f32 - x.cos();
        let mut acc = Accumulator1064::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1064(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_1064() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1064(total as u64) % 997) as f32;
        total
    }
}

pub mod m1065 {
    use super::*;

    pub struct Accumulator1065<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1065<T> {
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
        let b = y * 1.56_f32 - x.cos();
        let mut acc = Accumulator1065::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.328_f32 + y.sin();
        let b = y * 2.861_f32 - x.cos();
        let mut acc = Accumulator1065::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.453_f32 + y.sin();
        let b = y * 2.573_f32 - x.cos();
        let mut acc = Accumulator1065::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.968_f32 + y.sin();
        let b = y * 9.694_f32 - x.cos();
        let mut acc = Accumulator1065::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.917_f32 + y.sin();
        let b = y * 2.838_f32 - x.cos();
        let mut acc = Accumulator1065::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.503_f32 + y.sin();
        let b = y * 3.669_f32 - x.cos();
        let mut acc = Accumulator1065::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.136_f32 + y.sin();
        let b = y * 7.29_f32 - x.cos();
        let mut acc = Accumulator1065::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.701_f32 + y.sin();
        let b = y * 5.595_f32 - x.cos();
        let mut acc = Accumulator1065::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.798_f32 + y.sin();
        let b = y * 7.603_f32 - x.cos();
        let mut acc = Accumulator1065::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.176_f32 + y.sin();
        let b = y * 7.243_f32 - x.cos();
        let mut acc = Accumulator1065::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.263_f32 + y.sin();
        let b = y * 3.397_f32 - x.cos();
        let mut acc = Accumulator1065::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.803_f32 + y.sin();
        let b = y * 2.604_f32 - x.cos();
        let mut acc = Accumulator1065::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.918_f32 + y.sin();
        let b = y * 1.56_f32 - x.cos();
        let mut acc = Accumulator1065::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.884_f32 + y.sin();
        let b = y * 0.343_f32 - x.cos();
        let mut acc = Accumulator1065::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.209_f32 + y.sin();
        let b = y * 0.63_f32 - x.cos();
        let mut acc = Accumulator1065::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.898_f32 + y.sin();
        let b = y * 9.045_f32 - x.cos();
        let mut acc = Accumulator1065::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.671_f32 + y.sin();
        let b = y * 7.027_f32 - x.cos();
        let mut acc = Accumulator1065::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.034_f32 + y.sin();
        let b = y * 5.336_f32 - x.cos();
        let mut acc = Accumulator1065::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.985_f32 + y.sin();
        let b = y * 2.691_f32 - x.cos();
        let mut acc = Accumulator1065::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.702_f32 + y.sin();
        let b = y * 5.811_f32 - x.cos();
        let mut acc = Accumulator1065::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1065(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m1065-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_1065() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1065(total as u64) % 997) as f32;
        total
    }
}

pub mod m1066 {
    use super::*;

    pub struct Accumulator1066<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1066<T> {
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
        let b = y * 6.954_f32 - x.cos();
        let mut acc = Accumulator1066::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.139_f32 + y.sin();
        let b = y * 7.852_f32 - x.cos();
        let mut acc = Accumulator1066::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.834_f32 + y.sin();
        let b = y * 0.343_f32 - x.cos();
        let mut acc = Accumulator1066::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.846_f32 + y.sin();
        let b = y * 3.037_f32 - x.cos();
        let mut acc = Accumulator1066::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.592_f32 + y.sin();
        let b = y * 0.591_f32 - x.cos();
        let mut acc = Accumulator1066::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.403_f32 + y.sin();
        let b = y * 1.219_f32 - x.cos();
        let mut acc = Accumulator1066::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.212_f32 + y.sin();
        let b = y * 4.879_f32 - x.cos();
        let mut acc = Accumulator1066::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.465_f32 + y.sin();
        let b = y * 3.921_f32 - x.cos();
        let mut acc = Accumulator1066::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.392_f32 + y.sin();
        let b = y * 0.513_f32 - x.cos();
        let mut acc = Accumulator1066::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.887_f32 + y.sin();
        let b = y * 0.592_f32 - x.cos();
        let mut acc = Accumulator1066::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.465_f32 + y.sin();
        let b = y * 8.241_f32 - x.cos();
        let mut acc = Accumulator1066::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.578_f32 + y.sin();
        let b = y * 3.684_f32 - x.cos();
        let mut acc = Accumulator1066::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.817_f32 + y.sin();
        let b = y * 9.861_f32 - x.cos();
        let mut acc = Accumulator1066::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.444_f32 + y.sin();
        let b = y * 3.055_f32 - x.cos();
        let mut acc = Accumulator1066::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.191_f32 + y.sin();
        let b = y * 8.044_f32 - x.cos();
        let mut acc = Accumulator1066::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.277_f32 + y.sin();
        let b = y * 9.454_f32 - x.cos();
        let mut acc = Accumulator1066::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.903_f32 + y.sin();
        let b = y * 0.346_f32 - x.cos();
        let mut acc = Accumulator1066::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.6_f32 + y.sin();
        let b = y * 0.719_f32 - x.cos();
        let mut acc = Accumulator1066::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.522_f32 + y.sin();
        let b = y * 0.736_f32 - x.cos();
        let mut acc = Accumulator1066::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.279_f32 + y.sin();
        let b = y * 1.081_f32 - x.cos();
        let mut acc = Accumulator1066::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1066(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_1066() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1066(total as u64) % 997) as f32;
        total
    }
}

pub mod m1067 {
    use super::*;

    pub struct Accumulator1067<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1067<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.992_f32 + y.sin();
        let b = y * 0.333_f32 - x.cos();
        let mut acc = Accumulator1067::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.659_f32 + y.sin();
        let b = y * 9.647_f32 - x.cos();
        let mut acc = Accumulator1067::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.987_f32 + y.sin();
        let b = y * 3.305_f32 - x.cos();
        let mut acc = Accumulator1067::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.357_f32 + y.sin();
        let b = y * 7.644_f32 - x.cos();
        let mut acc = Accumulator1067::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.896_f32 + y.sin();
        let b = y * 7.84_f32 - x.cos();
        let mut acc = Accumulator1067::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.467_f32 + y.sin();
        let b = y * 9.415_f32 - x.cos();
        let mut acc = Accumulator1067::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.976_f32 + y.sin();
        let b = y * 5.348_f32 - x.cos();
        let mut acc = Accumulator1067::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.298_f32 + y.sin();
        let b = y * 9.432_f32 - x.cos();
        let mut acc = Accumulator1067::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.105_f32 + y.sin();
        let b = y * 6.081_f32 - x.cos();
        let mut acc = Accumulator1067::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.352_f32 + y.sin();
        let b = y * 2.825_f32 - x.cos();
        let mut acc = Accumulator1067::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.327_f32 + y.sin();
        let b = y * 7.489_f32 - x.cos();
        let mut acc = Accumulator1067::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.534_f32 + y.sin();
        let b = y * 9.39_f32 - x.cos();
        let mut acc = Accumulator1067::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.167_f32 + y.sin();
        let b = y * 6.223_f32 - x.cos();
        let mut acc = Accumulator1067::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.215_f32 + y.sin();
        let b = y * 8.908_f32 - x.cos();
        let mut acc = Accumulator1067::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.152_f32 + y.sin();
        let b = y * 1.257_f32 - x.cos();
        let mut acc = Accumulator1067::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.701_f32 + y.sin();
        let b = y * 5.947_f32 - x.cos();
        let mut acc = Accumulator1067::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.262_f32 + y.sin();
        let b = y * 7.72_f32 - x.cos();
        let mut acc = Accumulator1067::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.189_f32 + y.sin();
        let b = y * 9.053_f32 - x.cos();
        let mut acc = Accumulator1067::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.623_f32 + y.sin();
        let b = y * 8.196_f32 - x.cos();
        let mut acc = Accumulator1067::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.377_f32 + y.sin();
        let b = y * 8.346_f32 - x.cos();
        let mut acc = Accumulator1067::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1067(seed: u64) -> u64 {
        let re = Regex::new(r"m1067-(\d+)").unwrap();
        let hay = format!("m1067-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_1067() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1067(total as u64) % 997) as f32;
        total
    }
}

pub mod m1068 {
    use super::*;

    pub struct Accumulator1068<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1068<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.945_f32 + y.sin();
        let b = y * 1.082_f32 - x.cos();
        let mut acc = Accumulator1068::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.088_f32 + y.sin();
        let b = y * 8.497_f32 - x.cos();
        let mut acc = Accumulator1068::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.58_f32 + y.sin();
        let b = y * 9.256_f32 - x.cos();
        let mut acc = Accumulator1068::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.421_f32 + y.sin();
        let b = y * 6.184_f32 - x.cos();
        let mut acc = Accumulator1068::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.668_f32 + y.sin();
        let b = y * 7.585_f32 - x.cos();
        let mut acc = Accumulator1068::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.224_f32 + y.sin();
        let b = y * 3.937_f32 - x.cos();
        let mut acc = Accumulator1068::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.148_f32 + y.sin();
        let b = y * 2.199_f32 - x.cos();
        let mut acc = Accumulator1068::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.083_f32 + y.sin();
        let b = y * 2.858_f32 - x.cos();
        let mut acc = Accumulator1068::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.928_f32 + y.sin();
        let b = y * 5.089_f32 - x.cos();
        let mut acc = Accumulator1068::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.427_f32 + y.sin();
        let b = y * 2.673_f32 - x.cos();
        let mut acc = Accumulator1068::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.614_f32 + y.sin();
        let b = y * 3.958_f32 - x.cos();
        let mut acc = Accumulator1068::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.518_f32 + y.sin();
        let b = y * 6.427_f32 - x.cos();
        let mut acc = Accumulator1068::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.785_f32 + y.sin();
        let b = y * 3.936_f32 - x.cos();
        let mut acc = Accumulator1068::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.633_f32 + y.sin();
        let b = y * 3.423_f32 - x.cos();
        let mut acc = Accumulator1068::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.479_f32 + y.sin();
        let b = y * 3.963_f32 - x.cos();
        let mut acc = Accumulator1068::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.324_f32 + y.sin();
        let b = y * 1.871_f32 - x.cos();
        let mut acc = Accumulator1068::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.222_f32 + y.sin();
        let b = y * 0.468_f32 - x.cos();
        let mut acc = Accumulator1068::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.372_f32 + y.sin();
        let b = y * 4.61_f32 - x.cos();
        let mut acc = Accumulator1068::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.187_f32 + y.sin();
        let b = y * 6.288_f32 - x.cos();
        let mut acc = Accumulator1068::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.684_f32 + y.sin();
        let b = y * 6.016_f32 - x.cos();
        let mut acc = Accumulator1068::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1068(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_1068() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1068(total as u64) % 997) as f32;
        total
    }
}

pub mod m1069 {
    use super::*;

    pub struct Accumulator1069<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1069<T> {
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
        let b = y * 1.669_f32 - x.cos();
        let mut acc = Accumulator1069::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.515_f32 + y.sin();
        let b = y * 0.434_f32 - x.cos();
        let mut acc = Accumulator1069::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.072_f32 + y.sin();
        let b = y * 1.377_f32 - x.cos();
        let mut acc = Accumulator1069::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.244_f32 + y.sin();
        let b = y * 5.041_f32 - x.cos();
        let mut acc = Accumulator1069::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.941_f32 + y.sin();
        let b = y * 0.694_f32 - x.cos();
        let mut acc = Accumulator1069::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.608_f32 + y.sin();
        let b = y * 6.07_f32 - x.cos();
        let mut acc = Accumulator1069::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.493_f32 + y.sin();
        let b = y * 5.252_f32 - x.cos();
        let mut acc = Accumulator1069::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.763_f32 + y.sin();
        let b = y * 2.47_f32 - x.cos();
        let mut acc = Accumulator1069::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.995_f32 + y.sin();
        let b = y * 4.721_f32 - x.cos();
        let mut acc = Accumulator1069::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.433_f32 + y.sin();
        let b = y * 0.49_f32 - x.cos();
        let mut acc = Accumulator1069::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.882_f32 + y.sin();
        let b = y * 1.337_f32 - x.cos();
        let mut acc = Accumulator1069::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.647_f32 + y.sin();
        let b = y * 6.518_f32 - x.cos();
        let mut acc = Accumulator1069::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.887_f32 + y.sin();
        let b = y * 3.809_f32 - x.cos();
        let mut acc = Accumulator1069::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.046_f32 + y.sin();
        let b = y * 0.814_f32 - x.cos();
        let mut acc = Accumulator1069::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.335_f32 + y.sin();
        let b = y * 2.331_f32 - x.cos();
        let mut acc = Accumulator1069::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.695_f32 + y.sin();
        let b = y * 9.485_f32 - x.cos();
        let mut acc = Accumulator1069::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.474_f32 + y.sin();
        let b = y * 0.786_f32 - x.cos();
        let mut acc = Accumulator1069::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.191_f32 + y.sin();
        let b = y * 8.639_f32 - x.cos();
        let mut acc = Accumulator1069::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.276_f32 + y.sin();
        let b = y * 4.072_f32 - x.cos();
        let mut acc = Accumulator1069::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.209_f32 + y.sin();
        let b = y * 7.004_f32 - x.cos();
        let mut acc = Accumulator1069::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1069(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(1069u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_1069() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1069(total as u64) % 997) as f32;
        total
    }
}

pub mod m1070 {
    use super::*;

    pub struct Accumulator1070<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1070<T> {
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
        let b = y * 0.513_f32 - x.cos();
        let mut acc = Accumulator1070::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.46_f32 + y.sin();
        let b = y * 7.501_f32 - x.cos();
        let mut acc = Accumulator1070::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.658_f32 + y.sin();
        let b = y * 5.291_f32 - x.cos();
        let mut acc = Accumulator1070::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.718_f32 + y.sin();
        let b = y * 2.811_f32 - x.cos();
        let mut acc = Accumulator1070::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.661_f32 + y.sin();
        let b = y * 0.619_f32 - x.cos();
        let mut acc = Accumulator1070::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.183_f32 + y.sin();
        let b = y * 1.596_f32 - x.cos();
        let mut acc = Accumulator1070::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.9_f32 + y.sin();
        let b = y * 4.949_f32 - x.cos();
        let mut acc = Accumulator1070::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.457_f32 + y.sin();
        let b = y * 0.787_f32 - x.cos();
        let mut acc = Accumulator1070::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.361_f32 + y.sin();
        let b = y * 5.219_f32 - x.cos();
        let mut acc = Accumulator1070::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.85_f32 + y.sin();
        let b = y * 1.418_f32 - x.cos();
        let mut acc = Accumulator1070::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.653_f32 + y.sin();
        let b = y * 3.753_f32 - x.cos();
        let mut acc = Accumulator1070::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.606_f32 + y.sin();
        let b = y * 3.249_f32 - x.cos();
        let mut acc = Accumulator1070::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.435_f32 + y.sin();
        let b = y * 2.655_f32 - x.cos();
        let mut acc = Accumulator1070::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.54_f32 + y.sin();
        let b = y * 0.421_f32 - x.cos();
        let mut acc = Accumulator1070::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.431_f32 + y.sin();
        let b = y * 9.243_f32 - x.cos();
        let mut acc = Accumulator1070::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.344_f32 + y.sin();
        let b = y * 2.225_f32 - x.cos();
        let mut acc = Accumulator1070::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.973_f32 + y.sin();
        let b = y * 2.203_f32 - x.cos();
        let mut acc = Accumulator1070::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.943_f32 + y.sin();
        let b = y * 5.529_f32 - x.cos();
        let mut acc = Accumulator1070::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.057_f32 + y.sin();
        let b = y * 7.209_f32 - x.cos();
        let mut acc = Accumulator1070::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.316_f32 + y.sin();
        let b = y * 8.411_f32 - x.cos();
        let mut acc = Accumulator1070::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1070(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_1070() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1070(total as u64) % 997) as f32;
        total
    }
}

pub mod m1071 {
    use super::*;

    pub struct Accumulator1071<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1071<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.2_f32 + y.sin();
        let b = y * 4.728_f32 - x.cos();
        let mut acc = Accumulator1071::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.236_f32 + y.sin();
        let b = y * 5.551_f32 - x.cos();
        let mut acc = Accumulator1071::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.161_f32 + y.sin();
        let b = y * 1.737_f32 - x.cos();
        let mut acc = Accumulator1071::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.403_f32 + y.sin();
        let b = y * 3.686_f32 - x.cos();
        let mut acc = Accumulator1071::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.307_f32 + y.sin();
        let b = y * 5.072_f32 - x.cos();
        let mut acc = Accumulator1071::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.381_f32 + y.sin();
        let b = y * 3.547_f32 - x.cos();
        let mut acc = Accumulator1071::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.03_f32 + y.sin();
        let b = y * 3.362_f32 - x.cos();
        let mut acc = Accumulator1071::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.492_f32 + y.sin();
        let b = y * 3.989_f32 - x.cos();
        let mut acc = Accumulator1071::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.929_f32 + y.sin();
        let b = y * 4.712_f32 - x.cos();
        let mut acc = Accumulator1071::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.519_f32 + y.sin();
        let b = y * 4.791_f32 - x.cos();
        let mut acc = Accumulator1071::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.589_f32 + y.sin();
        let b = y * 8.06_f32 - x.cos();
        let mut acc = Accumulator1071::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.809_f32 + y.sin();
        let b = y * 5.427_f32 - x.cos();
        let mut acc = Accumulator1071::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.271_f32 + y.sin();
        let b = y * 7.557_f32 - x.cos();
        let mut acc = Accumulator1071::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.669_f32 + y.sin();
        let b = y * 7.283_f32 - x.cos();
        let mut acc = Accumulator1071::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.511_f32 + y.sin();
        let b = y * 5.282_f32 - x.cos();
        let mut acc = Accumulator1071::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.116_f32 + y.sin();
        let b = y * 0.749_f32 - x.cos();
        let mut acc = Accumulator1071::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.4_f32 + y.sin();
        let b = y * 6.072_f32 - x.cos();
        let mut acc = Accumulator1071::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.832_f32 + y.sin();
        let b = y * 9.861_f32 - x.cos();
        let mut acc = Accumulator1071::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.407_f32 + y.sin();
        let b = y * 8.862_f32 - x.cos();
        let mut acc = Accumulator1071::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.942_f32 + y.sin();
        let b = y * 5.597_f32 - x.cos();
        let mut acc = Accumulator1071::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1071(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_1071() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1071(total as u64) % 997) as f32;
        total
    }
}

pub mod m1072 {
    use super::*;

    pub struct Accumulator1072<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1072<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.431_f32 + y.sin();
        let b = y * 6.575_f32 - x.cos();
        let mut acc = Accumulator1072::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.573_f32 + y.sin();
        let b = y * 6.894_f32 - x.cos();
        let mut acc = Accumulator1072::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.952_f32 + y.sin();
        let b = y * 4.189_f32 - x.cos();
        let mut acc = Accumulator1072::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.842_f32 + y.sin();
        let b = y * 7.552_f32 - x.cos();
        let mut acc = Accumulator1072::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.649_f32 + y.sin();
        let b = y * 7.102_f32 - x.cos();
        let mut acc = Accumulator1072::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.906_f32 + y.sin();
        let b = y * 6.092_f32 - x.cos();
        let mut acc = Accumulator1072::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.985_f32 + y.sin();
        let b = y * 4.141_f32 - x.cos();
        let mut acc = Accumulator1072::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.579_f32 + y.sin();
        let b = y * 4.92_f32 - x.cos();
        let mut acc = Accumulator1072::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.7_f32 + y.sin();
        let b = y * 6.502_f32 - x.cos();
        let mut acc = Accumulator1072::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.051_f32 + y.sin();
        let b = y * 2.154_f32 - x.cos();
        let mut acc = Accumulator1072::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.54_f32 + y.sin();
        let b = y * 8.712_f32 - x.cos();
        let mut acc = Accumulator1072::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.562_f32 + y.sin();
        let b = y * 7.607_f32 - x.cos();
        let mut acc = Accumulator1072::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.345_f32 + y.sin();
        let b = y * 7.241_f32 - x.cos();
        let mut acc = Accumulator1072::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.817_f32 + y.sin();
        let b = y * 9.781_f32 - x.cos();
        let mut acc = Accumulator1072::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.527_f32 + y.sin();
        let b = y * 1.921_f32 - x.cos();
        let mut acc = Accumulator1072::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.849_f32 + y.sin();
        let b = y * 0.956_f32 - x.cos();
        let mut acc = Accumulator1072::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.195_f32 + y.sin();
        let b = y * 2.947_f32 - x.cos();
        let mut acc = Accumulator1072::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.991_f32 + y.sin();
        let b = y * 5.999_f32 - x.cos();
        let mut acc = Accumulator1072::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.195_f32 + y.sin();
        let b = y * 1.874_f32 - x.cos();
        let mut acc = Accumulator1072::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.591_f32 + y.sin();
        let b = y * 8.261_f32 - x.cos();
        let mut acc = Accumulator1072::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1072(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m1072-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_1072() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1072(total as u64) % 997) as f32;
        total
    }
}

pub mod m1073 {
    use super::*;

    pub struct Accumulator1073<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1073<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.9_f32 + y.sin();
        let b = y * 9.376_f32 - x.cos();
        let mut acc = Accumulator1073::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.536_f32 + y.sin();
        let b = y * 9.059_f32 - x.cos();
        let mut acc = Accumulator1073::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.391_f32 + y.sin();
        let b = y * 7.895_f32 - x.cos();
        let mut acc = Accumulator1073::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.774_f32 + y.sin();
        let b = y * 9.077_f32 - x.cos();
        let mut acc = Accumulator1073::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.277_f32 + y.sin();
        let b = y * 2.261_f32 - x.cos();
        let mut acc = Accumulator1073::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.81_f32 + y.sin();
        let b = y * 5.696_f32 - x.cos();
        let mut acc = Accumulator1073::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.585_f32 + y.sin();
        let b = y * 1.04_f32 - x.cos();
        let mut acc = Accumulator1073::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.703_f32 + y.sin();
        let b = y * 2.512_f32 - x.cos();
        let mut acc = Accumulator1073::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.795_f32 + y.sin();
        let b = y * 3.971_f32 - x.cos();
        let mut acc = Accumulator1073::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.362_f32 + y.sin();
        let b = y * 2.06_f32 - x.cos();
        let mut acc = Accumulator1073::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.216_f32 + y.sin();
        let b = y * 7.035_f32 - x.cos();
        let mut acc = Accumulator1073::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.02_f32 + y.sin();
        let b = y * 3.643_f32 - x.cos();
        let mut acc = Accumulator1073::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.261_f32 + y.sin();
        let b = y * 9.734_f32 - x.cos();
        let mut acc = Accumulator1073::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.835_f32 + y.sin();
        let b = y * 2.366_f32 - x.cos();
        let mut acc = Accumulator1073::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.872_f32 + y.sin();
        let b = y * 2.933_f32 - x.cos();
        let mut acc = Accumulator1073::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.265_f32 + y.sin();
        let b = y * 7.239_f32 - x.cos();
        let mut acc = Accumulator1073::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.733_f32 + y.sin();
        let b = y * 7.333_f32 - x.cos();
        let mut acc = Accumulator1073::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.697_f32 + y.sin();
        let b = y * 9.706_f32 - x.cos();
        let mut acc = Accumulator1073::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.386_f32 + y.sin();
        let b = y * 5.481_f32 - x.cos();
        let mut acc = Accumulator1073::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.602_f32 + y.sin();
        let b = y * 8.402_f32 - x.cos();
        let mut acc = Accumulator1073::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1073(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_1073() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1073(total as u64) % 997) as f32;
        total
    }
}

pub mod m1074 {
    use super::*;

    pub struct Accumulator1074<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1074<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.827_f32 + y.sin();
        let b = y * 3.677_f32 - x.cos();
        let mut acc = Accumulator1074::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.643_f32 + y.sin();
        let b = y * 1.491_f32 - x.cos();
        let mut acc = Accumulator1074::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.957_f32 + y.sin();
        let b = y * 6.18_f32 - x.cos();
        let mut acc = Accumulator1074::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.005_f32 + y.sin();
        let b = y * 6.678_f32 - x.cos();
        let mut acc = Accumulator1074::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.443_f32 + y.sin();
        let b = y * 7.267_f32 - x.cos();
        let mut acc = Accumulator1074::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.048_f32 + y.sin();
        let b = y * 0.63_f32 - x.cos();
        let mut acc = Accumulator1074::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.009_f32 + y.sin();
        let b = y * 3.203_f32 - x.cos();
        let mut acc = Accumulator1074::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.238_f32 + y.sin();
        let b = y * 1.359_f32 - x.cos();
        let mut acc = Accumulator1074::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.628_f32 + y.sin();
        let b = y * 0.385_f32 - x.cos();
        let mut acc = Accumulator1074::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.572_f32 + y.sin();
        let b = y * 5.801_f32 - x.cos();
        let mut acc = Accumulator1074::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.729_f32 + y.sin();
        let b = y * 1.4_f32 - x.cos();
        let mut acc = Accumulator1074::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.866_f32 + y.sin();
        let b = y * 3.335_f32 - x.cos();
        let mut acc = Accumulator1074::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.285_f32 + y.sin();
        let b = y * 0.239_f32 - x.cos();
        let mut acc = Accumulator1074::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.922_f32 + y.sin();
        let b = y * 5.932_f32 - x.cos();
        let mut acc = Accumulator1074::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.519_f32 + y.sin();
        let b = y * 9.073_f32 - x.cos();
        let mut acc = Accumulator1074::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.328_f32 + y.sin();
        let b = y * 9.369_f32 - x.cos();
        let mut acc = Accumulator1074::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.307_f32 + y.sin();
        let b = y * 1.392_f32 - x.cos();
        let mut acc = Accumulator1074::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.351_f32 + y.sin();
        let b = y * 1.36_f32 - x.cos();
        let mut acc = Accumulator1074::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.706_f32 + y.sin();
        let b = y * 9.086_f32 - x.cos();
        let mut acc = Accumulator1074::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.705_f32 + y.sin();
        let b = y * 9.008_f32 - x.cos();
        let mut acc = Accumulator1074::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1074(seed: u64) -> u64 {
        let re = Regex::new(r"m1074-(\d+)").unwrap();
        let hay = format!("m1074-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_1074() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1074(total as u64) % 997) as f32;
        total
    }
}

pub mod m1075 {
    use super::*;

    pub struct Accumulator1075<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1075<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.779_f32 + y.sin();
        let b = y * 2.751_f32 - x.cos();
        let mut acc = Accumulator1075::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.712_f32 + y.sin();
        let b = y * 9.731_f32 - x.cos();
        let mut acc = Accumulator1075::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.147_f32 + y.sin();
        let b = y * 2.873_f32 - x.cos();
        let mut acc = Accumulator1075::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.197_f32 + y.sin();
        let b = y * 1.783_f32 - x.cos();
        let mut acc = Accumulator1075::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.45_f32 + y.sin();
        let b = y * 9.257_f32 - x.cos();
        let mut acc = Accumulator1075::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.679_f32 + y.sin();
        let b = y * 0.871_f32 - x.cos();
        let mut acc = Accumulator1075::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.757_f32 + y.sin();
        let b = y * 1.395_f32 - x.cos();
        let mut acc = Accumulator1075::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.949_f32 + y.sin();
        let b = y * 6.435_f32 - x.cos();
        let mut acc = Accumulator1075::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.569_f32 + y.sin();
        let b = y * 4.327_f32 - x.cos();
        let mut acc = Accumulator1075::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.31_f32 + y.sin();
        let b = y * 6.357_f32 - x.cos();
        let mut acc = Accumulator1075::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.908_f32 + y.sin();
        let b = y * 1.832_f32 - x.cos();
        let mut acc = Accumulator1075::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.772_f32 + y.sin();
        let b = y * 3.937_f32 - x.cos();
        let mut acc = Accumulator1075::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.889_f32 + y.sin();
        let b = y * 2.002_f32 - x.cos();
        let mut acc = Accumulator1075::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.928_f32 + y.sin();
        let b = y * 7.553_f32 - x.cos();
        let mut acc = Accumulator1075::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.022_f32 + y.sin();
        let b = y * 7.818_f32 - x.cos();
        let mut acc = Accumulator1075::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.247_f32 + y.sin();
        let b = y * 1.407_f32 - x.cos();
        let mut acc = Accumulator1075::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.059_f32 + y.sin();
        let b = y * 0.152_f32 - x.cos();
        let mut acc = Accumulator1075::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.448_f32 + y.sin();
        let b = y * 1.116_f32 - x.cos();
        let mut acc = Accumulator1075::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.39_f32 + y.sin();
        let b = y * 4.044_f32 - x.cos();
        let mut acc = Accumulator1075::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.572_f32 + y.sin();
        let b = y * 0.97_f32 - x.cos();
        let mut acc = Accumulator1075::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1075(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_1075() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1075(total as u64) % 997) as f32;
        total
    }
}

pub mod m1076 {
    use super::*;

    pub struct Accumulator1076<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1076<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.548_f32 + y.sin();
        let b = y * 4.722_f32 - x.cos();
        let mut acc = Accumulator1076::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.355_f32 + y.sin();
        let b = y * 6.997_f32 - x.cos();
        let mut acc = Accumulator1076::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.364_f32 + y.sin();
        let b = y * 4.779_f32 - x.cos();
        let mut acc = Accumulator1076::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.764_f32 + y.sin();
        let b = y * 0.17_f32 - x.cos();
        let mut acc = Accumulator1076::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.956_f32 + y.sin();
        let b = y * 6.856_f32 - x.cos();
        let mut acc = Accumulator1076::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.762_f32 + y.sin();
        let b = y * 2.323_f32 - x.cos();
        let mut acc = Accumulator1076::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.589_f32 + y.sin();
        let b = y * 8.79_f32 - x.cos();
        let mut acc = Accumulator1076::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.077_f32 + y.sin();
        let b = y * 6.09_f32 - x.cos();
        let mut acc = Accumulator1076::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.501_f32 + y.sin();
        let b = y * 3.216_f32 - x.cos();
        let mut acc = Accumulator1076::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.036_f32 + y.sin();
        let b = y * 9.093_f32 - x.cos();
        let mut acc = Accumulator1076::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.549_f32 + y.sin();
        let b = y * 1.122_f32 - x.cos();
        let mut acc = Accumulator1076::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.396_f32 + y.sin();
        let b = y * 1.64_f32 - x.cos();
        let mut acc = Accumulator1076::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.039_f32 + y.sin();
        let b = y * 0.215_f32 - x.cos();
        let mut acc = Accumulator1076::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.944_f32 + y.sin();
        let b = y * 8.638_f32 - x.cos();
        let mut acc = Accumulator1076::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.846_f32 + y.sin();
        let b = y * 8.505_f32 - x.cos();
        let mut acc = Accumulator1076::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.964_f32 + y.sin();
        let b = y * 3.895_f32 - x.cos();
        let mut acc = Accumulator1076::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.301_f32 + y.sin();
        let b = y * 6.284_f32 - x.cos();
        let mut acc = Accumulator1076::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.447_f32 + y.sin();
        let b = y * 2.92_f32 - x.cos();
        let mut acc = Accumulator1076::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.864_f32 + y.sin();
        let b = y * 4.415_f32 - x.cos();
        let mut acc = Accumulator1076::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.088_f32 + y.sin();
        let b = y * 3.584_f32 - x.cos();
        let mut acc = Accumulator1076::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1076(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(1076u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_1076() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1076(total as u64) % 997) as f32;
        total
    }
}

pub mod m1077 {
    use super::*;

    pub struct Accumulator1077<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1077<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.81_f32 + y.sin();
        let b = y * 8.734_f32 - x.cos();
        let mut acc = Accumulator1077::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.592_f32 + y.sin();
        let b = y * 1.937_f32 - x.cos();
        let mut acc = Accumulator1077::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.318_f32 + y.sin();
        let b = y * 7.469_f32 - x.cos();
        let mut acc = Accumulator1077::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.694_f32 + y.sin();
        let b = y * 9.34_f32 - x.cos();
        let mut acc = Accumulator1077::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.567_f32 + y.sin();
        let b = y * 7.945_f32 - x.cos();
        let mut acc = Accumulator1077::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.424_f32 + y.sin();
        let b = y * 7.267_f32 - x.cos();
        let mut acc = Accumulator1077::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.952_f32 + y.sin();
        let b = y * 6.173_f32 - x.cos();
        let mut acc = Accumulator1077::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.338_f32 + y.sin();
        let b = y * 2.211_f32 - x.cos();
        let mut acc = Accumulator1077::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.394_f32 + y.sin();
        let b = y * 5.502_f32 - x.cos();
        let mut acc = Accumulator1077::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.02_f32 + y.sin();
        let b = y * 7.043_f32 - x.cos();
        let mut acc = Accumulator1077::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.579_f32 + y.sin();
        let b = y * 9.401_f32 - x.cos();
        let mut acc = Accumulator1077::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.22_f32 + y.sin();
        let b = y * 6.22_f32 - x.cos();
        let mut acc = Accumulator1077::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.126_f32 + y.sin();
        let b = y * 3.717_f32 - x.cos();
        let mut acc = Accumulator1077::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.898_f32 + y.sin();
        let b = y * 7.275_f32 - x.cos();
        let mut acc = Accumulator1077::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.219_f32 + y.sin();
        let b = y * 7.191_f32 - x.cos();
        let mut acc = Accumulator1077::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.524_f32 + y.sin();
        let b = y * 9.422_f32 - x.cos();
        let mut acc = Accumulator1077::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.792_f32 + y.sin();
        let b = y * 7.15_f32 - x.cos();
        let mut acc = Accumulator1077::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.199_f32 + y.sin();
        let b = y * 0.6_f32 - x.cos();
        let mut acc = Accumulator1077::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.214_f32 + y.sin();
        let b = y * 9.574_f32 - x.cos();
        let mut acc = Accumulator1077::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.784_f32 + y.sin();
        let b = y * 4.3_f32 - x.cos();
        let mut acc = Accumulator1077::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1077(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_1077() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1077(total as u64) % 997) as f32;
        total
    }
}

pub mod m1078 {
    use super::*;

    pub struct Accumulator1078<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1078<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.647_f32 + y.sin();
        let b = y * 1.852_f32 - x.cos();
        let mut acc = Accumulator1078::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.816_f32 + y.sin();
        let b = y * 7.514_f32 - x.cos();
        let mut acc = Accumulator1078::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.219_f32 + y.sin();
        let b = y * 6.733_f32 - x.cos();
        let mut acc = Accumulator1078::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.311_f32 + y.sin();
        let b = y * 3.786_f32 - x.cos();
        let mut acc = Accumulator1078::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.821_f32 + y.sin();
        let b = y * 8.165_f32 - x.cos();
        let mut acc = Accumulator1078::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.517_f32 + y.sin();
        let b = y * 0.914_f32 - x.cos();
        let mut acc = Accumulator1078::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.7_f32 + y.sin();
        let b = y * 2.005_f32 - x.cos();
        let mut acc = Accumulator1078::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.298_f32 + y.sin();
        let b = y * 8.488_f32 - x.cos();
        let mut acc = Accumulator1078::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.915_f32 + y.sin();
        let b = y * 5.789_f32 - x.cos();
        let mut acc = Accumulator1078::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.528_f32 + y.sin();
        let b = y * 9.152_f32 - x.cos();
        let mut acc = Accumulator1078::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.312_f32 + y.sin();
        let b = y * 2.02_f32 - x.cos();
        let mut acc = Accumulator1078::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.005_f32 + y.sin();
        let b = y * 9.106_f32 - x.cos();
        let mut acc = Accumulator1078::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.112_f32 + y.sin();
        let b = y * 2.962_f32 - x.cos();
        let mut acc = Accumulator1078::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.326_f32 + y.sin();
        let b = y * 6.812_f32 - x.cos();
        let mut acc = Accumulator1078::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.252_f32 + y.sin();
        let b = y * 5.936_f32 - x.cos();
        let mut acc = Accumulator1078::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.868_f32 + y.sin();
        let b = y * 6.029_f32 - x.cos();
        let mut acc = Accumulator1078::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.859_f32 + y.sin();
        let b = y * 0.529_f32 - x.cos();
        let mut acc = Accumulator1078::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.15_f32 + y.sin();
        let b = y * 0.806_f32 - x.cos();
        let mut acc = Accumulator1078::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.284_f32 + y.sin();
        let b = y * 6.245_f32 - x.cos();
        let mut acc = Accumulator1078::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.305_f32 + y.sin();
        let b = y * 4.468_f32 - x.cos();
        let mut acc = Accumulator1078::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1078(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_1078() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1078(total as u64) % 997) as f32;
        total
    }
}

pub mod m1079 {
    use super::*;

    pub struct Accumulator1079<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1079<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.223_f32 + y.sin();
        let b = y * 6.274_f32 - x.cos();
        let mut acc = Accumulator1079::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.819_f32 + y.sin();
        let b = y * 0.789_f32 - x.cos();
        let mut acc = Accumulator1079::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.212_f32 + y.sin();
        let b = y * 1.619_f32 - x.cos();
        let mut acc = Accumulator1079::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.237_f32 + y.sin();
        let b = y * 5.194_f32 - x.cos();
        let mut acc = Accumulator1079::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.799_f32 + y.sin();
        let b = y * 2.45_f32 - x.cos();
        let mut acc = Accumulator1079::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.254_f32 + y.sin();
        let b = y * 2.996_f32 - x.cos();
        let mut acc = Accumulator1079::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.75_f32 + y.sin();
        let b = y * 2.981_f32 - x.cos();
        let mut acc = Accumulator1079::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.55_f32 + y.sin();
        let b = y * 5.17_f32 - x.cos();
        let mut acc = Accumulator1079::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.376_f32 + y.sin();
        let b = y * 4.658_f32 - x.cos();
        let mut acc = Accumulator1079::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.8_f32 + y.sin();
        let b = y * 5.167_f32 - x.cos();
        let mut acc = Accumulator1079::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.546_f32 + y.sin();
        let b = y * 5.829_f32 - x.cos();
        let mut acc = Accumulator1079::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.623_f32 + y.sin();
        let b = y * 6.018_f32 - x.cos();
        let mut acc = Accumulator1079::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.377_f32 + y.sin();
        let b = y * 4.139_f32 - x.cos();
        let mut acc = Accumulator1079::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.877_f32 + y.sin();
        let b = y * 6.311_f32 - x.cos();
        let mut acc = Accumulator1079::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.509_f32 + y.sin();
        let b = y * 0.172_f32 - x.cos();
        let mut acc = Accumulator1079::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.89_f32 + y.sin();
        let b = y * 3.389_f32 - x.cos();
        let mut acc = Accumulator1079::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.519_f32 + y.sin();
        let b = y * 2.541_f32 - x.cos();
        let mut acc = Accumulator1079::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.454_f32 + y.sin();
        let b = y * 9.898_f32 - x.cos();
        let mut acc = Accumulator1079::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.692_f32 + y.sin();
        let b = y * 4.16_f32 - x.cos();
        let mut acc = Accumulator1079::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.173_f32 + y.sin();
        let b = y * 3.868_f32 - x.cos();
        let mut acc = Accumulator1079::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1079(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m1079-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_1079() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1079(total as u64) % 997) as f32;
        total
    }
}

pub mod m1080 {
    use super::*;

    pub struct Accumulator1080<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1080<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.585_f32 + y.sin();
        let b = y * 7.144_f32 - x.cos();
        let mut acc = Accumulator1080::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.2_f32 + y.sin();
        let b = y * 2.339_f32 - x.cos();
        let mut acc = Accumulator1080::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.251_f32 + y.sin();
        let b = y * 9.07_f32 - x.cos();
        let mut acc = Accumulator1080::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.793_f32 + y.sin();
        let b = y * 2.497_f32 - x.cos();
        let mut acc = Accumulator1080::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.763_f32 + y.sin();
        let b = y * 5.442_f32 - x.cos();
        let mut acc = Accumulator1080::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.602_f32 + y.sin();
        let b = y * 2.142_f32 - x.cos();
        let mut acc = Accumulator1080::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.872_f32 + y.sin();
        let b = y * 9.876_f32 - x.cos();
        let mut acc = Accumulator1080::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.595_f32 + y.sin();
        let b = y * 4.233_f32 - x.cos();
        let mut acc = Accumulator1080::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.994_f32 + y.sin();
        let b = y * 7.974_f32 - x.cos();
        let mut acc = Accumulator1080::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.95_f32 + y.sin();
        let b = y * 7.943_f32 - x.cos();
        let mut acc = Accumulator1080::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.433_f32 + y.sin();
        let b = y * 2.568_f32 - x.cos();
        let mut acc = Accumulator1080::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.872_f32 + y.sin();
        let b = y * 0.888_f32 - x.cos();
        let mut acc = Accumulator1080::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.263_f32 + y.sin();
        let b = y * 4.634_f32 - x.cos();
        let mut acc = Accumulator1080::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.92_f32 + y.sin();
        let b = y * 5.767_f32 - x.cos();
        let mut acc = Accumulator1080::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.562_f32 + y.sin();
        let b = y * 4.133_f32 - x.cos();
        let mut acc = Accumulator1080::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.558_f32 + y.sin();
        let b = y * 7.323_f32 - x.cos();
        let mut acc = Accumulator1080::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.829_f32 + y.sin();
        let b = y * 1.479_f32 - x.cos();
        let mut acc = Accumulator1080::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.567_f32 + y.sin();
        let b = y * 6.686_f32 - x.cos();
        let mut acc = Accumulator1080::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.28_f32 + y.sin();
        let b = y * 5.819_f32 - x.cos();
        let mut acc = Accumulator1080::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.356_f32 + y.sin();
        let b = y * 4.252_f32 - x.cos();
        let mut acc = Accumulator1080::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1080(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_1080() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1080(total as u64) % 997) as f32;
        total
    }
}

pub mod m1081 {
    use super::*;

    pub struct Accumulator1081<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1081<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.35_f32 + y.sin();
        let b = y * 5.324_f32 - x.cos();
        let mut acc = Accumulator1081::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.388_f32 + y.sin();
        let b = y * 5.47_f32 - x.cos();
        let mut acc = Accumulator1081::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.459_f32 + y.sin();
        let b = y * 2.738_f32 - x.cos();
        let mut acc = Accumulator1081::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.624_f32 + y.sin();
        let b = y * 0.473_f32 - x.cos();
        let mut acc = Accumulator1081::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.57_f32 + y.sin();
        let b = y * 4.199_f32 - x.cos();
        let mut acc = Accumulator1081::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.578_f32 + y.sin();
        let b = y * 5.636_f32 - x.cos();
        let mut acc = Accumulator1081::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.648_f32 + y.sin();
        let b = y * 9.764_f32 - x.cos();
        let mut acc = Accumulator1081::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.169_f32 + y.sin();
        let b = y * 3.915_f32 - x.cos();
        let mut acc = Accumulator1081::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.127_f32 + y.sin();
        let b = y * 1.534_f32 - x.cos();
        let mut acc = Accumulator1081::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.708_f32 + y.sin();
        let b = y * 4.383_f32 - x.cos();
        let mut acc = Accumulator1081::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.213_f32 + y.sin();
        let b = y * 3.347_f32 - x.cos();
        let mut acc = Accumulator1081::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.565_f32 + y.sin();
        let b = y * 0.96_f32 - x.cos();
        let mut acc = Accumulator1081::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.247_f32 + y.sin();
        let b = y * 7.327_f32 - x.cos();
        let mut acc = Accumulator1081::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.865_f32 + y.sin();
        let b = y * 4.533_f32 - x.cos();
        let mut acc = Accumulator1081::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.577_f32 + y.sin();
        let b = y * 3.614_f32 - x.cos();
        let mut acc = Accumulator1081::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.887_f32 + y.sin();
        let b = y * 0.979_f32 - x.cos();
        let mut acc = Accumulator1081::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.873_f32 + y.sin();
        let b = y * 5.453_f32 - x.cos();
        let mut acc = Accumulator1081::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.936_f32 + y.sin();
        let b = y * 4.58_f32 - x.cos();
        let mut acc = Accumulator1081::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.12_f32 + y.sin();
        let b = y * 7.617_f32 - x.cos();
        let mut acc = Accumulator1081::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.9_f32 + y.sin();
        let b = y * 3.254_f32 - x.cos();
        let mut acc = Accumulator1081::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1081(seed: u64) -> u64 {
        let re = Regex::new(r"m1081-(\d+)").unwrap();
        let hay = format!("m1081-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_1081() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1081(total as u64) % 997) as f32;
        total
    }
}

pub mod m1082 {
    use super::*;

    pub struct Accumulator1082<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1082<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.441_f32 + y.sin();
        let b = y * 2.339_f32 - x.cos();
        let mut acc = Accumulator1082::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.946_f32 + y.sin();
        let b = y * 2.743_f32 - x.cos();
        let mut acc = Accumulator1082::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.324_f32 + y.sin();
        let b = y * 4.953_f32 - x.cos();
        let mut acc = Accumulator1082::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.865_f32 + y.sin();
        let b = y * 1.565_f32 - x.cos();
        let mut acc = Accumulator1082::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.511_f32 + y.sin();
        let b = y * 7.358_f32 - x.cos();
        let mut acc = Accumulator1082::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.213_f32 + y.sin();
        let b = y * 6.45_f32 - x.cos();
        let mut acc = Accumulator1082::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.181_f32 + y.sin();
        let b = y * 9.408_f32 - x.cos();
        let mut acc = Accumulator1082::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.483_f32 + y.sin();
        let b = y * 4.25_f32 - x.cos();
        let mut acc = Accumulator1082::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.033_f32 + y.sin();
        let b = y * 4.858_f32 - x.cos();
        let mut acc = Accumulator1082::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.55_f32 + y.sin();
        let b = y * 0.343_f32 - x.cos();
        let mut acc = Accumulator1082::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.392_f32 + y.sin();
        let b = y * 4.456_f32 - x.cos();
        let mut acc = Accumulator1082::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.974_f32 + y.sin();
        let b = y * 9.47_f32 - x.cos();
        let mut acc = Accumulator1082::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.958_f32 + y.sin();
        let b = y * 8.655_f32 - x.cos();
        let mut acc = Accumulator1082::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.614_f32 + y.sin();
        let b = y * 9.536_f32 - x.cos();
        let mut acc = Accumulator1082::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.718_f32 + y.sin();
        let b = y * 6.056_f32 - x.cos();
        let mut acc = Accumulator1082::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.764_f32 + y.sin();
        let b = y * 7.546_f32 - x.cos();
        let mut acc = Accumulator1082::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.355_f32 + y.sin();
        let b = y * 0.591_f32 - x.cos();
        let mut acc = Accumulator1082::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.338_f32 + y.sin();
        let b = y * 3.034_f32 - x.cos();
        let mut acc = Accumulator1082::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.245_f32 + y.sin();
        let b = y * 2.532_f32 - x.cos();
        let mut acc = Accumulator1082::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.635_f32 + y.sin();
        let b = y * 3.918_f32 - x.cos();
        let mut acc = Accumulator1082::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1082(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_1082() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1082(total as u64) % 997) as f32;
        total
    }
}

pub mod m1083 {
    use super::*;

    pub struct Accumulator1083<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1083<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.466_f32 + y.sin();
        let b = y * 2.757_f32 - x.cos();
        let mut acc = Accumulator1083::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.212_f32 + y.sin();
        let b = y * 5.197_f32 - x.cos();
        let mut acc = Accumulator1083::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.878_f32 + y.sin();
        let b = y * 4.574_f32 - x.cos();
        let mut acc = Accumulator1083::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.95_f32 + y.sin();
        let b = y * 7.903_f32 - x.cos();
        let mut acc = Accumulator1083::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.135_f32 + y.sin();
        let b = y * 0.949_f32 - x.cos();
        let mut acc = Accumulator1083::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.134_f32 + y.sin();
        let b = y * 0.699_f32 - x.cos();
        let mut acc = Accumulator1083::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.713_f32 + y.sin();
        let b = y * 7.606_f32 - x.cos();
        let mut acc = Accumulator1083::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.64_f32 + y.sin();
        let b = y * 7.217_f32 - x.cos();
        let mut acc = Accumulator1083::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.444_f32 + y.sin();
        let b = y * 4.852_f32 - x.cos();
        let mut acc = Accumulator1083::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.75_f32 + y.sin();
        let b = y * 3.369_f32 - x.cos();
        let mut acc = Accumulator1083::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.555_f32 + y.sin();
        let b = y * 8.989_f32 - x.cos();
        let mut acc = Accumulator1083::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.331_f32 + y.sin();
        let b = y * 8.009_f32 - x.cos();
        let mut acc = Accumulator1083::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.892_f32 + y.sin();
        let b = y * 9.077_f32 - x.cos();
        let mut acc = Accumulator1083::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.886_f32 + y.sin();
        let b = y * 0.399_f32 - x.cos();
        let mut acc = Accumulator1083::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.243_f32 + y.sin();
        let b = y * 8.762_f32 - x.cos();
        let mut acc = Accumulator1083::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.026_f32 + y.sin();
        let b = y * 2.77_f32 - x.cos();
        let mut acc = Accumulator1083::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.92_f32 + y.sin();
        let b = y * 6.99_f32 - x.cos();
        let mut acc = Accumulator1083::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.458_f32 + y.sin();
        let b = y * 5.261_f32 - x.cos();
        let mut acc = Accumulator1083::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.277_f32 + y.sin();
        let b = y * 9.634_f32 - x.cos();
        let mut acc = Accumulator1083::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.073_f32 + y.sin();
        let b = y * 9.722_f32 - x.cos();
        let mut acc = Accumulator1083::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1083(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(1083u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_1083() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1083(total as u64) % 997) as f32;
        total
    }
}

pub mod m1084 {
    use super::*;

    pub struct Accumulator1084<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1084<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.649_f32 + y.sin();
        let b = y * 7.235_f32 - x.cos();
        let mut acc = Accumulator1084::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.638_f32 + y.sin();
        let b = y * 8.798_f32 - x.cos();
        let mut acc = Accumulator1084::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.096_f32 + y.sin();
        let b = y * 0.9_f32 - x.cos();
        let mut acc = Accumulator1084::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.212_f32 + y.sin();
        let b = y * 2.274_f32 - x.cos();
        let mut acc = Accumulator1084::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.149_f32 + y.sin();
        let b = y * 6.019_f32 - x.cos();
        let mut acc = Accumulator1084::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.307_f32 + y.sin();
        let b = y * 3.981_f32 - x.cos();
        let mut acc = Accumulator1084::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.778_f32 + y.sin();
        let b = y * 6.94_f32 - x.cos();
        let mut acc = Accumulator1084::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.276_f32 + y.sin();
        let b = y * 8.586_f32 - x.cos();
        let mut acc = Accumulator1084::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.455_f32 + y.sin();
        let b = y * 8.319_f32 - x.cos();
        let mut acc = Accumulator1084::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.499_f32 + y.sin();
        let b = y * 8.847_f32 - x.cos();
        let mut acc = Accumulator1084::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.809_f32 + y.sin();
        let b = y * 3.121_f32 - x.cos();
        let mut acc = Accumulator1084::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.242_f32 + y.sin();
        let b = y * 0.635_f32 - x.cos();
        let mut acc = Accumulator1084::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.836_f32 + y.sin();
        let b = y * 3.074_f32 - x.cos();
        let mut acc = Accumulator1084::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.707_f32 + y.sin();
        let b = y * 3.544_f32 - x.cos();
        let mut acc = Accumulator1084::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.989_f32 + y.sin();
        let b = y * 9.666_f32 - x.cos();
        let mut acc = Accumulator1084::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.572_f32 + y.sin();
        let b = y * 9.185_f32 - x.cos();
        let mut acc = Accumulator1084::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.216_f32 + y.sin();
        let b = y * 8.831_f32 - x.cos();
        let mut acc = Accumulator1084::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.666_f32 + y.sin();
        let b = y * 6.358_f32 - x.cos();
        let mut acc = Accumulator1084::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.439_f32 + y.sin();
        let b = y * 7.31_f32 - x.cos();
        let mut acc = Accumulator1084::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.959_f32 + y.sin();
        let b = y * 6.786_f32 - x.cos();
        let mut acc = Accumulator1084::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1084(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_1084() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1084(total as u64) % 997) as f32;
        total
    }
}

pub mod m1085 {
    use super::*;

    pub struct Accumulator1085<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1085<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.298_f32 + y.sin();
        let b = y * 8.728_f32 - x.cos();
        let mut acc = Accumulator1085::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.985_f32 + y.sin();
        let b = y * 7.666_f32 - x.cos();
        let mut acc = Accumulator1085::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.434_f32 + y.sin();
        let b = y * 8.235_f32 - x.cos();
        let mut acc = Accumulator1085::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.616_f32 + y.sin();
        let b = y * 2.655_f32 - x.cos();
        let mut acc = Accumulator1085::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.484_f32 + y.sin();
        let b = y * 6.894_f32 - x.cos();
        let mut acc = Accumulator1085::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.263_f32 + y.sin();
        let b = y * 0.972_f32 - x.cos();
        let mut acc = Accumulator1085::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.62_f32 + y.sin();
        let b = y * 5.785_f32 - x.cos();
        let mut acc = Accumulator1085::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.929_f32 + y.sin();
        let b = y * 0.591_f32 - x.cos();
        let mut acc = Accumulator1085::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.921_f32 + y.sin();
        let b = y * 1.084_f32 - x.cos();
        let mut acc = Accumulator1085::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.2_f32 + y.sin();
        let b = y * 7.551_f32 - x.cos();
        let mut acc = Accumulator1085::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.301_f32 + y.sin();
        let b = y * 6.324_f32 - x.cos();
        let mut acc = Accumulator1085::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.264_f32 + y.sin();
        let b = y * 9.64_f32 - x.cos();
        let mut acc = Accumulator1085::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.695_f32 + y.sin();
        let b = y * 8.307_f32 - x.cos();
        let mut acc = Accumulator1085::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.656_f32 + y.sin();
        let b = y * 8.245_f32 - x.cos();
        let mut acc = Accumulator1085::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.308_f32 + y.sin();
        let b = y * 9.548_f32 - x.cos();
        let mut acc = Accumulator1085::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.76_f32 + y.sin();
        let b = y * 8.374_f32 - x.cos();
        let mut acc = Accumulator1085::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.604_f32 + y.sin();
        let b = y * 1.567_f32 - x.cos();
        let mut acc = Accumulator1085::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.038_f32 + y.sin();
        let b = y * 1.919_f32 - x.cos();
        let mut acc = Accumulator1085::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.163_f32 + y.sin();
        let b = y * 3.22_f32 - x.cos();
        let mut acc = Accumulator1085::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.652_f32 + y.sin();
        let b = y * 8.604_f32 - x.cos();
        let mut acc = Accumulator1085::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1085(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_1085() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1085(total as u64) % 997) as f32;
        total
    }
}

pub mod m1086 {
    use super::*;

    pub struct Accumulator1086<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1086<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.312_f32 + y.sin();
        let b = y * 2.89_f32 - x.cos();
        let mut acc = Accumulator1086::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.558_f32 + y.sin();
        let b = y * 4.334_f32 - x.cos();
        let mut acc = Accumulator1086::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.709_f32 + y.sin();
        let b = y * 9.866_f32 - x.cos();
        let mut acc = Accumulator1086::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.767_f32 + y.sin();
        let b = y * 2.579_f32 - x.cos();
        let mut acc = Accumulator1086::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.088_f32 + y.sin();
        let b = y * 8.362_f32 - x.cos();
        let mut acc = Accumulator1086::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.546_f32 + y.sin();
        let b = y * 6.773_f32 - x.cos();
        let mut acc = Accumulator1086::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.991_f32 + y.sin();
        let b = y * 6.143_f32 - x.cos();
        let mut acc = Accumulator1086::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.314_f32 + y.sin();
        let b = y * 7.831_f32 - x.cos();
        let mut acc = Accumulator1086::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.496_f32 + y.sin();
        let b = y * 5.241_f32 - x.cos();
        let mut acc = Accumulator1086::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.162_f32 + y.sin();
        let b = y * 4.302_f32 - x.cos();
        let mut acc = Accumulator1086::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.478_f32 + y.sin();
        let b = y * 1.681_f32 - x.cos();
        let mut acc = Accumulator1086::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.938_f32 + y.sin();
        let b = y * 6.956_f32 - x.cos();
        let mut acc = Accumulator1086::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.599_f32 + y.sin();
        let b = y * 8.087_f32 - x.cos();
        let mut acc = Accumulator1086::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.449_f32 + y.sin();
        let b = y * 4.099_f32 - x.cos();
        let mut acc = Accumulator1086::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.19_f32 + y.sin();
        let b = y * 9.015_f32 - x.cos();
        let mut acc = Accumulator1086::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.7_f32 + y.sin();
        let b = y * 9.795_f32 - x.cos();
        let mut acc = Accumulator1086::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.466_f32 + y.sin();
        let b = y * 8.304_f32 - x.cos();
        let mut acc = Accumulator1086::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.371_f32 + y.sin();
        let b = y * 0.921_f32 - x.cos();
        let mut acc = Accumulator1086::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.175_f32 + y.sin();
        let b = y * 0.339_f32 - x.cos();
        let mut acc = Accumulator1086::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.426_f32 + y.sin();
        let b = y * 7.421_f32 - x.cos();
        let mut acc = Accumulator1086::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1086(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m1086-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_1086() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1086(total as u64) % 997) as f32;
        total
    }
}

pub mod m1087 {
    use super::*;

    pub struct Accumulator1087<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1087<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.127_f32 + y.sin();
        let b = y * 1.844_f32 - x.cos();
        let mut acc = Accumulator1087::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.303_f32 + y.sin();
        let b = y * 3.08_f32 - x.cos();
        let mut acc = Accumulator1087::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.156_f32 + y.sin();
        let b = y * 4.517_f32 - x.cos();
        let mut acc = Accumulator1087::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.599_f32 + y.sin();
        let b = y * 6.925_f32 - x.cos();
        let mut acc = Accumulator1087::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.2_f32 + y.sin();
        let b = y * 4.543_f32 - x.cos();
        let mut acc = Accumulator1087::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.412_f32 + y.sin();
        let b = y * 3.736_f32 - x.cos();
        let mut acc = Accumulator1087::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.469_f32 + y.sin();
        let b = y * 3.063_f32 - x.cos();
        let mut acc = Accumulator1087::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.534_f32 + y.sin();
        let b = y * 5.48_f32 - x.cos();
        let mut acc = Accumulator1087::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.559_f32 + y.sin();
        let b = y * 6.114_f32 - x.cos();
        let mut acc = Accumulator1087::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.805_f32 + y.sin();
        let b = y * 5.256_f32 - x.cos();
        let mut acc = Accumulator1087::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.255_f32 + y.sin();
        let b = y * 6.059_f32 - x.cos();
        let mut acc = Accumulator1087::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.741_f32 + y.sin();
        let b = y * 9.397_f32 - x.cos();
        let mut acc = Accumulator1087::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.159_f32 + y.sin();
        let b = y * 4.506_f32 - x.cos();
        let mut acc = Accumulator1087::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.514_f32 + y.sin();
        let b = y * 5.845_f32 - x.cos();
        let mut acc = Accumulator1087::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.708_f32 + y.sin();
        let b = y * 7.055_f32 - x.cos();
        let mut acc = Accumulator1087::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.373_f32 + y.sin();
        let b = y * 4.081_f32 - x.cos();
        let mut acc = Accumulator1087::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.95_f32 + y.sin();
        let b = y * 7.88_f32 - x.cos();
        let mut acc = Accumulator1087::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.585_f32 + y.sin();
        let b = y * 7.534_f32 - x.cos();
        let mut acc = Accumulator1087::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.086_f32 + y.sin();
        let b = y * 1.845_f32 - x.cos();
        let mut acc = Accumulator1087::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.15_f32 + y.sin();
        let b = y * 1.349_f32 - x.cos();
        let mut acc = Accumulator1087::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1087(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_1087() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1087(total as u64) % 997) as f32;
        total
    }
}

pub mod m1088 {
    use super::*;

    pub struct Accumulator1088<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1088<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.22_f32 + y.sin();
        let b = y * 5.272_f32 - x.cos();
        let mut acc = Accumulator1088::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.16_f32 + y.sin();
        let b = y * 8.336_f32 - x.cos();
        let mut acc = Accumulator1088::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.845_f32 + y.sin();
        let b = y * 1.039_f32 - x.cos();
        let mut acc = Accumulator1088::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.709_f32 + y.sin();
        let b = y * 3.529_f32 - x.cos();
        let mut acc = Accumulator1088::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.184_f32 + y.sin();
        let b = y * 0.662_f32 - x.cos();
        let mut acc = Accumulator1088::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.201_f32 + y.sin();
        let b = y * 9.887_f32 - x.cos();
        let mut acc = Accumulator1088::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.57_f32 + y.sin();
        let b = y * 8.714_f32 - x.cos();
        let mut acc = Accumulator1088::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.556_f32 + y.sin();
        let b = y * 6.566_f32 - x.cos();
        let mut acc = Accumulator1088::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.029_f32 + y.sin();
        let b = y * 7.778_f32 - x.cos();
        let mut acc = Accumulator1088::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.069_f32 + y.sin();
        let b = y * 2.084_f32 - x.cos();
        let mut acc = Accumulator1088::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.558_f32 + y.sin();
        let b = y * 7.3_f32 - x.cos();
        let mut acc = Accumulator1088::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.05_f32 + y.sin();
        let b = y * 8.953_f32 - x.cos();
        let mut acc = Accumulator1088::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.682_f32 + y.sin();
        let b = y * 8.248_f32 - x.cos();
        let mut acc = Accumulator1088::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.347_f32 + y.sin();
        let b = y * 3.929_f32 - x.cos();
        let mut acc = Accumulator1088::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.743_f32 + y.sin();
        let b = y * 6.576_f32 - x.cos();
        let mut acc = Accumulator1088::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.073_f32 + y.sin();
        let b = y * 0.213_f32 - x.cos();
        let mut acc = Accumulator1088::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.564_f32 + y.sin();
        let b = y * 0.226_f32 - x.cos();
        let mut acc = Accumulator1088::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.786_f32 + y.sin();
        let b = y * 5.512_f32 - x.cos();
        let mut acc = Accumulator1088::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.224_f32 + y.sin();
        let b = y * 8.818_f32 - x.cos();
        let mut acc = Accumulator1088::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.927_f32 + y.sin();
        let b = y * 4.945_f32 - x.cos();
        let mut acc = Accumulator1088::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1088(seed: u64) -> u64 {
        let re = Regex::new(r"m1088-(\d+)").unwrap();
        let hay = format!("m1088-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_1088() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1088(total as u64) % 997) as f32;
        total
    }
}

pub mod m1089 {
    use super::*;

    pub struct Accumulator1089<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1089<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.744_f32 + y.sin();
        let b = y * 4.217_f32 - x.cos();
        let mut acc = Accumulator1089::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.385_f32 + y.sin();
        let b = y * 7.198_f32 - x.cos();
        let mut acc = Accumulator1089::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.612_f32 + y.sin();
        let b = y * 4.889_f32 - x.cos();
        let mut acc = Accumulator1089::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.507_f32 + y.sin();
        let b = y * 2.381_f32 - x.cos();
        let mut acc = Accumulator1089::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.118_f32 + y.sin();
        let b = y * 5.443_f32 - x.cos();
        let mut acc = Accumulator1089::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.314_f32 + y.sin();
        let b = y * 1.833_f32 - x.cos();
        let mut acc = Accumulator1089::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.129_f32 + y.sin();
        let b = y * 3.603_f32 - x.cos();
        let mut acc = Accumulator1089::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.588_f32 + y.sin();
        let b = y * 5.622_f32 - x.cos();
        let mut acc = Accumulator1089::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.639_f32 + y.sin();
        let b = y * 9.409_f32 - x.cos();
        let mut acc = Accumulator1089::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.306_f32 + y.sin();
        let b = y * 1.504_f32 - x.cos();
        let mut acc = Accumulator1089::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.011_f32 + y.sin();
        let b = y * 2.124_f32 - x.cos();
        let mut acc = Accumulator1089::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.48_f32 + y.sin();
        let b = y * 0.791_f32 - x.cos();
        let mut acc = Accumulator1089::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.824_f32 + y.sin();
        let b = y * 2.145_f32 - x.cos();
        let mut acc = Accumulator1089::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.817_f32 + y.sin();
        let b = y * 7.552_f32 - x.cos();
        let mut acc = Accumulator1089::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.008_f32 + y.sin();
        let b = y * 2.716_f32 - x.cos();
        let mut acc = Accumulator1089::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.953_f32 + y.sin();
        let b = y * 0.744_f32 - x.cos();
        let mut acc = Accumulator1089::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.113_f32 + y.sin();
        let b = y * 1.07_f32 - x.cos();
        let mut acc = Accumulator1089::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.162_f32 + y.sin();
        let b = y * 5.281_f32 - x.cos();
        let mut acc = Accumulator1089::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.889_f32 + y.sin();
        let b = y * 2.66_f32 - x.cos();
        let mut acc = Accumulator1089::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.783_f32 + y.sin();
        let b = y * 3.261_f32 - x.cos();
        let mut acc = Accumulator1089::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1089(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_1089() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1089(total as u64) % 997) as f32;
        total
    }
}

pub mod m1090 {
    use super::*;

    pub struct Accumulator1090<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1090<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.837_f32 + y.sin();
        let b = y * 5.095_f32 - x.cos();
        let mut acc = Accumulator1090::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.109_f32 + y.sin();
        let b = y * 6.859_f32 - x.cos();
        let mut acc = Accumulator1090::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.026_f32 + y.sin();
        let b = y * 9.212_f32 - x.cos();
        let mut acc = Accumulator1090::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.789_f32 + y.sin();
        let b = y * 1.18_f32 - x.cos();
        let mut acc = Accumulator1090::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.654_f32 + y.sin();
        let b = y * 6.74_f32 - x.cos();
        let mut acc = Accumulator1090::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.759_f32 + y.sin();
        let b = y * 3.365_f32 - x.cos();
        let mut acc = Accumulator1090::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.304_f32 + y.sin();
        let b = y * 1.214_f32 - x.cos();
        let mut acc = Accumulator1090::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.853_f32 + y.sin();
        let b = y * 7.575_f32 - x.cos();
        let mut acc = Accumulator1090::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.41_f32 + y.sin();
        let b = y * 2.193_f32 - x.cos();
        let mut acc = Accumulator1090::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.384_f32 + y.sin();
        let b = y * 7.741_f32 - x.cos();
        let mut acc = Accumulator1090::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.623_f32 + y.sin();
        let b = y * 3.967_f32 - x.cos();
        let mut acc = Accumulator1090::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.22_f32 + y.sin();
        let b = y * 7.576_f32 - x.cos();
        let mut acc = Accumulator1090::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.338_f32 + y.sin();
        let b = y * 1.608_f32 - x.cos();
        let mut acc = Accumulator1090::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.246_f32 + y.sin();
        let b = y * 0.987_f32 - x.cos();
        let mut acc = Accumulator1090::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.68_f32 + y.sin();
        let b = y * 6.901_f32 - x.cos();
        let mut acc = Accumulator1090::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.675_f32 + y.sin();
        let b = y * 6.58_f32 - x.cos();
        let mut acc = Accumulator1090::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.949_f32 + y.sin();
        let b = y * 7.563_f32 - x.cos();
        let mut acc = Accumulator1090::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.385_f32 + y.sin();
        let b = y * 4.602_f32 - x.cos();
        let mut acc = Accumulator1090::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.198_f32 + y.sin();
        let b = y * 0.789_f32 - x.cos();
        let mut acc = Accumulator1090::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.25_f32 + y.sin();
        let b = y * 1.395_f32 - x.cos();
        let mut acc = Accumulator1090::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1090(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(1090u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_1090() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1090(total as u64) % 997) as f32;
        total
    }
}

pub mod m1091 {
    use super::*;

    pub struct Accumulator1091<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1091<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.561_f32 + y.sin();
        let b = y * 8.265_f32 - x.cos();
        let mut acc = Accumulator1091::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.257_f32 + y.sin();
        let b = y * 7.26_f32 - x.cos();
        let mut acc = Accumulator1091::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.172_f32 + y.sin();
        let b = y * 3.796_f32 - x.cos();
        let mut acc = Accumulator1091::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.042_f32 + y.sin();
        let b = y * 8.909_f32 - x.cos();
        let mut acc = Accumulator1091::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.698_f32 + y.sin();
        let b = y * 7.716_f32 - x.cos();
        let mut acc = Accumulator1091::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.69_f32 + y.sin();
        let b = y * 4.256_f32 - x.cos();
        let mut acc = Accumulator1091::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.872_f32 + y.sin();
        let b = y * 9.008_f32 - x.cos();
        let mut acc = Accumulator1091::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.539_f32 + y.sin();
        let b = y * 6.611_f32 - x.cos();
        let mut acc = Accumulator1091::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.56_f32 + y.sin();
        let b = y * 7.935_f32 - x.cos();
        let mut acc = Accumulator1091::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.307_f32 + y.sin();
        let b = y * 9.667_f32 - x.cos();
        let mut acc = Accumulator1091::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.581_f32 + y.sin();
        let b = y * 8.012_f32 - x.cos();
        let mut acc = Accumulator1091::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.29_f32 + y.sin();
        let b = y * 0.631_f32 - x.cos();
        let mut acc = Accumulator1091::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.775_f32 + y.sin();
        let b = y * 2.358_f32 - x.cos();
        let mut acc = Accumulator1091::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.324_f32 + y.sin();
        let b = y * 0.379_f32 - x.cos();
        let mut acc = Accumulator1091::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.78_f32 + y.sin();
        let b = y * 7.063_f32 - x.cos();
        let mut acc = Accumulator1091::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.338_f32 + y.sin();
        let b = y * 4.485_f32 - x.cos();
        let mut acc = Accumulator1091::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.725_f32 + y.sin();
        let b = y * 0.763_f32 - x.cos();
        let mut acc = Accumulator1091::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.757_f32 + y.sin();
        let b = y * 9.327_f32 - x.cos();
        let mut acc = Accumulator1091::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.176_f32 + y.sin();
        let b = y * 5.764_f32 - x.cos();
        let mut acc = Accumulator1091::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.316_f32 + y.sin();
        let b = y * 5.05_f32 - x.cos();
        let mut acc = Accumulator1091::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1091(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_1091() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1091(total as u64) % 997) as f32;
        total
    }
}

pub mod m1092 {
    use super::*;

    pub struct Accumulator1092<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1092<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.539_f32 + y.sin();
        let b = y * 2.047_f32 - x.cos();
        let mut acc = Accumulator1092::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.431_f32 + y.sin();
        let b = y * 0.312_f32 - x.cos();
        let mut acc = Accumulator1092::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.233_f32 + y.sin();
        let b = y * 5.258_f32 - x.cos();
        let mut acc = Accumulator1092::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.748_f32 + y.sin();
        let b = y * 1.25_f32 - x.cos();
        let mut acc = Accumulator1092::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.158_f32 + y.sin();
        let b = y * 0.485_f32 - x.cos();
        let mut acc = Accumulator1092::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.142_f32 + y.sin();
        let b = y * 9.298_f32 - x.cos();
        let mut acc = Accumulator1092::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.517_f32 + y.sin();
        let b = y * 5.788_f32 - x.cos();
        let mut acc = Accumulator1092::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.011_f32 + y.sin();
        let b = y * 6.69_f32 - x.cos();
        let mut acc = Accumulator1092::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.094_f32 + y.sin();
        let b = y * 9.705_f32 - x.cos();
        let mut acc = Accumulator1092::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.586_f32 + y.sin();
        let b = y * 0.691_f32 - x.cos();
        let mut acc = Accumulator1092::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.842_f32 + y.sin();
        let b = y * 7.898_f32 - x.cos();
        let mut acc = Accumulator1092::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.18_f32 + y.sin();
        let b = y * 5.284_f32 - x.cos();
        let mut acc = Accumulator1092::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.525_f32 + y.sin();
        let b = y * 8.115_f32 - x.cos();
        let mut acc = Accumulator1092::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.828_f32 + y.sin();
        let b = y * 9.654_f32 - x.cos();
        let mut acc = Accumulator1092::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.823_f32 + y.sin();
        let b = y * 8.184_f32 - x.cos();
        let mut acc = Accumulator1092::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.604_f32 + y.sin();
        let b = y * 4.19_f32 - x.cos();
        let mut acc = Accumulator1092::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.312_f32 + y.sin();
        let b = y * 3.272_f32 - x.cos();
        let mut acc = Accumulator1092::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.249_f32 + y.sin();
        let b = y * 2.448_f32 - x.cos();
        let mut acc = Accumulator1092::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.844_f32 + y.sin();
        let b = y * 5.299_f32 - x.cos();
        let mut acc = Accumulator1092::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.026_f32 + y.sin();
        let b = y * 2.664_f32 - x.cos();
        let mut acc = Accumulator1092::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1092(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_1092() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1092(total as u64) % 997) as f32;
        total
    }
}

pub mod m1093 {
    use super::*;

    pub struct Accumulator1093<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1093<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.982_f32 + y.sin();
        let b = y * 9.011_f32 - x.cos();
        let mut acc = Accumulator1093::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.811_f32 + y.sin();
        let b = y * 0.722_f32 - x.cos();
        let mut acc = Accumulator1093::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.42_f32 + y.sin();
        let b = y * 2.026_f32 - x.cos();
        let mut acc = Accumulator1093::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.125_f32 + y.sin();
        let b = y * 8.946_f32 - x.cos();
        let mut acc = Accumulator1093::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.349_f32 + y.sin();
        let b = y * 2.807_f32 - x.cos();
        let mut acc = Accumulator1093::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.988_f32 + y.sin();
        let b = y * 2.039_f32 - x.cos();
        let mut acc = Accumulator1093::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.305_f32 + y.sin();
        let b = y * 3.042_f32 - x.cos();
        let mut acc = Accumulator1093::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.571_f32 + y.sin();
        let b = y * 1.946_f32 - x.cos();
        let mut acc = Accumulator1093::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.255_f32 + y.sin();
        let b = y * 2.64_f32 - x.cos();
        let mut acc = Accumulator1093::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.562_f32 + y.sin();
        let b = y * 3.834_f32 - x.cos();
        let mut acc = Accumulator1093::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.575_f32 + y.sin();
        let b = y * 2.352_f32 - x.cos();
        let mut acc = Accumulator1093::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.56_f32 + y.sin();
        let b = y * 8.124_f32 - x.cos();
        let mut acc = Accumulator1093::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.888_f32 + y.sin();
        let b = y * 3.625_f32 - x.cos();
        let mut acc = Accumulator1093::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.527_f32 + y.sin();
        let b = y * 7.725_f32 - x.cos();
        let mut acc = Accumulator1093::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.674_f32 + y.sin();
        let b = y * 1.111_f32 - x.cos();
        let mut acc = Accumulator1093::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.177_f32 + y.sin();
        let b = y * 2.91_f32 - x.cos();
        let mut acc = Accumulator1093::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.844_f32 + y.sin();
        let b = y * 7.583_f32 - x.cos();
        let mut acc = Accumulator1093::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.478_f32 + y.sin();
        let b = y * 2.441_f32 - x.cos();
        let mut acc = Accumulator1093::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.265_f32 + y.sin();
        let b = y * 7.568_f32 - x.cos();
        let mut acc = Accumulator1093::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.633_f32 + y.sin();
        let b = y * 7.561_f32 - x.cos();
        let mut acc = Accumulator1093::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1093(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m1093-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_1093() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1093(total as u64) % 997) as f32;
        total
    }
}

pub mod m1094 {
    use super::*;

    pub struct Accumulator1094<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1094<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.45_f32 + y.sin();
        let b = y * 6.238_f32 - x.cos();
        let mut acc = Accumulator1094::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.264_f32 + y.sin();
        let b = y * 7.924_f32 - x.cos();
        let mut acc = Accumulator1094::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.758_f32 + y.sin();
        let b = y * 8.039_f32 - x.cos();
        let mut acc = Accumulator1094::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.109_f32 + y.sin();
        let b = y * 7.398_f32 - x.cos();
        let mut acc = Accumulator1094::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.58_f32 + y.sin();
        let b = y * 3.481_f32 - x.cos();
        let mut acc = Accumulator1094::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.933_f32 + y.sin();
        let b = y * 6.558_f32 - x.cos();
        let mut acc = Accumulator1094::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.291_f32 + y.sin();
        let b = y * 3.03_f32 - x.cos();
        let mut acc = Accumulator1094::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.358_f32 + y.sin();
        let b = y * 5.108_f32 - x.cos();
        let mut acc = Accumulator1094::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.075_f32 + y.sin();
        let b = y * 4.364_f32 - x.cos();
        let mut acc = Accumulator1094::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.536_f32 + y.sin();
        let b = y * 1.545_f32 - x.cos();
        let mut acc = Accumulator1094::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.762_f32 + y.sin();
        let b = y * 2.44_f32 - x.cos();
        let mut acc = Accumulator1094::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.717_f32 + y.sin();
        let b = y * 4.415_f32 - x.cos();
        let mut acc = Accumulator1094::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.421_f32 + y.sin();
        let b = y * 1.711_f32 - x.cos();
        let mut acc = Accumulator1094::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.051_f32 + y.sin();
        let b = y * 1.772_f32 - x.cos();
        let mut acc = Accumulator1094::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.955_f32 + y.sin();
        let b = y * 9.064_f32 - x.cos();
        let mut acc = Accumulator1094::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.663_f32 + y.sin();
        let b = y * 8.325_f32 - x.cos();
        let mut acc = Accumulator1094::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.081_f32 + y.sin();
        let b = y * 9.77_f32 - x.cos();
        let mut acc = Accumulator1094::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.334_f32 + y.sin();
        let b = y * 8.033_f32 - x.cos();
        let mut acc = Accumulator1094::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.01_f32 + y.sin();
        let b = y * 1.747_f32 - x.cos();
        let mut acc = Accumulator1094::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.23_f32 + y.sin();
        let b = y * 4.158_f32 - x.cos();
        let mut acc = Accumulator1094::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1094(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_1094() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1094(total as u64) % 997) as f32;
        total
    }
}

pub mod m1095 {
    use super::*;

    pub struct Accumulator1095<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1095<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.915_f32 + y.sin();
        let b = y * 2.535_f32 - x.cos();
        let mut acc = Accumulator1095::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.61_f32 + y.sin();
        let b = y * 0.246_f32 - x.cos();
        let mut acc = Accumulator1095::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.095_f32 + y.sin();
        let b = y * 3.228_f32 - x.cos();
        let mut acc = Accumulator1095::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.681_f32 + y.sin();
        let b = y * 1.697_f32 - x.cos();
        let mut acc = Accumulator1095::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.433_f32 + y.sin();
        let b = y * 3.225_f32 - x.cos();
        let mut acc = Accumulator1095::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.504_f32 + y.sin();
        let b = y * 1.632_f32 - x.cos();
        let mut acc = Accumulator1095::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.813_f32 + y.sin();
        let b = y * 0.614_f32 - x.cos();
        let mut acc = Accumulator1095::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.566_f32 + y.sin();
        let b = y * 3.981_f32 - x.cos();
        let mut acc = Accumulator1095::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.189_f32 + y.sin();
        let b = y * 9.329_f32 - x.cos();
        let mut acc = Accumulator1095::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.621_f32 + y.sin();
        let b = y * 9.224_f32 - x.cos();
        let mut acc = Accumulator1095::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.658_f32 + y.sin();
        let b = y * 6.505_f32 - x.cos();
        let mut acc = Accumulator1095::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.916_f32 + y.sin();
        let b = y * 1.001_f32 - x.cos();
        let mut acc = Accumulator1095::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.592_f32 + y.sin();
        let b = y * 3.768_f32 - x.cos();
        let mut acc = Accumulator1095::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.39_f32 + y.sin();
        let b = y * 5.912_f32 - x.cos();
        let mut acc = Accumulator1095::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.471_f32 + y.sin();
        let b = y * 4.453_f32 - x.cos();
        let mut acc = Accumulator1095::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.839_f32 + y.sin();
        let b = y * 8.405_f32 - x.cos();
        let mut acc = Accumulator1095::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.333_f32 + y.sin();
        let b = y * 1.176_f32 - x.cos();
        let mut acc = Accumulator1095::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.057_f32 + y.sin();
        let b = y * 7.315_f32 - x.cos();
        let mut acc = Accumulator1095::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.319_f32 + y.sin();
        let b = y * 5.704_f32 - x.cos();
        let mut acc = Accumulator1095::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.197_f32 + y.sin();
        let b = y * 8.546_f32 - x.cos();
        let mut acc = Accumulator1095::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1095(seed: u64) -> u64 {
        let re = Regex::new(r"m1095-(\d+)").unwrap();
        let hay = format!("m1095-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_1095() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1095(total as u64) % 997) as f32;
        total
    }
}

pub mod m1096 {
    use super::*;

    pub struct Accumulator1096<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1096<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.665_f32 + y.sin();
        let b = y * 0.333_f32 - x.cos();
        let mut acc = Accumulator1096::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.084_f32 + y.sin();
        let b = y * 1.811_f32 - x.cos();
        let mut acc = Accumulator1096::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.641_f32 + y.sin();
        let b = y * 8.38_f32 - x.cos();
        let mut acc = Accumulator1096::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.049_f32 + y.sin();
        let b = y * 8.716_f32 - x.cos();
        let mut acc = Accumulator1096::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.742_f32 + y.sin();
        let b = y * 3.644_f32 - x.cos();
        let mut acc = Accumulator1096::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.515_f32 + y.sin();
        let b = y * 1.423_f32 - x.cos();
        let mut acc = Accumulator1096::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.526_f32 + y.sin();
        let b = y * 0.534_f32 - x.cos();
        let mut acc = Accumulator1096::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.257_f32 + y.sin();
        let b = y * 3.139_f32 - x.cos();
        let mut acc = Accumulator1096::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.455_f32 + y.sin();
        let b = y * 3.824_f32 - x.cos();
        let mut acc = Accumulator1096::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.306_f32 + y.sin();
        let b = y * 8.638_f32 - x.cos();
        let mut acc = Accumulator1096::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.505_f32 + y.sin();
        let b = y * 6.639_f32 - x.cos();
        let mut acc = Accumulator1096::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.189_f32 + y.sin();
        let b = y * 1.725_f32 - x.cos();
        let mut acc = Accumulator1096::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.648_f32 + y.sin();
        let b = y * 0.608_f32 - x.cos();
        let mut acc = Accumulator1096::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.564_f32 + y.sin();
        let b = y * 4.904_f32 - x.cos();
        let mut acc = Accumulator1096::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.285_f32 + y.sin();
        let b = y * 2.696_f32 - x.cos();
        let mut acc = Accumulator1096::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.456_f32 + y.sin();
        let b = y * 8.138_f32 - x.cos();
        let mut acc = Accumulator1096::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.78_f32 + y.sin();
        let b = y * 7.865_f32 - x.cos();
        let mut acc = Accumulator1096::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.539_f32 + y.sin();
        let b = y * 3.555_f32 - x.cos();
        let mut acc = Accumulator1096::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.188_f32 + y.sin();
        let b = y * 6.951_f32 - x.cos();
        let mut acc = Accumulator1096::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.99_f32 + y.sin();
        let b = y * 8.013_f32 - x.cos();
        let mut acc = Accumulator1096::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1096(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_1096() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1096(total as u64) % 997) as f32;
        total
    }
}

pub mod m1097 {
    use super::*;

    pub struct Accumulator1097<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1097<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.114_f32 + y.sin();
        let b = y * 2.646_f32 - x.cos();
        let mut acc = Accumulator1097::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.822_f32 + y.sin();
        let b = y * 4.569_f32 - x.cos();
        let mut acc = Accumulator1097::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.342_f32 + y.sin();
        let b = y * 6.666_f32 - x.cos();
        let mut acc = Accumulator1097::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.378_f32 + y.sin();
        let b = y * 1.26_f32 - x.cos();
        let mut acc = Accumulator1097::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.848_f32 + y.sin();
        let b = y * 1.693_f32 - x.cos();
        let mut acc = Accumulator1097::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.176_f32 + y.sin();
        let b = y * 5.623_f32 - x.cos();
        let mut acc = Accumulator1097::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.756_f32 + y.sin();
        let b = y * 6.88_f32 - x.cos();
        let mut acc = Accumulator1097::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.541_f32 + y.sin();
        let b = y * 1.661_f32 - x.cos();
        let mut acc = Accumulator1097::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.378_f32 + y.sin();
        let b = y * 6.728_f32 - x.cos();
        let mut acc = Accumulator1097::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.656_f32 + y.sin();
        let b = y * 7.298_f32 - x.cos();
        let mut acc = Accumulator1097::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.833_f32 + y.sin();
        let b = y * 2.937_f32 - x.cos();
        let mut acc = Accumulator1097::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.696_f32 + y.sin();
        let b = y * 9.434_f32 - x.cos();
        let mut acc = Accumulator1097::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.089_f32 + y.sin();
        let b = y * 4.554_f32 - x.cos();
        let mut acc = Accumulator1097::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.482_f32 + y.sin();
        let b = y * 7.815_f32 - x.cos();
        let mut acc = Accumulator1097::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.984_f32 + y.sin();
        let b = y * 3.27_f32 - x.cos();
        let mut acc = Accumulator1097::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.736_f32 + y.sin();
        let b = y * 5.287_f32 - x.cos();
        let mut acc = Accumulator1097::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.807_f32 + y.sin();
        let b = y * 8.669_f32 - x.cos();
        let mut acc = Accumulator1097::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.658_f32 + y.sin();
        let b = y * 2.769_f32 - x.cos();
        let mut acc = Accumulator1097::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.547_f32 + y.sin();
        let b = y * 2.704_f32 - x.cos();
        let mut acc = Accumulator1097::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.998_f32 + y.sin();
        let b = y * 9.65_f32 - x.cos();
        let mut acc = Accumulator1097::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1097(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(1097u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_1097() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1097(total as u64) % 997) as f32;
        total
    }
}

pub mod m1098 {
    use super::*;

    pub struct Accumulator1098<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1098<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.256_f32 + y.sin();
        let b = y * 4.836_f32 - x.cos();
        let mut acc = Accumulator1098::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.536_f32 + y.sin();
        let b = y * 9.776_f32 - x.cos();
        let mut acc = Accumulator1098::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.765_f32 + y.sin();
        let b = y * 5.433_f32 - x.cos();
        let mut acc = Accumulator1098::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.3_f32 + y.sin();
        let b = y * 1.989_f32 - x.cos();
        let mut acc = Accumulator1098::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.099_f32 + y.sin();
        let b = y * 5.236_f32 - x.cos();
        let mut acc = Accumulator1098::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.433_f32 + y.sin();
        let b = y * 1.599_f32 - x.cos();
        let mut acc = Accumulator1098::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.706_f32 + y.sin();
        let b = y * 5.375_f32 - x.cos();
        let mut acc = Accumulator1098::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.154_f32 + y.sin();
        let b = y * 8.415_f32 - x.cos();
        let mut acc = Accumulator1098::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.61_f32 + y.sin();
        let b = y * 6.97_f32 - x.cos();
        let mut acc = Accumulator1098::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.283_f32 + y.sin();
        let b = y * 1.039_f32 - x.cos();
        let mut acc = Accumulator1098::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.781_f32 + y.sin();
        let b = y * 0.827_f32 - x.cos();
        let mut acc = Accumulator1098::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.058_f32 + y.sin();
        let b = y * 4.415_f32 - x.cos();
        let mut acc = Accumulator1098::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.301_f32 + y.sin();
        let b = y * 0.726_f32 - x.cos();
        let mut acc = Accumulator1098::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.718_f32 + y.sin();
        let b = y * 3.373_f32 - x.cos();
        let mut acc = Accumulator1098::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.867_f32 + y.sin();
        let b = y * 4.55_f32 - x.cos();
        let mut acc = Accumulator1098::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.739_f32 + y.sin();
        let b = y * 2.293_f32 - x.cos();
        let mut acc = Accumulator1098::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.077_f32 + y.sin();
        let b = y * 0.134_f32 - x.cos();
        let mut acc = Accumulator1098::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.843_f32 + y.sin();
        let b = y * 6.16_f32 - x.cos();
        let mut acc = Accumulator1098::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.067_f32 + y.sin();
        let b = y * 1.035_f32 - x.cos();
        let mut acc = Accumulator1098::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.129_f32 + y.sin();
        let b = y * 7.304_f32 - x.cos();
        let mut acc = Accumulator1098::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1098(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_1098() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1098(total as u64) % 997) as f32;
        total
    }
}

pub mod m1099 {
    use super::*;

    pub struct Accumulator1099<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator1099<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.706_f32 + y.sin();
        let b = y * 8.165_f32 - x.cos();
        let mut acc = Accumulator1099::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.628_f32 + y.sin();
        let b = y * 7.569_f32 - x.cos();
        let mut acc = Accumulator1099::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.972_f32 + y.sin();
        let b = y * 0.753_f32 - x.cos();
        let mut acc = Accumulator1099::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.881_f32 + y.sin();
        let b = y * 3.357_f32 - x.cos();
        let mut acc = Accumulator1099::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.815_f32 + y.sin();
        let b = y * 2.405_f32 - x.cos();
        let mut acc = Accumulator1099::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.732_f32 + y.sin();
        let b = y * 3.232_f32 - x.cos();
        let mut acc = Accumulator1099::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.126_f32 + y.sin();
        let b = y * 3.703_f32 - x.cos();
        let mut acc = Accumulator1099::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.021_f32 + y.sin();
        let b = y * 6.666_f32 - x.cos();
        let mut acc = Accumulator1099::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.074_f32 + y.sin();
        let b = y * 9.493_f32 - x.cos();
        let mut acc = Accumulator1099::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.736_f32 + y.sin();
        let b = y * 6.149_f32 - x.cos();
        let mut acc = Accumulator1099::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.514_f32 + y.sin();
        let b = y * 8.952_f32 - x.cos();
        let mut acc = Accumulator1099::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.93_f32 + y.sin();
        let b = y * 7.484_f32 - x.cos();
        let mut acc = Accumulator1099::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.493_f32 + y.sin();
        let b = y * 7.174_f32 - x.cos();
        let mut acc = Accumulator1099::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.429_f32 + y.sin();
        let b = y * 4.621_f32 - x.cos();
        let mut acc = Accumulator1099::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.003_f32 + y.sin();
        let b = y * 3.164_f32 - x.cos();
        let mut acc = Accumulator1099::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.525_f32 + y.sin();
        let b = y * 0.217_f32 - x.cos();
        let mut acc = Accumulator1099::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.972_f32 + y.sin();
        let b = y * 8.778_f32 - x.cos();
        let mut acc = Accumulator1099::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.581_f32 + y.sin();
        let b = y * 3.041_f32 - x.cos();
        let mut acc = Accumulator1099::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.854_f32 + y.sin();
        let b = y * 8.128_f32 - x.cos();
        let mut acc = Accumulator1099::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.907_f32 + y.sin();
        let b = y * 7.303_f32 - x.cos();
        let mut acc = Accumulator1099::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_1099(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_1099() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_1099(total as u64) % 997) as f32;
        total
    }
}

pub fn touch_bulk_10() -> f32 {
    let mut total = 0.0_f32;
    total += m1000::run_all_1000();
    total += m1001::run_all_1001();
    total += m1002::run_all_1002();
    total += m1003::run_all_1003();
    total += m1004::run_all_1004();
    total += m1005::run_all_1005();
    total += m1006::run_all_1006();
    total += m1007::run_all_1007();
    total += m1008::run_all_1008();
    total += m1009::run_all_1009();
    total += m1010::run_all_1010();
    total += m1011::run_all_1011();
    total += m1012::run_all_1012();
    total += m1013::run_all_1013();
    total += m1014::run_all_1014();
    total += m1015::run_all_1015();
    total += m1016::run_all_1016();
    total += m1017::run_all_1017();
    total += m1018::run_all_1018();
    total += m1019::run_all_1019();
    total += m1020::run_all_1020();
    total += m1021::run_all_1021();
    total += m1022::run_all_1022();
    total += m1023::run_all_1023();
    total += m1024::run_all_1024();
    total += m1025::run_all_1025();
    total += m1026::run_all_1026();
    total += m1027::run_all_1027();
    total += m1028::run_all_1028();
    total += m1029::run_all_1029();
    total += m1030::run_all_1030();
    total += m1031::run_all_1031();
    total += m1032::run_all_1032();
    total += m1033::run_all_1033();
    total += m1034::run_all_1034();
    total += m1035::run_all_1035();
    total += m1036::run_all_1036();
    total += m1037::run_all_1037();
    total += m1038::run_all_1038();
    total += m1039::run_all_1039();
    total += m1040::run_all_1040();
    total += m1041::run_all_1041();
    total += m1042::run_all_1042();
    total += m1043::run_all_1043();
    total += m1044::run_all_1044();
    total += m1045::run_all_1045();
    total += m1046::run_all_1046();
    total += m1047::run_all_1047();
    total += m1048::run_all_1048();
    total += m1049::run_all_1049();
    total += m1050::run_all_1050();
    total += m1051::run_all_1051();
    total += m1052::run_all_1052();
    total += m1053::run_all_1053();
    total += m1054::run_all_1054();
    total += m1055::run_all_1055();
    total += m1056::run_all_1056();
    total += m1057::run_all_1057();
    total += m1058::run_all_1058();
    total += m1059::run_all_1059();
    total += m1060::run_all_1060();
    total += m1061::run_all_1061();
    total += m1062::run_all_1062();
    total += m1063::run_all_1063();
    total += m1064::run_all_1064();
    total += m1065::run_all_1065();
    total += m1066::run_all_1066();
    total += m1067::run_all_1067();
    total += m1068::run_all_1068();
    total += m1069::run_all_1069();
    total += m1070::run_all_1070();
    total += m1071::run_all_1071();
    total += m1072::run_all_1072();
    total += m1073::run_all_1073();
    total += m1074::run_all_1074();
    total += m1075::run_all_1075();
    total += m1076::run_all_1076();
    total += m1077::run_all_1077();
    total += m1078::run_all_1078();
    total += m1079::run_all_1079();
    total += m1080::run_all_1080();
    total += m1081::run_all_1081();
    total += m1082::run_all_1082();
    total += m1083::run_all_1083();
    total += m1084::run_all_1084();
    total += m1085::run_all_1085();
    total += m1086::run_all_1086();
    total += m1087::run_all_1087();
    total += m1088::run_all_1088();
    total += m1089::run_all_1089();
    total += m1090::run_all_1090();
    total += m1091::run_all_1091();
    total += m1092::run_all_1092();
    total += m1093::run_all_1093();
    total += m1094::run_all_1094();
    total += m1095::run_all_1095();
    total += m1096::run_all_1096();
    total += m1097::run_all_1097();
    total += m1098::run_all_1098();
    total += m1099::run_all_1099();
    total
}
