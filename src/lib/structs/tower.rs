use super::general;
use super::strait;
pub fn find_moves(piece: &mut general::Piece, board: &general::Board) {
    piece.moves = vec![];
    strait::find_moves(piece, &board);
}
