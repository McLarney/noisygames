pub mod player;
use crate::player::*;
pub mod testbed;


pub mod game;
pub mod test_utilities;
use crate::testbed::Config;
use std::thread;
use std::fs;
use serde::Serialize;

fn main() {
    let is_round_robin = false;
    if is_round_robin {
        run_round_robin();
    }
    let is_iter = true;
    if is_iter {
        run_iterative();
    }

}

fn run_iterative() {
    let round_length=10;
    let num_copies = 1;
    let prob_step = 0.09;
    //to use generate_players, we need go generate a vector of strategies
    let base_player = BasicPlayer::new();
    let mut probs_vec = Vec::new();
    
    let mut tmp_prob = 0.0;
    while tmp_prob <= 1.0 {
        probs_vec.push(tmp_prob);
        let t: f32 = (tmp_prob+prob_step)*10000.0;
        tmp_prob=t.round()/10000.0;
    }
    println!("{:?}",&probs_vec);
    println!("Length Probs {}", probs_vec.len());
    let all_probs = test_utilities::get_permutations_with_replacement(probs_vec,4);

    let mut players = Vec::new();
    for perm in all_probs {
        let p_tmp=StochasticPlayer{play:base_player.clone(),
            prob_vec: vec![
                perm[0],
                perm[1],
                perm[2],
                perm[3]
            ]
            };
        let p = Strategies::StochasticPlayer{player: p_tmp};
        for _ in 0..num_copies {
            players.push(p.clone());
        }
    }

    println!("Player numbers {}",players.len());
    //now that we have a bunch of players, generate and shuffle the pairs
    let player_pairs = test_utilities::shuffle_and_pair(players);
    //now with a set of pairs, we can generate all the configs

    let a_mtx = vec![
        vec![3,0],
        vec![5,1]
    ];
    let b_mtx = vec![
        vec![3,5],
        vec![0,1]
    ];
    let mut g = game::Game{ 
        payoff_a: a_mtx,
        payoff_b: b_mtx,
        is_init: false};
    g.init_game(); 
    assert!(g.is_init);

    let dirstr = test_utilities::build_datetime_folder();
    let mut configs = Vec::new();
    
    for (idx, pr) in player_pairs.iter().enumerate() {
        let tmp_cfg = Config {
            player_a: pr[0].clone(),
            player_a_num: idx,
            player_b: pr[1].clone(),
            player_b_num: idx*player_pairs.len(),
            game: g.clone(),
            num_rounds: 1,
            num_round_lengths: vec![round_length],
            location: dirstr.to_string().clone(),
        };
        configs.push(tmp_cfg);
    }
    //for all configs, run the game!
    //code is now refactored to also return the configs after each round 
    println!("We have {} configs!", configs.len());
    let round_configs = run_multithreaded_configs(configs);
    //evaluate outcome, killing losers, double winners
    
    //save population statistics (distributions on each trait)
    
    //rerun for next round
}


fn run_round_robin() {
    let num_strategies = vec![2, 2, 2, 2];
    let round_lengths = vec![63, 77, 151, 151, 308];
    //potential strategies for now are always defect, tit for tat, and grim trigger
    let base_player = BasicPlayer::new();
    let alwaysdefect_tmp = AlwaysDefect { play: base_player.clone() };
    let grimtrigger_tmp = GrimTrigger { play: base_player.clone() };
    let titfortat_tmp = TitForTat { play: base_player.clone() };
    let randomdefect_tmp = RandomDefect { play: base_player.clone(), probability: 0.5 };

    let a_strat = Strategies::AlwaysDefect{ player: alwaysdefect_tmp };
    let b_strat = Strategies::GrimTrigger{ player: grimtrigger_tmp };
    let c_strat = Strategies::TitForTat{ player: titfortat_tmp };
    let d_strat = Strategies::RandomDefect{ player: randomdefect_tmp };
    let strat_types = vec![
        a_strat,
        b_strat,
        c_strat,
        d_strat,
    ];
    
    let a_mtx = vec![
        vec![3,0],
        vec![5,1]
    ];
    let b_mtx = vec![
        vec![3,5],
        vec![0,1]
    ];
    let mut g = game::Game{ 
        payoff_a: a_mtx,
        payoff_b: b_mtx,
        is_init: false};
    g.init_game(); 
    assert!(g.is_init);

    let players = testbed::generate_players(strat_types, num_strategies);
    let dirstr = test_utilities::build_datetime_folder();
    let configs = testbed::generate_round_robin_configs(
        g, players, round_lengths, dirstr );
    
    run_multithreaded_configs(configs);
}


use std::sync::{Arc,Mutex};
fn run_multithreaded_configs(mut configs: Vec<Config<Strategies,Strategies>>) -> Vec<Config<Strategies,Strategies>>{
    let mut threads = Vec::new();
    //then run all the configs and save them off
    let all_configs = Arc::new(Mutex::new(Vec::new()));
    for _ in 0..configs.len() {
        //println!("Running thread {}", idx);
        let all_configs_clone = Arc::clone(&all_configs);

        let tmp_config = configs.pop().unwrap();
        let thread = thread::spawn(move || {
            let mut all_cfg = all_configs_clone.lock().unwrap();
            let out_configs = run_instance(tmp_config);
            record_configs(&out_configs);
            for out_cfg in out_configs {
                all_cfg.push(out_cfg);
            }
            
            drop(all_cfg);
        });
        threads.push(thread);
    }
    //go through all threads and join
    for thread in threads {
        thread.join().unwrap()
    }
    let all_cfg = all_configs.lock().unwrap();
    all_cfg.to_vec()
}

fn record_configs<T:Serialize,U:Serialize>(configs: &Vec<Config<T,U>>) {
    let s = &configs[0].location;
    let group_dir = format!("{}/player{}player{}/", s, &configs[0].player_a_num, &configs[0].player_b_num);
    fs::create_dir_all(&group_dir).expect("Directory unable to be created");

    for idx in 0..configs.len() {
        let run_dir = format!("{}round{}play_num{}.json", group_dir, &configs[0].num_round_lengths[idx], idx);
        let j = serde_json::to_string(&configs[idx]).unwrap();

        //println!("{}", &run_dir);
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
