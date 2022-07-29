use core::{convert::TryFrom, str::FromStr};

use super::ExchangeRates;

impl TryFrom<&str> for ExchangeRates
{
	type Error = <Self as FromStr>::Err;

	fn try_from(s: &str) -> Result<Self, Self::Error>
	{
		s.parse()
	}
}
