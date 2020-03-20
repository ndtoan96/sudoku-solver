#[derive(PartialEq)]
#[derive(Debug)]
pub enum Status {
    NotCompleted,
    Valid,
    Invalid
}

pub fn sudoku_solver(board: &[[u8; 9]; 9]) -> Result<[[u8; 9]; 9], String> {
    let mut res = *board;
    let mut imin = 10;
    let mut jmin = 10;
    let mut pos = vec![];
    let mut minlen = 100;

    loop {
        let mut cont = false;
        for i in 0..9 {
            for j in 0..9 {
                if res[i][j] == 0 {
                    let possibility = possible_nums(&res, i, j);
                    let len = possibility.len();
                    if len == 0 {
                        return Err("Can't find solution".to_string());
                    } else if len == 1 {
                        res[i][j] = possibility[0];
                        cont = true;
                        break;
                    } else if len < minlen {
                        minlen = len;
                        imin = i;
                        jmin = j;
                        pos = possibility;
                    }
                }
            }
        }

        if !cont {
            break;
        }
    }

    match sudoku_check(&res) {
        Status::Valid => return Ok(res),
        Status::Invalid => return Err("Invalid sudoku.".to_string()),
        Status::NotCompleted => {
            for num in pos {
                res[imin][jmin] = num;
                if let Ok(b) = sudoku_solver(&res) {
                    return Ok(b);
                }
            }
            return Err("Can't find solution.".to_string());
        }
    }
}

fn possible_nums(board: &[[u8; 9]; 9], row: usize, col: usize) -> Vec<u8> {
    let mut res = vec![];
    let mut map = [0; 9];
    for k in 0..9 {
        if (k != col) && (board[row][k] != 0) && (board[row][k] < 9) {
            map[(board[row][k]-1) as usize] += 1;
        }

        if (k != row) && (board[k][col] != 0) && (board[k][col] < 9) {
            map[(board[k][col]-1) as usize] += 1;
        }
    }

    for i in (row/3)*3..(row/3)*3+3 {
        for j in (col/3)*3..(col/3)*3+3 {
            if (i != row) && (j != col) && (board[i][j] != 0) && (board[i][j] < 9) {
                map[(board[i][j]-1) as usize] += 1;
            }
        }
    }

    for (x, &cnt) in map.iter().enumerate() {
        if cnt == 0 {
            res.push((x+1) as u8);
        }
    }

    res
}

fn is_duplicate(arr: &[u8; 9]) -> bool {
    let mut map: [u8; 9] = [0; 9];
    for &value in arr {
        if value != 0 {
            map[(value-1) as usize] += 1;
        }
    }
    for &i in &map {
        if i > 1 {
            return true;
        }
    }
    
    false
}

