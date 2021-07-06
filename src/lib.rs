use std::time::{Duration,Instant};
use std::u8;
use ramp::Int;
use ramp::RandomInt;
use rand::prelude::*;

/*
#[macro_export]
macro_rules! time_it {
    ( $func : stmt) => {
        let tic = Instant::now();
        $func
        let toc = tic.elapsed();
        toc
    }
}
*/


pub fn prime_eh(inquiry: &Query) -> Reply {
    
    let n = inquiry.get_int();

    let tm = match inquiry.get_flags() {
        0 => false,
        1 => true,
        _ => true
    };

    let mr_rounds : u8 = 16;        // MILLER RABBIN ROUNDS

    let mut output = Reply::new(false);

    let t1 = if tm {

        let tic: Instant = Instant::now();

        let t1: bool = primitive_primality_test(n).get_res();

        let toc: Duration = tic.elapsed();

        output.push_that_time(&toc);

        t1
    } else {
        primitive_primality_test(n).get_res()
    };

    if !t1 {
        // out.set_res(false)
        return output;
    }

    let t2 = if tm {
        let tic2: Instant = Instant::now();

        let t2: bool = fermat_little(n).get_res();

        let toc2: Duration = tic2.elapsed();

        output.push_that_time(&toc2);

        t2
    } else {
        fermat_little(n).get_res()
    };

    if !t2 {
        return output;
    }

    let t3 = if tm {
        let tic3: Instant = Instant::now();

        let t3: bool = miller_rabin( n, &mr_rounds ).get_res();

        let toc3: Duration = tic3.elapsed();

        output.push_that_time(&toc3);

        t3
    } else {
        miller_rabin( n, &mr_rounds ).get_res()
    };

    if !t3 {
        return output;
    }

    output.set_res(true);

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

pub struct Query<'a> {

    theint : &'a Int,
    flagses : u8

    /*
    Flagses is going to be an 8 bit control word
    which will contain all the commands encoded
    in the 8 bit word.

    0 = No Optimisation. No Telemetry
    1 = No Optimisation. Telemetry.
    2 = Optimisation. No Telemetry.
    3 = Optimisation. Telemetry.
    */

}

impl<'a> Query<'a> {

    pub fn new ( gma : &'a Int) -> Query<'a>{
        Query {
            theint : gma,
            flagses : 1
        }
    }

    pub fn get_int(&self) -> &Int {
        self.theint
    }

    pub fn set_flags ( &mut self, new_flags : u8) {
        self.flagses = new_flags;
    }

    pub fn get_flags (&self) -> &u8 {
        &self.flagses
    }
}


pub struct Reply {
    boo_result : bool,
    pub timings : Vec<Duration>
}


impl Reply {

    pub fn new( boores : bool) -> Reply {
        
        Reply{
            boo_result : boores,
            timings : Vec::new()
        }

    }

    pub fn get_res(&self) -> bool {
        return self.boo_result;
    }

    pub fn set_res( &mut self, val : bool ) {
        self.boo_result = val;
    }

    pub fn push_that_time( &mut self, time_length : &Duration) {

        self.timings.push(*time_length);

    }

}

// pub enum control_word {
    
// }



// List of first 249 odd primes
pub static SMALL_PRIMES: [u16; 249] = [3,5,7,11,13,17,19,23,29,31,37,
                                    41,43,47,53,59,61,67,71,73,79,
                                    83,89,97,101,103,107,109,113,
                                    127,131,137,139,149,151,157,
                                    163,167,173,179,181,191,193,
                                    197,199,211,223,227,229,233,
                                    239,241,251,257,263,269,271,
                                    277,281,283,293,307,311,313,
                                    317,331,337,347,349,353,359,
                                    367,373,379,383,389,397,401,
                                    409,419,421,431,433,439,443,
                                    449,457,461,463,467,479,487,
                                    491,499,503,509,521,523,541,
                                    547,557,563,569,571,577,587,
                                    593,599,601,607,613,617,619,
                                    631,641,643,647,653,659,661,
                                    673,677,683,691,701,709,719,
                                    727,733,739,743,751,757,761,
                                    769,773,787,797,809,811,821,
                                    823,827,829,839,853,857,859,
                                    863,877,881,883,887,907,911,
                                    919,929,937,941,947,953,967,
                                    971,977,983,991,997,1009,1013,
                                    1019,1021,1031,1033,1039,1049,
                                    1051,1061,1063,1069,1087,1091,
                                    1093,1097,1103,1109,1117,1123,
                                    1129,1151,1153,1163,1171,1181,
                                    1187,1193,1201,1213,1217,1223,
                                    1229,1231,1237,1249,1259,1277,
                                    1279,1283,1289,1291,1297,1301,
                                    1303,1307,1319,1321,1327,1361,
                                    1367,1373,1381,1399,1409,1423,
                                    1427,1429,1433,1439,1447,1451,
                                    1453,1459,1471,1481,1483,1487,
                                    1489,1493,1499,1511,1523,1531,
                                    1543,1549,1553,1559,1567,1571,
                                    1579,1583];