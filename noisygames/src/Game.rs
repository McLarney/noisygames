use ndarray::prelude::*;

#[derive(Clone)]
pub struct Game {
    pub payoff_a: Array2<i32>,
    pub payoff_b: Array2<i32>,
    pub is_init: bool,
}

impl Game {
    pub fn init_game(&mut self){
        //here I should check that the dimensions are the same before assignment
        self.check_dimensions();
        self.is_init = true;
    }

    fn check_dimensions(&self) {
        assert_eq!(self.payoff_a.dim(), self.payoff_b.dim())
    }

    //might also want to have an init function to allow moves to be played, ie that all the checks
    //on proper game setup have been performed.

    pub fn get_payoff_mtx(&self, label: &str) -> Array2<i32>{
        match label {
            "payoff_a" => self.payoff_a.clone(),
            "payoff_b" => self.payoff_b.clone(),
            _ => panic!("Not a valid payoff matrix. Valid matrices are payoff_a and payoff_b"),
        }
    }
    
    fn check_applied_moves(&self, player_a_move: usize, player_b_move: usize){
        assert!(self.payoff_a.dim().0 > player_a_move);
        assert!(self.payoff_a.dim().1 > player_b_move);
    }
    pub fn turn_outcome(&self, player_a_move: usize, player_b_move: usize) -> (i32, i32) {
        assert!(self.is_init);
        self.check_applied_moves(player_a_move, player_b_move);
        (self.payoff_a[[player_a_move, player_b_move]], self.payoff_b[[player_a_move, player_b_move]]) 
    }

}
