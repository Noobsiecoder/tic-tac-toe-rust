fn check_row(matrix: &[&'static str; 9]) -> bool {
    let mut index = 0;
    while index < 9 {
        if matrix[index] == matrix[index + 1] && matrix[index + 1] == matrix[index + 2] {
            return true;
        }
        index += 3;
    }

    false
}

fn check_column(matrix: &[&'static str; 9]) -> bool {
    let mut index = 0;
    while index < 3 {
        if matrix[index] == matrix[index + 3] && matrix[index + 3] == matrix[index + 6] {
            return true;
        }
        index += 1;
    }

    false
}

fn check_left_diagonal(matrix: &[&'static str; 9]) -> bool {
    if matrix[0] == matrix[4] && matrix[4] == matrix[8] {
        return true;
    }

    false
}

fn check_right_diagonal(matrix: &[&'static str; 9]) -> bool {
    if matrix[2] == matrix[4] && matrix[4] == matrix[6] {
        return true;
    }

    false
}

pub fn check_winner(matrix: &[&'static str; 9]) -> bool {
    if check_row(&matrix)
        || check_column(&matrix)
        || check_left_diagonal(&matrix)
        || check_right_diagonal(&matrix)
    {
        return true;
    }

    false
}
