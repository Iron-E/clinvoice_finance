use core::ops::{Mul, MulAssign};

use super::Money;

impl MulAssign for Money
{
	/// # Panics
	///
	/// * When [`Money::mul`] does.
	///
	/// # See also
	///
	/// * [`MulAssign::mul_assign`]
	///
	/// # Examples
	///
	/// ```rust
	/// # use pretty_assertions::assert_eq;
	/// use money2::{Currency, Money};
	///
	/// let mut foo = Money::new(10, 0, Currency::Eur);
	/// foo *= Money::new(2, 0, Currency::Eur);
	/// assert_eq!(foo, Money::new(20, 0, Currency::Eur));
	/// ```
	///
	/// ```rust,should_panic
	/// # use pretty_assertions::assert_eq;
	/// # use money2::{Currency, Money};
	/// #
	/// # let mut foo = Money::new(10, 0, Currency::Eur);
	/// foo *= Money::new(2, 0, Currency::Usd);
	/// ```
	fn mul_assign(&mut self, rhs: Self)
	{
		*self = self.mul(rhs);
	}
}
