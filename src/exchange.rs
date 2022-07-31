use crate::{Currency, ExchangeRates};

/// Implementors of this trait contain quantities which are relative to the [`Currency`] they are
/// currently in. To view them in another [`Currency`], they must be [exchanged](Exchange::exchange) using
/// the [rates](ExchangeRates) of conversion.
pub trait Exchange
{
	/// The type which will be output by [`exchange`].
	type Output;

	/// Exchange some quantity into another `currency` using `rates`.
	fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Self::Output;
}

impl<T> Exchange for Vec<T>
where
	T: Exchange,
	Self: FromIterator<T::Output>,
{
	type Output = Self;

	fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Self
	{
		self
			.into_iter()
			.map(|exchangeable| exchangeable.exchange(currency, rates))
			.collect()
	}
}

impl<'any, T> Exchange for &'any [T]
where
	&'any T: Exchange,
	Vec<T>: FromIterator<<&'any T as Exchange>::Output>,
{
	type Output = Vec<T>;

	fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Self::Output
	{
		self
			.iter()
			.map(|exchangeable| exchangeable.exchange(currency, rates))
			.collect()
	}
}
