use num_traits::CheckedMul;

use super::Money;

impl CheckedMul for Money
{
	/// # See also
	///
	/// * [`CheckedMul::checked_mul`]
	/// * [`Money::checked_mul`]
	fn checked_mul(&self, rhs: &Self) -> Option<Self>
	{
		Self::checked_mul(*self, *rhs)
	}
}
