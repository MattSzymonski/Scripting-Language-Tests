//! Auto-generated bulk module (file 9) - exists purely to make `game_rs`
//! heavier for a hot-patch speed experiment. Real arithmetic, real generics
//! (so monomorphization isn't free), and real use of a rotating set of
//! external crates so none of them are dead code. `touch_bulk_9()` is
//! called once from `game::init` so nothing here gets stripped as unreachable.

use rand::{Rng, SeedableRng};
use sha2::{Digest, Sha256};
use noise::{NoiseFn, Perlin};
use regex::Regex;
use itertools::Itertools;
use uuid::Uuid;
use chrono::Utc;


pub mod m900 {
    use super::*;

    pub struct Accumulator900<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator900<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.422_f32 + y.sin();
        let b = y * 6.199_f32 - x.cos();
        let mut acc = Accumulator900::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.954_f32 + y.sin();
        let b = y * 5.288_f32 - x.cos();
        let mut acc = Accumulator900::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.894_f32 + y.sin();
        let b = y * 0.795_f32 - x.cos();
        let mut acc = Accumulator900::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.158_f32 + y.sin();
        let b = y * 2.444_f32 - x.cos();
        let mut acc = Accumulator900::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.819_f32 + y.sin();
        let b = y * 6.868_f32 - x.cos();
        let mut acc = Accumulator900::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.162_f32 + y.sin();
        let b = y * 0.963_f32 - x.cos();
        let mut acc = Accumulator900::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.565_f32 + y.sin();
        let b = y * 0.578_f32 - x.cos();
        let mut acc = Accumulator900::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.368_f32 + y.sin();
        let b = y * 4.06_f32 - x.cos();
        let mut acc = Accumulator900::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.256_f32 + y.sin();
        let b = y * 2.99_f32 - x.cos();
        let mut acc = Accumulator900::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.45_f32 + y.sin();
        let b = y * 0.857_f32 - x.cos();
        let mut acc = Accumulator900::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.503_f32 + y.sin();
        let b = y * 5.249_f32 - x.cos();
        let mut acc = Accumulator900::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.114_f32 + y.sin();
        let b = y * 8.199_f32 - x.cos();
        let mut acc = Accumulator900::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.047_f32 + y.sin();
        let b = y * 1.938_f32 - x.cos();
        let mut acc = Accumulator900::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.558_f32 + y.sin();
        let b = y * 7.264_f32 - x.cos();
        let mut acc = Accumulator900::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.66_f32 + y.sin();
        let b = y * 1.519_f32 - x.cos();
        let mut acc = Accumulator900::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.731_f32 + y.sin();
        let b = y * 2.623_f32 - x.cos();
        let mut acc = Accumulator900::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.723_f32 + y.sin();
        let b = y * 2.071_f32 - x.cos();
        let mut acc = Accumulator900::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.858_f32 + y.sin();
        let b = y * 1.114_f32 - x.cos();
        let mut acc = Accumulator900::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.784_f32 + y.sin();
        let b = y * 5.41_f32 - x.cos();
        let mut acc = Accumulator900::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.873_f32 + y.sin();
        let b = y * 9.815_f32 - x.cos();
        let mut acc = Accumulator900::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_900(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_900() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_900(total as u64) % 997) as f32;
        total
    }
}

pub mod m901 {
    use super::*;

    pub struct Accumulator901<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator901<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.609_f32 + y.sin();
        let b = y * 7.706_f32 - x.cos();
        let mut acc = Accumulator901::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.477_f32 + y.sin();
        let b = y * 5.347_f32 - x.cos();
        let mut acc = Accumulator901::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.342_f32 + y.sin();
        let b = y * 7.915_f32 - x.cos();
        let mut acc = Accumulator901::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.958_f32 + y.sin();
        let b = y * 9.399_f32 - x.cos();
        let mut acc = Accumulator901::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.817_f32 + y.sin();
        let b = y * 8.732_f32 - x.cos();
        let mut acc = Accumulator901::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.734_f32 + y.sin();
        let b = y * 7.53_f32 - x.cos();
        let mut acc = Accumulator901::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.117_f32 + y.sin();
        let b = y * 7.016_f32 - x.cos();
        let mut acc = Accumulator901::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.384_f32 + y.sin();
        let b = y * 1.683_f32 - x.cos();
        let mut acc = Accumulator901::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.319_f32 + y.sin();
        let b = y * 1.317_f32 - x.cos();
        let mut acc = Accumulator901::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.613_f32 + y.sin();
        let b = y * 3.307_f32 - x.cos();
        let mut acc = Accumulator901::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.06_f32 + y.sin();
        let b = y * 6.376_f32 - x.cos();
        let mut acc = Accumulator901::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.655_f32 + y.sin();
        let b = y * 2.199_f32 - x.cos();
        let mut acc = Accumulator901::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.471_f32 + y.sin();
        let b = y * 5.165_f32 - x.cos();
        let mut acc = Accumulator901::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.139_f32 + y.sin();
        let b = y * 3.605_f32 - x.cos();
        let mut acc = Accumulator901::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.013_f32 + y.sin();
        let b = y * 6.443_f32 - x.cos();
        let mut acc = Accumulator901::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.873_f32 + y.sin();
        let b = y * 1.345_f32 - x.cos();
        let mut acc = Accumulator901::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.673_f32 + y.sin();
        let b = y * 2.474_f32 - x.cos();
        let mut acc = Accumulator901::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.81_f32 + y.sin();
        let b = y * 8.154_f32 - x.cos();
        let mut acc = Accumulator901::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.155_f32 + y.sin();
        let b = y * 3.301_f32 - x.cos();
        let mut acc = Accumulator901::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.824_f32 + y.sin();
        let b = y * 5.906_f32 - x.cos();
        let mut acc = Accumulator901::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_901(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(901u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_901() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_901(total as u64) % 997) as f32;
        total
    }
}

pub mod m902 {
    use super::*;

    pub struct Accumulator902<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator902<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.246_f32 + y.sin();
        let b = y * 9.205_f32 - x.cos();
        let mut acc = Accumulator902::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.819_f32 + y.sin();
        let b = y * 5.574_f32 - x.cos();
        let mut acc = Accumulator902::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.741_f32 + y.sin();
        let b = y * 8.992_f32 - x.cos();
        let mut acc = Accumulator902::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.98_f32 + y.sin();
        let b = y * 0.271_f32 - x.cos();
        let mut acc = Accumulator902::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.268_f32 + y.sin();
        let b = y * 2.751_f32 - x.cos();
        let mut acc = Accumulator902::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.505_f32 + y.sin();
        let b = y * 5.057_f32 - x.cos();
        let mut acc = Accumulator902::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.201_f32 + y.sin();
        let b = y * 5.12_f32 - x.cos();
        let mut acc = Accumulator902::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.418_f32 + y.sin();
        let b = y * 8.543_f32 - x.cos();
        let mut acc = Accumulator902::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.718_f32 + y.sin();
        let b = y * 4.653_f32 - x.cos();
        let mut acc = Accumulator902::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.22_f32 + y.sin();
        let b = y * 1.488_f32 - x.cos();
        let mut acc = Accumulator902::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.418_f32 + y.sin();
        let b = y * 6.434_f32 - x.cos();
        let mut acc = Accumulator902::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.593_f32 + y.sin();
        let b = y * 7.649_f32 - x.cos();
        let mut acc = Accumulator902::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.945_f32 + y.sin();
        let b = y * 5.62_f32 - x.cos();
        let mut acc = Accumulator902::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.279_f32 + y.sin();
        let b = y * 1.906_f32 - x.cos();
        let mut acc = Accumulator902::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.945_f32 + y.sin();
        let b = y * 2.314_f32 - x.cos();
        let mut acc = Accumulator902::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.663_f32 + y.sin();
        let b = y * 3.52_f32 - x.cos();
        let mut acc = Accumulator902::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.025_f32 + y.sin();
        let b = y * 2.166_f32 - x.cos();
        let mut acc = Accumulator902::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.713_f32 + y.sin();
        let b = y * 3.468_f32 - x.cos();
        let mut acc = Accumulator902::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.226_f32 + y.sin();
        let b = y * 8.979_f32 - x.cos();
        let mut acc = Accumulator902::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.275_f32 + y.sin();
        let b = y * 1.224_f32 - x.cos();
        let mut acc = Accumulator902::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_902(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_902() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_902(total as u64) % 997) as f32;
        total
    }
}

pub mod m903 {
    use super::*;

    pub struct Accumulator903<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator903<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.39_f32 + y.sin();
        let b = y * 2.822_f32 - x.cos();
        let mut acc = Accumulator903::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.982_f32 + y.sin();
        let b = y * 2.532_f32 - x.cos();
        let mut acc = Accumulator903::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.753_f32 + y.sin();
        let b = y * 1.047_f32 - x.cos();
        let mut acc = Accumulator903::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.998_f32 + y.sin();
        let b = y * 1.027_f32 - x.cos();
        let mut acc = Accumulator903::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.285_f32 + y.sin();
        let b = y * 4.191_f32 - x.cos();
        let mut acc = Accumulator903::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.154_f32 + y.sin();
        let b = y * 5.979_f32 - x.cos();
        let mut acc = Accumulator903::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.733_f32 + y.sin();
        let b = y * 6.115_f32 - x.cos();
        let mut acc = Accumulator903::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.578_f32 + y.sin();
        let b = y * 0.113_f32 - x.cos();
        let mut acc = Accumulator903::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.515_f32 + y.sin();
        let b = y * 6.494_f32 - x.cos();
        let mut acc = Accumulator903::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.329_f32 + y.sin();
        let b = y * 0.299_f32 - x.cos();
        let mut acc = Accumulator903::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.11_f32 + y.sin();
        let b = y * 0.714_f32 - x.cos();
        let mut acc = Accumulator903::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.493_f32 + y.sin();
        let b = y * 0.93_f32 - x.cos();
        let mut acc = Accumulator903::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.74_f32 + y.sin();
        let b = y * 6.529_f32 - x.cos();
        let mut acc = Accumulator903::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.946_f32 + y.sin();
        let b = y * 2.287_f32 - x.cos();
        let mut acc = Accumulator903::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.769_f32 + y.sin();
        let b = y * 6.322_f32 - x.cos();
        let mut acc = Accumulator903::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.317_f32 + y.sin();
        let b = y * 4.645_f32 - x.cos();
        let mut acc = Accumulator903::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.739_f32 + y.sin();
        let b = y * 2.634_f32 - x.cos();
        let mut acc = Accumulator903::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.261_f32 + y.sin();
        let b = y * 2.895_f32 - x.cos();
        let mut acc = Accumulator903::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.156_f32 + y.sin();
        let b = y * 7.585_f32 - x.cos();
        let mut acc = Accumulator903::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.615_f32 + y.sin();
        let b = y * 5.215_f32 - x.cos();
        let mut acc = Accumulator903::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_903(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_903() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_903(total as u64) % 997) as f32;
        total
    }
}

pub mod m904 {
    use super::*;

    pub struct Accumulator904<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator904<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.797_f32 + y.sin();
        let b = y * 6.323_f32 - x.cos();
        let mut acc = Accumulator904::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.782_f32 + y.sin();
        let b = y * 8.436_f32 - x.cos();
        let mut acc = Accumulator904::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.955_f32 + y.sin();
        let b = y * 3.893_f32 - x.cos();
        let mut acc = Accumulator904::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.839_f32 + y.sin();
        let b = y * 2.543_f32 - x.cos();
        let mut acc = Accumulator904::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.111_f32 + y.sin();
        let b = y * 4.923_f32 - x.cos();
        let mut acc = Accumulator904::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.281_f32 + y.sin();
        let b = y * 4.545_f32 - x.cos();
        let mut acc = Accumulator904::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.919_f32 + y.sin();
        let b = y * 3.242_f32 - x.cos();
        let mut acc = Accumulator904::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.341_f32 + y.sin();
        let b = y * 3.414_f32 - x.cos();
        let mut acc = Accumulator904::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.424_f32 + y.sin();
        let b = y * 7.968_f32 - x.cos();
        let mut acc = Accumulator904::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.281_f32 + y.sin();
        let b = y * 5.984_f32 - x.cos();
        let mut acc = Accumulator904::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.447_f32 + y.sin();
        let b = y * 1.735_f32 - x.cos();
        let mut acc = Accumulator904::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.845_f32 + y.sin();
        let b = y * 5.437_f32 - x.cos();
        let mut acc = Accumulator904::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.352_f32 + y.sin();
        let b = y * 1.92_f32 - x.cos();
        let mut acc = Accumulator904::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.391_f32 + y.sin();
        let b = y * 0.247_f32 - x.cos();
        let mut acc = Accumulator904::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.736_f32 + y.sin();
        let b = y * 5.498_f32 - x.cos();
        let mut acc = Accumulator904::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.836_f32 + y.sin();
        let b = y * 1.127_f32 - x.cos();
        let mut acc = Accumulator904::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.422_f32 + y.sin();
        let b = y * 3.042_f32 - x.cos();
        let mut acc = Accumulator904::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.091_f32 + y.sin();
        let b = y * 4.61_f32 - x.cos();
        let mut acc = Accumulator904::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.061_f32 + y.sin();
        let b = y * 1.053_f32 - x.cos();
        let mut acc = Accumulator904::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.276_f32 + y.sin();
        let b = y * 3.387_f32 - x.cos();
        let mut acc = Accumulator904::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_904(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m904-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_904() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_904(total as u64) % 997) as f32;
        total
    }
}

pub mod m905 {
    use super::*;

    pub struct Accumulator905<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator905<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.837_f32 + y.sin();
        let b = y * 7.577_f32 - x.cos();
        let mut acc = Accumulator905::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.35_f32 + y.sin();
        let b = y * 5.632_f32 - x.cos();
        let mut acc = Accumulator905::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.902_f32 + y.sin();
        let b = y * 7.389_f32 - x.cos();
        let mut acc = Accumulator905::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.159_f32 + y.sin();
        let b = y * 3.144_f32 - x.cos();
        let mut acc = Accumulator905::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.421_f32 + y.sin();
        let b = y * 2.59_f32 - x.cos();
        let mut acc = Accumulator905::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.287_f32 + y.sin();
        let b = y * 7.344_f32 - x.cos();
        let mut acc = Accumulator905::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.586_f32 + y.sin();
        let b = y * 5.078_f32 - x.cos();
        let mut acc = Accumulator905::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.239_f32 + y.sin();
        let b = y * 6.72_f32 - x.cos();
        let mut acc = Accumulator905::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.27_f32 + y.sin();
        let b = y * 3.842_f32 - x.cos();
        let mut acc = Accumulator905::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.612_f32 + y.sin();
        let b = y * 5.265_f32 - x.cos();
        let mut acc = Accumulator905::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.263_f32 + y.sin();
        let b = y * 6.311_f32 - x.cos();
        let mut acc = Accumulator905::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.335_f32 + y.sin();
        let b = y * 3.89_f32 - x.cos();
        let mut acc = Accumulator905::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.677_f32 + y.sin();
        let b = y * 2.109_f32 - x.cos();
        let mut acc = Accumulator905::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.295_f32 + y.sin();
        let b = y * 0.146_f32 - x.cos();
        let mut acc = Accumulator905::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.347_f32 + y.sin();
        let b = y * 4.728_f32 - x.cos();
        let mut acc = Accumulator905::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.183_f32 + y.sin();
        let b = y * 6.475_f32 - x.cos();
        let mut acc = Accumulator905::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.889_f32 + y.sin();
        let b = y * 8.406_f32 - x.cos();
        let mut acc = Accumulator905::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.95_f32 + y.sin();
        let b = y * 3.806_f32 - x.cos();
        let mut acc = Accumulator905::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.482_f32 + y.sin();
        let b = y * 4.588_f32 - x.cos();
        let mut acc = Accumulator905::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.138_f32 + y.sin();
        let b = y * 7.905_f32 - x.cos();
        let mut acc = Accumulator905::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_905(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_905() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_905(total as u64) % 997) as f32;
        total
    }
}

pub mod m906 {
    use super::*;

    pub struct Accumulator906<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator906<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.129_f32 + y.sin();
        let b = y * 7.59_f32 - x.cos();
        let mut acc = Accumulator906::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.398_f32 + y.sin();
        let b = y * 4.954_f32 - x.cos();
        let mut acc = Accumulator906::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.106_f32 + y.sin();
        let b = y * 0.637_f32 - x.cos();
        let mut acc = Accumulator906::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.759_f32 + y.sin();
        let b = y * 6.197_f32 - x.cos();
        let mut acc = Accumulator906::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.23_f32 + y.sin();
        let b = y * 3.827_f32 - x.cos();
        let mut acc = Accumulator906::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.176_f32 + y.sin();
        let b = y * 3.871_f32 - x.cos();
        let mut acc = Accumulator906::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.221_f32 + y.sin();
        let b = y * 6.964_f32 - x.cos();
        let mut acc = Accumulator906::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.618_f32 + y.sin();
        let b = y * 8.367_f32 - x.cos();
        let mut acc = Accumulator906::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.909_f32 + y.sin();
        let b = y * 5.973_f32 - x.cos();
        let mut acc = Accumulator906::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.648_f32 + y.sin();
        let b = y * 6.415_f32 - x.cos();
        let mut acc = Accumulator906::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.696_f32 + y.sin();
        let b = y * 9.313_f32 - x.cos();
        let mut acc = Accumulator906::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.249_f32 + y.sin();
        let b = y * 2.048_f32 - x.cos();
        let mut acc = Accumulator906::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.937_f32 + y.sin();
        let b = y * 4.104_f32 - x.cos();
        let mut acc = Accumulator906::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.94_f32 + y.sin();
        let b = y * 5.311_f32 - x.cos();
        let mut acc = Accumulator906::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.57_f32 + y.sin();
        let b = y * 8.753_f32 - x.cos();
        let mut acc = Accumulator906::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.835_f32 + y.sin();
        let b = y * 8.867_f32 - x.cos();
        let mut acc = Accumulator906::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.912_f32 + y.sin();
        let b = y * 0.974_f32 - x.cos();
        let mut acc = Accumulator906::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.475_f32 + y.sin();
        let b = y * 4.351_f32 - x.cos();
        let mut acc = Accumulator906::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.439_f32 + y.sin();
        let b = y * 3.843_f32 - x.cos();
        let mut acc = Accumulator906::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.445_f32 + y.sin();
        let b = y * 5.395_f32 - x.cos();
        let mut acc = Accumulator906::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_906(seed: u64) -> u64 {
        let re = Regex::new(r"m906-(\d+)").unwrap();
        let hay = format!("m906-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_906() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_906(total as u64) % 997) as f32;
        total
    }
}

pub mod m907 {
    use super::*;

    pub struct Accumulator907<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator907<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.57_f32 + y.sin();
        let b = y * 4.51_f32 - x.cos();
        let mut acc = Accumulator907::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.307_f32 + y.sin();
        let b = y * 8.135_f32 - x.cos();
        let mut acc = Accumulator907::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.253_f32 + y.sin();
        let b = y * 9.762_f32 - x.cos();
        let mut acc = Accumulator907::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.727_f32 + y.sin();
        let b = y * 0.909_f32 - x.cos();
        let mut acc = Accumulator907::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.849_f32 + y.sin();
        let b = y * 7.542_f32 - x.cos();
        let mut acc = Accumulator907::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.653_f32 + y.sin();
        let b = y * 9.241_f32 - x.cos();
        let mut acc = Accumulator907::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.951_f32 + y.sin();
        let b = y * 4.132_f32 - x.cos();
        let mut acc = Accumulator907::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.569_f32 + y.sin();
        let b = y * 4.189_f32 - x.cos();
        let mut acc = Accumulator907::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.748_f32 + y.sin();
        let b = y * 9.42_f32 - x.cos();
        let mut acc = Accumulator907::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.608_f32 + y.sin();
        let b = y * 0.532_f32 - x.cos();
        let mut acc = Accumulator907::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.065_f32 + y.sin();
        let b = y * 7.465_f32 - x.cos();
        let mut acc = Accumulator907::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.061_f32 + y.sin();
        let b = y * 7.419_f32 - x.cos();
        let mut acc = Accumulator907::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.182_f32 + y.sin();
        let b = y * 7.879_f32 - x.cos();
        let mut acc = Accumulator907::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.104_f32 + y.sin();
        let b = y * 4.694_f32 - x.cos();
        let mut acc = Accumulator907::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.848_f32 + y.sin();
        let b = y * 4.066_f32 - x.cos();
        let mut acc = Accumulator907::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.752_f32 + y.sin();
        let b = y * 2.833_f32 - x.cos();
        let mut acc = Accumulator907::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.763_f32 + y.sin();
        let b = y * 2.439_f32 - x.cos();
        let mut acc = Accumulator907::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.832_f32 + y.sin();
        let b = y * 3.079_f32 - x.cos();
        let mut acc = Accumulator907::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.437_f32 + y.sin();
        let b = y * 4.228_f32 - x.cos();
        let mut acc = Accumulator907::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.55_f32 + y.sin();
        let b = y * 0.91_f32 - x.cos();
        let mut acc = Accumulator907::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_907(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_907() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_907(total as u64) % 997) as f32;
        total
    }
}

pub mod m908 {
    use super::*;

    pub struct Accumulator908<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator908<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.024_f32 + y.sin();
        let b = y * 2.493_f32 - x.cos();
        let mut acc = Accumulator908::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.68_f32 + y.sin();
        let b = y * 0.905_f32 - x.cos();
        let mut acc = Accumulator908::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.522_f32 + y.sin();
        let b = y * 0.841_f32 - x.cos();
        let mut acc = Accumulator908::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.22_f32 + y.sin();
        let b = y * 2.168_f32 - x.cos();
        let mut acc = Accumulator908::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.824_f32 + y.sin();
        let b = y * 3.044_f32 - x.cos();
        let mut acc = Accumulator908::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.0_f32 + y.sin();
        let b = y * 7.922_f32 - x.cos();
        let mut acc = Accumulator908::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.466_f32 + y.sin();
        let b = y * 7.629_f32 - x.cos();
        let mut acc = Accumulator908::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.501_f32 + y.sin();
        let b = y * 0.812_f32 - x.cos();
        let mut acc = Accumulator908::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.473_f32 + y.sin();
        let b = y * 5.784_f32 - x.cos();
        let mut acc = Accumulator908::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.443_f32 + y.sin();
        let b = y * 3.578_f32 - x.cos();
        let mut acc = Accumulator908::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.523_f32 + y.sin();
        let b = y * 8.718_f32 - x.cos();
        let mut acc = Accumulator908::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.163_f32 + y.sin();
        let b = y * 3.461_f32 - x.cos();
        let mut acc = Accumulator908::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.247_f32 + y.sin();
        let b = y * 5.586_f32 - x.cos();
        let mut acc = Accumulator908::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.359_f32 + y.sin();
        let b = y * 9.719_f32 - x.cos();
        let mut acc = Accumulator908::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.158_f32 + y.sin();
        let b = y * 3.586_f32 - x.cos();
        let mut acc = Accumulator908::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.334_f32 + y.sin();
        let b = y * 9.264_f32 - x.cos();
        let mut acc = Accumulator908::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.905_f32 + y.sin();
        let b = y * 6.652_f32 - x.cos();
        let mut acc = Accumulator908::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.074_f32 + y.sin();
        let b = y * 6.171_f32 - x.cos();
        let mut acc = Accumulator908::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.518_f32 + y.sin();
        let b = y * 5.211_f32 - x.cos();
        let mut acc = Accumulator908::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.457_f32 + y.sin();
        let b = y * 8.837_f32 - x.cos();
        let mut acc = Accumulator908::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_908(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(908u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_908() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_908(total as u64) % 997) as f32;
        total
    }
}

pub mod m909 {
    use super::*;

    pub struct Accumulator909<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator909<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.578_f32 + y.sin();
        let b = y * 9.14_f32 - x.cos();
        let mut acc = Accumulator909::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.264_f32 + y.sin();
        let b = y * 7.76_f32 - x.cos();
        let mut acc = Accumulator909::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.282_f32 + y.sin();
        let b = y * 4.411_f32 - x.cos();
        let mut acc = Accumulator909::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.137_f32 + y.sin();
        let b = y * 2.233_f32 - x.cos();
        let mut acc = Accumulator909::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.34_f32 + y.sin();
        let b = y * 5.207_f32 - x.cos();
        let mut acc = Accumulator909::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.415_f32 + y.sin();
        let b = y * 8.583_f32 - x.cos();
        let mut acc = Accumulator909::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.445_f32 + y.sin();
        let b = y * 9.535_f32 - x.cos();
        let mut acc = Accumulator909::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.349_f32 + y.sin();
        let b = y * 1.549_f32 - x.cos();
        let mut acc = Accumulator909::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.071_f32 + y.sin();
        let b = y * 2.15_f32 - x.cos();
        let mut acc = Accumulator909::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.419_f32 + y.sin();
        let b = y * 9.333_f32 - x.cos();
        let mut acc = Accumulator909::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.637_f32 + y.sin();
        let b = y * 0.41_f32 - x.cos();
        let mut acc = Accumulator909::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.712_f32 + y.sin();
        let b = y * 9.25_f32 - x.cos();
        let mut acc = Accumulator909::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.664_f32 + y.sin();
        let b = y * 2.468_f32 - x.cos();
        let mut acc = Accumulator909::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.418_f32 + y.sin();
        let b = y * 5.727_f32 - x.cos();
        let mut acc = Accumulator909::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.231_f32 + y.sin();
        let b = y * 3.831_f32 - x.cos();
        let mut acc = Accumulator909::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.686_f32 + y.sin();
        let b = y * 8.789_f32 - x.cos();
        let mut acc = Accumulator909::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.027_f32 + y.sin();
        let b = y * 3.499_f32 - x.cos();
        let mut acc = Accumulator909::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.261_f32 + y.sin();
        let b = y * 6.807_f32 - x.cos();
        let mut acc = Accumulator909::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.545_f32 + y.sin();
        let b = y * 6.055_f32 - x.cos();
        let mut acc = Accumulator909::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.875_f32 + y.sin();
        let b = y * 5.515_f32 - x.cos();
        let mut acc = Accumulator909::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_909(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_909() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_909(total as u64) % 997) as f32;
        total
    }
}

pub mod m910 {
    use super::*;

    pub struct Accumulator910<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator910<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.541_f32 + y.sin();
        let b = y * 5.916_f32 - x.cos();
        let mut acc = Accumulator910::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.663_f32 + y.sin();
        let b = y * 8.555_f32 - x.cos();
        let mut acc = Accumulator910::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.587_f32 + y.sin();
        let b = y * 5.608_f32 - x.cos();
        let mut acc = Accumulator910::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.174_f32 + y.sin();
        let b = y * 6.05_f32 - x.cos();
        let mut acc = Accumulator910::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.679_f32 + y.sin();
        let b = y * 5.676_f32 - x.cos();
        let mut acc = Accumulator910::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.034_f32 + y.sin();
        let b = y * 0.316_f32 - x.cos();
        let mut acc = Accumulator910::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.362_f32 + y.sin();
        let b = y * 9.491_f32 - x.cos();
        let mut acc = Accumulator910::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.495_f32 + y.sin();
        let b = y * 3.739_f32 - x.cos();
        let mut acc = Accumulator910::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.041_f32 + y.sin();
        let b = y * 2.812_f32 - x.cos();
        let mut acc = Accumulator910::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.271_f32 + y.sin();
        let b = y * 6.543_f32 - x.cos();
        let mut acc = Accumulator910::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.231_f32 + y.sin();
        let b = y * 1.701_f32 - x.cos();
        let mut acc = Accumulator910::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.426_f32 + y.sin();
        let b = y * 9.683_f32 - x.cos();
        let mut acc = Accumulator910::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.373_f32 + y.sin();
        let b = y * 7.441_f32 - x.cos();
        let mut acc = Accumulator910::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.763_f32 + y.sin();
        let b = y * 7.684_f32 - x.cos();
        let mut acc = Accumulator910::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.485_f32 + y.sin();
        let b = y * 9.714_f32 - x.cos();
        let mut acc = Accumulator910::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.165_f32 + y.sin();
        let b = y * 4.206_f32 - x.cos();
        let mut acc = Accumulator910::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.741_f32 + y.sin();
        let b = y * 4.437_f32 - x.cos();
        let mut acc = Accumulator910::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.193_f32 + y.sin();
        let b = y * 0.153_f32 - x.cos();
        let mut acc = Accumulator910::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.512_f32 + y.sin();
        let b = y * 5.027_f32 - x.cos();
        let mut acc = Accumulator910::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.447_f32 + y.sin();
        let b = y * 4.026_f32 - x.cos();
        let mut acc = Accumulator910::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_910(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_910() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_910(total as u64) % 997) as f32;
        total
    }
}

pub mod m911 {
    use super::*;

    pub struct Accumulator911<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator911<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.869_f32 + y.sin();
        let b = y * 3.939_f32 - x.cos();
        let mut acc = Accumulator911::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.861_f32 + y.sin();
        let b = y * 6.944_f32 - x.cos();
        let mut acc = Accumulator911::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.473_f32 + y.sin();
        let b = y * 2.811_f32 - x.cos();
        let mut acc = Accumulator911::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.601_f32 + y.sin();
        let b = y * 5.067_f32 - x.cos();
        let mut acc = Accumulator911::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.172_f32 + y.sin();
        let b = y * 4.463_f32 - x.cos();
        let mut acc = Accumulator911::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.603_f32 + y.sin();
        let b = y * 3.987_f32 - x.cos();
        let mut acc = Accumulator911::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.615_f32 + y.sin();
        let b = y * 5.975_f32 - x.cos();
        let mut acc = Accumulator911::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.544_f32 + y.sin();
        let b = y * 6.868_f32 - x.cos();
        let mut acc = Accumulator911::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.256_f32 + y.sin();
        let b = y * 5.618_f32 - x.cos();
        let mut acc = Accumulator911::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.446_f32 + y.sin();
        let b = y * 7.086_f32 - x.cos();
        let mut acc = Accumulator911::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.954_f32 + y.sin();
        let b = y * 0.198_f32 - x.cos();
        let mut acc = Accumulator911::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.683_f32 + y.sin();
        let b = y * 0.578_f32 - x.cos();
        let mut acc = Accumulator911::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.745_f32 + y.sin();
        let b = y * 0.772_f32 - x.cos();
        let mut acc = Accumulator911::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.342_f32 + y.sin();
        let b = y * 4.849_f32 - x.cos();
        let mut acc = Accumulator911::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.448_f32 + y.sin();
        let b = y * 1.474_f32 - x.cos();
        let mut acc = Accumulator911::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.998_f32 + y.sin();
        let b = y * 0.382_f32 - x.cos();
        let mut acc = Accumulator911::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.306_f32 + y.sin();
        let b = y * 0.869_f32 - x.cos();
        let mut acc = Accumulator911::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.275_f32 + y.sin();
        let b = y * 9.416_f32 - x.cos();
        let mut acc = Accumulator911::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.733_f32 + y.sin();
        let b = y * 9.618_f32 - x.cos();
        let mut acc = Accumulator911::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.449_f32 + y.sin();
        let b = y * 7.567_f32 - x.cos();
        let mut acc = Accumulator911::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_911(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m911-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_911() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_911(total as u64) % 997) as f32;
        total
    }
}

pub mod m912 {
    use super::*;

