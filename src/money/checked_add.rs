use num_traits::CheckedAdd;

use super::Money;

impl CheckedAdd for Money
{
	/// # See also
	///
	/// * [`CheckedAdd::checked_add`]
	/// * [`Money::checked_add`]
	fn checked_add(&self, rhs: &Self) -> Option<Self>
	{
		Self::checked_add(*self, *rhs)
	}
}
