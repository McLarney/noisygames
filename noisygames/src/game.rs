use serde::Serialize;

#[derive(Clone,Serialize)]
pub struct Game {
    pub payoff_a: Vec<Vec<i32>>,
    pub payoff_b: Vec<Vec<i32>>,
    pub is_init: bool,
}

impl Game {
    pub fn init_game(&mut self){
        //here I should check that the dimensions are the same before assignment
        self.is_init = true;
    }
    pub fn get_payoff_mtx(&self, label: &str) -> Vec<Vec<i32>>{
        match label {
            "payoff_a" => self.payoff_a.clone(),
            "payoff_b" => self.payoff_b.clone(),
            _ => panic!("Not a valid payoff matrix. Valid matrices are payoff_a and payoff_b"),
        }
    }
    fn check_applied_moves(&self, player_a_move: usize, player_b_move: usize){
        assert!(self.payoff_a.len() > player_a_move);
        assert!(self.payoff_a[0].len() > player_b_move);
    }
    pub fn turn_outcome(&self, player_a_move: usize, player_b_move: usize) -> (i32, i32) {
        assert!(self.is_init);
        self.check_applied_moves(player_a_move, player_b_move);
        (self.payoff_a[player_a_move][player_b_move], self.payoff_b[player_a_move][ player_b_move]) 
    }

}
