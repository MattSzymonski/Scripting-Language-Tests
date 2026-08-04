//! Auto-generated bulk module (file 8) - exists purely to make `game_rs`
//! heavier for a hot-patch speed experiment. Real arithmetic, real generics
//! (so monomorphization isn't free), and real use of a rotating set of
//! external crates so none of them are dead code. `touch_bulk_8()` is
//! called once from `game::init` so nothing here gets stripped as unreachable.

use rand::{Rng, SeedableRng};
use sha2::{Digest, Sha256};
use noise::{NoiseFn, Perlin};
use regex::Regex;
use itertools::Itertools;
use uuid::Uuid;
use chrono::Utc;


pub mod m800 {
    use super::*;

    pub struct Accumulator800<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator800<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.718_f32 + y.sin();
        let b = y * 3.587_f32 - x.cos();
        let mut acc = Accumulator800::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.42_f32 + y.sin();
        let b = y * 4.69_f32 - x.cos();
        let mut acc = Accumulator800::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.814_f32 + y.sin();
        let b = y * 1.216_f32 - x.cos();
        let mut acc = Accumulator800::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.74_f32 + y.sin();
        let b = y * 7.339_f32 - x.cos();
        let mut acc = Accumulator800::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.793_f32 + y.sin();
        let b = y * 9.64_f32 - x.cos();
        let mut acc = Accumulator800::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.009_f32 + y.sin();
        let b = y * 3.021_f32 - x.cos();
        let mut acc = Accumulator800::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.192_f32 + y.sin();
        let b = y * 6.677_f32 - x.cos();
        let mut acc = Accumulator800::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.064_f32 + y.sin();
        let b = y * 7.923_f32 - x.cos();
        let mut acc = Accumulator800::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.552_f32 + y.sin();
        let b = y * 2.954_f32 - x.cos();
        let mut acc = Accumulator800::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.696_f32 + y.sin();
        let b = y * 3.496_f32 - x.cos();
        let mut acc = Accumulator800::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.488_f32 + y.sin();
        let b = y * 3.975_f32 - x.cos();
        let mut acc = Accumulator800::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.806_f32 + y.sin();
        let b = y * 3.489_f32 - x.cos();
        let mut acc = Accumulator800::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.544_f32 + y.sin();
        let b = y * 4.869_f32 - x.cos();
        let mut acc = Accumulator800::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.492_f32 + y.sin();
        let b = y * 1.598_f32 - x.cos();
        let mut acc = Accumulator800::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.825_f32 + y.sin();
        let b = y * 8.261_f32 - x.cos();
        let mut acc = Accumulator800::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.458_f32 + y.sin();
        let b = y * 4.93_f32 - x.cos();
        let mut acc = Accumulator800::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.403_f32 + y.sin();
        let b = y * 8.21_f32 - x.cos();
        let mut acc = Accumulator800::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.457_f32 + y.sin();
        let b = y * 9.365_f32 - x.cos();
        let mut acc = Accumulator800::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.404_f32 + y.sin();
        let b = y * 4.78_f32 - x.cos();
        let mut acc = Accumulator800::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.114_f32 + y.sin();
        let b = y * 7.332_f32 - x.cos();
        let mut acc = Accumulator800::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_800(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_800() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_800(total as u64) % 997) as f32;
        total
    }
}

pub mod m801 {
    use super::*;

    pub struct Accumulator801<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator801<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.465_f32 + y.sin();
        let b = y * 5.768_f32 - x.cos();
        let mut acc = Accumulator801::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.133_f32 + y.sin();
        let b = y * 0.5_f32 - x.cos();
        let mut acc = Accumulator801::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.566_f32 + y.sin();
        let b = y * 0.398_f32 - x.cos();
        let mut acc = Accumulator801::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.487_f32 + y.sin();
        let b = y * 0.912_f32 - x.cos();
        let mut acc = Accumulator801::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.6_f32 + y.sin();
        let b = y * 5.585_f32 - x.cos();
        let mut acc = Accumulator801::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.564_f32 + y.sin();
        let b = y * 0.781_f32 - x.cos();
        let mut acc = Accumulator801::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.423_f32 + y.sin();
        let b = y * 3.121_f32 - x.cos();
        let mut acc = Accumulator801::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.918_f32 + y.sin();
        let b = y * 4.173_f32 - x.cos();
        let mut acc = Accumulator801::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.405_f32 + y.sin();
        let b = y * 8.533_f32 - x.cos();
        let mut acc = Accumulator801::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.229_f32 + y.sin();
        let b = y * 3.534_f32 - x.cos();
        let mut acc = Accumulator801::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.214_f32 + y.sin();
        let b = y * 7.872_f32 - x.cos();
        let mut acc = Accumulator801::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.076_f32 + y.sin();
        let b = y * 4.087_f32 - x.cos();
        let mut acc = Accumulator801::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.547_f32 + y.sin();
        let b = y * 6.052_f32 - x.cos();
        let mut acc = Accumulator801::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.975_f32 + y.sin();
        let b = y * 5.227_f32 - x.cos();
        let mut acc = Accumulator801::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.057_f32 + y.sin();
        let b = y * 7.255_f32 - x.cos();
        let mut acc = Accumulator801::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.645_f32 + y.sin();
        let b = y * 6.8_f32 - x.cos();
        let mut acc = Accumulator801::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.627_f32 + y.sin();
        let b = y * 9.075_f32 - x.cos();
        let mut acc = Accumulator801::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.746_f32 + y.sin();
        let b = y * 9.002_f32 - x.cos();
        let mut acc = Accumulator801::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.952_f32 + y.sin();
        let b = y * 0.815_f32 - x.cos();
        let mut acc = Accumulator801::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.165_f32 + y.sin();
        let b = y * 3.133_f32 - x.cos();
        let mut acc = Accumulator801::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_801(seed: u64) -> u64 {
        let re = Regex::new(r"m801-(\d+)").unwrap();
        let hay = format!("m801-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_801() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_801(total as u64) % 997) as f32;
        total
    }
}

pub mod m802 {
    use super::*;

    pub struct Accumulator802<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator802<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.365_f32 + y.sin();
        let b = y * 1.582_f32 - x.cos();
        let mut acc = Accumulator802::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.566_f32 + y.sin();
        let b = y * 8.915_f32 - x.cos();
        let mut acc = Accumulator802::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.662_f32 + y.sin();
        let b = y * 5.318_f32 - x.cos();
        let mut acc = Accumulator802::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.859_f32 + y.sin();
        let b = y * 8.551_f32 - x.cos();
        let mut acc = Accumulator802::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.643_f32 + y.sin();
        let b = y * 9.03_f32 - x.cos();
        let mut acc = Accumulator802::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.205_f32 + y.sin();
        let b = y * 7.955_f32 - x.cos();
        let mut acc = Accumulator802::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.587_f32 + y.sin();
        let b = y * 1.625_f32 - x.cos();
        let mut acc = Accumulator802::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.392_f32 + y.sin();
        let b = y * 4.733_f32 - x.cos();
        let mut acc = Accumulator802::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.615_f32 + y.sin();
        let b = y * 0.134_f32 - x.cos();
        let mut acc = Accumulator802::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.792_f32 + y.sin();
        let b = y * 0.952_f32 - x.cos();
        let mut acc = Accumulator802::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.599_f32 + y.sin();
        let b = y * 3.732_f32 - x.cos();
        let mut acc = Accumulator802::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.296_f32 + y.sin();
        let b = y * 8.541_f32 - x.cos();
        let mut acc = Accumulator802::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.995_f32 + y.sin();
        let b = y * 7.724_f32 - x.cos();
        let mut acc = Accumulator802::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.41_f32 + y.sin();
        let b = y * 8.313_f32 - x.cos();
        let mut acc = Accumulator802::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.393_f32 + y.sin();
        let b = y * 4.489_f32 - x.cos();
        let mut acc = Accumulator802::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.966_f32 + y.sin();
        let b = y * 6.837_f32 - x.cos();
        let mut acc = Accumulator802::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.564_f32 + y.sin();
        let b = y * 8.378_f32 - x.cos();
        let mut acc = Accumulator802::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.984_f32 + y.sin();
        let b = y * 9.527_f32 - x.cos();
        let mut acc = Accumulator802::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.173_f32 + y.sin();
        let b = y * 4.009_f32 - x.cos();
        let mut acc = Accumulator802::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.211_f32 + y.sin();
        let b = y * 4.069_f32 - x.cos();
        let mut acc = Accumulator802::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_802(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_802() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_802(total as u64) % 997) as f32;
        total
    }
}

pub mod m803 {
    use super::*;

    pub struct Accumulator803<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator803<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.947_f32 + y.sin();
        let b = y * 2.897_f32 - x.cos();
        let mut acc = Accumulator803::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.332_f32 + y.sin();
        let b = y * 4.73_f32 - x.cos();
        let mut acc = Accumulator803::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.277_f32 + y.sin();
        let b = y * 8.18_f32 - x.cos();
        let mut acc = Accumulator803::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.642_f32 + y.sin();
        let b = y * 8.352_f32 - x.cos();
        let mut acc = Accumulator803::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.582_f32 + y.sin();
        let b = y * 7.814_f32 - x.cos();
        let mut acc = Accumulator803::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.89_f32 + y.sin();
        let b = y * 3.572_f32 - x.cos();
        let mut acc = Accumulator803::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.488_f32 + y.sin();
        let b = y * 8.827_f32 - x.cos();
        let mut acc = Accumulator803::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.264_f32 + y.sin();
        let b = y * 8.073_f32 - x.cos();
        let mut acc = Accumulator803::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.805_f32 + y.sin();
        let b = y * 3.262_f32 - x.cos();
        let mut acc = Accumulator803::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.673_f32 + y.sin();
        let b = y * 6.487_f32 - x.cos();
        let mut acc = Accumulator803::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.302_f32 + y.sin();
        let b = y * 5.721_f32 - x.cos();
        let mut acc = Accumulator803::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.803_f32 + y.sin();
        let b = y * 5.719_f32 - x.cos();
        let mut acc = Accumulator803::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.95_f32 + y.sin();
        let b = y * 2.426_f32 - x.cos();
        let mut acc = Accumulator803::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.898_f32 + y.sin();
        let b = y * 9.591_f32 - x.cos();
        let mut acc = Accumulator803::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.675_f32 + y.sin();
        let b = y * 9.008_f32 - x.cos();
        let mut acc = Accumulator803::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.331_f32 + y.sin();
        let b = y * 2.224_f32 - x.cos();
        let mut acc = Accumulator803::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.641_f32 + y.sin();
        let b = y * 2.21_f32 - x.cos();
        let mut acc = Accumulator803::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.242_f32 + y.sin();
        let b = y * 3.712_f32 - x.cos();
        let mut acc = Accumulator803::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.067_f32 + y.sin();
        let b = y * 2.936_f32 - x.cos();
        let mut acc = Accumulator803::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.504_f32 + y.sin();
        let b = y * 3.076_f32 - x.cos();
        let mut acc = Accumulator803::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_803(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(803u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_803() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_803(total as u64) % 997) as f32;
        total
    }
}

pub mod m804 {
    use super::*;

    pub struct Accumulator804<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator804<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.266_f32 + y.sin();
        let b = y * 0.116_f32 - x.cos();
        let mut acc = Accumulator804::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.035_f32 + y.sin();
        let b = y * 5.827_f32 - x.cos();
        let mut acc = Accumulator804::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.095_f32 + y.sin();
        let b = y * 6.586_f32 - x.cos();
        let mut acc = Accumulator804::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.024_f32 + y.sin();
        let b = y * 2.906_f32 - x.cos();
        let mut acc = Accumulator804::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.85_f32 + y.sin();
        let b = y * 2.742_f32 - x.cos();
        let mut acc = Accumulator804::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.919_f32 + y.sin();
        let b = y * 4.09_f32 - x.cos();
        let mut acc = Accumulator804::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.853_f32 + y.sin();
        let b = y * 3.69_f32 - x.cos();
        let mut acc = Accumulator804::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.659_f32 + y.sin();
        let b = y * 9.303_f32 - x.cos();
        let mut acc = Accumulator804::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.056_f32 + y.sin();
        let b = y * 7.773_f32 - x.cos();
        let mut acc = Accumulator804::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.576_f32 + y.sin();
        let b = y * 4.369_f32 - x.cos();
        let mut acc = Accumulator804::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.97_f32 + y.sin();
        let b = y * 7.045_f32 - x.cos();
        let mut acc = Accumulator804::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.897_f32 + y.sin();
        let b = y * 3.471_f32 - x.cos();
        let mut acc = Accumulator804::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.136_f32 + y.sin();
        let b = y * 5.095_f32 - x.cos();
        let mut acc = Accumulator804::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.12_f32 + y.sin();
        let b = y * 7.342_f32 - x.cos();
        let mut acc = Accumulator804::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.158_f32 + y.sin();
        let b = y * 1.578_f32 - x.cos();
        let mut acc = Accumulator804::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.803_f32 + y.sin();
        let b = y * 7.5_f32 - x.cos();
        let mut acc = Accumulator804::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.759_f32 + y.sin();
        let b = y * 0.496_f32 - x.cos();
        let mut acc = Accumulator804::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.913_f32 + y.sin();
        let b = y * 5.296_f32 - x.cos();
        let mut acc = Accumulator804::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.585_f32 + y.sin();
        let b = y * 9.845_f32 - x.cos();
        let mut acc = Accumulator804::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.367_f32 + y.sin();
        let b = y * 3.454_f32 - x.cos();
        let mut acc = Accumulator804::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_804(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_804() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_804(total as u64) % 997) as f32;
        total
    }
}

pub mod m805 {
    use super::*;

    pub struct Accumulator805<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator805<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.624_f32 + y.sin();
        let b = y * 3.103_f32 - x.cos();
        let mut acc = Accumulator805::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.09_f32 + y.sin();
        let b = y * 1.623_f32 - x.cos();
        let mut acc = Accumulator805::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.889_f32 + y.sin();
        let b = y * 5.093_f32 - x.cos();
        let mut acc = Accumulator805::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.958_f32 + y.sin();
        let b = y * 0.653_f32 - x.cos();
        let mut acc = Accumulator805::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.6_f32 + y.sin();
        let b = y * 9.759_f32 - x.cos();
        let mut acc = Accumulator805::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.563_f32 + y.sin();
        let b = y * 4.62_f32 - x.cos();
        let mut acc = Accumulator805::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.419_f32 + y.sin();
        let b = y * 9.636_f32 - x.cos();
        let mut acc = Accumulator805::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.082_f32 + y.sin();
        let b = y * 0.134_f32 - x.cos();
        let mut acc = Accumulator805::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.313_f32 + y.sin();
        let b = y * 0.865_f32 - x.cos();
        let mut acc = Accumulator805::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.215_f32 + y.sin();
        let b = y * 4.724_f32 - x.cos();
        let mut acc = Accumulator805::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.476_f32 + y.sin();
        let b = y * 5.875_f32 - x.cos();
        let mut acc = Accumulator805::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.25_f32 + y.sin();
        let b = y * 4.695_f32 - x.cos();
        let mut acc = Accumulator805::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.86_f32 + y.sin();
        let b = y * 1.663_f32 - x.cos();
        let mut acc = Accumulator805::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.527_f32 + y.sin();
        let b = y * 3.277_f32 - x.cos();
        let mut acc = Accumulator805::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.344_f32 + y.sin();
        let b = y * 7.733_f32 - x.cos();
        let mut acc = Accumulator805::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.914_f32 + y.sin();
        let b = y * 3.617_f32 - x.cos();
        let mut acc = Accumulator805::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.136_f32 + y.sin();
        let b = y * 2.833_f32 - x.cos();
        let mut acc = Accumulator805::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.475_f32 + y.sin();
        let b = y * 7.187_f32 - x.cos();
        let mut acc = Accumulator805::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.509_f32 + y.sin();
        let b = y * 2.665_f32 - x.cos();
        let mut acc = Accumulator805::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.544_f32 + y.sin();
        let b = y * 7.486_f32 - x.cos();
        let mut acc = Accumulator805::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_805(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_805() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_805(total as u64) % 997) as f32;
        total
    }
}

pub mod m806 {
    use super::*;

    pub struct Accumulator806<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator806<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.846_f32 + y.sin();
        let b = y * 4.152_f32 - x.cos();
        let mut acc = Accumulator806::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.528_f32 + y.sin();
        let b = y * 1.6_f32 - x.cos();
        let mut acc = Accumulator806::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.222_f32 + y.sin();
        let b = y * 8.36_f32 - x.cos();
        let mut acc = Accumulator806::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.856_f32 + y.sin();
        let b = y * 5.276_f32 - x.cos();
        let mut acc = Accumulator806::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.112_f32 + y.sin();
        let b = y * 1.862_f32 - x.cos();
        let mut acc = Accumulator806::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.053_f32 + y.sin();
        let b = y * 3.217_f32 - x.cos();
        let mut acc = Accumulator806::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.252_f32 + y.sin();
        let b = y * 0.783_f32 - x.cos();
        let mut acc = Accumulator806::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.095_f32 + y.sin();
        let b = y * 3.107_f32 - x.cos();
        let mut acc = Accumulator806::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.298_f32 + y.sin();
        let b = y * 5.361_f32 - x.cos();
        let mut acc = Accumulator806::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.416_f32 + y.sin();
        let b = y * 0.7_f32 - x.cos();
        let mut acc = Accumulator806::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.67_f32 + y.sin();
        let b = y * 6.132_f32 - x.cos();
        let mut acc = Accumulator806::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.924_f32 + y.sin();
        let b = y * 3.939_f32 - x.cos();
        let mut acc = Accumulator806::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.2_f32 + y.sin();
        let b = y * 7.91_f32 - x.cos();
        let mut acc = Accumulator806::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.605_f32 + y.sin();
        let b = y * 8.491_f32 - x.cos();
        let mut acc = Accumulator806::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.616_f32 + y.sin();
        let b = y * 1.363_f32 - x.cos();
        let mut acc = Accumulator806::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.781_f32 + y.sin();
        let b = y * 9.615_f32 - x.cos();
        let mut acc = Accumulator806::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.117_f32 + y.sin();
        let b = y * 2.603_f32 - x.cos();
        let mut acc = Accumulator806::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.794_f32 + y.sin();
        let b = y * 9.602_f32 - x.cos();
        let mut acc = Accumulator806::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.369_f32 + y.sin();
        let b = y * 6.66_f32 - x.cos();
        let mut acc = Accumulator806::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.429_f32 + y.sin();
        let b = y * 0.377_f32 - x.cos();
        let mut acc = Accumulator806::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_806(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m806-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_806() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_806(total as u64) % 997) as f32;
        total
    }
}

pub mod m807 {
    use super::*;

    pub struct Accumulator807<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator807<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.346_f32 + y.sin();
        let b = y * 4.484_f32 - x.cos();
        let mut acc = Accumulator807::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.623_f32 + y.sin();
        let b = y * 4.302_f32 - x.cos();
        let mut acc = Accumulator807::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.662_f32 + y.sin();
        let b = y * 3.235_f32 - x.cos();
        let mut acc = Accumulator807::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.622_f32 + y.sin();
        let b = y * 6.775_f32 - x.cos();
        let mut acc = Accumulator807::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.493_f32 + y.sin();
        let b = y * 1.61_f32 - x.cos();
        let mut acc = Accumulator807::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.463_f32 + y.sin();
        let b = y * 2.631_f32 - x.cos();
        let mut acc = Accumulator807::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.879_f32 + y.sin();
        let b = y * 3.366_f32 - x.cos();
        let mut acc = Accumulator807::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.318_f32 + y.sin();
        let b = y * 1.487_f32 - x.cos();
        let mut acc = Accumulator807::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.197_f32 + y.sin();
        let b = y * 9.266_f32 - x.cos();
        let mut acc = Accumulator807::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.703_f32 + y.sin();
        let b = y * 1.804_f32 - x.cos();
        let mut acc = Accumulator807::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.272_f32 + y.sin();
        let b = y * 1.64_f32 - x.cos();
        let mut acc = Accumulator807::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.659_f32 + y.sin();
        let b = y * 8.929_f32 - x.cos();
        let mut acc = Accumulator807::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.765_f32 + y.sin();
        let b = y * 5.379_f32 - x.cos();
        let mut acc = Accumulator807::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.845_f32 + y.sin();
        let b = y * 1.676_f32 - x.cos();
        let mut acc = Accumulator807::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.745_f32 + y.sin();
        let b = y * 9.128_f32 - x.cos();
        let mut acc = Accumulator807::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.1_f32 + y.sin();
        let b = y * 3.626_f32 - x.cos();
        let mut acc = Accumulator807::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.819_f32 + y.sin();
        let b = y * 2.944_f32 - x.cos();
        let mut acc = Accumulator807::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.792_f32 + y.sin();
        let b = y * 7.151_f32 - x.cos();
        let mut acc = Accumulator807::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.166_f32 + y.sin();
        let b = y * 7.003_f32 - x.cos();
        let mut acc = Accumulator807::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.55_f32 + y.sin();
        let b = y * 3.901_f32 - x.cos();
        let mut acc = Accumulator807::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_807(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_807() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_807(total as u64) % 997) as f32;
        total
    }
}

pub mod m808 {
    use super::*;

    pub struct Accumulator808<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator808<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.798_f32 + y.sin();
        let b = y * 7.067_f32 - x.cos();
        let mut acc = Accumulator808::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.619_f32 + y.sin();
        let b = y * 2.333_f32 - x.cos();
        let mut acc = Accumulator808::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.308_f32 + y.sin();
        let b = y * 9.427_f32 - x.cos();
        let mut acc = Accumulator808::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.747_f32 + y.sin();
        let b = y * 5.796_f32 - x.cos();
        let mut acc = Accumulator808::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.333_f32 + y.sin();
        let b = y * 3.644_f32 - x.cos();
        let mut acc = Accumulator808::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.808_f32 + y.sin();
        let b = y * 7.802_f32 - x.cos();
        let mut acc = Accumulator808::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.2_f32 + y.sin();
        let b = y * 9.235_f32 - x.cos();
        let mut acc = Accumulator808::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.825_f32 + y.sin();
        let b = y * 0.826_f32 - x.cos();
        let mut acc = Accumulator808::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.761_f32 + y.sin();
        let b = y * 3.315_f32 - x.cos();
        let mut acc = Accumulator808::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.645_f32 + y.sin();
        let b = y * 5.587_f32 - x.cos();
        let mut acc = Accumulator808::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.175_f32 + y.sin();
        let b = y * 5.445_f32 - x.cos();
        let mut acc = Accumulator808::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.109_f32 + y.sin();
        let b = y * 1.166_f32 - x.cos();
        let mut acc = Accumulator808::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.354_f32 + y.sin();
        let b = y * 6.708_f32 - x.cos();
        let mut acc = Accumulator808::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.639_f32 + y.sin();
        let b = y * 5.496_f32 - x.cos();
        let mut acc = Accumulator808::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.128_f32 + y.sin();
        let b = y * 4.246_f32 - x.cos();
        let mut acc = Accumulator808::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.356_f32 + y.sin();
        let b = y * 3.489_f32 - x.cos();
        let mut acc = Accumulator808::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.715_f32 + y.sin();
        let b = y * 8.943_f32 - x.cos();
        let mut acc = Accumulator808::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.696_f32 + y.sin();
        let b = y * 4.678_f32 - x.cos();
        let mut acc = Accumulator808::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.596_f32 + y.sin();
        let b = y * 9.004_f32 - x.cos();
        let mut acc = Accumulator808::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.407_f32 + y.sin();
        let b = y * 3.961_f32 - x.cos();
        let mut acc = Accumulator808::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_808(seed: u64) -> u64 {
        let re = Regex::new(r"m808-(\d+)").unwrap();
        let hay = format!("m808-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_808() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_808(total as u64) % 997) as f32;
        total
    }
}

pub mod m809 {
    use super::*;

    pub struct Accumulator809<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator809<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.702_f32 + y.sin();
        let b = y * 5.619_f32 - x.cos();
        let mut acc = Accumulator809::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.881_f32 + y.sin();
        let b = y * 1.963_f32 - x.cos();
        let mut acc = Accumulator809::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.904_f32 + y.sin();
        let b = y * 5.038_f32 - x.cos();
        let mut acc = Accumulator809::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.696_f32 + y.sin();
        let b = y * 5.481_f32 - x.cos();
        let mut acc = Accumulator809::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.224_f32 + y.sin();
        let b = y * 4.799_f32 - x.cos();
        let mut acc = Accumulator809::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.82_f32 + y.sin();
        let b = y * 5.725_f32 - x.cos();
        let mut acc = Accumulator809::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.826_f32 + y.sin();
        let b = y * 3.335_f32 - x.cos();
        let mut acc = Accumulator809::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.811_f32 + y.sin();
        let b = y * 7.749_f32 - x.cos();
        let mut acc = Accumulator809::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.771_f32 + y.sin();
        let b = y * 3.789_f32 - x.cos();
        let mut acc = Accumulator809::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.195_f32 + y.sin();
        let b = y * 2.984_f32 - x.cos();
        let mut acc = Accumulator809::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.421_f32 + y.sin();
        let b = y * 9.158_f32 - x.cos();
        let mut acc = Accumulator809::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.698_f32 + y.sin();
        let b = y * 7.155_f32 - x.cos();
        let mut acc = Accumulator809::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.22_f32 + y.sin();
        let b = y * 6.97_f32 - x.cos();
        let mut acc = Accumulator809::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.419_f32 + y.sin();
        let b = y * 7.893_f32 - x.cos();
        let mut acc = Accumulator809::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.178_f32 + y.sin();
        let b = y * 5.021_f32 - x.cos();
        let mut acc = Accumulator809::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.261_f32 + y.sin();
        let b = y * 7.332_f32 - x.cos();
        let mut acc = Accumulator809::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.284_f32 + y.sin();
        let b = y * 5.351_f32 - x.cos();
        let mut acc = Accumulator809::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.494_f32 + y.sin();
        let b = y * 3.392_f32 - x.cos();
        let mut acc = Accumulator809::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.896_f32 + y.sin();
        let b = y * 4.153_f32 - x.cos();
        let mut acc = Accumulator809::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.027_f32 + y.sin();
        let b = y * 2.087_f32 - x.cos();
        let mut acc = Accumulator809::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_809(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_809() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_809(total as u64) % 997) as f32;
        total
    }
}

pub mod m810 {
    use super::*;

    pub struct Accumulator810<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator810<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.898_f32 + y.sin();
        let b = y * 4.323_f32 - x.cos();
        let mut acc = Accumulator810::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.11_f32 + y.sin();
        let b = y * 7.421_f32 - x.cos();
        let mut acc = Accumulator810::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.464_f32 + y.sin();
        let b = y * 9.758_f32 - x.cos();
        let mut acc = Accumulator810::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.976_f32 + y.sin();
        let b = y * 0.118_f32 - x.cos();
        let mut acc = Accumulator810::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.891_f32 + y.sin();
        let b = y * 7.716_f32 - x.cos();
        let mut acc = Accumulator810::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.423_f32 + y.sin();
        let b = y * 9.127_f32 - x.cos();
        let mut acc = Accumulator810::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.52_f32 + y.sin();
        let b = y * 6.875_f32 - x.cos();
        let mut acc = Accumulator810::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.514_f32 + y.sin();
        let b = y * 8.715_f32 - x.cos();
        let mut acc = Accumulator810::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.796_f32 + y.sin();
        let b = y * 9.373_f32 - x.cos();
        let mut acc = Accumulator810::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.486_f32 + y.sin();
        let b = y * 5.807_f32 - x.cos();
        let mut acc = Accumulator810::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.159_f32 + y.sin();
        let b = y * 5.874_f32 - x.cos();
        let mut acc = Accumulator810::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.935_f32 + y.sin();
        let b = y * 6.436_f32 - x.cos();
        let mut acc = Accumulator810::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.174_f32 + y.sin();
        let b = y * 9.411_f32 - x.cos();
        let mut acc = Accumulator810::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.522_f32 + y.sin();
        let b = y * 9.052_f32 - x.cos();
        let mut acc = Accumulator810::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.755_f32 + y.sin();
        let b = y * 3.825_f32 - x.cos();
        let mut acc = Accumulator810::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.729_f32 + y.sin();
        let b = y * 6.359_f32 - x.cos();
        let mut acc = Accumulator810::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.08_f32 + y.sin();
        let b = y * 2.418_f32 - x.cos();
        let mut acc = Accumulator810::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.686_f32 + y.sin();
        let b = y * 7.25_f32 - x.cos();
        let mut acc = Accumulator810::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.54_f32 + y.sin();
        let b = y * 0.392_f32 - x.cos();
        let mut acc = Accumulator810::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.621_f32 + y.sin();
        let b = y * 5.756_f32 - x.cos();
        let mut acc = Accumulator810::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_810(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(810u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_810() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_810(total as u64) % 997) as f32;
        total
    }
}

pub mod m811 {
    use super::*;

    pub struct Accumulator811<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator811<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.744_f32 + y.sin();
        let b = y * 5.269_f32 - x.cos();
        let mut acc = Accumulator811::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.369_f32 + y.sin();
        let b = y * 9.686_f32 - x.cos();
        let mut acc = Accumulator811::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.166_f32 + y.sin();
        let b = y * 6.558_f32 - x.cos();
        let mut acc = Accumulator811::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.786_f32 + y.sin();
        let b = y * 7.763_f32 - x.cos();
        let mut acc = Accumulator811::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.843_f32 + y.sin();
        let b = y * 8.904_f32 - x.cos();
        let mut acc = Accumulator811::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.235_f32 + y.sin();
        let b = y * 4.93_f32 - x.cos();
        let mut acc = Accumulator811::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.159_f32 + y.sin();
        let b = y * 3.979_f32 - x.cos();
        let mut acc = Accumulator811::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.545_f32 + y.sin();
        let b = y * 1.226_f32 - x.cos();
        let mut acc = Accumulator811::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.193_f32 + y.sin();
        let b = y * 8.241_f32 - x.cos();
        let mut acc = Accumulator811::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.447_f32 + y.sin();
        let b = y * 5.095_f32 - x.cos();
        let mut acc = Accumulator811::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.495_f32 + y.sin();
        let b = y * 9.848_f32 - x.cos();
        let mut acc = Accumulator811::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.225_f32 + y.sin();
        let b = y * 2.771_f32 - x.cos();
        let mut acc = Accumulator811::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.542_f32 + y.sin();
        let b = y * 3.637_f32 - x.cos();
        let mut acc = Accumulator811::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.59_f32 + y.sin();
        let b = y * 0.438_f32 - x.cos();
        let mut acc = Accumulator811::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.101_f32 + y.sin();
        let b = y * 3.918_f32 - x.cos();
        let mut acc = Accumulator811::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.713_f32 + y.sin();
        let b = y * 6.878_f32 - x.cos();
        let mut acc = Accumulator811::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.777_f32 + y.sin();
        let b = y * 2.305_f32 - x.cos();
        let mut acc = Accumulator811::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.281_f32 + y.sin();
        let b = y * 9.125_f32 - x.cos();
        let mut acc = Accumulator811::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.032_f32 + y.sin();
        let b = y * 1.742_f32 - x.cos();
        let mut acc = Accumulator811::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.39_f32 + y.sin();
        let b = y * 0.207_f32 - x.cos();
        let mut acc = Accumulator811::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_811(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_811() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_811(total as u64) % 997) as f32;
        total
    }
}

pub mod m812 {
    use super::*;

    pub struct Accumulator812<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator812<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.87_f32 + y.sin();
        let b = y * 8.959_f32 - x.cos();
        let mut acc = Accumulator812::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.617_f32 + y.sin();
        let b = y * 0.906_f32 - x.cos();
        let mut acc = Accumulator812::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.497_f32 + y.sin();
        let b = y * 9.195_f32 - x.cos();
        let mut acc = Accumulator812::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.39_f32 + y.sin();
        let b = y * 9.589_f32 - x.cos();
        let mut acc = Accumulator812::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.531_f32 + y.sin();
        let b = y * 7.033_f32 - x.cos();
        let mut acc = Accumulator812::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.956_f32 + y.sin();
        let b = y * 1.283_f32 - x.cos();
        let mut acc = Accumulator812::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.492_f32 + y.sin();
        let b = y * 0.688_f32 - x.cos();
        let mut acc = Accumulator812::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.467_f32 + y.sin();
        let b = y * 0.348_f32 - x.cos();
        let mut acc = Accumulator812::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.783_f32 + y.sin();
        let b = y * 1.204_f32 - x.cos();
        let mut acc = Accumulator812::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.271_f32 + y.sin();
        let b = y * 1.318_f32 - x.cos();
        let mut acc = Accumulator812::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.335_f32 + y.sin();
        let b = y * 1.622_f32 - x.cos();
        let mut acc = Accumulator812::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.719_f32 + y.sin();
        let b = y * 9.761_f32 - x.cos();
        let mut acc = Accumulator812::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.441_f32 + y.sin();
        let b = y * 0.883_f32 - x.cos();
        let mut acc = Accumulator812::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.939_f32 + y.sin();
        let b = y * 1.094_f32 - x.cos();
        let mut acc = Accumulator812::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.69_f32 + y.sin();
        let b = y * 6.522_f32 - x.cos();
        let mut acc = Accumulator812::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.203_f32 + y.sin();
        let b = y * 5.618_f32 - x.cos();
        let mut acc = Accumulator812::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.095_f32 + y.sin();
        let b = y * 6.226_f32 - x.cos();
        let mut acc = Accumulator812::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.579_f32 + y.sin();
        let b = y * 6.682_f32 - x.cos();
        let mut acc = Accumulator812::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.277_f32 + y.sin();
        let b = y * 6.208_f32 - x.cos();
        let mut acc = Accumulator812::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.923_f32 + y.sin();
        let b = y * 2.098_f32 - x.cos();
        let mut acc = Accumulator812::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_812(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_812() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_812(total as u64) % 997) as f32;
        total
    }
}

