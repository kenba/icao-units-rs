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

//! [![crates.io](https://img.shields.io/crates/v/icao-units.svg)](https://crates.io/crates/icao-units)
//! [![docs.io](https://docs.rs/icao-units/badge.svg)](https://docs.rs/icao-units/)
//! [![License](https://img.shields.io/badge/License-MIT-blue)](https://opensource.org/license/mit/)
//! [![Rust](https://github.com/kenba/icao-units-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/kenba/icao-units-rs/actions)
//! [![codecov](https://codecov.io/gh/kenba/icao-units-rs/graph/badge.svg?token=6DTOY9Y4BT)](https://codecov.io/gh/kenba/icao-units-rs)
//! 
//! Units for air navigation as defined in
//! [ICAO Annex 5](https://store.icao.int/en/annex-5-units-of-measurement-to-be-used-in-the-air-and-ground-services).
//! 
//! The library defines SI units used by the
//! [International Standard Atmosphere](https://en.wikipedia.org/wiki/International_Standard_Atmosphere) (ISA),
//! the non-SI units defined in  `ICAO Annex 5` Table 3-3 and
//! [FlightLevel](https://en.wikipedia.org/wiki/Flight_level).
//!
//! The library uses the [newtype](https://rust-unofficial.github.io/patterns/patterns/behavioural/newtype.html)
//! idiom to represent ICAO units and the [From](https://doc.rust-lang.org/std/convert/trait.From.html)
//! trait to convert between SI and non-SI units using the conversion factors
//! defined in `ICAO Annex 5` Table 3-3.

pub mod non_si;
pub mod si;
