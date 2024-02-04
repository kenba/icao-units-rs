# icao-units-rs

[![License](https://img.shields.io/badge/License-MIT-blue)](https://opensource.org/license/mit/)
[![Rust](https://github.com/kenba/icao-units-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/kenba/icao-units-rs/actions)
[![codecov](https://codecov.io/gh/kenba/icao-units-rs/graph/badge.svg?token=6DTOY9Y4BT)](https://codecov.io/gh/kenba/icao-units-rs)

Units for air navigation as defined in [ICAO Annex 5](https://store.icao.int/en/annex-5-units-of-measurement-to-be-used-in-the-air-and-ground-services).

The library defines SI units used by the [International Standard Atmosphere](https://en.wikipedia.org/wiki/International_Standard_Atmosphere) (ISA), the non-SI units defined in
`Annex 5` and [FlightLevel](https://en.wikipedia.org/wiki/Flight_level).

## Design

The library uses the [newtype](https://rust-unofficial.github.io/patterns/patterns/behavioural/newtype.html) idiom to represent ICAO units and the [From](https://doc.rust-lang.org/std/convert/trait.From.html) trait to convert between SI and non-SI units using
the conversion factors defined in `ICAO Annex 5` Table 3-3.

## Contribution

If you want to contribute through code or documentation, the [Contributing](CONTRIBUTING.md) guide is the best place to start.  
Just please abide by our [Code of Conduct](CODE_OF_CONDUCT.md).

## License

`via-units-rs` is provided under a MIT license, see [LICENSE](LICENSE).

Contact <enquiries@via-technology.aero> for more information.
