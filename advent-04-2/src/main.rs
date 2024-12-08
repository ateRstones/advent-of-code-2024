use std::fs;

fn count_xmas(xmas_map: &Vec<Vec<char>>) -> i32 {
    let mut xmas_count = 0;
    for line in xmas_map {
        let mut valid_xmas = false;
        let mut prev_letter: Option<char> = None;
        for letter in line {
            match letter {
                'X'=>{valid_xmas=true}
                'M'=>{if (valid_xmas && prev_letter != Some('X')) {valid_xmas = false}}
                'A'=>{if (valid_xmas && prev_letter != Some('M')) {valid_xmas = false}}
                'S'=>{if (valid_xmas && prev_letter == Some('A')) {xmas_count += 1}; valid_xmas=false;}
                _=>{valid_xmas = false}
            }
            prev_letter = Some(*letter);
        }
    }
    return xmas_count;
}

fn check_mas_cross(xmas_map: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    return xmas_map[x+1][y] == 'M' && xmas_map[x+1][y+1] == 'A' && xmas_map[x+1][y+2] == 'S'
        && xmas_map[x][y+1] == 'M' && xmas_map[x+2][y+1] == 'S';
}

fn check_mas_x(xmas_map: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    return xmas_map[x][y] == 'M' && xmas_map[x+1][y+1] == 'A' && xmas_map[x+2][y+2] == 'S'
        && xmas_map[x+2][y] == 'M' && xmas_map[x][y+2] == 'S';
}

fn count_crossmas(xmas_map: &Vec<Vec<char>>) -> i32 {
    let mut xmas_count = 0;
    // assume 2nd dim is always the same
    let y_len = xmas_map[0].len();

    for x in 0..xmas_map.len()-2 {
        for y in 0..y_len-2 {
            // if check_mas_cross(xmas_map, x, y) {
            //     xmas_count += 1;
            // }

            if check_mas_x(xmas_map, x, y) {
                xmas_count += 1;
            }
        }
    }

    return xmas_count;
}

fn run_for_input(contents: &String) {
    let mut xmas_map: Vec<Vec<char>> = contents.lines().map(|x | x.chars().collect()).collect();
    let mut xmas_count = 0;
    let mut xmas_count_2 = 0;

    for _i in 0..4 {
        let rotate_45 = rotate_2d_array_45(&xmas_map);
        let rotate_90 = rotate_2d_array_90(&xmas_map);

        let size1: usize = rotate_45.clone().into_iter().map(|x| x.len()).sum();
        let size2: usize = rotate_90.clone().into_iter().map(|x| x.len()).sum();
        println!("Size1 {size1} Size2 {size2}");

        xmas_count += count_xmas(&rotate_45);
        xmas_count += count_xmas(&rotate_90);
        xmas_count_2 += count_crossmas(&rotate_90);

        xmas_map = rotate_90;
    }

    println!("Count: {xmas_count}");
    println!("Count 2: {xmas_count_2}");
}

// 1924 min
// 1983 max

fn main() {
    let contents: String = fs::read_to_string("input").expect("Could not read data");

    let test_contents: String = String::from("MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX");

    let test_contents2: String = String::from(".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........");

    //run_for_input(&contents);
    run_for_input(&contents)
}

fn rotate_2d_array_45<T: Clone>(array: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let rows = array.len();
    if rows == 0 {
        return vec![];
    }
    let cols = array[0].len();

    // Create a new structure to hold diagonals
    let mut diagonals: Vec<Vec<T>> = vec![vec![]; rows + cols - 1];

    for i in 0..rows {
        for j in 0..cols {
            diagonals[i + j].push(array[i][j].clone());
        }
    }

    diagonals
}

fn rotate_2d_array_90<T: Clone>(array: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let rows = array.len();
    if rows == 0 {
        return vec![];
    }
    let cols = array[0].len();

    // Create a new array with dimensions flipped
    let mut rotated = vec![vec![array[0][0].clone(); rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            rotated[j][rows - i - 1] = array[i][j].clone();
        }
    }

    rotated
}

fn main_test() {
    let test_contents2: String = String::from(".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........");

    let mut array: Vec<Vec<char>> = test_contents2.lines().map(|x | x.chars().collect()).collect();

    for diagonal in &array {
        println!("{:?}", diagonal);
    }

    let mut rotated = rotate_2d_array_90(&array);

    println!("90");
    for diagonal in &rotated {
        println!("{:?}", diagonal);
    }

    rotated = rotate_2d_array_90(&rotated);

    println!("180");
    for diagonal in &rotated {
        println!("{:?}", diagonal);
    }

    rotated = rotate_2d_array_90(&rotated);

    println!("270");
    for diagonal in &rotated {
        println!("{:?}", diagonal);
    }
}
