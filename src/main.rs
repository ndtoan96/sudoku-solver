fn main() -> Result<(), String> {
    let mut prob: [[u8; 9]; 9] = [[0; 9]; 9];
    println!("Input each line of the sudoku, each number is seperated by whitespace and blank is replaced by 0:");
    for row in 0..9 {
        let mut buf = String::new();
        if std::io::stdin().read_line(&mut buf).is_err() {
            return Err("Cannot read user's input.".to_string());
        }
        let seq = buf[..buf.len()-1].split(" ");
        let mut nums = vec![];
        for num in seq {
            match num.parse::<u8>() {
                Ok(num) => nums.push(num),
                Err(_) => return Err(format!("Cannot convert {} to number.", num))
            }
        }
        if nums.len() >= 9 {
            for col in 0..9 {
                prob[row][col] = nums[col];
            }
        } else {
            return Err("Not enough arguments.".to_string());
        }
    }

    let res = sudoku::sudoku_solver(&prob)?;
    println!("\nThe result is:");
    for i in 0..9 {
        for j in 0..9 {
            print!("{} ", res[i][j]);
        }
        println!("");
    }
    Ok(())
}
