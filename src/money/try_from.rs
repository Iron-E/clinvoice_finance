use core::str::FromStr;

use super::Money;

impl TryFrom<&str> for Money
{
	type Error = <Self as FromStr>::Err;

	fn try_from(s: &str) -> Result<Self, Self::Error>
	{
		s.parse()
	}
}
