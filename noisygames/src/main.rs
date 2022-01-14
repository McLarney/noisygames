use ndarray::prelude::*;
pub mod player;
pub mod testbed;

pub mod game;

use crate::testbed::Config;

fn main() {
    let num_runs = 5;
    let num_instance = 3;
    //potential strategies for now are always defect, tit for tat, and grim trigger
    let a_strat = "equal_random";
    let b_strat = "equal_random";

    let a_mtx=arr2(&[[-1, -3],
                   [0, -2]]);
    let b_mtx=arr2(&[[-1, 0],
                   [-3, -2]]);
    //send this into a constructor for a Game type
    
    let mut g = game::Game{ 
        payoff_a: a_mtx,
        payoff_b: b_mtx,
        is_init: false};
    
    //run iterated some N times with strategy profiles specified for each player
    g.init_game(); 
    assert!(g.is_init);

    //now it's time to make the player types
    let player_a = player::Player::make_player("player_A".to_string(), a_strat.to_string());
    let player_b = player::Player::make_player("player_B".to_string(), b_strat.to_string());

    //set up the configuration for the experiment
    let cfg = Config {
        player_a,
        player_b,
        game: g,
        num_rounds: num_runs,
        num_instance: num_instance,
    };

    testbed::run(cfg);
}
