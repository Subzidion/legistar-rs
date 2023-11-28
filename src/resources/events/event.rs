use std::collections::HashMap;

use serde::{de::DeserializeOwned, Deserialize};
use serde_json::Value;
use time::format_description::well_known::Iso8601;
use time::macros::format_description;
use time::{Date, PrimitiveDateTime, Time};

use super::event_item::{get_event_items_json, EventItem};

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
    pub last_modified: Option<PrimitiveDateTime>,

    #[serde(rename = "EventAgendaFile")]
    pub agenda_file: Option<String>,

    #[serde(
        rename = "EventAgendaLastPublishedUTC",
        with = "legistar_datetime_format::option"
    )]
    pub agenda_last_updated: Option<PrimitiveDateTime>,

    #[serde(rename = "EventAgendaFile")]
    pub media: Option<String>,

    #[serde(rename = "EventItems")]
    pub items: Vec<EventItem>,

    #[serde(flatten)]
    pub _extra: HashMap<String, Value>,
}

pub async fn get_events(
    client: &str,
    begin: Option<Date>,
    end: Option<Date>,
) -> Result<Vec<Event>, Box<dyn std::error::Error>> {
    let params = match (begin, end) {
        (Some(begin), Some(end)) => Some([(
            "$filter",
            format!("EventDate ge datetime'{begin}' and EventDate lt datetime'{end}'"),
        )]),
        (Some(begin), _) => Some([("$filter", format!("EventDate ge datetime'{begin}'"))]),
        (_, Some(end)) => Some([("$filter", format!("EventDate lt datetime'{end}'"))]),
        _ => None,
    };
    let url = match params {
        Some(p) => reqwest::Url::parse_with_params(
            &format!("https://webapi.legistar.com/v1/{client}/events"),
            &p,
        )?,
        None => reqwest::Url::parse(&format!("https://webapi.legistar.com/v1/{client}/events"))?,
    };
    let response = reqwest::get(url).await?.text().await?;
    Ok(deserialize::<Vec<Event>>(&response).await?)
}

async fn deserialize<T>(json: &str) -> Result<T, serde_json::Error>
where
    T: DeserializeOwned,
{
    // The entire response is returned as an Array persisted in a serde Value.
    let mut blob_value: Value = serde_json::from_str(json)?;
    let array = match blob_value.as_array_mut() {
        Some(array) => array,
        None => panic!("JSON parsed is not the expected array: {:#?}.", json),
    };
    // Iterate over each Value within the Array, extract the Map of values to mutate.
    for object in array.iter_mut() {
        let map = match object.as_object_mut() {
            Some(map) => map,
            None => panic!("Object not found, panicking."),
        };
        // Date and Time fields are split from Legistar, we merge them into a sortable DateTime field instead.
        merge_date_time(map);
        // EventItems comes unpopulated for some reason, so we query it here and populate it in the Event to be deserialized.
        merge_event_items(map).await;
    }
    println!("Finished mutating JSON Blob");
    // After mutating the
    serde_json::from_value(blob_value)
}

async fn merge_event_items(map: &mut serde_json::Map<String, Value>) {
    let event_id = map["EventId"].as_u64().unwrap();
    map["EventItems"] =
        serde_json::from_str(&get_event_items_json(event_id).await.unwrap()).unwrap();
}

fn merge_date_time(map: &mut serde_json::Map<String, Value>) {
    let date_input = map["EventDate"].as_str().unwrap();
    let mut date = PrimitiveDateTime::parse(date_input, &Iso8601::DATE_TIME).unwrap();

    let event_time = map["EventTime"].as_str().unwrap();
    let format = format_description!("[hour padding:none repr:12]:[minute] [period case:upper]");
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

#[cfg(test)]
mod tests {
    use time::format_description::well_known::Iso8601;
    use time::PrimitiveDateTime;

    use crate::resources::events::event::{deserialize, Event};

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
