use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug)]
struct Tree {
    height: i16,
    position: [usize; 2],
}

impl Tree {
    fn new(height: i16, position: [usize; 2]) -> Self {
        Self { height, position }
    }
}

fn create_matrix(input: String) -> Vec<Vec<i16>> {
    let mut matrix: Vec<Vec<i16>> = Vec::new();

    for a in input.lines() {
        let mut row: Vec<i16> = Vec::new();
        for b in a.trim().chars() {
            row.push(b.to_digit(10).unwrap() as i16);
        }
        matrix.push(row);
    }

    matrix
}

fn get_all_visible_trees(mut matrix: Vec<Vec<i16>>) -> i16 {
    let mut total_visible_trees = 0;

    for row in 0..matrix.len() {

        let mut right_map: HashMap<i16, [usize; 2]> = HashMap::new();
        let mut left_map: HashMap<i16, [usize; 2]> = HashMap::new();

        let mut left = 0;
        let mut right = matrix[row].len() - 1;

        let mut left_max = Tree::new(0, [0, 0]);
        let mut right_max = Tree::new(0, [0, 0]);

        while left <= right {

            if row == 0 || row == matrix.len() - 1 {
                matrix[row][left] = -matrix[row][left];
                matrix[row][right] = -matrix[row][right];

                total_visible_trees += 2;
                if left == right {
                    total_visible_trees -= 1;
                }
            } else if left == 0 && right == matrix[0].len() - 1 {
                left_max = Tree::new(matrix[row][left], [row, left]);
                right_max = Tree::new(matrix[row][right], [row, right]);

                matrix[row][left] = -matrix[row][left];
                matrix[row][right] = -matrix[row][right];
                total_visible_trees += 2;
            } else if left == right
                && matrix[row][left] > left_max.height
                && matrix[row][left] > right_max.height
            {
                left_max = Tree::new(matrix[row][left], [row, left]);
                matrix[row][left] = -matrix[row][left];
                total_visible_trees += 1;
            } else {
                if matrix[row][left] >= left_max.height {
                    if matrix[row][left] == left_max.height {
                        left_max = Tree::new(matrix[row][left], [row, left]);
                    } else {
                        left_max = Tree::new(matrix[row][left], [row, left]);
                        matrix[row][left] = -matrix[row][left];
                        total_visible_trees += 1;
                    }
                } else if matrix[row][left] > right_max.height
                    && matrix[row][left] > matrix[row][right]
                {
                    left_map.insert(matrix[row][left], [row, left]);
                    matrix[row][left] = -matrix[row][left];
                    total_visible_trees += 1;
                }

                if matrix[row][right] >= right_max.height {
                    if matrix[row][right] == right_max.height {
                        right_max = Tree::new(matrix[row][right], [row, right]);
                    } else {
                        right_max = Tree::new(matrix[row][right], [row, right]);
                        matrix[row][right] = -matrix[row][right];
                        total_visible_trees += 1;
                    }
                } else if matrix[row][right] > left_max.height {
                    right_map.insert(matrix[row][right], [row, right]);
                    matrix[row][right] = -matrix[row][right];
                    total_visible_trees += 1;
                }
            }

            left += 1;
            right -= 1;
        }

        for (key, value) in left_map {
            if (key <= left_max.height && value[1] < left_max.position[1])
                || key <= right_max.height
            {
                matrix[value[0]][value[1]] = matrix[value[0]][value[1]].abs();
                total_visible_trees -= 1;
            }
        }

        for (key, value) in right_map {
            if (key <= right_max.height && value[1] > right_max.position[1])
                || key <= left_max.height
            {
                matrix[value[0]][value[1]] = matrix[value[0]][value[1]].abs();
                total_visible_trees -= 1;
            }
        }
    }

    for column in 0..matrix[0].len() {
        let mut top_map: HashMap<i16, [usize; 2]> = HashMap::new();
        let mut bot_map: HashMap<i16, [usize; 2]> = HashMap::new();

        let mut top = 0;
        let mut bot = matrix.len() - 1;

        let mut top_max = Tree::new(0, [0, 0]);
        let mut bot_max = Tree::new(0, [0, 0]);

        while top <= bot {
            if top == 0 && bot == matrix.len() - 1 {
                top_max = Tree::new(matrix[top][column].abs(), [top, column]);
                bot_max = Tree::new(matrix[bot][column].abs(), [bot, column]);
            } else if column == 0 && column == matrix[0].len() - 1 {
            } else if top == bot
                && matrix[top][column].abs() > top_max.height
                && matrix[top][column].abs() > bot_max.height
            {
                top_max = Tree::new(matrix[top][column].abs(), [top, column]);
                if matrix[top][column] >= 0 {
                    total_visible_trees += 1;
                }
            } else {
                if matrix[top][column].abs() >= top_max.height {
                    if matrix[top][column].abs() == top_max.height {
                        top_max = Tree::new(matrix[top][column].abs(), [top, column]);
                    } else {
                        top_max = Tree::new(matrix[top][column].abs(), [top, column]);
                        if matrix[top][column] >= 0 {
                            total_visible_trees += 1;
                        };
                    }
                } else if matrix[top][column] >= 0
                    && matrix[top][column] > bot_max.height
                    && matrix[top][column] > matrix[bot][column].abs()
                {
                    top_map.insert(matrix[top][column], [top, column]);
                    total_visible_trees += 1;
                }

                if matrix[bot][column].abs() >= bot_max.height {
                    if matrix[bot][column].abs() == bot_max.height {
                        bot_max = Tree::new(matrix[bot][column].abs(), [bot, column]);
                    } else {
                        bot_max = Tree::new(matrix[bot][column].abs(), [bot, column]);
                        if matrix[bot][column] >= 0 {
                            total_visible_trees += 1;
                        }
                    }
                } else if matrix[bot][column] >= 0 && matrix[bot][column] > top_max.height {
                    bot_map.insert(matrix[bot][column], [bot, column]);
                    total_visible_trees += 1;
                }
            }

            top += 1;
            bot -= 1;
        }

        for (key, value) in top_map {
            if (key <= top_max.height && value[0] < top_max.position[0]) || key <= bot_max.height {
                total_visible_trees -= 1;
            }
        }

        for (key, value) in bot_map {
            if (key <= bot_max.height && value[0] > bot_max.position[0]) || key <= top_max.height {
                total_visible_trees -= 1;
            }
        }
    }

    total_visible_trees
}

//================================================================================================

fn get_highest_scenic_score(matrix: Vec<Vec<i16>>) -> i32 {
    let mut max = 0;

    for row in 1..matrix.len() - 1 {
        for column in 1..matrix[0].len() - 1 {
            let curr = matrix[row][column];

            let mut scenic_score = 0;
            let mut top_visible_trees = 1;
            let mut bot_visible_trees = 1;
            let mut left_visible_trees = 1;
            let mut right_visible_trees = 1;

            let mut y = row - 1;

            while y >= 1 && curr > matrix[y][column] {
                top_visible_trees += 1;
                y -= 1;
            }

            y = row + 1;

            while y <= matrix.len() - 2 && curr > matrix[y][column] {
                bot_visible_trees += 1;
                y += 1;
            }

            let mut x = column - 1;

            while x >= 1 && curr > matrix[row][x] {
                left_visible_trees += 1;
                x -= 1;
            }

            x = column + 1;

            while x <= matrix[0].len() - 2 && curr > matrix[row][x] {
                right_visible_trees += 1;
                x += 1;
            }

            scenic_score =
                top_visible_trees * bot_visible_trees * left_visible_trees * right_visible_trees;

            if max < scenic_score {
                max = scenic_score;
            }
        }
    }
    max
}
