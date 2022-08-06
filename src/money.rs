mod add;
#[cfg(feature = "num-traits")]
mod checked_add;
#[cfg(feature = "num-traits")]
mod checked_div;
#[cfg(feature = "num-traits")]
mod checked_mul;
#[cfg(feature = "num-traits")]
mod checked_rem;
#[cfg(feature = "num-traits")]
mod checked_sub;
mod display;
mod div;
mod exchange;
mod from_str;
mod mul;
mod rem;
mod sub;
mod try_from;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Currency, Decimal};

/// An `amount` of [`Currency`].
///
/// To find out how much the `amount` would be in another [`Currency`], use [`exchange`](crate::Exchange::exchange).
///
/// # See also
///
/// * [`Money::new`], for how to create [`Money`] when an [amount](Decimal) does not already exist.
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Money
{
	/// The amount of [`Currency`] that this [`Money`] represents.
	pub amount: Decimal,

	/// The [`Currency`] that this [`Money`] is in.
	pub currency: Currency,
}

impl Money
{
	/// Performs a checked (i.e. the currencies are the same, otherwise returning [`None`]) `operation`
	/// on this value and the `operand`.
	fn checked(
		self,
		operation: fn(Decimal, Decimal) -> Option<Decimal>,
		operand: Self,
	) -> Option<Self>
	{
		match self.currency == operand.currency
		{
			false => None,
			_ => operation(self.amount, operand.amount).map(|amount| Self {
				amount,
				currency: self.currency,
			}),
		}
	}

	/// Returns [`Some`] if `rhs` is the same [`Currency`] and doesn't over/underflow.
	///
	/// # Examples
	///
	/// ```rust
	/// # use pretty_assertions::assert_eq;
	/// use money2::{Currency, Money};
	///
	/// let a = Money::new(20_00, 2, Currency::Usd);
	///
	/// assert_eq!(a.checked_add(Money::new(1, 0, Currency::Eur)), None);
	/// assert_eq!(
	///   a.checked_add(Money::new(5, 0, Currency::Usd)),
	///   Some(Money::new(25, 0, Currency::Usd))
	/// );
	/// ```
	pub fn checked_add(self, rhs: Self) -> Option<Self>
	{
		self.checked(Decimal::checked_add, rhs)
	}

	/// Returns [`Some`] if `rhs` is the same [`Currency`] and doesn't over/underflow.
	///
	/// # Examples
	///
	/// ```rust
	/// # use pretty_assertions::assert_eq;
	/// use money2::{Currency, Money};
	///
	/// let a = Money::new(20_00, 2, Currency::Usd);
	///
	/// assert_eq!(a.checked_div(Money::new(1, 0, Currency::Eur)), None);
	/// assert_eq!(
	///   a.checked_div(Money::new(2, 0, Currency::Usd)),
	///   Some(Money::new(10, 0, Currency::Usd))
	/// );
	/// ```
	pub fn checked_div(self, rhs: Self) -> Option<Self>
	{
		self.checked(Decimal::checked_div, rhs)
	}

	/// Returns [`Some`] if `rhs` is the same [`Currency`] and doesn't over/underflow.
	///
	/// # Examples
	///
	/// ```rust
	/// # use pretty_assertions::assert_eq;
	/// use money2::{Currency, Money};
	///
	/// let a = Money::new(20_00, 2, Currency::Usd);
	///
	/// assert_eq!(a.checked_mul(Money::new(1, 0, Currency::Eur)), None);
	/// assert_eq!(
	///   a.checked_mul(Money::new(2, 0, Currency::Usd)),
	///   Some(Money::new(40, 0, Currency::Usd))
	/// );
	/// ```
	pub fn checked_mul(self, rhs: Self) -> Option<Self>
	{
		self.checked(Decimal::checked_mul, rhs)
	}

	/// Returns [`Some`] if `rhs` is the same [`Currency`] and doesn't over/underflow.
	///
	/// # Examples
	///
	/// ```rust
	/// # use pretty_assertions::assert_eq;
	/// use money2::{Currency, Money};
	///
	/// let a = Money::new(20_00, 2, Currency::Usd);
	///
	/// assert_eq!(a.checked_rem(Money::new(1, 0, Currency::Eur)), None);
	/// assert_eq!(
	///   a.checked_rem(Money::new(3, 0, Currency::Usd)),
	///   Some(Money::new(2, 0, Currency::Usd))
	/// );
	/// ```
	pub fn checked_rem(self, rhs: Self) -> Option<Self>
	{
		self.checked(Decimal::checked_rem, rhs)
	}

	/// Returns [`Some`] if `rhs` is the same [`Currency`] and doesn't over/underflow.
	///
	/// # Examples
	///
	/// ```rust
	/// # use pretty_assertions::assert_eq;
	/// use money2::{Currency, Money};
	///
	/// let a = Money::new(20_00, 2, Currency::Usd);
	///
	/// assert_eq!(a.checked_sub(Money::new(1, 0, Currency::Eur)), None);
	/// assert_eq!(
	///   a.checked_sub(Money::new(5, 0, Currency::Usd)),
	///   Some(Money::new(15, 0, Currency::Usd))
	/// );
	/// ```
	pub fn checked_sub(self, rhs: Self) -> Option<Self>
	{
		self.checked(Decimal::checked_sub, rhs)
	}

	/// Create new [`Money`].
	///
	/// # Examples
	///
	/// ```rust
	/// use money2::{Currency, Decimal, Money};
	/// # use pretty_assertions::assert_eq;
	///
	/// let literal = "20.00 USD";
	/// let money = Money::new(20_00, 2, Currency::Usd);
	///
	/// assert_eq!(Money::try_from(literal).unwrap(), money);
	/// assert_eq!(money.to_string(), literal);
	/// ```
	pub fn new(amount: i64, decimal_places: u32, currency: Currency) -> Self
	{
		Self {
			amount: Decimal::new(amount, decimal_places),
			currency,
		}
	}

	/// Performs an unchecked (i.e. panicking) `operation` on this value and the `operand`.
	///
	/// # Panics
	///
	/// * If this currency and the `operand`'s currency are not the same.
	/// * If `operation` does.
	fn unchecked(self, operation: fn(Decimal, Decimal) -> Decimal, operand: Self) -> Self
	{
		match self.currency == operand.currency
		{
			false => panic!(
				"Attempted to perform operation on {} and {}, which have differing currencies",
				self, operand
			),

			_ => Self {
				amount: operation(self.amount, operand.amount),
				currency: self.currency,
			},
		}
	}
}
