mod board;
mod color;
mod movement;
mod piece;

// use std::mem::discriminant;
// use Color::*;
// use MoveType::*;
// use Piece::*;

// #[derive(Clone, Copy, PartialEq)]
// pub enum Color {
//     White,
//     Black,
// }

// pub enum Piece {
//     King { color: Color, castled: bool },
//     Queen { color: Color },
//     Bishop { color: Color },
//     Knight { color: Color },
//     Rook { color: Color, castled: bool },
//     Pawn { color: Color, just_jumped: bool },
// }

// impl Piece {
//     pub fn color(&self) -> Color {
//         match self {
//             Self::King { color, castled } => *color,
//             Self::Queen { color } => *color,
//             Self::Bishop { color } => *color,
//             Self::Knight { color } => *color,
//             Self::Rook { color, castled } => *color,
//             Self::Pawn { color, just_jumped } => *color,
//         }
//     }

//     pub fn get_movement_vecs(&self) -> Vec<(i8, i8)> {
//         match self {
//             King { color, castled } => {}
//         }
//     }

//     // Used to check if the Travel and Capture variant of Move need a legal move (start pos is only use to determinate if a pawn can jump)
//     pub fn is_movement_possible(
//         &self,
//         movement_vec: (i8, i8),
//         start_pos: Pos,
//         board: Board,
//         movement: Move,
//     ) -> bool {
//         if movement_vec.0 == 0 && movement_vec.1 == 0 {
//             return false;
//         }
//         match self {
//             King { color, castled } => {
//                 if std::cmp::max(movement_vec.0.abs(), movement_vec.1.abs()) == 1 {
//                     return true;
//                 }
//             }
//             Queen { color } => {
//                 if (movement_vec.0 == 0 || movement_vec.1 == 0)
//                     || movement_vec.0.abs() == movement_vec.1.abs()
//                 {
//                     return true;
//                 }
//             }
//             Bishop { color } => {
//                 if movement_vec.0.abs() == movement_vec.1.abs() {
//                     return true;
//                 }
//             }
//             Knight { color } => {
//                 if (movement_vec.0.abs() == 2 && movement_vec.1.abs() == 1)
//                     || (movement_vec.0.abs() == 1 && movement_vec.1.abs() == 2)
//                 {
//                     return true;
//                 }
//             }
//             Rook { color, castled } => {
//                 if movement_vec.0 == 0 || movement_vec.1 == 0 {
//                     return true;
//                 }
//             }
//             Pawn { color, just_jumped } => match color {
//                 White => {
//                     if movement_vec.0 == 0 {
//                         if movement_vec.1 == 1 {
//                             return true;
//                         }
//                         if start_pos.y == 1 && movement_vec.1 == 2 {
//                             return true;
//                         }
//                     }
//                 }
//                 Black => {
//                     if movement_vec.0 == 0 {
//                         if movement_vec.1 == -1 {
//                             return true;
//                         }
//                         if start_pos.y == 6 && movement_vec.1 == -2 {
//                             return true;
//                         }
//                     }
//                 }
//             },
//         }
//         return false;
//     }

//     pub fn king() -> Piece {
//         King {
//             color: White,
//             castled: false,
//         }
//     }

//     pub fn queen() -> Piece {
//         Queen { color: White }
//     }

//     pub fn bishop() -> Piece {
//         Bishop { color: White }
//     }

//     pub fn knight() -> Piece {
//         Knight { color: White }
//     }

//     pub fn rook() -> Piece {
//         Rook {
//             color: White,
//             castled: false,
//         }
//     }

//     pub fn pawn() -> Piece {
//         Pawn {
//             color: White,
//             just_jumped: false,
//         }
//     }
// }

// fn is_trajectory_blocked(movement_vec: (i8, i8), start_pos: Pos, board: Board) -> bool {
//     let normalized_vec = (
//         movement_vec.0 / movement_vec.0.abs(),
//         movement_vec.1 / movement_vec.1.abs(),
//     ); // Not really a normalized vector, but it does the job
//     let mut current_pos = (start_pos.x, start_pos.y);
//     for _ in 0..movement_vec.0.abs() {
//         current_pos.0 += normalized_vec.0;
//         current_pos.1 += normalized_vec.1;
//         match board[current_pos.0][current_pos.1] {
//             Some(_) => {
//                 return false;
//             }
//             None => {
//                 continue;
//             }
//         }
//     }
//     return true;
// }

// pub enum MoveType {
//     Travel,
//     Capture { gain: Piece, en_passant: bool },
//     Castle,
//     Promotion(Piece),
// }

// // Returned when computing all possible moves, provide the gain, type of move, start pos and end pos
// pub struct Move {
//     pub move_type: MoveType,
//     pub moving_piece: Piece,
//     pub start_pos: Pos,
//     pub end_pos: Pos,
// }

