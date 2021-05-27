use ramp::Int;
use ramp::RandomInt;
use rand::prelude::*;

fn main() {

    let mut rng = thread_rng();
    
    let mut k = 0;      // Count the number of times the loop is run.
    loop {
        let mut gma: Int = rng.gen_uint(2048);

        // instead of forcing the MSB to be 1, I am forcing any of
        // the top 8 MSBs to be 1 thereby increasing the range of
        // possible primes without any practical compromise in security
        let topcap: u16 = rng.gen_range(0,10) as u16;

        gma.set_bit(0,true);    // Only odd numbers
        gma.set_bit((2047-topcap) as u32,true);     // Lower bound on prime numbers

        if prime_eh(&gma) {
            println!("The prime is:\n{}",&gma);
            break;
        }
    }
    
    // println!("{}{}",gma, prime_eh(&gma));
}

fn prime_eh(n: &Int) -> bool {
    
    // Fake execution. Just checking the 69th bit.
    if n.bit(69) {
        true
    } else { false }
}