    pub struct Accumulator912<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator912<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.429_f32 + y.sin();
        let b = y * 1.078_f32 - x.cos();
        let mut acc = Accumulator912::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.372_f32 + y.sin();
        let b = y * 2.576_f32 - x.cos();
        let mut acc = Accumulator912::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.134_f32 + y.sin();
        let b = y * 1.169_f32 - x.cos();
        let mut acc = Accumulator912::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.891_f32 + y.sin();
        let b = y * 3.048_f32 - x.cos();
        let mut acc = Accumulator912::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.492_f32 + y.sin();
        let b = y * 0.163_f32 - x.cos();
        let mut acc = Accumulator912::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.173_f32 + y.sin();
        let b = y * 5.077_f32 - x.cos();
        let mut acc = Accumulator912::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.399_f32 + y.sin();
        let b = y * 7.822_f32 - x.cos();
        let mut acc = Accumulator912::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.469_f32 + y.sin();
        let b = y * 6.318_f32 - x.cos();
        let mut acc = Accumulator912::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.121_f32 + y.sin();
        let b = y * 8.056_f32 - x.cos();
        let mut acc = Accumulator912::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.795_f32 + y.sin();
        let b = y * 2.057_f32 - x.cos();
        let mut acc = Accumulator912::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.644_f32 + y.sin();
        let b = y * 9.435_f32 - x.cos();
        let mut acc = Accumulator912::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.303_f32 + y.sin();
        let b = y * 2.43_f32 - x.cos();
        let mut acc = Accumulator912::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.747_f32 + y.sin();
        let b = y * 6.671_f32 - x.cos();
        let mut acc = Accumulator912::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.573_f32 + y.sin();
        let b = y * 4.878_f32 - x.cos();
        let mut acc = Accumulator912::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.723_f32 + y.sin();
        let b = y * 7.121_f32 - x.cos();
        let mut acc = Accumulator912::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.147_f32 + y.sin();
        let b = y * 0.208_f32 - x.cos();
        let mut acc = Accumulator912::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.105_f32 + y.sin();
        let b = y * 5.199_f32 - x.cos();
        let mut acc = Accumulator912::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.983_f32 + y.sin();
        let b = y * 7.029_f32 - x.cos();
        let mut acc = Accumulator912::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.549_f32 + y.sin();
        let b = y * 2.628_f32 - x.cos();
        let mut acc = Accumulator912::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.851_f32 + y.sin();
        let b = y * 7.949_f32 - x.cos();
        let mut acc = Accumulator912::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_912(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_912() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_912(total as u64) % 997) as f32;
        total
    }
}

pub mod m913 {
    use super::*;

    pub struct Accumulator913<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator913<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.385_f32 + y.sin();
        let b = y * 1.689_f32 - x.cos();
        let mut acc = Accumulator913::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.082_f32 + y.sin();
        let b = y * 9.33_f32 - x.cos();
        let mut acc = Accumulator913::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.676_f32 + y.sin();
        let b = y * 5.479_f32 - x.cos();
        let mut acc = Accumulator913::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.86_f32 + y.sin();
        let b = y * 9.108_f32 - x.cos();
        let mut acc = Accumulator913::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.24_f32 + y.sin();
        let b = y * 4.659_f32 - x.cos();
        let mut acc = Accumulator913::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.257_f32 + y.sin();
        let b = y * 5.466_f32 - x.cos();
        let mut acc = Accumulator913::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.646_f32 + y.sin();
        let b = y * 1.677_f32 - x.cos();
        let mut acc = Accumulator913::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.958_f32 + y.sin();
        let b = y * 6.944_f32 - x.cos();
        let mut acc = Accumulator913::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.408_f32 + y.sin();
        let b = y * 7.075_f32 - x.cos();
        let mut acc = Accumulator913::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.191_f32 + y.sin();
        let b = y * 6.08_f32 - x.cos();
        let mut acc = Accumulator913::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.767_f32 + y.sin();
        let b = y * 8.501_f32 - x.cos();
        let mut acc = Accumulator913::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.7_f32 + y.sin();
        let b = y * 9.44_f32 - x.cos();
        let mut acc = Accumulator913::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.779_f32 + y.sin();
        let b = y * 8.675_f32 - x.cos();
        let mut acc = Accumulator913::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.144_f32 + y.sin();
        let b = y * 8.074_f32 - x.cos();
        let mut acc = Accumulator913::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.598_f32 + y.sin();
        let b = y * 7.845_f32 - x.cos();
        let mut acc = Accumulator913::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.554_f32 + y.sin();
        let b = y * 7.631_f32 - x.cos();
        let mut acc = Accumulator913::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.355_f32 + y.sin();
        let b = y * 5.406_f32 - x.cos();
        let mut acc = Accumulator913::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.078_f32 + y.sin();
        let b = y * 2.327_f32 - x.cos();
        let mut acc = Accumulator913::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.448_f32 + y.sin();
        let b = y * 8.944_f32 - x.cos();
        let mut acc = Accumulator913::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.696_f32 + y.sin();
        let b = y * 2.945_f32 - x.cos();
        let mut acc = Accumulator913::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_913(seed: u64) -> u64 {
        let re = Regex::new(r"m913-(\d+)").unwrap();
        let hay = format!("m913-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_913() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_913(total as u64) % 997) as f32;
        total
    }
}

pub mod m914 {
    use super::*;

    pub struct Accumulator914<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator914<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.012_f32 + y.sin();
        let b = y * 7.36_f32 - x.cos();
        let mut acc = Accumulator914::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.872_f32 + y.sin();
        let b = y * 4.475_f32 - x.cos();
        let mut acc = Accumulator914::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.232_f32 + y.sin();
        let b = y * 7.856_f32 - x.cos();
        let mut acc = Accumulator914::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.044_f32 + y.sin();
        let b = y * 7.712_f32 - x.cos();
        let mut acc = Accumulator914::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.837_f32 + y.sin();
        let b = y * 0.347_f32 - x.cos();
        let mut acc = Accumulator914::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.801_f32 + y.sin();
        let b = y * 3.56_f32 - x.cos();
        let mut acc = Accumulator914::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.161_f32 + y.sin();
        let b = y * 7.681_f32 - x.cos();
        let mut acc = Accumulator914::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.375_f32 + y.sin();
        let b = y * 2.252_f32 - x.cos();
        let mut acc = Accumulator914::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.37_f32 + y.sin();
        let b = y * 0.454_f32 - x.cos();
        let mut acc = Accumulator914::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.558_f32 + y.sin();
        let b = y * 3.223_f32 - x.cos();
        let mut acc = Accumulator914::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.606_f32 + y.sin();
        let b = y * 5.793_f32 - x.cos();
        let mut acc = Accumulator914::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.264_f32 + y.sin();
        let b = y * 7.84_f32 - x.cos();
        let mut acc = Accumulator914::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.257_f32 + y.sin();
        let b = y * 0.587_f32 - x.cos();
        let mut acc = Accumulator914::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.684_f32 + y.sin();
        let b = y * 6.354_f32 - x.cos();
        let mut acc = Accumulator914::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.88_f32 + y.sin();
        let b = y * 1.697_f32 - x.cos();
        let mut acc = Accumulator914::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.587_f32 + y.sin();
        let b = y * 1.748_f32 - x.cos();
        let mut acc = Accumulator914::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.831_f32 + y.sin();
        let b = y * 8.821_f32 - x.cos();
        let mut acc = Accumulator914::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.421_f32 + y.sin();
        let b = y * 0.559_f32 - x.cos();
        let mut acc = Accumulator914::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.318_f32 + y.sin();
        let b = y * 0.573_f32 - x.cos();
        let mut acc = Accumulator914::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.448_f32 + y.sin();
        let b = y * 2.554_f32 - x.cos();
        let mut acc = Accumulator914::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_914(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_914() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_914(total as u64) % 997) as f32;
        total
    }
}

pub mod m915 {
    use super::*;

    pub struct Accumulator915<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator915<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.233_f32 + y.sin();
        let b = y * 6.932_f32 - x.cos();
        let mut acc = Accumulator915::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.197_f32 + y.sin();
        let b = y * 2.91_f32 - x.cos();
        let mut acc = Accumulator915::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.664_f32 + y.sin();
        let b = y * 4.707_f32 - x.cos();
        let mut acc = Accumulator915::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.902_f32 + y.sin();
        let b = y * 8.622_f32 - x.cos();
        let mut acc = Accumulator915::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.187_f32 + y.sin();
        let b = y * 1.973_f32 - x.cos();
        let mut acc = Accumulator915::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.889_f32 + y.sin();
        let b = y * 5.139_f32 - x.cos();
        let mut acc = Accumulator915::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.998_f32 + y.sin();
        let b = y * 5.649_f32 - x.cos();
        let mut acc = Accumulator915::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.233_f32 + y.sin();
        let b = y * 8.658_f32 - x.cos();
        let mut acc = Accumulator915::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.3_f32 + y.sin();
        let b = y * 0.658_f32 - x.cos();
        let mut acc = Accumulator915::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.656_f32 + y.sin();
        let b = y * 8.59_f32 - x.cos();
        let mut acc = Accumulator915::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.365_f32 + y.sin();
        let b = y * 0.228_f32 - x.cos();
        let mut acc = Accumulator915::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.535_f32 + y.sin();
        let b = y * 1.98_f32 - x.cos();
        let mut acc = Accumulator915::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.607_f32 + y.sin();
        let b = y * 9.606_f32 - x.cos();
        let mut acc = Accumulator915::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.594_f32 + y.sin();
        let b = y * 1.864_f32 - x.cos();
        let mut acc = Accumulator915::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.296_f32 + y.sin();
        let b = y * 3.958_f32 - x.cos();
        let mut acc = Accumulator915::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.627_f32 + y.sin();
        let b = y * 9.289_f32 - x.cos();
        let mut acc = Accumulator915::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.211_f32 + y.sin();
        let b = y * 0.905_f32 - x.cos();
        let mut acc = Accumulator915::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.016_f32 + y.sin();
        let b = y * 3.891_f32 - x.cos();
        let mut acc = Accumulator915::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.279_f32 + y.sin();
        let b = y * 3.516_f32 - x.cos();
        let mut acc = Accumulator915::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.739_f32 + y.sin();
        let b = y * 7.721_f32 - x.cos();
        let mut acc = Accumulator915::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_915(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(915u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_915() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_915(total as u64) % 997) as f32;
        total
    }
}

pub mod m916 {
    use super::*;

    pub struct Accumulator916<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator916<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.846_f32 + y.sin();
        let b = y * 3.849_f32 - x.cos();
        let mut acc = Accumulator916::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.307_f32 + y.sin();
        let b = y * 7.889_f32 - x.cos();
        let mut acc = Accumulator916::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.786_f32 + y.sin();
        let b = y * 0.944_f32 - x.cos();
        let mut acc = Accumulator916::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.229_f32 + y.sin();
        let b = y * 0.135_f32 - x.cos();
        let mut acc = Accumulator916::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.403_f32 + y.sin();
        let b = y * 6.196_f32 - x.cos();
        let mut acc = Accumulator916::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.009_f32 + y.sin();
        let b = y * 6.453_f32 - x.cos();
        let mut acc = Accumulator916::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.15_f32 + y.sin();
        let b = y * 5.26_f32 - x.cos();
        let mut acc = Accumulator916::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.732_f32 + y.sin();
        let b = y * 5.817_f32 - x.cos();
        let mut acc = Accumulator916::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.524_f32 + y.sin();
        let b = y * 2.592_f32 - x.cos();
        let mut acc = Accumulator916::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.163_f32 + y.sin();
        let b = y * 8.339_f32 - x.cos();
        let mut acc = Accumulator916::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.326_f32 + y.sin();
        let b = y * 8.01_f32 - x.cos();
        let mut acc = Accumulator916::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.52_f32 + y.sin();
        let b = y * 5.606_f32 - x.cos();
        let mut acc = Accumulator916::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.385_f32 + y.sin();
        let b = y * 4.892_f32 - x.cos();
        let mut acc = Accumulator916::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.885_f32 + y.sin();
        let b = y * 6.757_f32 - x.cos();
        let mut acc = Accumulator916::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.158_f32 + y.sin();
        let b = y * 8.598_f32 - x.cos();
        let mut acc = Accumulator916::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.05_f32 + y.sin();
        let b = y * 8.994_f32 - x.cos();
        let mut acc = Accumulator916::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.671_f32 + y.sin();
        let b = y * 7.435_f32 - x.cos();
        let mut acc = Accumulator916::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.542_f32 + y.sin();
        let b = y * 9.165_f32 - x.cos();
        let mut acc = Accumulator916::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.165_f32 + y.sin();
        let b = y * 6.95_f32 - x.cos();
        let mut acc = Accumulator916::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.651_f32 + y.sin();
        let b = y * 7.998_f32 - x.cos();
        let mut acc = Accumulator916::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_916(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_916() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_916(total as u64) % 997) as f32;
        total
    }
}

pub mod m917 {
    use super::*;

    pub struct Accumulator917<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator917<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.412_f32 + y.sin();
        let b = y * 7.671_f32 - x.cos();
        let mut acc = Accumulator917::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.768_f32 + y.sin();
        let b = y * 8.585_f32 - x.cos();
        let mut acc = Accumulator917::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.148_f32 + y.sin();
        let b = y * 2.093_f32 - x.cos();
        let mut acc = Accumulator917::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.137_f32 + y.sin();
        let b = y * 1.096_f32 - x.cos();
        let mut acc = Accumulator917::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.791_f32 + y.sin();
        let b = y * 9.043_f32 - x.cos();
        let mut acc = Accumulator917::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.745_f32 + y.sin();
        let b = y * 1.356_f32 - x.cos();
        let mut acc = Accumulator917::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.562_f32 + y.sin();
        let b = y * 9.741_f32 - x.cos();
        let mut acc = Accumulator917::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.297_f32 + y.sin();
        let b = y * 2.289_f32 - x.cos();
        let mut acc = Accumulator917::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.281_f32 + y.sin();
        let b = y * 4.761_f32 - x.cos();
        let mut acc = Accumulator917::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.088_f32 + y.sin();
        let b = y * 2.685_f32 - x.cos();
        let mut acc = Accumulator917::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.548_f32 + y.sin();
        let b = y * 8.113_f32 - x.cos();
        let mut acc = Accumulator917::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.409_f32 + y.sin();
        let b = y * 2.764_f32 - x.cos();
        let mut acc = Accumulator917::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.326_f32 + y.sin();
        let b = y * 4.335_f32 - x.cos();
        let mut acc = Accumulator917::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.56_f32 + y.sin();
        let b = y * 6.678_f32 - x.cos();
        let mut acc = Accumulator917::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.192_f32 + y.sin();
        let b = y * 7.075_f32 - x.cos();
        let mut acc = Accumulator917::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.655_f32 + y.sin();
        let b = y * 4.835_f32 - x.cos();
        let mut acc = Accumulator917::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.674_f32 + y.sin();
        let b = y * 1.1_f32 - x.cos();
        let mut acc = Accumulator917::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.405_f32 + y.sin();
        let b = y * 7.721_f32 - x.cos();
        let mut acc = Accumulator917::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.409_f32 + y.sin();
        let b = y * 3.808_f32 - x.cos();
        let mut acc = Accumulator917::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.82_f32 + y.sin();
        let b = y * 2.651_f32 - x.cos();
        let mut acc = Accumulator917::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_917(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_917() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_917(total as u64) % 997) as f32;
        total
    }
}

pub mod m918 {
    use super::*;

    pub struct Accumulator918<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator918<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.824_f32 + y.sin();
        let b = y * 6.341_f32 - x.cos();
        let mut acc = Accumulator918::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.428_f32 + y.sin();
        let b = y * 8.439_f32 - x.cos();
        let mut acc = Accumulator918::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.165_f32 + y.sin();
        let b = y * 6.062_f32 - x.cos();
        let mut acc = Accumulator918::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.217_f32 + y.sin();
        let b = y * 1.342_f32 - x.cos();
        let mut acc = Accumulator918::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.441_f32 + y.sin();
        let b = y * 3.279_f32 - x.cos();
        let mut acc = Accumulator918::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.165_f32 + y.sin();
        let b = y * 9.611_f32 - x.cos();
        let mut acc = Accumulator918::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.958_f32 + y.sin();
        let b = y * 2.943_f32 - x.cos();
        let mut acc = Accumulator918::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.601_f32 + y.sin();
        let b = y * 1.255_f32 - x.cos();
        let mut acc = Accumulator918::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.289_f32 + y.sin();
        let b = y * 4.434_f32 - x.cos();
        let mut acc = Accumulator918::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.899_f32 + y.sin();
        let b = y * 1.927_f32 - x.cos();
        let mut acc = Accumulator918::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.555_f32 + y.sin();
        let b = y * 4.417_f32 - x.cos();
        let mut acc = Accumulator918::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.616_f32 + y.sin();
        let b = y * 5.512_f32 - x.cos();
        let mut acc = Accumulator918::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.15_f32 + y.sin();
        let b = y * 8.181_f32 - x.cos();
        let mut acc = Accumulator918::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.905_f32 + y.sin();
        let b = y * 1.692_f32 - x.cos();
        let mut acc = Accumulator918::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.312_f32 + y.sin();
        let b = y * 6.31_f32 - x.cos();
        let mut acc = Accumulator918::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.617_f32 + y.sin();
        let b = y * 2.525_f32 - x.cos();
        let mut acc = Accumulator918::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.734_f32 + y.sin();
        let b = y * 4.271_f32 - x.cos();
        let mut acc = Accumulator918::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.455_f32 + y.sin();
        let b = y * 5.813_f32 - x.cos();
        let mut acc = Accumulator918::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.752_f32 + y.sin();
        let b = y * 1.662_f32 - x.cos();
        let mut acc = Accumulator918::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.266_f32 + y.sin();
        let b = y * 1.87_f32 - x.cos();
        let mut acc = Accumulator918::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_918(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m918-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_918() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_918(total as u64) % 997) as f32;
        total
    }
}

pub mod m919 {
    use super::*;

    pub struct Accumulator919<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator919<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.289_f32 + y.sin();
        let b = y * 9.054_f32 - x.cos();
        let mut acc = Accumulator919::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.317_f32 + y.sin();
        let b = y * 6.063_f32 - x.cos();
        let mut acc = Accumulator919::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.523_f32 + y.sin();
        let b = y * 0.446_f32 - x.cos();
        let mut acc = Accumulator919::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.567_f32 + y.sin();
        let b = y * 8.705_f32 - x.cos();
        let mut acc = Accumulator919::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.448_f32 + y.sin();
        let b = y * 4.293_f32 - x.cos();
        let mut acc = Accumulator919::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.823_f32 + y.sin();
        let b = y * 7.947_f32 - x.cos();
        let mut acc = Accumulator919::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.837_f32 + y.sin();
        let b = y * 3.729_f32 - x.cos();
        let mut acc = Accumulator919::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.433_f32 + y.sin();
        let b = y * 0.303_f32 - x.cos();
        let mut acc = Accumulator919::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.393_f32 + y.sin();
        let b = y * 9.717_f32 - x.cos();
        let mut acc = Accumulator919::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.285_f32 + y.sin();
        let b = y * 6.489_f32 - x.cos();
        let mut acc = Accumulator919::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.765_f32 + y.sin();
        let b = y * 5.851_f32 - x.cos();
        let mut acc = Accumulator919::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.251_f32 + y.sin();
        let b = y * 5.81_f32 - x.cos();
        let mut acc = Accumulator919::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.055_f32 + y.sin();
        let b = y * 0.548_f32 - x.cos();
        let mut acc = Accumulator919::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.081_f32 + y.sin();
        let b = y * 3.832_f32 - x.cos();
        let mut acc = Accumulator919::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.294_f32 + y.sin();
        let b = y * 6.775_f32 - x.cos();
        let mut acc = Accumulator919::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.754_f32 + y.sin();
        let b = y * 1.549_f32 - x.cos();
        let mut acc = Accumulator919::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.055_f32 + y.sin();
        let b = y * 8.231_f32 - x.cos();
        let mut acc = Accumulator919::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.825_f32 + y.sin();
        let b = y * 6.675_f32 - x.cos();
        let mut acc = Accumulator919::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.083_f32 + y.sin();
        let b = y * 6.616_f32 - x.cos();
        let mut acc = Accumulator919::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.261_f32 + y.sin();
        let b = y * 4.414_f32 - x.cos();
        let mut acc = Accumulator919::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_919(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_919() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_919(total as u64) % 997) as f32;
        total
    }
}

pub mod m920 {
    use super::*;

    pub struct Accumulator920<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator920<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.065_f32 + y.sin();
        let b = y * 6.95_f32 - x.cos();
        let mut acc = Accumulator920::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.728_f32 + y.sin();
        let b = y * 0.856_f32 - x.cos();
        let mut acc = Accumulator920::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.002_f32 + y.sin();
        let b = y * 1.523_f32 - x.cos();
        let mut acc = Accumulator920::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.252_f32 + y.sin();
        let b = y * 7.03_f32 - x.cos();
        let mut acc = Accumulator920::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.415_f32 + y.sin();
        let b = y * 8.799_f32 - x.cos();
        let mut acc = Accumulator920::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.066_f32 + y.sin();
        let b = y * 9.043_f32 - x.cos();
        let mut acc = Accumulator920::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.284_f32 + y.sin();
        let b = y * 8.341_f32 - x.cos();
        let mut acc = Accumulator920::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.25_f32 + y.sin();
        let b = y * 2.565_f32 - x.cos();
        let mut acc = Accumulator920::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.682_f32 + y.sin();
        let b = y * 2.474_f32 - x.cos();
        let mut acc = Accumulator920::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.139_f32 + y.sin();
        let b = y * 1.177_f32 - x.cos();
        let mut acc = Accumulator920::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.588_f32 + y.sin();
        let b = y * 2.048_f32 - x.cos();
        let mut acc = Accumulator920::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.198_f32 + y.sin();
        let b = y * 2.646_f32 - x.cos();
        let mut acc = Accumulator920::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.98_f32 + y.sin();
        let b = y * 3.092_f32 - x.cos();
        let mut acc = Accumulator920::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.975_f32 + y.sin();
        let b = y * 1.9_f32 - x.cos();
        let mut acc = Accumulator920::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.169_f32 + y.sin();
        let b = y * 6.177_f32 - x.cos();
        let mut acc = Accumulator920::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.263_f32 + y.sin();
        let b = y * 5.911_f32 - x.cos();
        let mut acc = Accumulator920::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.086_f32 + y.sin();
        let b = y * 5.673_f32 - x.cos();
        let mut acc = Accumulator920::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.983_f32 + y.sin();
        let b = y * 4.161_f32 - x.cos();
        let mut acc = Accumulator920::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.013_f32 + y.sin();
        let b = y * 4.581_f32 - x.cos();
        let mut acc = Accumulator920::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.871_f32 + y.sin();
        let b = y * 1.351_f32 - x.cos();
        let mut acc = Accumulator920::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_920(seed: u64) -> u64 {
        let re = Regex::new(r"m920-(\d+)").unwrap();
        let hay = format!("m920-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_920() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_920(total as u64) % 997) as f32;
        total
    }
}

pub mod m921 {
    use super::*;

    pub struct Accumulator921<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator921<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.851_f32 + y.sin();
        let b = y * 8.428_f32 - x.cos();
        let mut acc = Accumulator921::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.442_f32 + y.sin();
        let b = y * 3.994_f32 - x.cos();
        let mut acc = Accumulator921::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.438_f32 + y.sin();
        let b = y * 9.5_f32 - x.cos();
        let mut acc = Accumulator921::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.507_f32 + y.sin();
        let b = y * 9.309_f32 - x.cos();
        let mut acc = Accumulator921::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.655_f32 + y.sin();
        let b = y * 0.174_f32 - x.cos();
        let mut acc = Accumulator921::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.55_f32 + y.sin();
        let b = y * 5.27_f32 - x.cos();
        let mut acc = Accumulator921::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.002_f32 + y.sin();
        let b = y * 9.825_f32 - x.cos();
        let mut acc = Accumulator921::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.266_f32 + y.sin();
        let b = y * 6.097_f32 - x.cos();
        let mut acc = Accumulator921::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.938_f32 + y.sin();
        let b = y * 4.956_f32 - x.cos();
        let mut acc = Accumulator921::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.867_f32 + y.sin();
        let b = y * 3.969_f32 - x.cos();
        let mut acc = Accumulator921::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.862_f32 + y.sin();
        let b = y * 6.166_f32 - x.cos();
        let mut acc = Accumulator921::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.117_f32 + y.sin();
        let b = y * 6.405_f32 - x.cos();
        let mut acc = Accumulator921::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.061_f32 + y.sin();
        let b = y * 6.628_f32 - x.cos();
        let mut acc = Accumulator921::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.768_f32 + y.sin();
        let b = y * 8.365_f32 - x.cos();
        let mut acc = Accumulator921::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.026_f32 + y.sin();
        let b = y * 1.666_f32 - x.cos();
        let mut acc = Accumulator921::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.728_f32 + y.sin();
        let b = y * 0.929_f32 - x.cos();
        let mut acc = Accumulator921::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.532_f32 + y.sin();
        let b = y * 1.086_f32 - x.cos();
        let mut acc = Accumulator921::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.656_f32 + y.sin();
        let b = y * 9.157_f32 - x.cos();
        let mut acc = Accumulator921::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.801_f32 + y.sin();
        let b = y * 3.779_f32 - x.cos();
        let mut acc = Accumulator921::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.843_f32 + y.sin();
        let b = y * 0.889_f32 - x.cos();
        let mut acc = Accumulator921::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_921(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_921() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_921(total as u64) % 997) as f32;
        total
    }
}

pub mod m922 {
    use super::*;

    pub struct Accumulator922<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator922<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.127_f32 + y.sin();
        let b = y * 6.489_f32 - x.cos();
        let mut acc = Accumulator922::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.042_f32 + y.sin();
        let b = y * 8.148_f32 - x.cos();
        let mut acc = Accumulator922::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.448_f32 + y.sin();
        let b = y * 3.419_f32 - x.cos();
        let mut acc = Accumulator922::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.232_f32 + y.sin();
        let b = y * 1.972_f32 - x.cos();
        let mut acc = Accumulator922::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.165_f32 + y.sin();
        let b = y * 6.293_f32 - x.cos();
        let mut acc = Accumulator922::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.546_f32 + y.sin();
        let b = y * 8.79_f32 - x.cos();
        let mut acc = Accumulator922::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.279_f32 + y.sin();
        let b = y * 9.617_f32 - x.cos();
        let mut acc = Accumulator922::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.816_f32 + y.sin();
        let b = y * 8.56_f32 - x.cos();
        let mut acc = Accumulator922::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.968_f32 + y.sin();
        let b = y * 6.421_f32 - x.cos();
        let mut acc = Accumulator922::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.895_f32 + y.sin();
        let b = y * 8.417_f32 - x.cos();
        let mut acc = Accumulator922::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.098_f32 + y.sin();
        let b = y * 3.892_f32 - x.cos();
        let mut acc = Accumulator922::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.632_f32 + y.sin();
        let b = y * 6.696_f32 - x.cos();
        let mut acc = Accumulator922::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.28_f32 + y.sin();
        let b = y * 6.168_f32 - x.cos();
        let mut acc = Accumulator922::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.012_f32 + y.sin();
        let b = y * 9.629_f32 - x.cos();
        let mut acc = Accumulator922::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.664_f32 + y.sin();
        let b = y * 3.578_f32 - x.cos();
        let mut acc = Accumulator922::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.927_f32 + y.sin();
        let b = y * 5.898_f32 - x.cos();
        let mut acc = Accumulator922::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.544_f32 + y.sin();
        let b = y * 8.033_f32 - x.cos();
        let mut acc = Accumulator922::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.771_f32 + y.sin();
        let b = y * 6.079_f32 - x.cos();
        let mut acc = Accumulator922::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.232_f32 + y.sin();
        let b = y * 4.585_f32 - x.cos();
        let mut acc = Accumulator922::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.838_f32 + y.sin();
        let b = y * 1.506_f32 - x.cos();
        let mut acc = Accumulator922::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_922(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(922u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_922() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_922(total as u64) % 997) as f32;
        total
    }
}

pub mod m923 {
    use super::*;

    pub struct Accumulator923<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator923<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.358_f32 + y.sin();
        let b = y * 1.127_f32 - x.cos();
        let mut acc = Accumulator923::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.904_f32 + y.sin();
        let b = y * 7.586_f32 - x.cos();
        let mut acc = Accumulator923::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.519_f32 + y.sin();
        let b = y * 8.786_f32 - x.cos();
        let mut acc = Accumulator923::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.592_f32 + y.sin();
        let b = y * 0.208_f32 - x.cos();
        let mut acc = Accumulator923::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.638_f32 + y.sin();
        let b = y * 5.929_f32 - x.cos();
        let mut acc = Accumulator923::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.268_f32 + y.sin();
        let b = y * 2.682_f32 - x.cos();
        let mut acc = Accumulator923::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.661_f32 + y.sin();
        let b = y * 5.26_f32 - x.cos();
        let mut acc = Accumulator923::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.01_f32 + y.sin();
        let b = y * 6.33_f32 - x.cos();
        let mut acc = Accumulator923::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.117_f32 + y.sin();
        let b = y * 1.966_f32 - x.cos();
        let mut acc = Accumulator923::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.171_f32 + y.sin();
        let b = y * 8.654_f32 - x.cos();
        let mut acc = Accumulator923::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.32_f32 + y.sin();
        let b = y * 4.715_f32 - x.cos();
        let mut acc = Accumulator923::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.815_f32 + y.sin();
        let b = y * 9.296_f32 - x.cos();
        let mut acc = Accumulator923::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.375_f32 + y.sin();
        let b = y * 8.774_f32 - x.cos();
        let mut acc = Accumulator923::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.5_f32 + y.sin();
        let b = y * 0.598_f32 - x.cos();
        let mut acc = Accumulator923::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.842_f32 + y.sin();
        let b = y * 9.712_f32 - x.cos();
        let mut acc = Accumulator923::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.441_f32 + y.sin();
        let b = y * 9.642_f32 - x.cos();
        let mut acc = Accumulator923::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.203_f32 + y.sin();
        let b = y * 7.89_f32 - x.cos();
        let mut acc = Accumulator923::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.983_f32 + y.sin();
        let b = y * 6.228_f32 - x.cos();
        let mut acc = Accumulator923::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.1_f32 + y.sin();
        let b = y * 9.439_f32 - x.cos();
        let mut acc = Accumulator923::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.642_f32 + y.sin();
        let b = y * 9.07_f32 - x.cos();
        let mut acc = Accumulator923::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_923(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_923() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_923(total as u64) % 997) as f32;
        total
    }
}

pub mod m924 {
    use super::*;

    pub struct Accumulator924<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator924<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.223_f32 + y.sin();
        let b = y * 1.161_f32 - x.cos();
        let mut acc = Accumulator924::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.44_f32 + y.sin();
        let b = y * 8.122_f32 - x.cos();
        let mut acc = Accumulator924::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.734_f32 + y.sin();
        let b = y * 5.805_f32 - x.cos();
        let mut acc = Accumulator924::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.504_f32 + y.sin();
        let b = y * 2.528_f32 - x.cos();
        let mut acc = Accumulator924::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.618_f32 + y.sin();
        let b = y * 5.978_f32 - x.cos();
        let mut acc = Accumulator924::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.76_f32 + y.sin();
        let b = y * 4.249_f32 - x.cos();
        let mut acc = Accumulator924::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.241_f32 + y.sin();
        let b = y * 0.158_f32 - x.cos();
        let mut acc = Accumulator924::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.128_f32 + y.sin();
        let b = y * 6.039_f32 - x.cos();
        let mut acc = Accumulator924::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.874_f32 + y.sin();
        let b = y * 9.601_f32 - x.cos();
        let mut acc = Accumulator924::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.598_f32 + y.sin();
        let b = y * 2.746_f32 - x.cos();
        let mut acc = Accumulator924::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.386_f32 + y.sin();
        let b = y * 5.274_f32 - x.cos();
        let mut acc = Accumulator924::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.855_f32 + y.sin();
        let b = y * 7.395_f32 - x.cos();
        let mut acc = Accumulator924::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.599_f32 + y.sin();
        let b = y * 9.662_f32 - x.cos();
        let mut acc = Accumulator924::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.486_f32 + y.sin();
        let b = y * 7.297_f32 - x.cos();
        let mut acc = Accumulator924::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.141_f32 + y.sin();
        let b = y * 8.854_f32 - x.cos();
        let mut acc = Accumulator924::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.871_f32 + y.sin();
        let b = y * 9.873_f32 - x.cos();
        let mut acc = Accumulator924::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.645_f32 + y.sin();
        let b = y * 4.503_f32 - x.cos();
        let mut acc = Accumulator924::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.488_f32 + y.sin();
        let b = y * 0.475_f32 - x.cos();
        let mut acc = Accumulator924::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.515_f32 + y.sin();
        let b = y * 8.168_f32 - x.cos();
        let mut acc = Accumulator924::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.917_f32 + y.sin();
        let b = y * 5.509_f32 - x.cos();
        let mut acc = Accumulator924::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_924(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_924() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_924(total as u64) % 997) as f32;
        total
    }
}

pub mod m925 {
    use super::*;

