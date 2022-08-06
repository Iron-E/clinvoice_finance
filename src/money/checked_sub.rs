use num_traits::CheckedSub;

use super::Money;

impl CheckedSub for Money
{
	/// # See also
	///
	/// * [`CheckedSub::checked_sub`]
	/// * [`Money::checked_sub`]
	fn checked_sub(&self, rhs: &Self) -> Option<Self>
	{
		Self::checked_sub(*self, *rhs)
	}
}
