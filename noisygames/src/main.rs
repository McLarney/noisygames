pub mod player;
use crate::player::*;
pub mod testbed;

pub mod game;
pub mod test_utilities;
use crate::testbed::Config;
//use std::thread;
use std::env;
use std::fs;
use serde::Serialize;
use threadpool::ThreadPool;


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
    
    let args: Vec<String> = env::args().collect();
    let mut dirstr = test_utilities::build_datetime_folder("/tmp/test_runs/".to_string());
    if args.len() > 1 {
    	dirstr = test_utilities::build_datetime_folder(args[1].clone().to_string());
    }
    let num_players = 128 - 1; 
    //let num_strategies = vec![2, 2, 2, 2];
    let mut num_strategies = vec![2];
    for _i in 0..num_players { // edited for the purpose of timing program, TODO recommend changeing in final product
    	num_strategies.push(2);
    }
    println!("{}", num_strategies.len());
    
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
    let mut g = game::Game{ 
        payoff_a: a_mtx,
        payoff_b: b_mtx,
        is_init: false};
    g.init_game(); 
    assert!(g.is_init);

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
    let players = testbed::generate_players(strat_types, num_strategies);
    let configs = testbed::generate_round_robin_configs(
        g, players, round_lengths, dirstr );
    
    run_multithreaded_configs_threadpool(configs);
}


fn run_multithreaded_configs_threadpool(mut configs: Vec<Config<Strategies,Strategies>>){
    let num_workers = 30;
    let pool = ThreadPool::new(num_workers);
    
    //run configs through the pool, record_config saves results
    for _idx in 0..configs.len() {

        //let tx = tx.clone();
        let tmp_config = configs.pop().unwrap();
	pool.execute(move || {
            //pool_job(tmp_config);
            let out_configs = run_instance(tmp_config);
            record_configs(out_configs);
            //tx.send(1).expect("panic during threadpool execution");          
        });
    }
    //assert_eq!(rx.iter().take(configs.len()).fold(0, |a,b| a+b), configs.len());
    pool.join();
}


/*
fn run_multithreaded_configs(mut configs: Vec<Config<Strategies,Strategies>>){

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
        thread.join().unwrap()
    }
} */

fn record_configs<T:Serialize,U:Serialize>(configs: Vec<Config<T,U>>) {
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
