//! This crate provides functionality for [storing][money] and [exchanging][exchange] various
//! [ISO-4217](https://www.iso.org/iso-4217-currency-codes.html) [currency codes][currency] using
//! the [European Central Bank](https://www.ecb.europa.eu/stats/policy_and_exchange_rates/euro_reference_exchange_rates/).
//!
//! # Features
//!
//! * `serde` adds support for the [serde](https://serde.rs) crate.
//!
//! # Re-exports
//!
//! * [`rust_decimal::Decimal`][decimal], because it is required to create [`Money`][money].
//!
//! [currency]: https://docs.rs/money2/latest/money2/enum.Currency.html
//! [decimal]: https://docs.rs/rust_decimal/latest/rust_decimal/struct.Decimal.html
//! [exchange]: https://docs.rs/money2/latest/money2/exchange/trait.Exchange.html
//! [money]: https://docs.rs/money2/latest/money2/struct.Money.html

#![allow(clippy::drop_non_drop)]
#![forbid(unsafe_code)]
#![warn(
	missing_docs,
	clippy::cargo_common_metadata,
	clippy::alloc_instead_of_core,
	clippy::allow_attributes_without_reason,
	clippy::as_underscore,
	clippy::branches_sharing_code,
	clippy::cast_lossless,
	clippy::checked_conversions,
	clippy::cloned_instead_of_copied,
	clippy::dbg_macro,
	clippy::debug_assert_with_mut_call,
	clippy::doc_link_with_quotes,
	clippy::doc_markdown,
	clippy::empty_line_after_outer_attr,
	clippy::empty_structs_with_brackets,
	clippy::enum_glob_use,
	clippy::equatable_if_let,
	clippy::exit,
	clippy::explicit_into_iter_loop,
	clippy::explicit_iter_loop,
	clippy::fallible_impl_from,
	clippy::filetype_is_file,
	clippy::filter_map_next,
	clippy::flat_map_option,
	clippy::fn_to_numeric_cast_any,
	clippy::format_push_string,
	clippy::from_iter_instead_of_collect,
	clippy::get_unwrap,
	clippy::implicit_clone,
	clippy::inefficient_to_string,
	clippy::items_after_statements,
	clippy::manual_assert,
	clippy::manual_ok_or,
	clippy::map_unwrap_or,
	clippy::match_same_arms,
	clippy::missing_const_for_fn,
	clippy::missing_panics_doc,
	clippy::multiple_inherent_impl,
	clippy::mut_mut,
	clippy::needless_continue,
	clippy::option_if_let_else,
	clippy::option_option,
	clippy::range_minus_one,
	clippy::range_plus_one,
	clippy::redundant_closure_for_method_calls,
	clippy::redundant_else,
	clippy::ref_binding_to_reference,
	clippy::ref_option_ref,
	clippy::same_functions_in_if_condition,
	clippy::single_char_lifetime_names,
	clippy::std_instead_of_core,
	clippy::str_to_string,
	clippy::string_add,
	clippy::string_add_assign,
	clippy::string_to_string,
	clippy::try_err,
	clippy::unnecessary_join,
	clippy::unnecessary_wraps,
	clippy::use_self,
	clippy::used_underscore_binding,
	clippy::wildcard_imports
)]

mod currency;
mod error;
mod exchange;
mod exchange_rates;
mod money;

pub use currency::Currency;
pub use error::{Error, Result};
pub use exchange::Exchange;
pub use exchange_rates::ExchangeRates;
pub use money::Money;
pub use rust_decimal::Decimal;

#[cfg(test)]
pub(crate) const SAMPLE_EXCHANGE_RATES_CSV: &str =
	"Date, USD, JPY, BGN, CZK, DKK, GBP, HUF, PLN, RON, SEK, CHF, ISK, NOK, HRK, RUB, TRY, AUD, \
	 BRL, CAD, CNY, HKD, IDR, ILS, INR, KRW, MXN, MYR, NZD, PHP, SGD, THB, ZAR, \n03 June 2021, \
	 1.2187, 133.81, 1.9558, 25.448, 7.4365, 0.85955, 345.82, 4.4520, 4.9220, 10.1145, 1.0961, \
	 146.30, 10.1501, 7.5013, 89.2163, 10.5650, 1.5792, 6.1894, 1.4710, 7.7910, 9.4551, 17420.91, \
	 3.9598, 88.8755, 1357.75, 24.3300, 5.0241, 1.6915, 58.208, 1.6141, 37.938, 16.5218, ";
