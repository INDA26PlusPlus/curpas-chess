use chess::*;

#[cfg(test)]
mod tests {
    use super::*;

    fn perft(mut board: Vec<Vec<bool>>, depth: i32) -> i32 {
        if depth == 0 {
            return 1;
        }
        let mut n = 0;
        for m in find_legal_moves(&mut board, 0) {
            let (_, new_board) = make_move(board.clone(), move_to_uci(m).as_str());
            n = n + perft(new_board, depth - 1);
        }
        n
    }

    #[test]
    fn test1() {
        let mut board = vec![vec![false; 64]; 14];
        init(&mut board);
        // These are the moves in the immortal game
        let moves = vec!["e2e4", "e7e5", "f2f4", "e5f4", "f1c4", "d8h4", "e1f1", "b7b5", "c4b5", "g8f6", "g1f3", "h4h6", "d2d3", "f6h5", "f3h4", "h6g5", "h4f5", "c7c6", "g2g4", "h5f6", "h1g1", "c6b5", "h2h4", "g5g6", "h4h5", "g6g5", "d1f3", "f6g8", "c1f4", "g5f6", "b1c3", "f8c5", "c3d5", "f6b2", "f4d6", "c5g1", "e4e5", "b2a1", "f1e2", "b8a6", "f5g7", "e8d8", "f3f6", "g8f6", "d6e7"];
        for m in moves {
            let (r, b) = make_move(board, m);
            assert_eq!(r, true);
            board = b;
        }
    }

    #[test]
    fn test2() {
        let mut board = vec![vec![false; 64]; 14];
        init(&mut board);
        let n = perft(board, 1); // Runs in ~10s
        assert_eq!(n, 40) // Note that my move generator does not include a turn, and so this is all moves for black and white
    }

    #[test]
    fn test3() {
        let mut board = vec![vec![false; 64]; 14];
        init(&mut board);
        let n = perft(board, 2); // Runs in ~ 450s
        assert_eq!(n, 1690) // That of course applies recursively, hence the very different values
    }
}

fn main() {
    // Some example function usage
    let mut board = vec![vec![false; 64]; 14];
    init(&mut board);
    let lm = find_legal_moves(&mut board, 0);
    println!("Legal moves:");
    for m in lm {
        if piece_at(&board, (m / 64) as usize) < 6 {
            println!("white {}", m);
        }
        else {
            println!("black {}", m);
        }
    }
    let (r, _new_board) = make_move(board, "e2e4");
    if r {
        println!("moved");
    }
    else {
        println!("invalid move");
    }
}