mod from_str;
mod try_from;

use core::ops::Range;
use std::{
	collections::{BTreeMap, HashMap},
	env,
	fs,
	io::{Cursor, Read},
	path::{Path, PathBuf},
};

use chrono::{DateTime, Datelike, Local, NaiveDate, NaiveDateTime};
use futures::TryFutureExt;
use reqwest::Response;
use strum::EnumCount;
use zip::ZipArchive;

use crate::{Currency, Decimal, Error, Result};

/// A collection of rates of exchange between currencies such that some `amount` of
/// [`Money`](crate::Money) divided by its [`Currency`] will yield [`Currency::Eur`], and an
/// `amount` of [`Currency::Eur`] multiplied by any [`Currency`]'s exchange rate will yield that
/// [`Currency`].
///
/// # See also
///
/// * [`ExchangeRates::get`], to get the corresponding rate for some [`Currency`].
/// * [`ExchangeRates::new`], to create new [`ExchangeRates`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExchangeRates(HashMap<Currency, Decimal>);

/// [`GET`](reqwest::get)s the [**zipped**](ZipArchive) CSV at the `url`, and then caches the
/// contents at the `path` provided for easy access in other methods.
async fn get_and_cache_csv(path: &Path, url: &str) -> Result<String>
{
	let cursor = reqwest::get(url).and_then(Response::bytes).await.map(Cursor::new)?;

	let mut archive = ZipArchive::new(cursor)?;
	let mut csv = archive.by_index(0)?;

	// NOTE: Capacity hint is fine to truncate on 32-bit platforms, it will still
	//       improve perf.
	let mut csv_contents = String::with_capacity(csv.size() as usize);
	csv.read_to_string(&mut csv_contents)?;

	// cache the download for next time this method is called
	fs::write(path, &csv_contents)?;

	Ok(csv_contents)
}

impl ExchangeRates
{
	/// Return the [filepath](PathBuf) which the latest [`ExchangeRates`] should be stored at.
	///
	/// There will be a new filepath each day.
	fn filepath(date: Option<DateTime<Local>>) -> PathBuf
	{
		let day = date.unwrap_or_else(Local::now);
		env::temp_dir().join(format!("money2--{}-{}-{}.csv", day.year(), day.month(), day.day()))
	}

	/// Create a new [`ExchangeRates`] instance, which uses the [European Central Bank][ecb] to
	/// determine how to convert between currencies.
	///
	/// [ecb]: https://www.ecb.europa.eu/stats/policy_and_exchange_rates/euro_reference_exchange_rates/
	pub async fn historical(date: DateTime<Local>) -> Result<Self>
	{
		let csv = match Self::historical_filepath()
		{
			path if path.exists() &&
				fs::metadata(&path)
					.and_then(|m| m.created().map(DateTime::<Local>::from))? >=
					date =>
			{
				fs::read_to_string(&path)?
			},
			path =>
			{
				let cursor =
					reqwest::get("https://www.ecb.europa.eu/stats/eurofxref/eurofxref-hist.zip")
						.and_then(Response::bytes)
						.await
						.map(Cursor::new)?;

				let mut archive = ZipArchive::new(cursor)?;
				let mut csv = archive.by_index(0)?;

				// NOTE: Capacity hint is fine to truncate on 32-bit platforms, it will still
				//       improve perf.
				let mut csv_contents = String::with_capacity(csv.size() as usize);
				csv.read_to_string(&mut csv_contents)?;

				// cache the download for next time this method is called
				fs::write(path, &csv_contents)?;

				csv_contents
			},
		};

		enum Header
		{
			Date,
			Currency(Currency),
		}

		enum Value
		{
			Date(NaiveDateTime),
			Decimal(Decimal),
		}

		let mut lines = csv.lines().map(|line| line.split(','));
		let headers = lines.next().ok_or_else(|| Error::csv_row_missing("headers"))?;
		let map: BTreeMap<NaiveDateTime, Self> =
			lines.try_fold(BTreeMap::new(), |m, values| {
				headers.zip(values).filter_map(|(header, value)| match header
				{
					"Date" => value.parse::<NaiveDate>().ok().and_then(|d| {
						d.and_hms_opt(0, 0, 0).map(|dt| (Header::Date, Value::Date(dt)))
					}),
				});

				Ok(m)
			})?;

		match map.range(..=date.naive_local()).rev().next().or_else(|| map.range(date.naive_local()..).next())
		{
			Some(entry) => Ok(entry.1.clone()),
			None => Err(Error::Decode {
				context: "the exchange rates CSV from the ECB".into(),
				reason:  format!("there was no date close to {date}"),
			}),
		}
	}

	/// Return the [filepath](PathBuf) which the latest [`ExchangeRates`] should be stored at.
	///
	/// There will be a new filepath each day.
	fn historical_filepath() -> PathBuf
	{
		env::temp_dir().join(format!("money2--historical.csv"))
	}

	/// Retrieve a rate of exchange such that any [`Decimal`] in the `current` [`Currency`]
	/// [multiplied by](std::ops::Mul) the return value will convert it to the `desired`
	/// [`Currency`].
	///
	/// # Returns
	///
	/// * [`Some`] if this set of exchange rates accounts for both the `current` and `desired`
	///   [`Currency`].
	/// * [`None`] otherwise.
	pub fn get(&self, current: &Currency, desired: &Currency) -> Option<Decimal>
	{
		self.0.get(current).and_then(|c| self.0.get(desired).map(|d| d / c))
	}

	/// Same as [`ExchangeRates::get`], except using range syntax (i.e. `current..desired`) and
	/// panics with a custom error message instead of returning [`None`].
	///
	/// # Panics
	///
	/// * If any [`Currency`] in `range` is not present in this set of [`ExchangeRates`].
	pub fn index(&self, range: Range<&Currency>) -> Decimal
	{
		self.get(range.start, range.end).unwrap_or_else(|| {
			panic!("Either {} or {} was not found in {self:?}", range.start, range.end)
		})
	}

	/// Create a new [`ExchangeRates`] instance, which uses the [European Central Bank][ecb] to
	/// determine how to convert between currencies.
	///
	/// [ecb]: https://www.ecb.europa.eu/stats/policy_and_exchange_rates/euro_reference_exchange_rates/
	pub async fn new() -> Result<Self>
	{
		// PERF: `money2` caches ECB data until `Self::filepath(None)` changes
		match Self::filepath(None)
		{
			// TODO: use `try_exists` after rust-lang/rust#83186
			path if path.exists() => fs::read_to_string(&path)?,
			path =>
			{
				get_and_cache_csv(&path, "https://www.ecb.europa.eu/stats/eurofxref/eurofxref.zip")
					.await?
			},
		}
		.parse()
	}
}

#[cfg(test)]
mod tests
{
	use std::fs;

	use super::ExchangeRates;

	#[tokio::test]
	async fn new()
	{
		let filepath = ExchangeRates::filepath(None);
		if filepath.exists()
		{
			fs::remove_file(&filepath).unwrap();
		}

		assert!(!filepath.is_file());
		let downloaded = ExchangeRates::new().await.unwrap();
		assert!(filepath.is_file());

		let cached = ExchangeRates::new().await.unwrap();
		assert!(filepath.is_file());
		assert_eq!(downloaded, cached);
	}
}
