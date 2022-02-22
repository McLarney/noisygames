use std::clone::Clone;
use serde::Serialize;

#[derive(Clone,Serialize)]
pub struct BasicPlayer {
    pub name: String,
    pub my_moves: Box<Vec<i32>>,
    pub their_moves: Box<Vec<i32>>,
    pub my_outcomes: Box<Vec<i32>>,
    pub their_outcomes: Box<Vec<i32>>,
    pub my_score: i32,
    pub their_score: i32,
}

pub trait Player {
    fn get_name(&self) -> &str;
    fn set_name(&mut self) -> &mut String;
    fn get_my_score(&self) -> i32;
    fn get_their_score(&self) -> i32;
    fn get_my_moves(&self) -> &Vec<i32>;
    fn get_their_moves(&self) -> &Vec<i32>;
    fn get_my_outcomes(&self) -> &Vec<i32>;
    fn get_their_outcomes(&self) -> &Vec<i32>;
    fn update_score(&mut self, round_score: (i32, i32));
    fn read_mv(&mut self, read_mvs: (i32, i32), outcome: (i32, i32));
}
impl BasicPlayer {
    pub fn new() -> BasicPlayer {
        BasicPlayer {
            name: "john".to_string(),
            my_score: 0,
            their_score: 0,
            my_moves: Box::new(Vec::new()),
            their_moves: Box::new(Vec::new()),
            my_outcomes: Box::new(Vec::new()),
            their_outcomes: Box::new(Vec::new()),
        }
    }
}

impl Player for BasicPlayer {
    fn get_name(&self) -> &str {
        &self.name[..]
    }
    fn set_name(&mut self) -> &mut String {
        &mut self.name
    }
    fn get_my_score(&self) -> i32 {
        self.my_score
    }

    fn get_their_score(&self) -> i32 {
        self.their_score
    }

    fn get_my_moves(&self) -> &Vec<i32> {
        &self.my_moves
    }

    fn get_their_moves(&self) -> &Vec<i32> {
        &self.their_moves
    }

    fn get_my_outcomes(&self) -> &Vec<i32> {
        &self.my_outcomes
    }

    fn get_their_outcomes(&self) -> &Vec<i32> {
        &self.their_outcomes
    }
    fn update_score(&mut self, round_score: (i32, i32)) {
        self.my_score += round_score.0;
        self.their_score += round_score.1;
    }
    fn read_mv(&mut self, read_mvs: (i32, i32), outcome: (i32, i32)) {
        self.my_moves.push(read_mvs.0);
        self.their_moves.push(read_mvs.1);
        self.my_outcomes.push(outcome.0);
        self.their_outcomes.push(outcome.1);
        self.update_score(outcome);
    }
}


pub trait Strategy {
	fn strategy(&self) -> i32;
    fn get_player(&mut self) -> &mut BasicPlayer;
}

#[derive(Clone,Serialize)]
pub struct TitForTat {
	pub play: BasicPlayer,
}

impl Strategy for TitForTat {
	fn strategy(&self) -> i32 {
		let their_moves = &self.play.get_their_moves();
		if their_moves.len() > 0 {
			their_moves.last().unwrap().clone()
		} else {
			0
		}
	}

    fn get_player(&mut self) -> &mut BasicPlayer {
        &mut self.play
    }
}
#[derive(Clone,Serialize)]
pub struct GrimTrigger {
	pub play: BasicPlayer,
}

impl Strategy for GrimTrigger {
	fn strategy(&self) -> i32 {
		let their_moves = &self.play.get_their_moves();
		if their_moves.len() == 0 {
			0
		} else if their_moves.last().unwrap().clone() == 0 {
			0
		}
		else {
			1
		}
	}

    fn get_player(&mut self) -> &mut BasicPlayer {
        &mut self.play
    }
}

#[derive(Clone,Serialize)]
pub struct AlwaysDefect {
	pub play: BasicPlayer,
}

impl Strategy for AlwaysDefect {
	fn strategy(&self) -> i32 {
		1
	}
    
    fn get_player(&mut self) -> &mut BasicPlayer {
        &mut self.play
    }
}

#[derive(Clone,Serialize)]
pub struct RandomDefect {
    pub play: BasicPlayer,
    pub probability: f32,
}

use rand::Rng;

impl Strategy for RandomDefect {
    fn strategy(&self) -> i32 {
        let num: f32 = rand::thread_rng().gen_range(0..=100) as f32;
        if 100.0 * self.probability > num {
            1
        } else {
            0
        }
    }
    
    fn get_player(&mut self) -> &mut BasicPlayer {
        &mut self.play
    }
}

#[derive(Clone,Serialize)]
pub struct StochasticPlayer {
    pub play: BasicPlayer,
    pub prob_vec: Vec<f32>,
}
impl Strategy for StochasticPlayer {
    fn strategy(&self) -> i32 {
        //initially cooperate
        if self.play.get_their_moves().len() == 0 {
            return 0
        }
        //get the play from last round
        let my_mv=*self.play.my_moves.last().unwrap();
        let their_mv=*self.play.their_moves.last().unwrap();
        //given the play requirement, roll a die
        let mut roll: f32 = rand::thread_rng().gen_range(0..=100) as f32;
        roll=roll/100.0;
        //if roll less than vec entry, return cooperate(0), else return defect(1)
        let retval;
        if my_mv==0 {
            if their_mv == 0 {
                if self.prob_vec[0] > roll {retval=0} else {retval=1}
            } else {
                if self.prob_vec[1] > roll {retval=0} else {retval=1}
            }
        } else {
            if their_mv == 0 {
                if self.prob_vec[2] > roll {retval=0} else {retval=1}
            } else {
                if self.prob_vec[3] > roll {retval=0} else {retval=1}
            }
        }
        retval
    }
    fn get_player(&mut self) -> &mut BasicPlayer {
        &mut self.play
    }
}
pub trait StochasticP {
    fn get_prob_vec(&self) -> Vec<f32>;
}
impl StochasticP for StochasticPlayer {
    fn get_prob_vec(&self) -> Vec<f32>{
        self.prob_vec.clone()
    }
}

#[derive(Clone,Serialize)]
pub enum Strategies {
    AlwaysDefect{player: AlwaysDefect},
    GrimTrigger{player: GrimTrigger},
    TitForTat{player: TitForTat},
    RandomDefect{player: RandomDefect},
    StochasticPlayer{player: StochasticPlayer},
}
impl StochasticP for Strategies {
    fn get_prob_vec(&self) -> Vec<f32> {
        let v = vec![0.23];
        match self {
            Strategies::StochasticPlayer{ player } => player.get_prob_vec(),
            _ => v,
        }
    }
}

impl Strategy for Strategies {
    fn strategy(&self) -> i32 {
        match self {
            Strategies::AlwaysDefect{ player } => player.strategy(),
            Strategies::GrimTrigger{ player } => player.strategy(),
            Strategies::TitForTat{ player } => player.strategy(),
            Strategies::RandomDefect{ player } => player.strategy(),
            Strategies::StochasticPlayer{ player } => player.strategy(),
        }
    }

    fn get_player(&mut self) -> &mut BasicPlayer {
        match self {
            Strategies::AlwaysDefect{ player } => player.get_player(),
            Strategies::GrimTrigger{ player } => player.get_player(),
            Strategies::TitForTat{ player } => player.get_player(),
            Strategies::RandomDefect{ player } => player.get_player(),
            Strategies::StochasticPlayer{ player } => player.get_player(),
        }
    }
}