    pub struct Accumulator925<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator925<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.268_f32 + y.sin();
        let b = y * 3.194_f32 - x.cos();
        let mut acc = Accumulator925::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.985_f32 + y.sin();
        let b = y * 5.879_f32 - x.cos();
        let mut acc = Accumulator925::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.305_f32 + y.sin();
        let b = y * 0.737_f32 - x.cos();
        let mut acc = Accumulator925::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.412_f32 + y.sin();
        let b = y * 0.687_f32 - x.cos();
        let mut acc = Accumulator925::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.493_f32 + y.sin();
        let b = y * 3.556_f32 - x.cos();
        let mut acc = Accumulator925::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.76_f32 + y.sin();
        let b = y * 4.709_f32 - x.cos();
        let mut acc = Accumulator925::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.685_f32 + y.sin();
        let b = y * 3.641_f32 - x.cos();
        let mut acc = Accumulator925::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.068_f32 + y.sin();
        let b = y * 3.477_f32 - x.cos();
        let mut acc = Accumulator925::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.836_f32 + y.sin();
        let b = y * 2.391_f32 - x.cos();
        let mut acc = Accumulator925::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.532_f32 + y.sin();
        let b = y * 4.447_f32 - x.cos();
        let mut acc = Accumulator925::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.087_f32 + y.sin();
        let b = y * 5.281_f32 - x.cos();
        let mut acc = Accumulator925::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.203_f32 + y.sin();
        let b = y * 6.024_f32 - x.cos();
        let mut acc = Accumulator925::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.139_f32 + y.sin();
        let b = y * 9.264_f32 - x.cos();
        let mut acc = Accumulator925::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.801_f32 + y.sin();
        let b = y * 1.962_f32 - x.cos();
        let mut acc = Accumulator925::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.698_f32 + y.sin();
        let b = y * 3.764_f32 - x.cos();
        let mut acc = Accumulator925::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.712_f32 + y.sin();
        let b = y * 4.49_f32 - x.cos();
        let mut acc = Accumulator925::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.398_f32 + y.sin();
        let b = y * 8.458_f32 - x.cos();
        let mut acc = Accumulator925::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.112_f32 + y.sin();
        let b = y * 1.711_f32 - x.cos();
        let mut acc = Accumulator925::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.982_f32 + y.sin();
        let b = y * 3.344_f32 - x.cos();
        let mut acc = Accumulator925::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.633_f32 + y.sin();
        let b = y * 1.2_f32 - x.cos();
        let mut acc = Accumulator925::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_925(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m925-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_925() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_925(total as u64) % 997) as f32;
        total
    }
}

pub mod m926 {
    use super::*;

    pub struct Accumulator926<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator926<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.492_f32 + y.sin();
        let b = y * 3.845_f32 - x.cos();
        let mut acc = Accumulator926::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.284_f32 + y.sin();
        let b = y * 1.058_f32 - x.cos();
        let mut acc = Accumulator926::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.643_f32 + y.sin();
        let b = y * 6.26_f32 - x.cos();
        let mut acc = Accumulator926::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.4_f32 + y.sin();
        let b = y * 5.272_f32 - x.cos();
        let mut acc = Accumulator926::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.129_f32 + y.sin();
        let b = y * 7.263_f32 - x.cos();
        let mut acc = Accumulator926::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.873_f32 + y.sin();
        let b = y * 6.836_f32 - x.cos();
        let mut acc = Accumulator926::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.829_f32 + y.sin();
        let b = y * 6.029_f32 - x.cos();
        let mut acc = Accumulator926::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.385_f32 + y.sin();
        let b = y * 5.597_f32 - x.cos();
        let mut acc = Accumulator926::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.108_f32 + y.sin();
        let b = y * 5.98_f32 - x.cos();
        let mut acc = Accumulator926::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.333_f32 + y.sin();
        let b = y * 1.047_f32 - x.cos();
        let mut acc = Accumulator926::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.589_f32 + y.sin();
        let b = y * 5.026_f32 - x.cos();
        let mut acc = Accumulator926::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.662_f32 + y.sin();
        let b = y * 6.906_f32 - x.cos();
        let mut acc = Accumulator926::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.054_f32 + y.sin();
        let b = y * 4.791_f32 - x.cos();
        let mut acc = Accumulator926::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.829_f32 + y.sin();
        let b = y * 8.561_f32 - x.cos();
        let mut acc = Accumulator926::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.508_f32 + y.sin();
        let b = y * 9.793_f32 - x.cos();
        let mut acc = Accumulator926::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.117_f32 + y.sin();
        let b = y * 6.513_f32 - x.cos();
        let mut acc = Accumulator926::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.467_f32 + y.sin();
        let b = y * 8.479_f32 - x.cos();
        let mut acc = Accumulator926::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.629_f32 + y.sin();
        let b = y * 4.1_f32 - x.cos();
        let mut acc = Accumulator926::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.672_f32 + y.sin();
        let b = y * 6.153_f32 - x.cos();
        let mut acc = Accumulator926::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.173_f32 + y.sin();
        let b = y * 2.633_f32 - x.cos();
        let mut acc = Accumulator926::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_926(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_926() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_926(total as u64) % 997) as f32;
        total
    }
}

pub mod m927 {
    use super::*;

    pub struct Accumulator927<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator927<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.416_f32 + y.sin();
        let b = y * 3.36_f32 - x.cos();
        let mut acc = Accumulator927::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.845_f32 + y.sin();
        let b = y * 6.579_f32 - x.cos();
        let mut acc = Accumulator927::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.451_f32 + y.sin();
        let b = y * 1.869_f32 - x.cos();
        let mut acc = Accumulator927::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.234_f32 + y.sin();
        let b = y * 4.074_f32 - x.cos();
        let mut acc = Accumulator927::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.586_f32 + y.sin();
        let b = y * 8.08_f32 - x.cos();
        let mut acc = Accumulator927::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.351_f32 + y.sin();
        let b = y * 1.438_f32 - x.cos();
        let mut acc = Accumulator927::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.845_f32 + y.sin();
        let b = y * 2.583_f32 - x.cos();
        let mut acc = Accumulator927::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.065_f32 + y.sin();
        let b = y * 8.529_f32 - x.cos();
        let mut acc = Accumulator927::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.821_f32 + y.sin();
        let b = y * 7.821_f32 - x.cos();
        let mut acc = Accumulator927::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.879_f32 + y.sin();
        let b = y * 9.39_f32 - x.cos();
        let mut acc = Accumulator927::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.659_f32 + y.sin();
        let b = y * 7.686_f32 - x.cos();
        let mut acc = Accumulator927::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.881_f32 + y.sin();
        let b = y * 6.884_f32 - x.cos();
        let mut acc = Accumulator927::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.451_f32 + y.sin();
        let b = y * 6.308_f32 - x.cos();
        let mut acc = Accumulator927::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.724_f32 + y.sin();
        let b = y * 8.073_f32 - x.cos();
        let mut acc = Accumulator927::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.607_f32 + y.sin();
        let b = y * 2.173_f32 - x.cos();
        let mut acc = Accumulator927::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.185_f32 + y.sin();
        let b = y * 1.821_f32 - x.cos();
        let mut acc = Accumulator927::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.361_f32 + y.sin();
        let b = y * 6.583_f32 - x.cos();
        let mut acc = Accumulator927::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.075_f32 + y.sin();
        let b = y * 5.238_f32 - x.cos();
        let mut acc = Accumulator927::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.374_f32 + y.sin();
        let b = y * 1.995_f32 - x.cos();
        let mut acc = Accumulator927::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.256_f32 + y.sin();
        let b = y * 6.396_f32 - x.cos();
        let mut acc = Accumulator927::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_927(seed: u64) -> u64 {
        let re = Regex::new(r"m927-(\d+)").unwrap();
        let hay = format!("m927-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_927() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_927(total as u64) % 997) as f32;
        total
    }
}

pub mod m928 {
    use super::*;

    pub struct Accumulator928<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator928<T> {
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
        let b = y * 5.33_f32 - x.cos();
        let mut acc = Accumulator928::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.885_f32 + y.sin();
        let b = y * 8.267_f32 - x.cos();
        let mut acc = Accumulator928::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.804_f32 + y.sin();
        let b = y * 5.431_f32 - x.cos();
        let mut acc = Accumulator928::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.643_f32 + y.sin();
        let b = y * 8.173_f32 - x.cos();
        let mut acc = Accumulator928::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.048_f32 + y.sin();
        let b = y * 1.544_f32 - x.cos();
        let mut acc = Accumulator928::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.143_f32 + y.sin();
        let b = y * 6.209_f32 - x.cos();
        let mut acc = Accumulator928::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.159_f32 + y.sin();
        let b = y * 8.742_f32 - x.cos();
        let mut acc = Accumulator928::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.43_f32 + y.sin();
        let b = y * 2.944_f32 - x.cos();
        let mut acc = Accumulator928::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.244_f32 + y.sin();
        let b = y * 3.079_f32 - x.cos();
        let mut acc = Accumulator928::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.799_f32 + y.sin();
        let b = y * 1.296_f32 - x.cos();
        let mut acc = Accumulator928::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.702_f32 + y.sin();
        let b = y * 2.815_f32 - x.cos();
        let mut acc = Accumulator928::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.719_f32 + y.sin();
        let b = y * 9.694_f32 - x.cos();
        let mut acc = Accumulator928::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.331_f32 + y.sin();
        let b = y * 9.659_f32 - x.cos();
        let mut acc = Accumulator928::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.352_f32 + y.sin();
        let b = y * 1.409_f32 - x.cos();
        let mut acc = Accumulator928::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.077_f32 + y.sin();
        let b = y * 0.884_f32 - x.cos();
        let mut acc = Accumulator928::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.589_f32 + y.sin();
        let b = y * 5.113_f32 - x.cos();
        let mut acc = Accumulator928::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.492_f32 + y.sin();
        let b = y * 5.712_f32 - x.cos();
        let mut acc = Accumulator928::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.147_f32 + y.sin();
        let b = y * 3.182_f32 - x.cos();
        let mut acc = Accumulator928::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.39_f32 + y.sin();
        let b = y * 4.787_f32 - x.cos();
        let mut acc = Accumulator928::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.635_f32 + y.sin();
        let b = y * 0.579_f32 - x.cos();
        let mut acc = Accumulator928::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_928(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_928() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_928(total as u64) % 997) as f32;
        total
    }
}

pub mod m929 {
    use super::*;

    pub struct Accumulator929<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator929<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.413_f32 + y.sin();
        let b = y * 1.096_f32 - x.cos();
        let mut acc = Accumulator929::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.127_f32 + y.sin();
        let b = y * 9.542_f32 - x.cos();
        let mut acc = Accumulator929::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.601_f32 + y.sin();
        let b = y * 4.356_f32 - x.cos();
        let mut acc = Accumulator929::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.928_f32 + y.sin();
        let b = y * 9.313_f32 - x.cos();
        let mut acc = Accumulator929::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.005_f32 + y.sin();
        let b = y * 3.161_f32 - x.cos();
        let mut acc = Accumulator929::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.357_f32 + y.sin();
        let b = y * 3.57_f32 - x.cos();
        let mut acc = Accumulator929::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.31_f32 + y.sin();
        let b = y * 7.189_f32 - x.cos();
        let mut acc = Accumulator929::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.359_f32 + y.sin();
        let b = y * 0.617_f32 - x.cos();
        let mut acc = Accumulator929::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.946_f32 + y.sin();
        let b = y * 1.157_f32 - x.cos();
        let mut acc = Accumulator929::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.723_f32 + y.sin();
        let b = y * 3.411_f32 - x.cos();
        let mut acc = Accumulator929::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.69_f32 + y.sin();
        let b = y * 3.279_f32 - x.cos();
        let mut acc = Accumulator929::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.116_f32 + y.sin();
        let b = y * 5.415_f32 - x.cos();
        let mut acc = Accumulator929::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.0_f32 + y.sin();
        let b = y * 3.88_f32 - x.cos();
        let mut acc = Accumulator929::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.97_f32 + y.sin();
        let b = y * 3.191_f32 - x.cos();
        let mut acc = Accumulator929::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.01_f32 + y.sin();
        let b = y * 0.832_f32 - x.cos();
        let mut acc = Accumulator929::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.039_f32 + y.sin();
        let b = y * 1.879_f32 - x.cos();
        let mut acc = Accumulator929::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.337_f32 + y.sin();
        let b = y * 7.021_f32 - x.cos();
        let mut acc = Accumulator929::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.267_f32 + y.sin();
        let b = y * 9.384_f32 - x.cos();
        let mut acc = Accumulator929::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.413_f32 + y.sin();
        let b = y * 5.982_f32 - x.cos();
        let mut acc = Accumulator929::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.163_f32 + y.sin();
        let b = y * 3.654_f32 - x.cos();
        let mut acc = Accumulator929::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_929(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(929u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_929() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_929(total as u64) % 997) as f32;
        total
    }
}

pub mod m930 {
    use super::*;

    pub struct Accumulator930<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator930<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.245_f32 + y.sin();
        let b = y * 3.366_f32 - x.cos();
        let mut acc = Accumulator930::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.242_f32 + y.sin();
        let b = y * 6.303_f32 - x.cos();
        let mut acc = Accumulator930::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.07_f32 + y.sin();
        let b = y * 6.424_f32 - x.cos();
        let mut acc = Accumulator930::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.256_f32 + y.sin();
        let b = y * 1.993_f32 - x.cos();
        let mut acc = Accumulator930::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.803_f32 + y.sin();
        let b = y * 6.402_f32 - x.cos();
        let mut acc = Accumulator930::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.58_f32 + y.sin();
        let b = y * 6.83_f32 - x.cos();
        let mut acc = Accumulator930::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.128_f32 + y.sin();
        let b = y * 8.875_f32 - x.cos();
        let mut acc = Accumulator930::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.653_f32 + y.sin();
        let b = y * 0.902_f32 - x.cos();
        let mut acc = Accumulator930::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.854_f32 + y.sin();
        let b = y * 2.942_f32 - x.cos();
        let mut acc = Accumulator930::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.549_f32 + y.sin();
        let b = y * 2.826_f32 - x.cos();
        let mut acc = Accumulator930::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.302_f32 + y.sin();
        let b = y * 7.164_f32 - x.cos();
        let mut acc = Accumulator930::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.875_f32 + y.sin();
        let b = y * 2.65_f32 - x.cos();
        let mut acc = Accumulator930::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.573_f32 + y.sin();
        let b = y * 9.637_f32 - x.cos();
        let mut acc = Accumulator930::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.893_f32 + y.sin();
        let b = y * 3.274_f32 - x.cos();
        let mut acc = Accumulator930::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.417_f32 + y.sin();
        let b = y * 9.367_f32 - x.cos();
        let mut acc = Accumulator930::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.024_f32 + y.sin();
        let b = y * 7.481_f32 - x.cos();
        let mut acc = Accumulator930::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.392_f32 + y.sin();
        let b = y * 8.495_f32 - x.cos();
        let mut acc = Accumulator930::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.002_f32 + y.sin();
        let b = y * 4.417_f32 - x.cos();
        let mut acc = Accumulator930::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.872_f32 + y.sin();
        let b = y * 5.213_f32 - x.cos();
        let mut acc = Accumulator930::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.711_f32 + y.sin();
        let b = y * 8.781_f32 - x.cos();
        let mut acc = Accumulator930::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_930(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_930() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_930(total as u64) % 997) as f32;
        total
    }
}

pub mod m931 {
    use super::*;

    pub struct Accumulator931<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator931<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.66_f32 + y.sin();
        let b = y * 4.456_f32 - x.cos();
        let mut acc = Accumulator931::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.032_f32 + y.sin();
        let b = y * 1.759_f32 - x.cos();
        let mut acc = Accumulator931::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.211_f32 + y.sin();
        let b = y * 7.322_f32 - x.cos();
        let mut acc = Accumulator931::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.651_f32 + y.sin();
        let b = y * 1.271_f32 - x.cos();
        let mut acc = Accumulator931::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.112_f32 + y.sin();
        let b = y * 4.914_f32 - x.cos();
        let mut acc = Accumulator931::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.851_f32 + y.sin();
        let b = y * 0.411_f32 - x.cos();
        let mut acc = Accumulator931::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.304_f32 + y.sin();
        let b = y * 0.907_f32 - x.cos();
        let mut acc = Accumulator931::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.187_f32 + y.sin();
        let b = y * 5.735_f32 - x.cos();
        let mut acc = Accumulator931::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.078_f32 + y.sin();
        let b = y * 7.033_f32 - x.cos();
        let mut acc = Accumulator931::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.491_f32 + y.sin();
        let b = y * 6.935_f32 - x.cos();
        let mut acc = Accumulator931::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.292_f32 + y.sin();
        let b = y * 3.588_f32 - x.cos();
        let mut acc = Accumulator931::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.885_f32 + y.sin();
        let b = y * 6.19_f32 - x.cos();
        let mut acc = Accumulator931::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.45_f32 + y.sin();
        let b = y * 7.086_f32 - x.cos();
        let mut acc = Accumulator931::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.761_f32 + y.sin();
        let b = y * 1.079_f32 - x.cos();
        let mut acc = Accumulator931::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.235_f32 + y.sin();
        let b = y * 5.817_f32 - x.cos();
        let mut acc = Accumulator931::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.682_f32 + y.sin();
        let b = y * 3.124_f32 - x.cos();
        let mut acc = Accumulator931::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.679_f32 + y.sin();
        let b = y * 6.297_f32 - x.cos();
        let mut acc = Accumulator931::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.302_f32 + y.sin();
        let b = y * 5.339_f32 - x.cos();
        let mut acc = Accumulator931::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.449_f32 + y.sin();
        let b = y * 6.159_f32 - x.cos();
        let mut acc = Accumulator931::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.486_f32 + y.sin();
        let b = y * 2.203_f32 - x.cos();
        let mut acc = Accumulator931::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_931(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_931() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_931(total as u64) % 997) as f32;
        total
    }
}

pub mod m932 {
    use super::*;

    pub struct Accumulator932<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator932<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.289_f32 + y.sin();
        let b = y * 5.646_f32 - x.cos();
        let mut acc = Accumulator932::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.923_f32 + y.sin();
        let b = y * 4.473_f32 - x.cos();
        let mut acc = Accumulator932::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.175_f32 + y.sin();
        let b = y * 8.221_f32 - x.cos();
        let mut acc = Accumulator932::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.662_f32 + y.sin();
        let b = y * 7.474_f32 - x.cos();
        let mut acc = Accumulator932::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.253_f32 + y.sin();
        let b = y * 9.216_f32 - x.cos();
        let mut acc = Accumulator932::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.059_f32 + y.sin();
        let b = y * 7.825_f32 - x.cos();
        let mut acc = Accumulator932::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.958_f32 + y.sin();
        let b = y * 1.493_f32 - x.cos();
        let mut acc = Accumulator932::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.797_f32 + y.sin();
        let b = y * 9.364_f32 - x.cos();
        let mut acc = Accumulator932::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.221_f32 + y.sin();
        let b = y * 2.504_f32 - x.cos();
        let mut acc = Accumulator932::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.287_f32 + y.sin();
        let b = y * 8.086_f32 - x.cos();
        let mut acc = Accumulator932::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.941_f32 + y.sin();
        let b = y * 9.57_f32 - x.cos();
        let mut acc = Accumulator932::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.749_f32 + y.sin();
        let b = y * 9.297_f32 - x.cos();
        let mut acc = Accumulator932::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.234_f32 + y.sin();
        let b = y * 7.24_f32 - x.cos();
        let mut acc = Accumulator932::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.005_f32 + y.sin();
        let b = y * 8.66_f32 - x.cos();
        let mut acc = Accumulator932::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.198_f32 + y.sin();
        let b = y * 0.801_f32 - x.cos();
        let mut acc = Accumulator932::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.388_f32 + y.sin();
        let b = y * 0.56_f32 - x.cos();
        let mut acc = Accumulator932::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.558_f32 + y.sin();
        let b = y * 8.308_f32 - x.cos();
        let mut acc = Accumulator932::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.18_f32 + y.sin();
        let b = y * 0.538_f32 - x.cos();
        let mut acc = Accumulator932::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.011_f32 + y.sin();
        let b = y * 4.4_f32 - x.cos();
        let mut acc = Accumulator932::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.684_f32 + y.sin();
        let b = y * 0.873_f32 - x.cos();
        let mut acc = Accumulator932::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_932(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m932-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_932() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_932(total as u64) % 997) as f32;
        total
    }
}

pub mod m933 {
    use super::*;

    pub struct Accumulator933<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator933<T> {
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
        let b = y * 6.325_f32 - x.cos();
        let mut acc = Accumulator933::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.273_f32 + y.sin();
        let b = y * 7.365_f32 - x.cos();
        let mut acc = Accumulator933::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.42_f32 + y.sin();
        let b = y * 3.944_f32 - x.cos();
        let mut acc = Accumulator933::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.49_f32 + y.sin();
        let b = y * 0.594_f32 - x.cos();
        let mut acc = Accumulator933::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.94_f32 + y.sin();
        let b = y * 0.156_f32 - x.cos();
        let mut acc = Accumulator933::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.779_f32 + y.sin();
        let b = y * 4.451_f32 - x.cos();
        let mut acc = Accumulator933::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.193_f32 + y.sin();
        let b = y * 7.84_f32 - x.cos();
        let mut acc = Accumulator933::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.418_f32 + y.sin();
        let b = y * 1.625_f32 - x.cos();
        let mut acc = Accumulator933::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.655_f32 + y.sin();
        let b = y * 0.595_f32 - x.cos();
        let mut acc = Accumulator933::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.907_f32 + y.sin();
        let b = y * 7.226_f32 - x.cos();
        let mut acc = Accumulator933::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.692_f32 + y.sin();
        let b = y * 0.483_f32 - x.cos();
        let mut acc = Accumulator933::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.078_f32 + y.sin();
        let b = y * 1.176_f32 - x.cos();
        let mut acc = Accumulator933::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.141_f32 + y.sin();
        let b = y * 9.18_f32 - x.cos();
        let mut acc = Accumulator933::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.317_f32 + y.sin();
        let b = y * 1.317_f32 - x.cos();
        let mut acc = Accumulator933::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.751_f32 + y.sin();
        let b = y * 2.539_f32 - x.cos();
        let mut acc = Accumulator933::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.155_f32 + y.sin();
        let b = y * 9.774_f32 - x.cos();
        let mut acc = Accumulator933::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.742_f32 + y.sin();
        let b = y * 6.783_f32 - x.cos();
        let mut acc = Accumulator933::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.229_f32 + y.sin();
        let b = y * 2.334_f32 - x.cos();
        let mut acc = Accumulator933::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.313_f32 + y.sin();
        let b = y * 1.303_f32 - x.cos();
        let mut acc = Accumulator933::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.197_f32 + y.sin();
        let b = y * 7.919_f32 - x.cos();
        let mut acc = Accumulator933::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_933(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_933() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_933(total as u64) % 997) as f32;
        total
    }
}

pub mod m934 {
    use super::*;

    pub struct Accumulator934<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator934<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.424_f32 + y.sin();
        let b = y * 4.927_f32 - x.cos();
        let mut acc = Accumulator934::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.48_f32 + y.sin();
        let b = y * 4.862_f32 - x.cos();
        let mut acc = Accumulator934::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.288_f32 + y.sin();
        let b = y * 7.932_f32 - x.cos();
        let mut acc = Accumulator934::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.569_f32 + y.sin();
        let b = y * 9.424_f32 - x.cos();
        let mut acc = Accumulator934::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.465_f32 + y.sin();
        let b = y * 7.903_f32 - x.cos();
        let mut acc = Accumulator934::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.338_f32 + y.sin();
        let b = y * 4.755_f32 - x.cos();
        let mut acc = Accumulator934::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.134_f32 + y.sin();
        let b = y * 7.969_f32 - x.cos();
        let mut acc = Accumulator934::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.668_f32 + y.sin();
        let b = y * 2.786_f32 - x.cos();
        let mut acc = Accumulator934::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.815_f32 + y.sin();
        let b = y * 4.351_f32 - x.cos();
        let mut acc = Accumulator934::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.148_f32 + y.sin();
        let b = y * 9.787_f32 - x.cos();
        let mut acc = Accumulator934::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.756_f32 + y.sin();
        let b = y * 9.785_f32 - x.cos();
        let mut acc = Accumulator934::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.733_f32 + y.sin();
        let b = y * 6.496_f32 - x.cos();
        let mut acc = Accumulator934::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.533_f32 + y.sin();
        let b = y * 7.156_f32 - x.cos();
        let mut acc = Accumulator934::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.731_f32 + y.sin();
        let b = y * 5.043_f32 - x.cos();
        let mut acc = Accumulator934::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.957_f32 + y.sin();
        let b = y * 1.547_f32 - x.cos();
        let mut acc = Accumulator934::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.445_f32 + y.sin();
        let b = y * 2.993_f32 - x.cos();
        let mut acc = Accumulator934::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.155_f32 + y.sin();
        let b = y * 5.113_f32 - x.cos();
        let mut acc = Accumulator934::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.872_f32 + y.sin();
        let b = y * 3.587_f32 - x.cos();
        let mut acc = Accumulator934::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.536_f32 + y.sin();
        let b = y * 0.423_f32 - x.cos();
        let mut acc = Accumulator934::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.157_f32 + y.sin();
        let b = y * 3.562_f32 - x.cos();
        let mut acc = Accumulator934::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_934(seed: u64) -> u64 {
        let re = Regex::new(r"m934-(\d+)").unwrap();
        let hay = format!("m934-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_934() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_934(total as u64) % 997) as f32;
        total
    }
}

pub mod m935 {
    use super::*;

    pub struct Accumulator935<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator935<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.565_f32 + y.sin();
        let b = y * 5.301_f32 - x.cos();
        let mut acc = Accumulator935::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.266_f32 + y.sin();
        let b = y * 6.521_f32 - x.cos();
        let mut acc = Accumulator935::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.692_f32 + y.sin();
        let b = y * 8.169_f32 - x.cos();
        let mut acc = Accumulator935::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.225_f32 + y.sin();
        let b = y * 6.999_f32 - x.cos();
        let mut acc = Accumulator935::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.97_f32 + y.sin();
        let b = y * 9.343_f32 - x.cos();
        let mut acc = Accumulator935::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.257_f32 + y.sin();
        let b = y * 7.538_f32 - x.cos();
        let mut acc = Accumulator935::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.298_f32 + y.sin();
        let b = y * 1.344_f32 - x.cos();
        let mut acc = Accumulator935::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.952_f32 + y.sin();
        let b = y * 0.519_f32 - x.cos();
        let mut acc = Accumulator935::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.467_f32 + y.sin();
        let b = y * 2.003_f32 - x.cos();
        let mut acc = Accumulator935::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.453_f32 + y.sin();
        let b = y * 2.845_f32 - x.cos();
        let mut acc = Accumulator935::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.583_f32 + y.sin();
        let b = y * 9.841_f32 - x.cos();
        let mut acc = Accumulator935::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.676_f32 + y.sin();
        let b = y * 1.785_f32 - x.cos();
        let mut acc = Accumulator935::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.93_f32 + y.sin();
        let b = y * 5.142_f32 - x.cos();
        let mut acc = Accumulator935::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.45_f32 + y.sin();
        let b = y * 6.611_f32 - x.cos();
        let mut acc = Accumulator935::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.405_f32 + y.sin();
        let b = y * 7.497_f32 - x.cos();
        let mut acc = Accumulator935::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.351_f32 + y.sin();
        let b = y * 3.073_f32 - x.cos();
        let mut acc = Accumulator935::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.545_f32 + y.sin();
        let b = y * 5.676_f32 - x.cos();
        let mut acc = Accumulator935::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.772_f32 + y.sin();
        let b = y * 2.918_f32 - x.cos();
        let mut acc = Accumulator935::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.893_f32 + y.sin();
        let b = y * 4.896_f32 - x.cos();
        let mut acc = Accumulator935::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.722_f32 + y.sin();
        let b = y * 1.161_f32 - x.cos();
        let mut acc = Accumulator935::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_935(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_935() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_935(total as u64) % 997) as f32;
        total
    }
}

pub mod m936 {
    use super::*;

    pub struct Accumulator936<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator936<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.745_f32 + y.sin();
        let b = y * 7.455_f32 - x.cos();
        let mut acc = Accumulator936::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.212_f32 + y.sin();
        let b = y * 1.826_f32 - x.cos();
        let mut acc = Accumulator936::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.051_f32 + y.sin();
        let b = y * 9.296_f32 - x.cos();
        let mut acc = Accumulator936::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.611_f32 + y.sin();
        let b = y * 3.385_f32 - x.cos();
        let mut acc = Accumulator936::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.074_f32 + y.sin();
        let b = y * 5.062_f32 - x.cos();
        let mut acc = Accumulator936::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.207_f32 + y.sin();
        let b = y * 5.15_f32 - x.cos();
        let mut acc = Accumulator936::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.483_f32 + y.sin();
        let b = y * 0.416_f32 - x.cos();
        let mut acc = Accumulator936::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.381_f32 + y.sin();
        let b = y * 4.268_f32 - x.cos();
        let mut acc = Accumulator936::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.367_f32 + y.sin();
        let b = y * 9.366_f32 - x.cos();
        let mut acc = Accumulator936::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.118_f32 + y.sin();
        let b = y * 0.261_f32 - x.cos();
        let mut acc = Accumulator936::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.021_f32 + y.sin();
        let b = y * 9.285_f32 - x.cos();
        let mut acc = Accumulator936::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.86_f32 + y.sin();
        let b = y * 5.824_f32 - x.cos();
        let mut acc = Accumulator936::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.647_f32 + y.sin();
        let b = y * 7.897_f32 - x.cos();
        let mut acc = Accumulator936::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.894_f32 + y.sin();
        let b = y * 2.588_f32 - x.cos();
        let mut acc = Accumulator936::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.563_f32 + y.sin();
        let b = y * 7.74_f32 - x.cos();
        let mut acc = Accumulator936::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.503_f32 + y.sin();
        let b = y * 4.502_f32 - x.cos();
        let mut acc = Accumulator936::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.341_f32 + y.sin();
        let b = y * 2.463_f32 - x.cos();
        let mut acc = Accumulator936::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.292_f32 + y.sin();
        let b = y * 1.973_f32 - x.cos();
        let mut acc = Accumulator936::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.185_f32 + y.sin();
        let b = y * 6.318_f32 - x.cos();
        let mut acc = Accumulator936::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.772_f32 + y.sin();
        let b = y * 1.939_f32 - x.cos();
        let mut acc = Accumulator936::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_936(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(936u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_936() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_936(total as u64) % 997) as f32;
        total
    }
}

pub mod m937 {
    use super::*;

    pub struct Accumulator937<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator937<T> {
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
        let b = y * 0.741_f32 - x.cos();
        let mut acc = Accumulator937::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.783_f32 + y.sin();
        let b = y * 3.156_f32 - x.cos();
        let mut acc = Accumulator937::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.472_f32 + y.sin();
        let b = y * 4.709_f32 - x.cos();
        let mut acc = Accumulator937::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.564_f32 + y.sin();
        let b = y * 6.721_f32 - x.cos();
        let mut acc = Accumulator937::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.601_f32 + y.sin();
        let b = y * 1.044_f32 - x.cos();
        let mut acc = Accumulator937::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.896_f32 + y.sin();
        let b = y * 0.173_f32 - x.cos();
        let mut acc = Accumulator937::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.816_f32 + y.sin();
        let b = y * 6.168_f32 - x.cos();
        let mut acc = Accumulator937::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.061_f32 + y.sin();
        let b = y * 1.023_f32 - x.cos();
        let mut acc = Accumulator937::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.294_f32 + y.sin();
        let b = y * 7.595_f32 - x.cos();
        let mut acc = Accumulator937::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.604_f32 + y.sin();
        let b = y * 0.776_f32 - x.cos();
        let mut acc = Accumulator937::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.799_f32 + y.sin();
        let b = y * 7.653_f32 - x.cos();
        let mut acc = Accumulator937::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.639_f32 + y.sin();
        let b = y * 7.298_f32 - x.cos();
        let mut acc = Accumulator937::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.588_f32 + y.sin();
        let b = y * 9.209_f32 - x.cos();
        let mut acc = Accumulator937::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.352_f32 + y.sin();
        let b = y * 2.503_f32 - x.cos();
        let mut acc = Accumulator937::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.29_f32 + y.sin();
        let b = y * 3.796_f32 - x.cos();
        let mut acc = Accumulator937::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.963_f32 + y.sin();
        let b = y * 4.164_f32 - x.cos();
        let mut acc = Accumulator937::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.867_f32 + y.sin();
        let b = y * 2.071_f32 - x.cos();
        let mut acc = Accumulator937::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.287_f32 + y.sin();
        let b = y * 8.582_f32 - x.cos();
        let mut acc = Accumulator937::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.218_f32 + y.sin();
        let b = y * 2.92_f32 - x.cos();
        let mut acc = Accumulator937::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.879_f32 + y.sin();
        let b = y * 3.191_f32 - x.cos();
        let mut acc = Accumulator937::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_937(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_937() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_937(total as u64) % 997) as f32;
        total
    }
}

