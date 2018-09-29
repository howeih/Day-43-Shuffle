
extern crate rand;
use rand::prelude::*;

fn shuffle(data: &mut [i32]){
    let n = data.len();
    let mut rng = thread_rng();
    for i in 0..n{
        let k = rng.gen_range(0, n);
        let temp = data[k];
        data[k] = data[i];
        data[i] = temp; 
    }
}

fn main() {
    let mut data: Vec<i32> = (0..10).collect();
    shuffle(&mut data);
    println!("{:?}", data);
}
