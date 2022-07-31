use core::str::FromStr;

use super::Money;
use crate::{Error, Result};

impl FromStr for Money
{
	type Err = Error;

	fn from_str(s: &str) -> Result<Self>
	{
		let new_error = |field: &str| -> Error {
			Error::Decode {
				context: format!(r#""{s}" into money"#),
				reason: format!("there was no {field}"),
			}
		};

		// {{{
		let mut split = s.split(' ');

		let amount = {
			let literal = split.next().ok_or_else(|| new_error("amount"))?;
			literal.parse()?
		};

		let currency = split
			.next()
			.ok_or_else(|| new_error("currency"))
			.and_then(str::parse)?;

		drop(split);
		// }}}

		Ok(Self { amount, currency })
	}
}
