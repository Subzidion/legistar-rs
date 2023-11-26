use crate::resources::{json_merge_dates, Event};

pub struct LegistarClient {
    events_url: String,
}

impl LegistarClient {
    pub fn new(client: String) -> Self {
        let events_url = format!("https://webapi.legistar.com/v1/{client}/events");
        LegistarClient {
            events_url: events_url,
        }
    }

    pub async fn get_events(&self) -> Result<Vec<Event>, Box<dyn std::error::Error>> {
        let response = reqwest::get(&self.events_url).await?.text().await?;
        Ok(json_merge_dates::<Vec<Event>>(&response).await?)
    }
}