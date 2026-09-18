fn uci_to_move(s: &str) -> i32 {
    let mut o: i32 = -1;
    if s.len() == 2 {
        let p = match s.chars().nth(1).unwrap() {
            '1' => 7,
            '2' => 6,
            '3' => 5,
            '4' => 4,
            '5' => 3,
            '6' => 2,
            '7' => 1,
            '8' => 0,
            _ => panic!()
        };
        o = match s.chars().nth(0).unwrap() {
            'a' => p*8 + 0,
            'b' => p*8 + 1,
            'c' => p*8 + 2,
            'd' => p*8 + 3,
            'e' => p*8 + 4,
            'f' => p*8 + 5,
            'g' => p*8 + 6,
            'h' => p*8 + 7,
            _ => panic!()
        };
    }
    o
}

pub fn move_to_uci(n: i32) -> String {
    let start = n / 64;
    let start_row = start / 8;
    let start_col = start % 8;
    let end = n % 64;
    let end_row = end / 8;
    let end_col = end % 8;

    let r1 = match start_row {
        0 => '8',
        1 => '7',
        2 => '6',
        3 => '5',
        4 => '4',
        5 => '3',
        6 => '2',
        7 => '1',
        _ => panic!(),
    };


    let r2 = match end_row {
        0 => '8',
        1 => '7',
        2 => '6',
        3 => '5',
        4 => '4',
        5 => '3',
        6 => '2',
        7 => '1',
        _ => panic!(),
    };

    let c1 = match start_col {
        0 => 'a',
        1 => 'b',
        2 => 'c',
        3 => 'd',
        4 => 'e',
        5 => 'f',
        6 => 'g',
        7 => 'h',
        _ => panic!(),
    };

    let c2 = match end_col {
        0 => 'a',
        1 => 'b',
        2 => 'c',
        3 => 'd',
        4 => 'e',
        5 => 'f',
        6 => 'g',
        7 => 'h',
        _ => panic!(),
    };

    format!("{}{}{}{}", c1, r1, c2, r2)
}

pub fn piece_at(board: &Vec<Vec<bool>>, sq: usize) -> i32 {
    let mut p: i32 = -1;
    for l in 0..12usize {
        if board[l][sq] {
            p = l as i32;
            break;
        }
    }
    p
}

fn path_clear(board: &Vec<Vec<bool>>, start: usize, end: usize) -> bool {
    let start_row = (start / 8) as i32;
    let start_col = (start % 8) as i32;
    let end_row = (end / 8) as i32;
    let end_col = (end % 8) as i32;
    let dr = (end_row - start_row).signum();
    let dc = (end_col - start_col).signum();
    let mut r = start_row + dr;
    let mut c = start_col + dc;
    while r != end_row || c != end_col {
        let sq = (r * 8 + c) as usize;
        if piece_at(board, sq) != -1 {
            return false;
        }
        r += dr;
        c += dc;
    }
    true
}

fn is_valid_target(p: i32, k: usize) -> bool {
    if p == -1 {
        return true;
    }
    if k < 6 {
        p > 5
    } else {
        p < 6
    }
}

