use sdl2::libc::ip_mreq_source;
use crate::tetrimino::Tetrimino;

type GameMap = Vec<Vec<u8>>;

struct Tetris {
    game_map: GameMap,
    current_level: u32,
    score: u32,
    nb_lines: u32,
    current_piece: Option<Tetrimino>,
}

impl Tetris {
    fn new() -> Tetris {
        let mut game_map = Vec::new();

        for _ in 0..16 {
            game_map.push(vec![0; 10]);
        }
        Tetris {
            game_map,
            current_level: 1,
            score: 0,
            nb_lines: 0,
            current_piece: None,
        }
    }

    fn check_lines(&mut self) {
        let mut y = 0;

        while y < self.game_map.len() {
            let mut complete = true;

            for x in &self.game_map[y] {
                if *x == 0 {
                    complete = false;
                    break;
                }
            }

            if complete == true {
                self.game_map.remove(y);
                y -= 1;
            }
            y += 1;
        }

        while self.game_map.len() < 16 {
            self.game_map.insert(0, vec![0; 10])
        }
    }

    fn make_permanent(&mut self) {
        if let Some(ref mut piece) = self.current_piece {
            let mut shift_y = 0;

            // check if we are not ran out of Tetrimino boundary or game map at Y axis
            while shift_y < piece.states[piece.current_state as usize].len() &&
                piece.y + shift_y < self.game_map.len() {
                let mut shift_x = 0;

                // same for X axis
                while shift_x < piece.states[piece.current_state as usize][shift_y].len() &&
                    (piece.x + shift_x as isize) < self.game_map[piece.y + shift_y].len() as isize {
                    if piece.states[piece.current_state as usize][shift_y][shift_x] != 0 {
                        let x = piece.x + shift_x as isize;
                        self.game_map[piece.y + shift_y][x as usize] =
                            piece.states[piece.current_state as usize][shift_y][shift_x];
                    }
                    shift_x += 1;
                }
                shift_y += 1;
            }
        }
        self.check_lines(); // After Tetrimino becomes permanent check if any line in game map in full
        self.current_piece = None;
    }
}


