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
            println!("{:?}",this_vec);
        }
        println!("{}",len);
        len = len*2;
    }
}