pub fn find_legal_moves(board: &mut Vec<Vec<bool>>, bothwhiteblack: i32) -> Vec<i32> {
    // This is the meaty function. In order to make a move on the board we simply check if it exists among all legal moves.
    // However, the way this is done is exceptionally algorithmically inefficient. Now, you could optimise it, but where's the fun in that?

    let mut legal_moves: Vec<i32> = Vec::new();
    for i in 0..64usize {
        for k in 0..12usize {
            if board[k][i] {
                // Now that we found something here, we can check if it has any moves
                if k < 6 && bothwhiteblack == 2 {
                    break;
                }
                if k > 5 && bothwhiteblack == 1 {
                    break;
                }
                for j in 0..64usize {
                    let p = piece_at(board, j);

                    let row_i = (i / 8) as i32;
                    let col_i = (i % 8) as i32;
                    let row_j = (j / 8) as i32;
                    let col_j = (j % 8) as i32;
                    let dr = row_j - row_i;
                    let dc = col_j - col_i;

                    let mut in_check = false;
                    if bothwhiteblack == 0 {
                        let mut board_2 = board.clone();
                        for l2 in 0..12 {
                            board_2[l2][j] = false;
                        }
                        board_2[k][i] = false;
                        board_2[k][j] = true;
                        let kl = if k < 6 { 5usize } else { 11usize };
                        let bwb2 = if k < 6 { 2 } else { 1 };
                        let mut king = 0;
                        for tile in 0..64usize {
                            if board_2[kl][tile] {
                                king = tile;
                                break;
                            }
                        }

                        let moves = find_legal_moves(&mut board_2, bwb2);
                        for o in moves {
                            if o % 64 == king as i32 {
                                in_check = true;
                                break;
                            }
                        }                   
                    }

                    if k == 0 {
                        // White pawn
                        if dc == 0 && dr == -1 && p == -1 {
                            if !in_check { legal_moves.push((i * 64 + j) as i32); }
                        } else if dc == 0 && dr == -2 && row_i == 6 && p == -1 && piece_at(board, i - 8) == -1 {
                            if !in_check { legal_moves.push((i * 64 + j) as i32); }
                        } else if (dc == 1 || dc == -1) && dr == -1 && p > 5 {
                            if !in_check { legal_moves.push((i * 64 + j) as i32); }
                        }
                    }
                    if k == 1 {
                        // White rook
                        if ((dr == 0) != (dc == 0)) && path_clear(board, i, j) && is_valid_target(p, k) {
                            if !in_check { legal_moves.push((i * 64 + j) as i32); }
                        }
                    }
                    if k == 2 {
                        // White knight
                        if matches!((dr.abs(), dc.abs()), (1, 2) | (2, 1)) && is_valid_target(p, k) {
                            if !in_check { legal_moves.push((i * 64 + j) as i32); }
                        }
                    }
                    if k == 3 {
                        // White bishop
                        if dr.abs() == dc.abs() && dr != 0 && path_clear(board, i, j) && is_valid_target(p, k) {
                            if !in_check { legal_moves.push((i * 64 + j) as i32); }
                        }
                    }
                    if k == 4 {
                        // White queen
                        let straight = (dr == 0) != (dc == 0);
                        let diagonal = dr.abs() == dc.abs() && dr != 0;
                        if (straight || diagonal) && path_clear(board, i, j) && is_valid_target(p, k) {
                            if !in_check { legal_moves.push((i * 64 + j) as i32); }
                        }
                    }
                    if k == 5 {
                        // White king
                        if dr.abs() <= 1 && dc.abs() <= 1 && (dr != 0 || dc != 0) && is_valid_target(p, k) {
                            if !in_check { legal_moves.push((i * 64 + j) as i32); }
                        } else if j == 58 && p == -1 && board[12][58] && piece_at(board, 59) == -1 && piece_at(board, 57) == -1 {
                            if !in_check { legal_moves.push((i * 64 + j) as i32); }
                        } else if j == 62 && p == -1 && board[12][62] && piece_at(board, 61) == -1 {
                            if !in_check { legal_moves.push((i * 64 + j) as i32); }
                        }
                    }
                    if k == 6 {
                        // Black pawn
                        if dc == 0 && dr == 1 && p == -1 {
                            if !in_check { legal_moves.push((i * 64 + j) as i32); }
                        } else if dc == 0 && dr == 2 && row_i == 1 && p == -1 && piece_at(board, i + 8) == -1 {
                            if !in_check { legal_moves.push((i * 64 + j) as i32); }
                        } else if (dc == 1 || dc == -1) && dr == 1 && p != -1 && p < 6 {
                            if !in_check { legal_moves.push((i * 64 + j) as i32); }
                        }
                    }
                    if k == 7 {
                        // Black rook
                        if ((dr == 0) != (dc == 0)) && path_clear(board, i, j) && is_valid_target(p, k) {
                            if !in_check { legal_moves.push((i * 64 + j) as i32); }
                        }
                    }
                    if k == 8 {
                        // Black knight
                        if matches!((dr.abs(), dc.abs()), (1, 2) | (2, 1)) && is_valid_target(p, k) {
                            if !in_check { legal_moves.push((i * 64 + j) as i32); }
                        }
                    }
                    if k == 9 {
                        // Black bishop
                        if dr.abs() == dc.abs() && dr != 0 && path_clear(board, i, j) && is_valid_target(p, k) {
                            if !in_check { legal_moves.push((i * 64 + j) as i32); }
                        }
                    }
                    if k == 10 {
                        // Black queen
                        let straight = (dr == 0) != (dc == 0);
                        let diagonal = dr.abs() == dc.abs() && dr != 0;
                        if (straight || diagonal) && path_clear(board, i, j) && is_valid_target(p, k) {
                            if !in_check { legal_moves.push((i * 64 + j) as i32); }
                        }
                    }
                    if k == 11 {
                        // Black king
                        if dr.abs() <= 1 && dc.abs() <= 1 && (dr != 0 || dc != 0) && is_valid_target(p, k) {
                            if !in_check { legal_moves.push((i * 64 + j) as i32); }
                        } else if j == 2 && p == -1 && board[12][2] && piece_at(board, 3) == -1 && piece_at(board, 1) == -1 {
                            if !in_check { legal_moves.push((i * 64 + j) as i32); }
                        } else if j == 6 && p == -1 && board[12][6] && piece_at(board, 5) == -1  {
                            if !in_check { legal_moves.push((i * 64 + j) as i32); }
                        }
                    }
                }
                break;
            }
        }
    }
    legal_moves
}

