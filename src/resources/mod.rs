use std::collections::HashMap;

use serde::{de::DeserializeOwned, Deserialize};
use serde_json::Value;
use time::format_description::well_known::Iso8601;
use time::macros::format_description;
use time::{PrimitiveDateTime, Time};

time::serde::format_description!(
    legistar_datetime_format,
    PrimitiveDateTime,
    "[year]-[month]-[day]T[hour]:[minute]:[second][optional [.[subsecond digits:1+]]]"
);

#[derive(Deserialize, Debug)]
pub struct Event {
    #[serde(rename = "EventId")]
    pub id: u32,

    #[serde(rename = "EventBodyName")]
    pub body_name: String,

    #[serde(rename = "EventDateTime", with = "legistar_datetime_format")]
    pub date_time: PrimitiveDateTime,

    #[serde(
        rename = "EventLastModifiedUtc",
        with = "legistar_datetime_format::option"
    )]
    pub event_last_modified: Option<PrimitiveDateTime>,

    #[serde(rename = "EventAgendaFile")]
    pub agenda_file: Option<String>,

    #[serde(
        rename = "EventAgendaLastPublishedUTC",
        with = "legistar_datetime_format::option"
    )]
    pub agenda_last_updated: Option<PrimitiveDateTime>,

    #[serde(rename = "EventAgendaFile")]
    pub media: Option<String>,

    #[serde(flatten)]
    pub _extra: HashMap<String, Value>,
}

fn merge_event_date_time(j: &mut Value) {
    let array = match j.as_array_mut() {
        Some(array) => array,
        None => panic!("Array not found, panicking."),
    };

    for object in array.iter_mut() {
        let map = match object.as_object_mut() {
            Some(map) => map,
            None => panic!("Object not found, panicking."),
        };

        let date_input = map["EventDate"].as_str().unwrap();
        let mut date = PrimitiveDateTime::parse(date_input, &Iso8601::DATE_TIME).unwrap();

        let event_time = map["EventTime"].as_str().unwrap();
        let format =
            format_description!("[hour padding:none repr:12]:[minute] [period case:upper]");
        let new_time = Time::parse(event_time, format).unwrap();

        date = date.replace_time(new_time);
        let date_value = match date.format(&Iso8601::DATE_TIME) {
            Ok(date) => date,
            Err(error) => panic!("Error: {:?}", error),
        };

        map.insert("EventDateTime".to_owned(), date_value.into());
        map.remove("EventDate");
        map.remove("EventTime");
    }
}

pub async fn event_deserialize<T>(j: &str) -> Result<T, serde_json::Error>
where
    T: DeserializeOwned,
{
    let mut v: Value = serde_json::from_str(j)?;
    merge_event_date_time(&mut v);
    serde_json::from_value(v)
}
