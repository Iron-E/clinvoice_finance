# `money`

<!-- cargo-rdme start -->

This crate provides functionality for [storing][money] and [exchanging][exchange] various [ISO-4217](https://www.iso.org/iso-4217-currency-codes.html) [currency codes][currency] using the [European Central Bank](https://www.ecb.europa.eu/stats/policy_and_exchange_rates/euro_reference_exchange_rates/).

Has the capability of performing exchanges based on the rates as they were in a specific date in time, and keeps up to date with the latest exchange rates if your software must stay running.

## Feature Flags

* `all` enables all feature flags.
* `history` enables the ability to perform exchanges based on historical records.
* `num-traits` enables performing safe numeric calculations with money via the [num-traits](https://crates.io/crates/num-traits) crate.
* `serde` enables support for the [serde](https://serde.rs) crate.

## Re-exports

* [`rust_decimal::Decimal`][decimal], because it is required to create [`Money`][money].

[currency]: https://docs.rs/money2/latest/money2/enum.Currency.html
[decimal]: https://docs.rs/rust_decimal/latest/rust_decimal/struct.Decimal.html
[exchange]: https://docs.rs/money2/latest/money2/exchange/trait.Exchange.html
[money]: https://docs.rs/money2/latest/money2/struct.Money.html

<!-- cargo-rdme end -->
