#[derive(Debug, Copy, Clone)]
struct Event {
  id: u16,
  starts_at: u16,
  duration: u16,
}

impl Event {
  fn ends_at(self) -> u16 {
    self.starts_at + self.duration
  }

  fn overlap(self, event: &Event) -> bool {
    self.starts_at < event.ends_at() && event.starts_at < self.ends_at()
  }
}

type EventsMatrix = Vec<Vec<u16>>;

fn get_event_by_id(events: &Vec<Event>, id: u16) -> Option<&Event> {
  for event in events {
    if event.id == id {
      return Some(event);
    }
  }
  None
}

fn should_create_row(event: &Event, last_row: &Vec<u16>, events: &Vec<Event>) -> bool {
  for id in last_row {
    let cur_event = get_event_by_id(events, *id).unwrap();
    if !event.overlap(cur_event) {
      return true;
    }
  }
  false
}

fn get_two_last_rows(matrix: &mut EventsMatrix) -> (&Vec<u16>, &mut Vec<u16>) {
  let split_index = matrix.len() - 1;
  let (head, tail) = matrix.split_at_mut(split_index);
  let last_row = tail.last_mut().unwrap();
  let before_last_row = head.last().unwrap();
  (before_last_row, last_row)
}

fn build_matrix(mut events: Vec<Event>) -> EventsMatrix {
  let mut matrix = EventsMatrix::default();

  events.sort_by_key(|event| event.starts_at);

  for event in &events {
    if matrix.is_empty() {
      matrix.push(vec![event.id]);
      continue;
    }

    let mut last_row_idx = matrix.len() - 1;
    let create_row = should_create_row(event, &matrix[last_row_idx], &events);

    if create_row {
      matrix.push(Vec::<u16>::new());
      last_row_idx += 1;
    };

    if last_row_idx == 0 {
      &matrix[last_row_idx].push(event.id);
      continue;
    }

    let (prev_row, row_to_push) = get_two_last_rows(&mut matrix);
    let mut pushed = false;
    for (idx, cur_id) in prev_row.into_iter().enumerate() {
      let cur_event = get_event_by_id(&events, *cur_id).unwrap();

      if event.overlap(cur_event) {
        let row_len = row_to_push.len();
        if row_len == 0 || row_len - 1 < idx {
          row_to_push.push(cur_event.id);
        }
      } else if !pushed {
        row_to_push.push(event.id);
        pushed = true;
      }
    }
  }

  matrix
}

#[derive(Debug, PartialEq)]
struct EventPosition {
  id: u16,
  width_multiplier: u8,
  offset: u8
}
fn calculate_positions(matrix: EventsMatrix) -> Vec<EventPosition> {
  let mut positions = vec![];
  let mut calculated_events = Vec::<u16>::new();

  for row in &matrix {
    let width_multiplier = (&matrix)
      .into_iter()
      .filter_map(|cur_row| {
        let mut has_elements = false;

        for event in row {
          has_elements = cur_row.contains(event);
          if has_elements { break; }
        }

        if has_elements { Some(cur_row.len()) } else { None }
      })
      .max()
      .unwrap();

    for (offset, event) in row.into_iter().enumerate() {
      if calculated_events.contains(event) { continue; }
      positions.push(EventPosition{
        id: *event,
        width_multiplier: width_multiplier as u8,
        offset: offset as u8,
      });
      calculated_events.push(*event);
    }
  }

  positions
}

fn main() {
  // let events = vec![
  //   Event{id: 0, starts_at: 120, duration: 45},
  //   Event{id: 1, starts_at: 240, duration: 60},
  //   Event{id: 2, starts_at: 75, duration: 60},
  //   Event{id: 3, starts_at: 360, duration: 25},
  //   Event{id: 4, starts_at: 420, duration: 120},
  // ];

  // let matrix = build_matrix(events);
  // calculate_positions(matrix);
}

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[test]
fn test_calculate_positions() {
  assert_eq!(calculate_positions(vec![
    vec![0, 1,  ],
    vec![2, 1, 3],
    vec![2, 1, 4],
    vec![5,     ],
  ]), vec![
    EventPosition{id: 0, width_multiplier: 3, offset: 0},
    EventPosition{id: 1, width_multiplier: 3, offset: 1},
    EventPosition{id: 2, width_multiplier: 3, offset: 0},
    EventPosition{id: 3, width_multiplier: 3, offset: 2},
    EventPosition{id: 4, width_multiplier: 3, offset: 2},
    EventPosition{id: 5, width_multiplier: 1, offset: 0},
  ]);

  assert_eq!(calculate_positions(vec![
    vec![0,        ],
    vec![1, 2,     ],
    vec![3, 2, 4, 5],
    vec![6,        ],
  ]), vec![
    EventPosition{id: 0, width_multiplier: 1, offset: 0},
    EventPosition{id: 1, width_multiplier: 4, offset: 0},
    EventPosition{id: 2, width_multiplier: 4, offset: 1},
    EventPosition{id: 3, width_multiplier: 4, offset: 0},
    EventPosition{id: 4, width_multiplier: 4, offset: 2},
    EventPosition{id: 5, width_multiplier: 4, offset: 3},
    EventPosition{id: 6, width_multiplier: 1, offset: 0},
  ]);
}

#[test]
fn test_build_matrix() {
  assert_eq!(build_matrix(vec![
    Event{id: 0, starts_at: 120, duration: 45},
    Event{id: 1, starts_at: 240, duration: 60},
    Event{id: 2, starts_at: 75, duration: 60},
    Event{id: 3, starts_at: 360, duration: 25},
    Event{id: 4, starts_at: 420, duration: 120},
  ]), vec![
    vec![2, 0],
    vec![1,  ],
    vec![3,  ],
    vec![4,  ],
  ]);

  assert_eq!(build_matrix(vec![
    Event{id: 0, starts_at: 120, duration: 45},
    Event{id: 1, starts_at: 240, duration: 160},
    Event{id: 2, starts_at: 75, duration: 60},
    Event{id: 3, starts_at: 360, duration: 25},
    Event{id: 4, starts_at: 420, duration: 120},
    Event{id: 5, starts_at: 90, duration: 280},
  ]), vec![
    vec![2, 5, 0],
    vec![1, 5, 3],
    vec![4,     ],
  ]);

  assert_eq!(build_matrix(vec![
    Event{id: 0, starts_at: 60, duration: 60},
    Event{id: 1, starts_at: 70, duration: 30},
    Event{id: 2, starts_at: 110, duration: 120},
    Event{id: 3, starts_at: 110, duration: 30},
    Event{id: 4, starts_at: 200, duration: 30},
    Event{id: 5, starts_at: 220, duration: 100},
    Event{id: 6, starts_at: 310, duration: 100},
  ]), vec![
    vec![0, 1   ],
    vec![0, 2, 3],
    vec![4, 2, 5],
    vec![6,    5],
  ]);
}