pub mod m938 {
    use super::*;

    pub struct Accumulator938<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator938<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.597_f32 + y.sin();
        let b = y * 8.804_f32 - x.cos();
        let mut acc = Accumulator938::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.394_f32 + y.sin();
        let b = y * 7.846_f32 - x.cos();
        let mut acc = Accumulator938::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.112_f32 + y.sin();
        let b = y * 0.623_f32 - x.cos();
        let mut acc = Accumulator938::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.856_f32 + y.sin();
        let b = y * 7.822_f32 - x.cos();
        let mut acc = Accumulator938::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.274_f32 + y.sin();
        let b = y * 0.387_f32 - x.cos();
        let mut acc = Accumulator938::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.347_f32 + y.sin();
        let b = y * 8.474_f32 - x.cos();
        let mut acc = Accumulator938::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.818_f32 + y.sin();
        let b = y * 8.317_f32 - x.cos();
        let mut acc = Accumulator938::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.216_f32 + y.sin();
        let b = y * 1.703_f32 - x.cos();
        let mut acc = Accumulator938::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.584_f32 + y.sin();
        let b = y * 0.556_f32 - x.cos();
        let mut acc = Accumulator938::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.576_f32 + y.sin();
        let b = y * 7.183_f32 - x.cos();
        let mut acc = Accumulator938::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.051_f32 + y.sin();
        let b = y * 2.806_f32 - x.cos();
        let mut acc = Accumulator938::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.108_f32 + y.sin();
        let b = y * 6.585_f32 - x.cos();
        let mut acc = Accumulator938::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.345_f32 + y.sin();
        let b = y * 7.982_f32 - x.cos();
        let mut acc = Accumulator938::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.512_f32 + y.sin();
        let b = y * 6.628_f32 - x.cos();
        let mut acc = Accumulator938::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.346_f32 + y.sin();
        let b = y * 6.389_f32 - x.cos();
        let mut acc = Accumulator938::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.386_f32 + y.sin();
        let b = y * 4.878_f32 - x.cos();
        let mut acc = Accumulator938::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.088_f32 + y.sin();
        let b = y * 1.232_f32 - x.cos();
        let mut acc = Accumulator938::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.237_f32 + y.sin();
        let b = y * 7.836_f32 - x.cos();
        let mut acc = Accumulator938::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.822_f32 + y.sin();
        let b = y * 0.329_f32 - x.cos();
        let mut acc = Accumulator938::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.073_f32 + y.sin();
        let b = y * 3.421_f32 - x.cos();
        let mut acc = Accumulator938::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_938(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_938() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_938(total as u64) % 997) as f32;
        total
    }
}

pub mod m939 {
    use super::*;

    pub struct Accumulator939<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator939<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.487_f32 + y.sin();
        let b = y * 3.652_f32 - x.cos();
        let mut acc = Accumulator939::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.462_f32 + y.sin();
        let b = y * 9.059_f32 - x.cos();
        let mut acc = Accumulator939::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.177_f32 + y.sin();
        let b = y * 8.932_f32 - x.cos();
        let mut acc = Accumulator939::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.674_f32 + y.sin();
        let b = y * 6.906_f32 - x.cos();
        let mut acc = Accumulator939::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.226_f32 + y.sin();
        let b = y * 1.492_f32 - x.cos();
        let mut acc = Accumulator939::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.643_f32 + y.sin();
        let b = y * 8.99_f32 - x.cos();
        let mut acc = Accumulator939::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.428_f32 + y.sin();
        let b = y * 3.065_f32 - x.cos();
        let mut acc = Accumulator939::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.392_f32 + y.sin();
        let b = y * 6.778_f32 - x.cos();
        let mut acc = Accumulator939::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.135_f32 + y.sin();
        let b = y * 9.521_f32 - x.cos();
        let mut acc = Accumulator939::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.116_f32 + y.sin();
        let b = y * 7.876_f32 - x.cos();
        let mut acc = Accumulator939::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.003_f32 + y.sin();
        let b = y * 6.273_f32 - x.cos();
        let mut acc = Accumulator939::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.034_f32 + y.sin();
        let b = y * 9.146_f32 - x.cos();
        let mut acc = Accumulator939::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.851_f32 + y.sin();
        let b = y * 2.955_f32 - x.cos();
        let mut acc = Accumulator939::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.329_f32 + y.sin();
        let b = y * 1.981_f32 - x.cos();
        let mut acc = Accumulator939::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.035_f32 + y.sin();
        let b = y * 2.061_f32 - x.cos();
        let mut acc = Accumulator939::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.351_f32 + y.sin();
        let b = y * 4.344_f32 - x.cos();
        let mut acc = Accumulator939::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.398_f32 + y.sin();
        let b = y * 5.898_f32 - x.cos();
        let mut acc = Accumulator939::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.97_f32 + y.sin();
        let b = y * 6.799_f32 - x.cos();
        let mut acc = Accumulator939::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.361_f32 + y.sin();
        let b = y * 2.173_f32 - x.cos();
        let mut acc = Accumulator939::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.668_f32 + y.sin();
        let b = y * 1.777_f32 - x.cos();
        let mut acc = Accumulator939::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_939(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m939-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_939() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_939(total as u64) % 997) as f32;
        total
    }
}

pub mod m940 {
    use super::*;

    pub struct Accumulator940<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator940<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.8_f32 + y.sin();
        let b = y * 9.727_f32 - x.cos();
        let mut acc = Accumulator940::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.693_f32 + y.sin();
        let b = y * 4.311_f32 - x.cos();
        let mut acc = Accumulator940::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.413_f32 + y.sin();
        let b = y * 9.517_f32 - x.cos();
        let mut acc = Accumulator940::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.185_f32 + y.sin();
        let b = y * 1.837_f32 - x.cos();
        let mut acc = Accumulator940::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.711_f32 + y.sin();
        let b = y * 2.218_f32 - x.cos();
        let mut acc = Accumulator940::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.193_f32 + y.sin();
        let b = y * 3.364_f32 - x.cos();
        let mut acc = Accumulator940::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.732_f32 + y.sin();
        let b = y * 3.677_f32 - x.cos();
        let mut acc = Accumulator940::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.47_f32 + y.sin();
        let b = y * 1.184_f32 - x.cos();
        let mut acc = Accumulator940::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.019_f32 + y.sin();
        let b = y * 6.052_f32 - x.cos();
        let mut acc = Accumulator940::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.647_f32 + y.sin();
        let b = y * 9.73_f32 - x.cos();
        let mut acc = Accumulator940::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.864_f32 + y.sin();
        let b = y * 1.211_f32 - x.cos();
        let mut acc = Accumulator940::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.104_f32 + y.sin();
        let b = y * 0.709_f32 - x.cos();
        let mut acc = Accumulator940::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.652_f32 + y.sin();
        let b = y * 5.916_f32 - x.cos();
        let mut acc = Accumulator940::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.823_f32 + y.sin();
        let b = y * 2.104_f32 - x.cos();
        let mut acc = Accumulator940::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.673_f32 + y.sin();
        let b = y * 9.191_f32 - x.cos();
        let mut acc = Accumulator940::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.755_f32 + y.sin();
        let b = y * 1.13_f32 - x.cos();
        let mut acc = Accumulator940::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.117_f32 + y.sin();
        let b = y * 6.848_f32 - x.cos();
        let mut acc = Accumulator940::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.563_f32 + y.sin();
        let b = y * 1.874_f32 - x.cos();
        let mut acc = Accumulator940::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.606_f32 + y.sin();
        let b = y * 8.427_f32 - x.cos();
        let mut acc = Accumulator940::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.023_f32 + y.sin();
        let b = y * 0.559_f32 - x.cos();
        let mut acc = Accumulator940::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_940(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_940() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_940(total as u64) % 997) as f32;
        total
    }
}

pub mod m941 {
    use super::*;

    pub struct Accumulator941<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator941<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.139_f32 + y.sin();
        let b = y * 8.299_f32 - x.cos();
        let mut acc = Accumulator941::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.604_f32 + y.sin();
        let b = y * 2.93_f32 - x.cos();
        let mut acc = Accumulator941::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.371_f32 + y.sin();
        let b = y * 6.209_f32 - x.cos();
        let mut acc = Accumulator941::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.663_f32 + y.sin();
        let b = y * 0.678_f32 - x.cos();
        let mut acc = Accumulator941::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.85_f32 + y.sin();
        let b = y * 5.662_f32 - x.cos();
        let mut acc = Accumulator941::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.126_f32 + y.sin();
        let b = y * 5.042_f32 - x.cos();
        let mut acc = Accumulator941::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.206_f32 + y.sin();
        let b = y * 9.658_f32 - x.cos();
        let mut acc = Accumulator941::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.308_f32 + y.sin();
        let b = y * 6.265_f32 - x.cos();
        let mut acc = Accumulator941::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.979_f32 + y.sin();
        let b = y * 7.084_f32 - x.cos();
        let mut acc = Accumulator941::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.309_f32 + y.sin();
        let b = y * 2.028_f32 - x.cos();
        let mut acc = Accumulator941::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.322_f32 + y.sin();
        let b = y * 5.769_f32 - x.cos();
        let mut acc = Accumulator941::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.244_f32 + y.sin();
        let b = y * 2.451_f32 - x.cos();
        let mut acc = Accumulator941::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.928_f32 + y.sin();
        let b = y * 1.102_f32 - x.cos();
        let mut acc = Accumulator941::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.624_f32 + y.sin();
        let b = y * 2.073_f32 - x.cos();
        let mut acc = Accumulator941::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.375_f32 + y.sin();
        let b = y * 2.04_f32 - x.cos();
        let mut acc = Accumulator941::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.993_f32 + y.sin();
        let b = y * 6.229_f32 - x.cos();
        let mut acc = Accumulator941::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.495_f32 + y.sin();
        let b = y * 0.767_f32 - x.cos();
        let mut acc = Accumulator941::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.997_f32 + y.sin();
        let b = y * 7.409_f32 - x.cos();
        let mut acc = Accumulator941::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.649_f32 + y.sin();
        let b = y * 3.497_f32 - x.cos();
        let mut acc = Accumulator941::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.433_f32 + y.sin();
        let b = y * 5.386_f32 - x.cos();
        let mut acc = Accumulator941::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_941(seed: u64) -> u64 {
        let re = Regex::new(r"m941-(\d+)").unwrap();
        let hay = format!("m941-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_941() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_941(total as u64) % 997) as f32;
        total
    }
}

pub mod m942 {
    use super::*;

    pub struct Accumulator942<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator942<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.069_f32 + y.sin();
        let b = y * 8.659_f32 - x.cos();
        let mut acc = Accumulator942::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.799_f32 + y.sin();
        let b = y * 3.507_f32 - x.cos();
        let mut acc = Accumulator942::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.862_f32 + y.sin();
        let b = y * 0.103_f32 - x.cos();
        let mut acc = Accumulator942::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.011_f32 + y.sin();
        let b = y * 0.941_f32 - x.cos();
        let mut acc = Accumulator942::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.967_f32 + y.sin();
        let b = y * 0.408_f32 - x.cos();
        let mut acc = Accumulator942::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.477_f32 + y.sin();
        let b = y * 1.232_f32 - x.cos();
        let mut acc = Accumulator942::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.065_f32 + y.sin();
        let b = y * 2.661_f32 - x.cos();
        let mut acc = Accumulator942::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.251_f32 + y.sin();
        let b = y * 6.03_f32 - x.cos();
        let mut acc = Accumulator942::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.176_f32 + y.sin();
        let b = y * 8.279_f32 - x.cos();
        let mut acc = Accumulator942::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.99_f32 + y.sin();
        let b = y * 3.274_f32 - x.cos();
        let mut acc = Accumulator942::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.004_f32 + y.sin();
        let b = y * 3.904_f32 - x.cos();
        let mut acc = Accumulator942::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.069_f32 + y.sin();
        let b = y * 7.057_f32 - x.cos();
        let mut acc = Accumulator942::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.174_f32 + y.sin();
        let b = y * 6.358_f32 - x.cos();
        let mut acc = Accumulator942::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.102_f32 + y.sin();
        let b = y * 3.723_f32 - x.cos();
        let mut acc = Accumulator942::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.603_f32 + y.sin();
        let b = y * 1.861_f32 - x.cos();
        let mut acc = Accumulator942::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.119_f32 + y.sin();
        let b = y * 3.508_f32 - x.cos();
        let mut acc = Accumulator942::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.487_f32 + y.sin();
        let b = y * 4.861_f32 - x.cos();
        let mut acc = Accumulator942::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.505_f32 + y.sin();
        let b = y * 1.625_f32 - x.cos();
        let mut acc = Accumulator942::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.804_f32 + y.sin();
        let b = y * 4.773_f32 - x.cos();
        let mut acc = Accumulator942::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.042_f32 + y.sin();
        let b = y * 5.941_f32 - x.cos();
        let mut acc = Accumulator942::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_942(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_942() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_942(total as u64) % 997) as f32;
        total
    }
}

pub mod m943 {
    use super::*;

    pub struct Accumulator943<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator943<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.805_f32 + y.sin();
        let b = y * 8.854_f32 - x.cos();
        let mut acc = Accumulator943::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.487_f32 + y.sin();
        let b = y * 3.717_f32 - x.cos();
        let mut acc = Accumulator943::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.691_f32 + y.sin();
        let b = y * 7.42_f32 - x.cos();
        let mut acc = Accumulator943::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.348_f32 + y.sin();
        let b = y * 2.323_f32 - x.cos();
        let mut acc = Accumulator943::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.645_f32 + y.sin();
        let b = y * 7.649_f32 - x.cos();
        let mut acc = Accumulator943::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.723_f32 + y.sin();
        let b = y * 5.198_f32 - x.cos();
        let mut acc = Accumulator943::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.202_f32 + y.sin();
        let b = y * 8.588_f32 - x.cos();
        let mut acc = Accumulator943::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.569_f32 + y.sin();
        let b = y * 7.77_f32 - x.cos();
        let mut acc = Accumulator943::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.46_f32 + y.sin();
        let b = y * 5.962_f32 - x.cos();
        let mut acc = Accumulator943::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.26_f32 + y.sin();
        let b = y * 4.035_f32 - x.cos();
        let mut acc = Accumulator943::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.872_f32 + y.sin();
        let b = y * 5.429_f32 - x.cos();
        let mut acc = Accumulator943::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.317_f32 + y.sin();
        let b = y * 9.731_f32 - x.cos();
        let mut acc = Accumulator943::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.594_f32 + y.sin();
        let b = y * 0.397_f32 - x.cos();
        let mut acc = Accumulator943::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.226_f32 + y.sin();
        let b = y * 5.496_f32 - x.cos();
        let mut acc = Accumulator943::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.69_f32 + y.sin();
        let b = y * 3.703_f32 - x.cos();
        let mut acc = Accumulator943::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.555_f32 + y.sin();
        let b = y * 9.11_f32 - x.cos();
        let mut acc = Accumulator943::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.449_f32 + y.sin();
        let b = y * 0.975_f32 - x.cos();
        let mut acc = Accumulator943::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.07_f32 + y.sin();
        let b = y * 8.913_f32 - x.cos();
        let mut acc = Accumulator943::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.572_f32 + y.sin();
        let b = y * 2.024_f32 - x.cos();
        let mut acc = Accumulator943::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.712_f32 + y.sin();
        let b = y * 5.992_f32 - x.cos();
        let mut acc = Accumulator943::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_943(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(943u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_943() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_943(total as u64) % 997) as f32;
        total
    }
}

pub mod m944 {
    use super::*;

    pub struct Accumulator944<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator944<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.169_f32 + y.sin();
        let b = y * 9.251_f32 - x.cos();
        let mut acc = Accumulator944::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.722_f32 + y.sin();
        let b = y * 6.493_f32 - x.cos();
        let mut acc = Accumulator944::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.465_f32 + y.sin();
        let b = y * 5.104_f32 - x.cos();
        let mut acc = Accumulator944::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.123_f32 + y.sin();
        let b = y * 0.701_f32 - x.cos();
        let mut acc = Accumulator944::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.69_f32 + y.sin();
        let b = y * 3.968_f32 - x.cos();
        let mut acc = Accumulator944::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.229_f32 + y.sin();
        let b = y * 3.534_f32 - x.cos();
        let mut acc = Accumulator944::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.961_f32 + y.sin();
        let b = y * 3.479_f32 - x.cos();
        let mut acc = Accumulator944::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.405_f32 + y.sin();
        let b = y * 7.5_f32 - x.cos();
        let mut acc = Accumulator944::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.395_f32 + y.sin();
        let b = y * 7.53_f32 - x.cos();
        let mut acc = Accumulator944::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.12_f32 + y.sin();
        let b = y * 8.882_f32 - x.cos();
        let mut acc = Accumulator944::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.021_f32 + y.sin();
        let b = y * 3.522_f32 - x.cos();
        let mut acc = Accumulator944::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.482_f32 + y.sin();
        let b = y * 7.537_f32 - x.cos();
        let mut acc = Accumulator944::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.183_f32 + y.sin();
        let b = y * 4.638_f32 - x.cos();
        let mut acc = Accumulator944::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.983_f32 + y.sin();
        let b = y * 8.68_f32 - x.cos();
        let mut acc = Accumulator944::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.165_f32 + y.sin();
        let b = y * 3.11_f32 - x.cos();
        let mut acc = Accumulator944::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.989_f32 + y.sin();
        let b = y * 8.118_f32 - x.cos();
        let mut acc = Accumulator944::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.372_f32 + y.sin();
        let b = y * 0.802_f32 - x.cos();
        let mut acc = Accumulator944::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.809_f32 + y.sin();
        let b = y * 1.243_f32 - x.cos();
        let mut acc = Accumulator944::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.241_f32 + y.sin();
        let b = y * 7.272_f32 - x.cos();
        let mut acc = Accumulator944::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.345_f32 + y.sin();
        let b = y * 0.891_f32 - x.cos();
        let mut acc = Accumulator944::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_944(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_944() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_944(total as u64) % 997) as f32;
        total
    }
}

pub mod m945 {
    use super::*;

    pub struct Accumulator945<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator945<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.43_f32 + y.sin();
        let b = y * 4.358_f32 - x.cos();
        let mut acc = Accumulator945::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.622_f32 + y.sin();
        let b = y * 3.648_f32 - x.cos();
        let mut acc = Accumulator945::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.377_f32 + y.sin();
        let b = y * 8.261_f32 - x.cos();
        let mut acc = Accumulator945::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.07_f32 + y.sin();
        let b = y * 1.351_f32 - x.cos();
        let mut acc = Accumulator945::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.93_f32 + y.sin();
        let b = y * 6.122_f32 - x.cos();
        let mut acc = Accumulator945::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.369_f32 + y.sin();
        let b = y * 6.092_f32 - x.cos();
        let mut acc = Accumulator945::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.846_f32 + y.sin();
        let b = y * 2.001_f32 - x.cos();
        let mut acc = Accumulator945::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.642_f32 + y.sin();
        let b = y * 2.591_f32 - x.cos();
        let mut acc = Accumulator945::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.999_f32 + y.sin();
        let b = y * 2.474_f32 - x.cos();
        let mut acc = Accumulator945::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.678_f32 + y.sin();
        let b = y * 1.087_f32 - x.cos();
        let mut acc = Accumulator945::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.256_f32 + y.sin();
        let b = y * 5.097_f32 - x.cos();
        let mut acc = Accumulator945::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.689_f32 + y.sin();
        let b = y * 9.448_f32 - x.cos();
        let mut acc = Accumulator945::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.383_f32 + y.sin();
        let b = y * 9.658_f32 - x.cos();
        let mut acc = Accumulator945::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.81_f32 + y.sin();
        let b = y * 0.63_f32 - x.cos();
        let mut acc = Accumulator945::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.559_f32 + y.sin();
        let b = y * 6.165_f32 - x.cos();
        let mut acc = Accumulator945::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.552_f32 + y.sin();
        let b = y * 5.022_f32 - x.cos();
        let mut acc = Accumulator945::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.781_f32 + y.sin();
        let b = y * 9.061_f32 - x.cos();
        let mut acc = Accumulator945::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.841_f32 + y.sin();
        let b = y * 7.41_f32 - x.cos();
        let mut acc = Accumulator945::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.941_f32 + y.sin();
        let b = y * 4.997_f32 - x.cos();
        let mut acc = Accumulator945::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.014_f32 + y.sin();
        let b = y * 6.967_f32 - x.cos();
        let mut acc = Accumulator945::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_945(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_945() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_945(total as u64) % 997) as f32;
        total
    }
}

pub mod m946 {
    use super::*;

    pub struct Accumulator946<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator946<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.904_f32 + y.sin();
        let b = y * 8.963_f32 - x.cos();
        let mut acc = Accumulator946::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.64_f32 + y.sin();
        let b = y * 1.871_f32 - x.cos();
        let mut acc = Accumulator946::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.615_f32 + y.sin();
        let b = y * 3.395_f32 - x.cos();
        let mut acc = Accumulator946::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.375_f32 + y.sin();
        let b = y * 7.552_f32 - x.cos();
        let mut acc = Accumulator946::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.228_f32 + y.sin();
        let b = y * 3.654_f32 - x.cos();
        let mut acc = Accumulator946::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.788_f32 + y.sin();
        let b = y * 3.211_f32 - x.cos();
        let mut acc = Accumulator946::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.96_f32 + y.sin();
        let b = y * 9.392_f32 - x.cos();
        let mut acc = Accumulator946::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.731_f32 + y.sin();
        let b = y * 7.769_f32 - x.cos();
        let mut acc = Accumulator946::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.448_f32 + y.sin();
        let b = y * 4.043_f32 - x.cos();
        let mut acc = Accumulator946::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.05_f32 + y.sin();
        let b = y * 5.112_f32 - x.cos();
        let mut acc = Accumulator946::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.85_f32 + y.sin();
        let b = y * 8.19_f32 - x.cos();
        let mut acc = Accumulator946::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.92_f32 + y.sin();
        let b = y * 1.825_f32 - x.cos();
        let mut acc = Accumulator946::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.59_f32 + y.sin();
        let b = y * 6.522_f32 - x.cos();
        let mut acc = Accumulator946::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.66_f32 + y.sin();
        let b = y * 6.068_f32 - x.cos();
        let mut acc = Accumulator946::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.056_f32 + y.sin();
        let b = y * 4.619_f32 - x.cos();
        let mut acc = Accumulator946::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.618_f32 + y.sin();
        let b = y * 0.765_f32 - x.cos();
        let mut acc = Accumulator946::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.307_f32 + y.sin();
        let b = y * 5.197_f32 - x.cos();
        let mut acc = Accumulator946::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.584_f32 + y.sin();
        let b = y * 8.475_f32 - x.cos();
        let mut acc = Accumulator946::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.988_f32 + y.sin();
        let b = y * 5.326_f32 - x.cos();
        let mut acc = Accumulator946::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.667_f32 + y.sin();
        let b = y * 1.675_f32 - x.cos();
        let mut acc = Accumulator946::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_946(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m946-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_946() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_946(total as u64) % 997) as f32;
        total
    }
}

pub mod m947 {
    use super::*;

    pub struct Accumulator947<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator947<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.004_f32 + y.sin();
        let b = y * 9.652_f32 - x.cos();
        let mut acc = Accumulator947::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.816_f32 + y.sin();
        let b = y * 7.106_f32 - x.cos();
        let mut acc = Accumulator947::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.504_f32 + y.sin();
        let b = y * 7.606_f32 - x.cos();
        let mut acc = Accumulator947::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.29_f32 + y.sin();
        let b = y * 3.641_f32 - x.cos();
        let mut acc = Accumulator947::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.006_f32 + y.sin();
        let b = y * 5.588_f32 - x.cos();
        let mut acc = Accumulator947::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.043_f32 + y.sin();
        let b = y * 7.985_f32 - x.cos();
        let mut acc = Accumulator947::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.317_f32 + y.sin();
        let b = y * 7.763_f32 - x.cos();
        let mut acc = Accumulator947::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.751_f32 + y.sin();
        let b = y * 7.184_f32 - x.cos();
        let mut acc = Accumulator947::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.864_f32 + y.sin();
        let b = y * 3.508_f32 - x.cos();
        let mut acc = Accumulator947::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.932_f32 + y.sin();
        let b = y * 3.228_f32 - x.cos();
        let mut acc = Accumulator947::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.653_f32 + y.sin();
        let b = y * 7.797_f32 - x.cos();
        let mut acc = Accumulator947::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.167_f32 + y.sin();
        let b = y * 6.411_f32 - x.cos();
        let mut acc = Accumulator947::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.286_f32 + y.sin();
        let b = y * 8.917_f32 - x.cos();
        let mut acc = Accumulator947::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.542_f32 + y.sin();
        let b = y * 9.066_f32 - x.cos();
        let mut acc = Accumulator947::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.046_f32 + y.sin();
        let b = y * 6.862_f32 - x.cos();
        let mut acc = Accumulator947::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.312_f32 + y.sin();
        let b = y * 6.134_f32 - x.cos();
        let mut acc = Accumulator947::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.89_f32 + y.sin();
        let b = y * 1.246_f32 - x.cos();
        let mut acc = Accumulator947::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.958_f32 + y.sin();
        let b = y * 6.079_f32 - x.cos();
        let mut acc = Accumulator947::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.376_f32 + y.sin();
        let b = y * 7.903_f32 - x.cos();
        let mut acc = Accumulator947::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.972_f32 + y.sin();
        let b = y * 0.752_f32 - x.cos();
        let mut acc = Accumulator947::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_947(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_947() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_947(total as u64) % 997) as f32;
        total
    }
}

pub mod m948 {
    use super::*;

    pub struct Accumulator948<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator948<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.659_f32 + y.sin();
        let b = y * 3.598_f32 - x.cos();
        let mut acc = Accumulator948::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.708_f32 + y.sin();
        let b = y * 1.278_f32 - x.cos();
        let mut acc = Accumulator948::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.495_f32 + y.sin();
        let b = y * 4.879_f32 - x.cos();
        let mut acc = Accumulator948::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.951_f32 + y.sin();
        let b = y * 9.248_f32 - x.cos();
        let mut acc = Accumulator948::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.514_f32 + y.sin();
        let b = y * 2.475_f32 - x.cos();
        let mut acc = Accumulator948::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.169_f32 + y.sin();
        let b = y * 5.884_f32 - x.cos();
        let mut acc = Accumulator948::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.927_f32 + y.sin();
        let b = y * 8.202_f32 - x.cos();
        let mut acc = Accumulator948::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.477_f32 + y.sin();
        let b = y * 0.804_f32 - x.cos();
        let mut acc = Accumulator948::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.381_f32 + y.sin();
        let b = y * 3.939_f32 - x.cos();
        let mut acc = Accumulator948::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.718_f32 + y.sin();
        let b = y * 8.092_f32 - x.cos();
        let mut acc = Accumulator948::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.495_f32 + y.sin();
        let b = y * 0.31_f32 - x.cos();
        let mut acc = Accumulator948::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.01_f32 + y.sin();
        let b = y * 3.522_f32 - x.cos();
        let mut acc = Accumulator948::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.415_f32 + y.sin();
        let b = y * 3.672_f32 - x.cos();
        let mut acc = Accumulator948::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.65_f32 + y.sin();
        let b = y * 6.877_f32 - x.cos();
        let mut acc = Accumulator948::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.875_f32 + y.sin();
        let b = y * 2.747_f32 - x.cos();
        let mut acc = Accumulator948::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.269_f32 + y.sin();
        let b = y * 7.844_f32 - x.cos();
        let mut acc = Accumulator948::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.151_f32 + y.sin();
        let b = y * 3.119_f32 - x.cos();
        let mut acc = Accumulator948::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.944_f32 + y.sin();
        let b = y * 0.734_f32 - x.cos();
        let mut acc = Accumulator948::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.825_f32 + y.sin();
        let b = y * 8.466_f32 - x.cos();
        let mut acc = Accumulator948::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.58_f32 + y.sin();
        let b = y * 9.523_f32 - x.cos();
        let mut acc = Accumulator948::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_948(seed: u64) -> u64 {
        let re = Regex::new(r"m948-(\d+)").unwrap();
        let hay = format!("m948-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_948() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_948(total as u64) % 997) as f32;
        total
    }
}

pub mod m949 {
    use super::*;

    pub struct Accumulator949<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator949<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.036_f32 + y.sin();
        let b = y * 8.74_f32 - x.cos();
        let mut acc = Accumulator949::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.386_f32 + y.sin();
        let b = y * 5.358_f32 - x.cos();
        let mut acc = Accumulator949::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.865_f32 + y.sin();
        let b = y * 2.746_f32 - x.cos();
        let mut acc = Accumulator949::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.031_f32 + y.sin();
        let b = y * 2.735_f32 - x.cos();
        let mut acc = Accumulator949::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.623_f32 + y.sin();
        let b = y * 5.059_f32 - x.cos();
        let mut acc = Accumulator949::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.291_f32 + y.sin();
        let b = y * 2.891_f32 - x.cos();
        let mut acc = Accumulator949::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.183_f32 + y.sin();
        let b = y * 4.974_f32 - x.cos();
        let mut acc = Accumulator949::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.563_f32 + y.sin();
        let b = y * 8.535_f32 - x.cos();
        let mut acc = Accumulator949::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.28_f32 + y.sin();
        let b = y * 7.723_f32 - x.cos();
        let mut acc = Accumulator949::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.074_f32 + y.sin();
        let b = y * 2.747_f32 - x.cos();
        let mut acc = Accumulator949::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.373_f32 + y.sin();
        let b = y * 9.261_f32 - x.cos();
        let mut acc = Accumulator949::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.81_f32 + y.sin();
        let b = y * 1.524_f32 - x.cos();
        let mut acc = Accumulator949::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.646_f32 + y.sin();
        let b = y * 3.639_f32 - x.cos();
        let mut acc = Accumulator949::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.256_f32 + y.sin();
        let b = y * 4.907_f32 - x.cos();
        let mut acc = Accumulator949::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.554_f32 + y.sin();
        let b = y * 1.929_f32 - x.cos();
        let mut acc = Accumulator949::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.043_f32 + y.sin();
        let b = y * 8.986_f32 - x.cos();
        let mut acc = Accumulator949::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.835_f32 + y.sin();
        let b = y * 8.801_f32 - x.cos();
        let mut acc = Accumulator949::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.366_f32 + y.sin();
        let b = y * 9.224_f32 - x.cos();
        let mut acc = Accumulator949::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.865_f32 + y.sin();
        let b = y * 1.138_f32 - x.cos();
        let mut acc = Accumulator949::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.883_f32 + y.sin();
        let b = y * 8.325_f32 - x.cos();
        let mut acc = Accumulator949::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_949(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_949() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_949(total as u64) % 997) as f32;
        total
    }
}

pub mod m950 {
    use super::*;

    pub struct Accumulator950<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator950<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.172_f32 + y.sin();
        let b = y * 4.95_f32 - x.cos();
        let mut acc = Accumulator950::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.257_f32 + y.sin();
        let b = y * 9.782_f32 - x.cos();
        let mut acc = Accumulator950::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.919_f32 + y.sin();
        let b = y * 6.123_f32 - x.cos();
        let mut acc = Accumulator950::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.107_f32 + y.sin();
        let b = y * 7.784_f32 - x.cos();
        let mut acc = Accumulator950::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.634_f32 + y.sin();
        let b = y * 5.214_f32 - x.cos();
        let mut acc = Accumulator950::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.377_f32 + y.sin();
        let b = y * 9.677_f32 - x.cos();
        let mut acc = Accumulator950::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.778_f32 + y.sin();
        let b = y * 6.464_f32 - x.cos();
        let mut acc = Accumulator950::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.654_f32 + y.sin();
        let b = y * 1.562_f32 - x.cos();
        let mut acc = Accumulator950::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.496_f32 + y.sin();
        let b = y * 6.872_f32 - x.cos();
        let mut acc = Accumulator950::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.388_f32 + y.sin();
        let b = y * 3.795_f32 - x.cos();
        let mut acc = Accumulator950::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.453_f32 + y.sin();
        let b = y * 7.101_f32 - x.cos();
        let mut acc = Accumulator950::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.124_f32 + y.sin();
        let b = y * 2.327_f32 - x.cos();
        let mut acc = Accumulator950::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.044_f32 + y.sin();
        let b = y * 9.021_f32 - x.cos();
        let mut acc = Accumulator950::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.222_f32 + y.sin();
        let b = y * 5.966_f32 - x.cos();
        let mut acc = Accumulator950::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.3_f32 + y.sin();
        let b = y * 1.339_f32 - x.cos();
        let mut acc = Accumulator950::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.765_f32 + y.sin();
        let b = y * 2.105_f32 - x.cos();
        let mut acc = Accumulator950::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.688_f32 + y.sin();
        let b = y * 7.01_f32 - x.cos();
        let mut acc = Accumulator950::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.538_f32 + y.sin();
        let b = y * 1.231_f32 - x.cos();
        let mut acc = Accumulator950::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.841_f32 + y.sin();
        let b = y * 2.524_f32 - x.cos();
        let mut acc = Accumulator950::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.113_f32 + y.sin();
        let b = y * 4.125_f32 - x.cos();
        let mut acc = Accumulator950::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_950(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(950u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_950() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_950(total as u64) % 997) as f32;
        total
    }
}

