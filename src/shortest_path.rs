pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    if grid.len() == 0 {
        return 0;
    }

    let mut grid = grid;
    for i in 0..grid.len() {
        for j in 0..grid.get(i).unwrap().len() {
            if i == 0 && j == 0 {
                continue;
            } else if i == 0 {
                grid[i][j] += grid[i][j - 1];
            } else if j == 0 {
                grid[i][j] += grid[i - 1][j];
            } else {
                grid[i][j] += std::cmp::min(grid[i - 1][j], grid[i][j - 1]);
            }
        }
    }

    grid.get(grid.len() - 1).unwrap()[grid[0].len() - 1]
}

#[test]
fn test_shortest_path() {
    let i = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
    let a = min_path_sum(i);
    assert_eq!(a, 7);
}
