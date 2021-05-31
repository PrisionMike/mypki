use ramp::Int;
use ramp::RandomInt;
use rand::prelude::*;
use mypki::SMALL_PRIMES;
// use std::time::{Duration, Instant};

// static mut rng: ThreadRng = thread_rng();

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

        // println!("Candi no. {}:\n{}",&k,&gma);

        if prime_eh( &gma ) {
            println!("The prime is:\n{}\n\nfound at the {}th attempt",gma,k);
            // println!("IT'S A PRIME!!");
            break;
        }
    }
    
    // println!("{}{}",gma, prime_eh(&gma));
}

fn prime_eh(n: &Int) -> bool {
    
    let mr_rounds : u8 = 10;

    if !primitive_primality_test(n) {
        return false
    } else

    if !fermat_little(&n) {
        return false
    } else

    if !miller_rabin(&n, &mr_rounds) {
        return false
    }

    true
}

fn fermat_little(n: &Int) -> bool {

    let mut rng = thread_rng();

    let somea: Int = rng.gen_uint_below(n);

    // let called = Instant::now();
    
    let res = somea.pow_mod( &(n - Int::one()) , n );

    // let did_the_powmod = called.elapsed();

    // println!("Doing a little fermat number here:\nTried with {}\nleft a residue {}", somea, res);

    // println!("Took {:?} to generate a rand brat.\n{:?} to do the pow mod",brought_the_rand,did_the_powmod);
    // println!("Powmod took: {:?}",did_the_powmod);
    
    res == Int::one()
}

fn miller_rabin( _n : &Int , _mr_rounds : &u8) -> bool {
    true
}

fn primitive_primality_test (n : &Int) -> bool {
    for p in SMALL_PRIMES.iter() {

        // println!("testing with {}",p);
        if n % Int::from(*p) == 0 { return false }

    }
    true
}