pub mod m813 {
    use super::*;

    pub struct Accumulator813<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator813<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.642_f32 + y.sin();
        let b = y * 4.732_f32 - x.cos();
        let mut acc = Accumulator813::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.659_f32 + y.sin();
        let b = y * 3.256_f32 - x.cos();
        let mut acc = Accumulator813::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.61_f32 + y.sin();
        let b = y * 6.868_f32 - x.cos();
        let mut acc = Accumulator813::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.108_f32 + y.sin();
        let b = y * 0.667_f32 - x.cos();
        let mut acc = Accumulator813::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.904_f32 + y.sin();
        let b = y * 7.159_f32 - x.cos();
        let mut acc = Accumulator813::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.017_f32 + y.sin();
        let b = y * 6.409_f32 - x.cos();
        let mut acc = Accumulator813::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.757_f32 + y.sin();
        let b = y * 7.509_f32 - x.cos();
        let mut acc = Accumulator813::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.328_f32 + y.sin();
        let b = y * 4.641_f32 - x.cos();
        let mut acc = Accumulator813::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.44_f32 + y.sin();
        let b = y * 5.155_f32 - x.cos();
        let mut acc = Accumulator813::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.918_f32 + y.sin();
        let b = y * 4.032_f32 - x.cos();
        let mut acc = Accumulator813::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.347_f32 + y.sin();
        let b = y * 8.818_f32 - x.cos();
        let mut acc = Accumulator813::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.458_f32 + y.sin();
        let b = y * 0.466_f32 - x.cos();
        let mut acc = Accumulator813::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.485_f32 + y.sin();
        let b = y * 2.463_f32 - x.cos();
        let mut acc = Accumulator813::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.645_f32 + y.sin();
        let b = y * 3.392_f32 - x.cos();
        let mut acc = Accumulator813::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.226_f32 + y.sin();
        let b = y * 1.197_f32 - x.cos();
        let mut acc = Accumulator813::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.883_f32 + y.sin();
        let b = y * 2.982_f32 - x.cos();
        let mut acc = Accumulator813::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.333_f32 + y.sin();
        let b = y * 9.851_f32 - x.cos();
        let mut acc = Accumulator813::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.572_f32 + y.sin();
        let b = y * 6.986_f32 - x.cos();
        let mut acc = Accumulator813::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.691_f32 + y.sin();
        let b = y * 1.69_f32 - x.cos();
        let mut acc = Accumulator813::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.622_f32 + y.sin();
        let b = y * 2.131_f32 - x.cos();
        let mut acc = Accumulator813::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_813(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m813-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_813() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_813(total as u64) % 997) as f32;
        total
    }
}

pub mod m814 {
    use super::*;

    pub struct Accumulator814<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator814<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.135_f32 + y.sin();
        let b = y * 7.097_f32 - x.cos();
        let mut acc = Accumulator814::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.287_f32 + y.sin();
        let b = y * 7.299_f32 - x.cos();
        let mut acc = Accumulator814::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.013_f32 + y.sin();
        let b = y * 8.544_f32 - x.cos();
        let mut acc = Accumulator814::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.46_f32 + y.sin();
        let b = y * 2.535_f32 - x.cos();
        let mut acc = Accumulator814::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.375_f32 + y.sin();
        let b = y * 0.283_f32 - x.cos();
        let mut acc = Accumulator814::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.735_f32 + y.sin();
        let b = y * 2.809_f32 - x.cos();
        let mut acc = Accumulator814::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.247_f32 + y.sin();
        let b = y * 7.072_f32 - x.cos();
        let mut acc = Accumulator814::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.758_f32 + y.sin();
        let b = y * 1.69_f32 - x.cos();
        let mut acc = Accumulator814::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.261_f32 + y.sin();
        let b = y * 9.072_f32 - x.cos();
        let mut acc = Accumulator814::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.174_f32 + y.sin();
        let b = y * 8.871_f32 - x.cos();
        let mut acc = Accumulator814::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.651_f32 + y.sin();
        let b = y * 9.705_f32 - x.cos();
        let mut acc = Accumulator814::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.901_f32 + y.sin();
        let b = y * 0.907_f32 - x.cos();
        let mut acc = Accumulator814::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.007_f32 + y.sin();
        let b = y * 0.924_f32 - x.cos();
        let mut acc = Accumulator814::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.136_f32 + y.sin();
        let b = y * 0.949_f32 - x.cos();
        let mut acc = Accumulator814::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.718_f32 + y.sin();
        let b = y * 3.502_f32 - x.cos();
        let mut acc = Accumulator814::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.644_f32 + y.sin();
        let b = y * 4.19_f32 - x.cos();
        let mut acc = Accumulator814::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.096_f32 + y.sin();
        let b = y * 9.802_f32 - x.cos();
        let mut acc = Accumulator814::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.311_f32 + y.sin();
        let b = y * 9.217_f32 - x.cos();
        let mut acc = Accumulator814::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.329_f32 + y.sin();
        let b = y * 3.673_f32 - x.cos();
        let mut acc = Accumulator814::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.982_f32 + y.sin();
        let b = y * 7.036_f32 - x.cos();
        let mut acc = Accumulator814::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_814(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_814() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_814(total as u64) % 997) as f32;
        total
    }
}

pub mod m815 {
    use super::*;

    pub struct Accumulator815<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator815<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.106_f32 + y.sin();
        let b = y * 4.129_f32 - x.cos();
        let mut acc = Accumulator815::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.731_f32 + y.sin();
        let b = y * 8.123_f32 - x.cos();
        let mut acc = Accumulator815::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.083_f32 + y.sin();
        let b = y * 7.347_f32 - x.cos();
        let mut acc = Accumulator815::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.704_f32 + y.sin();
        let b = y * 1.871_f32 - x.cos();
        let mut acc = Accumulator815::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.821_f32 + y.sin();
        let b = y * 9.16_f32 - x.cos();
        let mut acc = Accumulator815::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.702_f32 + y.sin();
        let b = y * 9.407_f32 - x.cos();
        let mut acc = Accumulator815::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.297_f32 + y.sin();
        let b = y * 6.011_f32 - x.cos();
        let mut acc = Accumulator815::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.729_f32 + y.sin();
        let b = y * 1.976_f32 - x.cos();
        let mut acc = Accumulator815::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.99_f32 + y.sin();
        let b = y * 3.262_f32 - x.cos();
        let mut acc = Accumulator815::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.672_f32 + y.sin();
        let b = y * 6.542_f32 - x.cos();
        let mut acc = Accumulator815::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.382_f32 + y.sin();
        let b = y * 7.495_f32 - x.cos();
        let mut acc = Accumulator815::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.722_f32 + y.sin();
        let b = y * 4.255_f32 - x.cos();
        let mut acc = Accumulator815::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.866_f32 + y.sin();
        let b = y * 2.037_f32 - x.cos();
        let mut acc = Accumulator815::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.108_f32 + y.sin();
        let b = y * 1.386_f32 - x.cos();
        let mut acc = Accumulator815::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.415_f32 + y.sin();
        let b = y * 5.183_f32 - x.cos();
        let mut acc = Accumulator815::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.636_f32 + y.sin();
        let b = y * 2.27_f32 - x.cos();
        let mut acc = Accumulator815::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.993_f32 + y.sin();
        let b = y * 5.292_f32 - x.cos();
        let mut acc = Accumulator815::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.254_f32 + y.sin();
        let b = y * 6.432_f32 - x.cos();
        let mut acc = Accumulator815::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.134_f32 + y.sin();
        let b = y * 9.292_f32 - x.cos();
        let mut acc = Accumulator815::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.383_f32 + y.sin();
        let b = y * 0.669_f32 - x.cos();
        let mut acc = Accumulator815::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_815(seed: u64) -> u64 {
        let re = Regex::new(r"m815-(\d+)").unwrap();
        let hay = format!("m815-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_815() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_815(total as u64) % 997) as f32;
        total
    }
}

pub mod m816 {
    use super::*;

    pub struct Accumulator816<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator816<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.618_f32 + y.sin();
        let b = y * 2.755_f32 - x.cos();
        let mut acc = Accumulator816::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.821_f32 + y.sin();
        let b = y * 4.81_f32 - x.cos();
        let mut acc = Accumulator816::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.226_f32 + y.sin();
        let b = y * 2.773_f32 - x.cos();
        let mut acc = Accumulator816::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.6_f32 + y.sin();
        let b = y * 8.944_f32 - x.cos();
        let mut acc = Accumulator816::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.87_f32 + y.sin();
        let b = y * 2.022_f32 - x.cos();
        let mut acc = Accumulator816::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.397_f32 + y.sin();
        let b = y * 5.539_f32 - x.cos();
        let mut acc = Accumulator816::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.734_f32 + y.sin();
        let b = y * 1.674_f32 - x.cos();
        let mut acc = Accumulator816::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.242_f32 + y.sin();
        let b = y * 7.34_f32 - x.cos();
        let mut acc = Accumulator816::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.281_f32 + y.sin();
        let b = y * 6.53_f32 - x.cos();
        let mut acc = Accumulator816::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.455_f32 + y.sin();
        let b = y * 6.124_f32 - x.cos();
        let mut acc = Accumulator816::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.155_f32 + y.sin();
        let b = y * 6.046_f32 - x.cos();
        let mut acc = Accumulator816::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.58_f32 + y.sin();
        let b = y * 8.602_f32 - x.cos();
        let mut acc = Accumulator816::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.609_f32 + y.sin();
        let b = y * 6.231_f32 - x.cos();
        let mut acc = Accumulator816::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.599_f32 + y.sin();
        let b = y * 7.909_f32 - x.cos();
        let mut acc = Accumulator816::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.436_f32 + y.sin();
        let b = y * 6.535_f32 - x.cos();
        let mut acc = Accumulator816::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.847_f32 + y.sin();
        let b = y * 8.648_f32 - x.cos();
        let mut acc = Accumulator816::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.244_f32 + y.sin();
        let b = y * 9.758_f32 - x.cos();
        let mut acc = Accumulator816::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.848_f32 + y.sin();
        let b = y * 4.847_f32 - x.cos();
        let mut acc = Accumulator816::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.667_f32 + y.sin();
        let b = y * 8.847_f32 - x.cos();
        let mut acc = Accumulator816::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.384_f32 + y.sin();
        let b = y * 1.267_f32 - x.cos();
        let mut acc = Accumulator816::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_816(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_816() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_816(total as u64) % 997) as f32;
        total
    }
}

pub mod m817 {
    use super::*;

    pub struct Accumulator817<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator817<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.024_f32 + y.sin();
        let b = y * 1.348_f32 - x.cos();
        let mut acc = Accumulator817::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.3_f32 + y.sin();
        let b = y * 1.09_f32 - x.cos();
        let mut acc = Accumulator817::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.898_f32 + y.sin();
        let b = y * 9.675_f32 - x.cos();
        let mut acc = Accumulator817::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.067_f32 + y.sin();
        let b = y * 5.614_f32 - x.cos();
        let mut acc = Accumulator817::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.25_f32 + y.sin();
        let b = y * 8.004_f32 - x.cos();
        let mut acc = Accumulator817::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.926_f32 + y.sin();
        let b = y * 6.662_f32 - x.cos();
        let mut acc = Accumulator817::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.257_f32 + y.sin();
        let b = y * 2.962_f32 - x.cos();
        let mut acc = Accumulator817::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.651_f32 + y.sin();
        let b = y * 0.873_f32 - x.cos();
        let mut acc = Accumulator817::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.101_f32 + y.sin();
        let b = y * 9.539_f32 - x.cos();
        let mut acc = Accumulator817::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.54_f32 + y.sin();
        let b = y * 1.886_f32 - x.cos();
        let mut acc = Accumulator817::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.648_f32 + y.sin();
        let b = y * 1.271_f32 - x.cos();
        let mut acc = Accumulator817::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.802_f32 + y.sin();
        let b = y * 8.717_f32 - x.cos();
        let mut acc = Accumulator817::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.243_f32 + y.sin();
        let b = y * 6.229_f32 - x.cos();
        let mut acc = Accumulator817::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.415_f32 + y.sin();
        let b = y * 9.375_f32 - x.cos();
        let mut acc = Accumulator817::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.165_f32 + y.sin();
        let b = y * 0.135_f32 - x.cos();
        let mut acc = Accumulator817::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.357_f32 + y.sin();
        let b = y * 7.735_f32 - x.cos();
        let mut acc = Accumulator817::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.854_f32 + y.sin();
        let b = y * 0.348_f32 - x.cos();
        let mut acc = Accumulator817::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.569_f32 + y.sin();
        let b = y * 5.302_f32 - x.cos();
        let mut acc = Accumulator817::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.192_f32 + y.sin();
        let b = y * 4.416_f32 - x.cos();
        let mut acc = Accumulator817::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.42_f32 + y.sin();
        let b = y * 2.789_f32 - x.cos();
        let mut acc = Accumulator817::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_817(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(817u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_817() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_817(total as u64) % 997) as f32;
        total
    }
}

pub mod m818 {
    use super::*;

    pub struct Accumulator818<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator818<T> {
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
        let b = y * 9.88_f32 - x.cos();
        let mut acc = Accumulator818::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.809_f32 + y.sin();
        let b = y * 4.587_f32 - x.cos();
        let mut acc = Accumulator818::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.445_f32 + y.sin();
        let b = y * 6.71_f32 - x.cos();
        let mut acc = Accumulator818::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.058_f32 + y.sin();
        let b = y * 8.569_f32 - x.cos();
        let mut acc = Accumulator818::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.236_f32 + y.sin();
        let b = y * 7.418_f32 - x.cos();
        let mut acc = Accumulator818::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.525_f32 + y.sin();
        let b = y * 0.344_f32 - x.cos();
        let mut acc = Accumulator818::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.924_f32 + y.sin();
        let b = y * 9.372_f32 - x.cos();
        let mut acc = Accumulator818::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.934_f32 + y.sin();
        let b = y * 6.13_f32 - x.cos();
        let mut acc = Accumulator818::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.892_f32 + y.sin();
        let b = y * 2.102_f32 - x.cos();
        let mut acc = Accumulator818::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.993_f32 + y.sin();
        let b = y * 0.858_f32 - x.cos();
        let mut acc = Accumulator818::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.591_f32 + y.sin();
        let b = y * 9.09_f32 - x.cos();
        let mut acc = Accumulator818::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.276_f32 + y.sin();
        let b = y * 7.229_f32 - x.cos();
        let mut acc = Accumulator818::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.919_f32 + y.sin();
        let b = y * 8.818_f32 - x.cos();
        let mut acc = Accumulator818::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.788_f32 + y.sin();
        let b = y * 3.576_f32 - x.cos();
        let mut acc = Accumulator818::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.386_f32 + y.sin();
        let b = y * 7.823_f32 - x.cos();
        let mut acc = Accumulator818::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.6_f32 + y.sin();
        let b = y * 2.038_f32 - x.cos();
        let mut acc = Accumulator818::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.539_f32 + y.sin();
        let b = y * 9.009_f32 - x.cos();
        let mut acc = Accumulator818::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.367_f32 + y.sin();
        let b = y * 2.022_f32 - x.cos();
        let mut acc = Accumulator818::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.249_f32 + y.sin();
        let b = y * 9.108_f32 - x.cos();
        let mut acc = Accumulator818::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.427_f32 + y.sin();
        let b = y * 5.494_f32 - x.cos();
        let mut acc = Accumulator818::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_818(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_818() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_818(total as u64) % 997) as f32;
        total
    }
}

pub mod m819 {
    use super::*;

    pub struct Accumulator819<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator819<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.855_f32 + y.sin();
        let b = y * 4.344_f32 - x.cos();
        let mut acc = Accumulator819::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.935_f32 + y.sin();
        let b = y * 1.135_f32 - x.cos();
        let mut acc = Accumulator819::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.709_f32 + y.sin();
        let b = y * 1.929_f32 - x.cos();
        let mut acc = Accumulator819::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.1_f32 + y.sin();
        let b = y * 1.912_f32 - x.cos();
        let mut acc = Accumulator819::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.792_f32 + y.sin();
        let b = y * 3.697_f32 - x.cos();
        let mut acc = Accumulator819::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.462_f32 + y.sin();
        let b = y * 4.397_f32 - x.cos();
        let mut acc = Accumulator819::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.786_f32 + y.sin();
        let b = y * 0.469_f32 - x.cos();
        let mut acc = Accumulator819::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.114_f32 + y.sin();
        let b = y * 5.256_f32 - x.cos();
        let mut acc = Accumulator819::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.772_f32 + y.sin();
        let b = y * 1.806_f32 - x.cos();
        let mut acc = Accumulator819::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.245_f32 + y.sin();
        let b = y * 0.607_f32 - x.cos();
        let mut acc = Accumulator819::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.941_f32 + y.sin();
        let b = y * 7.28_f32 - x.cos();
        let mut acc = Accumulator819::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.074_f32 + y.sin();
        let b = y * 2.418_f32 - x.cos();
        let mut acc = Accumulator819::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.974_f32 + y.sin();
        let b = y * 5.965_f32 - x.cos();
        let mut acc = Accumulator819::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.715_f32 + y.sin();
        let b = y * 3.302_f32 - x.cos();
        let mut acc = Accumulator819::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.108_f32 + y.sin();
        let b = y * 4.473_f32 - x.cos();
        let mut acc = Accumulator819::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.137_f32 + y.sin();
        let b = y * 9.663_f32 - x.cos();
        let mut acc = Accumulator819::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.275_f32 + y.sin();
        let b = y * 6.872_f32 - x.cos();
        let mut acc = Accumulator819::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.647_f32 + y.sin();
        let b = y * 6.647_f32 - x.cos();
        let mut acc = Accumulator819::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.037_f32 + y.sin();
        let b = y * 3.623_f32 - x.cos();
        let mut acc = Accumulator819::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.155_f32 + y.sin();
        let b = y * 9.264_f32 - x.cos();
        let mut acc = Accumulator819::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_819(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_819() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_819(total as u64) % 997) as f32;
        total
    }
}

pub mod m820 {
    use super::*;

    pub struct Accumulator820<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator820<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.779_f32 + y.sin();
        let b = y * 5.036_f32 - x.cos();
        let mut acc = Accumulator820::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.737_f32 + y.sin();
        let b = y * 2.833_f32 - x.cos();
        let mut acc = Accumulator820::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.596_f32 + y.sin();
        let b = y * 2.922_f32 - x.cos();
        let mut acc = Accumulator820::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.231_f32 + y.sin();
        let b = y * 2.701_f32 - x.cos();
        let mut acc = Accumulator820::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.893_f32 + y.sin();
        let b = y * 0.189_f32 - x.cos();
        let mut acc = Accumulator820::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.205_f32 + y.sin();
        let b = y * 2.552_f32 - x.cos();
        let mut acc = Accumulator820::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.995_f32 + y.sin();
        let b = y * 3.56_f32 - x.cos();
        let mut acc = Accumulator820::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.823_f32 + y.sin();
        let b = y * 6.359_f32 - x.cos();
        let mut acc = Accumulator820::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.916_f32 + y.sin();
        let b = y * 8.947_f32 - x.cos();
        let mut acc = Accumulator820::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.856_f32 + y.sin();
        let b = y * 9.066_f32 - x.cos();
        let mut acc = Accumulator820::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.452_f32 + y.sin();
        let b = y * 8.286_f32 - x.cos();
        let mut acc = Accumulator820::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.209_f32 + y.sin();
        let b = y * 9.341_f32 - x.cos();
        let mut acc = Accumulator820::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.118_f32 + y.sin();
        let b = y * 2.735_f32 - x.cos();
        let mut acc = Accumulator820::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.594_f32 + y.sin();
        let b = y * 9.898_f32 - x.cos();
        let mut acc = Accumulator820::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.761_f32 + y.sin();
        let b = y * 6.835_f32 - x.cos();
        let mut acc = Accumulator820::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.808_f32 + y.sin();
        let b = y * 0.983_f32 - x.cos();
        let mut acc = Accumulator820::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.651_f32 + y.sin();
        let b = y * 5.469_f32 - x.cos();
        let mut acc = Accumulator820::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.001_f32 + y.sin();
        let b = y * 8.763_f32 - x.cos();
        let mut acc = Accumulator820::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.688_f32 + y.sin();
        let b = y * 8.277_f32 - x.cos();
        let mut acc = Accumulator820::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.088_f32 + y.sin();
        let b = y * 8.916_f32 - x.cos();
        let mut acc = Accumulator820::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_820(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m820-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_820() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_820(total as u64) % 997) as f32;
        total
    }
}

pub mod m821 {
    use super::*;

    pub struct Accumulator821<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator821<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.22_f32 + y.sin();
        let b = y * 6.097_f32 - x.cos();
        let mut acc = Accumulator821::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.446_f32 + y.sin();
        let b = y * 0.499_f32 - x.cos();
        let mut acc = Accumulator821::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.211_f32 + y.sin();
        let b = y * 6.279_f32 - x.cos();
        let mut acc = Accumulator821::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.861_f32 + y.sin();
        let b = y * 9.251_f32 - x.cos();
        let mut acc = Accumulator821::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.953_f32 + y.sin();
        let b = y * 0.989_f32 - x.cos();
        let mut acc = Accumulator821::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.937_f32 + y.sin();
        let b = y * 8.871_f32 - x.cos();
        let mut acc = Accumulator821::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.696_f32 + y.sin();
        let b = y * 9.852_f32 - x.cos();
        let mut acc = Accumulator821::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.09_f32 + y.sin();
        let b = y * 9.414_f32 - x.cos();
        let mut acc = Accumulator821::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.172_f32 + y.sin();
        let b = y * 2.911_f32 - x.cos();
        let mut acc = Accumulator821::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.185_f32 + y.sin();
        let b = y * 1.593_f32 - x.cos();
        let mut acc = Accumulator821::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.486_f32 + y.sin();
        let b = y * 6.157_f32 - x.cos();
        let mut acc = Accumulator821::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.693_f32 + y.sin();
        let b = y * 5.563_f32 - x.cos();
        let mut acc = Accumulator821::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.31_f32 + y.sin();
        let b = y * 8.172_f32 - x.cos();
        let mut acc = Accumulator821::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.11_f32 + y.sin();
        let b = y * 7.459_f32 - x.cos();
        let mut acc = Accumulator821::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.082_f32 + y.sin();
        let b = y * 6.443_f32 - x.cos();
        let mut acc = Accumulator821::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.391_f32 + y.sin();
        let b = y * 6.754_f32 - x.cos();
        let mut acc = Accumulator821::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.99_f32 + y.sin();
        let b = y * 9.645_f32 - x.cos();
        let mut acc = Accumulator821::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.558_f32 + y.sin();
        let b = y * 8.075_f32 - x.cos();
        let mut acc = Accumulator821::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.319_f32 + y.sin();
        let b = y * 7.799_f32 - x.cos();
        let mut acc = Accumulator821::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.369_f32 + y.sin();
        let b = y * 4.987_f32 - x.cos();
        let mut acc = Accumulator821::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_821(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_821() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_821(total as u64) % 997) as f32;
        total
    }
}

pub mod m822 {
    use super::*;

    pub struct Accumulator822<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator822<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.886_f32 + y.sin();
        let b = y * 7.02_f32 - x.cos();
        let mut acc = Accumulator822::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.258_f32 + y.sin();
        let b = y * 9.703_f32 - x.cos();
        let mut acc = Accumulator822::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.414_f32 + y.sin();
        let b = y * 6.734_f32 - x.cos();
        let mut acc = Accumulator822::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.647_f32 + y.sin();
        let b = y * 0.734_f32 - x.cos();
        let mut acc = Accumulator822::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.708_f32 + y.sin();
        let b = y * 6.354_f32 - x.cos();
        let mut acc = Accumulator822::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.868_f32 + y.sin();
        let b = y * 6.043_f32 - x.cos();
        let mut acc = Accumulator822::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.999_f32 + y.sin();
        let b = y * 9.327_f32 - x.cos();
        let mut acc = Accumulator822::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.112_f32 + y.sin();
        let b = y * 4.666_f32 - x.cos();
        let mut acc = Accumulator822::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.218_f32 + y.sin();
        let b = y * 4.769_f32 - x.cos();
        let mut acc = Accumulator822::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.662_f32 + y.sin();
        let b = y * 6.896_f32 - x.cos();
        let mut acc = Accumulator822::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.802_f32 + y.sin();
        let b = y * 4.392_f32 - x.cos();
        let mut acc = Accumulator822::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.47_f32 + y.sin();
        let b = y * 1.872_f32 - x.cos();
        let mut acc = Accumulator822::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.747_f32 + y.sin();
        let b = y * 4.587_f32 - x.cos();
        let mut acc = Accumulator822::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.522_f32 + y.sin();
        let b = y * 2.509_f32 - x.cos();
        let mut acc = Accumulator822::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.315_f32 + y.sin();
        let b = y * 6.042_f32 - x.cos();
        let mut acc = Accumulator822::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.404_f32 + y.sin();
        let b = y * 0.502_f32 - x.cos();
        let mut acc = Accumulator822::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.775_f32 + y.sin();
        let b = y * 6.294_f32 - x.cos();
        let mut acc = Accumulator822::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.297_f32 + y.sin();
        let b = y * 5.594_f32 - x.cos();
        let mut acc = Accumulator822::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.835_f32 + y.sin();
        let b = y * 1.258_f32 - x.cos();
        let mut acc = Accumulator822::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.592_f32 + y.sin();
        let b = y * 7.583_f32 - x.cos();
        let mut acc = Accumulator822::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_822(seed: u64) -> u64 {
        let re = Regex::new(r"m822-(\d+)").unwrap();
        let hay = format!("m822-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_822() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_822(total as u64) % 997) as f32;
        total
    }
}

pub mod m823 {
    use super::*;

    pub struct Accumulator823<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator823<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.201_f32 + y.sin();
        let b = y * 8.502_f32 - x.cos();
        let mut acc = Accumulator823::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.901_f32 + y.sin();
        let b = y * 5.69_f32 - x.cos();
        let mut acc = Accumulator823::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.947_f32 + y.sin();
        let b = y * 6.23_f32 - x.cos();
        let mut acc = Accumulator823::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.054_f32 + y.sin();
        let b = y * 4.692_f32 - x.cos();
        let mut acc = Accumulator823::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.922_f32 + y.sin();
        let b = y * 1.796_f32 - x.cos();
        let mut acc = Accumulator823::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.94_f32 + y.sin();
        let b = y * 0.87_f32 - x.cos();
        let mut acc = Accumulator823::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.221_f32 + y.sin();
        let b = y * 8.357_f32 - x.cos();
        let mut acc = Accumulator823::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.655_f32 + y.sin();
        let b = y * 4.214_f32 - x.cos();
        let mut acc = Accumulator823::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.299_f32 + y.sin();
        let b = y * 0.496_f32 - x.cos();
        let mut acc = Accumulator823::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.082_f32 + y.sin();
        let b = y * 8.258_f32 - x.cos();
        let mut acc = Accumulator823::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.694_f32 + y.sin();
        let b = y * 3.723_f32 - x.cos();
        let mut acc = Accumulator823::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.073_f32 + y.sin();
        let b = y * 2.519_f32 - x.cos();
        let mut acc = Accumulator823::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.043_f32 + y.sin();
        let b = y * 8.396_f32 - x.cos();
        let mut acc = Accumulator823::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.904_f32 + y.sin();
        let b = y * 2.509_f32 - x.cos();
        let mut acc = Accumulator823::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.439_f32 + y.sin();
        let b = y * 1.586_f32 - x.cos();
        let mut acc = Accumulator823::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.908_f32 + y.sin();
        let b = y * 3.258_f32 - x.cos();
        let mut acc = Accumulator823::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.282_f32 + y.sin();
        let b = y * 6.919_f32 - x.cos();
        let mut acc = Accumulator823::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.706_f32 + y.sin();
        let b = y * 0.722_f32 - x.cos();
        let mut acc = Accumulator823::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.781_f32 + y.sin();
        let b = y * 4.204_f32 - x.cos();
        let mut acc = Accumulator823::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.706_f32 + y.sin();
        let b = y * 4.434_f32 - x.cos();
        let mut acc = Accumulator823::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_823(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_823() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_823(total as u64) % 997) as f32;
        total
    }
}

pub mod m824 {
    use super::*;

    pub struct Accumulator824<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator824<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.852_f32 + y.sin();
        let b = y * 1.276_f32 - x.cos();
        let mut acc = Accumulator824::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.13_f32 + y.sin();
        let b = y * 5.487_f32 - x.cos();
        let mut acc = Accumulator824::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.529_f32 + y.sin();
        let b = y * 1.823_f32 - x.cos();
        let mut acc = Accumulator824::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.363_f32 + y.sin();
        let b = y * 1.56_f32 - x.cos();
        let mut acc = Accumulator824::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.092_f32 + y.sin();
        let b = y * 8.505_f32 - x.cos();
        let mut acc = Accumulator824::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.581_f32 + y.sin();
        let b = y * 3.061_f32 - x.cos();
        let mut acc = Accumulator824::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.289_f32 + y.sin();
        let b = y * 9.847_f32 - x.cos();
        let mut acc = Accumulator824::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.437_f32 + y.sin();
        let b = y * 1.724_f32 - x.cos();
        let mut acc = Accumulator824::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.803_f32 + y.sin();
        let b = y * 9.754_f32 - x.cos();
        let mut acc = Accumulator824::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.698_f32 + y.sin();
        let b = y * 3.074_f32 - x.cos();
        let mut acc = Accumulator824::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.854_f32 + y.sin();
        let b = y * 1.883_f32 - x.cos();
        let mut acc = Accumulator824::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.395_f32 + y.sin();
        let b = y * 6.721_f32 - x.cos();
        let mut acc = Accumulator824::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.619_f32 + y.sin();
        let b = y * 6.777_f32 - x.cos();
        let mut acc = Accumulator824::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.252_f32 + y.sin();
        let b = y * 5.952_f32 - x.cos();
        let mut acc = Accumulator824::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.25_f32 + y.sin();
        let b = y * 9.017_f32 - x.cos();
        let mut acc = Accumulator824::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.752_f32 + y.sin();
        let b = y * 6.228_f32 - x.cos();
        let mut acc = Accumulator824::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.36_f32 + y.sin();
        let b = y * 0.514_f32 - x.cos();
        let mut acc = Accumulator824::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.973_f32 + y.sin();
        let b = y * 1.872_f32 - x.cos();
        let mut acc = Accumulator824::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.468_f32 + y.sin();
        let b = y * 7.596_f32 - x.cos();
        let mut acc = Accumulator824::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.2_f32 + y.sin();
        let b = y * 3.618_f32 - x.cos();
        let mut acc = Accumulator824::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_824(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(824u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_824() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_824(total as u64) % 997) as f32;
        total
    }
}

pub mod m825 {
    use super::*;

    pub struct Accumulator825<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator825<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.599_f32 + y.sin();
        let b = y * 0.707_f32 - x.cos();
        let mut acc = Accumulator825::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.224_f32 + y.sin();
        let b = y * 0.369_f32 - x.cos();
        let mut acc = Accumulator825::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.315_f32 + y.sin();
        let b = y * 9.755_f32 - x.cos();
        let mut acc = Accumulator825::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.896_f32 + y.sin();
        let b = y * 1.181_f32 - x.cos();
        let mut acc = Accumulator825::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.853_f32 + y.sin();
        let b = y * 4.139_f32 - x.cos();
        let mut acc = Accumulator825::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.127_f32 + y.sin();
        let b = y * 8.903_f32 - x.cos();
        let mut acc = Accumulator825::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.064_f32 + y.sin();
        let b = y * 6.824_f32 - x.cos();
        let mut acc = Accumulator825::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.506_f32 + y.sin();
        let b = y * 2.145_f32 - x.cos();
        let mut acc = Accumulator825::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.214_f32 + y.sin();
        let b = y * 2.802_f32 - x.cos();
        let mut acc = Accumulator825::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.517_f32 + y.sin();
        let b = y * 6.1_f32 - x.cos();
        let mut acc = Accumulator825::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.892_f32 + y.sin();
        let b = y * 0.65_f32 - x.cos();
        let mut acc = Accumulator825::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.008_f32 + y.sin();
        let b = y * 3.984_f32 - x.cos();
        let mut acc = Accumulator825::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.388_f32 + y.sin();
        let b = y * 4.617_f32 - x.cos();
        let mut acc = Accumulator825::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.584_f32 + y.sin();
        let b = y * 1.762_f32 - x.cos();
        let mut acc = Accumulator825::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.987_f32 + y.sin();
        let b = y * 1.124_f32 - x.cos();
        let mut acc = Accumulator825::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.3_f32 + y.sin();
        let b = y * 5.574_f32 - x.cos();
        let mut acc = Accumulator825::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.03_f32 + y.sin();
        let b = y * 7.036_f32 - x.cos();
        let mut acc = Accumulator825::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.08_f32 + y.sin();
        let b = y * 3.701_f32 - x.cos();
        let mut acc = Accumulator825::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.854_f32 + y.sin();
        let b = y * 8.299_f32 - x.cos();
        let mut acc = Accumulator825::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.011_f32 + y.sin();
        let b = y * 5.281_f32 - x.cos();
        let mut acc = Accumulator825::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_825(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_825() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_825(total as u64) % 997) as f32;
        total
    }
}

pub mod m826 {
    use super::*;

