#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

extern crate serde;
extern crate wasm_bindgen;

mod events;
mod matrix;
mod positions;
mod svg;
mod types;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn render_events(events_str: &str) -> String {
    // Transform JSON with events to Vec<Event>
    let events_vec = events::deserialize(events_str);

    // Build matrix with position for each event
    let matrix = matrix::build(events_vec.clone());

    // Calculate data to render events on grid
    let positions = positions::calculate(matrix);

    // Render events to SVG
    svg::render(&events_vec, &positions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_events() {
        let events = r#"[
          {"title":"Looooooooooooooooooooooooooong","starts_at":60,"duration":60},
          {"title":"1","starts_at":70,"duration":30},
          {"title":"2","starts_at":110,"duration":120},
          {"title":"3","starts_at":115,"duration":30},
          {"title":"4","starts_at":200,"duration":30},
          {"title":"5","starts_at":220,"duration":100},
          {"title":"6","starts_at":310,"duration":100}
        ]"#;

        println!("{}", render_events(events));
    }
}
