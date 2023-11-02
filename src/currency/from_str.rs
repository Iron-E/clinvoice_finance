use core::str::FromStr;

use super::Currency;
use crate::{Error, Result};

impl FromStr for Currency
{
	type Err = Error;

	fn from_str(s: &str) -> Result<Self>
	{
		Currency::reverse_lookup(s).ok_or_else(|| Error::UnsupportedCurrency(s.to_owned()))
	}
}