    pub struct Accumulator826<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator826<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.389_f32 + y.sin();
        let b = y * 4.006_f32 - x.cos();
        let mut acc = Accumulator826::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.585_f32 + y.sin();
        let b = y * 0.382_f32 - x.cos();
        let mut acc = Accumulator826::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.469_f32 + y.sin();
        let b = y * 4.667_f32 - x.cos();
        let mut acc = Accumulator826::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.489_f32 + y.sin();
        let b = y * 3.523_f32 - x.cos();
        let mut acc = Accumulator826::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.025_f32 + y.sin();
        let b = y * 3.866_f32 - x.cos();
        let mut acc = Accumulator826::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.578_f32 + y.sin();
        let b = y * 6.495_f32 - x.cos();
        let mut acc = Accumulator826::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.487_f32 + y.sin();
        let b = y * 8.521_f32 - x.cos();
        let mut acc = Accumulator826::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.573_f32 + y.sin();
        let b = y * 3.16_f32 - x.cos();
        let mut acc = Accumulator826::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.51_f32 + y.sin();
        let b = y * 7.668_f32 - x.cos();
        let mut acc = Accumulator826::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.557_f32 + y.sin();
        let b = y * 6.178_f32 - x.cos();
        let mut acc = Accumulator826::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.29_f32 + y.sin();
        let b = y * 0.157_f32 - x.cos();
        let mut acc = Accumulator826::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.21_f32 + y.sin();
        let b = y * 6.669_f32 - x.cos();
        let mut acc = Accumulator826::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.036_f32 + y.sin();
        let b = y * 1.61_f32 - x.cos();
        let mut acc = Accumulator826::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.021_f32 + y.sin();
        let b = y * 2.518_f32 - x.cos();
        let mut acc = Accumulator826::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.968_f32 + y.sin();
        let b = y * 2.976_f32 - x.cos();
        let mut acc = Accumulator826::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.738_f32 + y.sin();
        let b = y * 0.899_f32 - x.cos();
        let mut acc = Accumulator826::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.553_f32 + y.sin();
        let b = y * 9.547_f32 - x.cos();
        let mut acc = Accumulator826::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 9.758_f32 + y.sin();
        let b = y * 4.657_f32 - x.cos();
        let mut acc = Accumulator826::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.531_f32 + y.sin();
        let b = y * 5.462_f32 - x.cos();
        let mut acc = Accumulator826::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.409_f32 + y.sin();
        let b = y * 4.686_f32 - x.cos();
        let mut acc = Accumulator826::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_826(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_826() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_826(total as u64) % 997) as f32;
        total
    }
}

pub mod m827 {
    use super::*;

    pub struct Accumulator827<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator827<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.531_f32 + y.sin();
        let b = y * 7.781_f32 - x.cos();
        let mut acc = Accumulator827::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.827_f32 + y.sin();
        let b = y * 7.188_f32 - x.cos();
        let mut acc = Accumulator827::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.33_f32 + y.sin();
        let b = y * 7.102_f32 - x.cos();
        let mut acc = Accumulator827::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.179_f32 + y.sin();
        let b = y * 2.276_f32 - x.cos();
        let mut acc = Accumulator827::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.641_f32 + y.sin();
        let b = y * 8.47_f32 - x.cos();
        let mut acc = Accumulator827::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.631_f32 + y.sin();
        let b = y * 3.562_f32 - x.cos();
        let mut acc = Accumulator827::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.676_f32 + y.sin();
        let b = y * 2.49_f32 - x.cos();
        let mut acc = Accumulator827::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.221_f32 + y.sin();
        let b = y * 7.99_f32 - x.cos();
        let mut acc = Accumulator827::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.929_f32 + y.sin();
        let b = y * 4.523_f32 - x.cos();
        let mut acc = Accumulator827::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.4_f32 + y.sin();
        let b = y * 5.908_f32 - x.cos();
        let mut acc = Accumulator827::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.948_f32 + y.sin();
        let b = y * 1.618_f32 - x.cos();
        let mut acc = Accumulator827::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.838_f32 + y.sin();
        let b = y * 5.003_f32 - x.cos();
        let mut acc = Accumulator827::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.482_f32 + y.sin();
        let b = y * 7.962_f32 - x.cos();
        let mut acc = Accumulator827::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.098_f32 + y.sin();
        let b = y * 7.987_f32 - x.cos();
        let mut acc = Accumulator827::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.861_f32 + y.sin();
        let b = y * 1.13_f32 - x.cos();
        let mut acc = Accumulator827::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.023_f32 + y.sin();
        let b = y * 5.541_f32 - x.cos();
        let mut acc = Accumulator827::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.887_f32 + y.sin();
        let b = y * 6.733_f32 - x.cos();
        let mut acc = Accumulator827::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.983_f32 + y.sin();
        let b = y * 0.123_f32 - x.cos();
        let mut acc = Accumulator827::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.216_f32 + y.sin();
        let b = y * 4.184_f32 - x.cos();
        let mut acc = Accumulator827::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.3_f32 + y.sin();
        let b = y * 4.636_f32 - x.cos();
        let mut acc = Accumulator827::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_827(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m827-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_827() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_827(total as u64) % 997) as f32;
        total
    }
}

pub mod m828 {
    use super::*;

    pub struct Accumulator828<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator828<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.716_f32 + y.sin();
        let b = y * 5.258_f32 - x.cos();
        let mut acc = Accumulator828::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.199_f32 + y.sin();
        let b = y * 4.801_f32 - x.cos();
        let mut acc = Accumulator828::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.84_f32 + y.sin();
        let b = y * 5.493_f32 - x.cos();
        let mut acc = Accumulator828::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 9.404_f32 + y.sin();
        let b = y * 6.622_f32 - x.cos();
        let mut acc = Accumulator828::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.152_f32 + y.sin();
        let b = y * 2.352_f32 - x.cos();
        let mut acc = Accumulator828::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.503_f32 + y.sin();
        let b = y * 4.589_f32 - x.cos();
        let mut acc = Accumulator828::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.238_f32 + y.sin();
        let b = y * 0.303_f32 - x.cos();
        let mut acc = Accumulator828::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.11_f32 + y.sin();
        let b = y * 5.852_f32 - x.cos();
        let mut acc = Accumulator828::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.346_f32 + y.sin();
        let b = y * 5.735_f32 - x.cos();
        let mut acc = Accumulator828::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.453_f32 + y.sin();
        let b = y * 3.449_f32 - x.cos();
        let mut acc = Accumulator828::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.129_f32 + y.sin();
        let b = y * 2.047_f32 - x.cos();
        let mut acc = Accumulator828::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.831_f32 + y.sin();
        let b = y * 4.153_f32 - x.cos();
        let mut acc = Accumulator828::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.585_f32 + y.sin();
        let b = y * 3.131_f32 - x.cos();
        let mut acc = Accumulator828::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.792_f32 + y.sin();
        let b = y * 0.872_f32 - x.cos();
        let mut acc = Accumulator828::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.375_f32 + y.sin();
        let b = y * 1.872_f32 - x.cos();
        let mut acc = Accumulator828::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.031_f32 + y.sin();
        let b = y * 5.753_f32 - x.cos();
        let mut acc = Accumulator828::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.787_f32 + y.sin();
        let b = y * 4.228_f32 - x.cos();
        let mut acc = Accumulator828::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.885_f32 + y.sin();
        let b = y * 7.543_f32 - x.cos();
        let mut acc = Accumulator828::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.844_f32 + y.sin();
        let b = y * 0.768_f32 - x.cos();
        let mut acc = Accumulator828::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.644_f32 + y.sin();
        let b = y * 8.557_f32 - x.cos();
        let mut acc = Accumulator828::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_828(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_828() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_828(total as u64) % 997) as f32;
        total
    }
}

pub mod m829 {
    use super::*;

    pub struct Accumulator829<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator829<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.034_f32 + y.sin();
        let b = y * 9.833_f32 - x.cos();
        let mut acc = Accumulator829::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.213_f32 + y.sin();
        let b = y * 3.71_f32 - x.cos();
        let mut acc = Accumulator829::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.866_f32 + y.sin();
        let b = y * 2.959_f32 - x.cos();
        let mut acc = Accumulator829::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.465_f32 + y.sin();
        let b = y * 6.149_f32 - x.cos();
        let mut acc = Accumulator829::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.934_f32 + y.sin();
        let b = y * 1.823_f32 - x.cos();
        let mut acc = Accumulator829::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.812_f32 + y.sin();
        let b = y * 1.921_f32 - x.cos();
        let mut acc = Accumulator829::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.867_f32 + y.sin();
        let b = y * 8.685_f32 - x.cos();
        let mut acc = Accumulator829::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.327_f32 + y.sin();
        let b = y * 6.318_f32 - x.cos();
        let mut acc = Accumulator829::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.29_f32 + y.sin();
        let b = y * 7.315_f32 - x.cos();
        let mut acc = Accumulator829::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.781_f32 + y.sin();
        let b = y * 1.6_f32 - x.cos();
        let mut acc = Accumulator829::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.255_f32 + y.sin();
        let b = y * 0.76_f32 - x.cos();
        let mut acc = Accumulator829::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.757_f32 + y.sin();
        let b = y * 3.19_f32 - x.cos();
        let mut acc = Accumulator829::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.195_f32 + y.sin();
        let b = y * 4.392_f32 - x.cos();
        let mut acc = Accumulator829::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.131_f32 + y.sin();
        let b = y * 0.278_f32 - x.cos();
        let mut acc = Accumulator829::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.19_f32 + y.sin();
        let b = y * 4.639_f32 - x.cos();
        let mut acc = Accumulator829::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.298_f32 + y.sin();
        let b = y * 2.493_f32 - x.cos();
        let mut acc = Accumulator829::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.663_f32 + y.sin();
        let b = y * 7.519_f32 - x.cos();
        let mut acc = Accumulator829::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.36_f32 + y.sin();
        let b = y * 2.139_f32 - x.cos();
        let mut acc = Accumulator829::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.301_f32 + y.sin();
        let b = y * 9.598_f32 - x.cos();
        let mut acc = Accumulator829::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.658_f32 + y.sin();
        let b = y * 6.882_f32 - x.cos();
        let mut acc = Accumulator829::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_829(seed: u64) -> u64 {
        let re = Regex::new(r"m829-(\d+)").unwrap();
        let hay = format!("m829-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_829() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_829(total as u64) % 997) as f32;
        total
    }
}

pub mod m830 {
    use super::*;

    pub struct Accumulator830<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator830<T> {
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
        let b = y * 3.441_f32 - x.cos();
        let mut acc = Accumulator830::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.351_f32 + y.sin();
        let b = y * 2.124_f32 - x.cos();
        let mut acc = Accumulator830::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.219_f32 + y.sin();
        let b = y * 3.413_f32 - x.cos();
        let mut acc = Accumulator830::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.491_f32 + y.sin();
        let b = y * 5.752_f32 - x.cos();
        let mut acc = Accumulator830::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.661_f32 + y.sin();
        let b = y * 6.175_f32 - x.cos();
        let mut acc = Accumulator830::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.821_f32 + y.sin();
        let b = y * 6.935_f32 - x.cos();
        let mut acc = Accumulator830::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.49_f32 + y.sin();
        let b = y * 6.943_f32 - x.cos();
        let mut acc = Accumulator830::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.841_f32 + y.sin();
        let b = y * 8.024_f32 - x.cos();
        let mut acc = Accumulator830::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.464_f32 + y.sin();
        let b = y * 5.496_f32 - x.cos();
        let mut acc = Accumulator830::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.966_f32 + y.sin();
        let b = y * 4.711_f32 - x.cos();
        let mut acc = Accumulator830::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.797_f32 + y.sin();
        let b = y * 0.504_f32 - x.cos();
        let mut acc = Accumulator830::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.257_f32 + y.sin();
        let b = y * 9.201_f32 - x.cos();
        let mut acc = Accumulator830::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.757_f32 + y.sin();
        let b = y * 7.792_f32 - x.cos();
        let mut acc = Accumulator830::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.354_f32 + y.sin();
        let b = y * 5.943_f32 - x.cos();
        let mut acc = Accumulator830::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.253_f32 + y.sin();
        let b = y * 3.854_f32 - x.cos();
        let mut acc = Accumulator830::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.493_f32 + y.sin();
        let b = y * 6.496_f32 - x.cos();
        let mut acc = Accumulator830::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.012_f32 + y.sin();
        let b = y * 1.792_f32 - x.cos();
        let mut acc = Accumulator830::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.659_f32 + y.sin();
        let b = y * 3.611_f32 - x.cos();
        let mut acc = Accumulator830::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.565_f32 + y.sin();
        let b = y * 7.283_f32 - x.cos();
        let mut acc = Accumulator830::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.699_f32 + y.sin();
        let b = y * 7.185_f32 - x.cos();
        let mut acc = Accumulator830::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_830(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_830() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_830(total as u64) % 997) as f32;
        total
    }
}

pub mod m831 {
    use super::*;

    pub struct Accumulator831<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator831<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.961_f32 + y.sin();
        let b = y * 7.02_f32 - x.cos();
        let mut acc = Accumulator831::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.62_f32 + y.sin();
        let b = y * 5.419_f32 - x.cos();
        let mut acc = Accumulator831::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.22_f32 + y.sin();
        let b = y * 6.948_f32 - x.cos();
        let mut acc = Accumulator831::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.536_f32 + y.sin();
        let b = y * 9.64_f32 - x.cos();
        let mut acc = Accumulator831::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.061_f32 + y.sin();
        let b = y * 1.556_f32 - x.cos();
        let mut acc = Accumulator831::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.841_f32 + y.sin();
        let b = y * 9.685_f32 - x.cos();
        let mut acc = Accumulator831::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.03_f32 + y.sin();
        let b = y * 7.37_f32 - x.cos();
        let mut acc = Accumulator831::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.663_f32 + y.sin();
        let b = y * 0.84_f32 - x.cos();
        let mut acc = Accumulator831::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.828_f32 + y.sin();
        let b = y * 8.124_f32 - x.cos();
        let mut acc = Accumulator831::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.205_f32 + y.sin();
        let b = y * 1.864_f32 - x.cos();
        let mut acc = Accumulator831::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.456_f32 + y.sin();
        let b = y * 0.503_f32 - x.cos();
        let mut acc = Accumulator831::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.162_f32 + y.sin();
        let b = y * 9.187_f32 - x.cos();
        let mut acc = Accumulator831::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.894_f32 + y.sin();
        let b = y * 2.152_f32 - x.cos();
        let mut acc = Accumulator831::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.432_f32 + y.sin();
        let b = y * 4.623_f32 - x.cos();
        let mut acc = Accumulator831::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.949_f32 + y.sin();
        let b = y * 0.92_f32 - x.cos();
        let mut acc = Accumulator831::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.838_f32 + y.sin();
        let b = y * 7.55_f32 - x.cos();
        let mut acc = Accumulator831::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.429_f32 + y.sin();
        let b = y * 4.42_f32 - x.cos();
        let mut acc = Accumulator831::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.897_f32 + y.sin();
        let b = y * 6.046_f32 - x.cos();
        let mut acc = Accumulator831::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.553_f32 + y.sin();
        let b = y * 1.743_f32 - x.cos();
        let mut acc = Accumulator831::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.885_f32 + y.sin();
        let b = y * 8.361_f32 - x.cos();
        let mut acc = Accumulator831::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_831(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(831u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_831() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_831(total as u64) % 997) as f32;
        total
    }
}

pub mod m832 {
    use super::*;

    pub struct Accumulator832<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator832<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.95_f32 + y.sin();
        let b = y * 2.212_f32 - x.cos();
        let mut acc = Accumulator832::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.699_f32 + y.sin();
        let b = y * 0.657_f32 - x.cos();
        let mut acc = Accumulator832::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.873_f32 + y.sin();
        let b = y * 6.94_f32 - x.cos();
        let mut acc = Accumulator832::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.129_f32 + y.sin();
        let b = y * 5.613_f32 - x.cos();
        let mut acc = Accumulator832::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.33_f32 + y.sin();
        let b = y * 5.113_f32 - x.cos();
        let mut acc = Accumulator832::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.926_f32 + y.sin();
        let b = y * 8.712_f32 - x.cos();
        let mut acc = Accumulator832::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.124_f32 + y.sin();
        let b = y * 7.964_f32 - x.cos();
        let mut acc = Accumulator832::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.723_f32 + y.sin();
        let b = y * 4.875_f32 - x.cos();
        let mut acc = Accumulator832::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.804_f32 + y.sin();
        let b = y * 2.117_f32 - x.cos();
        let mut acc = Accumulator832::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.628_f32 + y.sin();
        let b = y * 2.618_f32 - x.cos();
        let mut acc = Accumulator832::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.357_f32 + y.sin();
        let b = y * 5.327_f32 - x.cos();
        let mut acc = Accumulator832::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.875_f32 + y.sin();
        let b = y * 9.346_f32 - x.cos();
        let mut acc = Accumulator832::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.464_f32 + y.sin();
        let b = y * 4.543_f32 - x.cos();
        let mut acc = Accumulator832::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.746_f32 + y.sin();
        let b = y * 3.589_f32 - x.cos();
        let mut acc = Accumulator832::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.266_f32 + y.sin();
        let b = y * 8.68_f32 - x.cos();
        let mut acc = Accumulator832::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.606_f32 + y.sin();
        let b = y * 1.019_f32 - x.cos();
        let mut acc = Accumulator832::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.612_f32 + y.sin();
        let b = y * 2.384_f32 - x.cos();
        let mut acc = Accumulator832::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.89_f32 + y.sin();
        let b = y * 5.862_f32 - x.cos();
        let mut acc = Accumulator832::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.34_f32 + y.sin();
        let b = y * 8.335_f32 - x.cos();
        let mut acc = Accumulator832::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.658_f32 + y.sin();
        let b = y * 1.346_f32 - x.cos();
        let mut acc = Accumulator832::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_832(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_832() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_832(total as u64) % 997) as f32;
        total
    }
}

pub mod m833 {
    use super::*;

    pub struct Accumulator833<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator833<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.009_f32 + y.sin();
        let b = y * 1.873_f32 - x.cos();
        let mut acc = Accumulator833::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.078_f32 + y.sin();
        let b = y * 8.545_f32 - x.cos();
        let mut acc = Accumulator833::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.709_f32 + y.sin();
        let b = y * 8.502_f32 - x.cos();
        let mut acc = Accumulator833::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.393_f32 + y.sin();
        let b = y * 9.356_f32 - x.cos();
        let mut acc = Accumulator833::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.881_f32 + y.sin();
        let b = y * 4.521_f32 - x.cos();
        let mut acc = Accumulator833::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.189_f32 + y.sin();
        let b = y * 2.286_f32 - x.cos();
        let mut acc = Accumulator833::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.743_f32 + y.sin();
        let b = y * 6.347_f32 - x.cos();
        let mut acc = Accumulator833::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.713_f32 + y.sin();
        let b = y * 4.305_f32 - x.cos();
        let mut acc = Accumulator833::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.964_f32 + y.sin();
        let b = y * 6.551_f32 - x.cos();
        let mut acc = Accumulator833::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.18_f32 + y.sin();
        let b = y * 6.517_f32 - x.cos();
        let mut acc = Accumulator833::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.595_f32 + y.sin();
        let b = y * 4.033_f32 - x.cos();
        let mut acc = Accumulator833::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.696_f32 + y.sin();
        let b = y * 7.831_f32 - x.cos();
        let mut acc = Accumulator833::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.988_f32 + y.sin();
        let b = y * 2.466_f32 - x.cos();
        let mut acc = Accumulator833::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.4_f32 + y.sin();
        let b = y * 5.96_f32 - x.cos();
        let mut acc = Accumulator833::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.921_f32 + y.sin();
        let b = y * 7.684_f32 - x.cos();
        let mut acc = Accumulator833::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.944_f32 + y.sin();
        let b = y * 4.923_f32 - x.cos();
        let mut acc = Accumulator833::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.259_f32 + y.sin();
        let b = y * 5.749_f32 - x.cos();
        let mut acc = Accumulator833::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.526_f32 + y.sin();
        let b = y * 0.305_f32 - x.cos();
        let mut acc = Accumulator833::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.586_f32 + y.sin();
        let b = y * 1.295_f32 - x.cos();
        let mut acc = Accumulator833::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.51_f32 + y.sin();
        let b = y * 6.586_f32 - x.cos();
        let mut acc = Accumulator833::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_833(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_833() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_833(total as u64) % 997) as f32;
        total
    }
}

pub mod m834 {
    use super::*;

    pub struct Accumulator834<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator834<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.374_f32 + y.sin();
        let b = y * 4.505_f32 - x.cos();
        let mut acc = Accumulator834::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.113_f32 + y.sin();
        let b = y * 3.972_f32 - x.cos();
        let mut acc = Accumulator834::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.025_f32 + y.sin();
        let b = y * 4.791_f32 - x.cos();
        let mut acc = Accumulator834::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.021_f32 + y.sin();
        let b = y * 9.364_f32 - x.cos();
        let mut acc = Accumulator834::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.653_f32 + y.sin();
        let b = y * 5.524_f32 - x.cos();
        let mut acc = Accumulator834::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.378_f32 + y.sin();
        let b = y * 2.763_f32 - x.cos();
        let mut acc = Accumulator834::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.306_f32 + y.sin();
        let b = y * 9.176_f32 - x.cos();
        let mut acc = Accumulator834::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.208_f32 + y.sin();
        let b = y * 0.678_f32 - x.cos();
        let mut acc = Accumulator834::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.855_f32 + y.sin();
        let b = y * 4.033_f32 - x.cos();
        let mut acc = Accumulator834::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.969_f32 + y.sin();
        let b = y * 5.091_f32 - x.cos();
        let mut acc = Accumulator834::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.706_f32 + y.sin();
        let b = y * 7.909_f32 - x.cos();
        let mut acc = Accumulator834::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.79_f32 + y.sin();
        let b = y * 1.37_f32 - x.cos();
        let mut acc = Accumulator834::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.969_f32 + y.sin();
        let b = y * 0.858_f32 - x.cos();
        let mut acc = Accumulator834::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.277_f32 + y.sin();
        let b = y * 4.284_f32 - x.cos();
        let mut acc = Accumulator834::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.765_f32 + y.sin();
        let b = y * 3.845_f32 - x.cos();
        let mut acc = Accumulator834::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.995_f32 + y.sin();
        let b = y * 4.034_f32 - x.cos();
        let mut acc = Accumulator834::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.143_f32 + y.sin();
        let b = y * 6.043_f32 - x.cos();
        let mut acc = Accumulator834::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.055_f32 + y.sin();
        let b = y * 3.604_f32 - x.cos();
        let mut acc = Accumulator834::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.082_f32 + y.sin();
        let b = y * 9.743_f32 - x.cos();
        let mut acc = Accumulator834::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.893_f32 + y.sin();
        let b = y * 1.422_f32 - x.cos();
        let mut acc = Accumulator834::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_834(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m834-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_834() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_834(total as u64) % 997) as f32;
        total
    }
}

pub mod m835 {
    use super::*;

    pub struct Accumulator835<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator835<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.668_f32 + y.sin();
        let b = y * 3.03_f32 - x.cos();
        let mut acc = Accumulator835::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.011_f32 + y.sin();
        let b = y * 9.361_f32 - x.cos();
        let mut acc = Accumulator835::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.244_f32 + y.sin();
        let b = y * 2.715_f32 - x.cos();
        let mut acc = Accumulator835::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.364_f32 + y.sin();
        let b = y * 0.471_f32 - x.cos();
        let mut acc = Accumulator835::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.109_f32 + y.sin();
        let b = y * 2.077_f32 - x.cos();
        let mut acc = Accumulator835::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.385_f32 + y.sin();
        let b = y * 7.738_f32 - x.cos();
        let mut acc = Accumulator835::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.632_f32 + y.sin();
        let b = y * 4.207_f32 - x.cos();
        let mut acc = Accumulator835::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.875_f32 + y.sin();
        let b = y * 2.528_f32 - x.cos();
        let mut acc = Accumulator835::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.415_f32 + y.sin();
        let b = y * 9.465_f32 - x.cos();
        let mut acc = Accumulator835::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.207_f32 + y.sin();
        let b = y * 3.987_f32 - x.cos();
        let mut acc = Accumulator835::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.728_f32 + y.sin();
        let b = y * 6.695_f32 - x.cos();
        let mut acc = Accumulator835::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.805_f32 + y.sin();
        let b = y * 4.055_f32 - x.cos();
        let mut acc = Accumulator835::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.249_f32 + y.sin();
        let b = y * 1.09_f32 - x.cos();
        let mut acc = Accumulator835::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.294_f32 + y.sin();
        let b = y * 9.15_f32 - x.cos();
        let mut acc = Accumulator835::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.856_f32 + y.sin();
        let b = y * 6.004_f32 - x.cos();
        let mut acc = Accumulator835::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.461_f32 + y.sin();
        let b = y * 2.396_f32 - x.cos();
        let mut acc = Accumulator835::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.338_f32 + y.sin();
        let b = y * 0.759_f32 - x.cos();
        let mut acc = Accumulator835::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.104_f32 + y.sin();
        let b = y * 4.96_f32 - x.cos();
        let mut acc = Accumulator835::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.092_f32 + y.sin();
        let b = y * 9.325_f32 - x.cos();
        let mut acc = Accumulator835::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.081_f32 + y.sin();
        let b = y * 7.078_f32 - x.cos();
        let mut acc = Accumulator835::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_835(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_835() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_835(total as u64) % 997) as f32;
        total
    }
}

pub mod m836 {
    use super::*;

    pub struct Accumulator836<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator836<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.737_f32 + y.sin();
        let b = y * 5.998_f32 - x.cos();
        let mut acc = Accumulator836::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.559_f32 + y.sin();
        let b = y * 0.601_f32 - x.cos();
        let mut acc = Accumulator836::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.735_f32 + y.sin();
        let b = y * 0.958_f32 - x.cos();
        let mut acc = Accumulator836::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.835_f32 + y.sin();
        let b = y * 5.957_f32 - x.cos();
        let mut acc = Accumulator836::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.167_f32 + y.sin();
        let b = y * 6.459_f32 - x.cos();
        let mut acc = Accumulator836::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.051_f32 + y.sin();
        let b = y * 9.179_f32 - x.cos();
        let mut acc = Accumulator836::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.494_f32 + y.sin();
        let b = y * 5.833_f32 - x.cos();
        let mut acc = Accumulator836::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.298_f32 + y.sin();
        let b = y * 2.016_f32 - x.cos();
        let mut acc = Accumulator836::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.083_f32 + y.sin();
        let b = y * 0.952_f32 - x.cos();
        let mut acc = Accumulator836::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.719_f32 + y.sin();
        let b = y * 1.439_f32 - x.cos();
        let mut acc = Accumulator836::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.222_f32 + y.sin();
        let b = y * 2.701_f32 - x.cos();
        let mut acc = Accumulator836::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.189_f32 + y.sin();
        let b = y * 5.155_f32 - x.cos();
        let mut acc = Accumulator836::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.21_f32 + y.sin();
        let b = y * 8.426_f32 - x.cos();
        let mut acc = Accumulator836::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.963_f32 + y.sin();
        let b = y * 4.79_f32 - x.cos();
        let mut acc = Accumulator836::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.827_f32 + y.sin();
        let b = y * 3.367_f32 - x.cos();
        let mut acc = Accumulator836::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.492_f32 + y.sin();
        let b = y * 4.032_f32 - x.cos();
        let mut acc = Accumulator836::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.89_f32 + y.sin();
        let b = y * 7.338_f32 - x.cos();
        let mut acc = Accumulator836::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.024_f32 + y.sin();
        let b = y * 3.27_f32 - x.cos();
        let mut acc = Accumulator836::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.403_f32 + y.sin();
        let b = y * 8.717_f32 - x.cos();
        let mut acc = Accumulator836::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.747_f32 + y.sin();
        let b = y * 1.597_f32 - x.cos();
        let mut acc = Accumulator836::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_836(seed: u64) -> u64 {
        let re = Regex::new(r"m836-(\d+)").unwrap();
        let hay = format!("m836-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_836() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_836(total as u64) % 997) as f32;
        total
    }
}

pub mod m837 {
    use super::*;

    pub struct Accumulator837<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator837<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.545_f32 + y.sin();
        let b = y * 7.557_f32 - x.cos();
        let mut acc = Accumulator837::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.486_f32 + y.sin();
        let b = y * 8.305_f32 - x.cos();
        let mut acc = Accumulator837::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.717_f32 + y.sin();
        let b = y * 6.843_f32 - x.cos();
        let mut acc = Accumulator837::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.375_f32 + y.sin();
        let b = y * 6.258_f32 - x.cos();
        let mut acc = Accumulator837::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.278_f32 + y.sin();
        let b = y * 8.014_f32 - x.cos();
        let mut acc = Accumulator837::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.446_f32 + y.sin();
        let b = y * 1.503_f32 - x.cos();
        let mut acc = Accumulator837::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.436_f32 + y.sin();
        let b = y * 9.497_f32 - x.cos();
        let mut acc = Accumulator837::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.399_f32 + y.sin();
        let b = y * 7.261_f32 - x.cos();
        let mut acc = Accumulator837::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.284_f32 + y.sin();
        let b = y * 2.692_f32 - x.cos();
        let mut acc = Accumulator837::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.08_f32 + y.sin();
        let b = y * 6.368_f32 - x.cos();
        let mut acc = Accumulator837::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.896_f32 + y.sin();
        let b = y * 6.617_f32 - x.cos();
        let mut acc = Accumulator837::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.539_f32 + y.sin();
        let b = y * 4.448_f32 - x.cos();
        let mut acc = Accumulator837::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.61_f32 + y.sin();
        let b = y * 9.574_f32 - x.cos();
        let mut acc = Accumulator837::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.226_f32 + y.sin();
        let b = y * 3.057_f32 - x.cos();
        let mut acc = Accumulator837::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.042_f32 + y.sin();
        let b = y * 2.259_f32 - x.cos();
        let mut acc = Accumulator837::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.772_f32 + y.sin();
        let b = y * 6.314_f32 - x.cos();
        let mut acc = Accumulator837::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.544_f32 + y.sin();
        let b = y * 4.9_f32 - x.cos();
        let mut acc = Accumulator837::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.876_f32 + y.sin();
        let b = y * 9.386_f32 - x.cos();
        let mut acc = Accumulator837::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.516_f32 + y.sin();
        let b = y * 2.6_f32 - x.cos();
        let mut acc = Accumulator837::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.664_f32 + y.sin();
        let b = y * 5.0_f32 - x.cos();
        let mut acc = Accumulator837::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_837(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_837() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_837(total as u64) % 997) as f32;
        total
    }
}

pub mod m838 {
    use super::*;

    pub struct Accumulator838<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator838<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.143_f32 + y.sin();
        let b = y * 3.737_f32 - x.cos();
        let mut acc = Accumulator838::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.445_f32 + y.sin();
        let b = y * 8.268_f32 - x.cos();
        let mut acc = Accumulator838::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.77_f32 + y.sin();
        let b = y * 8.365_f32 - x.cos();
        let mut acc = Accumulator838::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.511_f32 + y.sin();
        let b = y * 2.097_f32 - x.cos();
        let mut acc = Accumulator838::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.799_f32 + y.sin();
        let b = y * 5.462_f32 - x.cos();
        let mut acc = Accumulator838::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.409_f32 + y.sin();
        let b = y * 5.918_f32 - x.cos();
        let mut acc = Accumulator838::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.242_f32 + y.sin();
        let b = y * 3.676_f32 - x.cos();
        let mut acc = Accumulator838::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.383_f32 + y.sin();
        let b = y * 1.23_f32 - x.cos();
        let mut acc = Accumulator838::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.263_f32 + y.sin();
        let b = y * 4.248_f32 - x.cos();
        let mut acc = Accumulator838::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.036_f32 + y.sin();
        let b = y * 9.202_f32 - x.cos();
        let mut acc = Accumulator838::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.665_f32 + y.sin();
        let b = y * 2.179_f32 - x.cos();
        let mut acc = Accumulator838::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.571_f32 + y.sin();
        let b = y * 7.412_f32 - x.cos();
        let mut acc = Accumulator838::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.352_f32 + y.sin();
        let b = y * 1.141_f32 - x.cos();
        let mut acc = Accumulator838::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.614_f32 + y.sin();
        let b = y * 6.075_f32 - x.cos();
        let mut acc = Accumulator838::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.36_f32 + y.sin();
        let b = y * 6.088_f32 - x.cos();
        let mut acc = Accumulator838::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.73_f32 + y.sin();
        let b = y * 6.292_f32 - x.cos();
        let mut acc = Accumulator838::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.208_f32 + y.sin();
        let b = y * 8.975_f32 - x.cos();
        let mut acc = Accumulator838::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.12_f32 + y.sin();
        let b = y * 7.285_f32 - x.cos();
        let mut acc = Accumulator838::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.212_f32 + y.sin();
        let b = y * 1.269_f32 - x.cos();
        let mut acc = Accumulator838::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.02_f32 + y.sin();
        let b = y * 3.329_f32 - x.cos();
        let mut acc = Accumulator838::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_838(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(838u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_838() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_838(total as u64) % 997) as f32;
        total
    }
}

pub mod m839 {
    use super::*;

