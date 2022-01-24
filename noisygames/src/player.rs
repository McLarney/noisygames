use std::clone::Clone;
use serde::Serialize;

#[derive(Clone,Serialize)]
pub struct BasicPlayer {
    pub name: String,
    pub my_moves: Vec<i32>,
    pub their_moves: Vec<i32>,
    pub my_outcomes: Vec<i32>,
    pub their_outcomes: Vec<i32>,
    pub my_score: i32,
    pub their_score: i32,
}

pub trait Player {
    fn get_name(&self) -> &str;
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
            my_moves: Vec::new(),
            their_moves: Vec::new(),
            my_outcomes: Vec::new(),
            their_outcomes: Vec::new(),
        }
    }
}

impl Player for BasicPlayer {
    fn get_name(&self) -> &str {
        &self.name[..]
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
