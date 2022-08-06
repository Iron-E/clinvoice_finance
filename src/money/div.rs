use core::ops::Div;

use super::Money;

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
	/// use money2::{Currency, Money};
	/// # use pretty_assertions::assert_eq;
	///
	/// assert_eq!(
	///   Money::new(10, 0, Currency::Eur) / Money::new(2, 0, Currency::Eur),
	///   Money::new(5, 0, Currency::Eur)
	/// );
	/// ```
	///
	/// ```rust,should_panic
	/// # use money2::{Currency, Money};
	/// # use pretty_assertions::assert_eq;
	/// let _ = Money::new(10, 0, Currency::Eur) / Money::new(2, 0, Currency::Usd);
	/// ```
	fn div(self, rhs: Self) -> Self::Output
	{
		self.unchecked(Div::div, rhs)
	}
}
