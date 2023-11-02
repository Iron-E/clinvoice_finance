#![allow(clippy::std_instead_of_core)]

use core::{fmt::Display, result::Result as StdResult};
use std::io;

use thiserror::Error;

/// An [`Error`](std::error::Error) for the crate.
#[derive(Debug, Error)]
pub enum Error
{
	/// The error was caused while performing operations on a [`Decimal`](crate::Decimal).
	#[error(transparent)]
	Decimal(#[from] rust_decimal::Error),

	/// The error was caused while trying to decode a value in a given `context` for a specific
	/// `reason`.
	///
	/// # Examples
	///
	/// ```rust
	/// use money2::{Error::Decode, Money};
	/// assert!(matches!(Money::try_from("20.00"), Err(Decode { .. })));
	/// ```
	#[error("There was an error decoding {context}: {reason}")]
	Decode
	{
		/// What was being decoded when this error occurred.
		context: String,

		/// The reason for this error.
		reason: String,
	},

	/// The error was caused while interacting with [`io`].
	#[error(transparent)]
	Io(#[from] io::Error),

	/// The error was caused while [`reqwest`]ing exchange rates from upstream.
	#[error(transparent)]
	Reqwest(#[from] reqwest::Error),

	/// A value had no equivalent [`Currency`](crate::Currency) variant.
	///
	/// # Examples
	///
	/// ```rust
	/// use money2::{Currency, Error::UnsupportedCurrency};
	/// assert!(matches!(Currency::try_from("TMT"), Err(UnsupportedCurrency(_))));
	/// ```
	#[error("The {0} currency is not supported. See https://docs.rs/money2/latest/money2/type.Currency.html for a list of supported currencies")]
	UnsupportedCurrency(String),

	/// The error was caused while dealing with a downloaded [`zip`] file containing raw exchange
	/// rates.
	#[error(transparent)]
	Zip(#[from] zip::result::ZipError),
}

impl Error
{
	/// Returns an error which indicates that a CSV had a specific row missing which was expected to
	/// be present.
	pub(crate) fn csv_row_missing<D>(row: D) -> Self
	where
		D: Display,
	{
		Self::Decode {
			context: "the exchange rates CSV from the ECB".into(),
			reason:  format!("there was no {row} row"),
		}
	}
}

/// A [`Result`](StdResult) for the crate.
pub type Result<T> = StdResult<T, Error>;
