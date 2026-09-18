use chess::*;

fn main() {
    let mut board = vec![vec![false; 64]; 14];
    init(&mut board);
    let (_, board) = make_move(board, "f2f3");
    let (_, board) = make_move(board, "e7e5");
    let (_, board) = make_move(board, "g2g4");
    let (r, mut board) = make_move(board, "d8h4");
    let mut _moves = find_legal_moves(&mut board, 0);
    for m in _moves {
        if piece_at(&board, (m / 64) as usize) < 6 {
            println!("white {}", m);
        }
        else {
            println!("black {}", m);
        }
    }
    print_board(&board);
    println!("{}", r);


    let (_, board) = make_move(board, "h7h5");
    let (_, board) = make_move(board, "h4h2");
    let (_, board) = make_move(board, "h5h4");
    let (_, board) = make_move(board, "h4h3");
    let (_, board) = make_move(board, "h2g2");
    let (_, board) = make_move(board, "h3h2");
    let (r, board) = make_move(board, "h2g1r");
    print_board(&board);
    println!("{}", r)
}