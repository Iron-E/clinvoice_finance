use core::ops::Rem;

use super::Money;
use crate::Decimal;

impl Rem for Money
{
	type Output = Self;

	/// # Panics
	///
	/// * If this currency and the `operand`'s currency are not the same.
	/// * When [`Decimal::rem`] does.
	///
	/// # See also
	///
	/// * [`Rem::rem`]
	///
	/// # Examples
	///
	/// ```rust
	/// # use pretty_assertions::assert_eq;
	/// use money2::{Currency, Money};
	///
	/// assert_eq!(
	///   Money::new(10, 0, Currency::Eur) % Money::new(3, 0, Currency::Eur),
	///   Money::new(1, 0, Currency::Eur)
	/// );
	/// ```
	///
	/// ```rust,should_panic
	/// # use pretty_assertions::assert_eq;
	/// # use money2::{Currency, Money};
	/// let _ = Money::new(10, 0, Currency::Eur) % Money::new(3, 0, Currency::Usd);
	/// ```
	fn rem(self, rhs: Self) -> Self::Output
	{
		self.unchecked(Decimal::rem, rhs)
	}
}
