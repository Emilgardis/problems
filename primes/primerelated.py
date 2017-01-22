#!usr/bin/env python3.4
# coding: utf-8
"""Prime related stuff, really useful.
Various sources.
Examples
>>> divisors(24)
{1, 2, 3, 4, 6, 8, 12}
>>> prime_factors(24)
[2, 2, 2, 3]
"""
import itertools as it
from functools import reduce
from operator import mul
from collections import Counter


def prime_gen():
    """Makes primes effiecently
    inspired by David Eppstein: http://code.activestate.com/recipes/117119/"""
    sieve = {}
    for count in it.count(2):
        if count not in sieve:
            # count is a prime, now yield and find multiple
            yield count
            sieve[count*count] = [count]
            print(sieve)
        else:
            # count is a composite, now
            for p in sieve[count]:
                sieve.setdefault(p + count, []).append(p)
            del sieve[count]
        count += 1


def prime_gen2():
    """Makes primes
    David Eppstein, Raymond Hettinget et. al
    http://www.macdevcenter.com/pub/a/python/excerpt/pythonckbk_chap1/index1.html?page=2
    """
    D = {}
    yield 2
    for q in it.islice(it.count(3), 0, None, 2):
        p = D.pop(q, None)
        if p is None:
            D[q*q] = q
            yield q
        else:
            x = p + q
            while x in D or not (x & 1):
                x += p
            D[x] = p


def prime_gen3():
    """Makes primes
    Same authors as prime_gen2 but with variations from stackoverflow user tzots
    http://stackoverflow.com/a/3796442/4284367
    """
    D = {9: 3, 25: 5}
    yield 2
    yield 3
    yield 5
    MASK = 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0,
    MODULOS = frozenset((1, 7, 11, 13, 17, 19, 23, 29))

    for q in it.compress(
            it.islice(it.count(7), 0, None, 2),
            it.cycle(MASK)):
        p = D.pop(q, None)
        if p is None:
            D[q*q] = q
            yield q
        else:
            x = q + 2*p
            while x in D or (x % 30) not in MODULOS:
                x += 2*p
            D[x] = p


def prime_factors(n, prime_genx=prime_gen):
    """Gives all prime factors of n"""
    factors = []
    start = n
    for prime in prime_genx():
        if prime > n*n:
            break
        if n % prime == 0:
            while n % prime == 0:
                factors.append(prime)
                n = n / prime
            n = start
    return factors


def powerset(iterable):
    "powerset([1,2,3]) --> () (1,) (2,) (3,) (1,2) (1,3) (2,3) (1,2,3)"
    s = list(iterable)
    return it.chain.from_iterable(
        it.combinations(
            s,
            r) for r in range(
            len(s) +
            1))


def divisors(n):
    """Generates proper divisors.
    For sum of divisors use sum_divisors.
    TODO: For number of divisors use amount_divisors.
    """
    divisors = list(powerset(prime_factors(n)))
    del divisors[0], divisors[-1]
    divisors = set([reduce(mul, x) for x in divisors])
    divisors.add(1)
    return divisors


def sum_divisors(n):
    """The sum of all proper divisors of n.
    """
    if n == 1:
        return 1
    factors = prime_factors(n)
    count = Counter(factors)
    uniques = list(count.keys())
    denominator = reduce(mul, [n-1 for n in uniques])
    numerator = reduce(mul, [(n**(i+1))-1 for n, i in count.items()])
    result = (numerator/denominator) - n
    assert result.is_integer()
    return int(result)
