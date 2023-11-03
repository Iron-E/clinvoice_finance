use std::{
	collections::{BTreeMap, HashMap},
	sync::OnceLock as StdOnceLock,
};

use chrono::{DateTime, Duration, Local, NaiveDate};
use tokio::sync::{OnceCell, RwLock, RwLockReadGuard};

use crate::{request, Currency, Decimal, Error, Exchange, ExchangeRates, Result};

/// A collection of rates of exchange between currencies such that some `amount` of
/// [`Money`](crate::Money) divided by its [`Currency`] will yield [`Currency::Eur`], and an
/// `amount` of [`Currency::Eur`] multiplied by any [`Currency`]'s exchange rate will yield that
/// [`Currency`].
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
	pub(crate) async fn cached() -> Result<&'static HistoricalExchangeLock>
	{
		static CELL: OnceCell<HistoricalExchangeLock> = OnceCell::const_new();
		static LAST_CHECK: StdOnceLock<RwLock<NaiveDate>> = StdOnceLock::new();

		let cached = CELL
			.get_or_try_init(|| async {
				let map = Self::from_ecb().await?;
				LAST_CHECK.set(local_now().into()).ok();
				Result::Ok(RwLock::new(map))
			})
			.await?;

		let now = local_now();
		if LAST_CHECK.get_or_init(|| local_now().into()).read().await.signed_duration_since(now) >=
			Duration::days(1)
		{
			let mut history = cached.write().await;
			*history = Self::from_ecb().await?;
			drop(history);

			let mut last_check = LAST_CHECK.get_or_init(|| local_now().into()).write().await;
			*last_check = now;
		}

		Ok(cached)
	}

	/// Like [`HistoricalExchangeRates::try_exchange`] but panics when it would return [`Err`].
	///
	/// # Panics
	///
	/// * When [`HistoricalExchangeRates::try_exchange`] would return [`Err`].
	///
	/// # See also
	///
	/// * [`HistoricalExchangeRates::exchange_from`]
	pub async fn exchange<E>(
		date: Option<DateTime<Local>>,
		currency: Currency,
		exchangeable: E,
	) -> E
	where
		E: Exchange,
	{
		Self::try_exchange(date, currency, exchangeable).await.unwrap()
	}

	/// Like [`HistoricalExchangeMap::exchange_opt_from`] but panics when it would return [`None`].
	///
	/// # Panics
	///
	/// * When [`HistoricalExchangeMap::exchange_opt_from`] would return [`None`].
	///
	/// # See also
	///
	/// * [`HistoricalExchangeRates::exchange`]
	/// * [`HistoricalExchangeRates::history`]
	/// * [`HistoricalExchangeRates::parse_csv`]
	pub fn exchange_from<E>(
		history: &HistoricalExchangeMap,
		date: Option<DateTime<Local>>,
		currency: Currency,
		exchangeable: E,
	) -> E
	where
		E: Exchange,
	{
		Self::exchange_opt_from(history, date, currency, exchangeable).unwrap()
	}

	/// Like [`HistoricalExchangeRates::try_exchange_opt`] but panics when it would return [`Err`].
	///
	/// # Panics
	///
	/// * When [`HistoricalExchangeRates::try_exchange_opt`] would return [`Err`].
	///
	/// # See also
	///
	/// * [`HistoricalExchangeRates::try_exchange`]
	pub async fn exchange_opt<E>(
		date: Option<DateTime<Local>>,
		currency: Currency,
		exchangeable: E,
	) -> Option<E>
	where
		E: Exchange,
	{
		Self::try_exchange_opt(date, currency, exchangeable).await.unwrap()
	}

	/// [`Exchange`] the `exchangeable` value into the given `currency` using the rates as they were
	/// on the given `day` in the `history`. Returns [`None`] the `date` could not be found in the
	/// source of `history`.
	///
	/// # See also
	///
	/// * [`HistoricalExchangeRates::exchange_opt`]
	/// * [`HistoricalExchangeRates::get_ref_from`] for a breakdown of how the history is searched
	///   for `date`.
	/// * [`HistoricalExchangeRates::history`]
	/// * [`HistoricalExchangeRates::parse_csv`]
	pub fn exchange_opt_from<E>(
		history: &HistoricalExchangeMap,
		date: Option<DateTime<Local>>,
		currency: Currency,
		exchangeable: E,
	) -> Option<E>
	where
		E: Exchange,
	{
		Self::get_ref_from(&history, date).map(|rates| exchangeable.exchange(currency, rates))
	}

	/// Download the latest historical record of exchange rate data from the [ECB][ecb] and parse it
	/// into a [`HistoricalExchangeMap`].
	///
	/// [ecb]: https://www.ecb.europa.eu/stats/policy_and_exchange_rates/euro_reference_exchange_rates/
	async fn from_ecb() -> Result<HistoricalExchangeMap>
	{
		let csv =
			request::get_unzipped("https://www.ecb.europa.eu/stats/eurofxref/eurofxref-hist.zip")
				.await?;

		Self::parse_csv(&csv)
	}

	/// Like [`get_from`], but uses an automatically-managed source of historical data from the ECB.
	///
	/// Returns an [`Err`] if something went wrong retrieving the historical
	/// data, otherwise [`Ok(Some(rates))`] or [`Ok(None)`] to indicate the presence or absence of
	/// the rates in the historical record.
	///
	/// # See also
	///
	/// * [`HistoricalExchangeRates::get_from`]
	pub async fn get(date: Option<DateTime<Local>>) -> Result<Option<ExchangeRates>>
	{
		let history = Self::history().await?;
		Ok(Self::get_from(&history, date))
	}

	/// Like [`HistoricalExchangeRates::get_ref_from`] but return and owned value.
	///
	/// # See also
	///
	/// * [`HistoricalExchangeRates::get`]
	/// * [`HistoricalExchangeRates::history`]
	/// * [`HistoricalExchangeRates::parse_csv`]
	pub fn get_from(
		history: &HistoricalExchangeMap,
		date: Option<DateTime<Local>>,
	) -> Option<ExchangeRates>
	{
		Self::get_ref_from(history, date).cloned()
	}

	/// Retrieve the [`ExchangeRates`] from the given `date` (or the nearest-available date;
	/// today if [`None`]). Returns [`Some(rates)`] or [`None`] to indicate the presence or absence
	/// of the rates in the historical record.
	///
	/// # See also
	///
	/// * [`HistoricalExchangeRates::history`]
	/// * [`HistoricalExchangeRates::parse_csv`]
	pub fn get_ref_from(
		history: &HistoricalExchangeMap,
		date: Option<DateTime<Local>>,
	) -> Option<&ExchangeRates>
	{
		let naive = date.map_or_else(local_now, |d| d.naive_local().date());
		history
			.range(..=naive)
			.next_back()
			.or_else(|| history.range(naive..).next())
			.map(|(_, rates)| rates)
	}

	/// Obtain a read-only copy of the automatically-managed exchange rate history. Useful for
	/// pulling asynchrony out from a loop, and then passing the value manually to
	/// [`ExchangeRates::get_ref_from`].
	///
	/// # Warnings
	///
	/// * While the return value is in scope, the [`HistoricalExchangeRates`] cannot update itself!
	///   **This may cause other operations to lock until this value is released**.
	pub async fn history() -> Result<RwLockReadGuard<'static, HistoricalExchangeMap>>
	{
		let cached = Self::cached().await?;
		Ok(cached.read().await)
	}

	/// Like [`HistoricalExchangeRates::try_index`] but panics if it returns [`Err`].
	///
	/// # Panics
	///
	/// * When [`HistoricalExchangeRates::try_index`] returns [`Err`].
	///
	/// # See also
	///
	/// * [`HistoricalExchangeRates::index_from`]
	pub async fn index(date: Option<DateTime<Local>>) -> ExchangeRates
	{
		Self::try_index(date).await.unwrap()
	}

	/// Like [`HistoricalExchangeRates::get_from`] but panics if it returns [`None`].
	///
	/// # Panics
	///
	/// * When [`HistoricalExchangeRates::get_from`] return [`None`].
	///
	/// # See also
	///
	/// * [`HistoricalExchangeRates::history`]
	/// * [`HistoricalExchangeRates::parse_csv`]
	pub async fn index_from(
		history: &HistoricalExchangeMap,
		date: Option<DateTime<Local>>,
	) -> ExchangeRates
	{
		Self::get_from(history, date).unwrap()
	}

	/// Parse a CSV of the form:
	///
	/// ```csv
	/// Date,USA,JPY,…
	/// 2022-02-28,0.813,89.1,…
	/// …
	/// ```
	///
	/// Returns [`Ok(map)`] if the CSV was successfully parsed, otherwise returns [`Err`].
	///
	/// # Additional Details
	///
	/// Normally, the [`HistoricalExchangeRates`] will manage an internal [`HistoricalExchangeMap`]
	/// and update it periodically to keep it up-to-date as long as the program using this
	/// feature-set runs.
	///
	/// However, if there is a need to manually parse this data, the option is available.
	pub fn parse_csv(csv: &str) -> Result<HistoricalExchangeMap>
	{
		let mut lines = csv.lines().map(|line| line.split(','));
		let headers: Vec<_> = lines
			.next()
			.map(|split| split.skip(1).map(Currency::reverse_lookup).collect())
			.ok_or_else(|| Error::csv_row_missing("headers"))?;

		Ok(lines.fold(BTreeMap::new(), |mut m, mut values| {
			let date = values.next().and_then(|d| d.parse::<NaiveDate>().ok()).unwrap_or_default();

			let mut rates = headers.iter().zip(values).fold(
				ExchangeRates(HashMap::new()),
				|mut rates, (header, value)| {
					// TODO: if-let chain
					if let Some(c) = header
					{
						if let Ok(d) = value.parse::<Decimal>()
						{
							rates.0.insert(*c, d);
						}
					}

					rates
				},
			);

			// NOTE: conversion to EUR is not stored in ECB exchange rates, since the rates are
			// given in       context of EUR to some other currency.
			rates.0.insert(Currency::Eur, 1.into());
			m.insert(date, rates);
			m
		}))
	}

	/// Like [`HistoricalExchangeRates::try_exchange_opt`] but panics when it would return
	/// [`Ok(None)`].
	///
	/// # Panics
	///
	/// * When [`HistoricalExchangeRates::try_exchange_opt`] would return [`Ok(None)`].
	pub async fn try_exchange<E>(
		date: Option<DateTime<Local>>,
		currency: Currency,
		exchangeable: E,
	) -> Result<E>
	where
		E: Exchange,
	{
		Self::try_exchange_opt(date, currency, exchangeable).await.map(Option::unwrap)
	}

	/// Like [`HistoricalExchangeRates::exchange_from`], but attempt to use the internally-managed
	/// source of `history`. Will only return [`Err`] when this internal management fails.
	/// Otherwise, [`Ok(Some)`] or [`Ok(None)`] is returned depending on whether `date` can be found
	/// in the record.
	///
	/// # See also
	///
	/// * [`HistoricalExchangeRates::exchange_opt`]
	/// * [`HistoricalExchangeRates::get_ref_from`] for a breakdown of how the history is searched
	///   for `date`.
	pub async fn try_exchange_opt<E>(
		date: Option<DateTime<Local>>,
		currency: Currency,
		exchangeable: E,
	) -> Result<Option<E>>
	where
		E: Exchange,
	{
		let history = Self::history().await?;
		Ok(Self::exchange_opt_from(&history, date, currency, exchangeable))
	}

	/// Like [`HistoricalExchangeRates::get`] but panics if it returns [`Ok(None)`].
	///
	/// # Panics
	///
	/// * When [`HistoricalExchangeRates::get`] return [`Ok(None)`].
	pub async fn try_index(date: Option<DateTime<Local>>) -> Result<ExchangeRates>
	{
		Self::get(date)
			.await
			.map(|rates| rates.expect("The internal historical record has no data"))
	}
}

