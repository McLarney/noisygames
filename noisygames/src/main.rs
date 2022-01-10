use ndarray::prelude::*;

fn main() {
    let num_runs = 1;
    //potential strategies for now are always defect, tit for tat, and grim trigger
    let a_strat = "titfortat";
    let b_strat = "grimtrigger";

    let a_mtx=arr2(&[[-1, -3],
                   [0, -2]]);

    let b_mtx=arr2(&[[-1, 0],
                   [-3, -2]]);
    //send this into a constructor for a Game type
    

    //once we have a Game, I'd like to know what its Nash Equilibrium is

    //run iterated some N times with strategy profiles specified for each player


    println!("{}", b_mtx.slice(s![1,0]));

}
