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

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    X,
    O,
    Empty,
}

#[derive(Debug)]
struct Score {
    player: u16,
    cpu: u16,
    tie: u16,
}

#[derive(Debug)]
pub struct Game {
    moves_map: Option<[State; 9]>,
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
        self.moves_map = Some([State::Empty; 9]);

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
                Ok(()) => match self.check(State::X) {
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
            match self.check(State::O) {
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
        self.moves_map = Some([State::Empty; 9]);
    }

    fn is_full(&self) -> bool {
        match self.moves_map {
            Some(moves) => moves.iter().all(|&v| v != State::Empty),
            None => false,
        }
    }

    fn print_info(&self) {
        match &self.moves_map {
            Some(moves) => {
                for (i, &val) in moves.iter().enumerate() {
                    let symbol = match val {
                        State::X => "X",
                        State::O => "O",
                        State::Empty => ".",
                    };
                    print!("{:3}", symbol);
                    if (i + 1) % 3 == 0 {
                        println!();
                    }
                }
            }
            None => println!("No moves yet!"),
        };
        println!("{:?}", &self.score)
    }

    fn pick_cpu(&mut self) {
        loop {
            let mut rng = rand::thread_rng();
            let index: usize = rng.gen_range(0..=8);
            if self.is_full() {
                return;
            }

            if let Some(map) = &mut self.moves_map {
                if map[index] == State::Empty {
                    map[index] = State::O;
                    break;
                }
            }
        }
    }

    fn pick_player(&mut self, index: usize) -> Result<(), PickError> {
        match index {
            0..=8 => {
                if let Some(map) = &mut self.moves_map {
                    if map[index] == State::Empty {
                        map[index] = State::X;
                        Ok(())
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

    fn check(&mut self, state: State) -> CheckResult {
        if let Some(map) = self.moves_map {
            let (mut ptr1, mut ptr2, mut ptr3) = (0, 3, 6);

            // Scan map from left to right
            for _ in 0..=2 {
                if map[ptr1] == state && map[ptr2] == state && map[ptr3] == state {
                    return CheckResult::Win;
                }
                ptr1 += 1;
                ptr2 += 1;
                ptr3 += 1;
            }

            // Scan map from top to bottom
            (ptr1, ptr2, ptr3) = (0, 1, 2);
            for _ in 0..=2 {
                if map[ptr1] == state && map[ptr2] == state && map[ptr3] == state {
                    return CheckResult::Win;
                }
                ptr1 += 3;
                ptr2 += 3;
                ptr3 += 3;
            }

            // Scan map diagonally(both sides)
            (ptr1, ptr2, ptr3) = (0, 4, 8);
            if map[ptr1] == state && map[ptr2] == state && map[ptr3] == state {
                return CheckResult::Win;
            }
            (ptr1, ptr2, ptr3) = (2, 4, 6);
            if map[ptr1] == state && map[ptr2] == state && map[ptr3] == state {
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
