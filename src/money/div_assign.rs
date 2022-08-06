use core::ops::{Div, DivAssign};

use super::Money;

impl DivAssign for Money
{
	/// # Panics
	///
	/// * When [`Money::div`] does.
	///
	/// # See also
	///
	/// * [`DivAssign::div_assign`]
	///
	/// # Examples
	///
	/// ```rust
	/// use money2::{Currency, Money};
	/// # use pretty_assertions::assert_eq;
	///
	/// let mut foo = Money::new(10, 0, Currency::Eur);
	/// foo /= Money::new(2, 0, Currency::Eur);
	/// assert_eq!(foo, Money::new(5, 0, Currency::Eur));
	/// ```
	///
	/// ```rust,should_panic
	/// # use money2::{Currency, Money};
	/// # use pretty_assertions::assert_eq;
	/// #
	/// # let mut foo = Money::new(10, 0, Currency::Eur);
	/// foo /= Money::new(2, 0, Currency::Usd);
	/// ```
	fn div_assign(&mut self, rhs: Self)
	{
		*self = self.div(rhs);
	}
}
