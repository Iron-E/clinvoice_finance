use core::ops::Add;

use super::Money;
use crate::Decimal;

impl Add for Money
{
	type Output = Self;

	/// # Panics
	///
	/// * If this currency and the `operand`'s currency are not the same.
	/// * When [`Decimal::add`] does.
	///
	/// # See also
	///
	/// * [`Add::add`]
	///
	/// # Examples
	///
	/// ```rust
	/// # use pretty_assertions::assert_eq;
	/// use money2::{Currency, Money};
	///
	/// assert_eq!(
	///   Money::new(10, 0, Currency::Eur) + Money::new(0_50, 2, Currency::Eur),
	///   Money::new(10_50, 2, Currency::Eur)
	/// );
	/// ```
	///
	/// ```rust,should_panic
	/// # use pretty_assertions::assert_eq;
	/// # use money2::{Currency, Money};
	/// let _ = Money::new(10, 0, Currency::Eur) + Money::new(0_50, 2, Currency::Usd);
	/// ```
	fn add(self, rhs: Self) -> Self::Output
	{
		self.unchecked(Decimal::add, rhs)
	}
}
