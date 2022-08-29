# TNT
TNT is a free library that provides a rich set of functions that implement many certificates about Integers. It is carefully designed to be fast while producing correct results. The name stands for **TNT Number Theory**.

TNT also has a wide variety of tests that check the correctness of the implemented algorithm unlike other proposals.

Here is the list of certificates that TNT provides:

* **Primality:** whether the input is a prime number.
  * [AKS 2002](https://cse.iitk.ac.in/users/manindra/algebra/primality_original.pdf)
  * [AKS 2003](https://www.cse.iitk.ac.in/users/manindra/algebra/primality_v6.pdf) (in progress)
  * [AKS Bernstein](https://cr.yp.to/papers/aks-20030125-retypeset20220327.pdf#page=8) (in progress)
  * [AKS 2005](https://math.dartmouth.edu/~carlp/PDF/complexity12.pdf) (in progress)
* **Prime Sieves:** returns an ordered list of primes below the input.
  * [Sieve of Eratosthenes](https://cp-algorithms.com/algebra/sieve-of-eratosthenes.html#implementation)
  * [Quadratic Sieve of Atkin](https://cr.yp.to/papers/primesieves-20020329-retypeset20220327.pdf) (in progress)
* **Perfect Power Detection:** mostly algorithms by Daniel J. Bernstein.
  * [Detecting Perfect Powers in Essentially Linear Time](https://www.ams.org/journals/mcom/1998-67-223/S0025-5718-98-00952-1/S0025-5718-98-00952-1.pdf).
  * [Detecting perfect powers by factoring into coprimes](https://cr.yp.to/lineartime/powers2-20060914-ams.pdf) (in progress)
