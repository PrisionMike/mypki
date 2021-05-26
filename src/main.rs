// use ramp::int::Int;
// use ramp::int::RandomInt;
use ramp::RandomInt;
// use rand::rngs::OsRng;
use rand;

extern crate rand;
extern crate ramp;

fn main() {
    // let gma = Int::from(587454);
    let mut rng = rand::thread_rng();
    let gma:Int = rng.gen_uint(256);
    println!("{}",gma);
}
