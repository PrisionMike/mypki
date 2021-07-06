use ramp::Int;
use ramp::RandomInt;
use rand::prelude::*;
use mypki::*;
use std::time::{Duration, Instant};
// use std::mem;

fn main() {

    let tm : bool = true;       // TELEMETRY SWITCH. 'true : bool' to measure timing.
    
    let mut rng = thread_rng();
    
    let mut k : u16 = 0;      // Count the number of times the loop is run.
    loop {

        k += 1;

        let mut gma: Int = rng.gen_uint(2048);

        // let dur = time_it!(let mut gma: Int = rng.gen_uint(2048));

        /*
        Instead of forcing the MSB to be 1, I am forcing any of
        the top 10 MSBs to be 1 thereby increasing the range of
        possible primes without any practical compromise in security
        */
        let topcap: u16 = rng.gen_range(0,10) as u16;

        gma.set_bit(0,true);    // Only odd numbers
        gma.set_bit((2047-topcap) as u32,true);     // Lower bound on prime numbers

        let mut askpri = Query::new(&gma);

        if !tm {
            askpri.set_flags(0);
        }

        let result = prime_eh( &askpri );

        if result.get_res() {

            println!("The prime is:\n{}\n\nfound at the {}th attempt",gma,k);

            if tm {

                println!("It takes about...\n{:?} to run the primitive primality test,\n{:?} for the Fermat Little test,\n{:?} for the Miller Rabbin test",
                            result.timings[0],
                        result.timings[1],
                    result.timings[2]);

                println!("We forced the {}th bit to be high",topcap);
            
            }

            break;
        }
    }

}
