A Rust based Quantum Prime Number Generator.

The project was initially aimed at a full 'from scratch' Digital signature implementation.
But having found a nice niche convergence at creating prime numbers, I will wrap it up here.

Since the entire project is now dedicated to be a prime number generator, I better add more features to it.

BOOKKEEPING: Profiling/Logging. We'll log the different stats of generating each random number.
Number of attempts for different RNG streams. Individual test failure rate.

Such data would be made available for learning or just for fun.

WHY RUST?
To practise. I love the language. I love IoT. I love DLTs. I love cryptography in general. Seems like
a great common staple to all.

Target: A full fledged API which returns an in-situ prime number from a Quantum Random Stream
of numbers.

CAVEAT: COMPROMISING THE RNG = COMPROMISING THE PRIME.  
Since the QR stream I am gonna use (at least initially) will likely be public,
And INITIALLY there would be no POST PROCESSING, an adversary doesn't even need to copy
my code to get the prime number. They can use just about any primality test and have the same prime.

I'll likely leave well labelled scope (maybe some placeholder implementation) to add post processing
the RN stream. Also many systems need public prime numbers that would be as good. I may as well convert it into
simply a make_it_prime plugin on an RNG. If you don't care about the generated RN numbers be secured,
Use the code as it is.

However many popular schemes like RSA are literally all about keeping the primes a secret. If such
is your case, make sure your RNG is a secret.
