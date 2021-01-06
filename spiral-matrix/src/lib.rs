enum Direction {
    North,
    East,
    South,
    West,
}

fn matrix_available(matrix: &[Vec<u32>], x: isize, y: isize) -> bool {
    if x < 0 || y < 0 {
        return false;
    }
    let x = x as usize;
    let y = y as usize;
    if let Some(row) = matrix.get(y) {
        if let Some(element) = row.get(x) {
            if *element == 0 {
                return true;
            }
        }
    }
    false
}

fn matrix_set(matrix: &mut [Vec<u32>], x: isize, y: isize, value: u32) {
    matrix[y as usize][x as usize] = value;
}

fn grid_forward(x: &mut isize, y: &mut isize, dir: &Direction) {
    use Direction::*;
    match dir {
        East => *x += 1,
        North => *y += 1,
        South => *y -= 1,
        West => *x -= 1,
    }
}

fn grid_backup(x: &mut isize, y: &mut isize, dir: &Direction) {
    use Direction::*;
    match dir {
        East => *x -= 1,
        North => *y -= 1,
        South => *y += 1,
        West => *x += 1,
    }
}

fn turn(dir: Direction) -> Direction {
    use Direction::*;
    match dir {
        East => North,
        North => West,
        West => South,
        South => East,
    }
}

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    use Direction::*;
    let mut matrix = vec![vec![0; size as usize]; size as usize];

    if size == 0 {
        return matrix;
    }
    let mut x = 0;
    let mut y = 0;
    let mut dir = East;
    let mut value = 1;
    loop {
        if !matrix_available(&matrix, x, y) {
            return matrix;
        } else {
            loop {
                matrix_set(&mut matrix, x, y, value);
                value += 1;
                grid_forward(&mut x, &mut y, &dir);
                if !matrix_available(&matrix, x, y) {
                    grid_backup(&mut x, &mut y, &dir);
                    dir = turn(dir);
                    grid_forward(&mut x, &mut y, &dir);
                    break;
                }
            }
        }
    }
}
