import typing as t
import re
import itertools as i
import functools as ft
from functools import lru_cache
import operator

NUM_EVALUATIONS = 75
current_iteration = NUM_EVALUATIONS-1

@lru_cache
def expand(val: int) -> t.List[int]:
    if val == 0:
        return [1]
    s = str(val)
    l = len(s)
    if l != 0 and l % 2 == 0:
        lt = int(l/2)
        left, right = s[:lt], s[lt:]
        return [int(left), int(right)]
    return [2024*val]

# result 229043
# result plus 272673043446478
def solve(data: str):
    initial = [int(el) for el in data.split(" ") if el]
    print(initial)

    return solve_rec(initial)

    num = 0
    for stone in initial:
        print(f"stone {stone}")
        current = [stone]
        for _ in range(NUM_EVALUATIONS):
            print(f"NUM {_}")
            ev = []
            for el in current:
                res = expand(el)
                ev.extend(res)
            current = ev
        num += len(current)
    return num

def expand_rec(vals, it):
    print(len(vals))
    count = 0
    for el in vals:
        current = expand(el)
        if it > 0:
            sub = expand_rec(current, it-1)
            count += sub
        else:
            count += len(current)
    return count

# solved in rust cause faster :D
def solve_rec(initial):
    count = 0
    for stone in initial:
        print(f"stone {stone}")
        # first it
        c = expand_rec([stone], NUM_EVALUATIONS-1)
        count += c
    return count