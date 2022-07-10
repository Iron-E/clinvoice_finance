use core::fmt::{Display, Formatter, Result};

use super::Money;

impl Display for Money
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		write!(formatter, "{} {}", self.amount, self.currency)
	}
}

#[cfg(test)]
mod tests
{
	use pretty_assertions::assert_eq;

	use super::Money;
	use crate::Currency;

	#[test]
	fn display()
	{
		assert_eq!(Money::new(50_00, 2, Currency::Usd).to_string(), "50.00 USD");
		assert_eq!(Money::new(90_00, 2, Currency::Eur).to_string(), "90.00 EUR");
		assert_eq!(Money::new(20000, 0, Currency::Jpy).to_string(), "20000 JPY");
	}
}
