# `money`

<!-- cargo-rdme start -->

This crate provides functionality for [storing][money] and [exchanging][rates] various [ISO-4217](https://www.iso.org/iso-4217-currency-codes.html) [currency codes][currency] using the [European Central Bank](https://www.ecb.europa.eu/stats/policy_and_exchange_rates/euro_reference_exchange_rates/).

## Features

* `serde` adds support for the [serde][serde-rs] crate.

## Re-exports

* [`rust_decimal::Decimal`][decimal], because it is required to create [`Money`][money].

[currency]: https://docs.rs/money2/latest/money2/enum.Currency.html
[decimal]: https://docs.rs/rust_decimal/latest/rust_decimal/struct.Decimal.html
[money]: https://docs.rs/money2/latest/money2/struct.Money.html
[rates]: https://docs.rs/money2/latest/money2/struct.ExchangeRates.html
[serde-rs]: https://serde.rs

<!-- cargo-rdme end -->
