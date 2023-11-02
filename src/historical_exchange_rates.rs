use core::ops::Range;
use std::collections::BTreeMap;
use std::sync::OnceLock as StdOnceLock;

use chrono::{DateTime, Local, NaiveDate, NaiveDate, Duration, NaiveDateTime};
use futures::TryFutureExt;
use tokio::sync::{OnceCell, RwLock};

use crate::{request, Currency, Decimal, Error, ExchangeRates, Result};

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
pub struct HistoricalExchangeRates;

type HistoricalExchangeMap = BTreeMap<NaiveDate, ExchangeRates>;
type HistoricalExchangeLock = RwLock<HistoricalExchangeMap>;
static CELL: OnceCell<HistoricalExchangeLock> = OnceCell::new();

enum Header
{
	Currency(Currency),
	Date,
	Invalid,
}

enum Value
{
	Date(NaiveDate),
	Decimal(Decimal),
	Invalid,
}

/// Gets the [`Local`] time and converts it to a [`NaiveDateTime`].
fn local_now() -> NaiveDate {
	Local::now().naive_local().date()
}

impl HistoricalExchangeRates
{
	/// The single in-memory representation of the [`HistoricalExchangeMap`].
	pub async fn cached() -> Result<&'static HistoricalExchangeLock>
	{
		static LAST_CHECK: StdOnceLock<RwLock<NaiveDate>> = StdOnceLock::new();
		let cached = CELL.get_or_try_init(|| async {
			let map = Self::new().await?;
			LAST_CHECK.set(local_now().into());
			Result::Ok(RwLock::new(map))
		}).await?;

		let now = local_now();
		if LAST_CHECK.get().unwrap().read().await.signed_duration_since(now) >= Duration::days(1) {
			let mut history = cached.write().await;
			*history = Self::new().await?;
			drop(history);

			let mut last_check = LAST_CHECK.get().unwrap().write().await;
			*last_check = now;
		}

		Ok(cached)
	}

	/// Retrieve the [`ExchangeRates`] from the given `date`. Returns an [`Err`] if something went
	/// wrong retrieving the historical data, otherwise [`Ok(Some(rates))`] or [`Ok(None)`] to
	/// indicate the presence or absence of the rates in the historical record.
	pub async fn get(date: Option<DateTime<Local>>) -> Result<Option<ExchangeRates>>
	{
		let naive = date.map_or_else(local_now, |d| d.naive_local().date());
		let cached = Self::cached().await?;
		let history = cached.read().await;
		Ok(history
			.range(..=naive)
			.rev()
			.next()
			.or_else(|| history.range(naive..).next())
			.map(|(_, rates)| rates.clone()))
	}

	/// Like [`get`] but panics if it returns [`Ok(None)`] or [`Err`].
	pub async fn index(date: Option<DateTime<Local>>) -> ExchangeRates
	{
		let rates = Self::get(date).await.unwrap();
		rates.unwrap_or_else(|| {
			panic!("The history of exchange rates had no record of {}", date.unwrap_or_else(Local::now).naive_local())
		})
	}

	/// [ecb]: https://www.ecb.europa.eu/stats/policy_and_exchange_rates/euro_reference_exchange_rates/
	async fn new() -> Result<HistoricalExchangeMap>
	{
		let csv =
			request::get_unzipped("https://www.ecb.europa.eu/stats/eurofxref/eurofxref-hist.zip")
				.await?;

		let mut lines = csv.lines().map(|line| line.split(','));
		let headers = lines.next().ok_or_else(|| Error::csv_row_missing("headers"))?;
		lines.try_fold(BTreeMap::new(), |m, values| {
			headers.zip(values).filter_map(|(header, value)| match header
			{
				"Date" => value.parse::<NaiveDate>().ok().and_then(|d| {
					d.and_hms_opt(0, 0, 0).map(|dt| (Header::Date, Value::Date(dt)))
				}),
			});

			Ok(m)
		})
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
