mod heading;

use std::{
	collections::{BTreeMap, HashMap},
	sync::OnceLock as StdOnceLock,
};

use chrono::{DateTime, Duration, Local, NaiveDate};
use heading::Heading;
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

/// Gets the [`Local`] time and converts it to a [`NaiveDateTime`].
fn local_now() -> NaiveDate
{
	Local::now().naive_local().date()
}

impl HistoricalExchangeRates
{
	/// The single in-memory representation of the [`HistoricalExchangeMap`].
	pub async fn cached() -> Result<&'static HistoricalExchangeLock>
	{
		static CELL: OnceCell<HistoricalExchangeLock> = OnceCell::const_new();
		static LAST_CHECK: StdOnceLock<RwLock<NaiveDate>> = StdOnceLock::new();

		let cached = CELL
			.get_or_try_init(|| async {
				let map = Self::new().await?;
				LAST_CHECK.set(local_now().into()).ok();
				Result::Ok(RwLock::new(map))
			})
			.await?;

		let now = local_now();
		if LAST_CHECK.get().unwrap().read().await.signed_duration_since(now) >= Duration::days(1)
		{
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
			panic!(
				"The history of exchange rates had no record of {}",
				date.unwrap_or_else(Local::now).naive_local()
			)
		})
	}

	/// Download the latest historical record of exchange rate data from the [ECB][ecb] and parse it
	/// into a [`HistoricalExchangeMap`].
	///
	/// [ecb]: https://www.ecb.europa.eu/stats/policy_and_exchange_rates/euro_reference_exchange_rates/
	async fn new() -> Result<HistoricalExchangeMap>
	{
		let csv =
			request::get_unzipped("https://www.ecb.europa.eu/stats/eurofxref/eurofxref-hist.zip")
				.await?;

		let mut lines = csv.lines().map(|line| line.split(','));
		let headers: Vec<_> =
			lines
				.next()
				.map(|split| {
					split
						.map(|header| match header
						{
							"Date" => Heading::Date,
							h => Currency::reverse_lookup(h)
								.map_or(Heading::Invalid, Heading::Currency),
						})
						.collect()
				})
				.ok_or_else(|| Error::csv_row_missing("headers"))?;

		Ok(lines.fold(BTreeMap::new(), |mut m, values| {
			let (date, rates) = headers.iter().zip(values).fold(
				(NaiveDate::default(), ExchangeRates(HashMap::new())),
				|(mut date, mut rates), (header, value)| {
					match header
					{
						Heading::Currency(c) =>
						{
							if let Ok(d) = value.parse::<Decimal>()
							{
								rates.0.insert(*c, d);
							}
						},
						Heading::Date =>
						{
							if let Ok(d) = value.parse::<NaiveDate>()
							{
								date = d;
							}
						},
						Heading::Invalid => (),
					};

					(date, rates)
				},
			);

			m.insert(date, rates);
			m
		}))
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