pub mod m951 {
    use super::*;

    pub struct Accumulator951<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator951<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.425_f32 + y.sin();
        let b = y * 5.557_f32 - x.cos();
        let mut acc = Accumulator951::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.039_f32 + y.sin();
        let b = y * 8.101_f32 - x.cos();
        let mut acc = Accumulator951::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.428_f32 + y.sin();
        let b = y * 6.654_f32 - x.cos();
        let mut acc = Accumulator951::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.126_f32 + y.sin();
        let b = y * 7.069_f32 - x.cos();
        let mut acc = Accumulator951::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.735_f32 + y.sin();
        let b = y * 5.623_f32 - x.cos();
        let mut acc = Accumulator951::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.287_f32 + y.sin();
        let b = y * 4.61_f32 - x.cos();
        let mut acc = Accumulator951::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.664_f32 + y.sin();
        let b = y * 8.224_f32 - x.cos();
        let mut acc = Accumulator951::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.294_f32 + y.sin();
        let b = y * 8.599_f32 - x.cos();
        let mut acc = Accumulator951::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.858_f32 + y.sin();
        let b = y * 7.097_f32 - x.cos();
        let mut acc = Accumulator951::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.636_f32 + y.sin();
        let b = y * 3.164_f32 - x.cos();
        let mut acc = Accumulator951::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.642_f32 + y.sin();
        let b = y * 7.097_f32 - x.cos();
        let mut acc = Accumulator951::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.402_f32 + y.sin();
        let b = y * 7.452_f32 - x.cos();
        let mut acc = Accumulator951::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.402_f32 + y.sin();
        let b = y * 5.086_f32 - x.cos();
        let mut acc = Accumulator951::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.792_f32 + y.sin();
        let b = y * 7.114_f32 - x.cos();
        let mut acc = Accumulator951::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.409_f32 + y.sin();
        let b = y * 8.665_f32 - x.cos();
        let mut acc = Accumulator951::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.209_f32 + y.sin();
        let b = y * 0.969_f32 - x.cos();
        let mut acc = Accumulator951::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.035_f32 + y.sin();
        let b = y * 3.787_f32 - x.cos();
        let mut acc = Accumulator951::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.915_f32 + y.sin();
        let b = y * 5.673_f32 - x.cos();
        let mut acc = Accumulator951::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.894_f32 + y.sin();
        let b = y * 5.291_f32 - x.cos();
        let mut acc = Accumulator951::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.574_f32 + y.sin();
        let b = y * 1.368_f32 - x.cos();
        let mut acc = Accumulator951::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_951(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_951() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_951(total as u64) % 997) as f32;
        total
    }
}

pub mod m952 {
    use super::*;

    pub struct Accumulator952<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator952<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.465_f32 + y.sin();
        let b = y * 4.069_f32 - x.cos();
        let mut acc = Accumulator952::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.449_f32 + y.sin();
        let b = y * 9.224_f32 - x.cos();
        let mut acc = Accumulator952::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.225_f32 + y.sin();
        let b = y * 1.875_f32 - x.cos();
        let mut acc = Accumulator952::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.335_f32 + y.sin();
        let b = y * 2.398_f32 - x.cos();
        let mut acc = Accumulator952::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.522_f32 + y.sin();
        let b = y * 7.197_f32 - x.cos();
        let mut acc = Accumulator952::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.903_f32 + y.sin();
        let b = y * 4.38_f32 - x.cos();
        let mut acc = Accumulator952::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.047_f32 + y.sin();
        let b = y * 5.864_f32 - x.cos();
        let mut acc = Accumulator952::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.772_f32 + y.sin();
        let b = y * 8.872_f32 - x.cos();
        let mut acc = Accumulator952::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.497_f32 + y.sin();
        let b = y * 7.405_f32 - x.cos();
        let mut acc = Accumulator952::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.068_f32 + y.sin();
        let b = y * 4.367_f32 - x.cos();
        let mut acc = Accumulator952::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.922_f32 + y.sin();
        let b = y * 6.593_f32 - x.cos();
        let mut acc = Accumulator952::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.794_f32 + y.sin();
        let b = y * 2.742_f32 - x.cos();
        let mut acc = Accumulator952::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.826_f32 + y.sin();
        let b = y * 3.677_f32 - x.cos();
        let mut acc = Accumulator952::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.068_f32 + y.sin();
        let b = y * 8.737_f32 - x.cos();
        let mut acc = Accumulator952::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.748_f32 + y.sin();
        let b = y * 7.493_f32 - x.cos();
        let mut acc = Accumulator952::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.402_f32 + y.sin();
        let b = y * 8.196_f32 - x.cos();
        let mut acc = Accumulator952::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.329_f32 + y.sin();
        let b = y * 2.756_f32 - x.cos();
        let mut acc = Accumulator952::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.071_f32 + y.sin();
        let b = y * 4.474_f32 - x.cos();
        let mut acc = Accumulator952::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.967_f32 + y.sin();
        let b = y * 1.161_f32 - x.cos();
        let mut acc = Accumulator952::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.684_f32 + y.sin();
        let b = y * 9.361_f32 - x.cos();
        let mut acc = Accumulator952::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_952(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_952() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_952(total as u64) % 997) as f32;
        total
    }
}

pub mod m953 {
    use super::*;

    pub struct Accumulator953<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator953<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.719_f32 + y.sin();
        let b = y * 1.345_f32 - x.cos();
        let mut acc = Accumulator953::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.699_f32 + y.sin();
        let b = y * 2.52_f32 - x.cos();
        let mut acc = Accumulator953::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.756_f32 + y.sin();
        let b = y * 9.706_f32 - x.cos();
        let mut acc = Accumulator953::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.387_f32 + y.sin();
        let b = y * 6.111_f32 - x.cos();
        let mut acc = Accumulator953::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.213_f32 + y.sin();
        let b = y * 2.04_f32 - x.cos();
        let mut acc = Accumulator953::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.875_f32 + y.sin();
        let b = y * 3.659_f32 - x.cos();
        let mut acc = Accumulator953::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.768_f32 + y.sin();
        let b = y * 5.746_f32 - x.cos();
        let mut acc = Accumulator953::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.375_f32 + y.sin();
        let b = y * 3.779_f32 - x.cos();
        let mut acc = Accumulator953::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.62_f32 + y.sin();
        let b = y * 0.433_f32 - x.cos();
        let mut acc = Accumulator953::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.786_f32 + y.sin();
        let b = y * 7.708_f32 - x.cos();
        let mut acc = Accumulator953::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.738_f32 + y.sin();
        let b = y * 7.713_f32 - x.cos();
        let mut acc = Accumulator953::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.002_f32 + y.sin();
        let b = y * 5.888_f32 - x.cos();
        let mut acc = Accumulator953::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.613_f32 + y.sin();
        let b = y * 7.129_f32 - x.cos();
        let mut acc = Accumulator953::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.047_f32 + y.sin();
        let b = y * 2.12_f32 - x.cos();
        let mut acc = Accumulator953::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.888_f32 + y.sin();
        let b = y * 6.518_f32 - x.cos();
        let mut acc = Accumulator953::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.447_f32 + y.sin();
        let b = y * 7.847_f32 - x.cos();
        let mut acc = Accumulator953::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.312_f32 + y.sin();
        let b = y * 5.001_f32 - x.cos();
        let mut acc = Accumulator953::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.887_f32 + y.sin();
        let b = y * 2.1_f32 - x.cos();
        let mut acc = Accumulator953::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.528_f32 + y.sin();
        let b = y * 8.164_f32 - x.cos();
        let mut acc = Accumulator953::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.569_f32 + y.sin();
        let b = y * 3.342_f32 - x.cos();
        let mut acc = Accumulator953::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_953(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m953-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_953() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_953(total as u64) % 997) as f32;
        total
    }
}

pub mod m954 {
    use super::*;

    pub struct Accumulator954<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator954<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.03_f32 + y.sin();
        let b = y * 8.862_f32 - x.cos();
        let mut acc = Accumulator954::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.931_f32 + y.sin();
        let b = y * 5.082_f32 - x.cos();
        let mut acc = Accumulator954::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.282_f32 + y.sin();
        let b = y * 5.686_f32 - x.cos();
        let mut acc = Accumulator954::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.428_f32 + y.sin();
        let b = y * 6.814_f32 - x.cos();
        let mut acc = Accumulator954::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.928_f32 + y.sin();
        let b = y * 9.213_f32 - x.cos();
        let mut acc = Accumulator954::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.598_f32 + y.sin();
        let b = y * 9.576_f32 - x.cos();
        let mut acc = Accumulator954::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.427_f32 + y.sin();
        let b = y * 6.118_f32 - x.cos();
        let mut acc = Accumulator954::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.117_f32 + y.sin();
        let b = y * 6.462_f32 - x.cos();
        let mut acc = Accumulator954::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.161_f32 + y.sin();
        let b = y * 5.129_f32 - x.cos();
        let mut acc = Accumulator954::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.253_f32 + y.sin();
        let b = y * 0.203_f32 - x.cos();
        let mut acc = Accumulator954::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.678_f32 + y.sin();
        let b = y * 3.504_f32 - x.cos();
        let mut acc = Accumulator954::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.874_f32 + y.sin();
        let b = y * 4.052_f32 - x.cos();
        let mut acc = Accumulator954::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.836_f32 + y.sin();
        let b = y * 4.374_f32 - x.cos();
        let mut acc = Accumulator954::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.924_f32 + y.sin();
        let b = y * 1.54_f32 - x.cos();
        let mut acc = Accumulator954::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.57_f32 + y.sin();
        let b = y * 1.858_f32 - x.cos();
        let mut acc = Accumulator954::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.104_f32 + y.sin();
        let b = y * 1.098_f32 - x.cos();
        let mut acc = Accumulator954::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.864_f32 + y.sin();
        let b = y * 6.06_f32 - x.cos();
        let mut acc = Accumulator954::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.54_f32 + y.sin();
        let b = y * 6.418_f32 - x.cos();
        let mut acc = Accumulator954::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.14_f32 + y.sin();
        let b = y * 7.945_f32 - x.cos();
        let mut acc = Accumulator954::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.536_f32 + y.sin();
        let b = y * 8.99_f32 - x.cos();
        let mut acc = Accumulator954::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_954(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_954() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_954(total as u64) % 997) as f32;
        total
    }
}

pub mod m955 {
    use super::*;

    pub struct Accumulator955<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator955<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.534_f32 + y.sin();
        let b = y * 4.823_f32 - x.cos();
        let mut acc = Accumulator955::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.293_f32 + y.sin();
        let b = y * 7.627_f32 - x.cos();
        let mut acc = Accumulator955::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.247_f32 + y.sin();
        let b = y * 2.332_f32 - x.cos();
        let mut acc = Accumulator955::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.612_f32 + y.sin();
        let b = y * 1.924_f32 - x.cos();
        let mut acc = Accumulator955::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.142_f32 + y.sin();
        let b = y * 0.931_f32 - x.cos();
        let mut acc = Accumulator955::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.56_f32 + y.sin();
        let b = y * 8.546_f32 - x.cos();
        let mut acc = Accumulator955::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.221_f32 + y.sin();
        let b = y * 4.914_f32 - x.cos();
        let mut acc = Accumulator955::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.487_f32 + y.sin();
        let b = y * 7.364_f32 - x.cos();
        let mut acc = Accumulator955::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.887_f32 + y.sin();
        let b = y * 8.423_f32 - x.cos();
        let mut acc = Accumulator955::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.143_f32 + y.sin();
        let b = y * 4.715_f32 - x.cos();
        let mut acc = Accumulator955::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.711_f32 + y.sin();
        let b = y * 0.211_f32 - x.cos();
        let mut acc = Accumulator955::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.737_f32 + y.sin();
        let b = y * 7.055_f32 - x.cos();
        let mut acc = Accumulator955::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.598_f32 + y.sin();
        let b = y * 2.8_f32 - x.cos();
        let mut acc = Accumulator955::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.524_f32 + y.sin();
        let b = y * 8.085_f32 - x.cos();
        let mut acc = Accumulator955::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.907_f32 + y.sin();
        let b = y * 1.565_f32 - x.cos();
        let mut acc = Accumulator955::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.982_f32 + y.sin();
        let b = y * 9.252_f32 - x.cos();
        let mut acc = Accumulator955::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.671_f32 + y.sin();
        let b = y * 4.32_f32 - x.cos();
        let mut acc = Accumulator955::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.122_f32 + y.sin();
        let b = y * 8.216_f32 - x.cos();
        let mut acc = Accumulator955::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.596_f32 + y.sin();
        let b = y * 0.812_f32 - x.cos();
        let mut acc = Accumulator955::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.251_f32 + y.sin();
        let b = y * 3.935_f32 - x.cos();
        let mut acc = Accumulator955::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_955(seed: u64) -> u64 {
        let re = Regex::new(r"m955-(\d+)").unwrap();
        let hay = format!("m955-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_955() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_955(total as u64) % 997) as f32;
        total
    }
}

pub mod m956 {
    use super::*;

    pub struct Accumulator956<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator956<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.784_f32 + y.sin();
        let b = y * 1.242_f32 - x.cos();
        let mut acc = Accumulator956::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.664_f32 + y.sin();
        let b = y * 0.776_f32 - x.cos();
        let mut acc = Accumulator956::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.262_f32 + y.sin();
        let b = y * 0.275_f32 - x.cos();
        let mut acc = Accumulator956::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.028_f32 + y.sin();
        let b = y * 3.178_f32 - x.cos();
        let mut acc = Accumulator956::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.09_f32 + y.sin();
        let b = y * 7.797_f32 - x.cos();
        let mut acc = Accumulator956::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.116_f32 + y.sin();
        let b = y * 9.645_f32 - x.cos();
        let mut acc = Accumulator956::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.243_f32 + y.sin();
        let b = y * 8.771_f32 - x.cos();
        let mut acc = Accumulator956::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.473_f32 + y.sin();
        let b = y * 8.861_f32 - x.cos();
        let mut acc = Accumulator956::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.708_f32 + y.sin();
        let b = y * 5.542_f32 - x.cos();
        let mut acc = Accumulator956::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.508_f32 + y.sin();
        let b = y * 6.166_f32 - x.cos();
        let mut acc = Accumulator956::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.94_f32 + y.sin();
        let b = y * 8.199_f32 - x.cos();
        let mut acc = Accumulator956::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.215_f32 + y.sin();
        let b = y * 4.456_f32 - x.cos();
        let mut acc = Accumulator956::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.539_f32 + y.sin();
        let b = y * 7.507_f32 - x.cos();
        let mut acc = Accumulator956::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.759_f32 + y.sin();
        let b = y * 3.273_f32 - x.cos();
        let mut acc = Accumulator956::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.268_f32 + y.sin();
        let b = y * 6.366_f32 - x.cos();
        let mut acc = Accumulator956::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.922_f32 + y.sin();
        let b = y * 9.233_f32 - x.cos();
        let mut acc = Accumulator956::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.09_f32 + y.sin();
        let b = y * 4.263_f32 - x.cos();
        let mut acc = Accumulator956::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.389_f32 + y.sin();
        let b = y * 5.101_f32 - x.cos();
        let mut acc = Accumulator956::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.434_f32 + y.sin();
        let b = y * 4.578_f32 - x.cos();
        let mut acc = Accumulator956::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.543_f32 + y.sin();
        let b = y * 9.647_f32 - x.cos();
        let mut acc = Accumulator956::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_956(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_956() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_956(total as u64) % 997) as f32;
        total
    }
}

pub mod m957 {
    use super::*;

    pub struct Accumulator957<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator957<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.65_f32 + y.sin();
        let b = y * 3.258_f32 - x.cos();
        let mut acc = Accumulator957::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.82_f32 + y.sin();
        let b = y * 0.823_f32 - x.cos();
        let mut acc = Accumulator957::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.705_f32 + y.sin();
        let b = y * 3.628_f32 - x.cos();
        let mut acc = Accumulator957::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.163_f32 + y.sin();
        let b = y * 9.374_f32 - x.cos();
        let mut acc = Accumulator957::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.674_f32 + y.sin();
        let b = y * 2.787_f32 - x.cos();
        let mut acc = Accumulator957::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.842_f32 + y.sin();
        let b = y * 0.339_f32 - x.cos();
        let mut acc = Accumulator957::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.248_f32 + y.sin();
        let b = y * 7.712_f32 - x.cos();
        let mut acc = Accumulator957::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.314_f32 + y.sin();
        let b = y * 5.024_f32 - x.cos();
        let mut acc = Accumulator957::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.234_f32 + y.sin();
        let b = y * 8.095_f32 - x.cos();
        let mut acc = Accumulator957::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.631_f32 + y.sin();
        let b = y * 5.157_f32 - x.cos();
        let mut acc = Accumulator957::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.022_f32 + y.sin();
        let b = y * 4.279_f32 - x.cos();
        let mut acc = Accumulator957::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.188_f32 + y.sin();
        let b = y * 6.667_f32 - x.cos();
        let mut acc = Accumulator957::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.65_f32 + y.sin();
        let b = y * 2.675_f32 - x.cos();
        let mut acc = Accumulator957::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.768_f32 + y.sin();
        let b = y * 6.744_f32 - x.cos();
        let mut acc = Accumulator957::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.612_f32 + y.sin();
        let b = y * 9.354_f32 - x.cos();
        let mut acc = Accumulator957::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.44_f32 + y.sin();
        let b = y * 4.53_f32 - x.cos();
        let mut acc = Accumulator957::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.041_f32 + y.sin();
        let b = y * 5.189_f32 - x.cos();
        let mut acc = Accumulator957::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.709_f32 + y.sin();
        let b = y * 5.745_f32 - x.cos();
        let mut acc = Accumulator957::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.943_f32 + y.sin();
        let b = y * 9.489_f32 - x.cos();
        let mut acc = Accumulator957::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.611_f32 + y.sin();
        let b = y * 9.773_f32 - x.cos();
        let mut acc = Accumulator957::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_957(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(957u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_957() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_957(total as u64) % 997) as f32;
        total
    }
}

pub mod m958 {
    use super::*;

    pub struct Accumulator958<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator958<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.115_f32 + y.sin();
        let b = y * 9.613_f32 - x.cos();
        let mut acc = Accumulator958::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.279_f32 + y.sin();
        let b = y * 8.53_f32 - x.cos();
        let mut acc = Accumulator958::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.423_f32 + y.sin();
        let b = y * 5.137_f32 - x.cos();
        let mut acc = Accumulator958::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.142_f32 + y.sin();
        let b = y * 7.061_f32 - x.cos();
        let mut acc = Accumulator958::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.212_f32 + y.sin();
        let b = y * 9.886_f32 - x.cos();
        let mut acc = Accumulator958::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.946_f32 + y.sin();
        let b = y * 1.06_f32 - x.cos();
        let mut acc = Accumulator958::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.462_f32 + y.sin();
        let b = y * 3.637_f32 - x.cos();
        let mut acc = Accumulator958::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.004_f32 + y.sin();
        let b = y * 5.851_f32 - x.cos();
        let mut acc = Accumulator958::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.585_f32 + y.sin();
        let b = y * 3.164_f32 - x.cos();
        let mut acc = Accumulator958::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.192_f32 + y.sin();
        let b = y * 6.857_f32 - x.cos();
        let mut acc = Accumulator958::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.444_f32 + y.sin();
        let b = y * 4.679_f32 - x.cos();
        let mut acc = Accumulator958::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.247_f32 + y.sin();
        let b = y * 0.962_f32 - x.cos();
        let mut acc = Accumulator958::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.11_f32 + y.sin();
        let b = y * 7.819_f32 - x.cos();
        let mut acc = Accumulator958::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.681_f32 + y.sin();
        let b = y * 4.394_f32 - x.cos();
        let mut acc = Accumulator958::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.172_f32 + y.sin();
        let b = y * 0.894_f32 - x.cos();
        let mut acc = Accumulator958::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.986_f32 + y.sin();
        let b = y * 7.544_f32 - x.cos();
        let mut acc = Accumulator958::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.316_f32 + y.sin();
        let b = y * 2.387_f32 - x.cos();
        let mut acc = Accumulator958::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.996_f32 + y.sin();
        let b = y * 0.817_f32 - x.cos();
        let mut acc = Accumulator958::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.916_f32 + y.sin();
        let b = y * 6.329_f32 - x.cos();
        let mut acc = Accumulator958::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.451_f32 + y.sin();
        let b = y * 4.855_f32 - x.cos();
        let mut acc = Accumulator958::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_958(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_958() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_958(total as u64) % 997) as f32;
        total
    }
}

pub mod m959 {
    use super::*;

    pub struct Accumulator959<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator959<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.102_f32 + y.sin();
        let b = y * 5.946_f32 - x.cos();
        let mut acc = Accumulator959::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.55_f32 + y.sin();
        let b = y * 4.822_f32 - x.cos();
        let mut acc = Accumulator959::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.466_f32 + y.sin();
        let b = y * 0.168_f32 - x.cos();
        let mut acc = Accumulator959::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.054_f32 + y.sin();
        let b = y * 1.213_f32 - x.cos();
        let mut acc = Accumulator959::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.936_f32 + y.sin();
        let b = y * 6.522_f32 - x.cos();
        let mut acc = Accumulator959::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.287_f32 + y.sin();
        let b = y * 8.395_f32 - x.cos();
        let mut acc = Accumulator959::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.795_f32 + y.sin();
        let b = y * 8.825_f32 - x.cos();
        let mut acc = Accumulator959::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.454_f32 + y.sin();
        let b = y * 3.494_f32 - x.cos();
        let mut acc = Accumulator959::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.337_f32 + y.sin();
        let b = y * 2.345_f32 - x.cos();
        let mut acc = Accumulator959::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.307_f32 + y.sin();
        let b = y * 7.861_f32 - x.cos();
        let mut acc = Accumulator959::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.635_f32 + y.sin();
        let b = y * 6.193_f32 - x.cos();
        let mut acc = Accumulator959::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.604_f32 + y.sin();
        let b = y * 5.517_f32 - x.cos();
        let mut acc = Accumulator959::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.676_f32 + y.sin();
        let b = y * 7.634_f32 - x.cos();
        let mut acc = Accumulator959::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.466_f32 + y.sin();
        let b = y * 6.409_f32 - x.cos();
        let mut acc = Accumulator959::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.95_f32 + y.sin();
        let b = y * 6.891_f32 - x.cos();
        let mut acc = Accumulator959::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.392_f32 + y.sin();
        let b = y * 6.573_f32 - x.cos();
        let mut acc = Accumulator959::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.15_f32 + y.sin();
        let b = y * 7.139_f32 - x.cos();
        let mut acc = Accumulator959::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.164_f32 + y.sin();
        let b = y * 0.17_f32 - x.cos();
        let mut acc = Accumulator959::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.14_f32 + y.sin();
        let b = y * 4.04_f32 - x.cos();
        let mut acc = Accumulator959::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.628_f32 + y.sin();
        let b = y * 5.765_f32 - x.cos();
        let mut acc = Accumulator959::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_959(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_959() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_959(total as u64) % 997) as f32;
        total
    }
}

pub mod m960 {
    use super::*;

    pub struct Accumulator960<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator960<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.37_f32 + y.sin();
        let b = y * 3.35_f32 - x.cos();
        let mut acc = Accumulator960::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.216_f32 + y.sin();
        let b = y * 9.093_f32 - x.cos();
        let mut acc = Accumulator960::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.634_f32 + y.sin();
        let b = y * 7.452_f32 - x.cos();
        let mut acc = Accumulator960::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.019_f32 + y.sin();
        let b = y * 3.638_f32 - x.cos();
        let mut acc = Accumulator960::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.42_f32 + y.sin();
        let b = y * 9.466_f32 - x.cos();
        let mut acc = Accumulator960::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.461_f32 + y.sin();
        let b = y * 0.375_f32 - x.cos();
        let mut acc = Accumulator960::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.463_f32 + y.sin();
        let b = y * 1.187_f32 - x.cos();
        let mut acc = Accumulator960::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.555_f32 + y.sin();
        let b = y * 5.766_f32 - x.cos();
        let mut acc = Accumulator960::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.614_f32 + y.sin();
        let b = y * 4.832_f32 - x.cos();
        let mut acc = Accumulator960::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.124_f32 + y.sin();
        let b = y * 1.509_f32 - x.cos();
        let mut acc = Accumulator960::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.86_f32 + y.sin();
        let b = y * 8.182_f32 - x.cos();
        let mut acc = Accumulator960::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.677_f32 + y.sin();
        let b = y * 6.834_f32 - x.cos();
        let mut acc = Accumulator960::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.126_f32 + y.sin();
        let b = y * 0.712_f32 - x.cos();
        let mut acc = Accumulator960::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.499_f32 + y.sin();
        let b = y * 8.603_f32 - x.cos();
        let mut acc = Accumulator960::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.36_f32 + y.sin();
        let b = y * 9.713_f32 - x.cos();
        let mut acc = Accumulator960::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.489_f32 + y.sin();
        let b = y * 5.503_f32 - x.cos();
        let mut acc = Accumulator960::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.815_f32 + y.sin();
        let b = y * 0.746_f32 - x.cos();
        let mut acc = Accumulator960::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.203_f32 + y.sin();
        let b = y * 6.55_f32 - x.cos();
        let mut acc = Accumulator960::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.714_f32 + y.sin();
        let b = y * 7.639_f32 - x.cos();
        let mut acc = Accumulator960::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.918_f32 + y.sin();
        let b = y * 3.734_f32 - x.cos();
        let mut acc = Accumulator960::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_960(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m960-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_960() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_960(total as u64) % 997) as f32;
        total
    }
}

pub mod m961 {
    use super::*;

    pub struct Accumulator961<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator961<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.711_f32 + y.sin();
        let b = y * 4.105_f32 - x.cos();
        let mut acc = Accumulator961::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.782_f32 + y.sin();
        let b = y * 4.032_f32 - x.cos();
        let mut acc = Accumulator961::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.84_f32 + y.sin();
        let b = y * 9.15_f32 - x.cos();
        let mut acc = Accumulator961::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.547_f32 + y.sin();
        let b = y * 7.609_f32 - x.cos();
        let mut acc = Accumulator961::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.113_f32 + y.sin();
        let b = y * 0.114_f32 - x.cos();
        let mut acc = Accumulator961::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.604_f32 + y.sin();
        let b = y * 3.513_f32 - x.cos();
        let mut acc = Accumulator961::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.43_f32 + y.sin();
        let b = y * 1.968_f32 - x.cos();
        let mut acc = Accumulator961::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.626_f32 + y.sin();
        let b = y * 3.681_f32 - x.cos();
        let mut acc = Accumulator961::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.137_f32 + y.sin();
        let b = y * 2.507_f32 - x.cos();
        let mut acc = Accumulator961::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.014_f32 + y.sin();
        let b = y * 4.578_f32 - x.cos();
        let mut acc = Accumulator961::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.84_f32 + y.sin();
        let b = y * 0.165_f32 - x.cos();
        let mut acc = Accumulator961::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.505_f32 + y.sin();
        let b = y * 4.398_f32 - x.cos();
        let mut acc = Accumulator961::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.041_f32 + y.sin();
        let b = y * 7.743_f32 - x.cos();
        let mut acc = Accumulator961::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.144_f32 + y.sin();
        let b = y * 4.974_f32 - x.cos();
        let mut acc = Accumulator961::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.158_f32 + y.sin();
        let b = y * 3.412_f32 - x.cos();
        let mut acc = Accumulator961::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.705_f32 + y.sin();
        let b = y * 5.682_f32 - x.cos();
        let mut acc = Accumulator961::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.033_f32 + y.sin();
        let b = y * 6.086_f32 - x.cos();
        let mut acc = Accumulator961::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.341_f32 + y.sin();
        let b = y * 1.947_f32 - x.cos();
        let mut acc = Accumulator961::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.586_f32 + y.sin();
        let b = y * 7.583_f32 - x.cos();
        let mut acc = Accumulator961::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.45_f32 + y.sin();
        let b = y * 4.748_f32 - x.cos();
        let mut acc = Accumulator961::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_961(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_961() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_961(total as u64) % 997) as f32;
        total
    }
}

pub mod m962 {
    use super::*;

    pub struct Accumulator962<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator962<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.651_f32 + y.sin();
        let b = y * 7.727_f32 - x.cos();
        let mut acc = Accumulator962::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.189_f32 + y.sin();
        let b = y * 4.172_f32 - x.cos();
        let mut acc = Accumulator962::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.966_f32 + y.sin();
        let b = y * 0.591_f32 - x.cos();
        let mut acc = Accumulator962::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.352_f32 + y.sin();
        let b = y * 5.4_f32 - x.cos();
        let mut acc = Accumulator962::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.13_f32 + y.sin();
        let b = y * 5.399_f32 - x.cos();
        let mut acc = Accumulator962::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.626_f32 + y.sin();
        let b = y * 5.003_f32 - x.cos();
        let mut acc = Accumulator962::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.99_f32 + y.sin();
        let b = y * 8.054_f32 - x.cos();
        let mut acc = Accumulator962::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.361_f32 + y.sin();
        let b = y * 6.619_f32 - x.cos();
        let mut acc = Accumulator962::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.441_f32 + y.sin();
        let b = y * 7.606_f32 - x.cos();
        let mut acc = Accumulator962::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.258_f32 + y.sin();
        let b = y * 1.105_f32 - x.cos();
        let mut acc = Accumulator962::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.268_f32 + y.sin();
        let b = y * 7.287_f32 - x.cos();
        let mut acc = Accumulator962::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.136_f32 + y.sin();
        let b = y * 6.48_f32 - x.cos();
        let mut acc = Accumulator962::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.169_f32 + y.sin();
        let b = y * 0.327_f32 - x.cos();
        let mut acc = Accumulator962::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.695_f32 + y.sin();
        let b = y * 1.568_f32 - x.cos();
        let mut acc = Accumulator962::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.496_f32 + y.sin();
        let b = y * 1.459_f32 - x.cos();
        let mut acc = Accumulator962::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.332_f32 + y.sin();
        let b = y * 7.108_f32 - x.cos();
        let mut acc = Accumulator962::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.413_f32 + y.sin();
        let b = y * 2.105_f32 - x.cos();
        let mut acc = Accumulator962::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.483_f32 + y.sin();
        let b = y * 1.619_f32 - x.cos();
        let mut acc = Accumulator962::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.047_f32 + y.sin();
        let b = y * 4.235_f32 - x.cos();
        let mut acc = Accumulator962::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.889_f32 + y.sin();
        let b = y * 1.739_f32 - x.cos();
        let mut acc = Accumulator962::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_962(seed: u64) -> u64 {
        let re = Regex::new(r"m962-(\d+)").unwrap();
        let hay = format!("m962-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_962() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_962(total as u64) % 997) as f32;
        total
    }
}

pub mod m963 {
    use super::*;

