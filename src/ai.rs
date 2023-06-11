use wasm_bindgen::prelude::*;

use crate::Game;

const PLAYERS: i8 = 2;
const INF: i8 = i8::MAX;

#[wasm_bindgen]
pub struct AIMove {
    value: i8,
    column: usize
}

#[wasm_bindgen]
pub fn negamax(mut game: &mut Game, depth: i8, turn: i8, mut alpha: i8, beta: i8) -> i8 {
    let winner = game.win();
    if winner != -1 {
        return if winner == turn {
            depth + 1
        } else {
            1 - depth
        }
    }
    if depth == 0 {
        return 0;
    }

    let moves = game.moves();
    if moves.is_empty() {
        return 0;
    }
    
    let mut bestValue = -INF;
    for move_ in moves {
        game.make_move(turn, move_);
        let value = -negamax(&mut game, depth - 1, (turn + 1) % PLAYERS, -beta, -alpha);
        game.unmove();
        
        bestValue = i8::max(bestValue, value);
        alpha = i8::max(alpha, bestValue);
        if alpha >= beta {
            break;
        }
    }
    
    return bestValue;
}

#[wasm_bindgen]
pub fn negamax_move(mut game: &mut Game, depth: i8, turn: i8) -> Option<AIMove> {        
    let moves = game.moves();
    if moves.is_empty() {
        return None;
    }

    let mut alpha = -INF;
    let beta = INF;
    let mut bestValue = -INF;
    let mut bestMove: Option<usize> = None;

    for move_ in moves {
        game.make_move(turn, move_);
        let value = -negamax(&mut game, depth - 1, (turn + 1) % PLAYERS, -beta, -alpha);
        game.unmove();
        
        if value > bestValue {
            bestValue = value;
            bestMove = Some(move_);
        }
        
        alpha = i8::max(alpha, bestValue);
        if alpha >= beta {
            break
        }
    }
    if bestValue < 0 {
        println!("Forced loss");
    } else if bestValue > 0 {
        println!("Forced win");
    } else {
        println!("Nothing seen");
    }
    match bestMove {
        None => None,
        Some(move_) => Some(AIMove { value: bestValue, column: move_ })
    }
}

#[wasm_bindgen]
pub fn humanValue(value: i8, depth: i8) -> String {
    if value < 0 {
        format!("Loss in {} moves", (depth - value.abs() + 1) / 2)
    } else if value > 0 {
        format!("Win in {} moves", (depth - value) / 2)
    } else {
        "Draw?".to_owned()
    }
}

