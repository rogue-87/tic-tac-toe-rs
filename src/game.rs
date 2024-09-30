use rand::Rng;
use std::{io, usize};

#[derive(Debug)]
enum PickError {
    AreaOccupied,
    MovesMapNotInitialized,
    OutOfBounds,
}

enum CheckResult {
    Win,
    Tie,
    Contine,
}

#[derive(Debug)]
struct Score {
    player: u16,
    cpu: u16,
    tie: u16,
}

#[derive(Debug)]
pub struct Game {
    moves_map: Option<[u8; 9]>,
    score: Score,
}

impl Game {
    pub fn new() -> Self {
        Game {
            moves_map: None,
            score: Score {
                player: 0,
                cpu: 0,
                tie: 0,
            },
        }
    }

    pub fn start(&mut self) {
        // Initialize the moves_map with an empty board
        self.moves_map = Some([0; 9]);

        loop {
            println!("Choose index(0 to 8):");
            self.print_info();
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            // Convert the input to an integer, trimming the newline
            let number: usize = input.trim().parse().expect("Please enter a valid number");
            println!("You entered: {}", number);
            match self.pick_player(number) {
                Ok(()) => match self.check(1) {
                    CheckResult::Win => {
                        println!("** You win! **");
                        self.increase_score(1);
                        self.reset();
                        continue;
                    }
                    CheckResult::Tie => {
                        println!("** Tie! **");
                        self.increase_score(0);
                        self.reset();
                        continue;
                    }
                    CheckResult::Contine => {
                        println!("** Cpu turn **");
                    }
                },
                Err(PickError::AreaOccupied) => {
                    println!("That area is already occupied!");
                    continue;
                }
                Err(PickError::OutOfBounds) => {
                    println!("Invalid index!\nMust be between 0 and 8");
                    continue;
                }
                Err(PickError::MovesMapNotInitialized) => println!("The game has not started!"),
            };
            self.pick_cpu();
            match self.check(2) {
                CheckResult::Win => {
                    println!("** Cpu wins! **");
                    self.increase_score(2);
                    self.reset();
                    continue;
                }
                CheckResult::Tie => {
                    println!("** Tie! **");
                    self.increase_score(0);
                    self.reset();
                    continue;
                }
                CheckResult::Contine => {
                    println!("** Your turn **");
                }
            }
        }
    }

    fn increase_score(&mut self, turn: u8) {
        match turn {
            0 => self.score.tie += 1,
            1 => self.score.player += 1,
            2 => self.score.cpu += 1,
            _ => (),
        }
    }

    fn reset(&mut self) {
        self.moves_map = Some([0; 9]);
    }

    fn is_full(&self) -> bool {
        match self.moves_map {
            // Check if the moves_map is Some and contains a value
            Some(moves) => {
                // Iterate over the moves_map and check if all values are non-zero
                for &v in moves.iter() {
                    if v == 0 {
                        return false; // If there's a zero, the board is not full
                    }
                }
                true // If no zeros were found, the board is full
            }
            None => false, // If moves_map is None, the board is considered not full
        }
    }

    fn print_info(&self) {
        match &self.moves_map {
            Some(moves) => {
                for (i, &value) in moves.iter().enumerate() {
                    // Print the values with formatting (width of 3)
                    print!("{:3}", value);

                    // Print a newline after every 3 values (to simulate a 3x3 Tic Tac Toe board)
                    if (i + 1) % 3 == 0 {
                        println!(); // New line after every row
                    }
                }
            }
            None => println!("No moves yet!"), // Handle the case where the moves_map is None
        };
        println!();
        println!("{:?}", &self.score)
    }

    fn pick_cpu(&mut self) {
        let mut rng = rand::thread_rng();
        let index: usize = rng.gen_range(0..=8);
        if self.is_full() {
            return;
        }

        if let Some(map) = &mut self.moves_map {
            match map[index] {
                0 => map[index] = 2,
                _ => {}
            }
        }
    }

    fn pick_player(&mut self, index: usize) -> Result<(), PickError> {
        match index {
            0..=8 => {
                if let Some(map) = &mut self.moves_map {
                    if map[index] == 0 {
                        // Assuming 0 indicates an empty area
                        map[index] = 1; // Assuming 1 is the player's marker
                        Ok(()) // Success
                    } else {
                        Err(PickError::AreaOccupied) // Fail, already occupied
                    }
                } else {
                    Err(PickError::MovesMapNotInitialized) // Fail, moves_map is None
                }
            }
            _ => Err(PickError::OutOfBounds),
        }
    }

    fn check(&mut self, turn: u8) -> CheckResult {
        if let Some(map) = self.moves_map {
            let (mut ptr1, mut ptr2, mut ptr3) = (0, 3, 6);

            // Scan map from left to right
            for _ in 0..=2 {
                if map[ptr1] == turn && map[ptr2] == turn && map[ptr3] == turn {
                    return CheckResult::Win;
                }
                ptr1 += 1;
                ptr2 += 1;
                ptr3 += 1;
            }

            // Scan map from top to bottom
            (ptr1, ptr2, ptr3) = (0, 1, 2);
            for _ in 0..=2 {
                if map[ptr1] == turn && map[ptr2] == turn && map[ptr3] == turn {
                    return CheckResult::Win;
                }
                ptr1 += 3;
                ptr2 += 3;
                ptr3 += 3;
            }

            // Scan map diagonally(both sides)
            (ptr1, ptr2, ptr3) = (0, 4, 8);
            if map[ptr1] == turn && map[ptr2] == turn && map[ptr3] == turn {
                return CheckResult::Win;
            }
            (ptr1, ptr2, ptr3) = (2, 4, 6);
            if map[ptr1] == turn && map[ptr2] == turn && map[ptr3] == turn {
                return CheckResult::Win;
            }

            // Tie
            if self.is_full() {
                return CheckResult::Tie;
            }
        }
        return CheckResult::Contine;
    }
}
