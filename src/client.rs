use time::Date;

use crate::resources::events::{self, event::Event};

pub struct LegistarClient {
    client: String,
}

impl LegistarClient {
    pub fn new(client: String) -> Self {
        LegistarClient { client: client }
    }

    pub async fn get_events(
        &self,
        begin: Option<Date>,
        end: Option<Date>,
    ) -> Result<Vec<Event>, Box<dyn std::error::Error>> {
        events::event::get_events(&self.client, begin, end).await
    }
}
