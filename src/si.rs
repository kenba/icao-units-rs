// Copyright (c) 2024-2025 Ken Barker

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

//! Si units used in air navigation.
//!
//! See ICAO Annex 5 Chapter 3.

use core::ops::{Add, AddAssign, Neg, Sub, SubAssign};
use serde::{Deserialize, Serialize};

/// A `Metres` `newtype` for representing distance.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[repr(transparent)]
pub struct Metres(pub f64);

impl Metres {
    /// The absolute value.
    #[must_use]
    pub const fn abs(self) -> Self {
        Self(self.0.abs())
    }
}

impl Default for Metres {
    fn default() -> Self {
        Self(0.0)
    }
}

impl Add for Metres {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

impl AddAssign for Metres {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Neg for Metres {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(0.0 - self.0)
    }
}

impl Sub for Metres {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0)
    }
}

impl SubAssign for Metres {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

/// A `MetresPerSecond` `newtype` for representing speed.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[repr(transparent)]
pub struct MetresPerSecond(pub f64);

impl MetresPerSecond {
    /// The absolute value.
    #[must_use]
    pub const fn abs(self) -> Self {
        Self(self.0.abs())
    }
}

impl Default for MetresPerSecond {
    fn default() -> Self {
        Self(0.0)
    }
}

impl Add for MetresPerSecond {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

impl AddAssign for MetresPerSecond {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Neg for MetresPerSecond {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(0.0 - self.0)
    }
}

impl Sub for MetresPerSecond {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0)
    }
}

impl SubAssign for MetresPerSecond {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

/// A `MetresPerSecondSquared` `newtype` for representing acceleration.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[repr(transparent)]
pub struct MetresPerSecondSquared(pub f64);

impl Default for MetresPerSecondSquared {
    fn default() -> Self {
        Self(0.0)
    }
}

/// A Kelvin `newtype` for representing temperature.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[repr(transparent)]
pub struct Kelvin(pub f64);

impl Add for Kelvin {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

impl AddAssign for Kelvin {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Neg for Kelvin {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(0.0 - self.0)
    }
}

impl Sub for Kelvin {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0)
    }
}

impl SubAssign for Kelvin {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

/// A Pascals `newtype` for representing pressure.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[repr(transparent)]
pub struct Pascals(pub f64);

/// A Kilograms `newtype` for representing mass.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[repr(transparent)]
pub struct Kilograms(pub f64);

impl Default for Kilograms {
    fn default() -> Self {
        Self(0.0)
    }
}

impl Add for Kilograms {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

impl AddAssign for Kilograms {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Neg for Kilograms {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(0.0 - self.0)
    }
}

impl Sub for Kilograms {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0)
    }
}

impl SubAssign for Kilograms {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

/// A Kilograms `newtype` for representing density.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct KilogramsPerCubicMetre(pub f64);

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_metres() {
        let zero_m = Metres::default();
        assert_eq!(Metres(0.0), zero_m);
        let one_m = Metres(1.0);
        let mut one_m_clone = one_m.clone();
        assert_eq!(one_m, one_m_clone);
        let two_m = Metres(2.0);
        assert!(one_m < two_m);
        let minus_one_m = Metres(-1.0);
        assert_eq!(minus_one_m, -one_m);

        assert_eq!(one_m, minus_one_m.abs());

        assert_eq!(minus_one_m, one_m - two_m);
        one_m_clone -= two_m;
        assert_eq!(minus_one_m, one_m_clone);

        assert_eq!(one_m, minus_one_m + two_m);
        one_m_clone += two_m;
        assert_eq!(one_m, one_m_clone);

        let serialized = serde_json::to_string(&one_m).unwrap();
        let deserialized: Metres = serde_json::from_str(&serialized).unwrap();
        assert_eq!(one_m, deserialized);

        let bad_text = "junk";
        let _serde_error = serde_json::from_str::<Metres>(&bad_text).unwrap_err();

        print!("Metres: {:?}", one_m);
    }

    #[test]
    fn test_metres_per_second() {
        let zero_mps = MetresPerSecond::default();
        assert_eq!(MetresPerSecond(0.0), zero_mps);
        let one_mps = MetresPerSecond(1.0);
        let mut one_mps_clone = one_mps.clone();
        assert_eq!(one_mps, one_mps_clone);
        let two_mps = MetresPerSecond(2.0);
        assert!(one_mps < two_mps);
        let minus_one_mps = MetresPerSecond(-1.0);
        assert_eq!(minus_one_mps, -one_mps);

        assert_eq!(one_mps, minus_one_mps.abs());

        assert_eq!(minus_one_mps, one_mps - two_mps);
        one_mps_clone -= two_mps;
        assert_eq!(minus_one_mps, one_mps_clone);

        assert_eq!(one_mps, minus_one_mps + two_mps);
        one_mps_clone += two_mps;
        assert_eq!(one_mps, one_mps_clone);

        let serialized = serde_json::to_string(&one_mps).unwrap();
        let deserialized: MetresPerSecond = serde_json::from_str(&serialized).unwrap();
        assert_eq!(one_mps, deserialized);

        let bad_text = "junk";
        let _serde_error = serde_json::from_str::<MetresPerSecond>(&bad_text).unwrap_err();

        print!("MetresPerSecond: {:?}", one_mps);
    }

