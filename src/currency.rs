mod display;
mod from_str;
mod try_from;

use std::{collections::HashMap, sync::OnceLock};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{EnumCount, EnumIter, IntoEnumIterator, IntoStaticStr};
use unicase::UniCase;

/// [ISO-4217][iso] currency codes which are reported by the [European Central Bank][ecb] for
/// exchange.
///
/// [ecb]: https://www.ecb.europa.eu/stats/policy_and_exchange_rates/euro_reference_exchange_rates/
/// [iso]: https://www.iso.org/iso-4217-currency-codes.html
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize), serde(rename_all = "UPPERCASE"))]
#[derive(
	Copy,
	Clone,
	Debug,
	Default,
	EnumCount,
	EnumIter,
	Eq,
	Hash,
	IntoStaticStr,
	Ord,
	PartialEq,
	PartialOrd,
)]
#[strum(serialize_all = "UPPERCASE")]
pub enum Currency
{
	/// The Australian dollar.
	Aud,

	/// The Bulgarian lev.
	Bgn,

	/// The Brazilian real
	Brl,

	/// The Canadian dollar.
	Cad,

	/// The Swiss franc.
	Chf,

	/// The Chinese yuan.
	Cny,

	/// The Czech koruna.
	Czk,

	/// The Danish krone.
	Dkk,

	/// The Euro.
	#[default]
	Eur,

	/// The British pound.
	Gbp,

	/// The Hong Kong dollar.
	Hkd,

	/// The Hungarian forint.
	Huf,

	/// The Indonesian rupiah.
	Idr,

	/// The Israeli shekel.
	Ils,

	/// The Indian rupee.
	Inr,

	/// The Icelandic krona.
	Isk,

	/// The Japanese yen.
	Jpy,

	/// The South Korean won.
	Krw,

	/// The Mexican peso.
	Mxn,

	/// The Malaysian ringgit.
	Myr,

	/// The Norwegian krone.
	Nok,

	/// The New Zeland dollar.
	Nzd,

	/// The Philippine peso.
	Php,

	/// The Polish zloty.
	Pln,

	/// The Romanian leu.
	Ron,

	/// The Russian rouble.
	Rub,

	/// The Swedish krona.
	Sek,

	/// The Singapore dollar.
	Sgd,

	/// The Thai baht.
	Thb,

	/// The Turkish lira.
	Try,

	/// The US dollar.
	Usd,

	/// The South African rand.
	Zar,
}

impl Currency
{
	/// Attempts to convert a given string into a concrete [`Currency`], returning [`Some`] if the
	/// operation succeeds, or [`None`] if not.
	pub(crate) fn reverse_lookup(s: &str) -> Option<Currency>
	{
		static CELL: OnceLock<HashMap<UniCase<&'static str>, Currency>> = OnceLock::new();
		CELL.get_or_init(|| {
			Self::iter().map(|currency| (UniCase::new(currency.into()), currency)).collect()
		})
		.get(&s.into())
		.copied()
	}
}
