use super::*;
#[test]
pub fn returns_an_empty_array_when_the_target_and_source_are_in_the_same_box() {
    let source = Coordinate { x: 5., y: 5. };
    let target = Coordinate { x: 8., y: 8. };
    let box_size = 10.;
    let result = build_path(source, target, box_size, vec![]);

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].source, None);
    assert_eq!(result[0].point, Coordinate { x: 5., y: 5. })
}

#[test]
pub fn returns_a_path_of_two_items_when_the_source_and_target_are_in_adjacent_boxes() {
    let source = Coordinate { x: 15., y: 15. };
    let target = Coordinate { x: 22., y: 12. };
    let box_size = 10.;
    let result = build_path(source, target, box_size, vec![]);
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].source, None);
    assert_eq!(result[0].point, Coordinate { x: 15., y: 15. });
    assert_eq!(result[1].source, Some(Coordinate { x: 15., y: 15. }));
    assert_eq!(result[1].point, Coordinate { x: 25., y: 15. });
}

#[test]
pub fn returns_a_path_of_three_items_when_the_source_and_target_are_2_boxes_apart() {
    let source = Coordinate { x: 15., y: 15. };
    let target = Coordinate { x: 32., y: 12. };
    let box_size = 10.;
    let result = build_path(source, target, box_size, vec![]);
    assert_eq!(result.len(), 3);
    assert_eq!(result[0].source, None);
    assert_eq!(result[0].point, Coordinate { x: 15., y: 15. });
    assert_eq!(result[1].point, Coordinate { x: 25., y: 15. });
    assert_eq!(result[1].source, Some(Coordinate { x: 15., y: 15. }));
    assert_eq!(result[2].point, Coordinate { x: 35., y: 15. });
    assert_eq!(result[2].source, Some(Coordinate { x: 25., y: 15. }));
}

#[test]
pub fn finds_diagonal_path() {
    let source = Coordinate { x: 15., y: 5. };
    let target = Coordinate { x: 5., y: 15. };
    let box_size = 10.;
    let result = build_path(source, target, box_size, vec![]);
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].source, None);
    assert_eq!(result[0].point, Coordinate { x: 15., y: 5. });
    assert_eq!(result[1].source, Some(Coordinate { x: 15., y: 5. }));
    assert_eq!(result[1].point, Coordinate { x: 5., y: 15. });
}

#[test]
pub fn finds_correct_path_around_obstacles() {
    let source = Coordinate { x: 5., y: 15. };
    let target = Coordinate { x: 25., y: 15. };
    let box_size = 10.;
    let obstacles = vec![Coordinate { x: 15., y: 5. }, Coordinate { x: 15., y: 15. }];
    let result = build_path(source, target, box_size, obstacles);

    assert_eq!(result.len(), 5);
    assert_eq!(result[0].source, None);
    assert_eq!(result[0].point, Coordinate { x: 5., y: 15. });
    assert_eq!(result[1].source, Some(Coordinate { x: 5., y: 15. }));
    assert_eq!(result[1].point, Coordinate { x: 5., y: 25. });
    assert_eq!(result[2].source, Some(Coordinate { x: 5., y: 25. }));
    assert_eq!(result[2].point, Coordinate { x: 15., y: 25. });
    assert_eq!(result[3].source, Some(Coordinate { x: 15., y: 25. }));
    assert_eq!(result[3].point, Coordinate { x: 25., y: 25. });
    assert_eq!(result[4].source, Some(Coordinate { x: 25., y: 25. }));
    assert_eq!(result[4].point, Coordinate { x: 25., y: 15. });
}