#[cfg(test)]
mod tests
{
	use pretty_assertions::assert_eq;

	use super::{
		Currency,
		Decimal,
		ExchangeRates,
		HistoricalExchangeRates,
		Local,
		NaiveDate,
		Result,
	};
	use crate::Money;

	#[tokio::test]
	async fn cached() -> Result<()>
	{
		let lock = HistoricalExchangeRates::cached().await?;
		let history = lock.read().await;

		let (date, rates) = history.first_key_value().unwrap();
		assert_eq!(date, &NaiveDate::from_ymd_opt(1999, 01, 04).unwrap());
		assert_eq!(
			rates,
			&ExchangeRates(
				[
					(Currency::Aud, Decimal::new(1_91, 2)),
					(Currency::Cad, Decimal::new(1_8004, 4)),
					(Currency::Chf, Decimal::new(1_6168, 4)),
					(Currency::Czk, Decimal::new(35_107, 3)),
					(Currency::Dkk, Decimal::new(7_4501, 4)),
					(Currency::Eur, 1.into()),
					(Currency::Gbp, Decimal::new(0_7111, 4)),
					(Currency::Hkd, Decimal::new(9_1332, 4)),
					(Currency::Huf, Decimal::new(251_48, 2)),
					(Currency::Isk, Decimal::new(81_48, 2)),
					(Currency::Jpy, Decimal::new(133_73, 2)),
					(Currency::Krw, Decimal::new(1398_59, 2)),
					(Currency::Nok, Decimal::new(8_855, 3)),
					(Currency::Nzd, Decimal::new(2_2229, 4)),
					(Currency::Pln, Decimal::new(4_0712, 4)),
					(Currency::Sek, Decimal::new(9_4696, 4)),
					(Currency::Sgd, Decimal::new(1_9554, 4)),
					(Currency::Usd, Decimal::new(1_1789, 4)),
					(Currency::Zar, Decimal::new(6_9358, 4)),
				]
				.into_iter()
				.collect()
			)
		);

		Ok(())
	}

