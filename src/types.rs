use std::collections::BTreeMap;
use std::fmt;
use std::fmt::Display;

pub type EventsMatrix = Vec<Vec<u16>>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EventPosition {
    pub id: u16,
    pub width_multiplier: f32,
    pub offset: u8,
}

#[derive(Serialize, Deserialize)]
pub struct ExternalEvent {
    pub starts_at: u16,
    pub duration: u16,
    pub title: Option<String>,
    pub location: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Event {
    pub id: u16,
    pub starts_at: u16,
    pub duration: u16,
    pub title: Option<String>,
    pub location: Option<String>,
}
impl Event {
    pub fn ends_at(&self) -> u16 {
        self.starts_at + self.duration
    }

    pub fn overlap(&self, event: &Event) -> bool {
        self.starts_at < event.ends_at() && event.starts_at < self.ends_at()
    }
}

pub type HTMLAttr<'a> = (&'a str, String);

pub struct HTMLElement<'a> {
    tag: &'a str,
    attrs: BTreeMap<&'a str, String>,
    body: Vec<Box<Display + 'a>>,
}
impl<'a> HTMLElement<'a> {
    pub fn new(tag: &'a str, attrs: &[HTMLAttr<'a>]) -> HTMLElement<'a> {
        let mut el = HTMLElement {
            tag: tag,
            attrs: BTreeMap::new(),
            body: vec![],
        };
        el.attrs(attrs);
        el
    }

    pub fn attrs(&mut self, attrs: &[HTMLAttr<'a>]) {
        for (attr, value) in attrs {
            self.attrs.insert(attr, value.to_owned());
        }
    }

    pub fn append_child<T: 'a>(&mut self, el: T)
        where T:Display {
        self.body.push(Box::new(el));
    }
}
impl<'a> Display for HTMLElement<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Create open tag
        let mut res = format!("<{}", self.tag);
        // Add attrs
        for (attr, value) in &self.attrs {
            res = format!("{} {}=\"{}\"", res, attr, value);
        }
        // Close open tag
        res.push_str(">");
        // Add content
        for el in &self.body {
            res.push_str(&el.to_string());
        }
        // Close tag
        res = format!("{}</{}>", res, self.tag);

        write!(f, "{}", res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_element_new() {
        let attrs = &[("height", 720.to_string())];
        let el = HTMLElement::new("svg", attrs);

        assert_eq!(el.tag, "svg");
        assert_eq!(el.body.is_empty(), true);
        assert_eq!(el.attrs.get("height").unwrap(), "720");
    }

    #[test]
    fn test_html_element_append_child() {
        let mut el = HTMLElement::new("svg", &[]);
        el.append_child(HTMLElement::new("g", &[]));
        assert_eq!(el.body.len(), 1);
    }

    #[test]
    fn test_html_element_to_string() {
        let mut el = HTMLElement::new("svg", &[("width", 600.to_string())]);
        let g = HTMLElement::new("g", &[("transform", "translate(10,0)".to_string())]);
        el.append_child(g);
        assert_eq!(
            el.to_string(),
            "<svg width=\"600\"><g transform=\"translate(10,0)\"></g></svg>"
        );
    }
}
