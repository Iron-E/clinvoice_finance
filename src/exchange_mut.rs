use crate::{Currency, ExchangeRates};

/// Same as [`Exchange`][exchange], except exchanges the value in-place (rather than creating a new exchanged
/// value from the parameters).
///
/// Useful for when you do not own the data you wish to exchange. Otherwise, [`Exchange`][exchange]
/// is preferred.
///
/// [exchange]: crate::Exchange
pub trait ExchangeMut
{
	/// Exchange some quantity into another `currency` using `rates`.
	fn exchange_mut(&mut self, currency: Currency, rates: &ExchangeRates);
}

impl<T> ExchangeMut for [T]
where
	T: ExchangeMut,
{
	fn exchange_mut(&mut self, currency: Currency, rates: &ExchangeRates)
	{
		self
			.iter_mut()
			.for_each(|t| t.exchange_mut(currency, rates));
	}
}

impl<T> ExchangeMut for Vec<T>
where
	T: ExchangeMut,
{
	fn exchange_mut(&mut self, currency: Currency, rates: &ExchangeRates)
	{
		self.as_mut_slice().exchange_mut(currency, rates);
	}
}

#[cfg(test)]
mod tests
{
	use pretty_assertions::assert_eq;

	use crate::{Currency, ExchangeMut, Money, SAMPLE_EXCHANGE_RATES_CSV};

	#[test]
	fn exchange()
	{
		let rates = SAMPLE_EXCHANGE_RATES_CSV.parse().unwrap();

		let mut money = vec![
			Money::new(1750, 0, Currency::Jpy),
			Money::new(20_00, 2, Currency::Usd),
		];

		money.exchange_mut(Default::default(), &rates);
		money.into_iter().for_each(|m| {
			assert_eq!(m.currency, Default::default());
		});
	}
}
