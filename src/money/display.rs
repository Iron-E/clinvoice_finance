use core::fmt::{Display, Formatter, Result};

use super::Money;

impl Display for Money
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		write!(formatter, "{} {}", self.amount, self.currency)
	}
}
