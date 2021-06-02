use ramp::Int;
use ramp::RandomInt;
use rand::prelude::*;
use mypki::*;
use std::time::{Duration, Instant};

fn main() {

    // let tic: Instant = Instant::now();
    
    let mut rng = thread_rng();

    // let toc: Duration = tic.elapsed();
    
    let mut k : u16 = 0;      // Count the number of times the loop is run.
    loop {

        k += 1;

        let mut gma: Int = rng.gen_uint(2048);


        // Instead of forcing the MSB to be 1, I am forcing any of
        // the top 8 MSBs to be 1 thereby increasing the range of
        // possible primes without any practical compromise in security
        let topcap: u16 = rng.gen_range(0,10) as u16;

        gma.set_bit(0,true);    // Only odd numbers
        gma.set_bit((2047-topcap) as u32,true);     // Lower bound on prime numbers

        let result = prime_eh( &gma );

        if result.get_res() {

            println!("The prime is:\n{}\n\nfound at the {}th attempt",gma,k);

            println!("It takes about...\n{:?} to run the primitive primality test,\n{:?} for the Fermat Little test,\n{:?} for the Miller Rabbin test",
                        result.timings[0],
                    result.timings[1],
                result.timings[2]);

            break;
        }
    }

}

fn prime_eh(n: &Int) -> Reply {
    
    let mr_rounds : u8 = 16;        // MILLER RABBIN ROUNDS

    let mut output = Reply::new(false);

    let tic: Instant = Instant::now();

    let t1: bool = primitive_primality_test(n).get_res();

    let toc: Duration = tic.elapsed();

    if !t1 {
        // out.set_res(false)
        return output;
    }

    let tic2: Instant = Instant::now();

    let t2: bool = fermat_little(n).get_res();

    let toc2: Duration = tic2.elapsed();

    if !t2 {
        return output;
    }

    let tic3: Instant = Instant::now();

    let t3: bool = miller_rabin( n, &mr_rounds ).get_res();

    let toc3: Duration = tic3.elapsed();

    if !t3 {
        return output;
    }

    output.set_res(true);
    output.push_that_time(&toc);
    output.push_that_time(&toc2);
    output.push_that_time(&toc3);

    output
}

fn fermat_little(n: &Int) -> Reply {

    let mut output = Reply::new(false);

    let mut rng: ThreadRng = thread_rng();

    let somea: Int = rng.gen_uint_below(n);
    
    let res = somea.pow_mod( &(n - Int::one()) , n );
    
    output.set_res(res == Int::one());

    output
}

fn miller_rabin( n : &Int , mr_rounds : &u8) -> Reply {
    
    let mut output = Reply::new(false);
    
    let one: &Int = &Int::one();
    let two: &Int = &Int::from(2);

    let mut nminone : Int = n - one;

    let mut s : u16 = 0;
    while nminone.is_even() {

        s = s + 1;

        nminone = nminone.divmod(two).0;

    }

    let d: Int = nminone;

    let mut rng: ThreadRng = thread_rng();

    for _i in 1..*mr_rounds {

        let base : Int = rng.gen_uint_below( &(n - 4) ) + two;

        let mut x: Int = base.pow_mod(&d,n);

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
            return output
        }

    }

    output.set_res(true);
    output
}

fn primitive_primality_test (n : &Int) -> Reply {

    let mut output : Reply = Reply::new(false);
    for p in SMALL_PRIMES.iter() {

        // println!("testing with {}",p);
        if n % Int::from(*p) == 0 { return output }

    }
    
    output.set_res(true);

    output
}