// impl Move {
//     pub fn get_movement_vec(&self) -> (i8, i8) {
//         return (
//             self.end_pos.x as i8 - self.start_pos.x as i8,
//             self.end_pos.y as i8 - self.start_pos.y as i8,
//         );
//     }

//     // Most global way of checking if a Move is legal
//     pub fn is_legal(&self, board: Board) -> bool {
//         let movement_vec = self.get_movement_vec();
//         // First check the trait
//         if self.moving_piece.color() != board.trait_color {
//             return false;
//         }
//         // Then end pos
//         if !self.end_pos.exists() {
//             return false;
//         }
//         if movement_vec.0 == 0 && movement_vec.1 == 0 {
//             return false;
//         }
//         match self.moving_piece {
//             King { color, castled } => match self.move_type {
//                 Travel => {
//                     if !self
//                         .moving_piece
//                         .is_movement_possible(movement_vec, self.start_pos, board)
//                     {
//                         return false;
//                     }
//                 }
//                 Capture { gain, en_passant } => {
//                     if en_passant == true {
//                         return false;
//                     }
//                     if !self
//                         .moving_piece
//                         .is_movement_possible(movement_vec, self.start_pos, board)
//                     {
//                         return false;
//                     }
//                 }
//                 Castle => {
//                     if castled == true {
//                         return false;
//                     }
//                     // Short castle
//                     if movement_vec.0 == 2 {
//                         let rook_pos = Pos::new(7, self.start_pos.y);
//                         if let Rook {
//                             color,
//                             castled: rook_castled,
//                         } = board.content[rook_pos.x][rook_pos.y]
//                         {
//                             if rook_castled == true {
//                                 return false;
//                             }
//                         }
//                     }
//                     // Long castle
//                     if movement_vec.0 == -2 {
//                         let rook_pos = Pos::new(0, self.start_pos.y);
//                         if let Rook {
//                             color,
//                             castled: rook_castled,
//                         } = board.content[rook_pos.x][rook_pos.y]
//                         {
//                             if rook_castled == true {
//                                 return false;
//                             }
//                         }
//                     }
//                 }
//                 Promotion(_) => {
//                     return false;
//                 }
//             },
//             Queen { color } => match self.move_type {
//                 Travel => {
//                     if !self
//                         .moving_piece
//                         .is_movement_possible(movement_vec, self.start_pos, board)
//                     {
//                         return false;
//                     }
//                 }
//                 Capture { gain, en_passant } => {
//                     if en_passant == true {
//                         return false;
//                     }
//                     if !self
//                         .moving_piece
//                         .is_movement_possible(movement_vec, self.start_pos, board)
//                     {
//                         return false;
//                     }
//                 }
//                 Castle => {
//                     return false;
//                 }
//                 Promotion(_) => {
//                     return false;
//                 }
//             },
//             Bishop { color } => match self.move_type {
//                 Travel => {
//                     if !self
//                         .moving_piece
//                         .is_movement_possible(movement_vec, self.start_pos, board)
//                     {
//                         return false;
//                     }
//                     if is_trajectory_blocked(movement_vec, self.start_pos, board) {
//                         return false;
//                     }
//                 }
//                 Capture { gain, en_passant } => {
//                     if en_passant == true {
//                         return false;
//                     }
//                     if !self
//                         .moving_piece
//                         .is_movement_possible(movement_vec, self.start_pos, board)
//                     {
//                         return false;
//                     }
//                 }
//                 Castle => {
//                     return false;
//                 }
//                 Promotion(_) => {
//                     return false;
//                 }
//             },
//             Knight { color } => match self.move_type {
//                 Travel => {
//                     if !self
//                         .moving_piece
//                         .is_movement_possible(movement_vec, self.start_pos, board)
//                     {
//                         return false;
//                     }
//                 }
//                 Capture { gain, en_passant } => {
//                     if en_passant == true {
//                         return false;
//                     }
//                     if !self
//                         .moving_piece
//                         .is_movement_possible(movement_vec, self.start_pos, board)
//                     {
//                         return false;
//                     }
//                 }
//                 Castle => {
//                     return false;
//                 }
//                 Promotion(_) => {
//                     return false;
//                 }
//             },
//             Rook { color, castled } => match self.move_type {
//                 Travel => {
//                     if !self
//                         .moving_piece
//                         .is_movement_possible(movement_vec, self.start_pos, board)
//                     {
//                         return false;
//                     }
//                 }
//                 Capture { gain, en_passant } => {
//                     if en_passant == true {
//                         return false;
//                     }
//                     if !self
//                         .moving_piece
//                         .is_movement_possible(movement_vec, self.start_pos, board)
//                     {
//                         return false;
//                     }
//                 }
//                 Castle => {
//                     return false;
//                 }
//                 Promotion(_) => {
//                     return false;
//                 }
//             },
//         }
//     }
// }

