use ndarray::arr2;
pub struct Game {
    payoff_A: arr2,
    payoff_B: arr2,
    is_init: bool,
}

impl Game {
    pub fn load_game(&mut self, payoff_A: arr2, payoff_B: arr2){
        //here I should check that the dimensions are the same before assignment
        self.set_payoff_mtx(payoff_A, "payoff_A");
        self.set_payoff_mtx(payoff_B, "payoff_B");

        self.is_init=1;
    }

    fn set_payoff_mtx(&mut self, payoff_mtx: arr2, label: str){
        //there has to be a better way to write this
        if label == "payoff_A" {
            self.payoff_A = payoff_mtx;
        } else if label == "payoff_B" {
            self.payoff_B = payoff_mtx;
        }
    }

    //might also want to have an init function to allow moves to be played, ie that all the checks
    //on proper game setup have been performed.
    

    pub fn get_payoff_mtx(&mut self, label: str) -> arr2{
        match label {
            "payoff_A" => self.payoff_A,
            "payoff_B" => self.payoff_B,
            _ => panic!("Not a valid payoff matrix. Valid matrices are payoff_A and payoff_B"),
        }
    }
    
    pub fn turn_outcome(&mut self, player_A_move: i32, player_B_move: i32) -> (i32, i32) {
        
        let a_out = self.payoff_A.slice(s![player_A_move, player_B_move]);
        let b_out = self.payoff_B.slice(s![player_A_move, player_B_move]);

        (a_out, b_out) 
    }

}
