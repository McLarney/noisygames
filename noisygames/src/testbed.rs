use crate::game::Game;
use std::clone::Clone;
use serde::Serialize;
use crate::Strategies;

//what are some things that a testbed should have? run
#[derive(Clone,Serialize)]
pub struct Config<T, U> {
    pub player_a: T,
    pub player_b: U,
    pub game: Game,
    pub num_rounds: usize,
    pub num_round_lengths: Vec<i32>,
    pub location: String,
}

pub fn generate_round_robin_configs (
    game: Game,
    players: Vec<Strategies>,
    round_lengths: Vec<i32>,
    location: String ) 
    -> Vec<Config<Strategies,Strategies>> 
{
    //then create all the configs
    let mut configs = Vec::new();

    for i_idx in 0..players.len() {
        for j_idx in i_idx+1..players.len(){
            let tmp_cfg = Config {
                player_a: players[i_idx].clone(),
                player_b: players[j_idx].clone(),
                game: game.clone(),
                num_rounds: round_lengths.len(),
                num_round_lengths: round_lengths.clone(),
                location: location.to_string().clone(),
            };

            configs.push(tmp_cfg);
        }
    }
    configs
}
use crate::player::*;
pub fn generate_players (
    strat_types: Vec<Strategies>, 
    num_strats: Vec<i32>
    ) 
    -> Vec<Strategies> 
{
    let mut players = Vec::new();
    for i in 0..num_strats.len() {
        for j in 0..num_strats[i] {
            let mut p = strat_types[i].clone();
            let tmp_name = (i+1) as i32*(j+1) as i32;
            
            let q = p.get_player();
            *q.set_name()=tmp_name.to_string();
            players.push(p);
        }
    }
    players
}


