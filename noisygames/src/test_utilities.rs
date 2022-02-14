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
