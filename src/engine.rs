use chess::{Board, ChessMove, Color, MoveGen, Piece, Square};

pub fn next_move(board: &Board) -> Option<ChessMove> {
    let movegen = MoveGen::new_legal(&board);
    movegen
        .map(|mov| {
            let score = evaluate(&board.make_move_new(mov));
            (score, mov)
        })
        .max_by_key(|(score, _mov)| *score)
        .map(|(_, mov)| mov)
}

const fn piece_weight(piece: Piece) -> isize {
    match piece {
        Piece::Pawn => 1,
        Piece::Knight => 3,
        Piece::Bishop => 3,
        Piece::Rook => 5,
        Piece::Queen => 9,
        Piece::King => 20,
    }
}

fn evaluate(board: &Board) -> isize {
    (0..64)
        .map(|i| unsafe { Square::new(i) })
        .map(|i| {
            board
                .color_on(i)
                .map(|color| match color {
                    Color::White => -1,
                    Color::Black => 1,
                })
                .unwrap_or(0)
                * board
                    .piece_on(i)
                    .map(|piece| piece_weight(piece))
                    .unwrap_or(0)
        })
        .sum()
}
