use super::Money;
use crate::{Currency, Exchange, ExchangeRates};

impl Exchange for Money
{
	/// The result will be [rounded](crate::Decimal::rescale) to two decimal places.
	///
	/// # See
	///
	/// * [`Exchange::exchange`]
	fn exchange_mut(&mut self, currency: Currency, rates: &ExchangeRates)
	{
		// noop for same currency
		if self.currency == currency
		{
			return;
		}

		let mut exchanged = self.amount * rates.index(&self.currency..&currency);
		exchanged.rescale(2);

		self.amount = exchanged;
		self.currency = currency;
	}
}

#[cfg(test)]
mod tests
{
	use pretty_assertions::assert_eq;

	use super::{Currency, ExchangeRates, Money};
	use crate::{Exchange, SAMPLE_EXCHANGE_RATES_CSV};

	#[test]
	fn exchange()
	{
		let exchange_rates = SAMPLE_EXCHANGE_RATES_CSV.parse::<ExchangeRates>().unwrap();

		let usd = Money::new(20_00, 2, Currency::Usd);

		let usd_to_jpy = usd.exchange(Currency::Jpy, &exchange_rates);
		assert_eq!(usd_to_jpy, Money::new(2195_95, 2, Currency::Jpy));

		// Assert round-trip works
		let usd_to_jpy_to_usd = usd_to_jpy.exchange(Currency::Usd, &exchange_rates);
		assert_eq!(usd, usd_to_jpy_to_usd);
	}
}
