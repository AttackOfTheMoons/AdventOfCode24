use std::fs;

use log::{debug, info, trace, warn};

const INPUT_FILE: &str = "C:\\Projects\\adventofcode24\\day4.txt";

pub fn day4() {
    let file_contents = fs::read_to_string(INPUT_FILE)
        .expect(format!("Could not read file {}", INPUT_FILE).as_str());
    let line_vec: Vec<_> = file_contents
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .collect();

    debug!("line_vec:");
    for line in line_vec.clone() {
        debug!("{:?}", line);
    }
    info!("XMAS count: {}", find_x_mas(line_vec));
}

fn find_x_mas(outer_vec: Vec<Vec<char>>) -> u32 {
    let mut sum = 0;

    // Every instance of X-MAS needs an A.
    for (row_index, row) in outer_vec.clone().into_iter().enumerate() {
        for (column_index, item) in row.into_iter().enumerate() {
            if item == 'A' && valid_x_mas(row_index, column_index, &outer_vec) {
                sum += 1;
            }
        }
    }

    return sum;
}

fn valid_x_mas(start_row: usize, start_col: usize, outer_vec: &Vec<Vec<char>>) -> bool {
    trace!("valid_x_mas({}, {})", start_row, start_col);

    let south_valid = outer_vec.len() > start_row + 1;
    let north_valid = start_row as i32 - 1 >= 0;

    let inner_vec_len = match outer_vec.get(0) {
        Some(first_row) => first_row.len(),
        None => {
            warn!("outer_vec was empty");
            0
        }
    };

    let west_valid = start_col as i32 - 1 >= 0;
    let east_valid = inner_vec_len > start_col + 1;

    trace!(
        "start_row={}, start_col={}, north={}, east={}, south={}, west={}",
        start_row,
        start_col,
        north_valid,
        east_valid,
        south_valid,
        west_valid
    );

    if !north_valid || !south_valid || !east_valid || !west_valid {
        return false;
    }

    let mut found = 0;
    for (row, col) in vec![(-1, -1), (-1, 1), (1, 1), (1, -1)] {
        if unpack_char(
            (start_row as i32 + row) as usize,
            (start_col as i32 + col) as usize,
            outer_vec,
        ) == 'M'
            && unpack_char(
                (start_row as i32 + row * -1) as usize,
                (start_col as i32 + col * -1) as usize,
                outer_vec,
            ) == 'S'
        {
            found += 1;
        }
    }

    trace!("found = {}", found);
    return found == 2;
}

fn unpack_char(curr_row: usize, curr_col: usize, outer_vec: &Vec<Vec<char>>) -> char {
    let result = *outer_vec
        .get(curr_row)
        .expect(format!("ran out of bounds at row {}", curr_row).as_str())
        .get(curr_col)
        .expect(format!("ran out of bounds at col {}", curr_col).as_str());
    trace!(
        "unpack_char({}, {}, outer_vec) = {}",
        curr_row,
        curr_col,
        result
    );
    return result;
}

#[allow(dead_code)]
fn find_xmas(outer_vec: Vec<Vec<char>>) -> u32 {
    let mut sum = 0;

    // Every instance of XMAS starts with an X.
    for (row_index, row) in outer_vec.clone().into_iter().enumerate() {
        for (column_index, item) in row.into_iter().enumerate() {
            if item == 'X' {
                trace!(
                    "checking valid_xmas_cnt starting at ({}, {}) with X",
                    row_index,
                    column_index
                );
                sum += valid_xmas_cnt(row_index, column_index, &outer_vec);
            }
        }
    }

    return sum;
}

fn valid_xmas_cnt(start_row: usize, start_col: usize, outer_vec: &Vec<Vec<char>>) -> u32 {
    let mut xmas_count = 0;

    let south_valid = outer_vec.len() - start_row >= 4;
    let north_valid = start_row as i32 - 3 >= 0;

    let inner_vec_len = match outer_vec.get(0) {
        Some(first_row) => first_row.len(),
        None => {
            warn!("outer_vec was empty");
            0
        }
    };

    let west_valid = start_col as i32 - 3 >= 0;
    let east_valid = inner_vec_len - start_col >= 4;

    trace!(
        "start_row={}, start_col={}, north={}, east={}, south={}, west={}",
        start_row,
        start_col,
        north_valid,
        east_valid,
        south_valid,
        west_valid
    );

    // North West
    if north_valid
        && west_valid
        && search_unchecked_for_xmas(start_row, start_col, -1, -1, &outer_vec)
    {
        debug!("({}, {}) North West", start_row, start_col);
        xmas_count += 1;
    }

    // North
    if north_valid && search_unchecked_for_xmas(start_row, start_col, -1, 0, &outer_vec) {
        debug!("({}, {}) North", start_row, start_col);
        xmas_count += 1;
    }
    // North East
    if north_valid
        && east_valid
        && search_unchecked_for_xmas(start_row, start_col, -1, 1, &outer_vec)
    {
        debug!("({}, {}) North East", start_row, start_col);
        xmas_count += 1;
    }
    // West
    if west_valid && search_unchecked_for_xmas(start_row, start_col, 0, -1, &outer_vec) {
        debug!("({}, {}) West", start_row, start_col);
        xmas_count += 1;
    }
    // East
    if east_valid && search_unchecked_for_xmas(start_row, start_col, 0, 1, &outer_vec) {
        debug!("({}, {}) East", start_row, start_col);
        xmas_count += 1;
    }
    // South West
    if south_valid
        && west_valid
        && search_unchecked_for_xmas(start_row, start_col, 1, -1, &outer_vec)
    {
        debug!("({}, {}) South West", start_row, start_col);
        xmas_count += 1;
    }
    // South
    if south_valid && search_unchecked_for_xmas(start_row, start_col, 1, 0, &outer_vec) {
        debug!("({}, {}) South", start_row, start_col);
        xmas_count += 1;
    }
    // South East
    if south_valid
        && east_valid
        && search_unchecked_for_xmas(start_row, start_col, 1, 1, &outer_vec)
    {
        debug!("({}, {}) South East", start_row, start_col);
        xmas_count += 1;
    }

    return xmas_count;
}

fn search_unchecked_for_xmas(
    start_row: usize,
    start_col: usize,
    row_incr: i32,
    col_incr: i32,
    outer_vec: &Vec<Vec<char>>,
) -> bool {
    trace!(
        "search_unchecked_for_xmas({}, {}, {}, {}, outer_vec)",
        start_row,
        start_col,
        row_incr,
        col_incr
    );
    for (xmas_index, ch) in vec!['X', 'M', 'A', 'S'].into_iter().enumerate() {
        let curr_row = start_row as i32 + (row_incr * xmas_index as i32);
        let curr_col = start_col as i32 + (col_incr * xmas_index as i32);

        let found_ch = unpack_char(curr_row as usize, curr_col as usize, &outer_vec);

        trace!("Found {} at ({}, {})", found_ch, curr_row, curr_col);

        if found_ch != ch {
            return false;
        }
    }
    return true;
}
