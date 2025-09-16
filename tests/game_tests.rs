use dev_sys_2427::{is_win, check_winner, move_array_to_num, move_num_to_array};

fn empty_board() -> Vec<Vec<char>> {
    vec![vec![' '; 3], vec![' '; 3], vec![' '; 3]]
}

#[test]
fn test_move_array_to_num_and_back() {
    let arr = [2, 1];
    let num = move_array_to_num(arr, 3);
    assert_eq!(num, 8);
    let arr2 = move_num_to_array(num, 3);
    assert_eq!(arr, arr2);
}

#[test]
fn test_is_win_row() {
    let mut board = empty_board();
    board[0] = vec!['X', 'X', 'X'];
    assert!(is_win(board.clone(), 'X'));
    assert!(!is_win(board.clone(), 'O'));
}

#[test]
fn test_is_win_col() {
    let mut board = empty_board();
    board[0][1] = 'O';
    board[1][1] = 'O';
    board[2][1] = 'O';
    assert!(is_win(board.clone(), 'O'));
    assert!(!is_win(board.clone(), 'X'));
}

#[test]
fn test_is_win_diag() {
    let mut board = empty_board();
    board[0][0] = 'X';
    board[1][1] = 'X';
    board[2][2] = 'X';
    assert!(is_win(board.clone(), 'X'));
    board[0][2] = 'O';
    board[1][1] = 'O';
    board[2][0] = 'O';
    assert!(is_win(board.clone(), 'O'));
}

#[test]
fn test_check_winner_x() {
    let mut board = empty_board();
    board[1] = vec!['X', 'X', 'X'];
    assert_eq!(check_winner(board), 'X');
}

#[test]
fn test_check_winner_o() {
    let mut board = empty_board();
    board[0][0] = 'O';
    board[1][1] = 'O';
    board[2][2] = 'O';
    assert_eq!(check_winner(board), 'O');
}

#[test]
fn test_check_winner_draw() {
    let board = vec![
        vec!['X', 'O', 'X'],
        vec!['O', 'X', 'O'],
        vec!['O', 'X', 'O'],
    ];
    assert_eq!(check_winner(board), 'D');
}

#[test]
fn test_check_winner_none() {
    let board = empty_board();
    assert_eq!(check_winner(board), ' ');
}
