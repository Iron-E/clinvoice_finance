use std::io::{Cursor, Read};

use zip::ZipArchive;

use crate::Result;

/// [`GET`](reqwest::get)s the [**zipped**](ZipArchive) the zipped file at the `url` and unzip it,
/// returning the first file inside the zip.
pub async fn get_unzipped(url: &str) -> Result<String>
{
	let response = reqwest::get(url).await?;
	let bytes = response.bytes().await?;

	let mut archive = ZipArchive::new(Cursor::new(bytes))?;
	let mut file = archive.by_index(0)?;

	// NOTE: Capacity hint is fine to truncate on 32-bit platforms, it will still
	//       improve perf.
	let mut contents = String::with_capacity(file.size() as usize);
	file.read_to_string(&mut contents)?;

	Ok(contents)
}
