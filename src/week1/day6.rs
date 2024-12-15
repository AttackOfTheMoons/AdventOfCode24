use indicatif::ProgressBar;
use std::{collections::HashSet, fs, thread};

use log::{info, trace};

use crate::Direction;

const INPUT_FILE: &str = "C:\\Projects\\adventofcode24\\input\\week_1\\day6.txt";

pub fn day6() {
    let file_contents =
        fs::read_to_string(INPUT_FILE).expect(format!("Could not read file {INPUT_FILE}").as_str());

    let line_vec: Vec<_> = file_contents
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .collect();

    let start_pos = line_vec
        .iter()
        .enumerate()
        .map(|(index, line)| (index, line.iter().position(|c| *c == '^')))
        .filter(|(_outer_index, inner_result)| inner_result.is_some())
        .next();

    let start_x: i32;
    let start_y: i32;

    match start_pos {
        Some((outer_index, inner_result)) => match inner_result {
            Some(inner_index) => {
                start_x = inner_index.try_into().unwrap();
                start_y = outer_index.try_into().unwrap();
            }
            None => panic!("Couldn't find start_x"),
        },
        None => panic!("Couldn't find start_y"),
    }

    let grid_size_x = line_vec.len();
    let grid_size_y = line_vec.iter().next().unwrap().len();

    part1(start_x, start_y, grid_size_x, grid_size_y, &line_vec);
    part2(start_x, start_y, grid_size_x, grid_size_y, line_vec);
}

fn part2(
    start_x: i32,
    start_y: i32,
    grid_size_x: usize,
    grid_size_y: usize,
    line_vec: Vec<Vec<char>>,
) {
    let mut sum = 0;

    info!("Trying obstacles...");
    let bar = ProgressBar::new(((grid_size_x as i32) * (grid_size_y as i32)) as u64);
    let mut children = vec![];

    for grid_y in 0..grid_size_y {
        for grid_x in 0..grid_size_x {
            bar.inc(1);

            let start_vec = line_vec.clone();

            let handle = thread::spawn(move || {
                part2_main_loop(
                    start_x,
                    start_y,
                    start_vec,
                    grid_x,
                    grid_y,
                    grid_size_x,
                    grid_size_y,
                )
            });
            children.push(handle);
        }
    }

    for thread in children {
        let result = thread.join();
        if result.unwrap() {
            sum += 1
        }
    }

    info!("{sum} total obstructions result in loops");
}

fn part2_main_loop(
    start_x: i32,
    start_y: i32,
    mut start_vec: Vec<Vec<char>>,
    check_x: usize,
    check_y: usize,
    grid_size_x: usize,
    grid_size_y: usize,
) -> bool {
    let mut guard = Guard::new(start_x, start_y);

    let mut positions: HashSet<_> = HashSet::new();
    positions.insert((start_x, start_y, Direction::Up));

    unsafe {
        *start_vec
            .get_unchecked_mut(check_x)
            .get_unchecked_mut(check_y) = '#';
    }
    loop {
        if !guard.walk_or_turn((grid_size_x, grid_size_y), &start_vec) {
            trace!("Walked off");
            return false;
        }

        if positions.contains(&(guard.pos_x, guard.pos_y, guard.facing)) {
            return true;
        }

        positions.insert((guard.pos_x, guard.pos_y, guard.facing));
    }
}

#[allow(dead_code)]
fn part1(
    start_x: i32,
    start_y: i32,
    grid_size_x: usize,
    grid_size_y: usize,
    line_vec: &Vec<Vec<char>>,
) {
    let mut guard = Guard::new(start_x, start_y);

    // Part 1:
    let mut positions: HashSet<_> = HashSet::new();
    positions.insert((start_x, start_y));

    'walk_loop: loop {
        if !guard.walk_or_turn((grid_size_x, grid_size_y), &line_vec) {
            break 'walk_loop;
        }
        // Part 1:
        positions.insert((guard.pos_x, guard.pos_y));
    }

    info!("Loop was {} steps", positions.len());
}

struct Guard {
    pos_x: i32,
    pos_y: i32,
    facing: Direction,
}

impl Guard {
    fn new(pos_x: i32, pos_y: i32) -> Self {
        Self {
            pos_x,
            pos_y,
            facing: Direction::Up,
        }
    }

    fn turn(&mut self) {
        match &self.facing {
            Direction::Up => self.facing = Direction::Right,
            Direction::Down => self.facing = Direction::Left,
            Direction::Left => self.facing = Direction::Up,
            Direction::Right => self.facing = Direction::Down,
        };
        trace!("Guard turned {:?}", self.facing);
    }

    // returns true if walked or turned, false if walked out of bounds
    fn walk_or_turn(
        &mut self,
        (grid_size_x, grid_size_y): (usize, usize),
        outer_vec: &Vec<Vec<char>>,
    ) -> bool {
        let (move_x, move_y) = self.facing.coords();
        let (res_x, res_y) = (self.pos_x + move_x, self.pos_y + move_y);

        if res_x < 0
            || res_x >= grid_size_x.try_into().unwrap()
            || res_y < 0
            || res_y >= grid_size_y.try_into().unwrap()
        {
            return false;
        }

        unsafe {
            if *outer_vec
                .get_unchecked(res_y as usize)
                .get_unchecked(res_x as usize)
                == '#'
            {
                self.turn();
                return true;
            }
        }

        trace!("Guard walked {:?}", self.facing);
        self.pos_x = res_x;
        self.pos_y = res_y;
        return true;
    }
}
