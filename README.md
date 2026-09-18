# curpas-chess
Hyper-optimised chess API

So it's pretty straight forward.
You want to pass around a board object continuously, which is a vector< vector< bool > >

You can create it with:
    let mut board = vec![vec![false; 64]; 14];
    init(&mut board);

You can print all of its layers with:
    print_board(&board);
        * Each layer is a boolean mask of every instance a piece of that layers type. See the labels attached when calling print_board

You can find the layer the piece at a given position 0-63 with:
    piece_at(&board, p as usize)

You can see all the legal moves as integers 0-4095 (start_row * 8 + start_column) * 64 + end_row * 8 + end_column
    let moves = find_legal_moves(&mut board, 0);
        * The last integer can only be 0, but it seems like there are no default arguments in rust, so you have to type it yourself.
        * Also do note this gives all legal moves, for both sides, since it doesn't know whose turn it is.

Finally, you can make moves, just put in the UCI notation for it:
    let (result, board) = make_move(board, "c1f4");
        * The result will tell you if the move was legal or not

Unfortunately it can't tell the difference between stalemate and checkmate, they both just mean no legal moves, this system certainly has its downsides.
But stalemate should be a win anyway right?