    pub struct Accumulator963<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator963<T> {
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
        let b = y * 0.426_f32 - x.cos();
        let mut acc = Accumulator963::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.442_f32 + y.sin();
        let b = y * 8.035_f32 - x.cos();
        let mut acc = Accumulator963::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.807_f32 + y.sin();
        let b = y * 4.396_f32 - x.cos();
        let mut acc = Accumulator963::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.872_f32 + y.sin();
        let b = y * 7.736_f32 - x.cos();
        let mut acc = Accumulator963::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.132_f32 + y.sin();
        let b = y * 0.471_f32 - x.cos();
        let mut acc = Accumulator963::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.873_f32 + y.sin();
        let b = y * 3.608_f32 - x.cos();
        let mut acc = Accumulator963::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.633_f32 + y.sin();
        let b = y * 5.286_f32 - x.cos();
        let mut acc = Accumulator963::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.4_f32 + y.sin();
        let b = y * 6.853_f32 - x.cos();
        let mut acc = Accumulator963::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.156_f32 + y.sin();
        let b = y * 7.333_f32 - x.cos();
        let mut acc = Accumulator963::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.137_f32 + y.sin();
        let b = y * 8.026_f32 - x.cos();
        let mut acc = Accumulator963::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.488_f32 + y.sin();
        let b = y * 8.117_f32 - x.cos();
        let mut acc = Accumulator963::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.795_f32 + y.sin();
        let b = y * 9.722_f32 - x.cos();
        let mut acc = Accumulator963::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.064_f32 + y.sin();
        let b = y * 2.112_f32 - x.cos();
        let mut acc = Accumulator963::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.892_f32 + y.sin();
        let b = y * 6.378_f32 - x.cos();
        let mut acc = Accumulator963::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.629_f32 + y.sin();
        let b = y * 3.256_f32 - x.cos();
        let mut acc = Accumulator963::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.945_f32 + y.sin();
        let b = y * 1.283_f32 - x.cos();
        let mut acc = Accumulator963::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.989_f32 + y.sin();
        let b = y * 3.909_f32 - x.cos();
        let mut acc = Accumulator963::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.082_f32 + y.sin();
        let b = y * 0.681_f32 - x.cos();
        let mut acc = Accumulator963::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.181_f32 + y.sin();
        let b = y * 1.303_f32 - x.cos();
        let mut acc = Accumulator963::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.872_f32 + y.sin();
        let b = y * 6.512_f32 - x.cos();
        let mut acc = Accumulator963::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_963(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_963() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_963(total as u64) % 997) as f32;
        total
    }
}

pub mod m964 {
    use super::*;

    pub struct Accumulator964<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator964<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.45_f32 + y.sin();
        let b = y * 5.318_f32 - x.cos();
        let mut acc = Accumulator964::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.014_f32 + y.sin();
        let b = y * 3.498_f32 - x.cos();
        let mut acc = Accumulator964::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.513_f32 + y.sin();
        let b = y * 5.599_f32 - x.cos();
        let mut acc = Accumulator964::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.237_f32 + y.sin();
        let b = y * 1.884_f32 - x.cos();
        let mut acc = Accumulator964::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.336_f32 + y.sin();
        let b = y * 0.309_f32 - x.cos();
        let mut acc = Accumulator964::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.912_f32 + y.sin();
        let b = y * 5.159_f32 - x.cos();
        let mut acc = Accumulator964::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.935_f32 + y.sin();
        let b = y * 1.534_f32 - x.cos();
        let mut acc = Accumulator964::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.25_f32 + y.sin();
        let b = y * 9.72_f32 - x.cos();
        let mut acc = Accumulator964::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.456_f32 + y.sin();
        let b = y * 0.635_f32 - x.cos();
        let mut acc = Accumulator964::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.85_f32 + y.sin();
        let b = y * 9.861_f32 - x.cos();
        let mut acc = Accumulator964::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.26_f32 + y.sin();
        let b = y * 7.189_f32 - x.cos();
        let mut acc = Accumulator964::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.062_f32 + y.sin();
        let b = y * 7.313_f32 - x.cos();
        let mut acc = Accumulator964::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.895_f32 + y.sin();
        let b = y * 8.059_f32 - x.cos();
        let mut acc = Accumulator964::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.845_f32 + y.sin();
        let b = y * 6.038_f32 - x.cos();
        let mut acc = Accumulator964::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.402_f32 + y.sin();
        let b = y * 2.215_f32 - x.cos();
        let mut acc = Accumulator964::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.048_f32 + y.sin();
        let b = y * 3.323_f32 - x.cos();
        let mut acc = Accumulator964::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.481_f32 + y.sin();
        let b = y * 3.071_f32 - x.cos();
        let mut acc = Accumulator964::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.354_f32 + y.sin();
        let b = y * 4.514_f32 - x.cos();
        let mut acc = Accumulator964::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.129_f32 + y.sin();
        let b = y * 6.347_f32 - x.cos();
        let mut acc = Accumulator964::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.896_f32 + y.sin();
        let b = y * 5.109_f32 - x.cos();
        let mut acc = Accumulator964::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_964(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(964u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_964() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_964(total as u64) % 997) as f32;
        total
    }
}

pub mod m965 {
    use super::*;

    pub struct Accumulator965<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator965<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.543_f32 + y.sin();
        let b = y * 4.776_f32 - x.cos();
        let mut acc = Accumulator965::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.935_f32 + y.sin();
        let b = y * 1.803_f32 - x.cos();
        let mut acc = Accumulator965::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.505_f32 + y.sin();
        let b = y * 2.906_f32 - x.cos();
        let mut acc = Accumulator965::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.967_f32 + y.sin();
        let b = y * 4.126_f32 - x.cos();
        let mut acc = Accumulator965::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.591_f32 + y.sin();
        let b = y * 6.298_f32 - x.cos();
        let mut acc = Accumulator965::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.68_f32 + y.sin();
        let b = y * 2.074_f32 - x.cos();
        let mut acc = Accumulator965::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.788_f32 + y.sin();
        let b = y * 5.52_f32 - x.cos();
        let mut acc = Accumulator965::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.695_f32 + y.sin();
        let b = y * 8.197_f32 - x.cos();
        let mut acc = Accumulator965::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.956_f32 + y.sin();
        let b = y * 4.398_f32 - x.cos();
        let mut acc = Accumulator965::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.931_f32 + y.sin();
        let b = y * 5.95_f32 - x.cos();
        let mut acc = Accumulator965::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.261_f32 + y.sin();
        let b = y * 7.789_f32 - x.cos();
        let mut acc = Accumulator965::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.154_f32 + y.sin();
        let b = y * 1.435_f32 - x.cos();
        let mut acc = Accumulator965::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.184_f32 + y.sin();
        let b = y * 3.425_f32 - x.cos();
        let mut acc = Accumulator965::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.674_f32 + y.sin();
        let b = y * 6.684_f32 - x.cos();
        let mut acc = Accumulator965::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.649_f32 + y.sin();
        let b = y * 5.748_f32 - x.cos();
        let mut acc = Accumulator965::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.003_f32 + y.sin();
        let b = y * 2.935_f32 - x.cos();
        let mut acc = Accumulator965::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.155_f32 + y.sin();
        let b = y * 4.752_f32 - x.cos();
        let mut acc = Accumulator965::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.642_f32 + y.sin();
        let b = y * 7.622_f32 - x.cos();
        let mut acc = Accumulator965::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.58_f32 + y.sin();
        let b = y * 0.392_f32 - x.cos();
        let mut acc = Accumulator965::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.811_f32 + y.sin();
        let b = y * 0.141_f32 - x.cos();
        let mut acc = Accumulator965::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_965(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_965() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_965(total as u64) % 997) as f32;
        total
    }
}

pub mod m966 {
    use super::*;

    pub struct Accumulator966<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator966<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.806_f32 + y.sin();
        let b = y * 5.081_f32 - x.cos();
        let mut acc = Accumulator966::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.255_f32 + y.sin();
        let b = y * 9.357_f32 - x.cos();
        let mut acc = Accumulator966::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.73_f32 + y.sin();
        let b = y * 7.268_f32 - x.cos();
        let mut acc = Accumulator966::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.781_f32 + y.sin();
        let b = y * 6.543_f32 - x.cos();
        let mut acc = Accumulator966::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.938_f32 + y.sin();
        let b = y * 3.241_f32 - x.cos();
        let mut acc = Accumulator966::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.513_f32 + y.sin();
        let b = y * 7.936_f32 - x.cos();
        let mut acc = Accumulator966::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.318_f32 + y.sin();
        let b = y * 6.171_f32 - x.cos();
        let mut acc = Accumulator966::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.064_f32 + y.sin();
        let b = y * 3.636_f32 - x.cos();
        let mut acc = Accumulator966::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.928_f32 + y.sin();
        let b = y * 6.624_f32 - x.cos();
        let mut acc = Accumulator966::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.756_f32 + y.sin();
        let b = y * 1.685_f32 - x.cos();
        let mut acc = Accumulator966::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.365_f32 + y.sin();
        let b = y * 5.037_f32 - x.cos();
        let mut acc = Accumulator966::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.14_f32 + y.sin();
        let b = y * 3.333_f32 - x.cos();
        let mut acc = Accumulator966::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.205_f32 + y.sin();
        let b = y * 9.629_f32 - x.cos();
        let mut acc = Accumulator966::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.792_f32 + y.sin();
        let b = y * 6.691_f32 - x.cos();
        let mut acc = Accumulator966::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.974_f32 + y.sin();
        let b = y * 6.653_f32 - x.cos();
        let mut acc = Accumulator966::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.358_f32 + y.sin();
        let b = y * 0.49_f32 - x.cos();
        let mut acc = Accumulator966::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.673_f32 + y.sin();
        let b = y * 4.269_f32 - x.cos();
        let mut acc = Accumulator966::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.368_f32 + y.sin();
        let b = y * 4.304_f32 - x.cos();
        let mut acc = Accumulator966::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.169_f32 + y.sin();
        let b = y * 9.693_f32 - x.cos();
        let mut acc = Accumulator966::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.16_f32 + y.sin();
        let b = y * 5.385_f32 - x.cos();
        let mut acc = Accumulator966::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_966(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_966() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_966(total as u64) % 997) as f32;
        total
    }
}

pub mod m967 {
    use super::*;

    pub struct Accumulator967<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator967<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.501_f32 + y.sin();
        let b = y * 4.225_f32 - x.cos();
        let mut acc = Accumulator967::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.265_f32 + y.sin();
        let b = y * 7.892_f32 - x.cos();
        let mut acc = Accumulator967::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.354_f32 + y.sin();
        let b = y * 2.256_f32 - x.cos();
        let mut acc = Accumulator967::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.327_f32 + y.sin();
        let b = y * 9.352_f32 - x.cos();
        let mut acc = Accumulator967::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.664_f32 + y.sin();
        let b = y * 6.448_f32 - x.cos();
        let mut acc = Accumulator967::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.784_f32 + y.sin();
        let b = y * 7.71_f32 - x.cos();
        let mut acc = Accumulator967::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.832_f32 + y.sin();
        let b = y * 6.847_f32 - x.cos();
        let mut acc = Accumulator967::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.428_f32 + y.sin();
        let b = y * 0.158_f32 - x.cos();
        let mut acc = Accumulator967::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.353_f32 + y.sin();
        let b = y * 2.067_f32 - x.cos();
        let mut acc = Accumulator967::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.605_f32 + y.sin();
        let b = y * 7.379_f32 - x.cos();
        let mut acc = Accumulator967::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.714_f32 + y.sin();
        let b = y * 0.961_f32 - x.cos();
        let mut acc = Accumulator967::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.942_f32 + y.sin();
        let b = y * 0.486_f32 - x.cos();
        let mut acc = Accumulator967::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.403_f32 + y.sin();
        let b = y * 3.334_f32 - x.cos();
        let mut acc = Accumulator967::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.649_f32 + y.sin();
        let b = y * 7.843_f32 - x.cos();
        let mut acc = Accumulator967::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.936_f32 + y.sin();
        let b = y * 3.167_f32 - x.cos();
        let mut acc = Accumulator967::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.844_f32 + y.sin();
        let b = y * 4.619_f32 - x.cos();
        let mut acc = Accumulator967::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.745_f32 + y.sin();
        let b = y * 5.658_f32 - x.cos();
        let mut acc = Accumulator967::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.746_f32 + y.sin();
        let b = y * 4.626_f32 - x.cos();
        let mut acc = Accumulator967::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.184_f32 + y.sin();
        let b = y * 2.709_f32 - x.cos();
        let mut acc = Accumulator967::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.559_f32 + y.sin();
        let b = y * 2.911_f32 - x.cos();
        let mut acc = Accumulator967::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_967(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m967-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_967() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_967(total as u64) % 997) as f32;
        total
    }
}

pub mod m968 {
    use super::*;

    pub struct Accumulator968<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator968<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.871_f32 + y.sin();
        let b = y * 7.623_f32 - x.cos();
        let mut acc = Accumulator968::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.41_f32 + y.sin();
        let b = y * 9.444_f32 - x.cos();
        let mut acc = Accumulator968::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.706_f32 + y.sin();
        let b = y * 9.178_f32 - x.cos();
        let mut acc = Accumulator968::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.144_f32 + y.sin();
        let b = y * 7.598_f32 - x.cos();
        let mut acc = Accumulator968::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.966_f32 + y.sin();
        let b = y * 1.958_f32 - x.cos();
        let mut acc = Accumulator968::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.301_f32 + y.sin();
        let b = y * 2.092_f32 - x.cos();
        let mut acc = Accumulator968::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.107_f32 + y.sin();
        let b = y * 6.569_f32 - x.cos();
        let mut acc = Accumulator968::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.169_f32 + y.sin();
        let b = y * 2.586_f32 - x.cos();
        let mut acc = Accumulator968::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.69_f32 + y.sin();
        let b = y * 7.675_f32 - x.cos();
        let mut acc = Accumulator968::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.519_f32 + y.sin();
        let b = y * 9.889_f32 - x.cos();
        let mut acc = Accumulator968::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.048_f32 + y.sin();
        let b = y * 5.788_f32 - x.cos();
        let mut acc = Accumulator968::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.911_f32 + y.sin();
        let b = y * 1.687_f32 - x.cos();
        let mut acc = Accumulator968::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.595_f32 + y.sin();
        let b = y * 5.495_f32 - x.cos();
        let mut acc = Accumulator968::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.238_f32 + y.sin();
        let b = y * 3.265_f32 - x.cos();
        let mut acc = Accumulator968::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.59_f32 + y.sin();
        let b = y * 8.058_f32 - x.cos();
        let mut acc = Accumulator968::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.091_f32 + y.sin();
        let b = y * 0.839_f32 - x.cos();
        let mut acc = Accumulator968::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.163_f32 + y.sin();
        let b = y * 2.423_f32 - x.cos();
        let mut acc = Accumulator968::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.166_f32 + y.sin();
        let b = y * 3.889_f32 - x.cos();
        let mut acc = Accumulator968::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.683_f32 + y.sin();
        let b = y * 3.776_f32 - x.cos();
        let mut acc = Accumulator968::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.659_f32 + y.sin();
        let b = y * 1.755_f32 - x.cos();
        let mut acc = Accumulator968::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_968(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_968() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_968(total as u64) % 997) as f32;
        total
    }
}

pub mod m969 {
    use super::*;

    pub struct Accumulator969<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator969<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.47_f32 + y.sin();
        let b = y * 5.597_f32 - x.cos();
        let mut acc = Accumulator969::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.407_f32 + y.sin();
        let b = y * 8.931_f32 - x.cos();
        let mut acc = Accumulator969::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.597_f32 + y.sin();
        let b = y * 4.489_f32 - x.cos();
        let mut acc = Accumulator969::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.11_f32 + y.sin();
        let b = y * 0.541_f32 - x.cos();
        let mut acc = Accumulator969::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.769_f32 + y.sin();
        let b = y * 7.21_f32 - x.cos();
        let mut acc = Accumulator969::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.593_f32 + y.sin();
        let b = y * 6.01_f32 - x.cos();
        let mut acc = Accumulator969::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.106_f32 + y.sin();
        let b = y * 4.821_f32 - x.cos();
        let mut acc = Accumulator969::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.913_f32 + y.sin();
        let b = y * 5.403_f32 - x.cos();
        let mut acc = Accumulator969::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.774_f32 + y.sin();
        let b = y * 3.338_f32 - x.cos();
        let mut acc = Accumulator969::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.039_f32 + y.sin();
        let b = y * 2.366_f32 - x.cos();
        let mut acc = Accumulator969::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.19_f32 + y.sin();
        let b = y * 9.761_f32 - x.cos();
        let mut acc = Accumulator969::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.543_f32 + y.sin();
        let b = y * 5.244_f32 - x.cos();
        let mut acc = Accumulator969::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.766_f32 + y.sin();
        let b = y * 6.246_f32 - x.cos();
        let mut acc = Accumulator969::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.117_f32 + y.sin();
        let b = y * 9.534_f32 - x.cos();
        let mut acc = Accumulator969::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.541_f32 + y.sin();
        let b = y * 8.788_f32 - x.cos();
        let mut acc = Accumulator969::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.551_f32 + y.sin();
        let b = y * 3.03_f32 - x.cos();
        let mut acc = Accumulator969::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.675_f32 + y.sin();
        let b = y * 7.478_f32 - x.cos();
        let mut acc = Accumulator969::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.217_f32 + y.sin();
        let b = y * 2.891_f32 - x.cos();
        let mut acc = Accumulator969::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.623_f32 + y.sin();
        let b = y * 4.465_f32 - x.cos();
        let mut acc = Accumulator969::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.448_f32 + y.sin();
        let b = y * 4.963_f32 - x.cos();
        let mut acc = Accumulator969::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_969(seed: u64) -> u64 {
        let re = Regex::new(r"m969-(\d+)").unwrap();
        let hay = format!("m969-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_969() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_969(total as u64) % 997) as f32;
        total
    }
}

pub mod m970 {
    use super::*;

    pub struct Accumulator970<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator970<T> {
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
        let b = y * 0.546_f32 - x.cos();
        let mut acc = Accumulator970::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.404_f32 + y.sin();
        let b = y * 5.06_f32 - x.cos();
        let mut acc = Accumulator970::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.589_f32 + y.sin();
        let b = y * 8.039_f32 - x.cos();
        let mut acc = Accumulator970::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.215_f32 + y.sin();
        let b = y * 7.444_f32 - x.cos();
        let mut acc = Accumulator970::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.781_f32 + y.sin();
        let b = y * 0.719_f32 - x.cos();
        let mut acc = Accumulator970::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.065_f32 + y.sin();
        let b = y * 7.139_f32 - x.cos();
        let mut acc = Accumulator970::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.426_f32 + y.sin();
        let b = y * 2.274_f32 - x.cos();
        let mut acc = Accumulator970::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.397_f32 + y.sin();
        let b = y * 2.675_f32 - x.cos();
        let mut acc = Accumulator970::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.17_f32 + y.sin();
        let b = y * 7.426_f32 - x.cos();
        let mut acc = Accumulator970::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.591_f32 + y.sin();
        let b = y * 1.947_f32 - x.cos();
        let mut acc = Accumulator970::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.611_f32 + y.sin();
        let b = y * 0.143_f32 - x.cos();
        let mut acc = Accumulator970::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.635_f32 + y.sin();
        let b = y * 8.881_f32 - x.cos();
        let mut acc = Accumulator970::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.595_f32 + y.sin();
        let b = y * 1.17_f32 - x.cos();
        let mut acc = Accumulator970::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.792_f32 + y.sin();
        let b = y * 5.847_f32 - x.cos();
        let mut acc = Accumulator970::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.707_f32 + y.sin();
        let b = y * 5.841_f32 - x.cos();
        let mut acc = Accumulator970::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.705_f32 + y.sin();
        let b = y * 6.812_f32 - x.cos();
        let mut acc = Accumulator970::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.464_f32 + y.sin();
        let b = y * 3.705_f32 - x.cos();
        let mut acc = Accumulator970::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.916_f32 + y.sin();
        let b = y * 4.774_f32 - x.cos();
        let mut acc = Accumulator970::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.789_f32 + y.sin();
        let b = y * 3.53_f32 - x.cos();
        let mut acc = Accumulator970::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.072_f32 + y.sin();
        let b = y * 8.749_f32 - x.cos();
        let mut acc = Accumulator970::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_970(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_970() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_970(total as u64) % 997) as f32;
        total
    }
}

pub mod m971 {
    use super::*;

    pub struct Accumulator971<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator971<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.447_f32 + y.sin();
        let b = y * 4.52_f32 - x.cos();
        let mut acc = Accumulator971::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.813_f32 + y.sin();
        let b = y * 0.521_f32 - x.cos();
        let mut acc = Accumulator971::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.276_f32 + y.sin();
        let b = y * 1.466_f32 - x.cos();
        let mut acc = Accumulator971::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.779_f32 + y.sin();
        let b = y * 0.537_f32 - x.cos();
        let mut acc = Accumulator971::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.17_f32 + y.sin();
        let b = y * 5.363_f32 - x.cos();
        let mut acc = Accumulator971::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.17_f32 + y.sin();
        let b = y * 8.731_f32 - x.cos();
        let mut acc = Accumulator971::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.23_f32 + y.sin();
        let b = y * 5.141_f32 - x.cos();
        let mut acc = Accumulator971::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.646_f32 + y.sin();
        let b = y * 6.553_f32 - x.cos();
        let mut acc = Accumulator971::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.597_f32 + y.sin();
        let b = y * 6.15_f32 - x.cos();
        let mut acc = Accumulator971::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.813_f32 + y.sin();
        let b = y * 2.845_f32 - x.cos();
        let mut acc = Accumulator971::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.124_f32 + y.sin();
        let b = y * 1.107_f32 - x.cos();
        let mut acc = Accumulator971::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.328_f32 + y.sin();
        let b = y * 6.003_f32 - x.cos();
        let mut acc = Accumulator971::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.41_f32 + y.sin();
        let b = y * 4.848_f32 - x.cos();
        let mut acc = Accumulator971::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.477_f32 + y.sin();
        let b = y * 5.511_f32 - x.cos();
        let mut acc = Accumulator971::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.561_f32 + y.sin();
        let b = y * 3.901_f32 - x.cos();
        let mut acc = Accumulator971::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.219_f32 + y.sin();
        let b = y * 0.936_f32 - x.cos();
        let mut acc = Accumulator971::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.985_f32 + y.sin();
        let b = y * 5.148_f32 - x.cos();
        let mut acc = Accumulator971::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.807_f32 + y.sin();
        let b = y * 6.66_f32 - x.cos();
        let mut acc = Accumulator971::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.119_f32 + y.sin();
        let b = y * 5.012_f32 - x.cos();
        let mut acc = Accumulator971::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.746_f32 + y.sin();
        let b = y * 7.465_f32 - x.cos();
        let mut acc = Accumulator971::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_971(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(971u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_971() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_971(total as u64) % 997) as f32;
        total
    }
}

pub mod m972 {
    use super::*;

    pub struct Accumulator972<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator972<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.404_f32 + y.sin();
        let b = y * 1.18_f32 - x.cos();
        let mut acc = Accumulator972::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.092_f32 + y.sin();
        let b = y * 6.5_f32 - x.cos();
        let mut acc = Accumulator972::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.378_f32 + y.sin();
        let b = y * 4.791_f32 - x.cos();
        let mut acc = Accumulator972::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.917_f32 + y.sin();
        let b = y * 9.729_f32 - x.cos();
        let mut acc = Accumulator972::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.771_f32 + y.sin();
        let b = y * 9.704_f32 - x.cos();
        let mut acc = Accumulator972::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.591_f32 + y.sin();
        let b = y * 1.675_f32 - x.cos();
        let mut acc = Accumulator972::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.371_f32 + y.sin();
        let b = y * 6.923_f32 - x.cos();
        let mut acc = Accumulator972::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.98_f32 + y.sin();
        let b = y * 6.341_f32 - x.cos();
        let mut acc = Accumulator972::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.765_f32 + y.sin();
        let b = y * 6.722_f32 - x.cos();
        let mut acc = Accumulator972::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.467_f32 + y.sin();
        let b = y * 4.425_f32 - x.cos();
        let mut acc = Accumulator972::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.654_f32 + y.sin();
        let b = y * 9.313_f32 - x.cos();
        let mut acc = Accumulator972::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.84_f32 + y.sin();
        let b = y * 0.645_f32 - x.cos();
        let mut acc = Accumulator972::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.615_f32 + y.sin();
        let b = y * 3.248_f32 - x.cos();
        let mut acc = Accumulator972::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.879_f32 + y.sin();
        let b = y * 8.629_f32 - x.cos();
        let mut acc = Accumulator972::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.659_f32 + y.sin();
        let b = y * 6.293_f32 - x.cos();
        let mut acc = Accumulator972::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.339_f32 + y.sin();
        let b = y * 3.425_f32 - x.cos();
        let mut acc = Accumulator972::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.375_f32 + y.sin();
        let b = y * 7.383_f32 - x.cos();
        let mut acc = Accumulator972::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.973_f32 + y.sin();
        let b = y * 3.782_f32 - x.cos();
        let mut acc = Accumulator972::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.68_f32 + y.sin();
        let b = y * 4.762_f32 - x.cos();
        let mut acc = Accumulator972::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.408_f32 + y.sin();
        let b = y * 3.3_f32 - x.cos();
        let mut acc = Accumulator972::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_972(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_972() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_972(total as u64) % 997) as f32;
        total
    }
}

pub mod m973 {
    use super::*;

    pub struct Accumulator973<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator973<T> {
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
        let b = y * 3.802_f32 - x.cos();
        let mut acc = Accumulator973::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.539_f32 + y.sin();
        let b = y * 8.601_f32 - x.cos();
        let mut acc = Accumulator973::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.59_f32 + y.sin();
        let b = y * 4.545_f32 - x.cos();
        let mut acc = Accumulator973::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.229_f32 + y.sin();
        let b = y * 5.188_f32 - x.cos();
        let mut acc = Accumulator973::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.856_f32 + y.sin();
        let b = y * 8.227_f32 - x.cos();
        let mut acc = Accumulator973::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.376_f32 + y.sin();
        let b = y * 2.016_f32 - x.cos();
        let mut acc = Accumulator973::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.19_f32 + y.sin();
        let b = y * 9.734_f32 - x.cos();
        let mut acc = Accumulator973::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.435_f32 + y.sin();
        let b = y * 3.77_f32 - x.cos();
        let mut acc = Accumulator973::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.142_f32 + y.sin();
        let b = y * 8.243_f32 - x.cos();
        let mut acc = Accumulator973::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.614_f32 + y.sin();
        let b = y * 2.471_f32 - x.cos();
        let mut acc = Accumulator973::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.226_f32 + y.sin();
        let b = y * 8.22_f32 - x.cos();
        let mut acc = Accumulator973::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.824_f32 + y.sin();
        let b = y * 9.695_f32 - x.cos();
        let mut acc = Accumulator973::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.07_f32 + y.sin();
        let b = y * 0.697_f32 - x.cos();
        let mut acc = Accumulator973::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.916_f32 + y.sin();
        let b = y * 0.811_f32 - x.cos();
        let mut acc = Accumulator973::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.478_f32 + y.sin();
        let b = y * 5.838_f32 - x.cos();
        let mut acc = Accumulator973::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.936_f32 + y.sin();
        let b = y * 5.285_f32 - x.cos();
        let mut acc = Accumulator973::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.973_f32 + y.sin();
        let b = y * 8.021_f32 - x.cos();
        let mut acc = Accumulator973::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.22_f32 + y.sin();
        let b = y * 3.969_f32 - x.cos();
        let mut acc = Accumulator973::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.914_f32 + y.sin();
        let b = y * 1.69_f32 - x.cos();
        let mut acc = Accumulator973::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.813_f32 + y.sin();
        let b = y * 2.371_f32 - x.cos();
        let mut acc = Accumulator973::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_973(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_973() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_973(total as u64) % 997) as f32;
        total
    }
}

pub mod m974 {
    use super::*;

    pub struct Accumulator974<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator974<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.634_f32 + y.sin();
        let b = y * 8.325_f32 - x.cos();
        let mut acc = Accumulator974::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.352_f32 + y.sin();
        let b = y * 2.648_f32 - x.cos();
        let mut acc = Accumulator974::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.105_f32 + y.sin();
        let b = y * 7.985_f32 - x.cos();
        let mut acc = Accumulator974::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.428_f32 + y.sin();
        let b = y * 2.231_f32 - x.cos();
        let mut acc = Accumulator974::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.801_f32 + y.sin();
        let b = y * 5.709_f32 - x.cos();
        let mut acc = Accumulator974::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.994_f32 + y.sin();
        let b = y * 7.16_f32 - x.cos();
        let mut acc = Accumulator974::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.424_f32 + y.sin();
        let b = y * 0.812_f32 - x.cos();
        let mut acc = Accumulator974::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.212_f32 + y.sin();
        let b = y * 2.967_f32 - x.cos();
        let mut acc = Accumulator974::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.733_f32 + y.sin();
        let b = y * 6.047_f32 - x.cos();
        let mut acc = Accumulator974::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.15_f32 + y.sin();
        let b = y * 0.406_f32 - x.cos();
        let mut acc = Accumulator974::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.615_f32 + y.sin();
        let b = y * 1.589_f32 - x.cos();
        let mut acc = Accumulator974::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.356_f32 + y.sin();
        let b = y * 6.058_f32 - x.cos();
        let mut acc = Accumulator974::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.672_f32 + y.sin();
        let b = y * 7.01_f32 - x.cos();
        let mut acc = Accumulator974::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.56_f32 + y.sin();
        let b = y * 5.372_f32 - x.cos();
        let mut acc = Accumulator974::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.547_f32 + y.sin();
        let b = y * 7.759_f32 - x.cos();
        let mut acc = Accumulator974::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.842_f32 + y.sin();
        let b = y * 0.961_f32 - x.cos();
        let mut acc = Accumulator974::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.66_f32 + y.sin();
        let b = y * 6.295_f32 - x.cos();
        let mut acc = Accumulator974::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.396_f32 + y.sin();
        let b = y * 8.376_f32 - x.cos();
        let mut acc = Accumulator974::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.85_f32 + y.sin();
        let b = y * 0.182_f32 - x.cos();
        let mut acc = Accumulator974::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.089_f32 + y.sin();
        let b = y * 5.696_f32 - x.cos();
        let mut acc = Accumulator974::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_974(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m974-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_974() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_974(total as u64) % 997) as f32;
        total
    }
}

pub mod m975 {
    use super::*;

    pub struct Accumulator975<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator975<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.136_f32 + y.sin();
        let b = y * 8.118_f32 - x.cos();
        let mut acc = Accumulator975::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.365_f32 + y.sin();
        let b = y * 6.295_f32 - x.cos();
        let mut acc = Accumulator975::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.046_f32 + y.sin();
        let b = y * 8.086_f32 - x.cos();
        let mut acc = Accumulator975::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.16_f32 + y.sin();
        let b = y * 7.151_f32 - x.cos();
        let mut acc = Accumulator975::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.164_f32 + y.sin();
        let b = y * 1.944_f32 - x.cos();
        let mut acc = Accumulator975::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.885_f32 + y.sin();
        let b = y * 1.637_f32 - x.cos();
        let mut acc = Accumulator975::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.043_f32 + y.sin();
        let b = y * 9.356_f32 - x.cos();
        let mut acc = Accumulator975::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.931_f32 + y.sin();
        let b = y * 8.039_f32 - x.cos();
        let mut acc = Accumulator975::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.318_f32 + y.sin();
        let b = y * 5.081_f32 - x.cos();
        let mut acc = Accumulator975::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.668_f32 + y.sin();
        let b = y * 0.175_f32 - x.cos();
        let mut acc = Accumulator975::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.653_f32 + y.sin();
        let b = y * 5.405_f32 - x.cos();
        let mut acc = Accumulator975::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.631_f32 + y.sin();
        let b = y * 6.843_f32 - x.cos();
        let mut acc = Accumulator975::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.648_f32 + y.sin();
        let b = y * 0.443_f32 - x.cos();
        let mut acc = Accumulator975::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.831_f32 + y.sin();
        let b = y * 3.952_f32 - x.cos();
        let mut acc = Accumulator975::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.369_f32 + y.sin();
        let b = y * 7.145_f32 - x.cos();
        let mut acc = Accumulator975::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.012_f32 + y.sin();
        let b = y * 0.869_f32 - x.cos();
        let mut acc = Accumulator975::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.151_f32 + y.sin();
        let b = y * 8.885_f32 - x.cos();
        let mut acc = Accumulator975::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.785_f32 + y.sin();
        let b = y * 2.638_f32 - x.cos();
        let mut acc = Accumulator975::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.773_f32 + y.sin();
        let b = y * 7.175_f32 - x.cos();
        let mut acc = Accumulator975::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.682_f32 + y.sin();
        let b = y * 4.544_f32 - x.cos();
        let mut acc = Accumulator975::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_975(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_975() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_975(total as u64) % 997) as f32;
        total
    }
}

