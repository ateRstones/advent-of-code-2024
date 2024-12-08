use std::fs;
use std::collections;

struct Connections();

impl Connections {
    const HORIZONTAL_XMAS: i32 = 0b1 << 0;
    const VERT_RIGHT_XMAS: i32 = 0b1 << 1;
    const VERT_LEFT_XMAS: i32 = 0b1 << 2;
    const HORIZONTAL_SAMX: i32 = 0b1 << 3;
    const VERT_RIGHT_SAMX: i32 = 0b1 << 4;
    const VERT_LEFT_SAMX: i32 = 0b1 << 5;
    const SAMX: i32 = Connections::HORIZONTAL_SAMX | Connections::VERT_RIGHT_SAMX | Connections::VERT_LEFT_SAMX;
    const XMAS: i32 = Connections::HORIZONTAL_XMAS | Connections::VERT_RIGHT_XMAS | Connections::VERT_LEFT_XMAS;
}

fn index_map_safe<T>(map: &Vec<Vec<T>>, x: i32, y: i32) -> Option<&T> {
    if (map.len() as i32) <= x || x < 0 {
        return None;
    }

    let line = &map[x as usize];
    
    if (line.len() as i32) <= y || y < 0 {
        return None;
    }

    return Some(&line[y as usize]);
}

fn get_precondition(letter: &char) -> Vec<(Option<char>, i32)> {
    let mut precond: Vec<(Option<char>, i32)> = Vec::new();

    match letter {
        'X'=>{precond.push((Some('M'), Connections::SAMX));}
        'M'=>{precond.push((Some('X'), Connections::XMAS));precond.push((Some('A'), Connections::SAMX));}
        'A'=>{precond.push((Some('M'), Connections::XMAS));precond.push((Some('S'), Connections::SAMX));}
        'S'=>{precond.push((Some('A'), Connections::XMAS));}
        _=>{}
    }

    return precond;
}

fn compute_continuation(xmas_map: &Vec<Vec<char>>, mut connection_map: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    return;
}

fn run_for_input(contents: &String) {
    let xmas_map: Vec<Vec<char>> = contents.lines().map(|x | x.chars().collect()).collect();

    let mut connection_map: Vec<Vec<i32>> = xmas_map.clone().into_iter().map(|x| vec![0; x.len()]).collect();

    let mut xmas_counter = 0;

    for x in 0..xmas_map.len() {
        let line = &xmas_map[x];
        for y in 0..line.len() {
            let xc = x as i32;
            let yc = y as i32;
            let self_letter = index_map_safe(&xmas_map, xc, yc);
            let horizontal_letter = index_map_safe(&xmas_map, xc+1, yc);
            let vert_left_letter = index_map_safe(&xmas_map, xc+1, yc-1);
            let vert_right_letter = index_map_safe(&xmas_map, xc+1, yc+1);

            let self_state = index_map_safe(&connection_map, xc, yc);

            if self_state.is_some() {
                if (self_letter == Some(&'X') && (self_state.unwrap() & Connections::SAMX) > 0)
                    || (self_letter == Some(&'S') && (self_state.unwrap() & Connections::XMAS) > 0) {
                    xmas_counter += 1;
                }
            }

            if horizontal_letter.is_some() {
                let precond = get_precondition(&horizontal_letter.unwrap());
                for (prev_letter, state) in precond {

                }
            }
        }
    }
}

fn main() {
    let contents: String = fs::read_to_string("input").expect("Could not read data");

    run_for_input(&contents);
}
