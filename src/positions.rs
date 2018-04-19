use types::{EventPosition, EventsMatrix};

pub fn calculate(matrix: EventsMatrix) -> Vec<EventPosition> {
    let mut positions = vec![];
    let mut calculated_events = Vec::<u16>::new();

    for row in &matrix {
        let width_multiplier = (&matrix)
            .into_iter()
            .filter_map(|cur_row| {
                let mut has_elements = false;

                for event in row {
                    has_elements = cur_row.contains(event);
                    if has_elements {
                        break;
                    }
                }

                if has_elements {
                    Some(cur_row.len())
                } else {
                    None
                }
            })
            .max()
            .unwrap();

        for (offset, event) in row.into_iter().enumerate() {
            if calculated_events.contains(event) {
                continue;
            }
            positions.push(EventPosition {
                id: *event,
                width_multiplier: 1.0 / width_multiplier as f32,
                offset: offset as u8,
            });
            calculated_events.push(*event);
        }
    }

    positions
}

mod tests {
    use super::*;

    #[test]
    fn test_calculate() {
        assert_eq!(
            calculate(vec![vec![0, 1], vec![2, 1, 3], vec![2, 1, 4], vec![5]]),
            &[
                EventPosition {
                    id: 0,
                    width_multiplier: 1.0 / 3.0,
                    offset: 0,
                },
                EventPosition {
                    id: 1,
                    width_multiplier: 1.0 / 3.0,
                    offset: 1,
                },
                EventPosition {
                    id: 2,
                    width_multiplier: 1.0 / 3.0,
                    offset: 0,
                },
                EventPosition {
                    id: 3,
                    width_multiplier: 1.0 / 3.0,
                    offset: 2,
                },
                EventPosition {
                    id: 4,
                    width_multiplier: 1.0 / 3.0,
                    offset: 2,
                },
                EventPosition {
                    id: 5,
                    width_multiplier: 1.0,
                    offset: 0,
                },
            ]
        );

        assert_eq!(
            calculate(vec![vec![0], vec![1, 2], vec![3, 2, 4, 5], vec![6]]),
            &[
                EventPosition {
                    id: 0,
                    width_multiplier: 1.,
                    offset: 0,
                },
                EventPosition {
                    id: 1,
                    width_multiplier: 0.25,
                    offset: 0,
                },
                EventPosition {
                    id: 2,
                    width_multiplier: 0.25,
                    offset: 1,
                },
                EventPosition {
                    id: 3,
                    width_multiplier: 0.25,
                    offset: 0,
                },
                EventPosition {
                    id: 4,
                    width_multiplier: 0.25,
                    offset: 2,
                },
                EventPosition {
                    id: 5,
                    width_multiplier: 0.25,
                    offset: 3,
                },
                EventPosition {
                    id: 6,
                    width_multiplier: 1.0,
                    offset: 0,
                },
            ]
        );
    }
}
