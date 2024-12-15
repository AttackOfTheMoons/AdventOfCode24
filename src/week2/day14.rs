use colored::Colorize;
use crossbeam::channel;
use std::{
    collections::HashSet,
    fs, i32,
    thread::{self, sleep},
    time::Duration,
};

use log::{debug, info, trace};
use regex::Regex;

use crate::{Coord2D, Direction};

const INPUT_FILE: &str = "C:\\Projects\\adventofcode24\\input\\week_2\\day14.txt";

const INPUT_PATTERN: &str = r"(?m)(?R)^p=(?<p_x>\d+),(?<p_y>\d+) v=(?<v_x>-?\d+),(?<v_y>-?\d+)$";

const BOUNDS: (i64, i64) = (101, 103);
// 7, 11 == 0-6, 0-10
//(0, 1, 2), 3, (4, 5, 6)
//(0, 1, 2, 3, 4 ), 5, (6, 7, 8, 9, 10)
// Positions are 0-indexed.

#[derive(Debug, Clone)]
pub struct Robot {
    pub pos: Coord2D,
    pub velocity: Coord2D,
}

impl Robot {
    pub fn new(p_x: i64, p_y: i64, v_x: i64, v_y: i64) -> Self {
        Self {
            pos: Coord2D::new(p_x, p_y),
            velocity: Coord2D::new(v_x, v_y),
        }
    }
    pub fn move_one_second(&mut self) {
        // positions are 0 indexed, so moving from BOUNDS - 1 forward 1 will end up at 0.
        self.pos.x = (self.pos.x + self.velocity.x + BOUNDS.0) % BOUNDS.0;
        self.pos.y = (self.pos.y + self.velocity.y + BOUNDS.1) % BOUNDS.1;
    }
}

pub fn day14() {
    let file_contents =
        fs::read_to_string(INPUT_FILE).expect(format!("Could not read file {INPUT_FILE}").as_str());
    let re = Regex::new(INPUT_PATTERN).unwrap();
    let robots: Vec<_> = re
        .captures_iter(&file_contents)
        .map(|c| {
            Robot::new(
                c.name("p_x").unwrap().as_str().parse().unwrap(),
                c.name("p_y").unwrap().as_str().parse().unwrap(),
                c.name("v_x").unwrap().as_str().parse().unwrap(),
                c.name("v_y").unwrap().as_str().parse().unwrap(),
            )
        })
        .collect();
    debug!("{} robots in total", robots.len());
    part_two(robots);
}
#[derive(Debug)]
pub struct Shape {
    points: HashSet<(usize, usize)>,
}

impl Shape {
    fn new(point: (usize, usize)) -> Self {
        Shape {
            points: HashSet::from([point]),
        }
    }

    pub fn new_with_points(points: HashSet<(usize, usize)>) -> Self {
        Shape { points }
    }

    fn merge(self, other: Shape) -> Shape {
        let mut new_pts = self.points;
        new_pts.extend(other.points);
        Self::new_with_points(new_pts)
    }

    fn touches(&self, (pt_x, pt_y): (usize, usize)) -> bool {
        for dir in vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            let (dir_x, dir_y) = dir.coords();
            if pt_x == 0 && dir_x < 0 || pt_y == 0 && dir_y < 0 {
                // Invalid move.
                continue;
            }
            let result = (
                (pt_x as i32 + dir_x) as usize,
                (pt_y as i32 + dir_y) as usize,
            );
            if self.points.contains(&result) {
                return true;
            }
        }
        return false;
    }
}
fn part_two(robots: Vec<Robot>) {
    let (tx, rx) = channel::unbounded();
    let mut child_threads = Vec::new();
    for i in 0..10 {
        let mut robo_copy = robots.clone();
        let tx2 = tx.clone();

        let handle = thread::spawn(move || {
            let mut seconds = 0;
            let mut best_shape_cnt = usize::MAX;
            for _ in 0..i * 1000 {
                seconds += 1;
                move_all_robots_one_second(&mut robo_copy);
            }
            for _ in i * 1000..(i + 1) * 1000 {
                seconds += 1;
                move_all_robots_one_second(&mut robo_copy);
                let vis = visualize_robots(&robo_copy);
                let mut shapes: Vec<Shape> = Vec::new();
                for (line_idx, line) in vis.split("\n").enumerate() {
                    for (ch_idx, _ch) in line.chars().enumerate().filter(|&(_, ch)| ch != ' ') {
                        let mut new_shapes = Vec::new();
                        let mut shape_result = Shape::new((ch_idx, line_idx));
                        for shape in shapes {
                            if shape.touches((ch_idx, line_idx)) {
                                shape_result = shape_result.merge(shape);
                            } else {
                                new_shapes.push(shape);
                            }
                        }
                        new_shapes.push(shape_result);
                        shapes = new_shapes;
                    }
                }
                if shapes.len() < best_shape_cnt {
                    let _ = tx2.send((vis, seconds));
                    best_shape_cnt = shapes.len();
                }
            }
        });
        child_threads.push(handle);
    }
    for (vis, j) in rx {
        println!("{}", vis.green());
        println!("Frame #: {j}");
        sleep(Duration::from_millis(1000));
    }
    for handle in child_threads {
        let _ = handle.join();
    }
    println!("Job done!");
}

fn move_all_robots_one_second(robots: &mut Vec<Robot>) {
    for robot in robots.iter_mut() {
        robot.move_one_second();
    }
}

fn visualize_robots(robots: &Vec<Robot>) -> String {
    let mut counts =
        Vec::from_iter((0..BOUNDS.1).map(|_| Vec::from_iter((0..BOUNDS.0).map(|_| 0))));
    for &Robot {
        pos: Coord2D { x: pos_x, y: pos_y },
        velocity: _,
    } in robots
    {
        let (vec_x, vec_y) = (pos_x as usize, pos_y as usize);
        let c = counts.get_mut(vec_y).unwrap().get_mut(vec_x).unwrap();
        *c += 1;
    }
    counts
        .into_iter()
        .map(|line| {
            line.into_iter()
                .map(|c| {
                    if c == 0 {
                        " ".to_string()
                    } else {
                        c.to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join("")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[allow(dead_code)]
fn part_one(mut robots: Vec<Robot>) {
    for _ in 0..100 {
        move_all_robots_one_second(&mut robots);
    }

    let safety_score = calculate_safety_score(robots);
    info!("Safety score: {safety_score}");
}

fn calculate_safety_score(robots: Vec<Robot>) -> i64 {
    let mut top_left = 0;
    let mut top_right = 0;
    let mut bottom_left = 0;
    let mut bottom_right = 0;
    let mid = (BOUNDS.0 / 2, BOUNDS.1 / 2);
    for robot in robots {
        trace!("{robot:?}");
        if robot.pos.x < mid.0 {
            if robot.pos.y < mid.1 {
                top_left += 1;
            } else if robot.pos.y >= mid.1 + 1 {
                bottom_left += 1;
            }
        } else if robot.pos.x >= mid.0 + 1 {
            if robot.pos.y < mid.1 {
                top_right += 1;
            } else if robot.pos.y >= mid.1 + 1 {
                bottom_right += 1;
            }
        }
    }
    debug!("safety_score = {top_left} * {top_right} * {bottom_left} * {bottom_right}");
    top_left * top_right * bottom_left * bottom_right
}
