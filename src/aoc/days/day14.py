import typing as t
import re
import itertools as i
import functools as ft
import operator




# result 225943500
def solve(data: str):
    NUM_STEPS = 100
    WIDTH = 101
    HEIGHT= 103
    # position/velocity:
    # (x, y) where x -> right, y -> down


    line_expr = re.compile(r"p=([-\d]+),([-\d]+) v=([-\d]+),([-\d]+)")
    lines = data.split("\n")
    lines = [el for el in lines if el]
    robot_matches = [line_expr.match(el) for el in lines if el]
    
    # validate matches
    if None in robot_matches:
        raise "some lines didn't match"
    robots = [[int(r.group(1)), int(r.group(2)), int(r.group(3)), int(r.group(4))] for r in robot_matches]

    simulated_positions = []

    # simulate
    for robot in robots:
        position = robot[:2]
        velocity = robot[2:4]

        # x 
        posx = position[0]
        velx = velocity[0]
        x = (NUM_STEPS * velx + posx) % WIDTH

        # y 
        posy = position[1]
        vely = velocity[1]
        y = (NUM_STEPS * vely + posy) % HEIGHT

        simulated_positions.append([x, y])

    MID_X = int(WIDTH / 2)
    MID_Y = int(HEIGHT / 2)
    def is_in_middle(x: int, y: int) -> bool:
        if x == MID_X or y == MID_Y:
            return True
        else: 
            return False
    def get_quadrant_index(x: int, y: int) -> int:
        idx = 0
        # bottom half
        if x > MID_X:
            idx += 2
        # right half
        if y > MID_Y:
            idx += 1
        return idx
    
    quad_count = [0,0,0,0]
    for pos in simulated_positions:
        if not is_in_middle(pos[0], pos[1]):
            quad_count[get_quadrant_index(pos[0], pos[1])] += 1
    
    return ft.reduce(operator.mul, quad_count)

    