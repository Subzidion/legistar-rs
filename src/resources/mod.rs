use std::collections::HashMap;

use serde::{de::DeserializeOwned, Deserialize};
use serde_json::Value;
use time::format_description::well_known::Iso8601;
use time::macros::format_description;
use time::{PrimitiveDateTime, Time};

#[derive(Deserialize, Debug)]
pub struct Event {
    #[serde(rename = "EventId")]
    pub id: u32,

    #[serde(rename = "EventBodyName")]
    pub body_name: String,

    #[serde(rename = "EventDateTime")]
    pub date_time: String,

    #[serde(flatten)]
    pub _extra: HashMap<String, Value>,
}

fn handle_datetime(j: &mut Value) {
    let array = match j.as_array_mut() {
        Some(array) => array,
        None => return,
    };

    for object in array.iter_mut() {
        let map = match object.as_object_mut() {
            Some(map) => map,
            None => return,
        };

        let mut date =
            PrimitiveDateTime::parse(map["EventDate"].as_str().unwrap(), &Iso8601::DEFAULT)
                .unwrap();
        let event_time = map["EventTime"].as_str().unwrap();
        let format =
            format_description!("[hour padding:none repr:12]:[minute] [period case:upper]");
        let new_time = Time::parse(event_time, format);
        date = date.replace_time(new_time.unwrap());

        map.insert("EventDateTime".to_owned(), date.to_string().into());
        map.remove("EventDate");
        map.remove("EventTime");
    }
}

pub async fn json_merge_dates<T>(j: &str) -> Result<T, serde_json::Error>
where
    T: DeserializeOwned,
{
    let mut v: Value = serde_json::from_str(j)?;
    handle_datetime(&mut v);
    serde_json::from_value(v)
}
