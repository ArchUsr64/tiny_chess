use std::isize;

use chess::{Board, ChessMove, Color, MoveGen, Piece, Square};

const ENGINE_PLAYER: Color = Color::Black;

pub fn next_move(board: &Board) -> Option<ChessMove> {
    mini_max(board, 6, isize::MIN, isize::MAX, ENGINE_PLAYER).0
}

pub fn mini_max(
    board: &Board,
    depth: usize,
    alpha: isize,
    beta: isize,
    player: Color,
) -> (Option<ChessMove>, isize) {
    let (mut alpha, mut beta) = (alpha, beta);
    if depth == 0 {
        return (None, evaluate(board));
    }
    let movegen = MoveGen::new_legal(board);
    if player == ENGINE_PLAYER {
        let (mut best_move, mut best_score) = (None, isize::MIN);
        for mov in movegen {
            let (_, score) = mini_max(&board.make_move_new(mov), depth - 1, alpha, beta, !player);
            if score >= best_score {
                best_move = Some(mov);
            }
            best_score = best_score.max(score);
            alpha = alpha.max(best_score);
            if beta <= alpha {
                break;
            }
        }
        (best_move, best_score)
    } else {
        let (mut best_move, mut best_score) = (None, isize::MAX);
        for mov in movegen {
            let (_, score) = mini_max(&board.make_move_new(mov), depth - 1, alpha, beta, !player);
            if score <= best_score {
                best_move = Some(mov);
            }
            best_score = best_score.min(score);
            beta = beta.min(best_score);
            if beta <= alpha {
                break;
            }
        }
        (best_move, best_score)
    }
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
