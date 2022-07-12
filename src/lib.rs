//! This crate provides functionality for [storing](Money) and [exchanging](ExchangeRates) various
//! [ISO-4217](https://www.iso.org/iso-4217-currency-codes.html) [currency codes](Currency) using
//! the [European Central Bank](https://www.ecb.europa.eu/stats/policy_and_exchange_rates/euro_reference_exchange_rates/).
//!
//! # Features
//!
//! * `serde_support` adds support for the [`serde`] crate.

#![warn(missing_docs)]

mod currency;
mod error;
mod exchange_rates;
mod exchangeable;
mod money;

pub use currency::Currency;
pub use error::{Error, Result};
pub use exchange_rates::ExchangeRates;
pub use exchangeable::Exchangeable;
pub use money::Money;
pub use rust_decimal::Decimal;

#[cfg(test)]
pub(crate) const SAMPLE_EXCHANGE_RATES_CSV: &str =
	"Date, USD, JPY, BGN, CZK, DKK, GBP, HUF, PLN, RON, SEK, CHF, ISK, NOK, HRK, RUB, TRY, AUD, \
	 BRL, CAD, CNY, HKD, IDR, ILS, INR, KRW, MXN, MYR, NZD, PHP, SGD, THB, ZAR, \n03 June 2021, \
	 1.2187, 133.81, 1.9558, 25.448, 7.4365, 0.85955, 345.82, 4.4520, 4.9220, 10.1145, 1.0961, \
	 146.30, 10.1501, 7.5013, 89.2163, 10.5650, 1.5792, 6.1894, 1.4710, 7.7910, 9.4551, 17420.91, \
	 3.9598, 88.8755, 1357.75, 24.3300, 5.0241, 1.6915, 58.208, 1.6141, 37.938, 16.5218, ";
