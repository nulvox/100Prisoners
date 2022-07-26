use rand::prelude::SliceRandom;
use rand::thread_rng;
const SAMPLE: usize = 100;
const TRIAL_COUNT: usize = 1000000;
fn trial() -> bool {
    let mut successes: [bool; SAMPLE] = [false; SAMPLE];
    let mut rng = thread_rng();
    let mut boxen = [0; SAMPLE];
    for i in 0..=SAMPLE-1 {
        boxen[i] = i;
    }
    boxen.shuffle(&mut rng);
    for i in 0..=SAMPLE-1 {
        let mut next: usize = i as usize;
        for _j in 0..(SAMPLE/2)-1 {
            next = boxen[next];
            if next == i {
                successes[i as usize] = true;
                break;
            }
        }
        if !successes[i as usize] {
            return false
        }
    }
    return true
}
fn main() {
    let mut pass = 0.0;
    let mut fail = 0.0;
    for _i in 0..TRIAL_COUNT {
        if trial() == true {
            pass += 1.0;
        } else {
            fail += 1.0;
        }
    }
    if fail == 0.0 {
        fail += 1.0;
    }
    let rate = (pass/fail)*100.0;
    println!("The guessers pass {0}% of the time", rate);
}
