use std::collections::HashMap;

use serde::{de::DeserializeOwned, Deserialize};
use serde_json::Value;
use time::PrimitiveDateTime;

time::serde::format_description!(
    legistar_datetime_format,
    PrimitiveDateTime,
    "[year]-[month]-[day]T[hour]:[minute]:[second][optional [.[subsecond digits:1+]]]"
);

#[derive(Deserialize, Debug)]
pub struct EventItem {
    #[serde(rename = "EventItemId")]
    pub id: u32,

    #[serde(rename = "EventItemTitle")]
    pub body_name: Option<String>,

    #[serde(
        rename = "EventItemLastModifiedUtc",
        with = "legistar_datetime_format::option"
    )]
    pub last_modified: Option<PrimitiveDateTime>,

    #[serde(flatten)]
    pub _extra: HashMap<String, Value>,
}

pub async fn deserialize<T>(json: &str) -> Result<T, serde_json::Error>
where
    T: DeserializeOwned,
{
    let v: Value = serde_json::from_str(json)?;
    serde_json::from_value(v)
}

pub(super) async fn get_event_items_json(
    event_id: u64,
) -> Result<String, Box<dyn std::error::Error>> {
    let params = [
        ("AgendaNote", "1"),
        ("MinutesNote", "1"),
        ("Attachments", "1"),
    ];
    let url = reqwest::Url::parse_with_params(
        &format!("https://webapi.legistar.com/v1/seattle/events/{event_id}/EventItems"),
        &params,
    )?;
    Ok(reqwest::get(url).await?.text().await?)
}
