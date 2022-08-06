use core::ops::{Add, AddAssign};

use super::Money;

impl AddAssign for Money
{
	/// # Panics
	///
	/// * When [`Money::add`] does.
	///
	/// # See also
	///
	/// * [`AddAssign::add_assign`]
	///
	/// # Examples
	///
	/// ```rust
	/// use money2::{Currency, Money};
	/// # use pretty_assertions::assert_eq;
	///
	/// let mut foo = Money::new(10, 0, Currency::Eur);
	/// foo += Money::new(0_50, 2, Currency::Eur);
	/// assert_eq!(foo, Money::new(10_50, 2, Currency::Eur));
	/// ```
	///
	/// ```rust,should_panic
	/// # use money2::{Currency, Money};
	/// # use pretty_assertions::assert_eq;
	/// #
	/// # let mut foo = Money::new(10, 0, Currency::Eur);
	/// foo += Money::new(0_50, 2, Currency::Usd);
	/// ```
	fn add_assign(&mut self, rhs: Self)
	{
		*self = self.add(rhs);
	}
}
