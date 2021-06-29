pub fn print_board(matrix: &[&'static str; 9]) {
    println!("|-----|-----|-----|");
    for i in 1..=9 {
        if i % 3 == 0 {
            println!("|  {}  |", matrix[i - 1]);
            println!("|-----|-----|-----|");
        } else {
            print!("|  {}  ", matrix[i - 1]);
        }
    }
}
