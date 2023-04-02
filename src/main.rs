mod game_of_life;
use std::collections::HashSet;

use rand::Rng;

use game_of_life::GameOfLife;

fn get_random_initial_state(width: u64, height: u64) -> HashSet<(u64, u64)> {
    let mut rng = rand::thread_rng();
    let alive_cells_count = (width * height) / rng.gen_range(1..7);

    let mut alive_cells: HashSet<(u64, u64)> = HashSet::new();

    for _ in 0..alive_cells_count {
        let y = rng.gen_range(0..height);
        let x = rng.gen_range(0..width);
        alive_cells.insert((y, x));
    }
    alive_cells
}

fn main() {
    let blinker = [(2, 1), (3, 1), (4, 1)];
    let angel = [
        (2, 4),
        (3, 2),
        (3, 3),
        (3, 5),
        (3, 6),
        (4, 3),
        (4, 5),
        (5, 4),
    ];
    let gosper_glider_gun = [
        (30, 20),
        (30, 21),
        (31, 20),
        (31, 21),
        (40, 20),
        (40, 21),
        (40, 22),
        (41, 19),
        (41, 23),
        (42, 18),
        (42, 24),
        (43, 18),
        (43, 24),
        (44, 21),
        (45, 19),
        (45, 23),
        (46, 20),
        (46, 21),
        (46, 22),
        (47, 21),
        (50, 18),
        (50, 19),
        (50, 20),
        (51, 18),
        (51, 19),
        (51, 20),
        (52, 17),
        (52, 21),
        (54, 16),
        (54, 17),
        (54, 21),
        (54, 22),
        (64, 18),
        (64, 19),
        (65, 18),
        (65, 19),
    ];

    let width = 170;
    let height = 50;

    let random = get_random_initial_state(width, height);

    let initial_state = random.iter().cloned().collect();

    let mut game = GameOfLife::new(width, height, initial_state, 150, true);
    for _ in 0..100 {
        game.print();
        game.evolve();
    }
    println!("{}", game);
    // loop {
    //     game.print();
    //     game.evolve();
    // }
}
