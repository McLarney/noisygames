use rand::Rng;

#[derive(Clone)]
pub struct Player{
    pub name: String,
    strategy: String,
    my_moves: Vec<i32>,
    their_moves: Vec<i32>,
    my_outcomes: Vec<i32>,
    their_outcomes: Vec<i32>,
    my_score: i32,
    their_score: i32,
}
//might want to make some checks to prevent impossible scenarios from occurring, tests over things
//like if the score is ever over the max number in the game*number of turns, then we have an error.

impl Player{
    pub fn make_player(name: String, strategy: String) -> Player {
        let player = Player {
            name,
            strategy: strategy.to_string(),
            my_moves: Vec::new(),
            their_moves: Vec::new(),
            my_outcomes: Vec::new(),
            their_outcomes: Vec::new(),
            my_score: 0,
            their_score: 0,
        };
        player
    }
    pub fn get_my_score(&self) -> i32 {
        self.my_score
    }
    pub fn get_their_score(&self) -> i32 {
        self.their_score
    }

    pub fn write_mv(&self) -> i32 {
        self.mv_from_strategy()
    }
    fn mv_from_strategy(&self) -> i32 {
        match &self.strategy[..] {
            "always_defect" => self.always_defect(),
            "always_cooperate" => self.always_cooperate(),
            "equal_random" => self.equal_random(),
            _ => panic!("Invalid strategy wtf dude"),
        }
    }

    fn always_defect(&self) -> i32 {
        1
    }
    fn always_cooperate(&self) -> i32 {
        0
    }
    fn equal_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..2) as i32
    }

    pub fn read_mv(&mut self, read_mvs: (i32, i32), outcome: (i32, i32)) {
        //moves the game thought were played may not have been as intended
        self.my_moves.push(read_mvs.0);
        self.their_moves.push(read_mvs.1);
        //need to append to *_outcomes
        self.my_outcomes.push(outcome.0);
        self.their_outcomes.push(outcome.1);
        //need to update score
        self.update_score(outcome)
    }

    pub fn get_total_score(&self) -> (i32, i32) { (3,3) }

    pub fn get_player_score(&self) -> i32 { 2 }

    pub fn calculate_score_from_base(&mut self) -> (i32, i32) {
        //here we iterate over the entire series of outcomes
        
        //sometimes we may want to just see what the score is, other times we'd like to do some
        //assignment
        (3,3)
    }

    fn update_score(&mut self, round_score: (i32, i32)) {
        self.my_score += round_score.0;
        self.their_score += round_score.1;
    }

}
