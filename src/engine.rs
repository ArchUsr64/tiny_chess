use chess::{Board, ChessMove, MoveGen};
use macroquad::rand::rand;

pub fn next_move(board: &Board) -> Option<ChessMove> {
    let mut movegen = MoveGen::new_legal(&board);
    movegen.nth(rand() as usize % movegen.len())
}
