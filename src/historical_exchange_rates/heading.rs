use crate::Currency;

/// An intermediary format for parsing a historical CSV from the ECB.
pub enum Heading
{
	/// A currency for which there is an exchange rate. Note that the exchange rate may be
	/// blank, noted with a "N/A" in the row for that date.
	Currency(Currency),

	/// The column which indicates the value for a particular set of exchange rates.
	Date,

	/// The heading was a [`Heading::Currency`], but it does not pertain to a concrete
	/// [`Currency`]. Necessary to maintain this information, as stripping it early would cause
	/// the number of headers to change w.r.t. the number of [`Value`](super::Value)s
	Invalid,
}
