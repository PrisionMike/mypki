// use ramp::int::Int;
// use ramp::int::RandomInt;
use ramp::Int;
use ramp::RandomInt;
// use rand::rngs::OsRng;
// use rand::thread_rng;
// use rand::rngs::OsRng;
use rand::prelude::*;

// extern crate rand;
// extern crate ramp;

fn main() {
    // let gma = Int::from(587454);
    // let mut rng = rand::thread_rng();
    let mut rng = thread_rng();
    // let mut rng = OsRng;
    // let gma:Int = rng.gen_uint(256);
    let gma: Int = rng.gen_uint(2048);
    println!("{}",gma);
}