pub fn init(board: &mut Vec<Vec<bool>>) {
    board[0] = vec![false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , true ,true ,true ,true ,true ,true ,true ,true  , false,false,false,false,false,false,false,false];
    board[1] = vec![false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , true ,false,false,false,false,false,false,true ];
    board[2] = vec![false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,true ,false,false,false,false,true ,false];
    board[3] = vec![false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,true ,false,false,true ,false,false];
    board[4] = vec![false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,true ,false,false,false,false];
    board[5] = vec![false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,true ,false,false,false];
    board[6] = vec![false,false,false,false,false,false,false,false , true ,true ,true ,true ,true ,true ,true ,true  , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false];
    board[7] = vec![true ,false,false,false,false,false,false,true  , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false];
    board[8] = vec![false,true ,false,false,false,false,true ,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false];
    board[9] = vec![false,false,true ,false,false,true ,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false];
    board[10]= vec![false,false,false,true ,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false];
    board[11]= vec![false,false,false,false,true ,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false];
    board[12]= vec![true ,true ,true ,true ,true ,true ,true ,true  , true ,true ,true ,true ,true ,true ,true ,true  , true ,true ,true ,true ,true ,true ,true ,true  , true ,true ,true ,true ,true ,true ,true ,true  , true ,true ,true ,true ,true ,true ,true ,true  , true ,true ,true ,true ,true ,true ,true ,true  , true ,true ,true ,true ,true ,true ,true ,true  , true ,true ,true ,true ,true ,true ,true ,true ];
    board[13]= vec![false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false , false,false,false,false,false,false,false,false];
}

pub fn make_move(mut board: Vec<Vec<bool>>, code: &str) -> (bool, Vec<Vec<bool>>) {
    if code.len() != 4 && code.len() != 5 {
        println!("Recieved invalid move code. Use the standard start end square system. Example: e2e4");
        panic!();
    }
    let start = uci_to_move(&code[0..2]);
    let end = uci_to_move(&code[2..4]);
    let mut p = "q";
    if code.len() == 5 {
        p = &code[4..5];
    }

    let moves = find_legal_moves(&mut board, 0);
    if moves.contains(&(start * 64 + end)) {
        // Now we can assume the move is legal for the future
        board[13] = vec![false; 64];
        for l in 0..12usize {
            if board[l][start as usize] {
                if l == 5 {
                    // White King is moving
                    if start == 60 {
                        if end == start - 2 {
                            // Queenside
                            board[1][56] = false;
                            board[1][59] = true;
                        } else if end == start + 2 {
                            // Kingside
                            board[1][63] = false;
                            board[1][61] = true;
                        }
                        for sq in 32..64usize {
                            board[12][sq] = false;
                        }
                    }
                } else if l == 11 {
                    // Black King is moving
                    if start == 4 {
                        if end == start - 2 {
                            // Queenside
                            board[7][0] = false;
                            board[7][3] = true;
                        } else if end == start + 2 {
                            // Kingside
                            board[7][7] = false;
                            board[7][5] = true;
                        }
                        for sq in 0..32usize {
                            board[12][sq] = false;
                        }
                    }
                }
                if l == 1 {
                    // White Rook is moving
                    if start == 63 {
                        for r in 4..8usize {
                            for c in 4..8usize {
                                board[12][r * 8 + c] = false;
                            }
                        }
                    }
                    if start == 56 {
                        for r in 4..8usize {
                            for c in 0..4usize {
                                board[12][r * 8 + c] = false;
                            }
                        }
                    }
                }
                if l == 7 {
                    // Black Rook is moving
                    if start == 7 {
                        for r in 0..4usize {
                            for c in 4..8usize {
                                board[12][r * 8 + c] = false;
                            }
                        }
                    }
                    if start == 0 {
                        for r in 0..4usize {
                            for c in 0..4usize {
                                board[12][r * 8 + c] = false;
                            }
                        }
                    }
                }

                // White pawn moving
                if l == 0 {
                    if end == start - 16 {
                        board[13][((start + end) / 2) as usize] = true;
                    }
                    if end / 8 == 0 {
                        if p == "n" {
                            board[l][start as usize] = false;
                            board[2][end as usize] = true;
                        } else if p == "r" {
                            board[l][start as usize] = false;
                            board[1][end as usize] = true;
                        } else if p == "b" {
                            board[l][start as usize] = false;
                            board[3][end as usize] = true;
                        } else {
                            board[l][start as usize] = false;
                            board[4][end as usize] = true;
                        }
                        return (true, board)
                    }
                }
                // Black pawn moving
                if l == 6 {
                    if end == start + 16 {
                        board[13][((start + end) / 2) as usize] = true;
                    }
                    if end / 8 == 7 {
                        if p == "n" {
                            board[l][start as usize] = false;
                            board[8][end as usize] = true;
                        } else if p == "r" {
                            board[l][start as usize] = false;
                            board[7][end as usize] = true;
                        } else if p == "b" {
                            board[l][start as usize] = false;
                            board[9][end as usize] = true;
                        } else {
                            board[l][start as usize] = false;
                            board[10][end as usize] = true;
                        }
                        return (true, board)
                    }
                }
                
                for l2 in 0..12 {
                    board[l2][end as usize] = false;
                }
                board[l][end as usize] = true;
                board[l][start as usize] = false;
            }
        }
        (true, board)
    } else {
        (false, board)
    }
}