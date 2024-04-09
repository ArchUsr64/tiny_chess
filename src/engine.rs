use std::isize;

use chess::{Board, ChessMove, Color, MoveGen, Piece, Square};

const ENGINE_PLAYER: Color = Color::Black;

pub fn next_move(board: &Board) -> Option<ChessMove> {
    mini_max(board, 4, ENGINE_PLAYER)
}

pub fn mini_max(board: &Board, depth: usize, player: Color) -> Option<ChessMove> {
    let movegen = MoveGen::new_legal(&board);
    let scores = movegen.map(|mov| {
        let score = if depth == 0 {
            evaluate(&board.make_move_new(mov))
        } else {
            mini_max(&board.make_move_new(mov), depth - 1, !player)
                .map(|best_move| evaluate(&board.make_move_new(best_move)))
                .unwrap_or(if player == ENGINE_PLAYER {
                    isize::MAX
                } else {
                    isize::MIN
                })
        };
        (score, mov)
    });
    if player == ENGINE_PLAYER {
        scores.max_by_key(|(score, _mov)| *score)
    } else {
        scores.min_by_key(|(score, _mov)| *score)
    }
    .map(|(_, mov)| mov)
}

const fn piece_weight(piece: Piece) -> isize {
    match piece {
        Piece::Pawn => 1,
        Piece::Knight => 3,
        Piece::Bishop => 3,
        Piece::Rook => 5,
        Piece::Queen => 9,
        Piece::King => 0,
    }
}

fn evaluate(board: &Board) -> isize {
    (0..64)
        .map(|i| unsafe { Square::new(i) })
        .map(|i| {
            board
                .color_on(i)
                .map(|color| match color {
                    ENGINE_PLAYER => 1,
                    _ => -1,
                })
                .unwrap_or(0)
                * board
                    .piece_on(i)
                    .map(|piece| piece_weight(piece))
                    .unwrap_or(0)
        })
        .sum()
}
