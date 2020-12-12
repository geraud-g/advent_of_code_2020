use crate::utils::get_file;

const EMPTY: char = 'L';
const OCCUPIED: char = '#';

const DIRECTIONS: [(i32, i32); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];


#[test]
fn test() {
    day_11()
}

pub fn day_11() {
    let input = get_input();
    let plane = Plane { seats: input };
    let solution_a = solve(&plane, get_value_part_a);
    println!("Part A: {}", solution_a);

    let solution_b = solve(&plane, get_value_part_b);
    println!("Part B: {}", solution_b);
}


fn get_input() -> Vec<Vec<char>> {
    get_file("./inputs/day_11.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}


pub fn solve(plane: &Plane, get_value: fn(&Plane, char, usize, usize, usize, usize) -> char) -> usize {
    let mut tmp_plane = plane.copy();
    let width = plane.seats[0].len();
    let height = plane.seats.len();

    loop {
        let mut new_plane = tmp_plane.copy();

        for y in 0..height {
            for x in 0..width {
                new_plane.seats[y][x] = get_value(&tmp_plane, tmp_plane.seats[y][x], y, x, width, height)
            }
        }

        if new_plane.eq(&tmp_plane) {
            return new_plane.count_occupied();
        }
        tmp_plane = new_plane.copy();
    }
}


#[derive(Eq, PartialEq)]
pub struct Plane {
    pub seats: Vec<Vec<char>>
}


impl Plane {
    fn copy(&self) -> Self {
        let new_seats = self.seats.iter()
            .map(|s| s.clone())
            .collect::<Vec<Vec<char>>>();
        Plane { seats: new_seats }
    }

    fn count_occupied(&self) -> usize {
        self.seats.iter()
            .flat_map(|s| s)
            .filter(|&s| s.eq(&OCCUPIED))
            .count()
    }
}


fn get_value_part_a(tmp_plane: &Plane, state: char, pos_y: usize, pos_x: usize, width: usize, height: usize) -> char {
    let mut occupied_seats = 0;

    for (delta_y, delta_x) in DIRECTIONS.iter() {
        let tmp_y = pos_y as i32 + delta_y;
        let tmp_x = pos_x as i32 + delta_x;
        if is_valid_index(tmp_y, tmp_x, width, height) && tmp_plane.seats[tmp_y as usize][tmp_x as usize].eq(&OCCUPIED) {
            occupied_seats += 1
        }
    }

    if state.eq(&EMPTY) && occupied_seats == 0 {
        OCCUPIED
    } else if state.eq(&OCCUPIED) && occupied_seats >= 4 {
        EMPTY
    } else {
        state
    }
}

fn get_value_part_b(tmp_plane: &Plane, state: char, y: usize, x: usize, width: usize, height: usize) -> char {
    let mut occupied_seats = 0;

    for (delta_y, delta_x) in DIRECTIONS.iter() {
        let mut tmp_y = y as i32;
        let mut tmp_x = x as i32;

        'inner: loop {
            tmp_y += delta_y;
            tmp_x += delta_x;

            if !is_valid_index(tmp_y, tmp_x, width, height) {
                break;
            }
            if tmp_plane.seats[tmp_y as usize][tmp_x as usize].eq(&OCCUPIED) {
                occupied_seats += 1;
                break 'inner;
            } else if tmp_plane.seats[tmp_y as usize][tmp_x as usize].eq(&EMPTY) {
                break 'inner;
            }
        }
    }
    match state {
        OCCUPIED => if occupied_seats >= 5 { EMPTY } else { OCCUPIED },
        EMPTY => if occupied_seats == 0 { OCCUPIED } else { EMPTY }
        _ => state
    }
}


fn is_valid_index(y: i32, x: i32, width: usize, height: usize) -> bool {
    y >= 0 && y < height as i32 && x >= 0 && x < width as i32
}
