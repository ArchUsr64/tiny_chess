#![feature(let_chains)]
use std::{array, cmp::Ordering};

use chess::{Board, ChessMove, Color, MoveGen, Piece, Square};
use macroquad::{
    color,
    miniquad::window::{quit, set_window_size},
    prelude::*,
};

mod engine;

const SQUARE_SIZE: f32 = 100f32;
#[macroquad::main("Chess")]
async fn main() {
    let mut board = Board::default();
    set_window_size((SQUARE_SIZE * 9.) as u32, (SQUARE_SIZE * 9.) as u32);
    let pieces = PiecesAtlas::parse_atlas("res/pieces_atlas.png")
        .await
        .unwrap();
    let mut selected_square = Option::<Square>::None;
    let draw_square = |square: Square, color| {
        draw_rectangle(
            SQUARE_SIZE * square.get_file() as usize as f32,
            SQUARE_SIZE * square.get_rank() as usize as f32,
            SQUARE_SIZE,
            SQUARE_SIZE,
            color,
        );
    };

    loop {
        if is_key_pressed(KeyCode::Escape) {
            quit();
        }
        clear_background(BLACK);
        render_grid();
        pieces.render_board(&board);
        if is_mouse_button_down(MouseButton::Left) {
            if let Some(square) = selected_square.or(square_from_mouse(mouse_position()))
                && let Some(piece) = board.piece_on(square)
                && let Some(Color::White) = board.color_on(square)
            {
                MoveGen::new_legal(&board)
                    .filter(|i| i.get_source() == square)
                    .map(|i| i.get_dest())
                    .for_each(|i| draw_square(i, color::Color { a: 0.5, ..YELLOW }));
                draw_square(square, DARKGRAY);
                draw_texture_ex(
                    pieces.get_chess_piece(piece, Color::White),
                    mouse_position().0 - SQUARE_SIZE / 2f32,
                    mouse_position().1 - SQUARE_SIZE / 2f32,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
                        ..Default::default()
                    },
                );
                selected_square = Some(square);
            };
        }
        if is_mouse_button_released(MouseButton::Left) {
            if let Some(source) = selected_square
                && let Some(dest) = square_from_mouse(mouse_position())
            {
                let chess_move = ChessMove::new(source, dest, None);
                if board.legal(chess_move) {
                    board = board.make_move_new(chess_move);
                    if let Some(engine_move) = engine::next_move(&board) {
                        board = board.make_move_new(engine_move);
                    }
                }
            }
            selected_square = None;
        }
        draw_rectangle_lines(0., 0., SQUARE_SIZE * 9., SQUARE_SIZE * 9., 1., WHITE);
        next_frame().await
    }
}

fn render_grid() {
    for i in 0..8 {
        for j in 0..8 {
            let color = match (i % 2 == 0, j % 2 == 0) {
                (true, true) => WHITE,
                (true, false) => GRAY,
                (false, true) => GRAY,
                (false, false) => WHITE,
            };
            draw_rectangle(
                SQUARE_SIZE * j as f32,
                SQUARE_SIZE * i as f32,
                SQUARE_SIZE,
                SQUARE_SIZE,
                color,
            );
        }
    }
}

struct PiecesAtlas([Texture2D; 12]);

fn square_from_mouse((x, y): (f32, f32)) -> Option<Square> {
    let (x, y) = (x / SQUARE_SIZE, y / SQUARE_SIZE);
    if x.partial_cmp(&8.0) == Some(Ordering::Less) && y.partial_cmp(&8.0) == Some(Ordering::Less) {
        unsafe {
            return Some(Square::new(x as u8 + y as u8 * 8));
        }
    }
    None
}

impl PiecesAtlas {
    /// Expects an image atlas with 8x2 sprites in the format:
    /// kqbkrp/KQBKRP
    async fn parse_atlas(file_path: &str) -> Option<Self> {
        let atlas = load_image(file_path).await.ok()?;
        let piece_size = (atlas.width() as f32 / 6., atlas.height() as f32 / 2.);
        let mut res = array::from_fn(|_| Texture2D::empty());
        for (i, texture) in res.iter_mut().enumerate() {
            *texture = Texture2D::from_image(&atlas.sub_image(Rect {
                x: (i % 6) as f32 * piece_size.0,
                y: (i / 6) as f32 * piece_size.1,
                w: piece_size.0,
                h: piece_size.1,
            }));
        }
        Some(PiecesAtlas(res))
    }
    fn get_chess_piece(&self, piece: Piece, color: Color) -> &Texture2D {
        let idx = match piece {
            Piece::King => 0,
            Piece::Queen => 1,
            Piece::Bishop => 2,
            Piece::Knight => 3,
            Piece::Rook => 4,
            Piece::Pawn => 5,
        } + match color {
            Color::White => 0,
            Color::Black => 6,
        };
        &self.0[idx]
    }
    fn render_board(&self, board: &Board) {
        for i in 0..8 {
            for j in 0..8 {
                let chess_idx = i * 8 + j;
                unsafe {
                    if let Some(piece) = board.piece_on(Square::new(chess_idx)) {
                        if let Some(color) = board.color_on(Square::new(chess_idx)) {
                            draw_texture_ex(
                                self.get_chess_piece(piece, color),
                                SQUARE_SIZE * j as f32,
                                SQUARE_SIZE * i as f32,
                                WHITE,
                                DrawTextureParams {
                                    dest_size: Some(Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
                                    ..Default::default()
                                },
                            )
                        }
                    }
                }
            }
        }
        // Render files
        for (i, char) in "ABCDEFGH".char_indices() {
            draw_text(
                format!("{char}").as_str(),
                i as f32 * SQUARE_SIZE,
                SQUARE_SIZE * 9.,
                SQUARE_SIZE * 2.,
                LIGHTGRAY,
            );
        }
        // Render ranks
        for (i, char) in "01234567".char_indices() {
            draw_text(
                format!("{char}").as_str(),
                SQUARE_SIZE * 8.,
                (i + 1) as f32 * SQUARE_SIZE,
                SQUARE_SIZE * 2.,
                LIGHTGRAY,
            );
        }
    }
}