pub mod m976 {
    use super::*;

    pub struct Accumulator976<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator976<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.278_f32 + y.sin();
        let b = y * 1.433_f32 - x.cos();
        let mut acc = Accumulator976::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.111_f32 + y.sin();
        let b = y * 4.155_f32 - x.cos();
        let mut acc = Accumulator976::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.249_f32 + y.sin();
        let b = y * 7.039_f32 - x.cos();
        let mut acc = Accumulator976::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.139_f32 + y.sin();
        let b = y * 1.457_f32 - x.cos();
        let mut acc = Accumulator976::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.469_f32 + y.sin();
        let b = y * 0.334_f32 - x.cos();
        let mut acc = Accumulator976::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.464_f32 + y.sin();
        let b = y * 1.812_f32 - x.cos();
        let mut acc = Accumulator976::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.849_f32 + y.sin();
        let b = y * 5.9_f32 - x.cos();
        let mut acc = Accumulator976::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.834_f32 + y.sin();
        let b = y * 0.355_f32 - x.cos();
        let mut acc = Accumulator976::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.168_f32 + y.sin();
        let b = y * 2.49_f32 - x.cos();
        let mut acc = Accumulator976::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.444_f32 + y.sin();
        let b = y * 0.483_f32 - x.cos();
        let mut acc = Accumulator976::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.134_f32 + y.sin();
        let b = y * 5.102_f32 - x.cos();
        let mut acc = Accumulator976::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.226_f32 + y.sin();
        let b = y * 0.849_f32 - x.cos();
        let mut acc = Accumulator976::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.008_f32 + y.sin();
        let b = y * 0.789_f32 - x.cos();
        let mut acc = Accumulator976::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.765_f32 + y.sin();
        let b = y * 0.93_f32 - x.cos();
        let mut acc = Accumulator976::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.332_f32 + y.sin();
        let b = y * 5.475_f32 - x.cos();
        let mut acc = Accumulator976::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.843_f32 + y.sin();
        let b = y * 3.652_f32 - x.cos();
        let mut acc = Accumulator976::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.361_f32 + y.sin();
        let b = y * 2.916_f32 - x.cos();
        let mut acc = Accumulator976::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.242_f32 + y.sin();
        let b = y * 6.333_f32 - x.cos();
        let mut acc = Accumulator976::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.747_f32 + y.sin();
        let b = y * 0.988_f32 - x.cos();
        let mut acc = Accumulator976::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.221_f32 + y.sin();
        let b = y * 1.184_f32 - x.cos();
        let mut acc = Accumulator976::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_976(seed: u64) -> u64 {
        let re = Regex::new(r"m976-(\d+)").unwrap();
        let hay = format!("m976-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_976() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_976(total as u64) % 997) as f32;
        total
    }
}

pub mod m977 {
    use super::*;

    pub struct Accumulator977<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator977<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.6_f32 + y.sin();
        let b = y * 9.154_f32 - x.cos();
        let mut acc = Accumulator977::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.493_f32 + y.sin();
        let b = y * 5.8_f32 - x.cos();
        let mut acc = Accumulator977::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.978_f32 + y.sin();
        let b = y * 6.17_f32 - x.cos();
        let mut acc = Accumulator977::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.274_f32 + y.sin();
        let b = y * 7.44_f32 - x.cos();
        let mut acc = Accumulator977::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.627_f32 + y.sin();
        let b = y * 2.509_f32 - x.cos();
        let mut acc = Accumulator977::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.592_f32 + y.sin();
        let b = y * 0.961_f32 - x.cos();
        let mut acc = Accumulator977::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.608_f32 + y.sin();
        let b = y * 4.441_f32 - x.cos();
        let mut acc = Accumulator977::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.934_f32 + y.sin();
        let b = y * 0.436_f32 - x.cos();
        let mut acc = Accumulator977::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.45_f32 + y.sin();
        let b = y * 6.526_f32 - x.cos();
        let mut acc = Accumulator977::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.501_f32 + y.sin();
        let b = y * 7.495_f32 - x.cos();
        let mut acc = Accumulator977::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.43_f32 + y.sin();
        let b = y * 3.563_f32 - x.cos();
        let mut acc = Accumulator977::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.93_f32 + y.sin();
        let b = y * 7.82_f32 - x.cos();
        let mut acc = Accumulator977::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.407_f32 + y.sin();
        let b = y * 8.128_f32 - x.cos();
        let mut acc = Accumulator977::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.85_f32 + y.sin();
        let b = y * 7.032_f32 - x.cos();
        let mut acc = Accumulator977::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.338_f32 + y.sin();
        let b = y * 0.222_f32 - x.cos();
        let mut acc = Accumulator977::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.184_f32 + y.sin();
        let b = y * 4.669_f32 - x.cos();
        let mut acc = Accumulator977::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.372_f32 + y.sin();
        let b = y * 8.657_f32 - x.cos();
        let mut acc = Accumulator977::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.773_f32 + y.sin();
        let b = y * 0.422_f32 - x.cos();
        let mut acc = Accumulator977::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.109_f32 + y.sin();
        let b = y * 1.83_f32 - x.cos();
        let mut acc = Accumulator977::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.283_f32 + y.sin();
        let b = y * 2.861_f32 - x.cos();
        let mut acc = Accumulator977::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_977(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_977() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_977(total as u64) % 997) as f32;
        total
    }
}

pub mod m978 {
    use super::*;

    pub struct Accumulator978<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator978<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.357_f32 + y.sin();
        let b = y * 7.28_f32 - x.cos();
        let mut acc = Accumulator978::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.565_f32 + y.sin();
        let b = y * 3.209_f32 - x.cos();
        let mut acc = Accumulator978::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.583_f32 + y.sin();
        let b = y * 4.249_f32 - x.cos();
        let mut acc = Accumulator978::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.054_f32 + y.sin();
        let b = y * 6.09_f32 - x.cos();
        let mut acc = Accumulator978::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.996_f32 + y.sin();
        let b = y * 4.921_f32 - x.cos();
        let mut acc = Accumulator978::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.372_f32 + y.sin();
        let b = y * 7.151_f32 - x.cos();
        let mut acc = Accumulator978::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.167_f32 + y.sin();
        let b = y * 8.868_f32 - x.cos();
        let mut acc = Accumulator978::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.18_f32 + y.sin();
        let b = y * 7.963_f32 - x.cos();
        let mut acc = Accumulator978::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.669_f32 + y.sin();
        let b = y * 5.544_f32 - x.cos();
        let mut acc = Accumulator978::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.058_f32 + y.sin();
        let b = y * 0.599_f32 - x.cos();
        let mut acc = Accumulator978::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.354_f32 + y.sin();
        let b = y * 4.134_f32 - x.cos();
        let mut acc = Accumulator978::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.984_f32 + y.sin();
        let b = y * 3.323_f32 - x.cos();
        let mut acc = Accumulator978::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.492_f32 + y.sin();
        let b = y * 5.388_f32 - x.cos();
        let mut acc = Accumulator978::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.348_f32 + y.sin();
        let b = y * 6.186_f32 - x.cos();
        let mut acc = Accumulator978::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.621_f32 + y.sin();
        let b = y * 8.252_f32 - x.cos();
        let mut acc = Accumulator978::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.712_f32 + y.sin();
        let b = y * 3.581_f32 - x.cos();
        let mut acc = Accumulator978::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.301_f32 + y.sin();
        let b = y * 5.723_f32 - x.cos();
        let mut acc = Accumulator978::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.529_f32 + y.sin();
        let b = y * 2.997_f32 - x.cos();
        let mut acc = Accumulator978::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.778_f32 + y.sin();
        let b = y * 1.973_f32 - x.cos();
        let mut acc = Accumulator978::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.908_f32 + y.sin();
        let b = y * 2.641_f32 - x.cos();
        let mut acc = Accumulator978::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_978(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(978u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_978() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_978(total as u64) % 997) as f32;
        total
    }
}

pub mod m979 {
    use super::*;

    pub struct Accumulator979<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator979<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.163_f32 + y.sin();
        let b = y * 2.73_f32 - x.cos();
        let mut acc = Accumulator979::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.453_f32 + y.sin();
        let b = y * 0.49_f32 - x.cos();
        let mut acc = Accumulator979::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.198_f32 + y.sin();
        let b = y * 5.035_f32 - x.cos();
        let mut acc = Accumulator979::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.091_f32 + y.sin();
        let b = y * 5.82_f32 - x.cos();
        let mut acc = Accumulator979::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.558_f32 + y.sin();
        let b = y * 2.753_f32 - x.cos();
        let mut acc = Accumulator979::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.543_f32 + y.sin();
        let b = y * 9.716_f32 - x.cos();
        let mut acc = Accumulator979::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.345_f32 + y.sin();
        let b = y * 4.919_f32 - x.cos();
        let mut acc = Accumulator979::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.213_f32 + y.sin();
        let b = y * 8.804_f32 - x.cos();
        let mut acc = Accumulator979::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.074_f32 + y.sin();
        let b = y * 4.675_f32 - x.cos();
        let mut acc = Accumulator979::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.548_f32 + y.sin();
        let b = y * 0.183_f32 - x.cos();
        let mut acc = Accumulator979::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.199_f32 + y.sin();
        let b = y * 0.208_f32 - x.cos();
        let mut acc = Accumulator979::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.655_f32 + y.sin();
        let b = y * 1.279_f32 - x.cos();
        let mut acc = Accumulator979::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.021_f32 + y.sin();
        let b = y * 5.248_f32 - x.cos();
        let mut acc = Accumulator979::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.412_f32 + y.sin();
        let b = y * 4.648_f32 - x.cos();
        let mut acc = Accumulator979::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.598_f32 + y.sin();
        let b = y * 2.697_f32 - x.cos();
        let mut acc = Accumulator979::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.976_f32 + y.sin();
        let b = y * 7.238_f32 - x.cos();
        let mut acc = Accumulator979::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.753_f32 + y.sin();
        let b = y * 2.285_f32 - x.cos();
        let mut acc = Accumulator979::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.845_f32 + y.sin();
        let b = y * 3.638_f32 - x.cos();
        let mut acc = Accumulator979::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.641_f32 + y.sin();
        let b = y * 1.68_f32 - x.cos();
        let mut acc = Accumulator979::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.101_f32 + y.sin();
        let b = y * 6.234_f32 - x.cos();
        let mut acc = Accumulator979::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_979(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_979() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_979(total as u64) % 997) as f32;
        total
    }
}

pub mod m980 {
    use super::*;

    pub struct Accumulator980<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator980<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.136_f32 + y.sin();
        let b = y * 4.686_f32 - x.cos();
        let mut acc = Accumulator980::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.511_f32 + y.sin();
        let b = y * 2.318_f32 - x.cos();
        let mut acc = Accumulator980::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.565_f32 + y.sin();
        let b = y * 7.554_f32 - x.cos();
        let mut acc = Accumulator980::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.96_f32 + y.sin();
        let b = y * 2.658_f32 - x.cos();
        let mut acc = Accumulator980::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.43_f32 + y.sin();
        let b = y * 7.861_f32 - x.cos();
        let mut acc = Accumulator980::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.091_f32 + y.sin();
        let b = y * 2.888_f32 - x.cos();
        let mut acc = Accumulator980::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.7_f32 + y.sin();
        let b = y * 9.119_f32 - x.cos();
        let mut acc = Accumulator980::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.267_f32 + y.sin();
        let b = y * 4.908_f32 - x.cos();
        let mut acc = Accumulator980::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.16_f32 + y.sin();
        let b = y * 3.651_f32 - x.cos();
        let mut acc = Accumulator980::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.647_f32 + y.sin();
        let b = y * 7.078_f32 - x.cos();
        let mut acc = Accumulator980::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.766_f32 + y.sin();
        let b = y * 6.733_f32 - x.cos();
        let mut acc = Accumulator980::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.023_f32 + y.sin();
        let b = y * 1.072_f32 - x.cos();
        let mut acc = Accumulator980::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.853_f32 + y.sin();
        let b = y * 7.876_f32 - x.cos();
        let mut acc = Accumulator980::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.62_f32 + y.sin();
        let b = y * 9.084_f32 - x.cos();
        let mut acc = Accumulator980::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.072_f32 + y.sin();
        let b = y * 4.282_f32 - x.cos();
        let mut acc = Accumulator980::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.757_f32 + y.sin();
        let b = y * 3.704_f32 - x.cos();
        let mut acc = Accumulator980::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.157_f32 + y.sin();
        let b = y * 4.138_f32 - x.cos();
        let mut acc = Accumulator980::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.717_f32 + y.sin();
        let b = y * 4.362_f32 - x.cos();
        let mut acc = Accumulator980::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.361_f32 + y.sin();
        let b = y * 8.385_f32 - x.cos();
        let mut acc = Accumulator980::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.568_f32 + y.sin();
        let b = y * 0.455_f32 - x.cos();
        let mut acc = Accumulator980::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_980(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_980() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_980(total as u64) % 997) as f32;
        total
    }
}

pub mod m981 {
    use super::*;

    pub struct Accumulator981<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator981<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.142_f32 + y.sin();
        let b = y * 7.597_f32 - x.cos();
        let mut acc = Accumulator981::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.566_f32 + y.sin();
        let b = y * 7.574_f32 - x.cos();
        let mut acc = Accumulator981::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.634_f32 + y.sin();
        let b = y * 7.22_f32 - x.cos();
        let mut acc = Accumulator981::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.338_f32 + y.sin();
        let b = y * 7.446_f32 - x.cos();
        let mut acc = Accumulator981::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.016_f32 + y.sin();
        let b = y * 8.75_f32 - x.cos();
        let mut acc = Accumulator981::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.963_f32 + y.sin();
        let b = y * 8.749_f32 - x.cos();
        let mut acc = Accumulator981::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.7_f32 + y.sin();
        let b = y * 1.865_f32 - x.cos();
        let mut acc = Accumulator981::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.932_f32 + y.sin();
        let b = y * 3.165_f32 - x.cos();
        let mut acc = Accumulator981::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.855_f32 + y.sin();
        let b = y * 1.421_f32 - x.cos();
        let mut acc = Accumulator981::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.823_f32 + y.sin();
        let b = y * 3.255_f32 - x.cos();
        let mut acc = Accumulator981::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.12_f32 + y.sin();
        let b = y * 7.035_f32 - x.cos();
        let mut acc = Accumulator981::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.521_f32 + y.sin();
        let b = y * 3.71_f32 - x.cos();
        let mut acc = Accumulator981::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.779_f32 + y.sin();
        let b = y * 9.656_f32 - x.cos();
        let mut acc = Accumulator981::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.973_f32 + y.sin();
        let b = y * 7.794_f32 - x.cos();
        let mut acc = Accumulator981::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.75_f32 + y.sin();
        let b = y * 1.698_f32 - x.cos();
        let mut acc = Accumulator981::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.983_f32 + y.sin();
        let b = y * 6.608_f32 - x.cos();
        let mut acc = Accumulator981::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.146_f32 + y.sin();
        let b = y * 8.531_f32 - x.cos();
        let mut acc = Accumulator981::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.689_f32 + y.sin();
        let b = y * 0.802_f32 - x.cos();
        let mut acc = Accumulator981::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.452_f32 + y.sin();
        let b = y * 8.335_f32 - x.cos();
        let mut acc = Accumulator981::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.158_f32 + y.sin();
        let b = y * 8.063_f32 - x.cos();
        let mut acc = Accumulator981::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_981(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m981-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_981() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_981(total as u64) % 997) as f32;
        total
    }
}

pub mod m982 {
    use super::*;

    pub struct Accumulator982<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator982<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.548_f32 + y.sin();
        let b = y * 0.137_f32 - x.cos();
        let mut acc = Accumulator982::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.45_f32 + y.sin();
        let b = y * 8.646_f32 - x.cos();
        let mut acc = Accumulator982::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.802_f32 + y.sin();
        let b = y * 4.03_f32 - x.cos();
        let mut acc = Accumulator982::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.792_f32 + y.sin();
        let b = y * 5.305_f32 - x.cos();
        let mut acc = Accumulator982::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.864_f32 + y.sin();
        let b = y * 7.808_f32 - x.cos();
        let mut acc = Accumulator982::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.191_f32 + y.sin();
        let b = y * 6.337_f32 - x.cos();
        let mut acc = Accumulator982::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.363_f32 + y.sin();
        let b = y * 6.941_f32 - x.cos();
        let mut acc = Accumulator982::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.097_f32 + y.sin();
        let b = y * 6.482_f32 - x.cos();
        let mut acc = Accumulator982::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.684_f32 + y.sin();
        let b = y * 1.315_f32 - x.cos();
        let mut acc = Accumulator982::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.949_f32 + y.sin();
        let b = y * 7.265_f32 - x.cos();
        let mut acc = Accumulator982::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.409_f32 + y.sin();
        let b = y * 2.843_f32 - x.cos();
        let mut acc = Accumulator982::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.451_f32 + y.sin();
        let b = y * 3.165_f32 - x.cos();
        let mut acc = Accumulator982::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.435_f32 + y.sin();
        let b = y * 3.717_f32 - x.cos();
        let mut acc = Accumulator982::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.851_f32 + y.sin();
        let b = y * 3.266_f32 - x.cos();
        let mut acc = Accumulator982::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.407_f32 + y.sin();
        let b = y * 6.785_f32 - x.cos();
        let mut acc = Accumulator982::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.771_f32 + y.sin();
        let b = y * 8.032_f32 - x.cos();
        let mut acc = Accumulator982::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.582_f32 + y.sin();
        let b = y * 6.447_f32 - x.cos();
        let mut acc = Accumulator982::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.943_f32 + y.sin();
        let b = y * 2.943_f32 - x.cos();
        let mut acc = Accumulator982::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.303_f32 + y.sin();
        let b = y * 0.176_f32 - x.cos();
        let mut acc = Accumulator982::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.898_f32 + y.sin();
        let b = y * 4.331_f32 - x.cos();
        let mut acc = Accumulator982::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_982(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_982() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_982(total as u64) % 997) as f32;
        total
    }
}

pub mod m983 {
    use super::*;

    pub struct Accumulator983<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator983<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.311_f32 + y.sin();
        let b = y * 1.179_f32 - x.cos();
        let mut acc = Accumulator983::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.211_f32 + y.sin();
        let b = y * 7.144_f32 - x.cos();
        let mut acc = Accumulator983::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.515_f32 + y.sin();
        let b = y * 0.784_f32 - x.cos();
        let mut acc = Accumulator983::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.49_f32 + y.sin();
        let b = y * 4.079_f32 - x.cos();
        let mut acc = Accumulator983::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.76_f32 + y.sin();
        let b = y * 4.588_f32 - x.cos();
        let mut acc = Accumulator983::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.512_f32 + y.sin();
        let b = y * 6.747_f32 - x.cos();
        let mut acc = Accumulator983::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.001_f32 + y.sin();
        let b = y * 2.646_f32 - x.cos();
        let mut acc = Accumulator983::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.784_f32 + y.sin();
        let b = y * 4.31_f32 - x.cos();
        let mut acc = Accumulator983::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.641_f32 + y.sin();
        let b = y * 4.095_f32 - x.cos();
        let mut acc = Accumulator983::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.28_f32 + y.sin();
        let b = y * 0.879_f32 - x.cos();
        let mut acc = Accumulator983::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.98_f32 + y.sin();
        let b = y * 7.307_f32 - x.cos();
        let mut acc = Accumulator983::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.149_f32 + y.sin();
        let b = y * 0.109_f32 - x.cos();
        let mut acc = Accumulator983::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.811_f32 + y.sin();
        let b = y * 9.311_f32 - x.cos();
        let mut acc = Accumulator983::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.906_f32 + y.sin();
        let b = y * 2.229_f32 - x.cos();
        let mut acc = Accumulator983::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.791_f32 + y.sin();
        let b = y * 1.312_f32 - x.cos();
        let mut acc = Accumulator983::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.723_f32 + y.sin();
        let b = y * 8.745_f32 - x.cos();
        let mut acc = Accumulator983::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.209_f32 + y.sin();
        let b = y * 1.609_f32 - x.cos();
        let mut acc = Accumulator983::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.792_f32 + y.sin();
        let b = y * 0.721_f32 - x.cos();
        let mut acc = Accumulator983::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.364_f32 + y.sin();
        let b = y * 1.243_f32 - x.cos();
        let mut acc = Accumulator983::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.365_f32 + y.sin();
        let b = y * 7.681_f32 - x.cos();
        let mut acc = Accumulator983::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_983(seed: u64) -> u64 {
        let re = Regex::new(r"m983-(\d+)").unwrap();
        let hay = format!("m983-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_983() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_983(total as u64) % 997) as f32;
        total
    }
}

pub mod m984 {
    use super::*;

    pub struct Accumulator984<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator984<T> {
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
        let b = y * 4.632_f32 - x.cos();
        let mut acc = Accumulator984::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.029_f32 + y.sin();
        let b = y * 3.575_f32 - x.cos();
        let mut acc = Accumulator984::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.264_f32 + y.sin();
        let b = y * 3.602_f32 - x.cos();
        let mut acc = Accumulator984::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.856_f32 + y.sin();
        let b = y * 6.796_f32 - x.cos();
        let mut acc = Accumulator984::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.204_f32 + y.sin();
        let b = y * 6.497_f32 - x.cos();
        let mut acc = Accumulator984::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.161_f32 + y.sin();
        let b = y * 1.889_f32 - x.cos();
        let mut acc = Accumulator984::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.686_f32 + y.sin();
        let b = y * 7.318_f32 - x.cos();
        let mut acc = Accumulator984::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.521_f32 + y.sin();
        let b = y * 1.791_f32 - x.cos();
        let mut acc = Accumulator984::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.026_f32 + y.sin();
        let b = y * 5.461_f32 - x.cos();
        let mut acc = Accumulator984::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.76_f32 + y.sin();
        let b = y * 2.772_f32 - x.cos();
        let mut acc = Accumulator984::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.002_f32 + y.sin();
        let b = y * 5.928_f32 - x.cos();
        let mut acc = Accumulator984::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.89_f32 + y.sin();
        let b = y * 4.737_f32 - x.cos();
        let mut acc = Accumulator984::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.435_f32 + y.sin();
        let b = y * 7.887_f32 - x.cos();
        let mut acc = Accumulator984::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.66_f32 + y.sin();
        let b = y * 5.97_f32 - x.cos();
        let mut acc = Accumulator984::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.561_f32 + y.sin();
        let b = y * 7.607_f32 - x.cos();
        let mut acc = Accumulator984::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.685_f32 + y.sin();
        let b = y * 5.8_f32 - x.cos();
        let mut acc = Accumulator984::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.126_f32 + y.sin();
        let b = y * 9.846_f32 - x.cos();
        let mut acc = Accumulator984::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.34_f32 + y.sin();
        let b = y * 6.653_f32 - x.cos();
        let mut acc = Accumulator984::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.043_f32 + y.sin();
        let b = y * 2.406_f32 - x.cos();
        let mut acc = Accumulator984::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.735_f32 + y.sin();
        let b = y * 9.267_f32 - x.cos();
        let mut acc = Accumulator984::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_984(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_984() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_984(total as u64) % 997) as f32;
        total
    }
}

pub mod m985 {
    use super::*;

    pub struct Accumulator985<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator985<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.071_f32 + y.sin();
        let b = y * 9.815_f32 - x.cos();
        let mut acc = Accumulator985::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.725_f32 + y.sin();
        let b = y * 7.111_f32 - x.cos();
        let mut acc = Accumulator985::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.098_f32 + y.sin();
        let b = y * 4.866_f32 - x.cos();
        let mut acc = Accumulator985::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.145_f32 + y.sin();
        let b = y * 3.879_f32 - x.cos();
        let mut acc = Accumulator985::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.519_f32 + y.sin();
        let b = y * 7.168_f32 - x.cos();
        let mut acc = Accumulator985::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.722_f32 + y.sin();
        let b = y * 9.721_f32 - x.cos();
        let mut acc = Accumulator985::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.061_f32 + y.sin();
        let b = y * 4.077_f32 - x.cos();
        let mut acc = Accumulator985::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.213_f32 + y.sin();
        let b = y * 1.795_f32 - x.cos();
        let mut acc = Accumulator985::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.483_f32 + y.sin();
        let b = y * 8.062_f32 - x.cos();
        let mut acc = Accumulator985::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.317_f32 + y.sin();
        let b = y * 6.742_f32 - x.cos();
        let mut acc = Accumulator985::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.539_f32 + y.sin();
        let b = y * 2.493_f32 - x.cos();
        let mut acc = Accumulator985::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.313_f32 + y.sin();
        let b = y * 6.619_f32 - x.cos();
        let mut acc = Accumulator985::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.493_f32 + y.sin();
        let b = y * 4.912_f32 - x.cos();
        let mut acc = Accumulator985::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.711_f32 + y.sin();
        let b = y * 5.853_f32 - x.cos();
        let mut acc = Accumulator985::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.983_f32 + y.sin();
        let b = y * 6.463_f32 - x.cos();
        let mut acc = Accumulator985::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.974_f32 + y.sin();
        let b = y * 4.493_f32 - x.cos();
        let mut acc = Accumulator985::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.557_f32 + y.sin();
        let b = y * 2.642_f32 - x.cos();
        let mut acc = Accumulator985::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.789_f32 + y.sin();
        let b = y * 0.58_f32 - x.cos();
        let mut acc = Accumulator985::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.64_f32 + y.sin();
        let b = y * 2.061_f32 - x.cos();
        let mut acc = Accumulator985::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.63_f32 + y.sin();
        let b = y * 9.313_f32 - x.cos();
        let mut acc = Accumulator985::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_985(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(985u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_985() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_985(total as u64) % 997) as f32;
        total
    }
}

pub mod m986 {
    use super::*;

    pub struct Accumulator986<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator986<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.776_f32 + y.sin();
        let b = y * 8.036_f32 - x.cos();
        let mut acc = Accumulator986::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.504_f32 + y.sin();
        let b = y * 8.977_f32 - x.cos();
        let mut acc = Accumulator986::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.203_f32 + y.sin();
        let b = y * 3.0_f32 - x.cos();
        let mut acc = Accumulator986::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.481_f32 + y.sin();
        let b = y * 0.334_f32 - x.cos();
        let mut acc = Accumulator986::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.313_f32 + y.sin();
        let b = y * 1.72_f32 - x.cos();
        let mut acc = Accumulator986::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.586_f32 + y.sin();
        let b = y * 4.629_f32 - x.cos();
        let mut acc = Accumulator986::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.667_f32 + y.sin();
        let b = y * 7.662_f32 - x.cos();
        let mut acc = Accumulator986::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.629_f32 + y.sin();
        let b = y * 8.758_f32 - x.cos();
        let mut acc = Accumulator986::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.531_f32 + y.sin();
        let b = y * 0.495_f32 - x.cos();
        let mut acc = Accumulator986::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.968_f32 + y.sin();
        let b = y * 9.471_f32 - x.cos();
        let mut acc = Accumulator986::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.242_f32 + y.sin();
        let b = y * 4.926_f32 - x.cos();
        let mut acc = Accumulator986::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.084_f32 + y.sin();
        let b = y * 5.374_f32 - x.cos();
        let mut acc = Accumulator986::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.952_f32 + y.sin();
        let b = y * 3.188_f32 - x.cos();
        let mut acc = Accumulator986::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.54_f32 + y.sin();
        let b = y * 6.198_f32 - x.cos();
        let mut acc = Accumulator986::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.199_f32 + y.sin();
        let b = y * 0.94_f32 - x.cos();
        let mut acc = Accumulator986::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.879_f32 + y.sin();
        let b = y * 8.086_f32 - x.cos();
        let mut acc = Accumulator986::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.958_f32 + y.sin();
        let b = y * 0.195_f32 - x.cos();
        let mut acc = Accumulator986::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.029_f32 + y.sin();
        let b = y * 8.43_f32 - x.cos();
        let mut acc = Accumulator986::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.264_f32 + y.sin();
        let b = y * 7.993_f32 - x.cos();
        let mut acc = Accumulator986::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.83_f32 + y.sin();
        let b = y * 8.87_f32 - x.cos();
        let mut acc = Accumulator986::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_986(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_986() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_986(total as u64) % 997) as f32;
        total
    }
}

pub mod m987 {
    use super::*;

    pub struct Accumulator987<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator987<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.014_f32 + y.sin();
        let b = y * 8.217_f32 - x.cos();
        let mut acc = Accumulator987::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.596_f32 + y.sin();
        let b = y * 0.232_f32 - x.cos();
        let mut acc = Accumulator987::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.543_f32 + y.sin();
        let b = y * 3.43_f32 - x.cos();
        let mut acc = Accumulator987::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.697_f32 + y.sin();
        let b = y * 3.767_f32 - x.cos();
        let mut acc = Accumulator987::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.35_f32 + y.sin();
        let b = y * 7.968_f32 - x.cos();
        let mut acc = Accumulator987::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.62_f32 + y.sin();
        let b = y * 3.462_f32 - x.cos();
        let mut acc = Accumulator987::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.555_f32 + y.sin();
        let b = y * 9.696_f32 - x.cos();
        let mut acc = Accumulator987::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.774_f32 + y.sin();
        let b = y * 9.873_f32 - x.cos();
        let mut acc = Accumulator987::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.182_f32 + y.sin();
        let b = y * 1.391_f32 - x.cos();
        let mut acc = Accumulator987::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.379_f32 + y.sin();
        let b = y * 4.109_f32 - x.cos();
        let mut acc = Accumulator987::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.2_f32 + y.sin();
        let b = y * 1.595_f32 - x.cos();
        let mut acc = Accumulator987::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.106_f32 + y.sin();
        let b = y * 5.491_f32 - x.cos();
        let mut acc = Accumulator987::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.973_f32 + y.sin();
        let b = y * 4.17_f32 - x.cos();
        let mut acc = Accumulator987::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.245_f32 + y.sin();
        let b = y * 0.203_f32 - x.cos();
        let mut acc = Accumulator987::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.577_f32 + y.sin();
        let b = y * 0.632_f32 - x.cos();
        let mut acc = Accumulator987::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.955_f32 + y.sin();
        let b = y * 1.989_f32 - x.cos();
        let mut acc = Accumulator987::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.258_f32 + y.sin();
        let b = y * 7.266_f32 - x.cos();
        let mut acc = Accumulator987::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.847_f32 + y.sin();
        let b = y * 8.675_f32 - x.cos();
        let mut acc = Accumulator987::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.263_f32 + y.sin();
        let b = y * 2.912_f32 - x.cos();
        let mut acc = Accumulator987::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.609_f32 + y.sin();
        let b = y * 8.872_f32 - x.cos();
        let mut acc = Accumulator987::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_987(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_987() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_987(total as u64) % 997) as f32;
        total
    }
}

pub mod m988 {
    use super::*;

    pub struct Accumulator988<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator988<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.922_f32 + y.sin();
        let b = y * 5.957_f32 - x.cos();
        let mut acc = Accumulator988::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.008_f32 + y.sin();
        let b = y * 9.875_f32 - x.cos();
        let mut acc = Accumulator988::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.237_f32 + y.sin();
        let b = y * 3.887_f32 - x.cos();
        let mut acc = Accumulator988::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.139_f32 + y.sin();
        let b = y * 4.81_f32 - x.cos();
        let mut acc = Accumulator988::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.531_f32 + y.sin();
        let b = y * 2.264_f32 - x.cos();
        let mut acc = Accumulator988::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.135_f32 + y.sin();
        let b = y * 4.74_f32 - x.cos();
        let mut acc = Accumulator988::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.726_f32 + y.sin();
        let b = y * 6.443_f32 - x.cos();
        let mut acc = Accumulator988::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.518_f32 + y.sin();
        let b = y * 7.4_f32 - x.cos();
        let mut acc = Accumulator988::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.873_f32 + y.sin();
        let b = y * 2.345_f32 - x.cos();
        let mut acc = Accumulator988::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.166_f32 + y.sin();
        let b = y * 8.984_f32 - x.cos();
        let mut acc = Accumulator988::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.561_f32 + y.sin();
        let b = y * 9.082_f32 - x.cos();
        let mut acc = Accumulator988::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.883_f32 + y.sin();
        let b = y * 2.32_f32 - x.cos();
        let mut acc = Accumulator988::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.483_f32 + y.sin();
        let b = y * 3.355_f32 - x.cos();
        let mut acc = Accumulator988::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.822_f32 + y.sin();
        let b = y * 6.421_f32 - x.cos();
        let mut acc = Accumulator988::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.24_f32 + y.sin();
        let b = y * 8.559_f32 - x.cos();
        let mut acc = Accumulator988::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.716_f32 + y.sin();
        let b = y * 8.597_f32 - x.cos();
        let mut acc = Accumulator988::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.308_f32 + y.sin();
        let b = y * 4.566_f32 - x.cos();
        let mut acc = Accumulator988::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.076_f32 + y.sin();
        let b = y * 7.175_f32 - x.cos();
        let mut acc = Accumulator988::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.995_f32 + y.sin();
        let b = y * 9.154_f32 - x.cos();
        let mut acc = Accumulator988::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.059_f32 + y.sin();
        let b = y * 6.3_f32 - x.cos();
        let mut acc = Accumulator988::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_988(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m988-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_988() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_988(total as u64) % 997) as f32;
        total
    }
}

