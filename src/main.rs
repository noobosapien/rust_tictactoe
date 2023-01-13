use std::fmt;

#[derive(Debug, PartialEq)]
enum Cell {
    E,
    X,
    O,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Cell::E => write!(f, " "),
            Cell::X => write!(f, "X"),
            Cell::O => write!(f, "O"),
        }
    }
}

#[derive(Debug)]
struct Game {
    cells: Vec<Cell>,
    current_player: u8,
    won: Cell,
    valid: bool,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            " | {} | {} | {} | \n | {} | {} | {} | \n | {} | {} | {} | \n",
            self.cells[0],
            self.cells[1],
            self.cells[2],
            self.cells[3],
            self.cells[4],
            self.cells[5],
            self.cells[6],
            self.cells[7],
            self.cells[8],
        )
    }
}

impl Game {
    fn change_value(&mut self) {
        let mut input;

        loop {
            input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            let value = match input.trim().parse::<u32>() {
                Ok(n) => {
                    if n > 0 {
                        n - 1
                    } else {
                        10
                    }
                }

                Err(_e) => 10,
            };

            if value > 8 {
                println!("Invalid value please try again.\n");
                continue;
            } else {
                let cell = usize::try_from(value).unwrap();

                if self.cells[cell] == Cell::E {
                    if self.current_player == 0 {
                        self.cells[cell] = Cell::X;
                    } else {
                        self.cells[cell] = Cell::O;
                    }
                    println!("{}\n", self);
                } else {
                    println!("{}\n Please select another cell.", self);
                    continue;
                }
                break;
            }
        }
    }

    fn check_cells(&mut self) {
        let curr_val = match self.current_player {
            0 => Cell::X,
            1 => Cell::O,
            _ => Cell::E,
        };

        let mut valid = false;

        for cell in &self.cells {
            if *cell == Cell::E {
                valid = true;
            }
        }

        self.valid = valid;

        if (self.cells[0] != Cell::E
            && self.cells[0] == self.cells[1]
            && self.cells[1] == self.cells[2])
            || (self.cells[3] != Cell::E
                && self.cells[3] == self.cells[4]
                && self.cells[4] == self.cells[5])
            || (self.cells[6] != Cell::E
                && self.cells[6] == self.cells[7]
                && self.cells[7] == self.cells[8])
            || (self.cells[0] != Cell::E
                && self.cells[0] == self.cells[3]
                && self.cells[3] == self.cells[6])
            || (self.cells[1] != Cell::E
                && self.cells[1] == self.cells[4]
                && self.cells[4] == self.cells[7])
            || (self.cells[2] != Cell::E
                && self.cells[2] == self.cells[5]
                && self.cells[5] == self.cells[8])
            || (self.cells[0] != Cell::E
                && self.cells[0] == self.cells[4]
                && self.cells[4] == self.cells[8])
            || (self.cells[2] != Cell::E
                && self.cells[2] == self.cells[4]
                && self.cells[4] == self.cells[6])
        {
            self.won = curr_val;
        }
    }

    fn play(&mut self) {
        match self.current_player {
            0 => {
                println!("Player 'X' select a number from 1 - 9 \n");
                self.change_value();
                self.check_cells();
                self.current_player = 1;
            }

            1 => {
                println!("Player 'O' select a number from 1 - 9 \n");
                self.change_value();
                self.check_cells();
                self.current_player = 0;
            }

            _ => {
                println!("Error\n");
            }
        }
    }
}

fn main() {
    let mut game = Game {
        cells: vec![
            Cell::E,
            Cell::E,
            Cell::E,
            Cell::E,
            Cell::E,
            Cell::E,
            Cell::E,
            Cell::E,
            Cell::E,
        ],
        won: Cell::E,
        valid: true,
        current_player: 0,
    };

    println!("{}", game);

    while game.won == Cell::E && game.valid {
        game.play();
    }

    if !game.valid {
        println!("No result for this game")
    } else {
        println!("{} won the game.", game.won)
    }
}
