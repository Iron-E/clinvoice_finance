use crate::{Currency, ExchangeMut, ExchangeRates};

/// Implementors of this trait contain quantities which are relative to the [`Currency`] they are
/// currently in. To view them in another [`Currency`], they must be [exchanged](Exchange::exchange) using
/// the [rates](ExchangeRates) of conversion.
pub trait Exchange: ExchangeMut + Sized
{
	/// Exchange some quantity into another `currency` using `rates`.
	fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Self
	{
		let mut s = self;
		s.exchange_mut(currency, rates);
		s
	}
}

impl<T> Exchange for T where T: ExchangeMut {}

#[cfg(test)]
mod tests
{
	use pretty_assertions::assert_eq;

	use crate::{Currency, Exchange, Money, SAMPLE_EXCHANGE_RATES_CSV};

	#[test]
	fn exchange()
	{
		let rates = SAMPLE_EXCHANGE_RATES_CSV.parse().unwrap();

		vec![
			Money::new(1750, 0, Currency::Jpy),
			Money::new(20_00, 2, Currency::Usd),
		]
		.exchange(Default::default(), &rates)
		.into_iter()
		.for_each(|m| {
			assert_eq!(m.currency, Default::default());
		});
	}
}
