use chrono::{Datelike, Timelike, Utc};
use std::fs;
use rand::thread_rng;
use rand::seq::SliceRandom;

pub fn build_datetime_folder() -> String {
    let now = Utc::now();
    let s = format!(
        "Year{}Month{}Day{}Hour{}Min{}Sec{}",
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute(),
        now.second(),
        );
    let dirstr = format!("{}{}","/home/kennethmclarney/Documents/RustProjects/noisygames/test_runs/",s);

    fs::create_dir_all(&dirstr).expect("Directory unable to be created");
    dirstr
}

pub fn shuffle_and_pair<T: Clone>(mut arr: Vec<T>) -> Vec<Vec<T>> {
    //should have an even check to start
    if arr.len() % 2 == 1 {
        panic!("Uneven number of elements to pair.");
    }

    let mut rng = thread_rng();
    arr.shuffle(&mut rng);

    let mut paired=Vec::new();

    for (idx, _) in arr.iter().enumerate().step_by(2) {
        let new_pair = vec![arr[idx].clone(),arr[idx+1].clone()];
        paired.push(new_pair);
    }

    paired
}


pub fn get_permutations_with_replacement<T: Clone>(items: Vec<T>, num_slots: u32) -> Vec<Vec<T>>{
    if num_slots == 0 {
        panic!("Stuff has to go somewhere!");
    }

    let init_vecs=populate_initial(items.clone());
    if num_slots >1 {
        let mut all_vecs=populate_next_level(init_vecs, items.clone());
        for _ in 1..num_slots-1 {
            all_vecs=populate_next_level(all_vecs, items.clone());
        }
        return all_vecs;
    } else {
        return init_vecs;
    }
}

fn populate_next_level<T: Clone>(init_vecs: Vec<Vec<T>>, vars: Vec<T>) -> Vec<Vec<T>> {
    //for every vec in vec, go over variations
    let mut all_vecs = Vec::new();
    for vec in init_vecs {
        for var in &vars {
            let mut tmp_vec=vec.clone();
            tmp_vec.push(var.clone());
            all_vecs.push(tmp_vec);
        }
    }
    all_vecs
}
fn populate_initial<T: Clone>(m: Vec<T>) -> Vec<Vec<T>> {
    let mut vecs=Vec::new();
    for v in m {
        let tmp_vec=vec![v];
        vecs.push(tmp_vec);
    }
    vecs
}