    pub struct Accumulator839<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator839<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.376_f32 + y.sin();
        let b = y * 3.259_f32 - x.cos();
        let mut acc = Accumulator839::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.128_f32 + y.sin();
        let b = y * 3.04_f32 - x.cos();
        let mut acc = Accumulator839::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.555_f32 + y.sin();
        let b = y * 5.849_f32 - x.cos();
        let mut acc = Accumulator839::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.2_f32 + y.sin();
        let b = y * 0.489_f32 - x.cos();
        let mut acc = Accumulator839::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.25_f32 + y.sin();
        let b = y * 9.713_f32 - x.cos();
        let mut acc = Accumulator839::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.896_f32 + y.sin();
        let b = y * 1.814_f32 - x.cos();
        let mut acc = Accumulator839::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.22_f32 + y.sin();
        let b = y * 2.69_f32 - x.cos();
        let mut acc = Accumulator839::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.293_f32 + y.sin();
        let b = y * 6.854_f32 - x.cos();
        let mut acc = Accumulator839::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.422_f32 + y.sin();
        let b = y * 4.466_f32 - x.cos();
        let mut acc = Accumulator839::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.553_f32 + y.sin();
        let b = y * 4.392_f32 - x.cos();
        let mut acc = Accumulator839::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.035_f32 + y.sin();
        let b = y * 1.225_f32 - x.cos();
        let mut acc = Accumulator839::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.178_f32 + y.sin();
        let b = y * 9.854_f32 - x.cos();
        let mut acc = Accumulator839::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.082_f32 + y.sin();
        let b = y * 4.783_f32 - x.cos();
        let mut acc = Accumulator839::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.129_f32 + y.sin();
        let b = y * 6.435_f32 - x.cos();
        let mut acc = Accumulator839::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.516_f32 + y.sin();
        let b = y * 7.004_f32 - x.cos();
        let mut acc = Accumulator839::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.449_f32 + y.sin();
        let b = y * 9.325_f32 - x.cos();
        let mut acc = Accumulator839::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.775_f32 + y.sin();
        let b = y * 9.176_f32 - x.cos();
        let mut acc = Accumulator839::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.117_f32 + y.sin();
        let b = y * 5.394_f32 - x.cos();
        let mut acc = Accumulator839::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.937_f32 + y.sin();
        let b = y * 2.619_f32 - x.cos();
        let mut acc = Accumulator839::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.324_f32 + y.sin();
        let b = y * 2.574_f32 - x.cos();
        let mut acc = Accumulator839::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_839(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_839() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_839(total as u64) % 997) as f32;
        total
    }
}

pub mod m840 {
    use super::*;

    pub struct Accumulator840<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator840<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.883_f32 + y.sin();
        let b = y * 3.826_f32 - x.cos();
        let mut acc = Accumulator840::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.526_f32 + y.sin();
        let b = y * 9.82_f32 - x.cos();
        let mut acc = Accumulator840::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.282_f32 + y.sin();
        let b = y * 0.205_f32 - x.cos();
        let mut acc = Accumulator840::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.456_f32 + y.sin();
        let b = y * 8.435_f32 - x.cos();
        let mut acc = Accumulator840::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.969_f32 + y.sin();
        let b = y * 0.686_f32 - x.cos();
        let mut acc = Accumulator840::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.74_f32 + y.sin();
        let b = y * 1.019_f32 - x.cos();
        let mut acc = Accumulator840::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.734_f32 + y.sin();
        let b = y * 0.75_f32 - x.cos();
        let mut acc = Accumulator840::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.375_f32 + y.sin();
        let b = y * 5.383_f32 - x.cos();
        let mut acc = Accumulator840::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.258_f32 + y.sin();
        let b = y * 9.338_f32 - x.cos();
        let mut acc = Accumulator840::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.588_f32 + y.sin();
        let b = y * 3.441_f32 - x.cos();
        let mut acc = Accumulator840::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.852_f32 + y.sin();
        let b = y * 0.559_f32 - x.cos();
        let mut acc = Accumulator840::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.413_f32 + y.sin();
        let b = y * 2.577_f32 - x.cos();
        let mut acc = Accumulator840::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.556_f32 + y.sin();
        let b = y * 3.302_f32 - x.cos();
        let mut acc = Accumulator840::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.222_f32 + y.sin();
        let b = y * 7.735_f32 - x.cos();
        let mut acc = Accumulator840::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.486_f32 + y.sin();
        let b = y * 8.005_f32 - x.cos();
        let mut acc = Accumulator840::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.13_f32 + y.sin();
        let b = y * 8.897_f32 - x.cos();
        let mut acc = Accumulator840::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.029_f32 + y.sin();
        let b = y * 1.487_f32 - x.cos();
        let mut acc = Accumulator840::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.353_f32 + y.sin();
        let b = y * 0.179_f32 - x.cos();
        let mut acc = Accumulator840::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.53_f32 + y.sin();
        let b = y * 5.865_f32 - x.cos();
        let mut acc = Accumulator840::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.369_f32 + y.sin();
        let b = y * 7.649_f32 - x.cos();
        let mut acc = Accumulator840::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_840(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_840() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_840(total as u64) % 997) as f32;
        total
    }
}

pub mod m841 {
    use super::*;

    pub struct Accumulator841<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator841<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.265_f32 + y.sin();
        let b = y * 5.77_f32 - x.cos();
        let mut acc = Accumulator841::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.151_f32 + y.sin();
        let b = y * 0.356_f32 - x.cos();
        let mut acc = Accumulator841::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.456_f32 + y.sin();
        let b = y * 7.821_f32 - x.cos();
        let mut acc = Accumulator841::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.65_f32 + y.sin();
        let b = y * 1.73_f32 - x.cos();
        let mut acc = Accumulator841::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.535_f32 + y.sin();
        let b = y * 0.452_f32 - x.cos();
        let mut acc = Accumulator841::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.049_f32 + y.sin();
        let b = y * 2.407_f32 - x.cos();
        let mut acc = Accumulator841::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.418_f32 + y.sin();
        let b = y * 0.901_f32 - x.cos();
        let mut acc = Accumulator841::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.118_f32 + y.sin();
        let b = y * 3.326_f32 - x.cos();
        let mut acc = Accumulator841::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.599_f32 + y.sin();
        let b = y * 8.501_f32 - x.cos();
        let mut acc = Accumulator841::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.693_f32 + y.sin();
        let b = y * 1.2_f32 - x.cos();
        let mut acc = Accumulator841::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.81_f32 + y.sin();
        let b = y * 5.127_f32 - x.cos();
        let mut acc = Accumulator841::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.505_f32 + y.sin();
        let b = y * 1.261_f32 - x.cos();
        let mut acc = Accumulator841::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.98_f32 + y.sin();
        let b = y * 3.499_f32 - x.cos();
        let mut acc = Accumulator841::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.495_f32 + y.sin();
        let b = y * 4.661_f32 - x.cos();
        let mut acc = Accumulator841::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.961_f32 + y.sin();
        let b = y * 7.958_f32 - x.cos();
        let mut acc = Accumulator841::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.761_f32 + y.sin();
        let b = y * 7.462_f32 - x.cos();
        let mut acc = Accumulator841::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.594_f32 + y.sin();
        let b = y * 4.02_f32 - x.cos();
        let mut acc = Accumulator841::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.146_f32 + y.sin();
        let b = y * 2.935_f32 - x.cos();
        let mut acc = Accumulator841::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.126_f32 + y.sin();
        let b = y * 3.996_f32 - x.cos();
        let mut acc = Accumulator841::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.489_f32 + y.sin();
        let b = y * 0.663_f32 - x.cos();
        let mut acc = Accumulator841::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_841(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m841-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_841() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_841(total as u64) % 997) as f32;
        total
    }
}

pub mod m842 {
    use super::*;

    pub struct Accumulator842<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator842<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.691_f32 + y.sin();
        let b = y * 8.602_f32 - x.cos();
        let mut acc = Accumulator842::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.184_f32 + y.sin();
        let b = y * 5.866_f32 - x.cos();
        let mut acc = Accumulator842::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.573_f32 + y.sin();
        let b = y * 5.673_f32 - x.cos();
        let mut acc = Accumulator842::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.103_f32 + y.sin();
        let b = y * 8.353_f32 - x.cos();
        let mut acc = Accumulator842::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.286_f32 + y.sin();
        let b = y * 4.321_f32 - x.cos();
        let mut acc = Accumulator842::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.955_f32 + y.sin();
        let b = y * 5.663_f32 - x.cos();
        let mut acc = Accumulator842::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.779_f32 + y.sin();
        let b = y * 3.989_f32 - x.cos();
        let mut acc = Accumulator842::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.485_f32 + y.sin();
        let b = y * 0.659_f32 - x.cos();
        let mut acc = Accumulator842::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.771_f32 + y.sin();
        let b = y * 8.845_f32 - x.cos();
        let mut acc = Accumulator842::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.375_f32 + y.sin();
        let b = y * 2.788_f32 - x.cos();
        let mut acc = Accumulator842::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.189_f32 + y.sin();
        let b = y * 9.236_f32 - x.cos();
        let mut acc = Accumulator842::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.931_f32 + y.sin();
        let b = y * 1.471_f32 - x.cos();
        let mut acc = Accumulator842::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.783_f32 + y.sin();
        let b = y * 1.001_f32 - x.cos();
        let mut acc = Accumulator842::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 9.269_f32 + y.sin();
        let b = y * 6.314_f32 - x.cos();
        let mut acc = Accumulator842::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.524_f32 + y.sin();
        let b = y * 5.136_f32 - x.cos();
        let mut acc = Accumulator842::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.087_f32 + y.sin();
        let b = y * 1.023_f32 - x.cos();
        let mut acc = Accumulator842::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.155_f32 + y.sin();
        let b = y * 4.122_f32 - x.cos();
        let mut acc = Accumulator842::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.85_f32 + y.sin();
        let b = y * 7.711_f32 - x.cos();
        let mut acc = Accumulator842::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.891_f32 + y.sin();
        let b = y * 0.918_f32 - x.cos();
        let mut acc = Accumulator842::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.167_f32 + y.sin();
        let b = y * 3.18_f32 - x.cos();
        let mut acc = Accumulator842::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_842(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_842() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_842(total as u64) % 997) as f32;
        total
    }
}

pub mod m843 {
    use super::*;

    pub struct Accumulator843<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator843<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.789_f32 + y.sin();
        let b = y * 0.754_f32 - x.cos();
        let mut acc = Accumulator843::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.014_f32 + y.sin();
        let b = y * 7.367_f32 - x.cos();
        let mut acc = Accumulator843::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.272_f32 + y.sin();
        let b = y * 1.604_f32 - x.cos();
        let mut acc = Accumulator843::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.715_f32 + y.sin();
        let b = y * 4.833_f32 - x.cos();
        let mut acc = Accumulator843::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.373_f32 + y.sin();
        let b = y * 1.125_f32 - x.cos();
        let mut acc = Accumulator843::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.254_f32 + y.sin();
        let b = y * 7.155_f32 - x.cos();
        let mut acc = Accumulator843::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.221_f32 + y.sin();
        let b = y * 3.469_f32 - x.cos();
        let mut acc = Accumulator843::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.749_f32 + y.sin();
        let b = y * 7.881_f32 - x.cos();
        let mut acc = Accumulator843::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.074_f32 + y.sin();
        let b = y * 8.328_f32 - x.cos();
        let mut acc = Accumulator843::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.237_f32 + y.sin();
        let b = y * 2.807_f32 - x.cos();
        let mut acc = Accumulator843::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.528_f32 + y.sin();
        let b = y * 4.508_f32 - x.cos();
        let mut acc = Accumulator843::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.178_f32 + y.sin();
        let b = y * 3.641_f32 - x.cos();
        let mut acc = Accumulator843::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.229_f32 + y.sin();
        let b = y * 5.076_f32 - x.cos();
        let mut acc = Accumulator843::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.138_f32 + y.sin();
        let b = y * 8.744_f32 - x.cos();
        let mut acc = Accumulator843::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.207_f32 + y.sin();
        let b = y * 0.13_f32 - x.cos();
        let mut acc = Accumulator843::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.096_f32 + y.sin();
        let b = y * 8.429_f32 - x.cos();
        let mut acc = Accumulator843::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.188_f32 + y.sin();
        let b = y * 3.374_f32 - x.cos();
        let mut acc = Accumulator843::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.78_f32 + y.sin();
        let b = y * 5.732_f32 - x.cos();
        let mut acc = Accumulator843::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.513_f32 + y.sin();
        let b = y * 5.973_f32 - x.cos();
        let mut acc = Accumulator843::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.793_f32 + y.sin();
        let b = y * 3.823_f32 - x.cos();
        let mut acc = Accumulator843::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_843(seed: u64) -> u64 {
        let re = Regex::new(r"m843-(\d+)").unwrap();
        let hay = format!("m843-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_843() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_843(total as u64) % 997) as f32;
        total
    }
}

pub mod m844 {
    use super::*;

    pub struct Accumulator844<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator844<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.435_f32 + y.sin();
        let b = y * 9.623_f32 - x.cos();
        let mut acc = Accumulator844::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.245_f32 + y.sin();
        let b = y * 8.93_f32 - x.cos();
        let mut acc = Accumulator844::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.922_f32 + y.sin();
        let b = y * 5.65_f32 - x.cos();
        let mut acc = Accumulator844::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.364_f32 + y.sin();
        let b = y * 9.808_f32 - x.cos();
        let mut acc = Accumulator844::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.625_f32 + y.sin();
        let b = y * 5.79_f32 - x.cos();
        let mut acc = Accumulator844::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.323_f32 + y.sin();
        let b = y * 1.294_f32 - x.cos();
        let mut acc = Accumulator844::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.696_f32 + y.sin();
        let b = y * 5.847_f32 - x.cos();
        let mut acc = Accumulator844::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.254_f32 + y.sin();
        let b = y * 7.637_f32 - x.cos();
        let mut acc = Accumulator844::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.374_f32 + y.sin();
        let b = y * 7.711_f32 - x.cos();
        let mut acc = Accumulator844::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.912_f32 + y.sin();
        let b = y * 8.641_f32 - x.cos();
        let mut acc = Accumulator844::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.744_f32 + y.sin();
        let b = y * 2.347_f32 - x.cos();
        let mut acc = Accumulator844::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.216_f32 + y.sin();
        let b = y * 2.964_f32 - x.cos();
        let mut acc = Accumulator844::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.044_f32 + y.sin();
        let b = y * 1.463_f32 - x.cos();
        let mut acc = Accumulator844::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.97_f32 + y.sin();
        let b = y * 4.893_f32 - x.cos();
        let mut acc = Accumulator844::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.365_f32 + y.sin();
        let b = y * 6.691_f32 - x.cos();
        let mut acc = Accumulator844::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.887_f32 + y.sin();
        let b = y * 2.804_f32 - x.cos();
        let mut acc = Accumulator844::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.197_f32 + y.sin();
        let b = y * 2.305_f32 - x.cos();
        let mut acc = Accumulator844::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.088_f32 + y.sin();
        let b = y * 0.91_f32 - x.cos();
        let mut acc = Accumulator844::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.939_f32 + y.sin();
        let b = y * 8.335_f32 - x.cos();
        let mut acc = Accumulator844::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.722_f32 + y.sin();
        let b = y * 3.291_f32 - x.cos();
        let mut acc = Accumulator844::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_844(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_844() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_844(total as u64) % 997) as f32;
        total
    }
}

pub mod m845 {
    use super::*;

    pub struct Accumulator845<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator845<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.922_f32 + y.sin();
        let b = y * 7.009_f32 - x.cos();
        let mut acc = Accumulator845::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.31_f32 + y.sin();
        let b = y * 0.317_f32 - x.cos();
        let mut acc = Accumulator845::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.83_f32 + y.sin();
        let b = y * 0.238_f32 - x.cos();
        let mut acc = Accumulator845::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.805_f32 + y.sin();
        let b = y * 8.764_f32 - x.cos();
        let mut acc = Accumulator845::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.773_f32 + y.sin();
        let b = y * 0.743_f32 - x.cos();
        let mut acc = Accumulator845::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.017_f32 + y.sin();
        let b = y * 8.003_f32 - x.cos();
        let mut acc = Accumulator845::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.406_f32 + y.sin();
        let b = y * 5.468_f32 - x.cos();
        let mut acc = Accumulator845::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.63_f32 + y.sin();
        let b = y * 6.477_f32 - x.cos();
        let mut acc = Accumulator845::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.555_f32 + y.sin();
        let b = y * 7.526_f32 - x.cos();
        let mut acc = Accumulator845::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.472_f32 + y.sin();
        let b = y * 9.101_f32 - x.cos();
        let mut acc = Accumulator845::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.512_f32 + y.sin();
        let b = y * 4.52_f32 - x.cos();
        let mut acc = Accumulator845::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.63_f32 + y.sin();
        let b = y * 4.56_f32 - x.cos();
        let mut acc = Accumulator845::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.504_f32 + y.sin();
        let b = y * 6.784_f32 - x.cos();
        let mut acc = Accumulator845::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.659_f32 + y.sin();
        let b = y * 2.629_f32 - x.cos();
        let mut acc = Accumulator845::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.856_f32 + y.sin();
        let b = y * 3.01_f32 - x.cos();
        let mut acc = Accumulator845::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.061_f32 + y.sin();
        let b = y * 8.242_f32 - x.cos();
        let mut acc = Accumulator845::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.593_f32 + y.sin();
        let b = y * 2.321_f32 - x.cos();
        let mut acc = Accumulator845::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.103_f32 + y.sin();
        let b = y * 9.261_f32 - x.cos();
        let mut acc = Accumulator845::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.489_f32 + y.sin();
        let b = y * 2.848_f32 - x.cos();
        let mut acc = Accumulator845::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.82_f32 + y.sin();
        let b = y * 4.186_f32 - x.cos();
        let mut acc = Accumulator845::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_845(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(845u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_845() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_845(total as u64) % 997) as f32;
        total
    }
}

pub mod m846 {
    use super::*;

    pub struct Accumulator846<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator846<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.978_f32 + y.sin();
        let b = y * 5.808_f32 - x.cos();
        let mut acc = Accumulator846::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.89_f32 + y.sin();
        let b = y * 0.524_f32 - x.cos();
        let mut acc = Accumulator846::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.655_f32 + y.sin();
        let b = y * 0.439_f32 - x.cos();
        let mut acc = Accumulator846::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.924_f32 + y.sin();
        let b = y * 5.019_f32 - x.cos();
        let mut acc = Accumulator846::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.482_f32 + y.sin();
        let b = y * 0.666_f32 - x.cos();
        let mut acc = Accumulator846::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.73_f32 + y.sin();
        let b = y * 2.516_f32 - x.cos();
        let mut acc = Accumulator846::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.139_f32 + y.sin();
        let b = y * 9.496_f32 - x.cos();
        let mut acc = Accumulator846::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.6_f32 + y.sin();
        let b = y * 8.241_f32 - x.cos();
        let mut acc = Accumulator846::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.319_f32 + y.sin();
        let b = y * 2.051_f32 - x.cos();
        let mut acc = Accumulator846::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.095_f32 + y.sin();
        let b = y * 8.948_f32 - x.cos();
        let mut acc = Accumulator846::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.359_f32 + y.sin();
        let b = y * 6.209_f32 - x.cos();
        let mut acc = Accumulator846::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.364_f32 + y.sin();
        let b = y * 0.121_f32 - x.cos();
        let mut acc = Accumulator846::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.988_f32 + y.sin();
        let b = y * 0.749_f32 - x.cos();
        let mut acc = Accumulator846::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.852_f32 + y.sin();
        let b = y * 9.322_f32 - x.cos();
        let mut acc = Accumulator846::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.895_f32 + y.sin();
        let b = y * 6.167_f32 - x.cos();
        let mut acc = Accumulator846::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.935_f32 + y.sin();
        let b = y * 1.499_f32 - x.cos();
        let mut acc = Accumulator846::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.044_f32 + y.sin();
        let b = y * 3.759_f32 - x.cos();
        let mut acc = Accumulator846::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.317_f32 + y.sin();
        let b = y * 9.394_f32 - x.cos();
        let mut acc = Accumulator846::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.937_f32 + y.sin();
        let b = y * 2.904_f32 - x.cos();
        let mut acc = Accumulator846::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.386_f32 + y.sin();
        let b = y * 4.166_f32 - x.cos();
        let mut acc = Accumulator846::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_846(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_846() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_846(total as u64) % 997) as f32;
        total
    }
}

pub mod m847 {
    use super::*;

    pub struct Accumulator847<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator847<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.668_f32 + y.sin();
        let b = y * 7.308_f32 - x.cos();
        let mut acc = Accumulator847::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.445_f32 + y.sin();
        let b = y * 1.67_f32 - x.cos();
        let mut acc = Accumulator847::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.399_f32 + y.sin();
        let b = y * 1.315_f32 - x.cos();
        let mut acc = Accumulator847::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.038_f32 + y.sin();
        let b = y * 1.752_f32 - x.cos();
        let mut acc = Accumulator847::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.145_f32 + y.sin();
        let b = y * 8.166_f32 - x.cos();
        let mut acc = Accumulator847::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.995_f32 + y.sin();
        let b = y * 4.21_f32 - x.cos();
        let mut acc = Accumulator847::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.878_f32 + y.sin();
        let b = y * 4.989_f32 - x.cos();
        let mut acc = Accumulator847::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.219_f32 + y.sin();
        let b = y * 1.617_f32 - x.cos();
        let mut acc = Accumulator847::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.396_f32 + y.sin();
        let b = y * 3.953_f32 - x.cos();
        let mut acc = Accumulator847::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.852_f32 + y.sin();
        let b = y * 9.772_f32 - x.cos();
        let mut acc = Accumulator847::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.973_f32 + y.sin();
        let b = y * 5.754_f32 - x.cos();
        let mut acc = Accumulator847::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.015_f32 + y.sin();
        let b = y * 9.119_f32 - x.cos();
        let mut acc = Accumulator847::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.141_f32 + y.sin();
        let b = y * 2.894_f32 - x.cos();
        let mut acc = Accumulator847::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.982_f32 + y.sin();
        let b = y * 3.723_f32 - x.cos();
        let mut acc = Accumulator847::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.583_f32 + y.sin();
        let b = y * 9.544_f32 - x.cos();
        let mut acc = Accumulator847::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.561_f32 + y.sin();
        let b = y * 7.536_f32 - x.cos();
        let mut acc = Accumulator847::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.2_f32 + y.sin();
        let b = y * 8.179_f32 - x.cos();
        let mut acc = Accumulator847::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.102_f32 + y.sin();
        let b = y * 7.215_f32 - x.cos();
        let mut acc = Accumulator847::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.437_f32 + y.sin();
        let b = y * 1.617_f32 - x.cos();
        let mut acc = Accumulator847::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.846_f32 + y.sin();
        let b = y * 0.204_f32 - x.cos();
        let mut acc = Accumulator847::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_847(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_847() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_847(total as u64) % 997) as f32;
        total
    }
}

pub mod m848 {
    use super::*;

    pub struct Accumulator848<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator848<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.356_f32 + y.sin();
        let b = y * 8.228_f32 - x.cos();
        let mut acc = Accumulator848::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.649_f32 + y.sin();
        let b = y * 2.795_f32 - x.cos();
        let mut acc = Accumulator848::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.203_f32 + y.sin();
        let b = y * 8.599_f32 - x.cos();
        let mut acc = Accumulator848::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.771_f32 + y.sin();
        let b = y * 9.394_f32 - x.cos();
        let mut acc = Accumulator848::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.862_f32 + y.sin();
        let b = y * 6.176_f32 - x.cos();
        let mut acc = Accumulator848::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.479_f32 + y.sin();
        let b = y * 5.416_f32 - x.cos();
        let mut acc = Accumulator848::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.13_f32 + y.sin();
        let b = y * 5.842_f32 - x.cos();
        let mut acc = Accumulator848::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.063_f32 + y.sin();
        let b = y * 7.526_f32 - x.cos();
        let mut acc = Accumulator848::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.676_f32 + y.sin();
        let b = y * 4.812_f32 - x.cos();
        let mut acc = Accumulator848::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.427_f32 + y.sin();
        let b = y * 8.634_f32 - x.cos();
        let mut acc = Accumulator848::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.806_f32 + y.sin();
        let b = y * 3.571_f32 - x.cos();
        let mut acc = Accumulator848::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.226_f32 + y.sin();
        let b = y * 7.911_f32 - x.cos();
        let mut acc = Accumulator848::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.682_f32 + y.sin();
        let b = y * 5.607_f32 - x.cos();
        let mut acc = Accumulator848::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.576_f32 + y.sin();
        let b = y * 6.251_f32 - x.cos();
        let mut acc = Accumulator848::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.597_f32 + y.sin();
        let b = y * 8.014_f32 - x.cos();
        let mut acc = Accumulator848::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.574_f32 + y.sin();
        let b = y * 9.55_f32 - x.cos();
        let mut acc = Accumulator848::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.297_f32 + y.sin();
        let b = y * 9.858_f32 - x.cos();
        let mut acc = Accumulator848::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.366_f32 + y.sin();
        let b = y * 3.974_f32 - x.cos();
        let mut acc = Accumulator848::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.631_f32 + y.sin();
        let b = y * 4.54_f32 - x.cos();
        let mut acc = Accumulator848::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.575_f32 + y.sin();
        let b = y * 4.115_f32 - x.cos();
        let mut acc = Accumulator848::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_848(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m848-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_848() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_848(total as u64) % 997) as f32;
        total
    }
}

pub mod m849 {
    use super::*;

    pub struct Accumulator849<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator849<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.162_f32 + y.sin();
        let b = y * 6.329_f32 - x.cos();
        let mut acc = Accumulator849::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.084_f32 + y.sin();
        let b = y * 2.93_f32 - x.cos();
        let mut acc = Accumulator849::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.237_f32 + y.sin();
        let b = y * 6.306_f32 - x.cos();
        let mut acc = Accumulator849::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.065_f32 + y.sin();
        let b = y * 7.659_f32 - x.cos();
        let mut acc = Accumulator849::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.241_f32 + y.sin();
        let b = y * 0.534_f32 - x.cos();
        let mut acc = Accumulator849::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.467_f32 + y.sin();
        let b = y * 4.411_f32 - x.cos();
        let mut acc = Accumulator849::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.654_f32 + y.sin();
        let b = y * 6.555_f32 - x.cos();
        let mut acc = Accumulator849::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.833_f32 + y.sin();
        let b = y * 2.653_f32 - x.cos();
        let mut acc = Accumulator849::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.482_f32 + y.sin();
        let b = y * 0.224_f32 - x.cos();
        let mut acc = Accumulator849::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.147_f32 + y.sin();
        let b = y * 5.056_f32 - x.cos();
        let mut acc = Accumulator849::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.128_f32 + y.sin();
        let b = y * 9.51_f32 - x.cos();
        let mut acc = Accumulator849::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.678_f32 + y.sin();
        let b = y * 3.188_f32 - x.cos();
        let mut acc = Accumulator849::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.233_f32 + y.sin();
        let b = y * 4.703_f32 - x.cos();
        let mut acc = Accumulator849::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.086_f32 + y.sin();
        let b = y * 9.269_f32 - x.cos();
        let mut acc = Accumulator849::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 3.263_f32 + y.sin();
        let b = y * 3.758_f32 - x.cos();
        let mut acc = Accumulator849::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.889_f32 + y.sin();
        let b = y * 7.807_f32 - x.cos();
        let mut acc = Accumulator849::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.624_f32 + y.sin();
        let b = y * 8.555_f32 - x.cos();
        let mut acc = Accumulator849::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.84_f32 + y.sin();
        let b = y * 6.286_f32 - x.cos();
        let mut acc = Accumulator849::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.207_f32 + y.sin();
        let b = y * 4.241_f32 - x.cos();
        let mut acc = Accumulator849::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.739_f32 + y.sin();
        let b = y * 3.139_f32 - x.cos();
        let mut acc = Accumulator849::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_849(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_849() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_849(total as u64) % 997) as f32;
        total
    }
}

pub mod m850 {
    use super::*;

    pub struct Accumulator850<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator850<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.45_f32 + y.sin();
        let b = y * 9.251_f32 - x.cos();
        let mut acc = Accumulator850::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.266_f32 + y.sin();
        let b = y * 7.795_f32 - x.cos();
        let mut acc = Accumulator850::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.576_f32 + y.sin();
        let b = y * 6.951_f32 - x.cos();
        let mut acc = Accumulator850::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.334_f32 + y.sin();
        let b = y * 3.214_f32 - x.cos();
        let mut acc = Accumulator850::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.822_f32 + y.sin();
        let b = y * 0.852_f32 - x.cos();
        let mut acc = Accumulator850::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.335_f32 + y.sin();
        let b = y * 0.951_f32 - x.cos();
        let mut acc = Accumulator850::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.652_f32 + y.sin();
        let b = y * 9.547_f32 - x.cos();
        let mut acc = Accumulator850::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.616_f32 + y.sin();
        let b = y * 6.352_f32 - x.cos();
        let mut acc = Accumulator850::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.149_f32 + y.sin();
        let b = y * 5.702_f32 - x.cos();
        let mut acc = Accumulator850::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.314_f32 + y.sin();
        let b = y * 9.707_f32 - x.cos();
        let mut acc = Accumulator850::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.34_f32 + y.sin();
        let b = y * 2.879_f32 - x.cos();
        let mut acc = Accumulator850::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.931_f32 + y.sin();
        let b = y * 3.608_f32 - x.cos();
        let mut acc = Accumulator850::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.92_f32 + y.sin();
        let b = y * 2.185_f32 - x.cos();
        let mut acc = Accumulator850::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.134_f32 + y.sin();
        let b = y * 9.543_f32 - x.cos();
        let mut acc = Accumulator850::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.6_f32 + y.sin();
        let b = y * 6.153_f32 - x.cos();
        let mut acc = Accumulator850::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.974_f32 + y.sin();
        let b = y * 5.501_f32 - x.cos();
        let mut acc = Accumulator850::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.35_f32 + y.sin();
        let b = y * 1.115_f32 - x.cos();
        let mut acc = Accumulator850::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.367_f32 + y.sin();
        let b = y * 6.297_f32 - x.cos();
        let mut acc = Accumulator850::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.138_f32 + y.sin();
        let b = y * 6.682_f32 - x.cos();
        let mut acc = Accumulator850::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.196_f32 + y.sin();
        let b = y * 1.995_f32 - x.cos();
        let mut acc = Accumulator850::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_850(seed: u64) -> u64 {
        let re = Regex::new(r"m850-(\d+)").unwrap();
        let hay = format!("m850-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_850() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_850(total as u64) % 997) as f32;
        total
    }
}

pub mod m851 {
    use super::*;

    pub struct Accumulator851<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator851<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.784_f32 + y.sin();
        let b = y * 0.285_f32 - x.cos();
        let mut acc = Accumulator851::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.92_f32 + y.sin();
        let b = y * 6.25_f32 - x.cos();
        let mut acc = Accumulator851::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.026_f32 + y.sin();
        let b = y * 3.339_f32 - x.cos();
        let mut acc = Accumulator851::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.563_f32 + y.sin();
        let b = y * 9.624_f32 - x.cos();
        let mut acc = Accumulator851::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.033_f32 + y.sin();
        let b = y * 0.717_f32 - x.cos();
        let mut acc = Accumulator851::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.292_f32 + y.sin();
        let b = y * 7.059_f32 - x.cos();
        let mut acc = Accumulator851::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.843_f32 + y.sin();
        let b = y * 4.442_f32 - x.cos();
        let mut acc = Accumulator851::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.811_f32 + y.sin();
        let b = y * 3.803_f32 - x.cos();
        let mut acc = Accumulator851::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.285_f32 + y.sin();
        let b = y * 0.552_f32 - x.cos();
        let mut acc = Accumulator851::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.582_f32 + y.sin();
        let b = y * 6.955_f32 - x.cos();
        let mut acc = Accumulator851::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.803_f32 + y.sin();
        let b = y * 5.465_f32 - x.cos();
        let mut acc = Accumulator851::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.252_f32 + y.sin();
        let b = y * 1.169_f32 - x.cos();
        let mut acc = Accumulator851::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.751_f32 + y.sin();
        let b = y * 9.781_f32 - x.cos();
        let mut acc = Accumulator851::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.88_f32 + y.sin();
        let b = y * 4.603_f32 - x.cos();
        let mut acc = Accumulator851::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.277_f32 + y.sin();
        let b = y * 2.704_f32 - x.cos();
        let mut acc = Accumulator851::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.521_f32 + y.sin();
        let b = y * 4.491_f32 - x.cos();
        let mut acc = Accumulator851::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.419_f32 + y.sin();
        let b = y * 9.759_f32 - x.cos();
        let mut acc = Accumulator851::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.248_f32 + y.sin();
        let b = y * 1.488_f32 - x.cos();
        let mut acc = Accumulator851::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.796_f32 + y.sin();
        let b = y * 8.776_f32 - x.cos();
        let mut acc = Accumulator851::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.287_f32 + y.sin();
        let b = y * 4.892_f32 - x.cos();
        let mut acc = Accumulator851::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_851(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_851() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_851(total as u64) % 997) as f32;
        total
    }
}

pub mod m852 {
    use super::*;

    pub struct Accumulator852<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator852<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.21_f32 + y.sin();
        let b = y * 8.848_f32 - x.cos();
        let mut acc = Accumulator852::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.289_f32 + y.sin();
        let b = y * 8.612_f32 - x.cos();
        let mut acc = Accumulator852::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.616_f32 + y.sin();
        let b = y * 7.357_f32 - x.cos();
        let mut acc = Accumulator852::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.346_f32 + y.sin();
        let b = y * 4.393_f32 - x.cos();
        let mut acc = Accumulator852::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.156_f32 + y.sin();
        let b = y * 9.1_f32 - x.cos();
        let mut acc = Accumulator852::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.544_f32 + y.sin();
        let b = y * 7.435_f32 - x.cos();
        let mut acc = Accumulator852::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.124_f32 + y.sin();
        let b = y * 3.466_f32 - x.cos();
        let mut acc = Accumulator852::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.979_f32 + y.sin();
        let b = y * 1.42_f32 - x.cos();
        let mut acc = Accumulator852::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.776_f32 + y.sin();
        let b = y * 7.082_f32 - x.cos();
        let mut acc = Accumulator852::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.421_f32 + y.sin();
        let b = y * 3.197_f32 - x.cos();
        let mut acc = Accumulator852::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.767_f32 + y.sin();
        let b = y * 3.878_f32 - x.cos();
        let mut acc = Accumulator852::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.899_f32 + y.sin();
        let b = y * 2.977_f32 - x.cos();
        let mut acc = Accumulator852::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.675_f32 + y.sin();
        let b = y * 7.721_f32 - x.cos();
        let mut acc = Accumulator852::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.893_f32 + y.sin();
        let b = y * 9.697_f32 - x.cos();
        let mut acc = Accumulator852::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.008_f32 + y.sin();
        let b = y * 8.902_f32 - x.cos();
        let mut acc = Accumulator852::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.256_f32 + y.sin();
        let b = y * 9.34_f32 - x.cos();
        let mut acc = Accumulator852::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.715_f32 + y.sin();
        let b = y * 7.753_f32 - x.cos();
        let mut acc = Accumulator852::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.113_f32 + y.sin();
        let b = y * 6.591_f32 - x.cos();
        let mut acc = Accumulator852::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.988_f32 + y.sin();
        let b = y * 7.395_f32 - x.cos();
        let mut acc = Accumulator852::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.824_f32 + y.sin();
        let b = y * 2.189_f32 - x.cos();
        let mut acc = Accumulator852::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_852(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(852u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_852() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_852(total as u64) % 997) as f32;
        total
    }
}