// pub struct Pos {
//     pub x: u8,
//     pub y: u8,
// }

// impl Pos {
//     pub fn new(x: u8, y: u8) -> Pos {
//         Pos { x, y }
//     }

//     pub fn exists(&self) -> bool {
//         return (self.x >= 0 && self.x < 8) && (self.y >= 0 && self.y < 8);
//     }
// }

// pub struct Board {
//     pub content: [[Option<Piece>; 8]; 8], // Column, line
//     pub trait_color: Color,
//     pub turn_number: u32,
// }

// impl Board {
//     fn get_moves(&self, piece: Piece, pos: Pos) -> Vec<Move> {
//         vec![]
//     }

//     fn get_pieces(&self, piece: Piece) -> Vec<(Piece, Pos)> {
//         let mut matching_pieces: Vec<(Piece, Pos)> = Vec::<(Piece, Pos)>::new();
//         for i in 0..8 {
//             for j in 0..8 {
//                 if let Some(current_piece) = self.content[i][j] {
//                     if discriminant(&current_piece) == discriminant(&piece)
//                         && current_piece.color() == self.trait_color
//                     {
//                         // We push a Piece only if it's the same and if the color match the trait
//                         matching_pieces.push((current_piece, Pos::new(i as u8, j as u8)));
//                     }
//                 }
//             }
//         }
//         return matching_pieces;
//     }

//     pub fn new() -> Board {
//         Board {
//             content: [
//                 [
//                     Some(Rook {
//                         color: White,
//                         castled: false,
//                     }),
//                     Some(Pawn {
//                         color: White,
//                         just_jumped: false,
//                     }),
//                     None,
//                     None,
//                     None,
//                     None,
//                     Some(Pawn {
//                         color: Black,
//                         just_jumped: false,
//                     }),
//                     Some(Rook {
//                         color: Black,
//                         castled: false,
//                     }),
//                 ],
//                 [
//                     Some(Knight { color: White }),
//                     Some(Pawn {
//                         color: White,
//                         just_jumped: false,
//                     }),
//                     None,
//                     None,
//                     None,
//                     None,
//                     Some(Pawn {
//                         color: Black,
//                         just_jumped: false,
//                     }),
//                     Some(Knight { color: Black }),
//                 ],
//                 [
//                     Some(Bishop { color: White }),
//                     Some(Pawn {
//                         color: White,
//                         just_jumped: false,
//                     }),
//                     None,
//                     None,
//                     None,
//                     None,
//                     Some(Pawn {
//                         color: Black,
//                         just_jumped: false,
//                     }),
//                     Some(Bishop { color: Black }),
//                 ],
//                 [
//                     Some(Queen { color: White }),
//                     Some(Pawn {
//                         color: White,
//                         just_jumped: false,
//                     }),
//                     None,
//                     None,
//                     None,
//                     None,
//                     Some(Pawn {
//                         color: Black,
//                         just_jumped: false,
//                     }),
//                     Some(Queen { color: Black }),
//                 ],
//                 [
//                     Some(King {
//                         color: White,
//                         castled: false,
//                     }),
//                     Some(Pawn {
//                         color: White,
//                         just_jumped: false,
//                     }),
//                     None,
//                     None,
//                     None,
//                     None,
//                     Some(Pawn {
//                         color: Black,
//                         just_jumped: false,
//                     }),
//                     Some(King {
//                         color: Black,
//                         castled: false,
//                     }),
//                 ],
//                 [
//                     Some(Bishop { color: White }),
//                     Some(Pawn {
//                         color: White,
//                         just_jumped: false,
//                     }),
//                     None,
//                     None,
//                     None,
//                     None,
//                     Some(Pawn {
//                         color: Black,
//                         just_jumped: false,
//                     }),
//                     Some(Bishop { color: Black }),
//                 ],
//                 [
//                     Some(Knight { color: White }),
//                     Some(Pawn {
//                         color: White,
//                         just_jumped: false,
//                     }),
//                     None,
//                     None,
//                     None,
//                     None,
//                     Some(Pawn {
//                         color: Black,
//                         just_jumped: false,
//                     }),
//                     Some(Knight { color: Black }),
//                 ],
//                 [
//                     Some(Rook {
//                         color: White,
//                         castled: false,
//                     }),
//                     Some(Pawn {
//                         color: White,
//                         just_jumped: false,
//                     }),
//                     None,
//                     None,
//                     None,
//                     None,
//                     Some(Pawn {
//                         color: Black,
//                         just_jumped: false,
//                     }),
//                     Some(Rook {
//                         color: Black,
//                         castled: false,
//                     }),
//                 ],
//             ],
//             trait_color: White,
//             turn_number: 0,
//         }
//     }
// }
