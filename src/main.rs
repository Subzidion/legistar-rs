mod client;
mod resources;

use client::LegistarClient;

#[tokio::main]
async fn main() {
    let client = LegistarClient::new(String::from("seattle"));
    let response = client.get_events().await;
    let events = match response {
        Ok(events) => events,
        Err(error) => panic!("Error: {:?}", error),
    };

    for event in events.iter() {
        println!(
            "Event {:#?} hosted by {:#?} at {:#?}",
            event.id, event.body_name, event.date_time
        );
    }
}
