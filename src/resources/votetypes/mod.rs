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
pub struct VoteType {
    #[serde(rename = "VoteTypeId")]
    pub id: u32,

    #[serde(rename = "VoteTypeName")]
    pub name: String,

    #[serde(rename = "VoteTypePluralName")]
    pub plural_name: String,

    #[serde(rename = "VoteTypeLastModifiedUtc", with = "legistar_datetime_format")]
    pub last_modified: PrimitiveDateTime,

    #[serde(rename = "VoteTypeGuid")]
    pub guid: String,

    #[serde(rename = "VoteTypeRowVersion")]
    pub row_version: String,

    #[serde(rename = "VoteTypeUsedFor")]
    pub used_for: u32,

    #[serde(rename = "VoteTypeResult")]
    pub result: u32,

    #[serde(rename = "VoteTypeSort")]
    pub sort: u32,

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
