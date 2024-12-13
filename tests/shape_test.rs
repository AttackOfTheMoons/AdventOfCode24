use std::collections::HashSet;

use adventofcode24::day12::Shape;

// for _ in 0.100 is because the order that nodes are visited in is not the same every time, since it is a hashset traversal

#[test]
pub fn simple_test() {
    for i in 1..10 {
        let points = (1..(i + 1)).map(|x| (x, 3));
        let shape = Shape::new_with_points('T', HashSet::from_iter(points));
        assert_eq!(shape.sides(), 4);
    }
}

#[test]
pub fn simple_L_side_test() {
    for _ in 0..100 {
        let shape = Shape::new_with_points('T', HashSet::from([(1, 3), (2, 3), (1, 4)]));
        assert_eq!(shape.sides(), 6);
    }
}

#[test]
pub fn big_L_side_test() {
    for _ in 0..100 {
        let shape = Shape::new_with_points('T', HashSet::from([(1, 3), (2, 3), (3, 3), (1, 4)]));
        assert_eq!(shape.sides(), 6);
    }
}

#[test]
pub fn side_cube_test() {
    for _ in 0..100 {
        let shape = Shape::new_with_points('T', HashSet::from([(1, 3), (2, 3), (1, 4), (2, 4)]));
        assert_eq!(shape.sides(), 4);
    }
}
