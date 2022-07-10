use crate::{Currency, ExchangeRates};

/// Implementors of this trait contain quantities which are relative to the [`Currency`] they are
/// currently in. To view them in another [`Currency`], they must be [exchanged](Exchangeable::exchange) using
/// the [rates](ExchangeRates) of conversion.
pub trait Exchangeable
{
	/// Exchange some quantity into another `currency` using `rates`.
	fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Self;

	/// The same as [`exchange`](Self::exchange), but taking `self` by reference.
	fn exchange_ref(&self, currency: Currency, rates: &ExchangeRates) -> Self;
}

impl<T> Exchangeable for Vec<T>
where
	T: Exchangeable,
{
	fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Self
	{
		self
			.into_iter()
			.map(|exchangeable| exchangeable.exchange(currency, rates))
			.collect()
	}

	fn exchange_ref(&self, currency: Currency, rates: &ExchangeRates) -> Self
	{
		self
			.iter()
			.map(|exchangeable| exchangeable.exchange_ref(currency, rates))
			.collect()
	}
}