pub mod m853 {
    use super::*;

    pub struct Accumulator853<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator853<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.91_f32 + y.sin();
        let b = y * 3.069_f32 - x.cos();
        let mut acc = Accumulator853::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.223_f32 + y.sin();
        let b = y * 8.225_f32 - x.cos();
        let mut acc = Accumulator853::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.615_f32 + y.sin();
        let b = y * 1.88_f32 - x.cos();
        let mut acc = Accumulator853::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.325_f32 + y.sin();
        let b = y * 6.847_f32 - x.cos();
        let mut acc = Accumulator853::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.156_f32 + y.sin();
        let b = y * 7.785_f32 - x.cos();
        let mut acc = Accumulator853::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.966_f32 + y.sin();
        let b = y * 1.043_f32 - x.cos();
        let mut acc = Accumulator853::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.778_f32 + y.sin();
        let b = y * 8.199_f32 - x.cos();
        let mut acc = Accumulator853::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.424_f32 + y.sin();
        let b = y * 7.101_f32 - x.cos();
        let mut acc = Accumulator853::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.57_f32 + y.sin();
        let b = y * 4.752_f32 - x.cos();
        let mut acc = Accumulator853::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.053_f32 + y.sin();
        let b = y * 7.222_f32 - x.cos();
        let mut acc = Accumulator853::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.701_f32 + y.sin();
        let b = y * 0.652_f32 - x.cos();
        let mut acc = Accumulator853::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.007_f32 + y.sin();
        let b = y * 8.763_f32 - x.cos();
        let mut acc = Accumulator853::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.262_f32 + y.sin();
        let b = y * 3.29_f32 - x.cos();
        let mut acc = Accumulator853::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.39_f32 + y.sin();
        let b = y * 0.881_f32 - x.cos();
        let mut acc = Accumulator853::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.617_f32 + y.sin();
        let b = y * 9.528_f32 - x.cos();
        let mut acc = Accumulator853::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.479_f32 + y.sin();
        let b = y * 5.77_f32 - x.cos();
        let mut acc = Accumulator853::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.298_f32 + y.sin();
        let b = y * 1.811_f32 - x.cos();
        let mut acc = Accumulator853::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.793_f32 + y.sin();
        let b = y * 4.228_f32 - x.cos();
        let mut acc = Accumulator853::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.285_f32 + y.sin();
        let b = y * 8.159_f32 - x.cos();
        let mut acc = Accumulator853::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.313_f32 + y.sin();
        let b = y * 9.739_f32 - x.cos();
        let mut acc = Accumulator853::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_853(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_853() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_853(total as u64) % 997) as f32;
        total
    }
}

pub mod m854 {
    use super::*;

    pub struct Accumulator854<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator854<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.872_f32 + y.sin();
        let b = y * 6.791_f32 - x.cos();
        let mut acc = Accumulator854::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.186_f32 + y.sin();
        let b = y * 4.402_f32 - x.cos();
        let mut acc = Accumulator854::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.161_f32 + y.sin();
        let b = y * 6.836_f32 - x.cos();
        let mut acc = Accumulator854::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.691_f32 + y.sin();
        let b = y * 2.461_f32 - x.cos();
        let mut acc = Accumulator854::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.269_f32 + y.sin();
        let b = y * 3.399_f32 - x.cos();
        let mut acc = Accumulator854::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.46_f32 + y.sin();
        let b = y * 4.854_f32 - x.cos();
        let mut acc = Accumulator854::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.221_f32 + y.sin();
        let b = y * 5.352_f32 - x.cos();
        let mut acc = Accumulator854::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.368_f32 + y.sin();
        let b = y * 7.256_f32 - x.cos();
        let mut acc = Accumulator854::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.22_f32 + y.sin();
        let b = y * 3.039_f32 - x.cos();
        let mut acc = Accumulator854::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.524_f32 + y.sin();
        let b = y * 5.044_f32 - x.cos();
        let mut acc = Accumulator854::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.078_f32 + y.sin();
        let b = y * 8.522_f32 - x.cos();
        let mut acc = Accumulator854::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.303_f32 + y.sin();
        let b = y * 1.859_f32 - x.cos();
        let mut acc = Accumulator854::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.824_f32 + y.sin();
        let b = y * 1.165_f32 - x.cos();
        let mut acc = Accumulator854::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.034_f32 + y.sin();
        let b = y * 9.182_f32 - x.cos();
        let mut acc = Accumulator854::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.012_f32 + y.sin();
        let b = y * 2.206_f32 - x.cos();
        let mut acc = Accumulator854::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.049_f32 + y.sin();
        let b = y * 8.773_f32 - x.cos();
        let mut acc = Accumulator854::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.344_f32 + y.sin();
        let b = y * 7.051_f32 - x.cos();
        let mut acc = Accumulator854::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.366_f32 + y.sin();
        let b = y * 8.278_f32 - x.cos();
        let mut acc = Accumulator854::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.203_f32 + y.sin();
        let b = y * 3.233_f32 - x.cos();
        let mut acc = Accumulator854::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.05_f32 + y.sin();
        let b = y * 7.672_f32 - x.cos();
        let mut acc = Accumulator854::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_854(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_854() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_854(total as u64) % 997) as f32;
        total
    }
}

pub mod m855 {
    use super::*;

    pub struct Accumulator855<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator855<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.182_f32 + y.sin();
        let b = y * 6.847_f32 - x.cos();
        let mut acc = Accumulator855::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.594_f32 + y.sin();
        let b = y * 5.783_f32 - x.cos();
        let mut acc = Accumulator855::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.987_f32 + y.sin();
        let b = y * 7.751_f32 - x.cos();
        let mut acc = Accumulator855::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.53_f32 + y.sin();
        let b = y * 2.402_f32 - x.cos();
        let mut acc = Accumulator855::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.3_f32 + y.sin();
        let b = y * 0.356_f32 - x.cos();
        let mut acc = Accumulator855::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.747_f32 + y.sin();
        let b = y * 2.686_f32 - x.cos();
        let mut acc = Accumulator855::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.768_f32 + y.sin();
        let b = y * 5.975_f32 - x.cos();
        let mut acc = Accumulator855::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.593_f32 + y.sin();
        let b = y * 1.388_f32 - x.cos();
        let mut acc = Accumulator855::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.549_f32 + y.sin();
        let b = y * 9.697_f32 - x.cos();
        let mut acc = Accumulator855::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.237_f32 + y.sin();
        let b = y * 2.364_f32 - x.cos();
        let mut acc = Accumulator855::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.665_f32 + y.sin();
        let b = y * 1.686_f32 - x.cos();
        let mut acc = Accumulator855::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.306_f32 + y.sin();
        let b = y * 3.862_f32 - x.cos();
        let mut acc = Accumulator855::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.959_f32 + y.sin();
        let b = y * 9.848_f32 - x.cos();
        let mut acc = Accumulator855::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.449_f32 + y.sin();
        let b = y * 9.807_f32 - x.cos();
        let mut acc = Accumulator855::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.069_f32 + y.sin();
        let b = y * 9.285_f32 - x.cos();
        let mut acc = Accumulator855::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.122_f32 + y.sin();
        let b = y * 2.359_f32 - x.cos();
        let mut acc = Accumulator855::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.239_f32 + y.sin();
        let b = y * 7.31_f32 - x.cos();
        let mut acc = Accumulator855::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.756_f32 + y.sin();
        let b = y * 2.305_f32 - x.cos();
        let mut acc = Accumulator855::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.871_f32 + y.sin();
        let b = y * 0.553_f32 - x.cos();
        let mut acc = Accumulator855::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.149_f32 + y.sin();
        let b = y * 0.238_f32 - x.cos();
        let mut acc = Accumulator855::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_855(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m855-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_855() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_855(total as u64) % 997) as f32;
        total
    }
}

pub mod m856 {
    use super::*;

    pub struct Accumulator856<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator856<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.191_f32 + y.sin();
        let b = y * 9.439_f32 - x.cos();
        let mut acc = Accumulator856::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.263_f32 + y.sin();
        let b = y * 8.826_f32 - x.cos();
        let mut acc = Accumulator856::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.413_f32 + y.sin();
        let b = y * 5.728_f32 - x.cos();
        let mut acc = Accumulator856::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.937_f32 + y.sin();
        let b = y * 3.8_f32 - x.cos();
        let mut acc = Accumulator856::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.229_f32 + y.sin();
        let b = y * 2.658_f32 - x.cos();
        let mut acc = Accumulator856::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.254_f32 + y.sin();
        let b = y * 5.305_f32 - x.cos();
        let mut acc = Accumulator856::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.57_f32 + y.sin();
        let b = y * 7.068_f32 - x.cos();
        let mut acc = Accumulator856::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.573_f32 + y.sin();
        let b = y * 4.735_f32 - x.cos();
        let mut acc = Accumulator856::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.688_f32 + y.sin();
        let b = y * 4.324_f32 - x.cos();
        let mut acc = Accumulator856::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.342_f32 + y.sin();
        let b = y * 2.741_f32 - x.cos();
        let mut acc = Accumulator856::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.135_f32 + y.sin();
        let b = y * 5.678_f32 - x.cos();
        let mut acc = Accumulator856::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.831_f32 + y.sin();
        let b = y * 6.99_f32 - x.cos();
        let mut acc = Accumulator856::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.183_f32 + y.sin();
        let b = y * 2.937_f32 - x.cos();
        let mut acc = Accumulator856::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.414_f32 + y.sin();
        let b = y * 4.511_f32 - x.cos();
        let mut acc = Accumulator856::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.352_f32 + y.sin();
        let b = y * 1.568_f32 - x.cos();
        let mut acc = Accumulator856::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.161_f32 + y.sin();
        let b = y * 7.508_f32 - x.cos();
        let mut acc = Accumulator856::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.317_f32 + y.sin();
        let b = y * 4.505_f32 - x.cos();
        let mut acc = Accumulator856::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.601_f32 + y.sin();
        let b = y * 2.525_f32 - x.cos();
        let mut acc = Accumulator856::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.865_f32 + y.sin();
        let b = y * 3.866_f32 - x.cos();
        let mut acc = Accumulator856::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.193_f32 + y.sin();
        let b = y * 0.89_f32 - x.cos();
        let mut acc = Accumulator856::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_856(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_856() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_856(total as u64) % 997) as f32;
        total
    }
}

pub mod m857 {
    use super::*;

    pub struct Accumulator857<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator857<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.215_f32 + y.sin();
        let b = y * 1.84_f32 - x.cos();
        let mut acc = Accumulator857::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.293_f32 + y.sin();
        let b = y * 7.292_f32 - x.cos();
        let mut acc = Accumulator857::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.965_f32 + y.sin();
        let b = y * 6.513_f32 - x.cos();
        let mut acc = Accumulator857::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.246_f32 + y.sin();
        let b = y * 3.168_f32 - x.cos();
        let mut acc = Accumulator857::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.277_f32 + y.sin();
        let b = y * 5.595_f32 - x.cos();
        let mut acc = Accumulator857::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.84_f32 + y.sin();
        let b = y * 6.755_f32 - x.cos();
        let mut acc = Accumulator857::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.747_f32 + y.sin();
        let b = y * 7.91_f32 - x.cos();
        let mut acc = Accumulator857::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.34_f32 + y.sin();
        let b = y * 5.478_f32 - x.cos();
        let mut acc = Accumulator857::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.439_f32 + y.sin();
        let b = y * 5.601_f32 - x.cos();
        let mut acc = Accumulator857::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.626_f32 + y.sin();
        let b = y * 9.251_f32 - x.cos();
        let mut acc = Accumulator857::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.425_f32 + y.sin();
        let b = y * 0.969_f32 - x.cos();
        let mut acc = Accumulator857::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.062_f32 + y.sin();
        let b = y * 1.699_f32 - x.cos();
        let mut acc = Accumulator857::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.231_f32 + y.sin();
        let b = y * 7.801_f32 - x.cos();
        let mut acc = Accumulator857::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.171_f32 + y.sin();
        let b = y * 0.675_f32 - x.cos();
        let mut acc = Accumulator857::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.936_f32 + y.sin();
        let b = y * 8.309_f32 - x.cos();
        let mut acc = Accumulator857::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.219_f32 + y.sin();
        let b = y * 3.687_f32 - x.cos();
        let mut acc = Accumulator857::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.56_f32 + y.sin();
        let b = y * 9.393_f32 - x.cos();
        let mut acc = Accumulator857::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.419_f32 + y.sin();
        let b = y * 7.952_f32 - x.cos();
        let mut acc = Accumulator857::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.904_f32 + y.sin();
        let b = y * 6.164_f32 - x.cos();
        let mut acc = Accumulator857::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.769_f32 + y.sin();
        let b = y * 2.261_f32 - x.cos();
        let mut acc = Accumulator857::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_857(seed: u64) -> u64 {
        let re = Regex::new(r"m857-(\d+)").unwrap();
        let hay = format!("m857-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_857() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_857(total as u64) % 997) as f32;
        total
    }
}

pub mod m858 {
    use super::*;

    pub struct Accumulator858<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator858<T> {
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
        let b = y * 5.501_f32 - x.cos();
        let mut acc = Accumulator858::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.204_f32 + y.sin();
        let b = y * 0.421_f32 - x.cos();
        let mut acc = Accumulator858::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.868_f32 + y.sin();
        let b = y * 7.591_f32 - x.cos();
        let mut acc = Accumulator858::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.157_f32 + y.sin();
        let b = y * 9.607_f32 - x.cos();
        let mut acc = Accumulator858::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.739_f32 + y.sin();
        let b = y * 0.86_f32 - x.cos();
        let mut acc = Accumulator858::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.692_f32 + y.sin();
        let b = y * 4.019_f32 - x.cos();
        let mut acc = Accumulator858::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.976_f32 + y.sin();
        let b = y * 8.862_f32 - x.cos();
        let mut acc = Accumulator858::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.415_f32 + y.sin();
        let b = y * 7.997_f32 - x.cos();
        let mut acc = Accumulator858::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.599_f32 + y.sin();
        let b = y * 3.098_f32 - x.cos();
        let mut acc = Accumulator858::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.605_f32 + y.sin();
        let b = y * 5.7_f32 - x.cos();
        let mut acc = Accumulator858::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.432_f32 + y.sin();
        let b = y * 5.039_f32 - x.cos();
        let mut acc = Accumulator858::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.865_f32 + y.sin();
        let b = y * 6.024_f32 - x.cos();
        let mut acc = Accumulator858::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.807_f32 + y.sin();
        let b = y * 8.197_f32 - x.cos();
        let mut acc = Accumulator858::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.791_f32 + y.sin();
        let b = y * 7.032_f32 - x.cos();
        let mut acc = Accumulator858::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.325_f32 + y.sin();
        let b = y * 6.8_f32 - x.cos();
        let mut acc = Accumulator858::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.588_f32 + y.sin();
        let b = y * 9.519_f32 - x.cos();
        let mut acc = Accumulator858::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.453_f32 + y.sin();
        let b = y * 7.666_f32 - x.cos();
        let mut acc = Accumulator858::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.03_f32 + y.sin();
        let b = y * 6.794_f32 - x.cos();
        let mut acc = Accumulator858::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.949_f32 + y.sin();
        let b = y * 5.001_f32 - x.cos();
        let mut acc = Accumulator858::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 2.823_f32 + y.sin();
        let b = y * 8.498_f32 - x.cos();
        let mut acc = Accumulator858::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_858(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_858() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_858(total as u64) % 997) as f32;
        total
    }
}

pub mod m859 {
    use super::*;

    pub struct Accumulator859<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator859<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.112_f32 + y.sin();
        let b = y * 3.831_f32 - x.cos();
        let mut acc = Accumulator859::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.754_f32 + y.sin();
        let b = y * 6.819_f32 - x.cos();
        let mut acc = Accumulator859::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.316_f32 + y.sin();
        let b = y * 1.315_f32 - x.cos();
        let mut acc = Accumulator859::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.681_f32 + y.sin();
        let b = y * 2.714_f32 - x.cos();
        let mut acc = Accumulator859::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.883_f32 + y.sin();
        let b = y * 1.096_f32 - x.cos();
        let mut acc = Accumulator859::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.119_f32 + y.sin();
        let b = y * 8.138_f32 - x.cos();
        let mut acc = Accumulator859::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.541_f32 + y.sin();
        let b = y * 0.44_f32 - x.cos();
        let mut acc = Accumulator859::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.241_f32 + y.sin();
        let b = y * 2.029_f32 - x.cos();
        let mut acc = Accumulator859::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.116_f32 + y.sin();
        let b = y * 4.04_f32 - x.cos();
        let mut acc = Accumulator859::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.974_f32 + y.sin();
        let b = y * 4.708_f32 - x.cos();
        let mut acc = Accumulator859::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.274_f32 + y.sin();
        let b = y * 4.671_f32 - x.cos();
        let mut acc = Accumulator859::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.702_f32 + y.sin();
        let b = y * 8.71_f32 - x.cos();
        let mut acc = Accumulator859::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.855_f32 + y.sin();
        let b = y * 7.094_f32 - x.cos();
        let mut acc = Accumulator859::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.359_f32 + y.sin();
        let b = y * 9.052_f32 - x.cos();
        let mut acc = Accumulator859::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.471_f32 + y.sin();
        let b = y * 7.874_f32 - x.cos();
        let mut acc = Accumulator859::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.295_f32 + y.sin();
        let b = y * 5.377_f32 - x.cos();
        let mut acc = Accumulator859::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.399_f32 + y.sin();
        let b = y * 4.354_f32 - x.cos();
        let mut acc = Accumulator859::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.667_f32 + y.sin();
        let b = y * 8.905_f32 - x.cos();
        let mut acc = Accumulator859::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.611_f32 + y.sin();
        let b = y * 6.632_f32 - x.cos();
        let mut acc = Accumulator859::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.984_f32 + y.sin();
        let b = y * 8.739_f32 - x.cos();
        let mut acc = Accumulator859::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_859(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(859u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_859() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_859(total as u64) % 997) as f32;
        total
    }
}

pub mod m860 {
    use super::*;

    pub struct Accumulator860<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator860<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.814_f32 + y.sin();
        let b = y * 0.18_f32 - x.cos();
        let mut acc = Accumulator860::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.013_f32 + y.sin();
        let b = y * 3.438_f32 - x.cos();
        let mut acc = Accumulator860::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.157_f32 + y.sin();
        let b = y * 6.934_f32 - x.cos();
        let mut acc = Accumulator860::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.916_f32 + y.sin();
        let b = y * 6.519_f32 - x.cos();
        let mut acc = Accumulator860::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.923_f32 + y.sin();
        let b = y * 4.177_f32 - x.cos();
        let mut acc = Accumulator860::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.5_f32 + y.sin();
        let b = y * 7.077_f32 - x.cos();
        let mut acc = Accumulator860::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.543_f32 + y.sin();
        let b = y * 2.394_f32 - x.cos();
        let mut acc = Accumulator860::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.437_f32 + y.sin();
        let b = y * 6.255_f32 - x.cos();
        let mut acc = Accumulator860::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.539_f32 + y.sin();
        let b = y * 9.838_f32 - x.cos();
        let mut acc = Accumulator860::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.044_f32 + y.sin();
        let b = y * 5.986_f32 - x.cos();
        let mut acc = Accumulator860::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.818_f32 + y.sin();
        let b = y * 5.488_f32 - x.cos();
        let mut acc = Accumulator860::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.214_f32 + y.sin();
        let b = y * 1.601_f32 - x.cos();
        let mut acc = Accumulator860::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.791_f32 + y.sin();
        let b = y * 0.937_f32 - x.cos();
        let mut acc = Accumulator860::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.027_f32 + y.sin();
        let b = y * 4.035_f32 - x.cos();
        let mut acc = Accumulator860::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.126_f32 + y.sin();
        let b = y * 5.207_f32 - x.cos();
        let mut acc = Accumulator860::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.681_f32 + y.sin();
        let b = y * 9.165_f32 - x.cos();
        let mut acc = Accumulator860::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.667_f32 + y.sin();
        let b = y * 6.379_f32 - x.cos();
        let mut acc = Accumulator860::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.719_f32 + y.sin();
        let b = y * 1.126_f32 - x.cos();
        let mut acc = Accumulator860::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.245_f32 + y.sin();
        let b = y * 1.025_f32 - x.cos();
        let mut acc = Accumulator860::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.446_f32 + y.sin();
        let b = y * 9.896_f32 - x.cos();
        let mut acc = Accumulator860::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_860(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_860() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_860(total as u64) % 997) as f32;
        total
    }
}

pub mod m861 {
    use super::*;

    pub struct Accumulator861<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator861<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.117_f32 + y.sin();
        let b = y * 7.207_f32 - x.cos();
        let mut acc = Accumulator861::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.339_f32 + y.sin();
        let b = y * 5.59_f32 - x.cos();
        let mut acc = Accumulator861::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.745_f32 + y.sin();
        let b = y * 0.543_f32 - x.cos();
        let mut acc = Accumulator861::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.898_f32 + y.sin();
        let b = y * 9.347_f32 - x.cos();
        let mut acc = Accumulator861::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.65_f32 + y.sin();
        let b = y * 4.628_f32 - x.cos();
        let mut acc = Accumulator861::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.477_f32 + y.sin();
        let b = y * 1.048_f32 - x.cos();
        let mut acc = Accumulator861::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.107_f32 + y.sin();
        let b = y * 8.334_f32 - x.cos();
        let mut acc = Accumulator861::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.559_f32 + y.sin();
        let b = y * 9.658_f32 - x.cos();
        let mut acc = Accumulator861::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.499_f32 + y.sin();
        let b = y * 9.231_f32 - x.cos();
        let mut acc = Accumulator861::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.986_f32 + y.sin();
        let b = y * 1.458_f32 - x.cos();
        let mut acc = Accumulator861::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.964_f32 + y.sin();
        let b = y * 2.393_f32 - x.cos();
        let mut acc = Accumulator861::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.451_f32 + y.sin();
        let b = y * 5.335_f32 - x.cos();
        let mut acc = Accumulator861::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.858_f32 + y.sin();
        let b = y * 4.032_f32 - x.cos();
        let mut acc = Accumulator861::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.738_f32 + y.sin();
        let b = y * 3.193_f32 - x.cos();
        let mut acc = Accumulator861::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.35_f32 + y.sin();
        let b = y * 7.975_f32 - x.cos();
        let mut acc = Accumulator861::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.594_f32 + y.sin();
        let b = y * 1.346_f32 - x.cos();
        let mut acc = Accumulator861::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.3_f32 + y.sin();
        let b = y * 9.251_f32 - x.cos();
        let mut acc = Accumulator861::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.125_f32 + y.sin();
        let b = y * 3.28_f32 - x.cos();
        let mut acc = Accumulator861::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.195_f32 + y.sin();
        let b = y * 1.389_f32 - x.cos();
        let mut acc = Accumulator861::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.669_f32 + y.sin();
        let b = y * 7.257_f32 - x.cos();
        let mut acc = Accumulator861::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_861(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_861() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_861(total as u64) % 997) as f32;
        total
    }
}

pub mod m862 {
    use super::*;

    pub struct Accumulator862<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator862<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.237_f32 + y.sin();
        let b = y * 4.321_f32 - x.cos();
        let mut acc = Accumulator862::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.717_f32 + y.sin();
        let b = y * 2.55_f32 - x.cos();
        let mut acc = Accumulator862::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.85_f32 + y.sin();
        let b = y * 8.33_f32 - x.cos();
        let mut acc = Accumulator862::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.667_f32 + y.sin();
        let b = y * 2.217_f32 - x.cos();
        let mut acc = Accumulator862::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.574_f32 + y.sin();
        let b = y * 8.051_f32 - x.cos();
        let mut acc = Accumulator862::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.253_f32 + y.sin();
        let b = y * 1.228_f32 - x.cos();
        let mut acc = Accumulator862::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.545_f32 + y.sin();
        let b = y * 4.817_f32 - x.cos();
        let mut acc = Accumulator862::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.721_f32 + y.sin();
        let b = y * 7.299_f32 - x.cos();
        let mut acc = Accumulator862::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.464_f32 + y.sin();
        let b = y * 1.154_f32 - x.cos();
        let mut acc = Accumulator862::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.341_f32 + y.sin();
        let b = y * 2.156_f32 - x.cos();
        let mut acc = Accumulator862::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.432_f32 + y.sin();
        let b = y * 8.137_f32 - x.cos();
        let mut acc = Accumulator862::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.843_f32 + y.sin();
        let b = y * 5.185_f32 - x.cos();
        let mut acc = Accumulator862::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.781_f32 + y.sin();
        let b = y * 1.658_f32 - x.cos();
        let mut acc = Accumulator862::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.748_f32 + y.sin();
        let b = y * 7.783_f32 - x.cos();
        let mut acc = Accumulator862::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.719_f32 + y.sin();
        let b = y * 3.307_f32 - x.cos();
        let mut acc = Accumulator862::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.812_f32 + y.sin();
        let b = y * 8.889_f32 - x.cos();
        let mut acc = Accumulator862::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.163_f32 + y.sin();
        let b = y * 4.613_f32 - x.cos();
        let mut acc = Accumulator862::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.402_f32 + y.sin();
        let b = y * 5.304_f32 - x.cos();
        let mut acc = Accumulator862::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.133_f32 + y.sin();
        let b = y * 2.12_f32 - x.cos();
        let mut acc = Accumulator862::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.401_f32 + y.sin();
        let b = y * 4.23_f32 - x.cos();
        let mut acc = Accumulator862::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_862(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m862-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_862() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_862(total as u64) % 997) as f32;
        total
    }
}

pub mod m863 {
    use super::*;

    pub struct Accumulator863<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator863<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.938_f32 + y.sin();
        let b = y * 8.059_f32 - x.cos();
        let mut acc = Accumulator863::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.042_f32 + y.sin();
        let b = y * 2.498_f32 - x.cos();
        let mut acc = Accumulator863::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 0.374_f32 + y.sin();
        let b = y * 9.447_f32 - x.cos();
        let mut acc = Accumulator863::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 3.692_f32 + y.sin();
        let b = y * 2.937_f32 - x.cos();
        let mut acc = Accumulator863::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.443_f32 + y.sin();
        let b = y * 5.74_f32 - x.cos();
        let mut acc = Accumulator863::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.403_f32 + y.sin();
        let b = y * 3.762_f32 - x.cos();
        let mut acc = Accumulator863::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.527_f32 + y.sin();
        let b = y * 1.885_f32 - x.cos();
        let mut acc = Accumulator863::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.938_f32 + y.sin();
        let b = y * 1.344_f32 - x.cos();
        let mut acc = Accumulator863::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.32_f32 + y.sin();
        let b = y * 9.672_f32 - x.cos();
        let mut acc = Accumulator863::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.451_f32 + y.sin();
        let b = y * 6.409_f32 - x.cos();
        let mut acc = Accumulator863::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.339_f32 + y.sin();
        let b = y * 4.434_f32 - x.cos();
        let mut acc = Accumulator863::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.038_f32 + y.sin();
        let b = y * 7.661_f32 - x.cos();
        let mut acc = Accumulator863::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.422_f32 + y.sin();
        let b = y * 1.076_f32 - x.cos();
        let mut acc = Accumulator863::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.738_f32 + y.sin();
        let b = y * 0.999_f32 - x.cos();
        let mut acc = Accumulator863::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.688_f32 + y.sin();
        let b = y * 3.322_f32 - x.cos();
        let mut acc = Accumulator863::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.475_f32 + y.sin();
        let b = y * 4.84_f32 - x.cos();
        let mut acc = Accumulator863::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.03_f32 + y.sin();
        let b = y * 1.425_f32 - x.cos();
        let mut acc = Accumulator863::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.707_f32 + y.sin();
        let b = y * 4.436_f32 - x.cos();
        let mut acc = Accumulator863::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.903_f32 + y.sin();
        let b = y * 0.693_f32 - x.cos();
        let mut acc = Accumulator863::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.676_f32 + y.sin();
        let b = y * 9.359_f32 - x.cos();
        let mut acc = Accumulator863::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_863(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_863() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_863(total as u64) % 997) as f32;
        total
    }
}

pub mod m864 {
    use super::*;

    pub struct Accumulator864<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator864<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.7_f32 + y.sin();
        let b = y * 2.36_f32 - x.cos();
        let mut acc = Accumulator864::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.724_f32 + y.sin();
        let b = y * 7.239_f32 - x.cos();
        let mut acc = Accumulator864::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.235_f32 + y.sin();
        let b = y * 3.288_f32 - x.cos();
        let mut acc = Accumulator864::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.552_f32 + y.sin();
        let b = y * 8.268_f32 - x.cos();
        let mut acc = Accumulator864::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.369_f32 + y.sin();
        let b = y * 3.71_f32 - x.cos();
        let mut acc = Accumulator864::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.768_f32 + y.sin();
        let b = y * 0.693_f32 - x.cos();
        let mut acc = Accumulator864::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.133_f32 + y.sin();
        let b = y * 5.513_f32 - x.cos();
        let mut acc = Accumulator864::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.364_f32 + y.sin();
        let b = y * 0.651_f32 - x.cos();
        let mut acc = Accumulator864::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.278_f32 + y.sin();
        let b = y * 0.454_f32 - x.cos();
        let mut acc = Accumulator864::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.724_f32 + y.sin();
        let b = y * 2.116_f32 - x.cos();
        let mut acc = Accumulator864::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.775_f32 + y.sin();
        let b = y * 7.124_f32 - x.cos();
        let mut acc = Accumulator864::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.086_f32 + y.sin();
        let b = y * 0.269_f32 - x.cos();
        let mut acc = Accumulator864::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.6_f32 + y.sin();
        let b = y * 7.771_f32 - x.cos();
        let mut acc = Accumulator864::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.732_f32 + y.sin();
        let b = y * 4.506_f32 - x.cos();
        let mut acc = Accumulator864::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.496_f32 + y.sin();
        let b = y * 6.94_f32 - x.cos();
        let mut acc = Accumulator864::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.323_f32 + y.sin();
        let b = y * 1.204_f32 - x.cos();
        let mut acc = Accumulator864::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.061_f32 + y.sin();
        let b = y * 1.537_f32 - x.cos();
        let mut acc = Accumulator864::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.789_f32 + y.sin();
        let b = y * 2.671_f32 - x.cos();
        let mut acc = Accumulator864::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.541_f32 + y.sin();
        let b = y * 8.586_f32 - x.cos();
        let mut acc = Accumulator864::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.367_f32 + y.sin();
        let b = y * 7.575_f32 - x.cos();
        let mut acc = Accumulator864::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_864(seed: u64) -> u64 {
        let re = Regex::new(r"m864-(\d+)").unwrap();
        let hay = format!("m864-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_864() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_864(total as u64) % 997) as f32;
        total
    }
}

pub mod m865 {
    use super::*;

    pub struct Accumulator865<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator865<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.574_f32 + y.sin();
        let b = y * 4.677_f32 - x.cos();
        let mut acc = Accumulator865::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.566_f32 + y.sin();
        let b = y * 7.131_f32 - x.cos();
        let mut acc = Accumulator865::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.66_f32 + y.sin();
        let b = y * 9.845_f32 - x.cos();
        let mut acc = Accumulator865::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.44_f32 + y.sin();
        let b = y * 8.067_f32 - x.cos();
        let mut acc = Accumulator865::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.524_f32 + y.sin();
        let b = y * 0.947_f32 - x.cos();
        let mut acc = Accumulator865::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.801_f32 + y.sin();
        let b = y * 3.261_f32 - x.cos();
        let mut acc = Accumulator865::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.208_f32 + y.sin();
        let b = y * 3.219_f32 - x.cos();
        let mut acc = Accumulator865::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.015_f32 + y.sin();
        let b = y * 4.65_f32 - x.cos();
        let mut acc = Accumulator865::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.57_f32 + y.sin();
        let b = y * 9.765_f32 - x.cos();
        let mut acc = Accumulator865::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.046_f32 + y.sin();
        let b = y * 4.714_f32 - x.cos();
        let mut acc = Accumulator865::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.14_f32 + y.sin();
        let b = y * 8.223_f32 - x.cos();
        let mut acc = Accumulator865::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.005_f32 + y.sin();
        let b = y * 5.284_f32 - x.cos();
        let mut acc = Accumulator865::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.227_f32 + y.sin();
        let b = y * 8.84_f32 - x.cos();
        let mut acc = Accumulator865::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.81_f32 + y.sin();
        let b = y * 8.135_f32 - x.cos();
        let mut acc = Accumulator865::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.616_f32 + y.sin();
        let b = y * 3.282_f32 - x.cos();
        let mut acc = Accumulator865::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.369_f32 + y.sin();
        let b = y * 5.563_f32 - x.cos();
        let mut acc = Accumulator865::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.287_f32 + y.sin();
        let b = y * 8.366_f32 - x.cos();
        let mut acc = Accumulator865::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.244_f32 + y.sin();
        let b = y * 7.034_f32 - x.cos();
        let mut acc = Accumulator865::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.464_f32 + y.sin();
        let b = y * 0.346_f32 - x.cos();
        let mut acc = Accumulator865::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.701_f32 + y.sin();
        let b = y * 8.721_f32 - x.cos();
        let mut acc = Accumulator865::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_865(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_865() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_865(total as u64) % 997) as f32;
        total
    }
}

