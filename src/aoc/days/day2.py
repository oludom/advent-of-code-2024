import typing as t
import re
import itertools as i
import functools as ft
import operator
from copy import deepcopy

# result 524
# result 2 569
def solve(data: str):
    reports = data.split("\n")
    print(f"num reports: {len(reports)}")

    reports = [[int(num) for num in el.split(" ") if num] for el in reports if el]

    def validate(report: t.List[int]) -> bool:
        direction = 0
        for (left, right) in i.pairwise(report):
            if right == left: 
                return False
            if direction == 0: 
                direction = -1 if right < left else 1

            if abs(right-left) > 3:
                return False
            if direction == -1 and right > left:
                return False
            elif direction == 1 and left > right:
                return False
        return True
    
    num_valid = 0
    for report in reports:
        if validate(report):
            num_valid += 1
        else:
            # part two
            # remove one level
            for j in range(len(report)):
                r = deepcopy(report)
                r.pop(j)
                if validate(r):
                    num_valid += 1
                    break

    return num_valid