	#[tokio::test]
	async fn get() -> Result<()>
	{
		let mut after =
			HistoricalExchangeRates::get(NaiveDate::from_ymd_opt(1999, 01, 04).and_then(|d| {
				d.and_hms_opt(0, 0, 0).and_then(|dt| dt.and_local_timezone(Local).earliest())
			}))
			.await?;

		let mut before =
			HistoricalExchangeRates::get(NaiveDate::from_ymd_opt(1998, 01, 01).and_then(|d| {
				d.and_hms_opt(0, 0, 0).and_then(|dt| dt.and_local_timezone(Local).earliest())
			}))
			.await?;

		assert!(after.is_some());
		assert_eq!(after, before);

		after = HistoricalExchangeRates::get(NaiveDate::from_ymd_opt(2012, 05, 05).and_then(|d| {
			d.and_hms_opt(0, 0, 0).and_then(|dt| dt.and_local_timezone(Local).earliest())
		}))
		.await?;

		before =
			HistoricalExchangeRates::get(NaiveDate::from_ymd_opt(2012, 05, 04).and_then(|d| {
				d.and_hms_opt(0, 0, 0).and_then(|dt| dt.and_local_timezone(Local).earliest())
			}))
			.await?;

		assert!(after.is_some());
		assert_eq!(after, before);

		Ok(())
	}

	#[tokio::test]
	async fn exchange() -> Result<()>
	{
		let value = HistoricalExchangeRates::exchange(
			None,
			Default::default(),
			Money::new(20_00, 2, Currency::Usd),
		)
		.await;

		assert_eq!(value, Money::new(18_69, 2, Default::default()));
		Ok(())
	}
}