pub mod m866 {
    use super::*;

    pub struct Accumulator866<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator866<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.316_f32 + y.sin();
        let b = y * 3.785_f32 - x.cos();
        let mut acc = Accumulator866::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.268_f32 + y.sin();
        let b = y * 8.247_f32 - x.cos();
        let mut acc = Accumulator866::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.087_f32 + y.sin();
        let b = y * 2.424_f32 - x.cos();
        let mut acc = Accumulator866::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.329_f32 + y.sin();
        let b = y * 0.12_f32 - x.cos();
        let mut acc = Accumulator866::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.369_f32 + y.sin();
        let b = y * 6.283_f32 - x.cos();
        let mut acc = Accumulator866::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.956_f32 + y.sin();
        let b = y * 9.42_f32 - x.cos();
        let mut acc = Accumulator866::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.796_f32 + y.sin();
        let b = y * 1.933_f32 - x.cos();
        let mut acc = Accumulator866::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.774_f32 + y.sin();
        let b = y * 2.283_f32 - x.cos();
        let mut acc = Accumulator866::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 2.084_f32 + y.sin();
        let b = y * 7.777_f32 - x.cos();
        let mut acc = Accumulator866::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.984_f32 + y.sin();
        let b = y * 7.003_f32 - x.cos();
        let mut acc = Accumulator866::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.286_f32 + y.sin();
        let b = y * 1.094_f32 - x.cos();
        let mut acc = Accumulator866::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.554_f32 + y.sin();
        let b = y * 3.167_f32 - x.cos();
        let mut acc = Accumulator866::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.728_f32 + y.sin();
        let b = y * 2.581_f32 - x.cos();
        let mut acc = Accumulator866::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.548_f32 + y.sin();
        let b = y * 1.147_f32 - x.cos();
        let mut acc = Accumulator866::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.475_f32 + y.sin();
        let b = y * 7.374_f32 - x.cos();
        let mut acc = Accumulator866::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.868_f32 + y.sin();
        let b = y * 5.035_f32 - x.cos();
        let mut acc = Accumulator866::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.502_f32 + y.sin();
        let b = y * 4.976_f32 - x.cos();
        let mut acc = Accumulator866::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.952_f32 + y.sin();
        let b = y * 2.964_f32 - x.cos();
        let mut acc = Accumulator866::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.682_f32 + y.sin();
        let b = y * 6.224_f32 - x.cos();
        let mut acc = Accumulator866::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.607_f32 + y.sin();
        let b = y * 7.61_f32 - x.cos();
        let mut acc = Accumulator866::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_866(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(866u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_866() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_866(total as u64) % 997) as f32;
        total
    }
}

pub mod m867 {
    use super::*;

    pub struct Accumulator867<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator867<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.67_f32 + y.sin();
        let b = y * 3.391_f32 - x.cos();
        let mut acc = Accumulator867::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.343_f32 + y.sin();
        let b = y * 3.504_f32 - x.cos();
        let mut acc = Accumulator867::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.833_f32 + y.sin();
        let b = y * 6.182_f32 - x.cos();
        let mut acc = Accumulator867::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.735_f32 + y.sin();
        let b = y * 0.689_f32 - x.cos();
        let mut acc = Accumulator867::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.59_f32 + y.sin();
        let b = y * 7.047_f32 - x.cos();
        let mut acc = Accumulator867::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.462_f32 + y.sin();
        let b = y * 8.051_f32 - x.cos();
        let mut acc = Accumulator867::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.189_f32 + y.sin();
        let b = y * 8.563_f32 - x.cos();
        let mut acc = Accumulator867::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.084_f32 + y.sin();
        let b = y * 0.848_f32 - x.cos();
        let mut acc = Accumulator867::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.423_f32 + y.sin();
        let b = y * 3.566_f32 - x.cos();
        let mut acc = Accumulator867::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.99_f32 + y.sin();
        let b = y * 5.881_f32 - x.cos();
        let mut acc = Accumulator867::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.308_f32 + y.sin();
        let b = y * 6.347_f32 - x.cos();
        let mut acc = Accumulator867::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.843_f32 + y.sin();
        let b = y * 7.074_f32 - x.cos();
        let mut acc = Accumulator867::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.912_f32 + y.sin();
        let b = y * 6.18_f32 - x.cos();
        let mut acc = Accumulator867::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.734_f32 + y.sin();
        let b = y * 6.173_f32 - x.cos();
        let mut acc = Accumulator867::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.737_f32 + y.sin();
        let b = y * 7.268_f32 - x.cos();
        let mut acc = Accumulator867::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.575_f32 + y.sin();
        let b = y * 3.096_f32 - x.cos();
        let mut acc = Accumulator867::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.615_f32 + y.sin();
        let b = y * 4.042_f32 - x.cos();
        let mut acc = Accumulator867::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.366_f32 + y.sin();
        let b = y * 5.402_f32 - x.cos();
        let mut acc = Accumulator867::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.599_f32 + y.sin();
        let b = y * 6.511_f32 - x.cos();
        let mut acc = Accumulator867::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.61_f32 + y.sin();
        let b = y * 4.258_f32 - x.cos();
        let mut acc = Accumulator867::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_867(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_867() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_867(total as u64) % 997) as f32;
        total
    }
}

pub mod m868 {
    use super::*;

    pub struct Accumulator868<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator868<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.696_f32 + y.sin();
        let b = y * 3.015_f32 - x.cos();
        let mut acc = Accumulator868::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.985_f32 + y.sin();
        let b = y * 2.102_f32 - x.cos();
        let mut acc = Accumulator868::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.829_f32 + y.sin();
        let b = y * 3.085_f32 - x.cos();
        let mut acc = Accumulator868::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.071_f32 + y.sin();
        let b = y * 2.406_f32 - x.cos();
        let mut acc = Accumulator868::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.886_f32 + y.sin();
        let b = y * 0.898_f32 - x.cos();
        let mut acc = Accumulator868::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.722_f32 + y.sin();
        let b = y * 2.083_f32 - x.cos();
        let mut acc = Accumulator868::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.595_f32 + y.sin();
        let b = y * 1.62_f32 - x.cos();
        let mut acc = Accumulator868::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.718_f32 + y.sin();
        let b = y * 3.875_f32 - x.cos();
        let mut acc = Accumulator868::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.192_f32 + y.sin();
        let b = y * 5.86_f32 - x.cos();
        let mut acc = Accumulator868::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.872_f32 + y.sin();
        let b = y * 0.617_f32 - x.cos();
        let mut acc = Accumulator868::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.923_f32 + y.sin();
        let b = y * 9.21_f32 - x.cos();
        let mut acc = Accumulator868::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.295_f32 + y.sin();
        let b = y * 7.206_f32 - x.cos();
        let mut acc = Accumulator868::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.784_f32 + y.sin();
        let b = y * 2.333_f32 - x.cos();
        let mut acc = Accumulator868::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.38_f32 + y.sin();
        let b = y * 7.597_f32 - x.cos();
        let mut acc = Accumulator868::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.346_f32 + y.sin();
        let b = y * 7.068_f32 - x.cos();
        let mut acc = Accumulator868::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.494_f32 + y.sin();
        let b = y * 0.81_f32 - x.cos();
        let mut acc = Accumulator868::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.672_f32 + y.sin();
        let b = y * 3.888_f32 - x.cos();
        let mut acc = Accumulator868::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.55_f32 + y.sin();
        let b = y * 0.653_f32 - x.cos();
        let mut acc = Accumulator868::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.045_f32 + y.sin();
        let b = y * 7.285_f32 - x.cos();
        let mut acc = Accumulator868::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.233_f32 + y.sin();
        let b = y * 9.173_f32 - x.cos();
        let mut acc = Accumulator868::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_868(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_868() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_868(total as u64) % 997) as f32;
        total
    }
}

pub mod m869 {
    use super::*;

    pub struct Accumulator869<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator869<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.008_f32 + y.sin();
        let b = y * 9.003_f32 - x.cos();
        let mut acc = Accumulator869::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.468_f32 + y.sin();
        let b = y * 8.469_f32 - x.cos();
        let mut acc = Accumulator869::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.721_f32 + y.sin();
        let b = y * 0.583_f32 - x.cos();
        let mut acc = Accumulator869::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.631_f32 + y.sin();
        let b = y * 4.862_f32 - x.cos();
        let mut acc = Accumulator869::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.286_f32 + y.sin();
        let b = y * 2.302_f32 - x.cos();
        let mut acc = Accumulator869::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.824_f32 + y.sin();
        let b = y * 3.899_f32 - x.cos();
        let mut acc = Accumulator869::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.25_f32 + y.sin();
        let b = y * 1.33_f32 - x.cos();
        let mut acc = Accumulator869::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.792_f32 + y.sin();
        let b = y * 2.916_f32 - x.cos();
        let mut acc = Accumulator869::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.3_f32 + y.sin();
        let b = y * 2.1_f32 - x.cos();
        let mut acc = Accumulator869::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.569_f32 + y.sin();
        let b = y * 2.898_f32 - x.cos();
        let mut acc = Accumulator869::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.133_f32 + y.sin();
        let b = y * 4.012_f32 - x.cos();
        let mut acc = Accumulator869::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.639_f32 + y.sin();
        let b = y * 0.581_f32 - x.cos();
        let mut acc = Accumulator869::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.158_f32 + y.sin();
        let b = y * 8.451_f32 - x.cos();
        let mut acc = Accumulator869::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.457_f32 + y.sin();
        let b = y * 9.111_f32 - x.cos();
        let mut acc = Accumulator869::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.466_f32 + y.sin();
        let b = y * 8.386_f32 - x.cos();
        let mut acc = Accumulator869::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.439_f32 + y.sin();
        let b = y * 6.897_f32 - x.cos();
        let mut acc = Accumulator869::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.013_f32 + y.sin();
        let b = y * 9.733_f32 - x.cos();
        let mut acc = Accumulator869::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.643_f32 + y.sin();
        let b = y * 1.23_f32 - x.cos();
        let mut acc = Accumulator869::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.183_f32 + y.sin();
        let b = y * 8.936_f32 - x.cos();
        let mut acc = Accumulator869::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.462_f32 + y.sin();
        let b = y * 8.661_f32 - x.cos();
        let mut acc = Accumulator869::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_869(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m869-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_869() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_869(total as u64) % 997) as f32;
        total
    }
}

pub mod m870 {
    use super::*;

    pub struct Accumulator870<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator870<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.978_f32 + y.sin();
        let b = y * 6.938_f32 - x.cos();
        let mut acc = Accumulator870::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.427_f32 + y.sin();
        let b = y * 4.45_f32 - x.cos();
        let mut acc = Accumulator870::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.074_f32 + y.sin();
        let b = y * 2.377_f32 - x.cos();
        let mut acc = Accumulator870::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.933_f32 + y.sin();
        let b = y * 3.803_f32 - x.cos();
        let mut acc = Accumulator870::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.644_f32 + y.sin();
        let b = y * 6.41_f32 - x.cos();
        let mut acc = Accumulator870::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.403_f32 + y.sin();
        let b = y * 2.585_f32 - x.cos();
        let mut acc = Accumulator870::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.506_f32 + y.sin();
        let b = y * 7.462_f32 - x.cos();
        let mut acc = Accumulator870::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.264_f32 + y.sin();
        let b = y * 0.595_f32 - x.cos();
        let mut acc = Accumulator870::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.36_f32 + y.sin();
        let b = y * 1.451_f32 - x.cos();
        let mut acc = Accumulator870::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.367_f32 + y.sin();
        let b = y * 5.193_f32 - x.cos();
        let mut acc = Accumulator870::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.054_f32 + y.sin();
        let b = y * 8.73_f32 - x.cos();
        let mut acc = Accumulator870::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.05_f32 + y.sin();
        let b = y * 9.554_f32 - x.cos();
        let mut acc = Accumulator870::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.738_f32 + y.sin();
        let b = y * 8.16_f32 - x.cos();
        let mut acc = Accumulator870::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.908_f32 + y.sin();
        let b = y * 9.149_f32 - x.cos();
        let mut acc = Accumulator870::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.148_f32 + y.sin();
        let b = y * 8.816_f32 - x.cos();
        let mut acc = Accumulator870::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.503_f32 + y.sin();
        let b = y * 9.829_f32 - x.cos();
        let mut acc = Accumulator870::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 5.682_f32 + y.sin();
        let b = y * 2.38_f32 - x.cos();
        let mut acc = Accumulator870::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.963_f32 + y.sin();
        let b = y * 2.562_f32 - x.cos();
        let mut acc = Accumulator870::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.205_f32 + y.sin();
        let b = y * 5.35_f32 - x.cos();
        let mut acc = Accumulator870::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.775_f32 + y.sin();
        let b = y * 3.388_f32 - x.cos();
        let mut acc = Accumulator870::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_870(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_870() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_870(total as u64) % 997) as f32;
        total
    }
}

pub mod m871 {
    use super::*;

    pub struct Accumulator871<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator871<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.09_f32 + y.sin();
        let b = y * 8.289_f32 - x.cos();
        let mut acc = Accumulator871::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.529_f32 + y.sin();
        let b = y * 5.113_f32 - x.cos();
        let mut acc = Accumulator871::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 1.712_f32 + y.sin();
        let b = y * 2.92_f32 - x.cos();
        let mut acc = Accumulator871::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.121_f32 + y.sin();
        let b = y * 7.365_f32 - x.cos();
        let mut acc = Accumulator871::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.984_f32 + y.sin();
        let b = y * 0.773_f32 - x.cos();
        let mut acc = Accumulator871::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.533_f32 + y.sin();
        let b = y * 7.169_f32 - x.cos();
        let mut acc = Accumulator871::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 4.067_f32 + y.sin();
        let b = y * 0.693_f32 - x.cos();
        let mut acc = Accumulator871::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.666_f32 + y.sin();
        let b = y * 1.376_f32 - x.cos();
        let mut acc = Accumulator871::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.386_f32 + y.sin();
        let b = y * 0.438_f32 - x.cos();
        let mut acc = Accumulator871::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.403_f32 + y.sin();
        let b = y * 1.357_f32 - x.cos();
        let mut acc = Accumulator871::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.122_f32 + y.sin();
        let b = y * 9.43_f32 - x.cos();
        let mut acc = Accumulator871::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.825_f32 + y.sin();
        let b = y * 3.984_f32 - x.cos();
        let mut acc = Accumulator871::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 3.317_f32 + y.sin();
        let b = y * 4.742_f32 - x.cos();
        let mut acc = Accumulator871::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.728_f32 + y.sin();
        let b = y * 8.594_f32 - x.cos();
        let mut acc = Accumulator871::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.796_f32 + y.sin();
        let b = y * 8.776_f32 - x.cos();
        let mut acc = Accumulator871::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.021_f32 + y.sin();
        let b = y * 0.911_f32 - x.cos();
        let mut acc = Accumulator871::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.133_f32 + y.sin();
        let b = y * 8.362_f32 - x.cos();
        let mut acc = Accumulator871::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.163_f32 + y.sin();
        let b = y * 4.621_f32 - x.cos();
        let mut acc = Accumulator871::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 7.353_f32 + y.sin();
        let b = y * 6.186_f32 - x.cos();
        let mut acc = Accumulator871::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.269_f32 + y.sin();
        let b = y * 6.512_f32 - x.cos();
        let mut acc = Accumulator871::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_871(seed: u64) -> u64 {
        let re = Regex::new(r"m871-(\d+)").unwrap();
        let hay = format!("m871-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_871() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_871(total as u64) % 997) as f32;
        total
    }
}

pub mod m872 {
    use super::*;

    pub struct Accumulator872<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator872<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.084_f32 + y.sin();
        let b = y * 3.088_f32 - x.cos();
        let mut acc = Accumulator872::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.113_f32 + y.sin();
        let b = y * 5.537_f32 - x.cos();
        let mut acc = Accumulator872::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.344_f32 + y.sin();
        let b = y * 0.325_f32 - x.cos();
        let mut acc = Accumulator872::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.951_f32 + y.sin();
        let b = y * 0.707_f32 - x.cos();
        let mut acc = Accumulator872::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.075_f32 + y.sin();
        let b = y * 4.097_f32 - x.cos();
        let mut acc = Accumulator872::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.238_f32 + y.sin();
        let b = y * 0.189_f32 - x.cos();
        let mut acc = Accumulator872::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.603_f32 + y.sin();
        let b = y * 1.986_f32 - x.cos();
        let mut acc = Accumulator872::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.637_f32 + y.sin();
        let b = y * 1.347_f32 - x.cos();
        let mut acc = Accumulator872::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.434_f32 + y.sin();
        let b = y * 4.351_f32 - x.cos();
        let mut acc = Accumulator872::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.561_f32 + y.sin();
        let b = y * 3.18_f32 - x.cos();
        let mut acc = Accumulator872::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.109_f32 + y.sin();
        let b = y * 9.243_f32 - x.cos();
        let mut acc = Accumulator872::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 8.671_f32 + y.sin();
        let b = y * 5.983_f32 - x.cos();
        let mut acc = Accumulator872::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.557_f32 + y.sin();
        let b = y * 7.228_f32 - x.cos();
        let mut acc = Accumulator872::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.479_f32 + y.sin();
        let b = y * 1.251_f32 - x.cos();
        let mut acc = Accumulator872::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.638_f32 + y.sin();
        let b = y * 0.746_f32 - x.cos();
        let mut acc = Accumulator872::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.228_f32 + y.sin();
        let b = y * 4.299_f32 - x.cos();
        let mut acc = Accumulator872::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.741_f32 + y.sin();
        let b = y * 5.417_f32 - x.cos();
        let mut acc = Accumulator872::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.227_f32 + y.sin();
        let b = y * 1.895_f32 - x.cos();
        let mut acc = Accumulator872::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 6.724_f32 + y.sin();
        let b = y * 2.654_f32 - x.cos();
        let mut acc = Accumulator872::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.336_f32 + y.sin();
        let b = y * 6.592_f32 - x.cos();
        let mut acc = Accumulator872::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_872(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_872() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_872(total as u64) % 997) as f32;
        total
    }
}

pub mod m873 {
    use super::*;

    pub struct Accumulator873<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator873<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.966_f32 + y.sin();
        let b = y * 6.913_f32 - x.cos();
        let mut acc = Accumulator873::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.876_f32 + y.sin();
        let b = y * 4.682_f32 - x.cos();
        let mut acc = Accumulator873::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.134_f32 + y.sin();
        let b = y * 5.883_f32 - x.cos();
        let mut acc = Accumulator873::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.231_f32 + y.sin();
        let b = y * 3.266_f32 - x.cos();
        let mut acc = Accumulator873::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.448_f32 + y.sin();
        let b = y * 6.254_f32 - x.cos();
        let mut acc = Accumulator873::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.191_f32 + y.sin();
        let b = y * 8.734_f32 - x.cos();
        let mut acc = Accumulator873::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.675_f32 + y.sin();
        let b = y * 5.969_f32 - x.cos();
        let mut acc = Accumulator873::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.485_f32 + y.sin();
        let b = y * 8.841_f32 - x.cos();
        let mut acc = Accumulator873::new(a);
        acc.add(b);
        (acc.value * 73_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.182_f32 + y.sin();
        let b = y * 2.322_f32 - x.cos();
        let mut acc = Accumulator873::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.136_f32 + y.sin();
        let b = y * 0.826_f32 - x.cos();
        let mut acc = Accumulator873::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.76_f32 + y.sin();
        let b = y * 2.442_f32 - x.cos();
        let mut acc = Accumulator873::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.116_f32 + y.sin();
        let b = y * 0.874_f32 - x.cos();
        let mut acc = Accumulator873::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.652_f32 + y.sin();
        let b = y * 6.741_f32 - x.cos();
        let mut acc = Accumulator873::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.14_f32 + y.sin();
        let b = y * 2.557_f32 - x.cos();
        let mut acc = Accumulator873::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.203_f32 + y.sin();
        let b = y * 4.678_f32 - x.cos();
        let mut acc = Accumulator873::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 7.546_f32 + y.sin();
        let b = y * 1.334_f32 - x.cos();
        let mut acc = Accumulator873::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.484_f32 + y.sin();
        let b = y * 0.378_f32 - x.cos();
        let mut acc = Accumulator873::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.364_f32 + y.sin();
        let b = y * 8.376_f32 - x.cos();
        let mut acc = Accumulator873::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 1.638_f32 + y.sin();
        let b = y * 5.786_f32 - x.cos();
        let mut acc = Accumulator873::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.249_f32 + y.sin();
        let b = y * 2.575_f32 - x.cos();
        let mut acc = Accumulator873::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_873(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(873u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_873() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_873(total as u64) % 997) as f32;
        total
    }
}

pub mod m874 {
    use super::*;

    pub struct Accumulator874<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator874<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.72_f32 + y.sin();
        let b = y * 3.137_f32 - x.cos();
        let mut acc = Accumulator874::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.466_f32 + y.sin();
        let b = y * 4.701_f32 - x.cos();
        let mut acc = Accumulator874::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.663_f32 + y.sin();
        let b = y * 2.161_f32 - x.cos();
        let mut acc = Accumulator874::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.667_f32 + y.sin();
        let b = y * 1.841_f32 - x.cos();
        let mut acc = Accumulator874::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.798_f32 + y.sin();
        let b = y * 3.202_f32 - x.cos();
        let mut acc = Accumulator874::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.182_f32 + y.sin();
        let b = y * 3.921_f32 - x.cos();
        let mut acc = Accumulator874::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.844_f32 + y.sin();
        let b = y * 0.904_f32 - x.cos();
        let mut acc = Accumulator874::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.164_f32 + y.sin();
        let b = y * 8.196_f32 - x.cos();
        let mut acc = Accumulator874::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.848_f32 + y.sin();
        let b = y * 5.129_f32 - x.cos();
        let mut acc = Accumulator874::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.97_f32 + y.sin();
        let b = y * 6.051_f32 - x.cos();
        let mut acc = Accumulator874::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.712_f32 + y.sin();
        let b = y * 4.663_f32 - x.cos();
        let mut acc = Accumulator874::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.543_f32 + y.sin();
        let b = y * 0.278_f32 - x.cos();
        let mut acc = Accumulator874::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.745_f32 + y.sin();
        let b = y * 9.774_f32 - x.cos();
        let mut acc = Accumulator874::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.485_f32 + y.sin();
        let b = y * 8.241_f32 - x.cos();
        let mut acc = Accumulator874::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.057_f32 + y.sin();
        let b = y * 8.221_f32 - x.cos();
        let mut acc = Accumulator874::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.272_f32 + y.sin();
        let b = y * 4.405_f32 - x.cos();
        let mut acc = Accumulator874::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.492_f32 + y.sin();
        let b = y * 6.607_f32 - x.cos();
        let mut acc = Accumulator874::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.824_f32 + y.sin();
        let b = y * 7.617_f32 - x.cos();
        let mut acc = Accumulator874::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.904_f32 + y.sin();
        let b = y * 2.629_f32 - x.cos();
        let mut acc = Accumulator874::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.578_f32 + y.sin();
        let b = y * 0.304_f32 - x.cos();
        let mut acc = Accumulator874::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_874(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_874() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_874(total as u64) % 997) as f32;
        total
    }
}

pub mod m875 {
    use super::*;

    pub struct Accumulator875<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator875<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.85_f32 + y.sin();
        let b = y * 5.416_f32 - x.cos();
        let mut acc = Accumulator875::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.852_f32 + y.sin();
        let b = y * 2.385_f32 - x.cos();
        let mut acc = Accumulator875::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.19_f32 + y.sin();
        let b = y * 1.104_f32 - x.cos();
        let mut acc = Accumulator875::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.55_f32 + y.sin();
        let b = y * 4.893_f32 - x.cos();
        let mut acc = Accumulator875::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.822_f32 + y.sin();
        let b = y * 8.648_f32 - x.cos();
        let mut acc = Accumulator875::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.544_f32 + y.sin();
        let b = y * 2.081_f32 - x.cos();
        let mut acc = Accumulator875::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.194_f32 + y.sin();
        let b = y * 5.278_f32 - x.cos();
        let mut acc = Accumulator875::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.856_f32 + y.sin();
        let b = y * 5.552_f32 - x.cos();
        let mut acc = Accumulator875::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.64_f32 + y.sin();
        let b = y * 9.39_f32 - x.cos();
        let mut acc = Accumulator875::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.979_f32 + y.sin();
        let b = y * 4.524_f32 - x.cos();
        let mut acc = Accumulator875::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.134_f32 + y.sin();
        let b = y * 1.749_f32 - x.cos();
        let mut acc = Accumulator875::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.33_f32 + y.sin();
        let b = y * 5.801_f32 - x.cos();
        let mut acc = Accumulator875::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.316_f32 + y.sin();
        let b = y * 1.629_f32 - x.cos();
        let mut acc = Accumulator875::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.329_f32 + y.sin();
        let b = y * 3.571_f32 - x.cos();
        let mut acc = Accumulator875::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.328_f32 + y.sin();
        let b = y * 2.243_f32 - x.cos();
        let mut acc = Accumulator875::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.103_f32 + y.sin();
        let b = y * 6.552_f32 - x.cos();
        let mut acc = Accumulator875::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.51_f32 + y.sin();
        let b = y * 4.433_f32 - x.cos();
        let mut acc = Accumulator875::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.091_f32 + y.sin();
        let b = y * 0.443_f32 - x.cos();
        let mut acc = Accumulator875::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.265_f32 + y.sin();
        let b = y * 7.081_f32 - x.cos();
        let mut acc = Accumulator875::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.827_f32 + y.sin();
        let b = y * 9.778_f32 - x.cos();
        let mut acc = Accumulator875::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_875(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_875() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_875(total as u64) % 997) as f32;
        total
    }
}

pub mod m876 {
    use super::*;

    pub struct Accumulator876<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator876<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.227_f32 + y.sin();
        let b = y * 0.654_f32 - x.cos();
        let mut acc = Accumulator876::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.638_f32 + y.sin();
        let b = y * 8.687_f32 - x.cos();
        let mut acc = Accumulator876::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.574_f32 + y.sin();
        let b = y * 3.292_f32 - x.cos();
        let mut acc = Accumulator876::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.601_f32 + y.sin();
        let b = y * 2.856_f32 - x.cos();
        let mut acc = Accumulator876::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.447_f32 + y.sin();
        let b = y * 4.845_f32 - x.cos();
        let mut acc = Accumulator876::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.283_f32 + y.sin();
        let b = y * 3.335_f32 - x.cos();
        let mut acc = Accumulator876::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.715_f32 + y.sin();
        let b = y * 5.093_f32 - x.cos();
        let mut acc = Accumulator876::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.203_f32 + y.sin();
        let b = y * 0.611_f32 - x.cos();
        let mut acc = Accumulator876::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.25_f32 + y.sin();
        let b = y * 7.806_f32 - x.cos();
        let mut acc = Accumulator876::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.475_f32 + y.sin();
        let b = y * 6.697_f32 - x.cos();
        let mut acc = Accumulator876::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 3.588_f32 + y.sin();
        let b = y * 9.855_f32 - x.cos();
        let mut acc = Accumulator876::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.338_f32 + y.sin();
        let b = y * 1.42_f32 - x.cos();
        let mut acc = Accumulator876::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.28_f32 + y.sin();
        let b = y * 8.796_f32 - x.cos();
        let mut acc = Accumulator876::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.79_f32 + y.sin();
        let b = y * 2.992_f32 - x.cos();
        let mut acc = Accumulator876::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.676_f32 + y.sin();
        let b = y * 5.013_f32 - x.cos();
        let mut acc = Accumulator876::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.626_f32 + y.sin();
        let b = y * 0.407_f32 - x.cos();
        let mut acc = Accumulator876::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.896_f32 + y.sin();
        let b = y * 5.61_f32 - x.cos();
        let mut acc = Accumulator876::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.364_f32 + y.sin();
        let b = y * 6.32_f32 - x.cos();
        let mut acc = Accumulator876::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.835_f32 + y.sin();
        let b = y * 3.569_f32 - x.cos();
        let mut acc = Accumulator876::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 9.578_f32 + y.sin();
        let b = y * 2.396_f32 - x.cos();
        let mut acc = Accumulator876::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_876(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m876-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_876() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_876(total as u64) % 997) as f32;
        total
    }
}

pub mod m877 {
    use super::*;

    pub struct Accumulator877<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator877<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 3.88_f32 + y.sin();
        let b = y * 8.408_f32 - x.cos();
        let mut acc = Accumulator877::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 4.24_f32 + y.sin();
        let b = y * 3.221_f32 - x.cos();
        let mut acc = Accumulator877::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.839_f32 + y.sin();
        let b = y * 2.782_f32 - x.cos();
        let mut acc = Accumulator877::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.759_f32 + y.sin();
        let b = y * 6.952_f32 - x.cos();
        let mut acc = Accumulator877::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.274_f32 + y.sin();
        let b = y * 8.938_f32 - x.cos();
        let mut acc = Accumulator877::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.639_f32 + y.sin();
        let b = y * 4.349_f32 - x.cos();
        let mut acc = Accumulator877::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.387_f32 + y.sin();
        let b = y * 6.681_f32 - x.cos();
        let mut acc = Accumulator877::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.057_f32 + y.sin();
        let b = y * 9.686_f32 - x.cos();
        let mut acc = Accumulator877::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.692_f32 + y.sin();
        let b = y * 3.155_f32 - x.cos();
        let mut acc = Accumulator877::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.695_f32 + y.sin();
        let b = y * 3.923_f32 - x.cos();
        let mut acc = Accumulator877::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 9.219_f32 + y.sin();
        let b = y * 3.642_f32 - x.cos();
        let mut acc = Accumulator877::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.834_f32 + y.sin();
        let b = y * 8.613_f32 - x.cos();
        let mut acc = Accumulator877::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 1.49_f32 + y.sin();
        let b = y * 6.405_f32 - x.cos();
        let mut acc = Accumulator877::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.311_f32 + y.sin();
        let b = y * 7.087_f32 - x.cos();
        let mut acc = Accumulator877::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.571_f32 + y.sin();
        let b = y * 8.362_f32 - x.cos();
        let mut acc = Accumulator877::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.425_f32 + y.sin();
        let b = y * 6.398_f32 - x.cos();
        let mut acc = Accumulator877::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.36_f32 + y.sin();
        let b = y * 6.007_f32 - x.cos();
        let mut acc = Accumulator877::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.299_f32 + y.sin();
        let b = y * 2.039_f32 - x.cos();
        let mut acc = Accumulator877::new(a);
        acc.add(b);
        (acc.value * 21_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.498_f32 + y.sin();
        let b = y * 7.649_f32 - x.cos();
        let mut acc = Accumulator877::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.067_f32 + y.sin();
        let b = y * 3.906_f32 - x.cos();
        let mut acc = Accumulator877::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_877(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_877() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_877(total as u64) % 997) as f32;
        total
    }
}

pub mod m878 {
    use super::*;

    pub struct Accumulator878<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator878<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.889_f32 + y.sin();
        let b = y * 0.357_f32 - x.cos();
        let mut acc = Accumulator878::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.335_f32 + y.sin();
        let b = y * 6.347_f32 - x.cos();
        let mut acc = Accumulator878::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.123_f32 + y.sin();
        let b = y * 1.638_f32 - x.cos();
        let mut acc = Accumulator878::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.56_f32 + y.sin();
        let b = y * 0.953_f32 - x.cos();
        let mut acc = Accumulator878::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.026_f32 + y.sin();
        let b = y * 0.678_f32 - x.cos();
        let mut acc = Accumulator878::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 4.316_f32 + y.sin();
        let b = y * 5.887_f32 - x.cos();
        let mut acc = Accumulator878::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.345_f32 + y.sin();
        let b = y * 5.658_f32 - x.cos();
        let mut acc = Accumulator878::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.495_f32 + y.sin();
        let b = y * 2.219_f32 - x.cos();
        let mut acc = Accumulator878::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.739_f32 + y.sin();
        let b = y * 6.914_f32 - x.cos();
        let mut acc = Accumulator878::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.894_f32 + y.sin();
        let b = y * 2.728_f32 - x.cos();
        let mut acc = Accumulator878::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.139_f32 + y.sin();
        let b = y * 1.168_f32 - x.cos();
        let mut acc = Accumulator878::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.608_f32 + y.sin();
        let b = y * 8.273_f32 - x.cos();
        let mut acc = Accumulator878::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.993_f32 + y.sin();
        let b = y * 0.372_f32 - x.cos();
        let mut acc = Accumulator878::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.399_f32 + y.sin();
        let b = y * 9.086_f32 - x.cos();
        let mut acc = Accumulator878::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.695_f32 + y.sin();
        let b = y * 7.123_f32 - x.cos();
        let mut acc = Accumulator878::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.993_f32 + y.sin();
        let b = y * 9.843_f32 - x.cos();
        let mut acc = Accumulator878::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.658_f32 + y.sin();
        let b = y * 1.276_f32 - x.cos();
        let mut acc = Accumulator878::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.24_f32 + y.sin();
        let b = y * 5.347_f32 - x.cos();
        let mut acc = Accumulator878::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.037_f32 + y.sin();
        let b = y * 4.021_f32 - x.cos();
        let mut acc = Accumulator878::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.909_f32 + y.sin();
        let b = y * 8.53_f32 - x.cos();
        let mut acc = Accumulator878::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_878(seed: u64) -> u64 {
        let re = Regex::new(r"m878-(\d+)").unwrap();
        let hay = format!("m878-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_878() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_878(total as u64) % 997) as f32;
        total
    }
}

pub mod m879 {
    use super::*;

