use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct SudokuFile {
    sudokus: Vec<Sudoku>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Sudoku {
    id: String,
    difficulty: u8,
    start: Vec<String>,
}

#[derive(Debug, PartialEq)]
enum SudokuError {
    InvalidPlacement(String),
}

impl From<Sudoku> for SudokuRaw {
    fn from(value: Sudoku) -> Self {
        let mut arr: [[u8; 9]; 9] = [[0; 9]; 9];
        for (x, a) in value.start.clone().into_iter().enumerate() {
            let mut acc_arr = [0; 9];
            for (y, c) in a.chars().enumerate() {
                acc_arr[y] = c.to_digit(10).unwrap() as u8;
            }
            arr[x] = acc_arr;
        }
        SudokuRaw { cell: arr }
    }
}

struct SudokuRaw {
    cell: [[u8; 9]; 9],
}
impl SudokuRaw {
    fn is_empty(&self, x: u8, y: u8) -> bool {
        self.cell[x as usize][y as usize] != 0
    }

    fn place(&mut self, x: u8, y: u8, val: u8) -> Result<(), SudokuError> {
        for (_x, c) in self.cell.into_iter().enumerate() {
            if c[y as usize] == val {
                return Err(SudokuError::InvalidPlacement(format!(
                    "Due to {},{}",
                    _x, y
                )));
            }
        }
        for (_y, c2) in self.cell[y as usize].into_iter().enumerate() {
            if c2 == val {
                return Err(SudokuError::InvalidPlacement(format!(
                    "Due to {},{}",
                    x, _y
                )));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let s: Vec<Sudoku> = serde_json::from_str(include_str!("../assets/expert.json")).unwrap();
        let s1 = s.first().unwrap().clone();
        let mut s2: SudokuRaw = s1.into();
        let result = s2.place(2, 2, 9);
        println!("Result: {result:?}");
        assert_eq!(Ok(()), result);
    }
}

impl Display for SudokuRaw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (x, a) in self.cell.clone().into_iter().enumerate() {
            for (i, c) in a.into_iter().enumerate() {
                f.write_str(
                    match c {
                        x if i == 2 || i == 5 => x.to_string() + " | ",
                        x => x.to_string() + " ",
                        _ => "x".to_string(),
                    }
                    .as_str()
                    .replace("0", " ")
                    .as_str(),
                );
            }
            f.write_str("\n");
            if x == 2 || x == 5 {
                f.write_str("---------------------\n");
            }
        }
        Ok(())
    }
}

impl Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("Suduko: {} ", self.id).as_str());
        f.write_str(format!("Difficulty: {} \n", self.difficulty).as_str());
        for (x, a) in self.start.clone().into_iter().enumerate() {
            for (i, c) in a.chars().enumerate() {
                f.write_str(
                    match c {
                        x @ '0'..='9' if i == 2 || i == 5 => x.to_string() + " | ",
                        x @ '0'..='9' => x.to_string() + " ",
                        _ => "x".to_string(),
                    }
                    .as_str()
                    .replace("0", " ")
                    .as_str(),
                );
            }
            f.write_str("\n");
            if x == 2 || x == 5 {
                f.write_str("---------------------\n");
            }
        }
        Ok(())
    }
}

fn main() {
    /*
    let sf = SudokuFile {
        sudokus: vec![Sudoku {
            id: "1".to_string(),
            dificulty: 1,
            start: vec![
                "000000000".to_string(),
                "000000000".to_string(),
                "000000000".to_string(),
                "000000000".to_string(),
                "000000000".to_string(),
                "000000000".to_string(),
                "000000000".to_string(),
                "000000000".to_string(),
                "000000000".to_string(),
            ],
        }],
    };
    */
    //let h = serde_json::to_string(&sf).unwrap();
    //println!("{}", h);
    let a: Vec<Sudoku> = serde_json::from_str(include_str!("../assets/expert.json")).unwrap();
    /*
    for s in a {
        println!("{s}");
    }
    */

    let s1 = a.first().unwrap().clone();
    let mut s2: SudokuRaw = s1.into();

    let mut x: u8 = 0;
    let mut y: u8 = 0;

    let mut v = 1;
    let mut history: Vec<(u8, u8)> = vec![];
    let mut reset = false;
    loop {
        if s2.is_empty(x, y) || reset {
            loop {
                match s2.place(x, y, v) {
                    Ok(_) => {
                        v = 1;
                        history.push((x, y));
                        reset = false;
                        break;
                    }
                    Err(_) if v > 8 => {
                        (x, y) = history.pop().unwrap_or((0, 0));
                        reset = true;
                        break;
                    }
                    Err(_) => {
                        v += 1;
                        reset = false;
                    }
                }
            }
        } else {
            if x == 8 {
                y += 1;
            } else {
                x += 1;
            }
        }
        if x == 8 && y == 8 {
            break;
        }
        if history.last().unwrap_or(&(0, 0)) == &(x, y) {
            (x, y) = history.pop().unwrap_or((0, 0));
            reset = true;
        }
        println!("{}{}", x, y);
    }
    println!("{s2}");
}