pub fn sudoku_check(board: &[[u8; 9]; 9]) -> Status {
    
    for i in 0..9 {
        for j in 0..9 {
            if board[i][j] > 9 {
                return Status::Invalid;
            }
        }
    }

    for k in 0..9 {
        let row = k;
        if is_duplicate(&board[row]) {
            return Status::Invalid;
        }
        
        let col = k;
        let mut tmp_col = [0; 9];
        for i in 0..9 {
            tmp_col[i] = board[i][col];
        }
        if is_duplicate(&tmp_col) {
            return Status::Invalid;
        }
        
        let reg = k;
        let mut tmp_reg = [0; 9];
        let mut idx = 0;
        for i in (reg/3)*3..(reg/3)*3+3 {
            for j in (reg%3)*3..(reg%3)*3+3 {
                tmp_reg[idx] = board[i][j];
                idx += 1;
            }
        }
        if is_duplicate(&tmp_reg) {
            return Status::Invalid;
        }
    }
    
    for i in 0..9 {
        for j in 0..9 {
            if board[i][j] == 0 {
                return Status::NotCompleted;
            }
        }
    }

    Status::Valid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_right_one(){
        let res = [
            [5,3,4,6,7,8,9,1,2],
            [6,7,2,1,9,5,3,4,8],
            [1,9,8,3,4,2,5,6,7],
            [8,5,9,7,6,1,4,2,3],
            [4,2,6,8,5,3,7,9,1],
            [7,1,3,9,2,4,8,5,6],
            [9,6,1,5,3,7,2,8,4],
            [2,8,7,4,1,9,6,3,5],
            [3,4,5,2,8,6,1,7,9]
        ];
        assert_eq!(sudoku_check(&res), Status::Valid);
    }

    #[test]
    fn check_right_two() {
        let res = [
            [5,7,4,2,3,6,8,1,9],
            [3,8,1,5,4,9,2,6,7],
            [2,6,9,1,8,7,5,4,3],
            [8,4,7,3,1,2,9,5,6],
            [9,1,2,7,6,5,4,3,8],
            [6,3,5,8,9,4,1,7,2],
            [4,2,6,9,7,1,3,8,5],
            [1,9,3,6,5,8,7,2,4],
            [7,5,8,4,2,3,6,9,1],
        ];
        assert_eq!(sudoku_check(&res), Status::Valid);
    }

    #[test]
    fn check_invalid() {
        let res = [
            [5,3,4,6,7,8,9,1,2],
            [6,7,2,1,9,5,3,4,8],
            [1,9,10,8,4,2,5,6,7],
            [8,5,9,7,6,1,4,2,3],
            [4,2,6,8,5,3,7,9,1],
            [7,1,3,9,2,4,8,5,6],
            [9,6,1,5,3,7,2,8,4],
            [2,8,7,4,1,9,6,3,5],
            [3,4,5,2,8,6,1,7,9]
        ];
        assert_eq!(sudoku_check(&res), Status::Invalid);

        let res = [
            [5,3,4,6,7,8,9,1,2],
            [6,7,2,1,9,5,3,4,8],
            [1,9,3,8,4,2,5,6,7],
            [8,5,9,7,6,1,4,2,3],
            [4,2,6,8,5,3,7,9,1],
            [7,1,3,9,2,4,8,5,6],
            [9,6,1,5,3,7,2,8,4],
            [2,8,7,4,1,9,6,3,5],
            [3,4,5,2,8,6,1,7,9]
        ];
        assert_eq!(sudoku_check(&res), Status::Invalid);

        let res = [
            [5,3,4,6,7,8,9,1,2],
            [6,7,2,1,9,5,3,4,8],
            [1,9,3,3,4,2,5,6,7],
            [8,5,9,7,6,1,4,2,3],
            [4,2,6,8,5,3,7,9,1],
            [7,1,3,9,2,4,8,5,6],
            [9,6,1,5,3,7,2,8,4],
            [2,8,7,4,1,9,6,3,5],
            [3,4,5,2,8,6,1,7,9]
        ];
        assert_eq!(sudoku_check(&res), Status::Invalid);

        let res = [
            [1,2,3,4,5,6,7,8,9],
            [9,1,2,3,4,5,6,7,8],
            [8,9,1,2,3,4,5,6,7],
            [7,8,9,1,2,3,4,5,6],
            [6,7,8,9,1,2,3,4,5],
            [5,6,7,8,9,1,2,3,4],
            [4,5,6,7,8,9,1,2,3],
            [3,4,5,6,7,8,9,1,2],
            [2,3,4,5,6,7,8,9,1]
        ];
        assert_eq!(sudoku_check(&res), Status::Invalid);
    }

    #[test]
    fn check_not_completed() {
        let prob = [
            [5,3,0,0,7,0,0,0,0],
            [6,0,0,1,9,5,0,0,0],
            [0,9,8,0,0,0,0,6,0],
            [8,0,0,0,6,0,0,0,3],
            [4,0,0,8,0,3,0,0,1],
            [7,0,0,0,2,0,0,0,6],
            [0,6,0,0,0,0,2,8,0],
            [0,0,0,4,1,9,0,0,5],
            [0,0,0,0,8,0,0,7,9]
        ];
        assert_eq!(sudoku_check(&prob), Status::NotCompleted);
    }

    #[test]
    fn problem_one() {
        let prob = [
            [5,3,0,0,7,0,0,0,0],
            [6,0,0,1,9,5,0,0,0],
            [0,9,8,0,0,0,0,6,0],
            [8,0,0,0,6,0,0,0,3],
            [4,0,0,8,0,3,0,0,1],
            [7,0,0,0,2,0,0,0,6],
            [0,6,0,0,0,0,2,8,0],
            [0,0,0,4,1,9,0,0,5],
            [0,0,0,0,8,0,0,7,9]
        ];
        let res = [
            [5,3,4,6,7,8,9,1,2],
            [6,7,2,1,9,5,3,4,8],
            [1,9,8,3,4,2,5,6,7],
            [8,5,9,7,6,1,4,2,3],
            [4,2,6,8,5,3,7,9,1],
            [7,1,3,9,2,4,8,5,6],
            [9,6,1,5,3,7,2,8,4],
            [2,8,7,4,1,9,6,3,5],
            [3,4,5,2,8,6,1,7,9]
        ];
        assert_eq!(sudoku_solver(&prob).unwrap(), res);
    }

    #[test]
    fn problem_two() {
        let prob = [
            [5,7,0,0,0,6,8,0,0],
            [3,0,1,5,4,0,0,0,0],
            [0,6,9,0,8,0,0,0,3],
            [0,0,0,3,0,2,9,0,0],
            [9,0,0,0,0,0,0,0,8],
            [0,0,5,8,0,4,0,0,0],
            [4,0,0,0,7,0,3,8,0],
            [0,0,0,0,5,8,7,0,4],
            [0,0,8,4,0,0,0,9,1],
        ];
        let res = [
            [5,7,4,2,3,6,8,1,9],
            [3,8,1,5,4,9,2,6,7],
            [2,6,9,1,8,7,5,4,3],
            [8,4,7,3,1,2,9,5,6],
            [9,1,2,7,6,5,4,3,8],
            [6,3,5,8,9,4,1,7,2],
            [4,2,6,9,7,1,3,8,5],
            [1,9,3,6,5,8,7,2,4],
            [7,5,8,4,2,3,6,9,1],
        ];
        assert_eq!(sudoku_solver(&prob).unwrap(), res);
    }
}