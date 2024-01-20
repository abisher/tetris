use crate::tetrimino::{Tetrimino, TetriminoJ, TetriminoI, TetriminoT, TetriminoZ, TetriminoS, TetriminoO, TetriminoL, TetriminoGenerator};


const LEVEL_TIMES: [u32; 10] = [1000, 850, 700, 600, 500, 400, 300, 250,
    221, 190];

const LEVEL_LINES: [u32; 10] = [20, 40, 60, 80, 100, 120, 140, 160,
    180, 200];

type GameMap = Vec<Vec<u8>>;

pub struct Tetris {
    pub game_map: GameMap,
    pub current_level: u32,
    pub score: u32,
    pub nb_lines: u32,
    pub current_piece: Option<Tetrimino>,
}

impl Tetris {
    pub(crate) fn new() -> Tetris {
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

    pub fn create_new_tetrimino(&mut self) -> Tetrimino {
        static mut PREV: u8 = 7;
        let mut rand_rb = rand::random::<u8>() % 7; // TODO: let having no more than two tetrimino(now only one)

        if unsafe { PREV } == rand_rb {
            rand_rb = rand::random::<u8>() % 7;
        }

        unsafe { PREV = rand_rb; }

        match rand_rb {
            0 => TetriminoI::new(),
            1 => TetriminoJ::new(),
            2 => TetriminoL::new(),
            3 => TetriminoO::new(),
            4 => TetriminoS::new(),
            5 => TetriminoZ::new(),
            6 => TetriminoT::new(),
            _ => unreachable!()
        }
    }

    fn check_lines(&mut self) {
        let mut y = 0;
        let mut score_add = 0;

        while y < self.game_map.len() {
            let mut complete = true;

            for x in &self.game_map[y] {
                if *x == 0 {
                    complete = false;
                    break;
                }
            }

            if complete == true {
                score_add += self.current_level;
                self.game_map.remove(y);
                y -= 1;
            }
            y += 1;
        }

        if self.game_map.len() == 0 {
            // A "tetris"!
            score_add += 1000;
        }
        self.update_score(score_add);

        while self.game_map.len() < 16 {
            // ToDo adding score
            self.game_map.insert(0, vec![0; 10])
        }
    }

    pub fn make_permanent(&mut self) {
        let mut to_add = 0;

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
            to_add += self.current_level;
        }

        self.update_score(to_add);
        self.check_lines(); // After Tetrimino becomes permanent check if any line in game map is full
        self.current_piece = None;
    }

    fn update_score(&mut self, to_add: u32) {
        self.score += to_add;
    }

    fn increase_line(&mut self) {
        self.nb_lines += 1;
        if self.nb_lines > LEVEL_LINES[self.current_level as usize - 1] {

        }

    }
}