    #[test]
    fn test_metres_per_second_squared() {
        let zero_mps2 = MetresPerSecondSquared::default();
        assert_eq!(MetresPerSecondSquared(0.0), zero_mps2);
        let one_mps2 = MetresPerSecondSquared(1.0);
        let one_mps2_clone = one_mps2.clone();
        assert_eq!(one_mps2, one_mps2_clone);
        let two_mps2 = MetresPerSecondSquared(2.0);
        assert!(one_mps2 < two_mps2);

        let serialized = serde_json::to_string(&one_mps2).unwrap();
        let deserialized: MetresPerSecondSquared = serde_json::from_str(&serialized).unwrap();
        assert_eq!(one_mps2, deserialized);

        let bad_text = "junk";
        let _serde_error = serde_json::from_str::<MetresPerSecondSquared>(&bad_text).unwrap_err();

        print!("MetresPerSecondSquared: {:?}", one_mps2);
    }

    #[test]
    fn test_kelvin() {
        let one_k = Kelvin(1.0);
        let mut one_k_clone = one_k.clone();
        assert_eq!(one_k, one_k_clone);
        let two_k = Kelvin(2.0);
        assert!(one_k < two_k);
        let minus_one_k = Kelvin(-1.0);
        assert_eq!(minus_one_k, -one_k);

        assert_eq!(minus_one_k, one_k - two_k);
        one_k_clone -= two_k;
        assert_eq!(minus_one_k, one_k_clone);

        assert_eq!(one_k, minus_one_k + two_k);
        one_k_clone += two_k;
        assert_eq!(one_k, one_k_clone);

        let serialized = serde_json::to_string(&one_k).unwrap();
        let deserialized: Kelvin = serde_json::from_str(&serialized).unwrap();
        assert_eq!(one_k, deserialized);

        let bad_text = "junk";
        let _serde_error = serde_json::from_str::<Kelvin>(&bad_text).unwrap_err();

        print!("Kelvin: {:?}", one_k);
    }

    #[test]
    fn test_pascals() {
        let one_pa = Pascals(1.0);
        let one_pa_clone = one_pa.clone();
        assert_eq!(one_pa, one_pa_clone);
        let two_pa = Pascals(2.0);
        assert!(one_pa < two_pa);

        let serialized = serde_json::to_string(&one_pa).unwrap();
        let deserialized: Pascals = serde_json::from_str(&serialized).unwrap();
        assert_eq!(one_pa, deserialized);

        let bad_text = "junk";
        let _serde_error = serde_json::from_str::<Pascals>(&bad_text).unwrap_err();

        print!("Pascals: {:?}", one_pa);
    }

    #[test]
    fn test_kilograms() {
        let zero_kg = Kilograms::default();
        assert_eq!(Kilograms(0.0), zero_kg);
        let one_kg = Kilograms(1.0);
        let mut one_kg_clone = one_kg.clone();
        assert_eq!(one_kg, one_kg_clone);
        let two_kg = Kilograms(2.0);
        assert!(one_kg < two_kg);
        let minus_one_kg = Kilograms(-1.0);
        assert_eq!(minus_one_kg, -one_kg);

        assert_eq!(minus_one_kg, one_kg - two_kg);
        one_kg_clone -= two_kg;
        assert_eq!(minus_one_kg, one_kg_clone);

        assert_eq!(one_kg, minus_one_kg + two_kg);
        one_kg_clone += two_kg;
        assert_eq!(one_kg, one_kg_clone);

        let serialized = serde_json::to_string(&one_kg).unwrap();
        let deserialized: Kilograms = serde_json::from_str(&serialized).unwrap();
        assert_eq!(one_kg, deserialized);

        let bad_text = "junk";
        let _serde_error = serde_json::from_str::<Kilograms>(&bad_text).unwrap_err();

        print!("Kilograms: {:?}", one_kg);
    }

    #[test]
    fn test_kilograms_per_cubic_metre() {
        let one_kgm3 = KilogramsPerCubicMetre(1.0);
        let one_kgm3_clone = one_kgm3.clone();
        assert_eq!(one_kgm3, one_kgm3_clone);
        let two_kgm3 = KilogramsPerCubicMetre(2.0);
        assert!(one_kgm3 < two_kgm3);

        let serialized = serde_json::to_string(&one_kgm3).unwrap();
        let deserialized: KilogramsPerCubicMetre = serde_json::from_str(&serialized).unwrap();
        assert_eq!(one_kgm3, deserialized);

        let bad_text = "junk";
        let _serde_error = serde_json::from_str::<KilogramsPerCubicMetre>(&bad_text).unwrap_err();

        print!("KilogramsPerCubicMetre: {:?}", one_kgm3);
    }
}
