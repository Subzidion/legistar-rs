use time::Date;

use crate::resources::bodytypes::{self, BodyType};
use crate::resources::events::{self, Event};
pub struct LegistarClient {
    events_url: String,
    body_types_url: String,
}

impl LegistarClient {
    pub fn new(client: String) -> Self {
        let events_url = format!("https://webapi.legistar.com/v1/{client}/events");
        let body_types_url = format!("https://webapi.legistar.com/v1/{client}/bodytypes");
        LegistarClient {
            events_url: events_url,
            body_types_url: body_types_url,
        }
    }

    pub async fn get_events(
        &self,
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
            Some(p) => reqwest::Url::parse_with_params(&self.events_url, &p)?,
            None => reqwest::Url::parse(&self.events_url)?,
        };
        let response = reqwest::get(url).await?.text().await?;
        Ok(events::deserialize::<Vec<Event>>(&response).await?)
    }

    pub async fn get_body_types(&self) -> Result<Vec<BodyType>, Box<dyn std::error::Error>> {
        let url = reqwest::Url::parse(&self.body_types_url)?;
        let response = reqwest::get(url).await?.text().await?;
        Ok(bodytypes::deserialize::<Vec<BodyType>>(&response).await?)
    }
}
