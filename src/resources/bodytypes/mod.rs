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
pub struct BodyType {
    #[serde(rename = "BodyTypeId")]
    pub id: u32,

    #[serde(rename = "BodyTypeName")]
    pub name: String,

    #[serde(rename = "BodyTypeLastModifiedUtc", with = "legistar_datetime_format")]
    pub last_modified: PrimitiveDateTime,

    #[serde(rename = "BodyTypeGuid")]
    pub guid: String,

    #[serde(rename = "BodyTypeRowVersion")]
    pub row_version: String,

    #[serde(flatten)]
    pub _extra: HashMap<String, Value>,
}

pub async fn deserialize<T>(j: &str) -> Result<T, serde_json::Error>
where
    T: DeserializeOwned,
{
    let v: Value = serde_json::from_str(j)?;
    serde_json::from_value(v)
}
