use std::collections::HashSet;
use std::fmt::Display;
use std::process::Command;
use std::{thread, time};

pub struct GameOfLife {
    width: u64,
    height: u64,
    generation: usize,
    alive_hashset: HashSet<(u64, u64)>,
    tickrate_ms: u64,
    use_tickrate: bool,
}

impl GameOfLife {
    pub fn new(
        width: u64,
        height: u64,
        alive_hashset: HashSet<(u64, u64)>,
        tickrate_ms: u64,
        use_tickrate: bool,
    ) -> GameOfLife {
        GameOfLife {
            generation: 1,
            width,
            height,
            alive_hashset,
            tickrate_ms,
            use_tickrate,
        }
    }

    pub fn print(&self) {
        Command::new("clear").status();
        println!("Generation: {}", self.generation);
        println!(
            "Cells: {}/{}",
            self.alive_hashset.len(),
            self.width * self.height
        );
        for y in 0..self.height {
            for x in 0..self.width {
                if self.alive_hashset.contains(&(y, x)) {
                    print!("x");
                } else {
                    print!(".")
                }
            }
            println!();
        }
    }

    pub fn evolve(&mut self) {
        let mut next_alive_cells: HashSet<(u64, u64)> = HashSet::new();
        let mut cells_to_check: HashSet<(u64, u64)> = HashSet::new();

        self.alive_hashset
            .iter()
            .for_each(|cell| cells_to_check.extend(self.get_neighbours(cell).into_iter()));

        for cell in cells_to_check {
            let alive_neighbours = self.count_alive_neighbours(cell);
            if alive_neighbours == 3
                || (alive_neighbours == 2 && self.alive_hashset.contains(&cell))
            {
                next_alive_cells.insert(cell);
            }
        }

        self.alive_hashset = next_alive_cells;
        self.generation += 1;

        if self.use_tickrate {
            thread::sleep(time::Duration::from_millis(self.tickrate_ms));
        }
    }

    fn get_neighbours(&self, cell: &(u64, u64)) -> Vec<(u64, u64)> {
        let offsets = [
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
        ];

        offsets
            .iter()
            .map(|&(dx, dy)| {
                (
                    cell.0.wrapping_add(dx as u64),
                    cell.1.wrapping_add(dy as u64),
                )
            })
            .collect()
    }

    fn count_alive_neighbours(&self, cell: (u64, u64)) -> u8 {
        let mut count: u8 = 0;
        for neighbour in self.get_neighbours(&cell) {
            if self.alive_hashset.contains(&neighbour) {
                count += 1;
            }
        }
        count
    }
}

impl Display for GameOfLife {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "GameOfLife({}, {}, {}, {}, {}, {})",
            self.width,
            self.height,
            self.generation,
            self.alive_hashset.len(),
            self.tickrate_ms,
            self.use_tickrate
        )
    }
}
