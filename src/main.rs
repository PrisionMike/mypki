use ramp::Int;
use ramp::RandomInt;
use rand::prelude::*;
use mypki::SMALL_PRIMES;

fn main() {

    let mut rng = thread_rng();
    
    let mut k = 0;      // Count the number of times the loop is run.
    loop {

        k += 1;
        let mut gma: Int = rng.gen_uint(2048);

        // instead of forcing the MSB to be 1, I am forcing any of
        // the top 8 MSBs to be 1 thereby increasing the range of
        // possible primes without any practical compromise in security
        let topcap: u16 = rng.gen_range(0,10) as u16;

        gma.set_bit(0,true);    // Only odd numbers
        gma.set_bit((2047-topcap) as u32,true);     // Lower bound on prime numbers

        if prime_eh(&gma) {
            println!("The prime is:\n{}\n\nfound at the {}th attempt",&gma,k);
            println!("Forced the {}th fucker to be high",2048-topcap);
            // println!("List of small primes:\n{:?}",SMALL_PRIMES);
            break;
        }
    }
    
    // println!("{}{}",gma, prime_eh(&gma));
}

fn prime_eh(n: &Int) -> bool {
    
    // Fake execution. Just checking the 69th bit.
    // if n.bit(69) {
    //     true
    // } else { false }

    for p in SMALL_PRIMES.iter() {
        if n % Int::from(*p) == 0 { return false }
        // println!("testing with {}", p);
    }
    // true

    if !fermat_little(&n) {
        return false
    } else {
        miller_rabin(&n)
    }

    true
}

fn fermat_little(n: &Int) -> bool {
    let mut rng = thread_rng();
    let somea: Int = rng.gen_uint_below(n);
    let res = somea.pow_mod( &(n - Int::one()) , n );
    res == Int::one()
}
