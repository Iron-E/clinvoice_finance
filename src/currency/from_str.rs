use core::str::FromStr;

use strum::IntoEnumIterator;

use super::Currency;
use crate::{Error, Result};

impl FromStr for Currency
{
	type Err = Error;

	fn from_str(s: &str) -> Result<Self>
	{
		Self::iter()
			.find(|c| s.eq_ignore_ascii_case(c.into()))
			.ok_or_else(|| Error::UnsupportedCurrency(s.to_owned()))
	}
}
