use core::str::FromStr;

use super::Currency;

impl TryFrom<&str> for Currency
{
	type Error = <Self as FromStr>::Err;

	fn try_from(s: &str) -> Result<Self, Self::Error>
	{
		s.parse()
	}
}
