use core::fmt::{Display, Formatter, Result};

use super::Currency;

impl Display for Currency
{
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		let as_str: &str = self.into();
		as_str.fmt(f)
	}
}
