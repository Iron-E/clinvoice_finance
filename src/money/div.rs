use core::ops::Div;

use super::Money;
use crate::Decimal;

impl Div for Money
{
	type Output = Self;

	/// # Panics
	///
	/// * If this currency and the `operand`'s currency are not the same.
	/// * When [`Decimal::div`] does.
	///
	/// # See also
	///
	/// * [`Div::div`]
	///
	/// # Examples
	///
	/// ```rust
	/// # use pretty_assertions::assert_eq;
	/// use money2::{Currency, Money};
	///
	/// assert_eq!(
	///   Money::new(10, 0, Currency::Eur) / Money::new(2, 0, Currency::Eur),
	///   Money::new(5, 0, Currency::Eur)
	/// );
	/// ```
	///
	/// ```rust,should_panic
	/// # use pretty_assertions::assert_eq;
	/// # use money2::{Currency, Money};
	/// let _ = Money::new(10, 0, Currency::Eur) / Money::new(2, 0, Currency::Usd);
	/// ```
	fn div(self, rhs: Self) -> Self::Output
	{
		self.unchecked(Decimal::div, rhs)
	}
}
