mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, web-tac-toe!");
}

#[wasm_bindgen]
pub struct Board {
    squares: Vec<i8>,
    player: i8,
    game_over: bool,
}

#[wasm_bindgen]
impl Board {
    pub fn new() -> Board {
        let squares = vec![0; 9];
        let player = 1;
        let game_over = false;

        Board {
            squares,
            player,
            game_over,
        }
    }

    pub fn get_state(&self) -> Vec<i8> {
        self.squares.clone()
    }

    pub fn get_available_squares(&self) -> Vec<usize> {
        self.squares
            .clone()
            .iter()
            .enumerate()
            .filter(|(_, &r)| r == 0)
            .map(|(index, _)| index)
            .collect::<Vec<_>>()
    }

    pub fn square_weight(&self, index: usize) -> f32 {
        0.5
    }

    pub fn say_hi(&self) -> String {
        String::from("Hi!")
    }

    pub fn is_game_over(&self) -> bool {
        return self.game_over;
    }

    pub fn reset(&mut self) {
        self.squares = vec![0; 9];
        self.player = 1;
        self.game_over = false;
    }

    pub fn get_grid(&self, index: usize) -> i8 {
        self.squares[index]
    }

    pub fn claim_square(&mut self, index: usize) {
        if self.squares[index] == 0 {
            self.squares[index] = self.player;
            self.swap_player();
        }
    }

    fn swap_player(&mut self) {
        if self.player == 1 {
            self.player = -1;
        } else {
            self.player = 1;
        }
    }

    pub fn get_symbol(&self, index: usize) -> String {
        match self.squares[index] {
            1 => String::from("X"),
            -1 => String::from("O"),
            _ => String::from(" "),
        }
    }

    pub fn check_for_win(&mut self) -> String {
        if (self.squares[0] + self.squares[1] + self.squares[2]).abs() == 3 {
            self.game_over = true;
            return self.get_symbol(0);
        } else if (self.squares[3] + self.squares[4] + self.squares[5]).abs() == 3 {
            self.game_over = true;
            return self.get_symbol(3);
        } else if (self.squares[6] + self.squares[7] + self.squares[8]).abs() == 3 {
            self.game_over = true;
            return self.get_symbol(6);
        } else if (self.squares[0] + self.squares[3] + self.squares[6]).abs() == 3 {
            self.game_over = true;
            return self.get_symbol(0);
        } else if (self.squares[1] + self.squares[4] + self.squares[7]).abs() == 3 {
            self.game_over = true;
            return self.get_symbol(1);
        } else if (self.squares[2] + self.squares[5] + self.squares[8]).abs() == 3 {
            self.game_over = true;
            return self.get_symbol(2);
        } else if (self.squares[0] + self.squares[4] + self.squares[8]).abs() == 3 {
            self.game_over = true;
            return self.get_symbol(0);
        } else if (self.squares[2] + self.squares[4] + self.squares[6]).abs() == 3 {
            self.game_over = true;
            return self.get_symbol(2);
        } else {
            return String::from(" ");
        }
    }
}

#[wasm_bindgen]
pub struct Agent {
    name: String,
}

#[wasm_bindgen]
impl Agent {
    pub fn new() -> Agent {
        let name = String::from("Arundhati");

        Agent { name }
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn get_choice(&self) -> i8 {
        6
    }
}
