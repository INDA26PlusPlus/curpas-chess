# curpas-chess
Hyper-optimised chess API  
```
So it's pretty straight forward.

You want to pass around a board object continuously, which is a vector<vector<bool>>

You can create it with:
    let mut board = vec![vec![false; 64]; 14];
    init(&mut board);

You can find the layer of the piece at a given position 0-63 with:
    piece_at(&board, p as usize)
        * Returns -1 if there is no piece at that position
        * This is your main tool to dispaly the board, fortunately it doesn't have any ridiculous inefficiency so you can call it as much as you want

You can see all the legal moves as integers 0-4095 (start_row * 8 + start_column) * 64 + end_row * 8 + end_column
    let moves = find_legal_moves(&mut board, 0);
        * The last integer can only be 0, but it seems like there are no default arguments in rust, so you have to type it yourself.
        * Also do note this gives all legal moves, for both sides, since the ame doesn't know whose turn it is.

You can turn any of said move into a uci string again with:
    let uci = move_to_uci(3981)

Finally, you can make moves, just put in the UCI notation for it:
    let (result, board) = make_move(board, "c1f4");
        * The result will tell you if the move was legal or not

Unfortunately the only way to tell if there is a checkmate is if the player has no legal moves remaining, which is the same as stalemate.
But stalemate should be a win anyway right?
Also the other draws, insufficient material, 50 move rule, and draw by repetition, are not calculated either.
```
