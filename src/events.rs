extern crate serde_json;

use types::{Event, ExternalEvent};

pub fn deserialize(events_json: &str) -> Vec<Event> {
    let raw_events: Vec<ExternalEvent> = serde_json::from_str(events_json).unwrap();

    raw_events
        .into_iter()
        .enumerate()
        .map(|(id, raw_event)| Event {
            id: id as u16,
            starts_at: raw_event.starts_at,
            duration: raw_event.duration,
            title: raw_event.title,
            location: raw_event.location,
        })
        .collect()
}
