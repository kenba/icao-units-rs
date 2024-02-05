// Copyright (c) 2024 Via Technology Ltd.

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

//! Non-SI units used in air navigation and conversions to their SI equivalents.  
//! See ICAO Annex 5 Chapter 3, Table 3-3 and Chapter 4, Table 4-1.

use crate::si;
use std::convert::From;

/// A Nautical Mile `newtype` for representing distance.  
/// Used in navigation, generally for distances in excess of `4 000` m.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct NauticalMiles(pub f64);

/// The length of a Nautical Mile (NM) in metres (m).  
/// Definition from ICAO Annex 5 Table 3-3.
pub const METRES_PER_NAUTICAL_MILE: f64 = 1_852.0;

impl From<si::Metres> for NauticalMiles {
    fn from(a: si::Metres) -> Self {
        Self(a.0 / METRES_PER_NAUTICAL_MILE)
    }
}

impl From<NauticalMiles> for si::Metres {
    fn from(a: NauticalMiles) -> Self {
        Self(a.0 * METRES_PER_NAUTICAL_MILE)
    }
}

/// A Feet `newtype` for representing altitude.  
/// Used to report aircraft altitude below the
/// [transition altitude](https://en.wikipedia.org/wiki/Flight_level#Transition_altitude).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Feet(pub f64);

/// The length of a foot (ft) in metres (m).  
/// Definition from ICAO Annex 5 Table 3-3.
pub const METRES_PER_FOOT: f64 = 0.304_8;

impl From<si::Metres> for Feet {
    fn from(a: si::Metres) -> Self {
        Self(a.0 / METRES_PER_FOOT)
    }
}

impl From<Feet> for si::Metres {
    fn from(a: Feet) -> Self {
        Self(a.0 * METRES_PER_FOOT)
    }
}

/// A Knots `newtype` for representing speed.
/// Airspeed is sometimes reported in flight operations in terms of the ratio
/// MACH number.  
/// A conversion of 1 kt = 0.5 m/s is used in ICAO Annexes for the representation
/// of wind speed.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Knots(pub f64);

/// The conversion factor to Knots (kt) from metres per second (m/s).  
/// Calculated from `METRES_PER_NAUTICAL_MILE` / seconds in an hour,
/// because it is more precise than the ICAO definition: 0.514 444.
pub const METRES_PER_SECOND_TO_KNOTS: f64 = METRES_PER_NAUTICAL_MILE / 3600.0;

impl From<si::MetresPerSecond> for Knots {
    fn from(a: si::MetresPerSecond) -> Self {
        Self(a.0 / METRES_PER_SECOND_TO_KNOTS)
    }
}

impl From<Knots> for si::MetresPerSecond {
    fn from(a: Knots) -> Self {
        Self(a.0 * METRES_PER_SECOND_TO_KNOTS)
    }
}

/// A `FlightLevel` `newtype.`  
/// A `FlightLevel` is 100 `Feet`.  
/// Used to report aircraft altitude above the
/// [transition altitude](https://en.wikipedia.org/wiki/Flight_level#Transition_altitude).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct FlightLevel(pub f64);

/// The height of a flight level (FL) in feet (ft).
pub const FEET_PER_FLIGHT_LEVEL: f64 = 100.0;

impl From<Feet> for FlightLevel {
    fn from(a: Feet) -> Self {
        Self(a.0 / FEET_PER_FLIGHT_LEVEL)
    }
}

impl From<FlightLevel> for Feet {
    fn from(a: FlightLevel) -> Self {
        Self(a.0 * FEET_PER_FLIGHT_LEVEL)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::si;

    #[test]
    fn test_nautical_miles() {
        let one_nm = NauticalMiles(1.0);
        let one_nm_clone = one_nm.clone();
        assert_eq!(one_nm, one_nm_clone);
        let two_nm = NauticalMiles(2.0);
        assert!(one_nm < two_nm);

        print!("NauticalMiles: {:?}", one_nm);
    }

    #[test]
    fn test_convert_nautical_miles() {
        let one_nm = NauticalMiles(1.0);
        let metres = si::Metres::from(one_nm);
        assert_eq!(1852.0, metres.0);

        let result = NauticalMiles::from(metres);
        assert_eq!(1.0, result.0);
    }

    #[test]
    fn test_feet() {
        let one_ft = Feet(1.0);
        let one_ft_clone = one_ft.clone();
        assert_eq!(one_ft, one_ft_clone);
        let two_ft = Feet(2.0);
        assert!(one_ft < two_ft);

        print!("Feet: {:?}", one_ft);
    }

    #[test]
    fn test_convert_feet() {
        let one_foot = Feet(1.0);
        let metres = si::Metres::from(one_foot);
        assert_eq!(0.304_8, metres.0);

        let result = Feet::from(metres);
        assert_eq!(1.0, result.0);
    }

    #[test]
    fn test_knots() {
        let one_kt = Knots(1.0);
        let one_kt_clone = one_kt.clone();
        assert_eq!(one_kt, one_kt_clone);
        let two_kt = Knots(2.0);
        assert!(one_kt < two_kt);

        print!("Knots: {:?}", one_kt);
    }

    #[test]
    fn test_convert_knots() {
        let one_knot = Knots(1.0);
        let metres_per_second = si::MetresPerSecond::from(one_knot);

        // Definition from ICAO Annex 5 Table 3-3 is 0.514 444
        assert!(0.514_444 < metres_per_second.0);
        assert!(0.514_444_5 > metres_per_second.0);

        let result = Knots::from(metres_per_second);
        assert_eq!(1.0, result.0);
    }

    #[test]
    fn test_flight_level() {
        let one_fl = FlightLevel(1.0);
        let one_fl_clone = one_fl.clone();
        assert_eq!(one_fl, one_fl_clone);
        let two_fl = FlightLevel(2.0);
        assert!(one_fl < two_fl);

        print!("FlightLevel: {:?}", one_fl);
    }

    #[test]
    fn test_convert_flight_level() {
        let one_fl = FlightLevel(1.0);
        let feet = Feet::from(one_fl);
        assert_eq!(100.0, feet.0);

        let result = FlightLevel::from(feet);
        assert_eq!(1.0, result.0);
    }
}
