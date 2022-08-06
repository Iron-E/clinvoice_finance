use core::ops::Add;

use super::Money;

impl Add for Money
{
	type Output = Self;

	/// # Panics
	///
	/// * If this currency and the `rhs`'s currency are not the same.
	/// * When [`Decimal::add`] does.
	///
	/// # See also
	///
	/// * [`Add::add`]
	///
	/// # Examples
	///
	/// ```rust
	/// use money2::{Currency, Money};
	/// # use pretty_assertions::assert_eq;
	///
	/// assert_eq!(
	///   Money::new(10, 0, Currency::Eur) + Money::new(0_50, 2, Currency::Eur),
	///   Money::new(10_50, 2, Currency::Eur)
	/// );
	/// ```
	///
	/// ```rust,should_panic
	/// # use money2::{Currency, Money};
	/// # use pretty_assertions::assert_eq;
	/// let _ = Money::new(10, 0, Currency::Eur) + Money::new(0_50, 2, Currency::Usd);
	/// ```
	fn add(self, rhs: Self) -> Self::Output
	{
		self.unchecked(Add::add, rhs)
	}
}