pub mod m989 {
    use super::*;

    pub struct Accumulator989<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator989<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.367_f32 + y.sin();
        let b = y * 3.822_f32 - x.cos();
        let mut acc = Accumulator989::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.268_f32 + y.sin();
        let b = y * 3.006_f32 - x.cos();
        let mut acc = Accumulator989::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.178_f32 + y.sin();
        let b = y * 1.793_f32 - x.cos();
        let mut acc = Accumulator989::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.64_f32 + y.sin();
        let b = y * 9.39_f32 - x.cos();
        let mut acc = Accumulator989::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.273_f32 + y.sin();
        let b = y * 1.828_f32 - x.cos();
        let mut acc = Accumulator989::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.963_f32 + y.sin();
        let b = y * 2.177_f32 - x.cos();
        let mut acc = Accumulator989::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.023_f32 + y.sin();
        let b = y * 6.134_f32 - x.cos();
        let mut acc = Accumulator989::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.15_f32 + y.sin();
        let b = y * 9.21_f32 - x.cos();
        let mut acc = Accumulator989::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.125_f32 + y.sin();
        let b = y * 5.348_f32 - x.cos();
        let mut acc = Accumulator989::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.933_f32 + y.sin();
        let b = y * 4.973_f32 - x.cos();
        let mut acc = Accumulator989::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.703_f32 + y.sin();
        let b = y * 1.301_f32 - x.cos();
        let mut acc = Accumulator989::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.017_f32 + y.sin();
        let b = y * 8.209_f32 - x.cos();
        let mut acc = Accumulator989::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.681_f32 + y.sin();
        let b = y * 2.737_f32 - x.cos();
        let mut acc = Accumulator989::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.475_f32 + y.sin();
        let b = y * 8.822_f32 - x.cos();
        let mut acc = Accumulator989::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.534_f32 + y.sin();
        let b = y * 6.449_f32 - x.cos();
        let mut acc = Accumulator989::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.353_f32 + y.sin();
        let b = y * 9.149_f32 - x.cos();
        let mut acc = Accumulator989::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.203_f32 + y.sin();
        let b = y * 1.951_f32 - x.cos();
        let mut acc = Accumulator989::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.185_f32 + y.sin();
        let b = y * 2.25_f32 - x.cos();
        let mut acc = Accumulator989::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.612_f32 + y.sin();
        let b = y * 4.959_f32 - x.cos();
        let mut acc = Accumulator989::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.1_f32 + y.sin();
        let b = y * 5.714_f32 - x.cos();
        let mut acc = Accumulator989::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_989(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_989() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_989(total as u64) % 997) as f32;
        total
    }
}

pub mod m990 {
    use super::*;

    pub struct Accumulator990<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator990<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.733_f32 + y.sin();
        let b = y * 2.358_f32 - x.cos();
        let mut acc = Accumulator990::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.515_f32 + y.sin();
        let b = y * 4.307_f32 - x.cos();
        let mut acc = Accumulator990::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.4_f32 + y.sin();
        let b = y * 4.286_f32 - x.cos();
        let mut acc = Accumulator990::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.313_f32 + y.sin();
        let b = y * 5.685_f32 - x.cos();
        let mut acc = Accumulator990::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.148_f32 + y.sin();
        let b = y * 1.025_f32 - x.cos();
        let mut acc = Accumulator990::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.635_f32 + y.sin();
        let b = y * 2.509_f32 - x.cos();
        let mut acc = Accumulator990::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.118_f32 + y.sin();
        let b = y * 0.986_f32 - x.cos();
        let mut acc = Accumulator990::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.782_f32 + y.sin();
        let b = y * 0.631_f32 - x.cos();
        let mut acc = Accumulator990::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.706_f32 + y.sin();
        let b = y * 1.599_f32 - x.cos();
        let mut acc = Accumulator990::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.055_f32 + y.sin();
        let b = y * 9.807_f32 - x.cos();
        let mut acc = Accumulator990::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.484_f32 + y.sin();
        let b = y * 6.712_f32 - x.cos();
        let mut acc = Accumulator990::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.928_f32 + y.sin();
        let b = y * 3.179_f32 - x.cos();
        let mut acc = Accumulator990::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.52_f32 + y.sin();
        let b = y * 5.204_f32 - x.cos();
        let mut acc = Accumulator990::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.833_f32 + y.sin();
        let b = y * 0.657_f32 - x.cos();
        let mut acc = Accumulator990::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.101_f32 + y.sin();
        let b = y * 4.157_f32 - x.cos();
        let mut acc = Accumulator990::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.843_f32 + y.sin();
        let b = y * 3.998_f32 - x.cos();
        let mut acc = Accumulator990::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.348_f32 + y.sin();
        let b = y * 5.387_f32 - x.cos();
        let mut acc = Accumulator990::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.237_f32 + y.sin();
        let b = y * 1.063_f32 - x.cos();
        let mut acc = Accumulator990::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.038_f32 + y.sin();
        let b = y * 3.61_f32 - x.cos();
        let mut acc = Accumulator990::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.7_f32 + y.sin();
        let b = y * 5.104_f32 - x.cos();
        let mut acc = Accumulator990::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_990(seed: u64) -> u64 {
        let re = Regex::new(r"m990-(\d+)").unwrap();
        let hay = format!("m990-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_990() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_990(total as u64) % 997) as f32;
        total
    }
}

pub mod m991 {
    use super::*;

    pub struct Accumulator991<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator991<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.901_f32 + y.sin();
        let b = y * 2.439_f32 - x.cos();
        let mut acc = Accumulator991::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.354_f32 + y.sin();
        let b = y * 9.405_f32 - x.cos();
        let mut acc = Accumulator991::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.783_f32 + y.sin();
        let b = y * 6.609_f32 - x.cos();
        let mut acc = Accumulator991::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.198_f32 + y.sin();
        let b = y * 1.6_f32 - x.cos();
        let mut acc = Accumulator991::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.97_f32 + y.sin();
        let b = y * 4.111_f32 - x.cos();
        let mut acc = Accumulator991::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.577_f32 + y.sin();
        let b = y * 9.895_f32 - x.cos();
        let mut acc = Accumulator991::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.311_f32 + y.sin();
        let b = y * 3.93_f32 - x.cos();
        let mut acc = Accumulator991::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.227_f32 + y.sin();
        let b = y * 9.004_f32 - x.cos();
        let mut acc = Accumulator991::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.789_f32 + y.sin();
        let b = y * 6.252_f32 - x.cos();
        let mut acc = Accumulator991::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.605_f32 + y.sin();
        let b = y * 3.444_f32 - x.cos();
        let mut acc = Accumulator991::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.445_f32 + y.sin();
        let b = y * 5.692_f32 - x.cos();
        let mut acc = Accumulator991::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.833_f32 + y.sin();
        let b = y * 2.21_f32 - x.cos();
        let mut acc = Accumulator991::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.4_f32 + y.sin();
        let b = y * 9.885_f32 - x.cos();
        let mut acc = Accumulator991::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.155_f32 + y.sin();
        let b = y * 6.427_f32 - x.cos();
        let mut acc = Accumulator991::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.341_f32 + y.sin();
        let b = y * 2.756_f32 - x.cos();
        let mut acc = Accumulator991::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.509_f32 + y.sin();
        let b = y * 1.431_f32 - x.cos();
        let mut acc = Accumulator991::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.448_f32 + y.sin();
        let b = y * 6.442_f32 - x.cos();
        let mut acc = Accumulator991::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.087_f32 + y.sin();
        let b = y * 9.066_f32 - x.cos();
        let mut acc = Accumulator991::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.063_f32 + y.sin();
        let b = y * 3.086_f32 - x.cos();
        let mut acc = Accumulator991::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.372_f32 + y.sin();
        let b = y * 4.11_f32 - x.cos();
        let mut acc = Accumulator991::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_991(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_991() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_991(total as u64) % 997) as f32;
        total
    }
}

pub mod m992 {
    use super::*;

    pub struct Accumulator992<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator992<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.857_f32 + y.sin();
        let b = y * 3.635_f32 - x.cos();
        let mut acc = Accumulator992::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.402_f32 + y.sin();
        let b = y * 8.475_f32 - x.cos();
        let mut acc = Accumulator992::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.242_f32 + y.sin();
        let b = y * 9.44_f32 - x.cos();
        let mut acc = Accumulator992::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.544_f32 + y.sin();
        let b = y * 8.899_f32 - x.cos();
        let mut acc = Accumulator992::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.005_f32 + y.sin();
        let b = y * 4.642_f32 - x.cos();
        let mut acc = Accumulator992::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.045_f32 + y.sin();
        let b = y * 8.545_f32 - x.cos();
        let mut acc = Accumulator992::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.322_f32 + y.sin();
        let b = y * 7.414_f32 - x.cos();
        let mut acc = Accumulator992::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.789_f32 + y.sin();
        let b = y * 5.16_f32 - x.cos();
        let mut acc = Accumulator992::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.231_f32 + y.sin();
        let b = y * 7.448_f32 - x.cos();
        let mut acc = Accumulator992::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.787_f32 + y.sin();
        let b = y * 0.62_f32 - x.cos();
        let mut acc = Accumulator992::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.784_f32 + y.sin();
        let b = y * 7.705_f32 - x.cos();
        let mut acc = Accumulator992::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.114_f32 + y.sin();
        let b = y * 2.742_f32 - x.cos();
        let mut acc = Accumulator992::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.423_f32 + y.sin();
        let b = y * 2.96_f32 - x.cos();
        let mut acc = Accumulator992::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.297_f32 + y.sin();
        let b = y * 5.412_f32 - x.cos();
        let mut acc = Accumulator992::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.424_f32 + y.sin();
        let b = y * 6.683_f32 - x.cos();
        let mut acc = Accumulator992::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.634_f32 + y.sin();
        let b = y * 0.523_f32 - x.cos();
        let mut acc = Accumulator992::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.95_f32 + y.sin();
        let b = y * 4.471_f32 - x.cos();
        let mut acc = Accumulator992::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.471_f32 + y.sin();
        let b = y * 4.697_f32 - x.cos();
        let mut acc = Accumulator992::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.995_f32 + y.sin();
        let b = y * 3.501_f32 - x.cos();
        let mut acc = Accumulator992::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.054_f32 + y.sin();
        let b = y * 7.355_f32 - x.cos();
        let mut acc = Accumulator992::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_992(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(992u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_992() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_992(total as u64) % 997) as f32;
        total
    }
}

pub mod m993 {
    use super::*;

    pub struct Accumulator993<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator993<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.869_f32 + y.sin();
        let b = y * 9.432_f32 - x.cos();
        let mut acc = Accumulator993::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.948_f32 + y.sin();
        let b = y * 5.96_f32 - x.cos();
        let mut acc = Accumulator993::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.376_f32 + y.sin();
        let b = y * 9.722_f32 - x.cos();
        let mut acc = Accumulator993::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.392_f32 + y.sin();
        let b = y * 0.926_f32 - x.cos();
        let mut acc = Accumulator993::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.715_f32 + y.sin();
        let b = y * 7.952_f32 - x.cos();
        let mut acc = Accumulator993::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.219_f32 + y.sin();
        let b = y * 8.148_f32 - x.cos();
        let mut acc = Accumulator993::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.359_f32 + y.sin();
        let b = y * 4.592_f32 - x.cos();
        let mut acc = Accumulator993::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.256_f32 + y.sin();
        let b = y * 1.123_f32 - x.cos();
        let mut acc = Accumulator993::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.002_f32 + y.sin();
        let b = y * 4.132_f32 - x.cos();
        let mut acc = Accumulator993::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.018_f32 + y.sin();
        let b = y * 9.56_f32 - x.cos();
        let mut acc = Accumulator993::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.86_f32 + y.sin();
        let b = y * 7.158_f32 - x.cos();
        let mut acc = Accumulator993::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.768_f32 + y.sin();
        let b = y * 7.35_f32 - x.cos();
        let mut acc = Accumulator993::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.677_f32 + y.sin();
        let b = y * 8.437_f32 - x.cos();
        let mut acc = Accumulator993::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.76_f32 + y.sin();
        let b = y * 7.307_f32 - x.cos();
        let mut acc = Accumulator993::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.23_f32 + y.sin();
        let b = y * 0.702_f32 - x.cos();
        let mut acc = Accumulator993::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.083_f32 + y.sin();
        let b = y * 3.929_f32 - x.cos();
        let mut acc = Accumulator993::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.426_f32 + y.sin();
        let b = y * 7.076_f32 - x.cos();
        let mut acc = Accumulator993::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.173_f32 + y.sin();
        let b = y * 4.258_f32 - x.cos();
        let mut acc = Accumulator993::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.267_f32 + y.sin();
        let b = y * 8.899_f32 - x.cos();
        let mut acc = Accumulator993::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.206_f32 + y.sin();
        let b = y * 2.519_f32 - x.cos();
        let mut acc = Accumulator993::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_993(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_993() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_993(total as u64) % 997) as f32;
        total
    }
}

pub mod m994 {
    use super::*;

    pub struct Accumulator994<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator994<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.101_f32 + y.sin();
        let b = y * 6.022_f32 - x.cos();
        let mut acc = Accumulator994::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.406_f32 + y.sin();
        let b = y * 6.412_f32 - x.cos();
        let mut acc = Accumulator994::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.055_f32 + y.sin();
        let b = y * 1.383_f32 - x.cos();
        let mut acc = Accumulator994::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.071_f32 + y.sin();
        let b = y * 5.691_f32 - x.cos();
        let mut acc = Accumulator994::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.538_f32 + y.sin();
        let b = y * 3.789_f32 - x.cos();
        let mut acc = Accumulator994::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.295_f32 + y.sin();
        let b = y * 0.415_f32 - x.cos();
        let mut acc = Accumulator994::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.457_f32 + y.sin();
        let b = y * 8.799_f32 - x.cos();
        let mut acc = Accumulator994::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.171_f32 + y.sin();
        let b = y * 2.51_f32 - x.cos();
        let mut acc = Accumulator994::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.24_f32 + y.sin();
        let b = y * 8.558_f32 - x.cos();
        let mut acc = Accumulator994::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.974_f32 + y.sin();
        let b = y * 2.41_f32 - x.cos();
        let mut acc = Accumulator994::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.936_f32 + y.sin();
        let b = y * 1.06_f32 - x.cos();
        let mut acc = Accumulator994::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.31_f32 + y.sin();
        let b = y * 5.201_f32 - x.cos();
        let mut acc = Accumulator994::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.494_f32 + y.sin();
        let b = y * 7.786_f32 - x.cos();
        let mut acc = Accumulator994::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.063_f32 + y.sin();
        let b = y * 3.896_f32 - x.cos();
        let mut acc = Accumulator994::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.256_f32 + y.sin();
        let b = y * 5.474_f32 - x.cos();
        let mut acc = Accumulator994::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.321_f32 + y.sin();
        let b = y * 8.004_f32 - x.cos();
        let mut acc = Accumulator994::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.53_f32 + y.sin();
        let b = y * 4.772_f32 - x.cos();
        let mut acc = Accumulator994::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.52_f32 + y.sin();
        let b = y * 4.779_f32 - x.cos();
        let mut acc = Accumulator994::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.974_f32 + y.sin();
        let b = y * 8.504_f32 - x.cos();
        let mut acc = Accumulator994::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.396_f32 + y.sin();
        let b = y * 6.71_f32 - x.cos();
        let mut acc = Accumulator994::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_994(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_994() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_994(total as u64) % 997) as f32;
        total
    }
}

pub mod m995 {
    use super::*;

    pub struct Accumulator995<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator995<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.097_f32 + y.sin();
        let b = y * 8.441_f32 - x.cos();
        let mut acc = Accumulator995::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.349_f32 + y.sin();
        let b = y * 9.274_f32 - x.cos();
        let mut acc = Accumulator995::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.835_f32 + y.sin();
        let b = y * 1.749_f32 - x.cos();
        let mut acc = Accumulator995::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.121_f32 + y.sin();
        let b = y * 0.436_f32 - x.cos();
        let mut acc = Accumulator995::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.491_f32 + y.sin();
        let b = y * 7.622_f32 - x.cos();
        let mut acc = Accumulator995::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.14_f32 + y.sin();
        let b = y * 2.26_f32 - x.cos();
        let mut acc = Accumulator995::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.187_f32 + y.sin();
        let b = y * 6.529_f32 - x.cos();
        let mut acc = Accumulator995::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.197_f32 + y.sin();
        let b = y * 1.949_f32 - x.cos();
        let mut acc = Accumulator995::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.462_f32 + y.sin();
        let b = y * 2.932_f32 - x.cos();
        let mut acc = Accumulator995::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.615_f32 + y.sin();
        let b = y * 6.193_f32 - x.cos();
        let mut acc = Accumulator995::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.729_f32 + y.sin();
        let b = y * 9.417_f32 - x.cos();
        let mut acc = Accumulator995::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.273_f32 + y.sin();
        let b = y * 7.701_f32 - x.cos();
        let mut acc = Accumulator995::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.285_f32 + y.sin();
        let b = y * 8.451_f32 - x.cos();
        let mut acc = Accumulator995::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.253_f32 + y.sin();
        let b = y * 0.604_f32 - x.cos();
        let mut acc = Accumulator995::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.39_f32 + y.sin();
        let b = y * 4.371_f32 - x.cos();
        let mut acc = Accumulator995::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.28_f32 + y.sin();
        let b = y * 2.795_f32 - x.cos();
        let mut acc = Accumulator995::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.606_f32 + y.sin();
        let b = y * 4.508_f32 - x.cos();
        let mut acc = Accumulator995::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.097_f32 + y.sin();
        let b = y * 4.581_f32 - x.cos();
        let mut acc = Accumulator995::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.5_f32 + y.sin();
        let b = y * 5.97_f32 - x.cos();
        let mut acc = Accumulator995::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.832_f32 + y.sin();
        let b = y * 2.11_f32 - x.cos();
        let mut acc = Accumulator995::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_995(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m995-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_995() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_995(total as u64) % 997) as f32;
        total
    }
}

pub mod m996 {
    use super::*;

    pub struct Accumulator996<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator996<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.158_f32 + y.sin();
        let b = y * 8.913_f32 - x.cos();
        let mut acc = Accumulator996::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.959_f32 + y.sin();
        let b = y * 4.502_f32 - x.cos();
        let mut acc = Accumulator996::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.268_f32 + y.sin();
        let b = y * 5.918_f32 - x.cos();
        let mut acc = Accumulator996::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.322_f32 + y.sin();
        let b = y * 5.376_f32 - x.cos();
        let mut acc = Accumulator996::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.344_f32 + y.sin();
        let b = y * 2.467_f32 - x.cos();
        let mut acc = Accumulator996::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.8_f32 + y.sin();
        let b = y * 7.354_f32 - x.cos();
        let mut acc = Accumulator996::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.63_f32 + y.sin();
        let b = y * 0.949_f32 - x.cos();
        let mut acc = Accumulator996::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.829_f32 + y.sin();
        let b = y * 4.819_f32 - x.cos();
        let mut acc = Accumulator996::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.594_f32 + y.sin();
        let b = y * 1.301_f32 - x.cos();
        let mut acc = Accumulator996::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.598_f32 + y.sin();
        let b = y * 5.411_f32 - x.cos();
        let mut acc = Accumulator996::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.927_f32 + y.sin();
        let b = y * 5.467_f32 - x.cos();
        let mut acc = Accumulator996::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.83_f32 + y.sin();
        let b = y * 4.879_f32 - x.cos();
        let mut acc = Accumulator996::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.443_f32 + y.sin();
        let b = y * 5.042_f32 - x.cos();
        let mut acc = Accumulator996::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.872_f32 + y.sin();
        let b = y * 7.617_f32 - x.cos();
        let mut acc = Accumulator996::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.762_f32 + y.sin();
        let b = y * 0.369_f32 - x.cos();
        let mut acc = Accumulator996::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.487_f32 + y.sin();
        let b = y * 1.166_f32 - x.cos();
        let mut acc = Accumulator996::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.296_f32 + y.sin();
        let b = y * 3.042_f32 - x.cos();
        let mut acc = Accumulator996::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.438_f32 + y.sin();
        let b = y * 9.774_f32 - x.cos();
        let mut acc = Accumulator996::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.942_f32 + y.sin();
        let b = y * 9.705_f32 - x.cos();
        let mut acc = Accumulator996::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.777_f32 + y.sin();
        let b = y * 4.703_f32 - x.cos();
        let mut acc = Accumulator996::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_996(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_996() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_996(total as u64) % 997) as f32;
        total
    }
}

pub mod m997 {
    use super::*;

    pub struct Accumulator997<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator997<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.317_f32 + y.sin();
        let b = y * 1.048_f32 - x.cos();
        let mut acc = Accumulator997::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.331_f32 + y.sin();
        let b = y * 1.022_f32 - x.cos();
        let mut acc = Accumulator997::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.882_f32 + y.sin();
        let b = y * 3.2_f32 - x.cos();
        let mut acc = Accumulator997::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.019_f32 + y.sin();
        let b = y * 1.181_f32 - x.cos();
        let mut acc = Accumulator997::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.945_f32 + y.sin();
        let b = y * 0.287_f32 - x.cos();
        let mut acc = Accumulator997::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.877_f32 + y.sin();
        let b = y * 5.973_f32 - x.cos();
        let mut acc = Accumulator997::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.22_f32 + y.sin();
        let b = y * 3.872_f32 - x.cos();
        let mut acc = Accumulator997::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.985_f32 + y.sin();
        let b = y * 0.665_f32 - x.cos();
        let mut acc = Accumulator997::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.112_f32 + y.sin();
        let b = y * 5.38_f32 - x.cos();
        let mut acc = Accumulator997::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.208_f32 + y.sin();
        let b = y * 9.519_f32 - x.cos();
        let mut acc = Accumulator997::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.262_f32 + y.sin();
        let b = y * 3.92_f32 - x.cos();
        let mut acc = Accumulator997::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.887_f32 + y.sin();
        let b = y * 2.185_f32 - x.cos();
        let mut acc = Accumulator997::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.923_f32 + y.sin();
        let b = y * 2.485_f32 - x.cos();
        let mut acc = Accumulator997::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.639_f32 + y.sin();
        let b = y * 4.01_f32 - x.cos();
        let mut acc = Accumulator997::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.224_f32 + y.sin();
        let b = y * 2.806_f32 - x.cos();
        let mut acc = Accumulator997::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.055_f32 + y.sin();
        let b = y * 7.134_f32 - x.cos();
        let mut acc = Accumulator997::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.631_f32 + y.sin();
        let b = y * 2.403_f32 - x.cos();
        let mut acc = Accumulator997::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.264_f32 + y.sin();
        let b = y * 1.498_f32 - x.cos();
        let mut acc = Accumulator997::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.998_f32 + y.sin();
        let b = y * 7.141_f32 - x.cos();
        let mut acc = Accumulator997::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.367_f32 + y.sin();
        let b = y * 3.854_f32 - x.cos();
        let mut acc = Accumulator997::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_997(seed: u64) -> u64 {
        let re = Regex::new(r"m997-(\d+)").unwrap();
        let hay = format!("m997-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_997() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_997(total as u64) % 997) as f32;
        total
    }
}

pub mod m998 {
    use super::*;

    pub struct Accumulator998<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator998<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.254_f32 + y.sin();
        let b = y * 8.108_f32 - x.cos();
        let mut acc = Accumulator998::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.228_f32 + y.sin();
        let b = y * 8.821_f32 - x.cos();
        let mut acc = Accumulator998::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.6_f32 + y.sin();
        let b = y * 8.093_f32 - x.cos();
        let mut acc = Accumulator998::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.44_f32 + y.sin();
        let b = y * 1.763_f32 - x.cos();
        let mut acc = Accumulator998::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.621_f32 + y.sin();
        let b = y * 8.294_f32 - x.cos();
        let mut acc = Accumulator998::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.858_f32 + y.sin();
        let b = y * 7.5_f32 - x.cos();
        let mut acc = Accumulator998::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.107_f32 + y.sin();
        let b = y * 1.062_f32 - x.cos();
        let mut acc = Accumulator998::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.307_f32 + y.sin();
        let b = y * 9.345_f32 - x.cos();
        let mut acc = Accumulator998::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.634_f32 + y.sin();
        let b = y * 8.814_f32 - x.cos();
        let mut acc = Accumulator998::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.294_f32 + y.sin();
        let b = y * 5.797_f32 - x.cos();
        let mut acc = Accumulator998::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.081_f32 + y.sin();
        let b = y * 5.372_f32 - x.cos();
        let mut acc = Accumulator998::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.191_f32 + y.sin();
        let b = y * 7.8_f32 - x.cos();
        let mut acc = Accumulator998::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.748_f32 + y.sin();
        let b = y * 7.576_f32 - x.cos();
        let mut acc = Accumulator998::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.249_f32 + y.sin();
        let b = y * 4.277_f32 - x.cos();
        let mut acc = Accumulator998::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.607_f32 + y.sin();
        let b = y * 8.394_f32 - x.cos();
        let mut acc = Accumulator998::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.048_f32 + y.sin();
        let b = y * 5.994_f32 - x.cos();
        let mut acc = Accumulator998::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.616_f32 + y.sin();
        let b = y * 5.301_f32 - x.cos();
        let mut acc = Accumulator998::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.771_f32 + y.sin();
        let b = y * 9.015_f32 - x.cos();
        let mut acc = Accumulator998::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.625_f32 + y.sin();
        let b = y * 3.519_f32 - x.cos();
        let mut acc = Accumulator998::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.719_f32 + y.sin();
        let b = y * 5.75_f32 - x.cos();
        let mut acc = Accumulator998::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_998(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_998() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_998(total as u64) % 997) as f32;
        total
    }
}

pub mod m999 {
    use super::*;

    pub struct Accumulator999<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator999<T> {
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
        let b = y * 0.969_f32 - x.cos();
        let mut acc = Accumulator999::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.751_f32 + y.sin();
        let b = y * 7.14_f32 - x.cos();
        let mut acc = Accumulator999::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.976_f32 + y.sin();
        let b = y * 4.566_f32 - x.cos();
        let mut acc = Accumulator999::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.469_f32 + y.sin();
        let b = y * 5.155_f32 - x.cos();
        let mut acc = Accumulator999::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.449_f32 + y.sin();
        let b = y * 8.796_f32 - x.cos();
        let mut acc = Accumulator999::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.451_f32 + y.sin();
        let b = y * 0.968_f32 - x.cos();
        let mut acc = Accumulator999::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.609_f32 + y.sin();
        let b = y * 5.806_f32 - x.cos();
        let mut acc = Accumulator999::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.892_f32 + y.sin();
        let b = y * 0.572_f32 - x.cos();
        let mut acc = Accumulator999::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.316_f32 + y.sin();
        let b = y * 8.017_f32 - x.cos();
        let mut acc = Accumulator999::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.358_f32 + y.sin();
        let b = y * 6.589_f32 - x.cos();
        let mut acc = Accumulator999::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.994_f32 + y.sin();
        let b = y * 3.43_f32 - x.cos();
        let mut acc = Accumulator999::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.567_f32 + y.sin();
        let b = y * 9.571_f32 - x.cos();
        let mut acc = Accumulator999::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.366_f32 + y.sin();
        let b = y * 7.487_f32 - x.cos();
        let mut acc = Accumulator999::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.555_f32 + y.sin();
        let b = y * 5.281_f32 - x.cos();
        let mut acc = Accumulator999::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.0_f32 + y.sin();
        let b = y * 2.425_f32 - x.cos();
        let mut acc = Accumulator999::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.147_f32 + y.sin();
        let b = y * 1.281_f32 - x.cos();
        let mut acc = Accumulator999::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.199_f32 + y.sin();
        let b = y * 2.844_f32 - x.cos();
        let mut acc = Accumulator999::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.687_f32 + y.sin();
        let b = y * 4.31_f32 - x.cos();
        let mut acc = Accumulator999::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.608_f32 + y.sin();
        let b = y * 3.772_f32 - x.cos();
        let mut acc = Accumulator999::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.886_f32 + y.sin();
        let b = y * 6.993_f32 - x.cos();
        let mut acc = Accumulator999::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_999(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(999u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_999() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_999(total as u64) % 997) as f32;
        total
    }
}

pub fn touch_bulk_9() -> f32 {
    let mut total = 0.0_f32;
    total += m900::run_all_900();
    total += m901::run_all_901();
    total += m902::run_all_902();
    total += m903::run_all_903();
    total += m904::run_all_904();
    total += m905::run_all_905();
    total += m906::run_all_906();
    total += m907::run_all_907();
    total += m908::run_all_908();
    total += m909::run_all_909();
    total += m910::run_all_910();
    total += m911::run_all_911();
    total += m912::run_all_912();
    total += m913::run_all_913();
    total += m914::run_all_914();
    total += m915::run_all_915();
    total += m916::run_all_916();
    total += m917::run_all_917();
    total += m918::run_all_918();
    total += m919::run_all_919();
    total += m920::run_all_920();
    total += m921::run_all_921();
    total += m922::run_all_922();
    total += m923::run_all_923();
    total += m924::run_all_924();
    total += m925::run_all_925();
    total += m926::run_all_926();
    total += m927::run_all_927();
    total += m928::run_all_928();
    total += m929::run_all_929();
    total += m930::run_all_930();
    total += m931::run_all_931();
    total += m932::run_all_932();
    total += m933::run_all_933();
    total += m934::run_all_934();
    total += m935::run_all_935();
    total += m936::run_all_936();
    total += m937::run_all_937();
    total += m938::run_all_938();
    total += m939::run_all_939();
    total += m940::run_all_940();
    total += m941::run_all_941();
    total += m942::run_all_942();
    total += m943::run_all_943();
    total += m944::run_all_944();
    total += m945::run_all_945();
    total += m946::run_all_946();
    total += m947::run_all_947();
    total += m948::run_all_948();
    total += m949::run_all_949();
    total += m950::run_all_950();
    total += m951::run_all_951();
    total += m952::run_all_952();
    total += m953::run_all_953();
    total += m954::run_all_954();
    total += m955::run_all_955();
    total += m956::run_all_956();
    total += m957::run_all_957();
    total += m958::run_all_958();
    total += m959::run_all_959();
    total += m960::run_all_960();
    total += m961::run_all_961();
    total += m962::run_all_962();
    total += m963::run_all_963();
    total += m964::run_all_964();
    total += m965::run_all_965();
    total += m966::run_all_966();
    total += m967::run_all_967();
    total += m968::run_all_968();
    total += m969::run_all_969();
    total += m970::run_all_970();
    total += m971::run_all_971();
    total += m972::run_all_972();
    total += m973::run_all_973();
    total += m974::run_all_974();
    total += m975::run_all_975();
    total += m976::run_all_976();
    total += m977::run_all_977();
    total += m978::run_all_978();
    total += m979::run_all_979();
    total += m980::run_all_980();
    total += m981::run_all_981();
    total += m982::run_all_982();
    total += m983::run_all_983();
    total += m984::run_all_984();
    total += m985::run_all_985();
    total += m986::run_all_986();
    total += m987::run_all_987();
    total += m988::run_all_988();
    total += m989::run_all_989();
    total += m990::run_all_990();
    total += m991::run_all_991();
    total += m992::run_all_992();
    total += m993::run_all_993();
    total += m994::run_all_994();
    total += m995::run_all_995();
    total += m996::run_all_996();
    total += m997::run_all_997();
    total += m998::run_all_998();
    total += m999::run_all_999();
    total
}
