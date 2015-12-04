extern crate rand;
extern crate time;

use time::PreciseTime;
use rand::Rng;

fn main() {
    let mut len = 16;
    let last = 16384;
    let how_many = 10000;
    let mut rng = rand::thread_rng();

    while len <= last {
        let start = PreciseTime::now();
        for i in 0..how_many {
            let this_vec: Vec<bool> = rng.gen_iter::<bool>().take(len).collect();
            let maxones = this_vec.iter().fold(0, |acc, &item| acc + (if item == true { 1 } else {0}));
        }
        let end = PreciseTime::now();
        println!("Rust-BitVector,{},{}", len,start.to(end));
        len = len*2;
    }
}

