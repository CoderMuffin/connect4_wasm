use wasm_bindgen::prelude::*;

const ROWS: usize = 6;
const COLUMNS: usize = 7;

#[wasm_bindgen]
pub struct Move {
    row: usize,
    column: usize,
    color: i8
}

#[wasm_bindgen]
pub struct Game {
    board: [[i8; ROWS]; COLUMNS],
    history: Vec<Move>,
    nonFull: Vec<usize>,
    lastMove: usize
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            board: [[-1; ROWS]; COLUMNS],
            history: Vec::new(),
            nonFull: (0..COLUMNS).collect(),
            lastMove: 3
        }
    }
    #[wasm_bindgen]
    pub fn fourEqual(a: i8, b: i8, c: i8, d: i8) -> bool {
        a == b && a == c && a == d
    }
    #[wasm_bindgen]
    pub fn win(&self) -> i8 {
        // right
        for y in 0..ROWS {
            for x in 0..COLUMNS - 3 {
                if self.board[y][x] != -1 && Self::fourEqual(self.board[y][x], self.board[y][x + 1], self.board[y][x + 2], self.board[y][x + 3]) {
                    return self.board[y][x];
                }
            }
        }
        // down
        for x in 0..COLUMNS {
            for y in 0..ROWS - 3 {
                if self.board[y][x] != -1 && Self::fourEqual(self.board[y][x], self.board[y + 1][x], self.board[y + 2][x], self.board[y + 3][x]) {
                    return self.board[y][x];
                }
            }
        }
        // down right
        for y in 0..ROWS - 3 {
            for x in 0..COLUMNS {
                if self.board[y][x] != -1 && Self::fourEqual(self.board[y][x], self.board[y + 1][x + 1], self.board[y + 2][x + 2], self.board[y + 3][x + 3]) {
                    return self.board[y][x];
                }
            }
        }
        // down left
        for y in 0..ROWS - 3 {
            for x in 3..COLUMNS {
                if self.board[y][x] != -1 && Self::fourEqual(self.board[y][x], self.board[y + 1][x - 1], self.board[y + 2][x - 2], self.board[y + 3][x - 3]) {
                    return self.board[y][x];
                }
            }
        }
        return -1;
    }

    #[wasm_bindgen]
    pub fn make_move(&mut self, color: i8, column: usize) -> usize {
        let mut row = ROWS - 1;
        for y in 0..ROWS {
            if self.board[y][column] != -1 {
                row = y - 1;
                break;
            }
        }

        if row < 0 {
            panic!("Invalid move!");
        }
        if row == 0 {
            self.nonFull.retain(|&x| x != column)
        }

        self.history.push(Move {
            color: color,
            row: row,
            column: column
        });

        self.board[row][column] = color;
        self.lastMove = column;
        return row;
    }
    
    #[wasm_bindgen]
    pub fn unmove(&mut self) {
        let r#move = self.history.pop().expect("Cannot pop any more moves");
        self.board[r#move.row][r#move.column] = -1;
        if r#move.row == 0 {
            self.nonFull.push(r#move.column);
        }
    }
    #[wasm_bindgen]
    pub fn moves(&self) -> Vec<usize> {
        let mut vec = self.nonFull.iter().cloned().collect::<Vec<_>>();
        vec.sort_by_key(|a| (self.lastMove as isize - (*a) as isize).abs());
        vec
    }
}
