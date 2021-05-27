use ramp::Int;
use ramp::RandomInt;
use rand::prelude::*;

fn main() {

    let mut rng = thread_rng();
    let gma: Int = rng.gen_uint(2048);

    println!("{}",gma);
}
