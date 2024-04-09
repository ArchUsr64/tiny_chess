use chess::{Board, ChessMove, Color, MoveGen, Square};

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

fn evaluate(board: &Board) -> isize {
    (0..64)
        .map(|i| match unsafe { board.color_on(Square::new(i)) } {
            None => 0,
            Some(Color::White) => -1,
            Some(Color::Black) => 1,
        })
        .sum()
}
