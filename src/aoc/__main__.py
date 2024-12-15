from .parser import AocArgParser
from .days import day2, day3, day11, day14

def main():
    print("Welcome to the Advent of Code.")

    parser = AocArgParser()
    args = parser.parse_args()

    print(f"selected day {args.day}")

    with open(args.input, "r") as f:
        data = "".join(f.readlines())

        match args.day:
            case "2":
                print(f"result: {day2.solve(data)}")
            case "3":
                print(f"result: {day3.solve(data)}")
            case "3+":
                print(f"result: {day3.solve_plus(data)}")
            case "11":
                print(f"result: {day11.solve(data)}")
            case "14":
                print(f"result: {day14.solve(data)}")
            case _:
                print("the selected day has not been solved yet.")