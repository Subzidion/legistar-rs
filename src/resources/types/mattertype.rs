use std::collections::HashMap;

use serde::{de::DeserializeOwned, Deserialize, Deserializer};
use serde_json::Value;
use time::PrimitiveDateTime;

time::serde::format_description!(
    legistar_datetime_format,
    PrimitiveDateTime,
    "[year]-[month]-[day]T[hour]:[minute]:[second][optional [.[subsecond digits:1+]]]"
);

#[derive(Deserialize, Debug)]
pub struct MatterType {
    #[serde(rename = "MatterTypeId")]
    pub id: u32,

    #[serde(rename = "MatterTypeName")]
    pub name: String,

    #[serde(rename = "MatterTypeDescription")]
    pub description: String,

    #[serde(
        rename = "MatterTypeLastModifiedUtc",
        with = "legistar_datetime_format"
    )]
    pub last_modified: PrimitiveDateTime,

    #[serde(rename = "MatterTypeGuid")]
    pub guid: String,

    #[serde(rename = "MatterTypeRowVersion")]
    pub row_version: String,

    #[serde(rename = "MatterTypeActiveFlag", deserialize_with = "u32_to_bool")]
    pub active: bool,

    #[serde(rename = "MatterTypeUsedFlag", deserialize_with = "u32_to_bool")]
    pub used: bool,

    #[serde(rename = "MatterTypeSort")]
    pub sort: u32,

    #[serde(flatten)]
    pub _extra: HashMap<String, Value>,
}

fn u32_to_bool<'de, D>(d: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or_else(|| 0) != 0)
}

pub async fn deserialize<T>(j: &str) -> Result<T, serde_json::Error>
where
    T: DeserializeOwned,
{
    let v: Value = serde_json::from_str(j)?;
    serde_json::from_value(v)
}
