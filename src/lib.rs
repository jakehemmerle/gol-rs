//! Game of Life

#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

const SIZE: usize = 10;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GolCell {
    Dead = 0,
    Alive,
}

pub type Matrix = [[GolCell; SIZE]; SIZE];

static mut MATRIX1: Matrix = [[GolCell::Dead; SIZE]; SIZE];
static mut MATRIX2: Matrix = [[GolCell::Dead; SIZE]; SIZE];

pub struct GameBoard {
    pub current_state: &'static mut Matrix,
    temp_state: &'static mut Matrix,
}

#[cfg(not(test))]
#[panic_handler]
pub fn panic_handler(_: &core::panic::PanicInfo) -> ! {
   loop {}
}

impl GameBoard {
    pub fn new() -> Self {
        unsafe {
            GameBoard {
                current_state: &mut MATRIX1,
                temp_state: &mut MATRIX2,
            }

        }
    }

    /// Update current state to the next state
    pub fn go_to_next_state<'a>(&'a mut self) -> &'a mut Matrix {
        for y in 0..SIZE {
            for x in 0..SIZE {
                let alive_neighbours = self.count_alive_neighbours(y, x);
                self.temp_state[y][x] = match self.current_state[y][x] {
                    GolCell::Dead => {
                        if alive_neighbours == 3 {
                            GolCell::Alive
                        } else {
                            GolCell::Dead
                        }
                    }
                    GolCell::Alive => {
                        if alive_neighbours < 2 || alive_neighbours > 3 {
                            GolCell::Dead
                        } else {
                            GolCell::Alive
                        }
                    }
                };
            }
        }

        core::mem::swap(&mut self.current_state, &mut self.temp_state);

        self.current_state
    }

    /// Count the number of alive neighbours for a given cell in the current state
    fn count_alive_neighbours(&self, y: usize, x: usize) -> usize {
        let y_above = if y == 0 { SIZE - 1 } else { y - 1 };
        let y_below = if y == SIZE - 1 { 0 } else { y + 1 };
        let x_above = if x == 0 { SIZE - 1 } else { x - 1 };
        let x_below = if x == SIZE - 1 { 0 } else { x + 1 };

        let mut count = 0;
        let mut update_count = |cell: GolCell| {
            if cell == GolCell::Alive {
                count += 1;
            }
        };

        update_count(self.current_state[y_above][x_above]);
        update_count(self.current_state[y_above][x]);
        update_count(self.current_state[y_above][x_below]);
        update_count(self.current_state[y][x_above]);
        update_count(self.current_state[y][x_below]);
        update_count(self.current_state[y_below][x_above]);
        update_count(self.current_state[y_below][x]);
        update_count(self.current_state[y_below][x_below]);

        count
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    pub fn print_board(board: &Matrix) {
        for row in board {
            for cell in row {
                match cell {
                    GolCell::Dead => print!("-"),
                    GolCell::Alive => print!("O"),
                }
            }
            println!();
        }
    }

    #[test]
    fn check_valid_matrix() {
        let mut game_board = GameBoard::new();

        // set an initial basic glider
        game_board.current_state[0][1] = GolCell::Alive;
        game_board.current_state[1][2] = GolCell::Alive;
        game_board.current_state[2][1] = GolCell::Alive;
        game_board.current_state[2][0] = GolCell::Alive;
        game_board.current_state[2][2] = GolCell::Alive;

        // print intial state
        print_board(game_board.current_state);

        // repeat 100 times
        for _ in 0..100 {
            game_board.go_to_next_state();
            print_board(game_board.current_state);
            // delay 100 ms using std
            // std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}