    pub struct Accumulator879<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator879<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.696_f32 + y.sin();
        let b = y * 6.126_f32 - x.cos();
        let mut acc = Accumulator879::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.716_f32 + y.sin();
        let b = y * 2.175_f32 - x.cos();
        let mut acc = Accumulator879::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.017_f32 + y.sin();
        let b = y * 6.95_f32 - x.cos();
        let mut acc = Accumulator879::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.24_f32 + y.sin();
        let b = y * 2.677_f32 - x.cos();
        let mut acc = Accumulator879::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.389_f32 + y.sin();
        let b = y * 3.633_f32 - x.cos();
        let mut acc = Accumulator879::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.079_f32 + y.sin();
        let b = y * 8.318_f32 - x.cos();
        let mut acc = Accumulator879::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 6.354_f32 + y.sin();
        let b = y * 6.978_f32 - x.cos();
        let mut acc = Accumulator879::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.262_f32 + y.sin();
        let b = y * 4.259_f32 - x.cos();
        let mut acc = Accumulator879::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.131_f32 + y.sin();
        let b = y * 3.512_f32 - x.cos();
        let mut acc = Accumulator879::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.437_f32 + y.sin();
        let b = y * 1.947_f32 - x.cos();
        let mut acc = Accumulator879::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.441_f32 + y.sin();
        let b = y * 1.95_f32 - x.cos();
        let mut acc = Accumulator879::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.195_f32 + y.sin();
        let b = y * 7.496_f32 - x.cos();
        let mut acc = Accumulator879::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.959_f32 + y.sin();
        let b = y * 7.066_f32 - x.cos();
        let mut acc = Accumulator879::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.108_f32 + y.sin();
        let b = y * 6.899_f32 - x.cos();
        let mut acc = Accumulator879::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.184_f32 + y.sin();
        let b = y * 5.325_f32 - x.cos();
        let mut acc = Accumulator879::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.762_f32 + y.sin();
        let b = y * 7.346_f32 - x.cos();
        let mut acc = Accumulator879::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 9.89_f32 + y.sin();
        let b = y * 6.023_f32 - x.cos();
        let mut acc = Accumulator879::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.856_f32 + y.sin();
        let b = y * 2.578_f32 - x.cos();
        let mut acc = Accumulator879::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.774_f32 + y.sin();
        let b = y * 6.87_f32 - x.cos();
        let mut acc = Accumulator879::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.99_f32 + y.sin();
        let b = y * 1.78_f32 - x.cos();
        let mut acc = Accumulator879::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_879(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_879() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_879(total as u64) % 997) as f32;
        total
    }
}

pub mod m880 {
    use super::*;

    pub struct Accumulator880<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator880<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.385_f32 + y.sin();
        let b = y * 7.184_f32 - x.cos();
        let mut acc = Accumulator880::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.013_f32 + y.sin();
        let b = y * 7.147_f32 - x.cos();
        let mut acc = Accumulator880::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.064_f32 + y.sin();
        let b = y * 7.973_f32 - x.cos();
        let mut acc = Accumulator880::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.816_f32 + y.sin();
        let b = y * 0.831_f32 - x.cos();
        let mut acc = Accumulator880::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.894_f32 + y.sin();
        let b = y * 8.052_f32 - x.cos();
        let mut acc = Accumulator880::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.692_f32 + y.sin();
        let b = y * 8.856_f32 - x.cos();
        let mut acc = Accumulator880::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 0.213_f32 + y.sin();
        let b = y * 4.741_f32 - x.cos();
        let mut acc = Accumulator880::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.676_f32 + y.sin();
        let b = y * 7.609_f32 - x.cos();
        let mut acc = Accumulator880::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 4.589_f32 + y.sin();
        let b = y * 4.795_f32 - x.cos();
        let mut acc = Accumulator880::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 2.66_f32 + y.sin();
        let b = y * 5.583_f32 - x.cos();
        let mut acc = Accumulator880::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.7_f32 + y.sin();
        let b = y * 4.518_f32 - x.cos();
        let mut acc = Accumulator880::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.004_f32 + y.sin();
        let b = y * 6.928_f32 - x.cos();
        let mut acc = Accumulator880::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.861_f32 + y.sin();
        let b = y * 2.284_f32 - x.cos();
        let mut acc = Accumulator880::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.929_f32 + y.sin();
        let b = y * 6.857_f32 - x.cos();
        let mut acc = Accumulator880::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.736_f32 + y.sin();
        let b = y * 1.241_f32 - x.cos();
        let mut acc = Accumulator880::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.747_f32 + y.sin();
        let b = y * 3.857_f32 - x.cos();
        let mut acc = Accumulator880::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.164_f32 + y.sin();
        let b = y * 4.288_f32 - x.cos();
        let mut acc = Accumulator880::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.692_f32 + y.sin();
        let b = y * 2.222_f32 - x.cos();
        let mut acc = Accumulator880::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.465_f32 + y.sin();
        let b = y * 3.723_f32 - x.cos();
        let mut acc = Accumulator880::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.558_f32 + y.sin();
        let b = y * 9.899_f32 - x.cos();
        let mut acc = Accumulator880::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_880(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(880u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_880() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_880(total as u64) % 997) as f32;
        total
    }
}

pub mod m881 {
    use super::*;

    pub struct Accumulator881<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator881<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 1.116_f32 + y.sin();
        let b = y * 9.149_f32 - x.cos();
        let mut acc = Accumulator881::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.387_f32 + y.sin();
        let b = y * 7.561_f32 - x.cos();
        let mut acc = Accumulator881::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.846_f32 + y.sin();
        let b = y * 9.244_f32 - x.cos();
        let mut acc = Accumulator881::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.678_f32 + y.sin();
        let b = y * 7.012_f32 - x.cos();
        let mut acc = Accumulator881::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.127_f32 + y.sin();
        let b = y * 6.544_f32 - x.cos();
        let mut acc = Accumulator881::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.603_f32 + y.sin();
        let b = y * 0.637_f32 - x.cos();
        let mut acc = Accumulator881::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 9.342_f32 + y.sin();
        let b = y * 0.156_f32 - x.cos();
        let mut acc = Accumulator881::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.237_f32 + y.sin();
        let b = y * 9.034_f32 - x.cos();
        let mut acc = Accumulator881::new(a);
        acc.add(b);
        (acc.value * 28_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.16_f32 + y.sin();
        let b = y * 8.81_f32 - x.cos();
        let mut acc = Accumulator881::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.229_f32 + y.sin();
        let b = y * 0.321_f32 - x.cos();
        let mut acc = Accumulator881::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.871_f32 + y.sin();
        let b = y * 8.406_f32 - x.cos();
        let mut acc = Accumulator881::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.74_f32 + y.sin();
        let b = y * 3.271_f32 - x.cos();
        let mut acc = Accumulator881::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.168_f32 + y.sin();
        let b = y * 0.328_f32 - x.cos();
        let mut acc = Accumulator881::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.468_f32 + y.sin();
        let b = y * 4.135_f32 - x.cos();
        let mut acc = Accumulator881::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.878_f32 + y.sin();
        let b = y * 2.028_f32 - x.cos();
        let mut acc = Accumulator881::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 8.879_f32 + y.sin();
        let b = y * 4.639_f32 - x.cos();
        let mut acc = Accumulator881::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.139_f32 + y.sin();
        let b = y * 1.954_f32 - x.cos();
        let mut acc = Accumulator881::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.073_f32 + y.sin();
        let b = y * 6.87_f32 - x.cos();
        let mut acc = Accumulator881::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.704_f32 + y.sin();
        let b = y * 2.973_f32 - x.cos();
        let mut acc = Accumulator881::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.208_f32 + y.sin();
        let b = y * 1.142_f32 - x.cos();
        let mut acc = Accumulator881::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_881(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_881() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_881(total as u64) % 997) as f32;
        total
    }
}

pub mod m882 {
    use super::*;

    pub struct Accumulator882<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator882<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.612_f32 + y.sin();
        let b = y * 5.999_f32 - x.cos();
        let mut acc = Accumulator882::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 6.908_f32 + y.sin();
        let b = y * 3.215_f32 - x.cos();
        let mut acc = Accumulator882::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.96_f32 + y.sin();
        let b = y * 3.133_f32 - x.cos();
        let mut acc = Accumulator882::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.152_f32 + y.sin();
        let b = y * 0.189_f32 - x.cos();
        let mut acc = Accumulator882::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.407_f32 + y.sin();
        let b = y * 2.119_f32 - x.cos();
        let mut acc = Accumulator882::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 9.613_f32 + y.sin();
        let b = y * 0.648_f32 - x.cos();
        let mut acc = Accumulator882::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.26_f32 + y.sin();
        let b = y * 6.931_f32 - x.cos();
        let mut acc = Accumulator882::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.803_f32 + y.sin();
        let b = y * 4.238_f32 - x.cos();
        let mut acc = Accumulator882::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.311_f32 + y.sin();
        let b = y * 4.051_f32 - x.cos();
        let mut acc = Accumulator882::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.951_f32 + y.sin();
        let b = y * 6.008_f32 - x.cos();
        let mut acc = Accumulator882::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 1.248_f32 + y.sin();
        let b = y * 3.271_f32 - x.cos();
        let mut acc = Accumulator882::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.404_f32 + y.sin();
        let b = y * 0.749_f32 - x.cos();
        let mut acc = Accumulator882::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 4.302_f32 + y.sin();
        let b = y * 3.817_f32 - x.cos();
        let mut acc = Accumulator882::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.758_f32 + y.sin();
        let b = y * 2.977_f32 - x.cos();
        let mut acc = Accumulator882::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 2.65_f32 + y.sin();
        let b = y * 6.526_f32 - x.cos();
        let mut acc = Accumulator882::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.772_f32 + y.sin();
        let b = y * 1.474_f32 - x.cos();
        let mut acc = Accumulator882::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.387_f32 + y.sin();
        let b = y * 1.174_f32 - x.cos();
        let mut acc = Accumulator882::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 3.961_f32 + y.sin();
        let b = y * 6.668_f32 - x.cos();
        let mut acc = Accumulator882::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.083_f32 + y.sin();
        let b = y * 7.417_f32 - x.cos();
        let mut acc = Accumulator882::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.094_f32 + y.sin();
        let b = y * 8.608_f32 - x.cos();
        let mut acc = Accumulator882::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_882(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_882() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_882(total as u64) % 997) as f32;
        total
    }
}

pub mod m883 {
    use super::*;

    pub struct Accumulator883<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator883<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.938_f32 + y.sin();
        let b = y * 6.624_f32 - x.cos();
        let mut acc = Accumulator883::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.805_f32 + y.sin();
        let b = y * 0.129_f32 - x.cos();
        let mut acc = Accumulator883::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.255_f32 + y.sin();
        let b = y * 3.889_f32 - x.cos();
        let mut acc = Accumulator883::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.407_f32 + y.sin();
        let b = y * 2.352_f32 - x.cos();
        let mut acc = Accumulator883::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.894_f32 + y.sin();
        let b = y * 7.441_f32 - x.cos();
        let mut acc = Accumulator883::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.474_f32 + y.sin();
        let b = y * 9.241_f32 - x.cos();
        let mut acc = Accumulator883::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.104_f32 + y.sin();
        let b = y * 3.096_f32 - x.cos();
        let mut acc = Accumulator883::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 2.353_f32 + y.sin();
        let b = y * 1.515_f32 - x.cos();
        let mut acc = Accumulator883::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.913_f32 + y.sin();
        let b = y * 0.355_f32 - x.cos();
        let mut acc = Accumulator883::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.29_f32 + y.sin();
        let b = y * 8.601_f32 - x.cos();
        let mut acc = Accumulator883::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.819_f32 + y.sin();
        let b = y * 1.366_f32 - x.cos();
        let mut acc = Accumulator883::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 9.552_f32 + y.sin();
        let b = y * 8.018_f32 - x.cos();
        let mut acc = Accumulator883::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 7.384_f32 + y.sin();
        let b = y * 4.771_f32 - x.cos();
        let mut acc = Accumulator883::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.878_f32 + y.sin();
        let b = y * 1.887_f32 - x.cos();
        let mut acc = Accumulator883::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 5.939_f32 + y.sin();
        let b = y * 8.096_f32 - x.cos();
        let mut acc = Accumulator883::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.886_f32 + y.sin();
        let b = y * 0.247_f32 - x.cos();
        let mut acc = Accumulator883::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 1.767_f32 + y.sin();
        let b = y * 3.603_f32 - x.cos();
        let mut acc = Accumulator883::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.02_f32 + y.sin();
        let b = y * 1.623_f32 - x.cos();
        let mut acc = Accumulator883::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.543_f32 + y.sin();
        let b = y * 0.493_f32 - x.cos();
        let mut acc = Accumulator883::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.695_f32 + y.sin();
        let b = y * 2.221_f32 - x.cos();
        let mut acc = Accumulator883::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_883(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m883-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_883() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_883(total as u64) % 997) as f32;
        total
    }
}

pub mod m884 {
    use super::*;

    pub struct Accumulator884<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator884<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 9.339_f32 + y.sin();
        let b = y * 3.71_f32 - x.cos();
        let mut acc = Accumulator884::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 7.54_f32 + y.sin();
        let b = y * 7.153_f32 - x.cos();
        let mut acc = Accumulator884::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.675_f32 + y.sin();
        let b = y * 3.167_f32 - x.cos();
        let mut acc = Accumulator884::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.698_f32 + y.sin();
        let b = y * 6.216_f32 - x.cos();
        let mut acc = Accumulator884::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.511_f32 + y.sin();
        let b = y * 9.211_f32 - x.cos();
        let mut acc = Accumulator884::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 2.579_f32 + y.sin();
        let b = y * 5.394_f32 - x.cos();
        let mut acc = Accumulator884::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.631_f32 + y.sin();
        let b = y * 4.441_f32 - x.cos();
        let mut acc = Accumulator884::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 0.723_f32 + y.sin();
        let b = y * 7.502_f32 - x.cos();
        let mut acc = Accumulator884::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.64_f32 + y.sin();
        let b = y * 7.556_f32 - x.cos();
        let mut acc = Accumulator884::new(a);
        acc.add(b);
        (acc.value * 17_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.826_f32 + y.sin();
        let b = y * 6.828_f32 - x.cos();
        let mut acc = Accumulator884::new(a);
        acc.add(b);
        (acc.value * 49_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.182_f32 + y.sin();
        let b = y * 7.061_f32 - x.cos();
        let mut acc = Accumulator884::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.145_f32 + y.sin();
        let b = y * 6.453_f32 - x.cos();
        let mut acc = Accumulator884::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.066_f32 + y.sin();
        let b = y * 5.057_f32 - x.cos();
        let mut acc = Accumulator884::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.165_f32 + y.sin();
        let b = y * 8.39_f32 - x.cos();
        let mut acc = Accumulator884::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.824_f32 + y.sin();
        let b = y * 2.602_f32 - x.cos();
        let mut acc = Accumulator884::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 4.18_f32 + y.sin();
        let b = y * 1.713_f32 - x.cos();
        let mut acc = Accumulator884::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.393_f32 + y.sin();
        let b = y * 6.377_f32 - x.cos();
        let mut acc = Accumulator884::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 1.355_f32 + y.sin();
        let b = y * 8.965_f32 - x.cos();
        let mut acc = Accumulator884::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.195_f32 + y.sin();
        let b = y * 7.749_f32 - x.cos();
        let mut acc = Accumulator884::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.995_f32 + y.sin();
        let b = y * 9.471_f32 - x.cos();
        let mut acc = Accumulator884::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_884(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_884() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_884(total as u64) % 997) as f32;
        total
    }
}

pub mod m885 {
    use super::*;

