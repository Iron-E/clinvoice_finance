mod display;
mod exchange;
mod from_str;
mod try_from;

use rust_decimal::Decimal;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::Currency;

/// An `amount` of [`Currency`].
///
/// To find out how much the `amount` would be in another [`Currency`], use [`exchange`](crate::Exchangeable::exchange).
///
/// # See also
///
/// * [`Money::new`], for how to create [`Money`] when an [amount](Decimal) does not already exist.
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Money
{
	/// The amount of [`Currency`] that this [`Money`] represents.
	pub amount: Decimal,

	/// The [`Currency`] that this [`Money`] is in.
	pub currency: Currency,
}

impl Money
{
	/// Create new [`Money`].
	///
	/// # Examples
	///
	/// ```rust
	/// use money2::{Currency, Decimal, Money};
	/// # use pretty_assertions::assert_eq;
	///
	/// let literal = "20.00 USD";
	/// let money = Money::new(20_00, 2, Currency::Usd);
	///
	/// assert_eq!(Money::try_from(literal).unwrap(), money);
	/// assert_eq!(money.to_string(), literal);
	/// ```
	pub fn new(amount: i64, decimal_places: u32, currency: Currency) -> Self
	{
		Self {
			amount: Decimal::new(amount, decimal_places),
			currency,
		}
	}
}
