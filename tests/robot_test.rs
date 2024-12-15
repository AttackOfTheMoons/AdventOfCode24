use adventofcode24::{week2::day14::Robot, Coord2D};

#[test]
pub fn move_straight() {
    let mut robot = Robot::new(0, 0, 1, 1);
    for _ in 0..100 {
        robot.move_one_second();
    }
    assert_eq!(robot.pos, Coord2D::new(100, 100));
}
