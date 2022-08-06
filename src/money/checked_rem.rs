use num_traits::CheckedRem;

use super::Money;

impl CheckedRem for Money
{
	/// # See also
	///
	/// * [`CheckedRem::checked_rem`]
	/// * [`Money::checked_rem`]
	fn checked_rem(&self, rhs: &Self) -> Option<Self>
	{
		Self::checked_rem(*self, *rhs)
	}
}
