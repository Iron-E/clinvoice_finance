use core::{convert::TryFrom, str::FromStr};

use super::Currency;

impl TryFrom<&str> for Currency
{
	type Error = <Currency as FromStr>::Err;

	fn try_from(s: &str) -> Result<Self, Self::Error>
	{
		s.parse()
	}
}
