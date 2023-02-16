use chrono::{DateTime, Utc};
use serde::Deserialize;

mod time_format {
    use chrono::{DateTime, NaiveDateTime, NaiveTime, Utc};
    use serde::{Deserialize, Deserializer};

    const FORMAT: &str = "%I:%M:%S %p";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let time = NaiveTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        let date = Utc::now().date_naive();
        Ok(DateTime::from_utc(NaiveDateTime::new(date, time), Utc))
    }
}

#[derive(Deserialize, Debug)]
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

pub fn get_day_info(latitude: f64, longitude: f64) -> anyhow::Result<DayInfo> {
    let url = format!("https://api.sunrisesunset.io/json?lat={latitude}&lng={longitude}");
    Ok(reqwest::blocking::get(url)?.json::<ApiResponse>()?.results)
}
