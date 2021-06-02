use ramp::Int;
use ramp::RandomInt;
use rand::prelude::*;
use mypki::*;
use std::time::{Duration, Instant};

fn main() {

    let mut rng = thread_rng();
    
    let mut k = 0;      // Count the number of times the loop is run.
    loop {

        k += 1;

        let tic: Instant = Instant::now();

        let mut gma: Int = rng.gen_uint(2048);

        let toc: Duration = tic.elapsed();

        // Instead of forcing the MSB to be 1, I am forcing any of
        // the top 8 MSBs to be 1 thereby increasing the range of
        // possible primes without any practical compromise in security
        let topcap: u16 = rng.gen_range(0,10) as u16;

        gma.set_bit(0,true);    // Only odd numbers
        gma.set_bit((2047-topcap) as u32,true);     // Lower bound on prime numbers

        // println!("Candi no. {}:\n{}",&k,&gma);

        if prime_eh( &gma ) {

            println!("The prime is:\n{}\n\nfound at the {}th attempt",gma,k);

            println!("It takes about {:?} to thread out an rng",toc);

            break;
        }
    }
    
    // println!("{}{}",gma, prime_eh(&gma));

    // miller_rabin(&Int::from(69),&0);
}

fn prime_eh(n: &Int) -> bool {
    
    let mr_rounds : u8 = 16;

    // if !primitive_primality_test(n) {
    //     return false
    // } else

    // if !fermat_little(&n) {
    //     return false
    // } else

    // if !miller_rabin(&n, &mr_rounds) {
    //     return false
    // }

    let t1: bool = primitive_primality_test(n);

    if !t1 {
        return false;
    }

    let t2: bool = fermat_little(n);

    if !t2 {
        return false;
    }

    let t3: bool = miller_rabin( n, &mr_rounds );

    if !t3 {
        return false;
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

fn miller_rabin( n : &Int , mr_rounds : &u8) -> bool {
    let one = &Int::one();
    let two = &Int::from(2);
    // println!("One is {} and two is {}", one, two);

    let mut nminone : Int = n - one;

    let mut s : u16 = 0;
    while nminone.is_even() {

        s = s + 1;

        nminone = nminone.divmod(two).0;

    }

    let d = nminone;

    // d = &d + one;
    // d = &d - one;       // Just to check if d is mutable or not.

    let mut rng = thread_rng();

    for _i in 1..*mr_rounds {

        let base : Int = rng.gen_uint_below( &(n - 4) ) + two;

        let mut x = base.pow_mod(&d,n);

        if x == *one || x == n - one {
            continue;
        }

        let mut flag: bool = false;
        for _j in 1..s-1 {
            x = x.pow_mod( two, n );
            if x == n - one {
                flag = true;
                break;
            }
        }

        if flag {
            continue;
        } else {
            return false
        }

    }
    // println!("s,d are: {},{}",s,d);

    true
}

fn primitive_primality_test (n : &Int) -> bool {
    for p in SMALL_PRIMES.iter() {

        // println!("testing with {}",p);
        if n % Int::from(*p) == 0 { return false }

    }
    true
}