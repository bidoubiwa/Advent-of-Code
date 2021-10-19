use day_9::Game;
use std::collections::VecDeque;
use utils::*;

fn next_player(nbr_players: usize, current_player: usize) -> usize {
    return (current_player + 1) % (nbr_players);
}

fn place_marble(
    current_marble_pos: usize,
    board: &mut VecDeque<usize>,
    marble_to_insert: usize,
) -> usize {
    let new_pos = (current_marble_pos + 2) % board.len();
    board.insert(new_pos, marble_to_insert);
    return new_pos;
}

fn remove_rule_23_marble(current_marble_pos: usize, board: &mut VecDeque<usize>) -> (usize, usize) {
    let counter_clockwise_position = current_marble_pos as i32 - 7;
    let marble_pos = counter_clockwise_position.rem_euclid(board.len() as i32) as usize;
    let removed_marble = board.remove(marble_pos).unwrap();
    return (removed_marble, marble_pos);
}

fn main() {
    let game = read_lines_as::<Game>()[0];
    dbg!(game);
    let nbr_players = game.players;
    let nbr_marbles = game.marbles;

    let mut players_score: Vec<usize> = vec![0; nbr_players];

    let mut board: VecDeque<usize> = VecDeque::new();
    board.push_back(0);
    board.push_back(1);

    let mut current_marble_pos = 1;

    let mut current_player = 1;
    for marble in 2..=nbr_marbles {
        if marble % 23 == 0 {
            let (player_points, new_current_marble_pos) =
                remove_rule_23_marble(current_marble_pos, &mut board);
            current_marble_pos = new_current_marble_pos;
            players_score[current_player] += marble + player_points;
        } else {
            current_marble_pos = place_marble(current_marble_pos, &mut board, marble);
        }
        current_player = next_player(nbr_players, current_player);
    }

    println!("Maximum score is: {}", &players_score.iter().max().unwrap());
}
