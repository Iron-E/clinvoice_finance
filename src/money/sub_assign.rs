use core::ops::{Sub, SubAssign};

use super::Money;

impl SubAssign for Money
{
	/// # Panics
	///
	/// * When [`Money::sub`] does.
	///
	/// # See also
	///
	/// * [`SubAssign::sub_assign`]
	///
	/// # Examples
	///
	/// ```rust
	/// # use pretty_assertions::assert_eq;
	/// use money2::{Currency, Money};
	///
	/// let mut foo = Money::new(10, 0, Currency::Eur);
	/// foo -= Money::new(2, 0, Currency::Eur);
	/// assert_eq!(foo, Money::new(8, 0, Currency::Eur));
	/// ```
	///
	/// ```rust,should_panic
	/// # use pretty_assertions::assert_eq;
	/// # use money2::{Currency, Money};
	/// #
	/// # let mut foo = Money::new(10, 0, Currency::Eur);
	/// foo -= Money::new(2, 0, Currency::Usd);
	/// ```
	fn sub_assign(&mut self, rhs: Self)
	{
		*self = self.sub(rhs);
	}
}