    pub struct Accumulator885<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator885<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.785_f32 + y.sin();
        let b = y * 2.157_f32 - x.cos();
        let mut acc = Accumulator885::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.086_f32 + y.sin();
        let b = y * 6.557_f32 - x.cos();
        let mut acc = Accumulator885::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.344_f32 + y.sin();
        let b = y * 6.664_f32 - x.cos();
        let mut acc = Accumulator885::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.605_f32 + y.sin();
        let b = y * 2.195_f32 - x.cos();
        let mut acc = Accumulator885::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.307_f32 + y.sin();
        let b = y * 7.49_f32 - x.cos();
        let mut acc = Accumulator885::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.498_f32 + y.sin();
        let b = y * 1.831_f32 - x.cos();
        let mut acc = Accumulator885::new(a);
        acc.add(b);
        (acc.value * 26_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.921_f32 + y.sin();
        let b = y * 2.022_f32 - x.cos();
        let mut acc = Accumulator885::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.755_f32 + y.sin();
        let b = y * 6.15_f32 - x.cos();
        let mut acc = Accumulator885::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.962_f32 + y.sin();
        let b = y * 9.707_f32 - x.cos();
        let mut acc = Accumulator885::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.042_f32 + y.sin();
        let b = y * 8.752_f32 - x.cos();
        let mut acc = Accumulator885::new(a);
        acc.add(b);
        (acc.value * 79_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.735_f32 + y.sin();
        let b = y * 1.662_f32 - x.cos();
        let mut acc = Accumulator885::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.115_f32 + y.sin();
        let b = y * 6.856_f32 - x.cos();
        let mut acc = Accumulator885::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.237_f32 + y.sin();
        let b = y * 4.304_f32 - x.cos();
        let mut acc = Accumulator885::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.541_f32 + y.sin();
        let b = y * 0.309_f32 - x.cos();
        let mut acc = Accumulator885::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.124_f32 + y.sin();
        let b = y * 7.242_f32 - x.cos();
        let mut acc = Accumulator885::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.261_f32 + y.sin();
        let b = y * 0.363_f32 - x.cos();
        let mut acc = Accumulator885::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.316_f32 + y.sin();
        let b = y * 1.81_f32 - x.cos();
        let mut acc = Accumulator885::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.117_f32 + y.sin();
        let b = y * 6.412_f32 - x.cos();
        let mut acc = Accumulator885::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.372_f32 + y.sin();
        let b = y * 7.745_f32 - x.cos();
        let mut acc = Accumulator885::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.304_f32 + y.sin();
        let b = y * 4.01_f32 - x.cos();
        let mut acc = Accumulator885::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_885(seed: u64) -> u64 {
        let re = Regex::new(r"m885-(\d+)").unwrap();
        let hay = format!("m885-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_885() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_885(total as u64) % 997) as f32;
        total
    }
}

pub mod m886 {
    use super::*;

    pub struct Accumulator886<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator886<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 4.215_f32 + y.sin();
        let b = y * 4.05_f32 - x.cos();
        let mut acc = Accumulator886::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.985_f32 + y.sin();
        let b = y * 7.673_f32 - x.cos();
        let mut acc = Accumulator886::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.152_f32 + y.sin();
        let b = y * 6.108_f32 - x.cos();
        let mut acc = Accumulator886::new(a);
        acc.add(b);
        (acc.value * 81_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.266_f32 + y.sin();
        let b = y * 5.774_f32 - x.cos();
        let mut acc = Accumulator886::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 0.328_f32 + y.sin();
        let b = y * 4.895_f32 - x.cos();
        let mut acc = Accumulator886::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 7.651_f32 + y.sin();
        let b = y * 1.085_f32 - x.cos();
        let mut acc = Accumulator886::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 7.979_f32 + y.sin();
        let b = y * 2.823_f32 - x.cos();
        let mut acc = Accumulator886::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.028_f32 + y.sin();
        let b = y * 7.101_f32 - x.cos();
        let mut acc = Accumulator886::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.923_f32 + y.sin();
        let b = y * 2.317_f32 - x.cos();
        let mut acc = Accumulator886::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 7.31_f32 + y.sin();
        let b = y * 2.541_f32 - x.cos();
        let mut acc = Accumulator886::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.418_f32 + y.sin();
        let b = y * 2.235_f32 - x.cos();
        let mut acc = Accumulator886::new(a);
        acc.add(b);
        (acc.value * 37_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 7.192_f32 + y.sin();
        let b = y * 5.688_f32 - x.cos();
        let mut acc = Accumulator886::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.521_f32 + y.sin();
        let b = y * 2.105_f32 - x.cos();
        let mut acc = Accumulator886::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.963_f32 + y.sin();
        let b = y * 4.908_f32 - x.cos();
        let mut acc = Accumulator886::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 9.612_f32 + y.sin();
        let b = y * 7.886_f32 - x.cos();
        let mut acc = Accumulator886::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.46_f32 + y.sin();
        let b = y * 7.27_f32 - x.cos();
        let mut acc = Accumulator886::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.714_f32 + y.sin();
        let b = y * 6.833_f32 - x.cos();
        let mut acc = Accumulator886::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.372_f32 + y.sin();
        let b = y * 4.918_f32 - x.cos();
        let mut acc = Accumulator886::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.073_f32 + y.sin();
        let b = y * 6.479_f32 - x.cos();
        let mut acc = Accumulator886::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.905_f32 + y.sin();
        let b = y * 1.388_f32 - x.cos();
        let mut acc = Accumulator886::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_886(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_886() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_886(total as u64) % 997) as f32;
        total
    }
}

pub mod m887 {
    use super::*;

    pub struct Accumulator887<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator887<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.46_f32 + y.sin();
        let b = y * 9.786_f32 - x.cos();
        let mut acc = Accumulator887::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.788_f32 + y.sin();
        let b = y * 4.573_f32 - x.cos();
        let mut acc = Accumulator887::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.042_f32 + y.sin();
        let b = y * 4.36_f32 - x.cos();
        let mut acc = Accumulator887::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.77_f32 + y.sin();
        let b = y * 2.606_f32 - x.cos();
        let mut acc = Accumulator887::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.442_f32 + y.sin();
        let b = y * 6.926_f32 - x.cos();
        let mut acc = Accumulator887::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.156_f32 + y.sin();
        let b = y * 9.751_f32 - x.cos();
        let mut acc = Accumulator887::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.439_f32 + y.sin();
        let b = y * 5.582_f32 - x.cos();
        let mut acc = Accumulator887::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.001_f32 + y.sin();
        let b = y * 7.597_f32 - x.cos();
        let mut acc = Accumulator887::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.947_f32 + y.sin();
        let b = y * 0.84_f32 - x.cos();
        let mut acc = Accumulator887::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 5.251_f32 + y.sin();
        let b = y * 8.956_f32 - x.cos();
        let mut acc = Accumulator887::new(a);
        acc.add(b);
        (acc.value * 23_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.25_f32 + y.sin();
        let b = y * 4.715_f32 - x.cos();
        let mut acc = Accumulator887::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.347_f32 + y.sin();
        let b = y * 7.885_f32 - x.cos();
        let mut acc = Accumulator887::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.719_f32 + y.sin();
        let b = y * 6.465_f32 - x.cos();
        let mut acc = Accumulator887::new(a);
        acc.add(b);
        (acc.value * 84_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.276_f32 + y.sin();
        let b = y * 4.001_f32 - x.cos();
        let mut acc = Accumulator887::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.582_f32 + y.sin();
        let b = y * 5.773_f32 - x.cos();
        let mut acc = Accumulator887::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.767_f32 + y.sin();
        let b = y * 5.986_f32 - x.cos();
        let mut acc = Accumulator887::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.554_f32 + y.sin();
        let b = y * 1.445_f32 - x.cos();
        let mut acc = Accumulator887::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.509_f32 + y.sin();
        let b = y * 5.066_f32 - x.cos();
        let mut acc = Accumulator887::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.687_f32 + y.sin();
        let b = y * 4.963_f32 - x.cos();
        let mut acc = Accumulator887::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.092_f32 + y.sin();
        let b = y * 1.571_f32 - x.cos();
        let mut acc = Accumulator887::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_887(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(887u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_887() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_887(total as u64) % 997) as f32;
        total
    }
}

pub mod m888 {
    use super::*;

    pub struct Accumulator888<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator888<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 8.885_f32 + y.sin();
        let b = y * 3.851_f32 - x.cos();
        let mut acc = Accumulator888::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 5.241_f32 + y.sin();
        let b = y * 5.886_f32 - x.cos();
        let mut acc = Accumulator888::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.408_f32 + y.sin();
        let b = y * 7.47_f32 - x.cos();
        let mut acc = Accumulator888::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 6.818_f32 + y.sin();
        let b = y * 2.492_f32 - x.cos();
        let mut acc = Accumulator888::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.751_f32 + y.sin();
        let b = y * 1.817_f32 - x.cos();
        let mut acc = Accumulator888::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.214_f32 + y.sin();
        let b = y * 7.589_f32 - x.cos();
        let mut acc = Accumulator888::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.064_f32 + y.sin();
        let b = y * 5.042_f32 - x.cos();
        let mut acc = Accumulator888::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.818_f32 + y.sin();
        let b = y * 1.222_f32 - x.cos();
        let mut acc = Accumulator888::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.685_f32 + y.sin();
        let b = y * 0.445_f32 - x.cos();
        let mut acc = Accumulator888::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.361_f32 + y.sin();
        let b = y * 4.255_f32 - x.cos();
        let mut acc = Accumulator888::new(a);
        acc.add(b);
        (acc.value * 95_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 6.024_f32 + y.sin();
        let b = y * 3.859_f32 - x.cos();
        let mut acc = Accumulator888::new(a);
        acc.add(b);
        (acc.value * 83_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.618_f32 + y.sin();
        let b = y * 7.512_f32 - x.cos();
        let mut acc = Accumulator888::new(a);
        acc.add(b);
        (acc.value * 65_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.958_f32 + y.sin();
        let b = y * 3.327_f32 - x.cos();
        let mut acc = Accumulator888::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 1.348_f32 + y.sin();
        let b = y * 2.195_f32 - x.cos();
        let mut acc = Accumulator888::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.994_f32 + y.sin();
        let b = y * 8.386_f32 - x.cos();
        let mut acc = Accumulator888::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.974_f32 + y.sin();
        let b = y * 4.873_f32 - x.cos();
        let mut acc = Accumulator888::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 4.331_f32 + y.sin();
        let b = y * 7.312_f32 - x.cos();
        let mut acc = Accumulator888::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 8.226_f32 + y.sin();
        let b = y * 2.425_f32 - x.cos();
        let mut acc = Accumulator888::new(a);
        acc.add(b);
        (acc.value * 86_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 9.368_f32 + y.sin();
        let b = y * 5.253_f32 - x.cos();
        let mut acc = Accumulator888::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 4.8_f32 + y.sin();
        let b = y * 1.376_f32 - x.cos();
        let mut acc = Accumulator888::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_888(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_888() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_888(total as u64) % 997) as f32;
        total
    }
}

pub mod m889 {
    use super::*;

    pub struct Accumulator889<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator889<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.203_f32 + y.sin();
        let b = y * 6.713_f32 - x.cos();
        let mut acc = Accumulator889::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.876_f32 + y.sin();
        let b = y * 5.839_f32 - x.cos();
        let mut acc = Accumulator889::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 2.411_f32 + y.sin();
        let b = y * 7.837_f32 - x.cos();
        let mut acc = Accumulator889::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 2.555_f32 + y.sin();
        let b = y * 1.529_f32 - x.cos();
        let mut acc = Accumulator889::new(a);
        acc.add(b);
        (acc.value * 43_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.509_f32 + y.sin();
        let b = y * 4.399_f32 - x.cos();
        let mut acc = Accumulator889::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.533_f32 + y.sin();
        let b = y * 2.468_f32 - x.cos();
        let mut acc = Accumulator889::new(a);
        acc.add(b);
        (acc.value * 96_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.197_f32 + y.sin();
        let b = y * 2.23_f32 - x.cos();
        let mut acc = Accumulator889::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 8.956_f32 + y.sin();
        let b = y * 1.256_f32 - x.cos();
        let mut acc = Accumulator889::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 0.943_f32 + y.sin();
        let b = y * 5.101_f32 - x.cos();
        let mut acc = Accumulator889::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.679_f32 + y.sin();
        let b = y * 0.869_f32 - x.cos();
        let mut acc = Accumulator889::new(a);
        acc.add(b);
        (acc.value * 76_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 7.895_f32 + y.sin();
        let b = y * 5.206_f32 - x.cos();
        let mut acc = Accumulator889::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.923_f32 + y.sin();
        let b = y * 1.215_f32 - x.cos();
        let mut acc = Accumulator889::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.709_f32 + y.sin();
        let b = y * 4.894_f32 - x.cos();
        let mut acc = Accumulator889::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 4.729_f32 + y.sin();
        let b = y * 1.027_f32 - x.cos();
        let mut acc = Accumulator889::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.926_f32 + y.sin();
        let b = y * 2.836_f32 - x.cos();
        let mut acc = Accumulator889::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 5.702_f32 + y.sin();
        let b = y * 8.557_f32 - x.cos();
        let mut acc = Accumulator889::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.782_f32 + y.sin();
        let b = y * 3.829_f32 - x.cos();
        let mut acc = Accumulator889::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.227_f32 + y.sin();
        let b = y * 7.534_f32 - x.cos();
        let mut acc = Accumulator889::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.405_f32 + y.sin();
        let b = y * 6.477_f32 - x.cos();
        let mut acc = Accumulator889::new(a);
        acc.add(b);
        (acc.value * 92_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.09_f32 + y.sin();
        let b = y * 5.905_f32 - x.cos();
        let mut acc = Accumulator889::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_889(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_889() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_889(total as u64) % 997) as f32;
        total
    }
}

pub mod m890 {
    use super::*;

    pub struct Accumulator890<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator890<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.265_f32 + y.sin();
        let b = y * 7.761_f32 - x.cos();
        let mut acc = Accumulator890::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 3.194_f32 + y.sin();
        let b = y * 9.195_f32 - x.cos();
        let mut acc = Accumulator890::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.181_f32 + y.sin();
        let b = y * 4.4_f32 - x.cos();
        let mut acc = Accumulator890::new(a);
        acc.add(b);
        (acc.value * 8_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.919_f32 + y.sin();
        let b = y * 9.333_f32 - x.cos();
        let mut acc = Accumulator890::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.38_f32 + y.sin();
        let b = y * 1.54_f32 - x.cos();
        let mut acc = Accumulator890::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 3.078_f32 + y.sin();
        let b = y * 4.991_f32 - x.cos();
        let mut acc = Accumulator890::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.585_f32 + y.sin();
        let b = y * 3.801_f32 - x.cos();
        let mut acc = Accumulator890::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 5.545_f32 + y.sin();
        let b = y * 3.769_f32 - x.cos();
        let mut acc = Accumulator890::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 8.43_f32 + y.sin();
        let b = y * 7.802_f32 - x.cos();
        let mut acc = Accumulator890::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 4.102_f32 + y.sin();
        let b = y * 6.169_f32 - x.cos();
        let mut acc = Accumulator890::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.465_f32 + y.sin();
        let b = y * 9.723_f32 - x.cos();
        let mut acc = Accumulator890::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.339_f32 + y.sin();
        let b = y * 2.898_f32 - x.cos();
        let mut acc = Accumulator890::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.372_f32 + y.sin();
        let b = y * 0.173_f32 - x.cos();
        let mut acc = Accumulator890::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.943_f32 + y.sin();
        let b = y * 2.77_f32 - x.cos();
        let mut acc = Accumulator890::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 0.966_f32 + y.sin();
        let b = y * 5.548_f32 - x.cos();
        let mut acc = Accumulator890::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.293_f32 + y.sin();
        let b = y * 7.636_f32 - x.cos();
        let mut acc = Accumulator890::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 2.041_f32 + y.sin();
        let b = y * 7.787_f32 - x.cos();
        let mut acc = Accumulator890::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 0.426_f32 + y.sin();
        let b = y * 1.319_f32 - x.cos();
        let mut acc = Accumulator890::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.059_f32 + y.sin();
        let b = y * 6.371_f32 - x.cos();
        let mut acc = Accumulator890::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 1.215_f32 + y.sin();
        let b = y * 8.084_f32 - x.cos();
        let mut acc = Accumulator890::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_890(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m890-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_890() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_890(total as u64) % 997) as f32;
        total
    }
}

pub mod m891 {
    use super::*;

    pub struct Accumulator891<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator891<T> {
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
        let b = y * 5.921_f32 - x.cos();
        let mut acc = Accumulator891::new(a);
        acc.add(b);
        (acc.value * 89_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.286_f32 + y.sin();
        let b = y * 0.429_f32 - x.cos();
        let mut acc = Accumulator891::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 9.894_f32 + y.sin();
        let b = y * 4.247_f32 - x.cos();
        let mut acc = Accumulator891::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 1.177_f32 + y.sin();
        let b = y * 5.607_f32 - x.cos();
        let mut acc = Accumulator891::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 4.646_f32 + y.sin();
        let b = y * 6.288_f32 - x.cos();
        let mut acc = Accumulator891::new(a);
        acc.add(b);
        (acc.value * 87_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 0.162_f32 + y.sin();
        let b = y * 4.45_f32 - x.cos();
        let mut acc = Accumulator891::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.734_f32 + y.sin();
        let b = y * 2.16_f32 - x.cos();
        let mut acc = Accumulator891::new(a);
        acc.add(b);
        (acc.value * 34_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 9.116_f32 + y.sin();
        let b = y * 5.362_f32 - x.cos();
        let mut acc = Accumulator891::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 5.882_f32 + y.sin();
        let b = y * 3.839_f32 - x.cos();
        let mut acc = Accumulator891::new(a);
        acc.add(b);
        (acc.value * 20_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.439_f32 + y.sin();
        let b = y * 9.498_f32 - x.cos();
        let mut acc = Accumulator891::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.662_f32 + y.sin();
        let b = y * 3.89_f32 - x.cos();
        let mut acc = Accumulator891::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 3.3_f32 + y.sin();
        let b = y * 1.676_f32 - x.cos();
        let mut acc = Accumulator891::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.139_f32 + y.sin();
        let b = y * 3.228_f32 - x.cos();
        let mut acc = Accumulator891::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 2.079_f32 + y.sin();
        let b = y * 1.636_f32 - x.cos();
        let mut acc = Accumulator891::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.458_f32 + y.sin();
        let b = y * 2.66_f32 - x.cos();
        let mut acc = Accumulator891::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.033_f32 + y.sin();
        let b = y * 1.096_f32 - x.cos();
        let mut acc = Accumulator891::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.722_f32 + y.sin();
        let b = y * 7.007_f32 - x.cos();
        let mut acc = Accumulator891::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.61_f32 + y.sin();
        let b = y * 5.239_f32 - x.cos();
        let mut acc = Accumulator891::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.366_f32 + y.sin();
        let b = y * 7.897_f32 - x.cos();
        let mut acc = Accumulator891::new(a);
        acc.add(b);
        (acc.value * 53_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 6.391_f32 + y.sin();
        let b = y * 1.452_f32 - x.cos();
        let mut acc = Accumulator891::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_891(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_891() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_891(total as u64) % 997) as f32;
        total
    }
}

pub mod m892 {
    use super::*;

    pub struct Accumulator892<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator892<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.485_f32 + y.sin();
        let b = y * 1.443_f32 - x.cos();
        let mut acc = Accumulator892::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.479_f32 + y.sin();
        let b = y * 0.225_f32 - x.cos();
        let mut acc = Accumulator892::new(a);
        acc.add(b);
        (acc.value * 50_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.54_f32 + y.sin();
        let b = y * 5.711_f32 - x.cos();
        let mut acc = Accumulator892::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.103_f32 + y.sin();
        let b = y * 2.971_f32 - x.cos();
        let mut acc = Accumulator892::new(a);
        acc.add(b);
        (acc.value * 45_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 5.167_f32 + y.sin();
        let b = y * 8.576_f32 - x.cos();
        let mut acc = Accumulator892::new(a);
        acc.add(b);
        (acc.value * 4_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.06_f32 + y.sin();
        let b = y * 5.61_f32 - x.cos();
        let mut acc = Accumulator892::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.489_f32 + y.sin();
        let b = y * 9.225_f32 - x.cos();
        let mut acc = Accumulator892::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.066_f32 + y.sin();
        let b = y * 4.673_f32 - x.cos();
        let mut acc = Accumulator892::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.133_f32 + y.sin();
        let b = y * 5.192_f32 - x.cos();
        let mut acc = Accumulator892::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.731_f32 + y.sin();
        let b = y * 1.279_f32 - x.cos();
        let mut acc = Accumulator892::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 4.875_f32 + y.sin();
        let b = y * 7.642_f32 - x.cos();
        let mut acc = Accumulator892::new(a);
        acc.add(b);
        (acc.value * 41_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.284_f32 + y.sin();
        let b = y * 6.987_f32 - x.cos();
        let mut acc = Accumulator892::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.256_f32 + y.sin();
        let b = y * 1.091_f32 - x.cos();
        let mut acc = Accumulator892::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.57_f32 + y.sin();
        let b = y * 0.169_f32 - x.cos();
        let mut acc = Accumulator892::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.341_f32 + y.sin();
        let b = y * 8.949_f32 - x.cos();
        let mut acc = Accumulator892::new(a);
        acc.add(b);
        (acc.value * 19_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 3.227_f32 + y.sin();
        let b = y * 5.766_f32 - x.cos();
        let mut acc = Accumulator892::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 3.533_f32 + y.sin();
        let b = y * 5.577_f32 - x.cos();
        let mut acc = Accumulator892::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.167_f32 + y.sin();
        let b = y * 0.879_f32 - x.cos();
        let mut acc = Accumulator892::new(a);
        acc.add(b);
        (acc.value * 31_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.122_f32 + y.sin();
        let b = y * 9.72_f32 - x.cos();
        let mut acc = Accumulator892::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 0.471_f32 + y.sin();
        let b = y * 9.498_f32 - x.cos();
        let mut acc = Accumulator892::new(a);
        acc.add(b);
        (acc.value * 51_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_892(seed: u64) -> u64 {
        let re = Regex::new(r"m892-(\d+)").unwrap();
        let hay = format!("m892-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_892() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_892(total as u64) % 997) as f32;
        total
    }
}

pub mod m893 {
    use super::*;

    pub struct Accumulator893<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator893<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 7.477_f32 + y.sin();
        let b = y * 6.681_f32 - x.cos();
        let mut acc = Accumulator893::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.347_f32 + y.sin();
        let b = y * 3.094_f32 - x.cos();
        let mut acc = Accumulator893::new(a);
        acc.add(b);
        (acc.value * 38_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 6.716_f32 + y.sin();
        let b = y * 6.923_f32 - x.cos();
        let mut acc = Accumulator893::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.791_f32 + y.sin();
        let b = y * 8.112_f32 - x.cos();
        let mut acc = Accumulator893::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 1.515_f32 + y.sin();
        let b = y * 0.51_f32 - x.cos();
        let mut acc = Accumulator893::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.982_f32 + y.sin();
        let b = y * 8.201_f32 - x.cos();
        let mut acc = Accumulator893::new(a);
        acc.add(b);
        (acc.value * 90_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 2.975_f32 + y.sin();
        let b = y * 2.403_f32 - x.cos();
        let mut acc = Accumulator893::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.109_f32 + y.sin();
        let b = y * 6.255_f32 - x.cos();
        let mut acc = Accumulator893::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.78_f32 + y.sin();
        let b = y * 1.734_f32 - x.cos();
        let mut acc = Accumulator893::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 3.581_f32 + y.sin();
        let b = y * 1.297_f32 - x.cos();
        let mut acc = Accumulator893::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 2.804_f32 + y.sin();
        let b = y * 0.932_f32 - x.cos();
        let mut acc = Accumulator893::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.571_f32 + y.sin();
        let b = y * 7.594_f32 - x.cos();
        let mut acc = Accumulator893::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.389_f32 + y.sin();
        let b = y * 0.996_f32 - x.cos();
        let mut acc = Accumulator893::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 3.291_f32 + y.sin();
        let b = y * 9.721_f32 - x.cos();
        let mut acc = Accumulator893::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 8.391_f32 + y.sin();
        let b = y * 7.615_f32 - x.cos();
        let mut acc = Accumulator893::new(a);
        acc.add(b);
        (acc.value * 32_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.048_f32 + y.sin();
        let b = y * 3.354_f32 - x.cos();
        let mut acc = Accumulator893::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 6.524_f32 + y.sin();
        let b = y * 6.201_f32 - x.cos();
        let mut acc = Accumulator893::new(a);
        acc.add(b);
        (acc.value * 27_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 5.554_f32 + y.sin();
        let b = y * 0.664_f32 - x.cos();
        let mut acc = Accumulator893::new(a);
        acc.add(b);
        (acc.value * 1_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.385_f32 + y.sin();
        let b = y * 6.7_f32 - x.cos();
        let mut acc = Accumulator893::new(a);
        acc.add(b);
        (acc.value * 54_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 7.024_f32 + y.sin();
        let b = y * 7.845_f32 - x.cos();
        let mut acc = Accumulator893::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_893(seed: u64) -> u64 {
        let v: Vec<u64> = (0..32).collect();
        v.iter().tuple_windows().map(|(a, b): (&u64, &u64)| a + b).sum::<u64>() + seed
    }

    pub fn run_all_893() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_893(total as u64) % 997) as f32;
        total
    }
}

pub mod m894 {
    use super::*;

    pub struct Accumulator894<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator894<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 6.738_f32 + y.sin();
        let b = y * 8.125_f32 - x.cos();
        let mut acc = Accumulator894::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 0.764_f32 + y.sin();
        let b = y * 6.395_f32 - x.cos();
        let mut acc = Accumulator894::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 4.113_f32 + y.sin();
        let b = y * 5.538_f32 - x.cos();
        let mut acc = Accumulator894::new(a);
        acc.add(b);
        (acc.value * 18_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 8.689_f32 + y.sin();
        let b = y * 8.372_f32 - x.cos();
        let mut acc = Accumulator894::new(a);
        acc.add(b);
        (acc.value * 91_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 6.583_f32 + y.sin();
        let b = y * 5.625_f32 - x.cos();
        let mut acc = Accumulator894::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.902_f32 + y.sin();
        let b = y * 2.145_f32 - x.cos();
        let mut acc = Accumulator894::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.352_f32 + y.sin();
        let b = y * 5.116_f32 - x.cos();
        let mut acc = Accumulator894::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 1.517_f32 + y.sin();
        let b = y * 7.041_f32 - x.cos();
        let mut acc = Accumulator894::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 6.664_f32 + y.sin();
        let b = y * 5.326_f32 - x.cos();
        let mut acc = Accumulator894::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 8.692_f32 + y.sin();
        let b = y * 9.725_f32 - x.cos();
        let mut acc = Accumulator894::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.376_f32 + y.sin();
        let b = y * 2.048_f32 - x.cos();
        let mut acc = Accumulator894::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 1.155_f32 + y.sin();
        let b = y * 1.296_f32 - x.cos();
        let mut acc = Accumulator894::new(a);
        acc.add(b);
        (acc.value * 67_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 5.692_f32 + y.sin();
        let b = y * 4.853_f32 - x.cos();
        let mut acc = Accumulator894::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 8.535_f32 + y.sin();
        let b = y * 2.303_f32 - x.cos();
        let mut acc = Accumulator894::new(a);
        acc.add(b);
        (acc.value * 78_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 4.154_f32 + y.sin();
        let b = y * 0.206_f32 - x.cos();
        let mut acc = Accumulator894::new(a);
        acc.add(b);
        (acc.value * 42_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 0.688_f32 + y.sin();
        let b = y * 5.019_f32 - x.cos();
        let mut acc = Accumulator894::new(a);
        acc.add(b);
        (acc.value * 46_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.995_f32 + y.sin();
        let b = y * 7.136_f32 - x.cos();
        let mut acc = Accumulator894::new(a);
        acc.add(b);
        (acc.value * 12_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.644_f32 + y.sin();
        let b = y * 8.26_f32 - x.cos();
        let mut acc = Accumulator894::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 0.584_f32 + y.sin();
        let b = y * 2.668_f32 - x.cos();
        let mut acc = Accumulator894::new(a);
        acc.add(b);
        (acc.value * 16_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.991_f32 + y.sin();
        let b = y * 4.218_f32 - x.cos();
        let mut acc = Accumulator894::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_894(seed: u64) -> u64 {
        let ns = Uuid::from_u64_pair(894u64, seed);
        ns.as_u64_pair().0
    }

    pub fn run_all_894() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_894(total as u64) % 997) as f32;
        total
    }
}

pub mod m895 {
    use super::*;

    pub struct Accumulator895<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator895<T> {
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
        let b = y * 8.886_f32 - x.cos();
        let mut acc = Accumulator895::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 2.85_f32 + y.sin();
        let b = y * 3.907_f32 - x.cos();
        let mut acc = Accumulator895::new(a);
        acc.add(b);
        (acc.value * 74_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 8.42_f32 + y.sin();
        let b = y * 1.716_f32 - x.cos();
        let mut acc = Accumulator895::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 5.982_f32 + y.sin();
        let b = y * 9.516_f32 - x.cos();
        let mut acc = Accumulator895::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 7.883_f32 + y.sin();
        let b = y * 2.937_f32 - x.cos();
        let mut acc = Accumulator895::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.259_f32 + y.sin();
        let b = y * 3.077_f32 - x.cos();
        let mut acc = Accumulator895::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 3.465_f32 + y.sin();
        let b = y * 4.614_f32 - x.cos();
        let mut acc = Accumulator895::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 7.033_f32 + y.sin();
        let b = y * 5.868_f32 - x.cos();
        let mut acc = Accumulator895::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 9.565_f32 + y.sin();
        let b = y * 4.96_f32 - x.cos();
        let mut acc = Accumulator895::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 0.69_f32 + y.sin();
        let b = y * 4.184_f32 - x.cos();
        let mut acc = Accumulator895::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.707_f32 + y.sin();
        let b = y * 8.991_f32 - x.cos();
        let mut acc = Accumulator895::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 5.822_f32 + y.sin();
        let b = y * 6.079_f32 - x.cos();
        let mut acc = Accumulator895::new(a);
        acc.add(b);
        (acc.value * 22_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 2.285_f32 + y.sin();
        let b = y * 0.361_f32 - x.cos();
        let mut acc = Accumulator895::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.57_f32 + y.sin();
        let b = y * 6.582_f32 - x.cos();
        let mut acc = Accumulator895::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.475_f32 + y.sin();
        let b = y * 9.082_f32 - x.cos();
        let mut acc = Accumulator895::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 1.191_f32 + y.sin();
        let b = y * 7.795_f32 - x.cos();
        let mut acc = Accumulator895::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.255_f32 + y.sin();
        let b = y * 1.587_f32 - x.cos();
        let mut acc = Accumulator895::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 4.714_f32 + y.sin();
        let b = y * 0.621_f32 - x.cos();
        let mut acc = Accumulator895::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 5.167_f32 + y.sin();
        let b = y * 4.392_f32 - x.cos();
        let mut acc = Accumulator895::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.33_f32 + y.sin();
        let b = y * 6.061_f32 - x.cos();
        let mut acc = Accumulator895::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_895(seed: u64) -> u64 {
        let now = Utc::now();
        (now.timestamp() as u64).wrapping_add(seed)
    }

    pub fn run_all_895() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_895(total as u64) % 997) as f32;
        total
    }
}

pub mod m896 {
    use super::*;

    pub struct Accumulator896<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator896<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 0.142_f32 + y.sin();
        let b = y * 8.845_f32 - x.cos();
        let mut acc = Accumulator896::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.433_f32 + y.sin();
        let b = y * 8.944_f32 - x.cos();
        let mut acc = Accumulator896::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.063_f32 + y.sin();
        let b = y * 0.624_f32 - x.cos();
        let mut acc = Accumulator896::new(a);
        acc.add(b);
        (acc.value * 11_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.602_f32 + y.sin();
        let b = y * 3.103_f32 - x.cos();
        let mut acc = Accumulator896::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 8.46_f32 + y.sin();
        let b = y * 3.522_f32 - x.cos();
        let mut acc = Accumulator896::new(a);
        acc.add(b);
        (acc.value * 70_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 1.442_f32 + y.sin();
        let b = y * 0.335_f32 - x.cos();
        let mut acc = Accumulator896::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 8.39_f32 + y.sin();
        let b = y * 1.042_f32 - x.cos();
        let mut acc = Accumulator896::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.969_f32 + y.sin();
        let b = y * 4.709_f32 - x.cos();
        let mut acc = Accumulator896::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.32_f32 + y.sin();
        let b = y * 4.786_f32 - x.cos();
        let mut acc = Accumulator896::new(a);
        acc.add(b);
        (acc.value * 60_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 9.243_f32 + y.sin();
        let b = y * 4.472_f32 - x.cos();
        let mut acc = Accumulator896::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 8.765_f32 + y.sin();
        let b = y * 0.842_f32 - x.cos();
        let mut acc = Accumulator896::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 4.087_f32 + y.sin();
        let b = y * 7.239_f32 - x.cos();
        let mut acc = Accumulator896::new(a);
        acc.add(b);
        (acc.value * 52_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 8.51_f32 + y.sin();
        let b = y * 8.67_f32 - x.cos();
        let mut acc = Accumulator896::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 0.315_f32 + y.sin();
        let b = y * 5.346_f32 - x.cos();
        let mut acc = Accumulator896::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.562_f32 + y.sin();
        let b = y * 4.216_f32 - x.cos();
        let mut acc = Accumulator896::new(a);
        acc.add(b);
        (acc.value * 55_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 9.558_f32 + y.sin();
        let b = y * 3.548_f32 - x.cos();
        let mut acc = Accumulator896::new(a);
        acc.add(b);
        (acc.value * 82_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.381_f32 + y.sin();
        let b = y * 3.166_f32 - x.cos();
        let mut acc = Accumulator896::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 7.043_f32 + y.sin();
        let b = y * 6.097_f32 - x.cos();
        let mut acc = Accumulator896::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 2.377_f32 + y.sin();
        let b = y * 3.435_f32 - x.cos();
        let mut acc = Accumulator896::new(a);
        acc.add(b);
        (acc.value * 13_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.257_f32 + y.sin();
        let b = y * 5.323_f32 - x.cos();
        let mut acc = Accumulator896::new(a);
        acc.add(b);
        (acc.value * 68_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_896(seed: u64) -> u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(0..1_000_000);
        n as u64
    }

    pub fn run_all_896() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_896(total as u64) % 997) as f32;
        total
    }
}

pub mod m897 {
    use super::*;

    pub struct Accumulator897<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator897<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.22_f32 + y.sin();
        let b = y * 2.48_f32 - x.cos();
        let mut acc = Accumulator897::new(a);
        acc.add(b);
        (acc.value * 24_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 8.576_f32 + y.sin();
        let b = y * 6.649_f32 - x.cos();
        let mut acc = Accumulator897::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 5.712_f32 + y.sin();
        let b = y * 4.963_f32 - x.cos();
        let mut acc = Accumulator897::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 4.778_f32 + y.sin();
        let b = y * 1.728_f32 - x.cos();
        let mut acc = Accumulator897::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 3.74_f32 + y.sin();
        let b = y * 8.775_f32 - x.cos();
        let mut acc = Accumulator897::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 6.139_f32 + y.sin();
        let b = y * 6.737_f32 - x.cos();
        let mut acc = Accumulator897::new(a);
        acc.add(b);
        (acc.value * 80_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.309_f32 + y.sin();
        let b = y * 0.764_f32 - x.cos();
        let mut acc = Accumulator897::new(a);
        acc.add(b);
        (acc.value * 36_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 6.643_f32 + y.sin();
        let b = y * 7.421_f32 - x.cos();
        let mut acc = Accumulator897::new(a);
        acc.add(b);
        (acc.value * 35_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 7.602_f32 + y.sin();
        let b = y * 7.064_f32 - x.cos();
        let mut acc = Accumulator897::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 6.652_f32 + y.sin();
        let b = y * 8.89_f32 - x.cos();
        let mut acc = Accumulator897::new(a);
        acc.add(b);
        (acc.value * 44_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.164_f32 + y.sin();
        let b = y * 9.654_f32 - x.cos();
        let mut acc = Accumulator897::new(a);
        acc.add(b);
        (acc.value * 85_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 0.399_f32 + y.sin();
        let b = y * 1.463_f32 - x.cos();
        let mut acc = Accumulator897::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 0.391_f32 + y.sin();
        let b = y * 3.424_f32 - x.cos();
        let mut acc = Accumulator897::new(a);
        acc.add(b);
        (acc.value * 64_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 7.038_f32 + y.sin();
        let b = y * 3.474_f32 - x.cos();
        let mut acc = Accumulator897::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 1.87_f32 + y.sin();
        let b = y * 7.101_f32 - x.cos();
        let mut acc = Accumulator897::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.707_f32 + y.sin();
        let b = y * 6.078_f32 - x.cos();
        let mut acc = Accumulator897::new(a);
        acc.add(b);
        (acc.value * 88_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 0.152_f32 + y.sin();
        let b = y * 2.089_f32 - x.cos();
        let mut acc = Accumulator897::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.859_f32 + y.sin();
        let b = y * 2.087_f32 - x.cos();
        let mut acc = Accumulator897::new(a);
        acc.add(b);
        (acc.value * 10_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 3.707_f32 + y.sin();
        let b = y * 7.707_f32 - x.cos();
        let mut acc = Accumulator897::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 8.545_f32 + y.sin();
        let b = y * 3.709_f32 - x.cos();
        let mut acc = Accumulator897::new(a);
        acc.add(b);
        (acc.value * 71_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_897(seed: u64) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(format!("m897-{seed}").as_bytes());
        let digest = hasher.finalize();
        u64::from_le_bytes(digest[0..8].try_into().unwrap())
    }

    pub fn run_all_897() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_897(total as u64) % 997) as f32;
        total
    }
}

pub mod m898 {
    use super::*;

    pub struct Accumulator898<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator898<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 2.171_f32 + y.sin();
        let b = y * 7.525_f32 - x.cos();
        let mut acc = Accumulator898::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 1.742_f32 + y.sin();
        let b = y * 7.936_f32 - x.cos();
        let mut acc = Accumulator898::new(a);
        acc.add(b);
        (acc.value * 25_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 7.773_f32 + y.sin();
        let b = y * 6.244_f32 - x.cos();
        let mut acc = Accumulator898::new(a);
        acc.add(b);
        (acc.value * 3_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 0.365_f32 + y.sin();
        let b = y * 2.626_f32 - x.cos();
        let mut acc = Accumulator898::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 9.472_f32 + y.sin();
        let b = y * 6.226_f32 - x.cos();
        let mut acc = Accumulator898::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 5.772_f32 + y.sin();
        let b = y * 3.353_f32 - x.cos();
        let mut acc = Accumulator898::new(a);
        acc.add(b);
        (acc.value * 5_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 1.641_f32 + y.sin();
        let b = y * 5.959_f32 - x.cos();
        let mut acc = Accumulator898::new(a);
        acc.add(b);
        (acc.value * 72_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 3.113_f32 + y.sin();
        let b = y * 4.259_f32 - x.cos();
        let mut acc = Accumulator898::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 3.975_f32 + y.sin();
        let b = y * 7.954_f32 - x.cos();
        let mut acc = Accumulator898::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.185_f32 + y.sin();
        let b = y * 8.259_f32 - x.cos();
        let mut acc = Accumulator898::new(a);
        acc.add(b);
        (acc.value * 33_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 0.402_f32 + y.sin();
        let b = y * 5.431_f32 - x.cos();
        let mut acc = Accumulator898::new(a);
        acc.add(b);
        (acc.value * 29_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 2.764_f32 + y.sin();
        let b = y * 1.023_f32 - x.cos();
        let mut acc = Accumulator898::new(a);
        acc.add(b);
        (acc.value * 77_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 6.929_f32 + y.sin();
        let b = y * 0.475_f32 - x.cos();
        let mut acc = Accumulator898::new(a);
        acc.add(b);
        (acc.value * 66_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 5.936_f32 + y.sin();
        let b = y * 3.547_f32 - x.cos();
        let mut acc = Accumulator898::new(a);
        acc.add(b);
        (acc.value * 59_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 6.506_f32 + y.sin();
        let b = y * 5.182_f32 - x.cos();
        let mut acc = Accumulator898::new(a);
        acc.add(b);
        (acc.value * 14_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 6.059_f32 + y.sin();
        let b = y * 5.673_f32 - x.cos();
        let mut acc = Accumulator898::new(a);
        acc.add(b);
        (acc.value * 39_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 8.948_f32 + y.sin();
        let b = y * 5.422_f32 - x.cos();
        let mut acc = Accumulator898::new(a);
        acc.add(b);
        (acc.value * 6_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 6.554_f32 + y.sin();
        let b = y * 8.186_f32 - x.cos();
        let mut acc = Accumulator898::new(a);
        acc.add(b);
        (acc.value * 7_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 4.395_f32 + y.sin();
        let b = y * 8.347_f32 - x.cos();
        let mut acc = Accumulator898::new(a);
        acc.add(b);
        (acc.value * 9_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 3.435_f32 + y.sin();
        let b = y * 4.516_f32 - x.cos();
        let mut acc = Accumulator898::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_898(seed: u64) -> u64 {
        let perlin = Perlin::new(seed as u32);
        let v = perlin.get([seed as f64 * 0.01, 1.0]);
        (v * 1_000_000.0) as i64 as u64
    }

    pub fn run_all_898() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_898(total as u64) % 997) as f32;
        total
    }
}

pub mod m899 {
    use super::*;

    pub struct Accumulator899<T> {
        pub value: T,
    }

    impl<T: std::ops::Add<Output = T> + Copy> Accumulator899<T> {
        pub fn new(value: T) -> Self {
            Self { value }
        }
        pub fn add(&mut self, other: T) -> T {
            self.value = self.value + other;
            self.value
        }
    }

    pub fn calc_0(x: f32, y: f32) -> f32 {
        let a = x * 5.252_f32 + y.sin();
        let b = y * 9.173_f32 - x.cos();
        let mut acc = Accumulator899::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_1(x: f32, y: f32) -> f32 {
        let a = x * 9.694_f32 + y.sin();
        let b = y * 4.453_f32 - x.cos();
        let mut acc = Accumulator899::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_2(x: f32, y: f32) -> f32 {
        let a = x * 3.263_f32 + y.sin();
        let b = y * 2.149_f32 - x.cos();
        let mut acc = Accumulator899::new(a);
        acc.add(b);
        (acc.value * 93_f32).rem_euclid(1000.0)
    }

    pub fn calc_3(x: f32, y: f32) -> f32 {
        let a = x * 7.952_f32 + y.sin();
        let b = y * 2.541_f32 - x.cos();
        let mut acc = Accumulator899::new(a);
        acc.add(b);
        (acc.value * 61_f32).rem_euclid(1000.0)
    }

    pub fn calc_4(x: f32, y: f32) -> f32 {
        let a = x * 2.014_f32 + y.sin();
        let b = y * 6.719_f32 - x.cos();
        let mut acc = Accumulator899::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_5(x: f32, y: f32) -> f32 {
        let a = x * 8.707_f32 + y.sin();
        let b = y * 7.854_f32 - x.cos();
        let mut acc = Accumulator899::new(a);
        acc.add(b);
        (acc.value * 47_f32).rem_euclid(1000.0)
    }

    pub fn calc_6(x: f32, y: f32) -> f32 {
        let a = x * 5.921_f32 + y.sin();
        let b = y * 1.552_f32 - x.cos();
        let mut acc = Accumulator899::new(a);
        acc.add(b);
        (acc.value * 75_f32).rem_euclid(1000.0)
    }

    pub fn calc_7(x: f32, y: f32) -> f32 {
        let a = x * 4.843_f32 + y.sin();
        let b = y * 1.725_f32 - x.cos();
        let mut acc = Accumulator899::new(a);
        acc.add(b);
        (acc.value * 69_f32).rem_euclid(1000.0)
    }

    pub fn calc_8(x: f32, y: f32) -> f32 {
        let a = x * 1.771_f32 + y.sin();
        let b = y * 1.61_f32 - x.cos();
        let mut acc = Accumulator899::new(a);
        acc.add(b);
        (acc.value * 40_f32).rem_euclid(1000.0)
    }

    pub fn calc_9(x: f32, y: f32) -> f32 {
        let a = x * 1.509_f32 + y.sin();
        let b = y * 6.671_f32 - x.cos();
        let mut acc = Accumulator899::new(a);
        acc.add(b);
        (acc.value * 58_f32).rem_euclid(1000.0)
    }

    pub fn calc_10(x: f32, y: f32) -> f32 {
        let a = x * 5.902_f32 + y.sin();
        let b = y * 1.924_f32 - x.cos();
        let mut acc = Accumulator899::new(a);
        acc.add(b);
        (acc.value * 94_f32).rem_euclid(1000.0)
    }

    pub fn calc_11(x: f32, y: f32) -> f32 {
        let a = x * 6.475_f32 + y.sin();
        let b = y * 8.037_f32 - x.cos();
        let mut acc = Accumulator899::new(a);
        acc.add(b);
        (acc.value * 56_f32).rem_euclid(1000.0)
    }

    pub fn calc_12(x: f32, y: f32) -> f32 {
        let a = x * 9.759_f32 + y.sin();
        let b = y * 3.382_f32 - x.cos();
        let mut acc = Accumulator899::new(a);
        acc.add(b);
        (acc.value * 97_f32).rem_euclid(1000.0)
    }

    pub fn calc_13(x: f32, y: f32) -> f32 {
        let a = x * 6.363_f32 + y.sin();
        let b = y * 3.699_f32 - x.cos();
        let mut acc = Accumulator899::new(a);
        acc.add(b);
        (acc.value * 62_f32).rem_euclid(1000.0)
    }

    pub fn calc_14(x: f32, y: f32) -> f32 {
        let a = x * 7.644_f32 + y.sin();
        let b = y * 4.402_f32 - x.cos();
        let mut acc = Accumulator899::new(a);
        acc.add(b);
        (acc.value * 15_f32).rem_euclid(1000.0)
    }

    pub fn calc_15(x: f32, y: f32) -> f32 {
        let a = x * 2.294_f32 + y.sin();
        let b = y * 3.221_f32 - x.cos();
        let mut acc = Accumulator899::new(a);
        acc.add(b);
        (acc.value * 63_f32).rem_euclid(1000.0)
    }

    pub fn calc_16(x: f32, y: f32) -> f32 {
        let a = x * 7.106_f32 + y.sin();
        let b = y * 4.962_f32 - x.cos();
        let mut acc = Accumulator899::new(a);
        acc.add(b);
        (acc.value * 48_f32).rem_euclid(1000.0)
    }

    pub fn calc_17(x: f32, y: f32) -> f32 {
        let a = x * 2.671_f32 + y.sin();
        let b = y * 0.422_f32 - x.cos();
        let mut acc = Accumulator899::new(a);
        acc.add(b);
        (acc.value * 57_f32).rem_euclid(1000.0)
    }

    pub fn calc_18(x: f32, y: f32) -> f32 {
        let a = x * 8.103_f32 + y.sin();
        let b = y * 4.8_f32 - x.cos();
        let mut acc = Accumulator899::new(a);
        acc.add(b);
        (acc.value * 2_f32).rem_euclid(1000.0)
    }

    pub fn calc_19(x: f32, y: f32) -> f32 {
        let a = x * 5.692_f32 + y.sin();
        let b = y * 2.251_f32 - x.cos();
        let mut acc = Accumulator899::new(a);
        acc.add(b);
        (acc.value * 30_f32).rem_euclid(1000.0)
    }

    pub fn dep_touch_899(seed: u64) -> u64 {
        let re = Regex::new(r"m899-(\d+)").unwrap();
        let hay = format!("m899-{seed}-tail");
        re.captures(&hay).map(|c| c[1].len() as u64).unwrap_or(0)
    }

    pub fn run_all_899() -> f32 {
        let mut total = 0.0_f32;
        total += calc_0(total, 0.0);
        total += calc_1(total, 1.0);
        total += calc_2(total, 2.0);
        total += calc_3(total, 3.0);
        total += calc_4(total, 4.0);
        total += calc_5(total, 5.0);
        total += calc_6(total, 6.0);
        total += calc_7(total, 7.0);
        total += calc_8(total, 8.0);
        total += calc_9(total, 9.0);
        total += calc_10(total, 10.0);
        total += calc_11(total, 11.0);
        total += calc_12(total, 12.0);
        total += calc_13(total, 13.0);
        total += calc_14(total, 14.0);
        total += calc_15(total, 15.0);
        total += calc_16(total, 16.0);
        total += calc_17(total, 17.0);
        total += calc_18(total, 18.0);
        total += calc_19(total, 19.0);
        total += (dep_touch_899(total as u64) % 997) as f32;
        total
    }
}

pub fn touch_bulk_8() -> f32 {
    let mut total = 0.0_f32;
    total += m800::run_all_800();
    total += m801::run_all_801();
    total += m802::run_all_802();
    total += m803::run_all_803();
    total += m804::run_all_804();
    total += m805::run_all_805();
    total += m806::run_all_806();
    total += m807::run_all_807();
    total += m808::run_all_808();
    total += m809::run_all_809();
    total += m810::run_all_810();
    total += m811::run_all_811();
    total += m812::run_all_812();
    total += m813::run_all_813();
    total += m814::run_all_814();
    total += m815::run_all_815();
    total += m816::run_all_816();
    total += m817::run_all_817();
    total += m818::run_all_818();
    total += m819::run_all_819();
    total += m820::run_all_820();
    total += m821::run_all_821();
    total += m822::run_all_822();
    total += m823::run_all_823();
    total += m824::run_all_824();
    total += m825::run_all_825();
    total += m826::run_all_826();
    total += m827::run_all_827();
    total += m828::run_all_828();
    total += m829::run_all_829();
    total += m830::run_all_830();
    total += m831::run_all_831();
    total += m832::run_all_832();
    total += m833::run_all_833();
    total += m834::run_all_834();
    total += m835::run_all_835();
    total += m836::run_all_836();
    total += m837::run_all_837();
    total += m838::run_all_838();
    total += m839::run_all_839();
    total += m840::run_all_840();
    total += m841::run_all_841();
    total += m842::run_all_842();
    total += m843::run_all_843();
    total += m844::run_all_844();
    total += m845::run_all_845();
    total += m846::run_all_846();
    total += m847::run_all_847();
    total += m848::run_all_848();
    total += m849::run_all_849();
    total += m850::run_all_850();
    total += m851::run_all_851();
    total += m852::run_all_852();
    total += m853::run_all_853();
    total += m854::run_all_854();
    total += m855::run_all_855();
    total += m856::run_all_856();
    total += m857::run_all_857();
    total += m858::run_all_858();
    total += m859::run_all_859();
    total += m860::run_all_860();
    total += m861::run_all_861();
    total += m862::run_all_862();
    total += m863::run_all_863();
    total += m864::run_all_864();
    total += m865::run_all_865();
    total += m866::run_all_866();
    total += m867::run_all_867();
    total += m868::run_all_868();
    total += m869::run_all_869();
    total += m870::run_all_870();
    total += m871::run_all_871();
    total += m872::run_all_872();
    total += m873::run_all_873();
    total += m874::run_all_874();
    total += m875::run_all_875();
    total += m876::run_all_876();
    total += m877::run_all_877();
    total += m878::run_all_878();
    total += m879::run_all_879();
    total += m880::run_all_880();
    total += m881::run_all_881();
    total += m882::run_all_882();
    total += m883::run_all_883();
    total += m884::run_all_884();
    total += m885::run_all_885();
    total += m886::run_all_886();
    total += m887::run_all_887();
    total += m888::run_all_888();
    total += m889::run_all_889();
    total += m890::run_all_890();
    total += m891::run_all_891();
    total += m892::run_all_892();
    total += m893::run_all_893();
    total += m894::run_all_894();
    total += m895::run_all_895();
    total += m896::run_all_896();
    total += m897::run_all_897();
    total += m898::run_all_898();
    total += m899::run_all_899();
    total
}
