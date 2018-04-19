use types::{Event, EventsMatrix};

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

pub fn build(mut events: Vec<Event>) -> EventsMatrix {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build() {
        assert_eq!(
            build(vec![
                Event {
                    id: 0,
                    starts_at: 120,
                    duration: 45,
                    title: None,
                    location: None,
                },
                Event {
                    id: 1,
                    starts_at: 240,
                    duration: 60,
                    title: None,
                    location: None,
                },
                Event {
                    id: 2,
                    starts_at: 75,
                    duration: 60,
                    title: None,
                    location: None,
                },
                Event {
                    id: 3,
                    starts_at: 360,
                    duration: 25,
                    title: None,
                    location: None,
                },
                Event {
                    id: 4,
                    starts_at: 420,
                    duration: 120,
                    title: None,
                    location: None,
                },
            ]),
            vec![vec![2, 0], vec![1], vec![3], vec![4]]
        );

        assert_eq!(
            build(vec![
                Event {
                    id: 0,
                    starts_at: 120,
                    duration: 45,
                    title: None,
                    location: None,
                },
                Event {
                    id: 1,
                    starts_at: 240,
                    duration: 160,
                    title: None,
                    location: None,
                },
                Event {
                    id: 2,
                    starts_at: 75,
                    duration: 60,
                    title: None,
                    location: None,
                },
                Event {
                    id: 3,
                    starts_at: 360,
                    duration: 25,
                    title: None,
                    location: None,
                },
                Event {
                    id: 4,
                    starts_at: 420,
                    duration: 120,
                    title: None,
                    location: None,
                },
                Event {
                    id: 5,
                    starts_at: 90,
                    duration: 280,
                    title: None,
                    location: None,
                },
            ]),
            vec![vec![2, 5, 0], vec![1, 5, 3], vec![4]]
        );

        assert_eq!(
            build(vec![
                Event {
                    id: 0,
                    starts_at: 60,
                    duration: 60,
                    title: None,
                    location: None,
                },
                Event {
                    id: 1,
                    starts_at: 70,
                    duration: 30,
                    title: None,
                    location: None,
                },
                Event {
                    id: 2,
                    starts_at: 110,
                    duration: 120,
                    title: None,
                    location: None,
                },
                Event {
                    id: 3,
                    starts_at: 110,
                    duration: 30,
                    title: None,
                    location: None,
                },
                Event {
                    id: 4,
                    starts_at: 200,
                    duration: 30,
                    title: None,
                    location: None,
                },
                Event {
                    id: 5,
                    starts_at: 220,
                    duration: 100,
                    title: None,
                    location: None,
                },
                Event {
                    id: 6,
                    starts_at: 310,
                    duration: 100,
                    title: None,
                    location: None,
                },
            ]),
            vec![vec![0, 1], vec![0, 2, 3], vec![4, 2, 5], vec![6, 5]]
        );
    }
}
