use types::{Event, EventPosition, HTMLElement};

const SVG_WIDTH: u16 = 600;
const SVG_HEIGHT: u16 = 720;
const HOURS: [&'static str; 13] = [
    "9 AM", "10 AM", "11 AM", "12 PM", "1 PM", "2 PM", "3 PM", "4 PM", "5 PM", "6 PM", "7 PM",
    "8 PM", "9 PM",
];
const EVENT_WIDTH: u16 = SVG_WIDTH - 20;
static mut CLIP_PATH_ID: u16 = 0;
lazy_static! {
    static ref HOUR_HEIGHT: f32 = SVG_HEIGHT as f32 / (HOURS.len() - 1) as f32;
}

fn create_svg_container<'a>() -> HTMLElement<'a> {
    let view_box = format!("0 0 {} {}", SVG_WIDTH, SVG_HEIGHT);
    let style = r#"
       overflow: visible;
       font-family: Helvetica, Arial;
       font-size: 0.8em;
    "#.to_string();
    HTMLElement::new("svg", &[
        ("width", SVG_WIDTH.to_string()),
        ("height", SVG_HEIGHT.to_string()),
        ("viewBox", view_box),
        ("style", style),
    ])
}

fn create_events_container<'a>() -> HTMLElement<'a> {
    HTMLElement::new("g", &[
        ("transform", "translate(10,0)".to_string())
    ])
}

fn create_grid_container<'a>() -> HTMLElement<'a> {
    let mut g = HTMLElement::new("g", &[
        ("stroke", "#f8f8f8".to_string())
    ]);

    for (i, hour) in HOURS.into_iter().enumerate() {
        g.append_child(create_hour_group(hour, (i as f32 * *HOUR_HEIGHT).to_string()));
    }

    g.append_child(create_vertical_line(SVG_WIDTH.to_string()));
    g.append_child(create_vertical_line("0".to_string()));

    g
}

fn create_hour_group<'a>(hour: &'a str, offset: String) -> HTMLElement<'a> {
    let translate = format!("translate(0,{})", offset);
    let mut g = HTMLElement::new("g", &[
        ("transform", translate),
    ]);

    let line = HTMLElement::new("line", &[("x2", SVG_WIDTH.to_string())]);
    let mut text = HTMLElement::new("text", &[
        ("text-anchor", "end".to_string()),
        ("dy", ".35em".to_string()),
        ("x", "-10".to_string()),
        ("fill", "#c9c9c9".to_string()),
        ("stroke", "none".to_string())
    ]);
    text.append_child(hour);

    g.append_child(line);
    g.append_child(text);
    g
}

fn create_vertical_line<'a>(offset: String) -> HTMLElement<'a> {
    let translate = format!("translate({},0)", offset);
    HTMLElement::new("line", &[
        ("transform", translate),
        ("y2", SVG_HEIGHT.to_string()),
    ])
}

fn create_event_title<'a>(event: &Event) -> Option<HTMLElement<'a>> {
    if event.title == None && event.location == None {
        return None;
    }

    let mut tspan = HTMLElement::new("tspan", &[
        ("style", "font-weight: bold;".to_string()),
        ("dy", "-0.1em".to_string()),
    ]);
    let mut text = String::new();
    if let Some(title) = &event.title {
        text = format!("{}", title);

        if let Some(location) = &event.location {
            text = format!("{}/{}", text, location);
        }
    } else if let Some(location) = &event.location {
        text = format!("{}", location);
    }
    tspan.append_child(text);

    Some(tspan)
}

fn format_interval(interval: u16) -> String {
    let minute = interval % 60;
    let hour = (interval - minute) / 60 + 9;
    let period = if hour < 12 { "AM" } else { "PM" };

    let mut result = (if hour > 12 {
        hour - 12
    } else {
        hour
    }).to_string();

    if minute != 0 {
        result = format!("{}:{}", result, minute);
    }

    format!("{} {}", result, period)
}

fn create_event_time<'a>(event: &Event, has_title: bool) -> HTMLElement<'a> {
    let dy = if has_title { "0.9em" } else { "0.35em" };
    let mut tspan = HTMLElement::new("tspan", &[
        ("dy", dy.to_string()),
        ("x", "0".to_string()),
    ]);
    tspan.append_child(format!(
        "{} – {}",
        format_interval(event.starts_at),
        format_interval(event.ends_at()),
    ));

    tspan
}

fn create_clip_path<'a>(event: &Event, width_multiplier: f32) -> (String, HTMLElement<'a>) {
    let width = EVENT_WIDTH as f32 * width_multiplier;
    let height = *HOUR_HEIGHT / 60.0 * event.duration as f32;
    unsafe { CLIP_PATH_ID += 1; }

    let id = format!("clip-path-{}", unsafe { CLIP_PATH_ID });
    let mut clip_path = HTMLElement::new("clipPath", &[
        ("id", id.to_owned()),
    ]);
    let rect = HTMLElement::new("rect", &[
        ("width", width.to_string()),
        ("height", height.to_string()),
    ]);
    clip_path.append_child(rect);

    (id, clip_path)
}

fn create_event_container<'a>(event: &Event, width_multiplier: f32, offset: u8, clip_path_id: String) -> HTMLElement<'a> {
    let width = EVENT_WIDTH as f32 * width_multiplier;
    let height = *HOUR_HEIGHT / 60.0 * event.duration as f32;

    let translate = format!("translate({},{})", offset as f32 * width, event.starts_at);
    let mut g = HTMLElement::new("g", &[
        ("transform", translate),
        ("clip-path", format!("url(#{})", clip_path_id)),
    ]);

    let rect = HTMLElement::new("rect", &[
        ("width", width.to_string()),
        ("height", height.to_string()),
        ("fill", "#45a51c".to_string()),
        ("opacity", "0.3".to_string()),
    ]);
    g.append_child(rect);

    let line = HTMLElement::new("line", &[
        ("stroke-width", 4.to_string()),
        ("stroke", "#45a51c".to_string()),
        ("y2", height.to_string()),
    ]);
    g.append_child(line);

    let mut text = HTMLElement::new("text", &[
        ("transform", format!("translate(10,{})", height / 2.0)),
        ("fill", "#45a51c".to_string())
    ]);

    let title_container = create_event_title(&event);
    match title_container {
        Some(title) => {
            text.append_child(title);
            text.append_child(create_event_time(&event, true));
        },
        None => {
            text.append_child(create_event_time(&event, false));
        },
    }
    g.append_child(text);

    g
}

pub fn render(events: &Vec<Event>, positions: &Vec<EventPosition>) -> String {
    let mut svg = create_svg_container();

    svg.append_child(create_grid_container());

    let mut events_container = create_events_container();
    for pos in positions {
        let event = &events[pos.id as usize];
        let clip_path = create_clip_path(&event, pos.width_multiplier);
        let event_container = create_event_container(&event, pos.width_multiplier, pos.offset, clip_path.0);
        svg.append_child(clip_path.1);
        events_container.append_child(event_container);
    }
    svg.append_child(events_container);

    svg.to_string()
}
