use core::ops::{Rem, RemAssign};

use super::Money;

impl RemAssign for Money
{
	/// # Panics
	///
	/// * When [`Money::rem`] does.
	///
	/// # See also
	///
	/// * [`RemAssign::rem_assign`]
	///
	/// # Examples
	///
	/// ```rust
	/// use money2::{Currency, Money};
	/// # use pretty_assertions::assert_eq;
	///
	/// let mut foo = Money::new(10, 0, Currency::Eur);
	/// foo %= Money::new(3, 0, Currency::Eur);
	/// assert_eq!(foo, Money::new(1, 0, Currency::Eur));
	/// ```
	///
	/// ```rust,should_panic
	/// # use money2::{Currency, Money};
	/// # use pretty_assertions::assert_eq;
	/// #
	/// # let mut foo = Money::new(10, 0, Currency::Eur);
	/// foo %= Money::new(3, 0, Currency::Usd);
	/// ```
	fn rem_assign(&mut self, rhs: Self)
	{
		*self = self.rem(rhs);
	}
}
