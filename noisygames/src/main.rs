pub mod player;
use crate::player::*;
pub mod testbed;

pub mod game;
pub mod test_utilities;
use crate::testbed::Config;
use std::thread;
use std::fs;
use serde::Serialize;

#[derive(Clone,Serialize)]
pub enum Strategies {
    AlwaysDefect{player: AlwaysDefect},
    GrimTrigger{player: GrimTrigger},
    TitForTat{player: TitForTat},
    RandomDefect{player: RandomDefect},
}

impl Strategy for Strategies {
    fn strategy(&self) -> i32 {
        match self {
            Strategies::AlwaysDefect{ player } => player.strategy(),
            Strategies::GrimTrigger{ player } => player.strategy(),
            Strategies::TitForTat{ player } => player.strategy(),
            Strategies::RandomDefect{ player } => player.strategy(),
        }
    }

    fn get_player(&mut self) -> &mut BasicPlayer {
        match self {
            Strategies::AlwaysDefect{ player } => player.get_player(),
            Strategies::GrimTrigger{ player } => player.get_player(),
            Strategies::TitForTat{ player } => player.get_player(),
            Strategies::RandomDefect{ player } => player.get_player(),
        }
    }
}


fn main() {
    let num_rounds = 5;
    let num_strategies = [20, 20, 20, 20];
    let round_lengths = vec![63, 77, 151, 151, 308];
    //potential strategies for now are always defect, tit for tat, and grim trigger
    let a_mtx = vec![
        vec![3,0],
        vec![5,1]
    ];

    let b_mtx = vec![
        vec![3,5],
        vec![0,1]
    ];

    //send this into a constructor for a Game type
    
    let mut g = game::Game{ 
        payoff_a: a_mtx,
        payoff_b: b_mtx,
        is_init: false};
    
    //run iterated some N times with strategy profiles specified for each player
    g.init_game(); 
    assert!(g.is_init);

    //now it's time to make the player types
    let player_a = BasicPlayer {
        name: "john".to_string(),
        my_score: 0,
        their_score: 0,
        my_moves: Vec::new(),
        their_moves: Vec::new(),
        my_outcomes: Vec::new(),
        their_outcomes: Vec::new(),
    };
    let player_b = BasicPlayer {
        name: "jacob".to_string(),
        my_score: 0,
        their_score: 0,
        my_moves: Vec::new(),
        their_moves: Vec::new(),
        my_outcomes: Vec::new(),
        their_outcomes: Vec::new(),
    }; 
    let player_c = BasicPlayer {
        name: "jimmy".to_string(),
        my_score: 0,
        their_score: 0,
        my_moves: Vec::new(),
        their_moves: Vec::new(),
        my_outcomes: Vec::new(),
        their_outcomes: Vec::new(),
    };
    
    let player_d = BasicPlayer {
        name: "jason".to_string(),
        my_score: 0,
        their_score: 0,
        my_moves: Vec::new(),
        their_moves: Vec::new(),
        my_outcomes: Vec::new(),
        their_outcomes: Vec::new(),
    };
    
    let a_tmp = AlwaysDefect { play: player_a };
    let b_tmp = GrimTrigger { play: player_b};
    let c_tmp = TitForTat { play: player_c };
    let d_tmp = RandomDefect { play: player_d, probability: 0.5 };

    let a_strat = Strategies::AlwaysDefect{ player: a_tmp };
    let b_strat = Strategies::GrimTrigger{ player: b_tmp };
    let c_strat = Strategies::TitForTat{ player: c_tmp };
    let d_strat = Strategies::RandomDefect{ player: d_tmp };

    let strat_types = vec![
        a_strat,
        b_strat,
        c_strat,
        d_strat,
    ];
    let mut players = Vec::new();
    //first set up all the players
    for i in 0..num_strategies.len() {
        for _j in 0..num_strategies[i] {
            players.push(strat_types[i].clone());
        }
    }

    let dirstr = test_utilities::build_datetime_folder();
    
    //then create all the configs
    let mut configs = Vec::new();

    for i_idx in 0..players.len() {
        for j_idx in i_idx+1..players.len(){
            let tmp_cfg = Config {
                player_a: players[i_idx].clone(),
                player_a_num: i_idx,
                player_b: players[j_idx].clone(),
                player_b_num: j_idx,
                game: g.clone(),
                num_rounds: num_rounds,
                num_round_lengths: round_lengths.clone(),
                location: dirstr.to_string().clone(),
            };

            configs.push(tmp_cfg);
        }
    }

    let mut threads = Vec::new();
    //then run all the configs and save them off
    for idx in 0..configs.len() {

        println!("Running thread {}", idx);
        let tmp_config = configs.pop().unwrap();
        let thread = thread::spawn(move || {
            let out_configs = run_instance(tmp_config);
            record_configs(out_configs);
        });

        threads.push(thread);
    }

    //go through all threads and join
    for thread in threads {
        thread.join().unwrap();
    }

}

fn record_configs<T:Serialize,U:Serialize>(configs: Vec<Config<T,U>>) {
    let s = &configs[0].location;
    let group_dir = format!("{}/player{}player{}/", s, &configs[0].player_a_num, &configs[0].player_b_num);
    
    fs::create_dir_all(&group_dir).expect("Directory unable to be created");

    for idx in 0..configs.len() {
        let run_dir = format!("{}round{}play_num{}.json", group_dir, &configs[0].num_round_lengths[idx], idx);
        let j = serde_json::to_string(&configs[idx]).unwrap();

        println!("{}", &run_dir);

        fs::write(run_dir, j).expect("Unable to write file");
    }
}


fn run_instance<T: Strategy+Clone, U: Strategy+Clone>(config: Config<T, U>) -> Vec<Config<T, U>> {
    let mut configs = Vec::new();

    for idx in 0..config.num_rounds {
        
        let mut tmp_cfg = config.clone();
        
        for _ in 0..config.num_round_lengths[idx] {
            let move_a = tmp_cfg.player_a.strategy();
            let move_b = tmp_cfg.player_b.strategy();

            let moves_a = (move_a.clone(), move_b.clone());
            let moves_b = (move_b.clone(), move_a.clone());

            let outcome_a = tmp_cfg.game.turn_outcome(move_a as usize, move_b as usize);

            let temp = outcome_a.clone();
            let outcome_b = (temp.1, temp.0);
            let tmp_a = tmp_cfg.player_a.get_player();
            let tmp_b = tmp_cfg.player_b.get_player();
            tmp_a.read_mv(moves_a, outcome_a);
            tmp_b.read_mv(moves_b, outcome_b);
        }
        configs.push(tmp_cfg);
    }
    configs
}
