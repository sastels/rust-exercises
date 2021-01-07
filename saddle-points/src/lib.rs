pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    if input.is_empty() || input[0].is_empty() {
        return vec![];
    }

    let mut saddle_points: Vec<(usize, usize)> = vec![];
    let row_max: Vec<u64> = input
        .iter()
        .map(|r| r.iter().max().unwrap())
        .cloned()
        .collect();
    let col_min: Vec<u64> = (0..input[0].len())
        .map(|n| input.iter().map(|v| v[n]).min().unwrap())
        .collect();

    for (x, row) in input.iter().enumerate() {
        for (y, &element) in row.iter().enumerate() {
            if element == row_max[x] && element == col_min[y] {
                saddle_points.push((x, y));
            }
        }
    }
    saddle_points
}
