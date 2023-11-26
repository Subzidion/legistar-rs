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

pub async fn deserialize<T>(j: &str) -> Result<T, serde_json::Error>
where
    T: DeserializeOwned,
{
    let mut v: Value = serde_json::from_str(j)?;
    merge_date_time(&mut v);
    serde_json::from_value(v)
}

fn merge_date_time(j: &mut Value) {
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

#[cfg(test)]
mod tests {
    use time::format_description::well_known::Iso8601;
    use time::PrimitiveDateTime;

    use crate::resources::events::{deserialize, Event};

    #[tokio::test]
    async fn validate_date_time_replacement() {
        let test_string = "[{\"EventId\":5611,\"EventGuid\":\"52479834-25E0-4868-9336-C3676E97A9AC\",\"EventLastModifiedUtc\":\"2023-11-10T20:19:39.15\",\"EventRowVersion\":\"AAAAAAD80A4=\",\"EventBodyId\":198,\"EventBodyName\":\"Select Budget Committee\",\"EventDate\":\"2023-11-13T00:00:00\",\"EventTime\":\"10:00 AM\",\"EventVideoStatus\":\"Public\",\"EventAgendaStatusId\":10,\"EventAgendaStatusName\":\"Final\",\"EventMinutesStatusId\":10,\"EventMinutesStatusName\":\"Final\",\"EventLocation\":\"Council Chamber, City Hall, 600 4th Avenue, Seattle, WA 98104\",\"EventAgendaFile\":\"https://legistar2.granicus.com/seattle/meetings/2023/11/5611_A_Select_Budget_Committee_23-11-13_Committee_Agenda.pdf\",\"EventMinutesFile\":null,\"EventAgendaLastPublishedUTC\":\"2023-11-10T20:02:23.843\",\"EventMinutesLastPublishedUTC\":null,\"EventComment\":\"Session I at 10 a.m. & Session II at 2 p.m.\",\"EventVideoPath\":null,\"EventMedia\":\"https://seattlechannel.org/BudgetCommittee/?videoid=x151681\",\"EventInSiteURL\":\"https://seattle.legistar.com/MeetingDetail.aspx?LEGID=5611&GID=393&G=FFE3B678-CEF6-4197-84AC-5204EA4CFC0C\",\"EventItems\":[]}]";
        let result: Vec<Event> = deserialize(test_string).await.unwrap();
        assert_eq!(
            result[0].date_time,
            PrimitiveDateTime::parse("2023-11-13T10:00:00", &Iso8601::DATE_TIME).unwrap()
        );
        assert!(!result[0]._extra.contains_key("EventDate"));
        assert!(!result[0]._extra.contains_key("EventTime"));
    }
}
