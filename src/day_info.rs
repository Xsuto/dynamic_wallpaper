use std::io::{Read, Write};
use std::path::Path;

use chrono::{DateTime, Utc};
use log::info;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};

mod time_format {
    use chrono::{DateTime, NaiveDateTime, NaiveTime, Utc};
    use log::info;
    use serde::{Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%I:%M:%S %p";

    pub fn serialize<S>(datetime: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = datetime.format(FORMAT).to_string();
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let time = NaiveTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        let date = Utc::now().date_naive();
        let thing = DateTime::from_utc(NaiveDateTime::new(date, time), Utc);
        Ok(thing)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DayInfo {
    #[serde(with = "time_format")]
    pub sunrise: DateTime<Utc>,
    #[serde(with = "time_format")]
    pub sunset: DateTime<Utc>,
    #[serde(with = "time_format")]
    pub first_light: DateTime<Utc>,
    #[serde(with = "time_format")]
    pub last_light: DateTime<Utc>,
    #[serde(with = "time_format")]
    pub dawn: DateTime<Utc>,
    #[serde(with = "time_format")]
    pub dusk: DateTime<Utc>,
    #[serde(with = "time_format")]
    pub solar_noon: DateTime<Utc>,
    #[serde(with = "time_format")]
    pub golden_hour: DateTime<Utc>,
    pub day_length: String,
    pub timezone: String,
}

#[derive(Deserialize, Debug)]
struct ApiResponse {
    results: DayInfo,
    status: String,
}

pub fn get_from_file(path: impl AsRef<Path>) -> anyhow::Result<DayInfo> {
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(serde_json::from_str::<DayInfo>(&contents)?)
}
pub fn save_to_file(path: impl AsRef<Path>, data: &DayInfo) -> anyhow::Result<()> {
    let mut file = std::fs::File::create(path)?;
    file.write_all(serde_json::to_string(&data)?.as_bytes())?;
    Ok(())
}

pub fn fetch(latitude: f64, longitude: f64) -> anyhow::Result<DayInfo> {
    let url = format!("https://api.sunrisesunset.io/json?lat={latitude}&lng={longitude}");
    info!("Sending api request {}", &url);
    Ok(reqwest::blocking::get(url)?.json::<ApiResponse>()?.results)
}
