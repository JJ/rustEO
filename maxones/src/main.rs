extern crate rand;

use rand::Rng;

fn main() {
    let mut len = 16;
    let last = 16384;
    let how_many = 10000;
    let mut rng = rand::thread_rng();
    while len <= last {
        for i in 0..how_many {
            let this_vec: Vec<bool> = rng.gen_iter::<bool>().take(len).collect();
            let maxones = this_vec.iter().fold(0, |acc, &item| acc + is_one(item));
            println!("{}", maxones)
        }
        println!("{}",len);
        len = len*2;
    }
}

fn is_one( bit: bool ) -> i16 {
    if  bit == true  {
        1
    } else {
        0
    }
}
