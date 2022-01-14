use crate::player::Player;
use crate::game::Game;

//what are some things that a testbed should have? run
#[derive(Clone)]
pub struct Config {
    pub player_a: Player,
    pub player_b: Player,
    pub game: Game,
    pub num_rounds: i32,
    pub num_instance: i32,
}

//This is to be used for data that might get analyzed after the fact
//struct DerivedData {
//}

//This is to collate the Config and the DerivedData for comprehensive access.
//struct RunResults {
//    config: Config,
//    der_dat: DerivedData,
//}
//really what I'd like to do is to initialize players with all the relevant strategy stuff and
//whatnot, then leave it to run with whatever strategy through some number of rounds with some
//number of instances. For the time being, we can just do number of rounds.

pub fn run(config: Config) {
    for idx in 1..=config.num_instance {
        let tmp_cfg = config.clone();
        println!("Running set {}", idx);
        println!("===============");

        run_instance(tmp_cfg);
    }
    println!("Done!");
}

fn run_instance(mut config: Config) -> Config {
    //in an instance we would like to run the game for some number of rounds
    //in each round a player should submit a new move calling on the strategy underneath to
    //interpret previous rounds
    
    for idx in 1..=config.num_rounds {
        let move_a = config.player_a.write_mv();
        let move_b = config.player_b.write_mv();

        let moves_a = (move_a.clone(), move_b.clone());
        let moves_b = (move_b.clone(), move_a.clone());

        //here moves and transition matrix go in describing chance of changing state

        //now we've got both moves. Let's read the outcome from the game
        let outcome_a = config.game.turn_outcome(move_a as usize,move_b as usize);
        let temp = outcome_a.clone();
        let outcome_b = (temp.1, temp.0);
        //here we'd to transition among outcomes

        config.player_a.read_mv(moves_a, outcome_a);
        config.player_b.read_mv(moves_b, outcome_b);

        println!("Played round {}.", idx);
        println!("{} played {}. {} played {}", config.player_a.name, move_a, config.player_b.name, move_b);
        println!("Player A scored {} points. Player B scored {} points.", outcome_a.0, outcome_a.1);
        println!("Player A current score is {} and thinks B's score is {}.", config.player_a.get_my_score(), config.player_a.get_their_score());
        println!("Player B current score is {} and thinks A's score is {}.", config.player_b.get_my_score(), config.player_b.get_their_score());
        println!("");
    }
    config
}