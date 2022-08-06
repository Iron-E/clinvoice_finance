use num_traits::CheckedDiv;

use super::Money;

impl CheckedDiv for Money
{
	/// # See also
	///
	/// * [`CheckedDiv::checked_div`]
	/// * [`Money::checked_div`]
	fn checked_div(&self, rhs: &Self) -> Option<Self>
	{
		Self::checked_div(*self, *rhs)
	}
}
