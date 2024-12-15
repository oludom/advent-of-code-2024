import typing as t
import re
import itertools as i
import functools as ft
import operator



# result 173517243
def solve(data: str):
    expr = re.compile(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)")
    res = ft.reduce(operator.add, [int(el.group(1)) * int(el.group(2)) for el in expr.finditer(data)])
    return res

# result 100450138
def solve_plus(data: str):
    ido_expr = re.compile(r"do\(\)")
    idont_expr = re.compile(r"don't\(\)")

    # find do()
    starts = [el.end(0) for el in ido_expr.finditer(data)]
    # find don't()
    ends = [el.start(0) for el in idont_expr.finditer(data)]

    enabled = True
    lastpos = 0
    ranges = []
    si = iter(starts)
    ei = iter(ends)

    # number of do's and don't not the same
    # so search for next with context
    while True:
        if enabled:
            el = next(ei, -1)
            if el > lastpos:
                ranges.append((lastpos, el))
                enabled = False
                lastpos = el
            elif el == -1:
                break
        else:
            el = next(si, -1)
            if el > lastpos:
                enabled = True
                lastpos = el
            elif el == -1:
                break
    
    # solve only for ranges between do's and don'ts
    res = ft.reduce(operator.add, [solve(data[el[0]:el[1]]) for el in ranges])
    return res
