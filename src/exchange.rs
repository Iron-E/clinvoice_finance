use crate::{Currency, ExchangeRates};

/// Implementors of this trait contain quantities which are relative to the [`Currency`] they are
/// currently in. To view them in another [`Currency`], they must be [exchanged](Exchange::exchange)
/// using the [rates](ExchangeRates) of conversion.
pub trait Exchange
{
	/// Exchange some quantity into another `currency` using `rates`. Derived from the
	/// [`exchange_mut`](Self::exchange_mut) implementation.
	fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Self
	where
		Self: Sized,
	{
		let mut s = self;
		s.exchange_mut(currency, rates);
		s
	}

	/// Mutably exchange some quantity into another `currency` using `rates`.
	fn exchange_mut(&mut self, currency: Currency, rates: &ExchangeRates);
}

impl<T> Exchange for [T]
where
	T: Exchange,
{
	fn exchange_mut(&mut self, currency: Currency, rates: &ExchangeRates)
	{
		self.iter_mut().for_each(|t| t.exchange_mut(currency, rates));
	}
}

impl<T> Exchange for Vec<T>
where
	T: Exchange,
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

	use crate::{Currency, Exchange, Money, SAMPLE_EXCHANGE_RATES_CSV};

	#[test]
	fn exchange()
	{
		let rates = SAMPLE_EXCHANGE_RATES_CSV.parse().unwrap();

		let mut money =
			vec![Money::new(1750, 0, Currency::Jpy), Money::new(20_00, 2, Currency::Usd)];

		let exchanged = money.clone().exchange(Default::default(), &rates);

		money.exchange_mut(Default::default(), &rates);
		money.into_iter().zip(exchanged.into_iter()).for_each(|(lhs, rhs)| {
			assert_eq!(lhs, rhs);
			assert_eq!(lhs.currency, Currency::Eur);
		});
	}
}
